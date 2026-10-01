use crate::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[cfg(unix)]
use std::os::unix::{
    fs::{OpenOptionsExt, PermissionsExt},
    process::ExitStatusExt,
};
use std::{
    collections::BTreeSet,
    fs::{self, File, OpenOptions},
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkItem {
    request: ReplayRequest,
    artifact: ReplayArtifact,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
enum WorkerReply {
    Decoded { report: Box<ReplayReport> },
    Quarantined { reason: ReplayFailure },
}

/// Clone and reuse ONE port instance from the shared scheduler. Busy work is rejected, not queued.
/// Each worker is local-only, credential-free, CPU/address-space/output bounded and kill/reaped.
#[derive(Debug, Clone)]
pub struct WorkerDecoder {
    executable: PathBuf,
    busy: Arc<AtomicBool>,
}
impl WorkerDecoder {
    pub fn new(executable: PathBuf) -> Self {
        Self {
            executable,
            busy: Arc::new(AtomicBool::new(false)),
        }
    }
}
struct Slot(Arc<AtomicBool>);
impl Drop for Slot {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}
struct ChildGuard {
    child: Child,
    reaped: bool,
}
impl Drop for ChildGuard {
    fn drop(&mut self) {
        if !self.reaped {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

impl ReplayDecoder for WorkerDecoder {
    fn decode(
        &self,
        raw_file: &Path,
        request: &ReplayRequest,
    ) -> Result<ReplayReport, ReplayFailure> {
        validate_request(request)?;
        if !cfg!(all(
            target_os = "linux",
            any(target_arch = "x86_64", target_arch = "aarch64")
        )) {
            return Err(ReplayFailure::UnsupportedPlatform);
        }
        if self
            .busy
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return Err(ReplayFailure::BudgetExceeded);
        }
        let _slot = Slot(self.busy.clone());
        let started = Instant::now();
        let deadline = started + Duration::from_millis(request.budget.wall_time_ms);
        let mut request = request.clone();
        request.selection.entity_classes.sort();
        let temp = tempfile::Builder::new()
            .prefix("dbrain-replay-")
            .tempdir()
            .map_err(|_| ReplayFailure::InputIo)?;
        #[cfg(unix)]
        fs::set_permissions(temp.path(), fs::Permissions::from_mode(0o700))
            .map_err(|_| ReplayFailure::InputIo)?;
        let mut options = OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
        let mut input = options.open(raw_file).map_err(|_| ReplayFailure::InputIo)?;
        let before = input.metadata().map_err(|_| ReplayFailure::InputIo)?;
        if !before.is_file() {
            return Err(ReplayFailure::InputIo);
        }
        if before.len() > request.budget.max_file_bytes {
            return Err(ReplayFailure::BudgetExceeded);
        }
        let mut snapshot = private_file(&temp.path().join("input.raw"))?;
        let mut digest = Sha256::new();
        let mut length = 0u64;
        let mut buffer = [0u8; 64 * 1024];
        loop {
            if Instant::now() >= deadline {
                return Err(ReplayFailure::WorkerTimeout);
            }
            let n = input
                .read(&mut buffer)
                .map_err(|_| ReplayFailure::InputIo)?;
            if n == 0 {
                break;
            }
            length = length
                .checked_add(n as u64)
                .ok_or(ReplayFailure::BudgetExceeded)?;
            if length > request.budget.max_file_bytes {
                return Err(ReplayFailure::BudgetExceeded);
            }
            snapshot
                .write_all(&buffer[..n])
                .map_err(|_| ReplayFailure::InputIo)?;
            digest.update(&buffer[..n]);
        }
        let after = input.metadata().map_err(|_| ReplayFailure::InputIo)?;
        if before.len() != length
            || after.len() != length
            || before.modified().ok() != after.modified().ok()
        {
            return Err(ReplayFailure::InputIo);
        }
        snapshot.flush().map_err(|_| ReplayFailure::InputIo)?;
        #[cfg(unix)]
        snapshot
            .set_permissions(fs::Permissions::from_mode(0o400))
            .map_err(|_| ReplayFailure::InputIo)?;
        drop(snapshot);
        drop(input);
        let sha256 = format!("{:x}", digest.finalize());
        if request
            .source
            .expected_sha256
            .as_ref()
            .is_some_and(|expected| expected != &sha256)
        {
            return Err(ReplayFailure::HashMismatch);
        }
        let artifact = ReplayArtifact {
            source: request.source.clone(),
            sha256,
            byte_length: length,
        };
        let item = WorkItem {
            request: request.clone(),
            artifact: artifact.clone(),
        };
        let encoded = serde_json::to_vec(&item).map_err(|_| ReplayFailure::InvalidRequest)?;
        if encoded.len() > 32_768 {
            return Err(ReplayFailure::InvalidRequest);
        }
        private_file(&temp.path().join("request.json"))?
            .write_all(&encoded)
            .map_err(|_| ReplayFailure::InputIo)?;
        let output_path = temp.path().join("result.json");
        let output = private_file(&output_path)?;
        let executable = self
            .executable
            .canonicalize()
            .map_err(|_| ReplayFailure::WorkerFailure)?;
        let child = Command::new(executable)
            .arg("--worker")
            .current_dir(temp.path())
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::from(output))
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| ReplayFailure::WorkerFailure)?;
        let mut guard = ChildGuard {
            child,
            reaped: false,
        };
        let status = wait_for_worker(
            &mut guard,
            deadline,
            &output_path,
            request.budget.max_output_bytes,
        )?;
        if !status.success() {
            #[cfg(unix)]
            if matches!(status.signal(), Some(libc::SIGXCPU | libc::SIGXFSZ)) {
                return Err(ReplayFailure::BudgetExceeded);
            }
            // SIGKILL/abort might be resource exhaustion OR a parser fault. Do not invent a cause.
            return Err(ReplayFailure::WorkerFailure);
        }
        if Instant::now() > deadline {
            return Err(ReplayFailure::WorkerTimeout);
        }
        let mut bytes = Vec::new();
        File::open(output_path)
            .map_err(|_| ReplayFailure::InputIo)?
            .take(request.budget.max_output_bytes + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| ReplayFailure::InputIo)?;
        if bytes.len() as u64 > request.budget.max_output_bytes {
            return Err(ReplayFailure::BudgetExceeded);
        }
        let response: WorkerReply =
            serde_json::from_slice(&bytes).map_err(|_| ReplayFailure::InvalidWorkerOutput)?;
        match response {
            WorkerReply::Quarantined { reason } => Err(reason),
            WorkerReply::Decoded { report } => {
                validate_report(&report, &artifact, &request)?;
                Ok(*report)
            }
        }
    }
}
fn wait_for_worker(
    guard: &mut ChildGuard,
    deadline: Instant,
    output: &Path,
    maximum: u64,
) -> Result<std::process::ExitStatus, ReplayFailure> {
    loop {
        if let Some(status) = guard
            .child
            .try_wait()
            .map_err(|_| ReplayFailure::WorkerFailure)?
        {
            guard.reaped = true;
            return Ok(status);
        }
        if Instant::now() >= deadline {
            return Err(ReplayFailure::WorkerTimeout);
        }
        if fs::metadata(output)
            .map_err(|_| ReplayFailure::InputIo)?
            .len()
            > maximum
        {
            return Err(ReplayFailure::BudgetExceeded);
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[cfg(test)]
#[path = "sandbox_tests.rs"]
mod tests;

fn private_file(path: &Path) -> Result<File, ReplayFailure> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);
    options.open(path).map_err(|_| ReplayFailure::InputIo)
}
fn validate_report(
    report: &ReplayReport,
    artifact: &ReplayArtifact,
    request: &ReplayRequest,
) -> Result<(), ReplayFailure> {
    let bad = ReplayFailure::InvalidWorkerOutput;
    if &report.artifact != artifact
        || report.contract_version != CONTRACT_VERSION
        || report.parser_revision != parser_revision()
        || report.schema_revision != SCHEMA_REVISION
        || report.extraction_revision != EXTRACTION_REVISION
        || report.validity != brain_contracts::source::GameValidity::unknown()
        || !report.entity_mapping.is_empty()
        || report.selection != request.selection
        || report.generation_id != generation_id(artifact, &request.selection)?
        || report.real_replay_verified
        || report.coaching_eligible
        || report.capabilities != capabilities()
        || report.commands == 0
        || report.commands > request.budget.max_commands
        || report.packets > request.budget.max_packets
        || report.total_decoded_bytes > request.budget.max_total_decoded_bytes
        || report.observations.len() > request.budget.max_observations as usize
    {
        return Err(bad);
    }
    let mut ids = BTreeSet::new();
    for o in &report.observations {
        if o.raw.file_byte_offset < 16
            || o.raw.stored_byte_length == 0
            || o.raw
                .file_byte_offset
                .checked_add(o.raw.stored_byte_length)
                .is_none_or(|end| end > artifact.byte_length)
            || o.raw.command_index >= report.commands
            || o.raw.payload_sha256.len() != 64
            || matches!(o.time.tick, Observed::Known { value } if value > i32::MAX as u32)
            || matches!(o.time.tick, Observed::Unknown { reason } if reason != UnknownReason::InitializationTick)
            || matches!(o.time.game_time_seconds, Observed::Known { .. })
        {
            return Err(bad);
        }
        if let Observed::Known { value } = o.time.tick_interval_seconds
            && (!value.is_finite() || !(0.000_001..=1.0).contains(&value))
        {
            return Err(bad);
        }
        if let Observed::Known { ref value } = o.entity
            && (value.network_index < 0
                || value.network_index as u32 >= request.budget.max_entities
                || value.network_serial.is_some()
                || !o.raw.requires_state_prefix
                || !request.selection.entity_classes.contains(&value.class_name))
        {
            return Err(bad);
        }
        let identity =
            serde_json::to_vec(&(&o.raw, &o.time, &o.entity, &o.event)).map_err(|_| bad)?;
        let id = hash_parts(&[report.generation_id.as_bytes(), &identity]);
        if id != o.observation_id || !ids.insert(id) {
            return Err(bad);
        }
    }
    Ok(())
}

#[cfg(all(
    target_os = "linux",
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
fn sandbox(budget: &DecodeBudget) -> Result<(), ReplayFailure> {
    use nix::sys::resource::{Resource, setrlimit};
    use seccompiler::{BpfProgram, SeccompAction, SeccompFilter};
    for (resource, limit) in [
        (Resource::RLIMIT_CORE, 0),
        (Resource::RLIMIT_AS, budget.memory_bytes),
        (Resource::RLIMIT_CPU, budget.cpu_seconds),
        (Resource::RLIMIT_FSIZE, budget.max_output_bytes),
        (Resource::RLIMIT_NOFILE, 16),
    ] {
        setrlimit(resource, limit, limit).map_err(|_| ReplayFailure::SandboxFailure)?;
    }
    // Allow only computation plus already-open input/stdout. No open/openat, socket/connect,
    // exec, fork/clone, ptrace, process signaling, or filesystem mutations. apply_filter also
    // sets PR_SET_NO_NEW_PRIVS. Any setup failure is fatal BEFORE touching upstream parsing.
    let syscalls = [
        libc::SYS_read,
        libc::SYS_write,
        libc::SYS_readv,
        libc::SYS_writev,
        libc::SYS_close,
        libc::SYS_lseek,
        libc::SYS_fstat,
        libc::SYS_brk,
        libc::SYS_mmap,
        libc::SYS_munmap,
        libc::SYS_mremap,
        libc::SYS_mprotect,
        libc::SYS_madvise,
        libc::SYS_futex,
        libc::SYS_clock_gettime,
        libc::SYS_getrandom,
        libc::SYS_getpid,
        libc::SYS_gettid,
        libc::SYS_sched_yield,
        libc::SYS_rt_sigaction,
        libc::SYS_rt_sigprocmask,
        libc::SYS_rt_sigreturn,
        libc::SYS_sigaltstack,
        libc::SYS_exit,
        libc::SYS_exit_group,
    ];
    let filter = SeccompFilter::new(
        syscalls.into_iter().map(|n| (n, vec![])).collect(),
        SeccompAction::Errno(libc::EPERM as u32),
        SeccompAction::Allow,
        std::env::consts::ARCH
            .try_into()
            .map_err(|_| ReplayFailure::SandboxFailure)?,
    )
    .map_err(|_| ReplayFailure::SandboxFailure)?;
    let program: BpfProgram = filter
        .try_into()
        .map_err(|_| ReplayFailure::SandboxFailure)?;
    seccompiler::apply_filter(&program).map_err(|_| ReplayFailure::SandboxFailure)
}
#[cfg(not(all(
    target_os = "linux",
    any(target_arch = "x86_64", target_arch = "aarch64")
)))]
fn sandbox(_: &DecodeBudget) -> Result<(), ReplayFailure> {
    Err(ReplayFailure::UnsupportedPlatform)
}

#[cfg(target_os = "linux")]
fn close_inherited_descriptors() -> Result<(), ReplayFailure> {
    // A caller may own a socket without CLOEXEC. Seccomp blocks new sockets, but read/write on
    // an inherited socket would still be possible. Drop everything except redirected stdio
    // BEFORE opening the private request and raw snapshot.
    let descriptors = fs::read_dir("/proc/self/fd")
        .map_err(|_| ReplayFailure::SandboxFailure)?
        .map(|entry| {
            entry
                .map_err(|_| ReplayFailure::SandboxFailure)
                .and_then(|entry| {
                    entry
                        .file_name()
                        .to_str()
                        .and_then(|s| s.parse::<i32>().ok())
                        .ok_or(ReplayFailure::SandboxFailure)
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    for descriptor in descriptors.into_iter().filter(|fd| *fd > 2) {
        let _ = nix::unistd::close(descriptor);
    }
    Ok(())
}
#[cfg(not(target_os = "linux"))]
fn close_inherited_descriptors() -> Result<(), ReplayFailure> {
    Err(ReplayFailure::UnsupportedPlatform)
}

/// Worker protocol entry point; application code uses ReplayDecoder instead.
/// The only files opened are the supervisor's private fixed-name snapshot and bounded request.
pub fn worker_stdio() -> Result<(), ReplayFailure> {
    close_inherited_descriptors()?;
    std::panic::set_hook(Box::new(|_| {}));
    let mut encoded = Vec::new();
    File::open("request.json")
        .map_err(|_| ReplayFailure::InputIo)?
        .take(32_769)
        .read_to_end(&mut encoded)
        .map_err(|_| ReplayFailure::InputIo)?;
    if encoded.len() > 32_768 {
        return Err(ReplayFailure::InvalidRequest);
    }
    let item: WorkItem =
        serde_json::from_slice(&encoded).map_err(|_| ReplayFailure::InvalidRequest)?;
    validate_request(&item.request)?;
    let input = File::open("input.raw").map_err(|_| ReplayFailure::InputIo)?;
    let result = match sandbox(&item.request.budget) {
        Err(reason) => Err(reason),
        Ok(()) => std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            crate::decode::decode_stream(BufReader::new(input), item.artifact, item.request)
        }))
        .unwrap_or(Err(ReplayFailure::ParserPanic)),
    };
    let reply = match result {
        Ok(report) => WorkerReply::Decoded {
            report: Box::new(report),
        },
        Err(reason) => WorkerReply::Quarantined { reason },
    };
    let mut out = std::io::stdout().lock();
    serde_json::to_writer(&mut out, &reply).map_err(|_| ReplayFailure::WorkerFailure)?;
    out.flush().map_err(|_| ReplayFailure::WorkerFailure)
}
