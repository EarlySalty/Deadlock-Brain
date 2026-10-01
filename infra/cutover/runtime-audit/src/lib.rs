//! Read-only S11 metadata collection, deliberately not a deployment or G4/G5 gate.
//! Only fixed systemctl queries are constructed. Raw properties/errors are never rendered.
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

pub type AuditResult<T> = Result<T, &'static str>;
pub const MAX_OUTPUT_BYTES: usize = 1_048_576;
const MAX_UNITS: usize = 256;
const COMMON_PROPERTIES: &str = "--property=Id,LoadState,ActiveState,SubState,UnitFileState";
const SERVICE_PROPERTIES: &str = "--property=Result,ExecMainStatus,MainPID,MemoryMax,CPUQuotaPerSecUSec,TasksMax,TimeoutStopUSec,ExecStart,ControlGroup";

#[derive(Debug, PartialEq, Eq)]
pub struct Unit {
    name: String,
    load: String,
    active: String,
    sub: String,
    installed: String,
    service: Option<Service>,
}

#[derive(Debug, PartialEq, Eq)]
struct Service {
    result: String,
    exit_status: u32,
    pid: u32,
    memory: &'static str,
    cpu: &'static str,
    tasks: &'static str,
    stop_timeout: &'static str,
    entrypoint: &'static str,
    control_group: Option<String>,
}

fn in_scope(name: &str) -> bool {
    if name == "dl-knowledge.service" {
        return true;
    }
    let Some(stem) = name
        .strip_suffix(".service")
        .or_else(|| name.strip_suffix(".timer"))
    else {
        return false;
    };
    stem == "deadlock-brain"
        || stem.starts_with("deadlock-brain-")
        || stem.starts_with("deadlock-brain@")
}

fn safe_name(name: &str) -> bool {
    in_scope(name)
        && name.len() <= 255
        && !name.contains("..")
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_.@".contains(&b))
}

/// Union installed and loaded units. Empty/oversized/unparseable scope is not success.
pub fn discover(loaded: &str, installed: &str) -> AuditResult<Vec<String>> {
    if loaded.len() > MAX_OUTPUT_BYTES || installed.len() > MAX_OUTPUT_BYTES {
        return Err("systemd_output_too_large");
    }
    let mut names = BTreeSet::new();
    for line in loaded.lines().chain(installed.lines()) {
        let line = line.trim().strip_prefix('●').unwrap_or(line.trim()).trim();
        let Some(name) = line.split_whitespace().next() else {
            continue;
        };
        if !in_scope(name) {
            continue;
        }
        if !safe_name(name) {
            return Err("unsafe_unit_name");
        }
        names.insert(name.to_owned());
        if names.len() > MAX_UNITS {
            return Err("too_many_units");
        }
    }
    if names.is_empty() {
        return Err("no_in_scope_units_found");
    }
    Ok(names.into_iter().collect())
}

fn one_of(value: &str, allowed: &[&str]) -> AuditResult<String> {
    if allowed.contains(&value) {
        Ok(value.to_owned())
    } else {
        Err("unknown_systemd_state")
    }
}

fn numeric(value: &str) -> AuditResult<u32> {
    if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
        return Err("invalid_numeric_property");
    }
    value.parse().map_err(|_| "invalid_numeric_property")
}

fn duration(value: &str) -> bool {
    !value.is_empty()
        && value.split_whitespace().all(|part| {
            ["us", "ms", "s", "min", "h", "d", "w", "month", "y"]
                .iter()
                .any(|suffix| {
                    part.strip_suffix(suffix).is_some_and(|number| {
                        !number.is_empty()
                            && number.bytes().all(|b| b.is_ascii_digit() || b == b'.')
                            && number
                                .parse::<f64>()
                                .is_ok_and(|n| n.is_finite() && n >= 0.0)
                    })
                })
        })
}

fn limit(value: &str, time: bool) -> AuditResult<&'static str> {
    if value == "infinity" {
        return Ok("unbounded");
    }
    let integer = !value.is_empty()
        && value.bytes().all(|b| b.is_ascii_digit())
        && value.parse::<u64>().is_ok();
    if integer || (time && duration(value)) {
        Ok("bounded")
    } else {
        Err("invalid_resource_property")
    }
}

fn entrypoint(value: &str) -> &'static str {
    // Do not retain the executable path, argv or any original property text.
    let Some(body) = value.trim().strip_prefix("{ path=") else {
        return "unknown_entrypoint";
    };
    if body.matches("{ path=").next().is_some() {
        return "unknown_entrypoint";
    }
    let Some(path) = body.split(';').next() else {
        return "unknown_entrypoint";
    };
    let Some(name) = path.trim().rsplit('/').next() else {
        return "unknown_entrypoint";
    };
    if name.is_empty()
        || !name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
    {
        return "unknown_entrypoint";
    }
    if ["python", "pypy"].iter().any(|prefix| {
        name.strip_prefix(prefix)
            .is_some_and(|suffix| suffix.bytes().all(|b| b.is_ascii_digit() || b == b'.'))
    }) {
        "python_candidate"
    } else if ["sh", "bash", "dash", "zsh", "ksh", "fish", "env"].contains(&name) {
        "shell_unverified"
    } else {
        // A different executable can still be a script, wrapper or spawn Python.
        "other_unverified"
    }
}

pub fn parse_unit(name: &str, properties: &str) -> AuditResult<Unit> {
    if !safe_name(name) {
        return Err("unsafe_unit_name");
    }
    if properties.len() > MAX_OUTPUT_BYTES {
        return Err("systemd_output_too_large");
    }
    const ALLOWED: &[&str] = &[
        "Id",
        "LoadState",
        "ActiveState",
        "SubState",
        "UnitFileState",
        "Result",
        "ExecMainStatus",
        "MainPID",
        "MemoryMax",
        "CPUQuotaPerSecUSec",
        "TasksMax",
        "TimeoutStopUSec",
        "ExecStart",
        "ControlGroup",
    ];
    let mut values = BTreeMap::new();
    for line in properties.lines() {
        if let Some((key, value)) = line.split_once('=') {
            if ALLOWED.contains(&key) && values.insert(key, value).is_some() {
                return Err("duplicate_systemd_property");
            }
        }
    }
    let get = |key| values.get(key).copied().ok_or("missing_systemd_property");
    if get("Id")? != name {
        return Err("unit_identity_mismatch");
    }
    let load = one_of(
        get("LoadState")?,
        &[
            "loaded",
            "not-found",
            "error",
            "bad-setting",
            "masked",
            "merged",
            "stub",
        ],
    )?;
    let active = one_of(
        get("ActiveState")?,
        &[
            "active",
            "reloading",
            "inactive",
            "failed",
            "activating",
            "deactivating",
            "maintenance",
            "refreshing",
        ],
    )?;
    let sub = one_of(
        get("SubState")?,
        &[
            "running",
            "dead",
            "waiting",
            "exited",
            "failed",
            "start-pre",
            "start",
            "start-post",
            "reload",
            "reload-signal",
            "reload-notify",
            "stop",
            "stop-watchdog",
            "stop-sigterm",
            "stop-sigkill",
            "stop-post",
            "final-watchdog",
            "final-sigterm",
            "final-sigkill",
            "auto-restart",
            "auto-restart-queued",
            "cleaning",
            "condition",
            "elapsed",
        ],
    )?;
    let installed = match get("UnitFileState")? {
        "" => "unknown".to_owned(),
        value => one_of(
            value,
            &[
                "enabled",
                "enabled-runtime",
                "linked",
                "linked-runtime",
                "alias",
                "masked",
                "masked-runtime",
                "static",
                "disabled",
                "indirect",
                "generated",
                "transient",
                "bad",
                "not-found",
            ],
        )?,
    };
    let service = if name.ends_with(".service") {
        Some(Service {
            result: one_of(
                get("Result")?,
                &[
                    "success",
                    "resources",
                    "timeout",
                    "exit-code",
                    "signal",
                    "core-dump",
                    "watchdog",
                    "start-limit-hit",
                    "exec-condition",
                    "oom-kill",
                    "protocol",
                    "skipped",
                ],
            )?,
            exit_status: numeric(get("ExecMainStatus")?)?,
            pid: numeric(get("MainPID")?)?,
            memory: limit(get("MemoryMax")?, false)?,
            cpu: limit(get("CPUQuotaPerSecUSec")?, true)?,
            tasks: limit(get("TasksMax")?, false)?,
            stop_timeout: limit(get("TimeoutStopUSec")?, true)?,
            entrypoint: entrypoint(get("ExecStart")?),
            control_group: values.get("ControlGroup").map(|v| (*v).to_owned()),
        })
    } else {
        None
    };
    Ok(Unit {
        name: name.to_owned(),
        load,
        active,
        sub,
        installed,
        service,
    })
}

/// Errors invalidate the entire observation. Never propagate subprocess stderr or argv.
pub fn collect(read: &mut impl FnMut(&[&str]) -> AuditResult<String>) -> AuditResult<Vec<Unit>> {
    let loaded = read(&[
        "--user",
        "--no-pager",
        "--no-legend",
        "--plain",
        "--all",
        "list-units",
    ])
    .map_err(|_| "loaded_unit_query_failed")?;
    let installed = read(&[
        "--user",
        "--no-pager",
        "--no-legend",
        "--plain",
        "list-unit-files",
    ])
    .map_err(|_| "installed_unit_query_failed")?;
    let mut units = Vec::new();
    for name in discover(&loaded, &installed)? {
        let mut args = vec!["--user", "--no-pager", "show", COMMON_PROPERTIES];
        if name.ends_with(".service") {
            args.push(SERVICE_PROPERTIES);
        }
        args.extend(["--", name.as_str()]);
        let properties = read(&args).map_err(|_| "unit_property_query_failed")?;
        units.push(parse_unit(&name, &properties)?);
    }
    Ok(units)
}

/// Plain TSV uses only validated identifiers, enums and numbers; never raw properties.
pub fn render(units: &[Unit]) -> String {
    let mut out = String::from(
        "scope=user_systemd_metadata_only\ncutover_authorized=false\nfull_runtime_verification=not_performed\n\
         unit\tload\tactive\tsub\tinstalled\tresult\texit\tpid\tmemory\tcpu\ttasks\tstop_timeout\tentrypoint\n",
    );
    for unit in units {
        write!(
            &mut out,
            "{}\t{}\t{}\t{}\t{}\t",
            unit.name, unit.load, unit.active, unit.sub, unit.installed
        )
        .expect("writing to String");
        if let Some(service) = &unit.service {
            writeln!(
                &mut out,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                service.result,
                service.exit_status,
                service.pid,
                service.memory,
                service.cpu,
                service.tasks,
                service.stop_timeout,
                service.entrypoint
            )
            .expect("writing to String");
        } else {
            out.push_str("-\t-\t-\t-\t-\t-\t-\t-\n");
        }
    }
    out
}

pub mod runtime;
