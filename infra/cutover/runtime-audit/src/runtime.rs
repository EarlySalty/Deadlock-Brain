//! Bounded, read-only cgroup-v1/v2/procfs observation. Not a Rust provenance or cycle proof.
//! No process arguments, environment, memory contents or arbitrary operator paths are read.
use crate::{AuditResult, Unit, MAX_OUTPUT_BYTES};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::fs::{self, File};
use std::io::Read;
use std::time::{Duration, Instant};

const MAX_PROCESSES: usize = 4096;
const MAX_GROUPS: usize = 256;
const MAX_DEPTH: usize = 32;
const TOTAL_BYTES: usize = 64 * 1024 * 1024;
const OBSERVATION_BUDGET: Duration = Duration::from_secs(30);

/// Read adapter for kernel metadata, injectable for hermetic tests only.
/// The CLI never accepts alternate roots or caller-supplied paths.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CgroupLayout {
    #[default]
    UnifiedV2,
    SystemdV1,
}

impl CgroupLayout {
    fn mountpoint(self) -> &'static str {
        match self {
            Self::UnifiedV2 => "/sys/fs/cgroup",
            Self::SystemdV1 => "/sys/fs/cgroup/systemd",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::UnifiedV2 => "unified_v2",
            Self::SystemdV1 => "systemd_v1",
        }
    }
}

/// Recognize only full-root kernel mounts at the two supported fixed locations.
/// Unknown mount layouts/namespaces are an explicit limitation, never guessed paths.
pub fn detect_layout(mountinfo: &str) -> AuditResult<CgroupLayout> {
    if mountinfo.len() > MAX_OUTPUT_BYTES {
        return Err("mount_metadata_too_large");
    }
    let mut selected = None;
    for line in mountinfo.lines() {
        let Some((left, right)) = line.split_once(" - ") else {
            continue;
        };
        let fields: Vec<_> = left.split_whitespace().collect();
        let filesystem: Vec<_> = right.split_whitespace().collect();
        if fields.len() < 6 || filesystem.len() < 3 {
            continue;
        }
        let layout = match (fields[3], fields[4], filesystem[0]) {
            ("/", "/sys/fs/cgroup", "cgroup2") => Some(CgroupLayout::UnifiedV2),
            ("/", "/sys/fs/cgroup/systemd", "cgroup")
                if filesystem[2].split(',').any(|v| v == "name=systemd") =>
            {
                Some(CgroupLayout::SystemdV1)
            }
            _ => None,
        };
        if let Some(layout) = layout {
            if selected.replace(layout).is_some() {
                return Err("ambiguous_cgroup_mount");
            }
        }
    }
    selected.ok_or("unsupported_cgroup_mount_or_namespace")
}

pub trait RuntimeReader {
    fn layout(&mut self) -> AuditResult<CgroupLayout> {
        Ok(CgroupLayout::UnifiedV2)
    }
    fn text(&mut self, path: &str) -> AuditResult<String>;
    fn link(&mut self, path: &str) -> AuditResult<String>;
    fn directories(&mut self, path: &str) -> AuditResult<Vec<String>>;
}

pub struct LinuxReader;

impl RuntimeReader for LinuxReader {
    fn layout(&mut self) -> AuditResult<CgroupLayout> {
        detect_layout(&self.text("/proc/self/mountinfo")?)
    }

    fn text(&mut self, path: &str) -> AuditResult<String> {
        let mut value = String::new();
        File::open(path)
            .map_err(|_| "kernel_metadata_unavailable")?
            .take((MAX_OUTPUT_BYTES + 1) as u64)
            .read_to_string(&mut value)
            .map_err(|_| "kernel_metadata_unreadable")?;
        if value.len() > MAX_OUTPUT_BYTES {
            return Err("kernel_metadata_too_large");
        }
        Ok(value)
    }

    fn link(&mut self, path: &str) -> AuditResult<String> {
        fs::read_link(path)
            .map_err(|_| "executable_unavailable")?
            .into_os_string()
            .into_string()
            .map_err(|_| "executable_non_utf8")
    }

    fn directories(&mut self, path: &str) -> AuditResult<Vec<String>> {
        let mut result = Vec::new();
        for entry in fs::read_dir(path).map_err(|_| "cgroup_unavailable")? {
            let entry = entry.map_err(|_| "cgroup_entry_unavailable")?;
            let kind = entry.file_type().map_err(|_| "cgroup_type_unavailable")?;
            // cgroupfs has no directory symlinks. Never follow an unexpected one.
            if kind.is_symlink() {
                return Err("cgroup_symlink_rejected");
            }
            if kind.is_dir() {
                result.push(
                    entry
                        .file_name()
                        .into_string()
                        .map_err(|_| "cgroup_non_utf8")?,
                );
                if result.len() > MAX_GROUPS {
                    return Err("too_many_cgroups");
                }
            }
        }
        Ok(result)
    }
}

#[derive(Debug)]
struct Process {
    unit: String,
    pid: u32,
    start: u64,
    runtime: &'static str,
    embedded_python: bool,
    deleted_executable: bool,
    deleted_mapping: bool,
}

#[derive(Debug)]
pub struct RuntimeReport {
    layout: CgroupLayout,
    processes: Vec<Process>,
    unobserved: usize,
}

struct Context<'a, R> {
    reader: &'a mut R,
    layout: CgroupLayout,
    bytes: usize,
    started: Instant,
}

impl<R: RuntimeReader> Context<'_, R> {
    fn budget(&mut self, value: String) -> AuditResult<String> {
        self.bytes = self.bytes.saturating_add(value.len());
        if value.len() > MAX_OUTPUT_BYTES || self.bytes > TOTAL_BYTES {
            return Err("runtime_metadata_budget_exceeded");
        }
        if self.started.elapsed() > OBSERVATION_BUDGET {
            return Err("runtime_observation_budget_exceeded");
        }
        Ok(value)
    }

    fn text(&mut self, path: &str) -> AuditResult<String> {
        let value = self
            .reader
            .text(path)
            .map_err(|_| "runtime_metadata_unavailable")?;
        self.budget(value)
    }

    fn link(&mut self, path: &str) -> AuditResult<String> {
        let value = self
            .reader
            .link(path)
            .map_err(|_| "runtime_executable_unavailable")?;
        self.budget(value)
    }

    fn groups(&mut self, root: &str) -> AuditResult<BTreeMap<String, BTreeSet<u32>>> {
        let mut pending = vec![(root.to_owned(), 0)];
        let mut groups = BTreeMap::new();
        let mut all_pids = BTreeSet::new();
        while let Some((group, depth)) = pending.pop() {
            if depth > MAX_DEPTH || groups.len() + pending.len() >= MAX_GROUPS {
                return Err("too_many_cgroups");
            }
            let path = format!("{}{group}", self.layout.mountpoint());
            let pids = parse_pids(&self.text(&format!("{path}/cgroup.procs"))?)?;
            for pid in &pids {
                if !all_pids.insert(*pid) {
                    return Err("process_seen_in_multiple_cgroups");
                }
            }
            if all_pids.len() > MAX_PROCESSES {
                return Err("too_many_processes");
            }
            let children = self
                .reader
                .directories(&path)
                .map_err(|_| "runtime_cgroup_unavailable")?;
            if children.len() > MAX_GROUPS {
                return Err("too_many_cgroups");
            }
            let mut unique_children = BTreeSet::new();
            for child in children {
                if !safe_component(&child) || !unique_children.insert(child.clone()) {
                    return Err("unsafe_or_duplicate_cgroup_child");
                }
                pending.push((format!("{group}/{child}"), depth + 1));
            }
            if groups.insert(group, pids).is_some() {
                return Err("duplicate_cgroup");
            }
        }
        Ok(groups)
    }

    fn identity(&mut self, pid: u32, group: &str) -> AuditResult<(u64, String)> {
        let start = start_time(pid, &self.text(&format!("/proc/{pid}/stat"))?)?;
        let memberships = self.text(&format!("/proc/{pid}/cgroup"))?;
        let mut selected = memberships.lines().filter_map(|line| {
            let mut fields = line.splitn(3, ':');
            let (id, controllers, path) = (fields.next()?, fields.next()?, fields.next()?);
            match self.layout {
                CgroupLayout::UnifiedV2 if id == "0" && controllers.is_empty() => Some(path),
                CgroupLayout::SystemdV1
                    if !id.is_empty()
                        && id.bytes().all(|b| b.is_ascii_digit())
                        && controllers.split(',').any(|v| v == "name=systemd") =>
                {
                    Some(path)
                }
                _ => None,
            }
        });
        if selected.next() != Some(group) || selected.next().is_some() {
            return Err("process_cgroup_identity_mismatch");
        }
        let exe = self.link(&format!("/proc/{pid}/exe"))?;
        if !exe.starts_with('/') || exe.ends_with('/') || exe.contains(['\n', '\r', '\0']) {
            return Err("invalid_executable_identity");
        }
        Ok((start, exe))
    }
}

fn safe_component(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 255
        && value != "."
        && value != ".."
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_.@:\\".contains(&b))
}

fn validate_group(unit: &str, group: &str) -> AuditResult<()> {
    let Some(relative) = group.strip_prefix('/') else {
        return Err("invalid_service_cgroup");
    };
    if group.len() > 4096
        || !relative.split('/').all(safe_component)
        || relative.rsplit('/').next() != Some(unit)
    {
        return Err("invalid_service_cgroup");
    }
    Ok(())
}

fn parse_pids(value: &str) -> AuditResult<BTreeSet<u32>> {
    let mut pids = BTreeSet::new();
    for line in value.lines() {
        if line.is_empty() || !line.bytes().all(|b| b.is_ascii_digit()) {
            return Err("invalid_cgroup_pid");
        }
        let pid = line.parse::<u32>().map_err(|_| "invalid_cgroup_pid")?;
        if pid == 0 {
            return Err("invalid_cgroup_pid");
        }
        pids.insert(pid);
        if pids.len() > MAX_PROCESSES {
            return Err("too_many_processes");
        }
    }
    Ok(pids)
}

fn start_time(pid: u32, value: &str) -> AuditResult<u64> {
    if !value.starts_with(&format!("{pid} (")) {
        return Err("process_stat_identity_mismatch");
    }
    // comm may contain spaces and parentheses. Field 22 follows the LAST closing ')'.
    let (_, fields) = value.rsplit_once(") ").ok_or("invalid_process_stat")?;
    let value = fields
        .split_whitespace()
        .nth(19)
        .ok_or("invalid_process_stat")?;
    if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
        return Err("invalid_process_start_time");
    }
    value.parse().map_err(|_| "invalid_process_start_time")
}

fn runtime_name(exe: &str) -> &'static str {
    let exe = exe.strip_suffix(" (deleted)").unwrap_or(exe);
    let name = exe.rsplit('/').next().unwrap_or_default();
    if ["python", "pypy"].iter().any(|prefix| {
        name.strip_prefix(prefix)
            .is_some_and(|suffix| suffix.bytes().all(|b| b.is_ascii_digit() || b == b'.'))
    }) {
        "python_candidate"
    } else if ["sh", "bash", "dash", "zsh", "ksh", "fish", "env"].contains(&name) {
        "shell_unverified"
    } else {
        "other_unverified"
    }
}

fn mapped_runtime(value: &str) -> AuditResult<(bool, bool)> {
    if value.is_empty() {
        return Err("process_maps_empty");
    }
    let hex = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_hexdigit());
    let mut python = false;
    let mut deleted = false;
    for line in value.lines() {
        let mut parts = line.split_whitespace();
        let (Some(address), Some(perms), Some(offset), Some(device), Some(inode)) = (
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
        ) else {
            return Err("invalid_process_maps");
        };
        let permissions = perms.as_bytes();
        if !address
            .split_once('-')
            .is_some_and(|(a, b)| hex(a) && hex(b))
            || permissions.len() != 4
            || !b"r-".contains(&permissions[0])
            || !b"w-".contains(&permissions[1])
            || !b"x-".contains(&permissions[2])
            || !b"ps".contains(&permissions[3])
            || !hex(offset)
            || !device
                .split_once(':')
                .is_some_and(|(a, b)| hex(a) && hex(b))
            || inode.is_empty()
            || !inode.bytes().all(|b| b.is_ascii_digit())
        {
            return Err("invalid_process_maps");
        }
        let path = parts.collect::<Vec<_>>().join(" ");
        deleted |= path.ends_with(" (deleted)");
        let path = path.strip_suffix(" (deleted)").unwrap_or(&path);
        let name = path.rsplit('/').next().unwrap_or_default();
        python |= name.starts_with("libpython") || name.starts_with("libpypy");
    }
    Ok((python, deleted))
}

/// Observe service cgroups recursively and recheck membership plus PID/start/executable.
/// This detects common collection races, not every exec/dlopen/short-lived subprocess.
pub fn collect_runtime(
    units: &[Unit],
    reader: &mut impl RuntimeReader,
) -> AuditResult<RuntimeReport> {
    if units.is_empty() {
        return Err("empty_runtime_scope");
    }
    let layout = reader
        .layout()
        .map_err(|_| "runtime_cgroup_layout_unavailable")?;
    let mut context = Context {
        reader,
        layout,
        bytes: 0,
        started: Instant::now(),
    };
    let mut result = RuntimeReport {
        layout,
        processes: Vec::new(),
        unobserved: 0,
    };
    let mut all_pids = BTreeSet::new();
    for unit in units {
        let Some(service) = &unit.service else {
            continue;
        };
        let group = service
            .control_group
            .as_deref()
            .ok_or("missing_service_cgroup")?;
        if group.is_empty() {
            if service.pid != 0 || !["inactive", "failed"].contains(&unit.active.as_str()) {
                return Err("running_service_without_cgroup");
            }
            result.unobserved += 1;
            continue;
        }
        validate_group(&unit.name, group)?;
        let before = context.groups(group)?;
        if service.pid != 0 && !before.values().any(|pids| pids.contains(&service.pid)) {
            return Err("main_pid_missing_from_cgroup");
        }
        let mut identities = Vec::new();
        for (subgroup, pids) in &before {
            for &pid in pids {
                if !all_pids.insert(pid) || all_pids.len() > MAX_PROCESSES {
                    return Err("duplicate_or_excessive_runtime_processes");
                }
                let (start, exe) = context.identity(pid, subgroup)?;
                let (embedded_python, deleted_mapping) =
                    mapped_runtime(&context.text(&format!("/proc/{pid}/maps"))?)?;
                result.processes.push(Process {
                    unit: unit.name.clone(),
                    pid,
                    start,
                    runtime: runtime_name(&exe),
                    embedded_python,
                    deleted_executable: exe.ends_with(" (deleted)"),
                    deleted_mapping,
                });
                identities.push((pid, subgroup, start, exe));
            }
        }
        if identities.is_empty() {
            result.unobserved += 1;
        }
        if before != context.groups(group)? {
            return Err("cgroup_changed_during_observation");
        }
        for (pid, subgroup, start, exe) in identities {
            if context.identity(pid, subgroup)? != (start, exe) {
                return Err("process_changed_during_observation");
            }
        }
    }
    Ok(result)
}

/// Only validated unit names, numeric identities and fixed findings leave the collector.
pub fn render_runtime(report: &RuntimeReport) -> String {
    let mut out = format!(
        "scope=user_service_cgroup_procfs\ncgroup_layout={}\ncutover_authorized=false\nfull_runtime_verification=not_performed\npoint_in_time_only=true\nprocesses_observed={}\nservices_without_process_observation={}\npython_candidates={}\ndeleted_executables={}\nprocesses_with_deleted_mappings={}\nunit\tpid\tstart_ticks\truntime\tmapped_runtime\texecutable_deleted\tmapping_deleted\n",
        report.layout.label(), report.processes.len(), report.unobserved,
        report.processes.iter().filter(|p| p.runtime == "python_candidate" || p.embedded_python).count(),
        report.processes.iter().filter(|p| p.deleted_executable).count(),
        report.processes.iter().filter(|p| p.deleted_mapping).count(),
    );
    for p in &report.processes {
        writeln!(
            &mut out,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            p.unit,
            p.pid,
            p.start,
            p.runtime,
            if p.embedded_python {
                "embedded_python_candidate"
            } else {
                "no_named_python_mapping_observed"
            },
            p.deleted_executable,
            p.deleted_mapping
        )
        .expect("writing to String");
    }
    out
}
