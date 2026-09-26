use brain_runtime_audit::runtime::{collect_runtime, render_runtime, RuntimeReader};
use brain_runtime_audit::{parse_unit, AuditResult, Unit};
use std::collections::{BTreeMap, VecDeque};

const NAME: &str = "deadlock-brain-test.service";
const GROUP: &str =
    "/user.slice/user-1000.slice/user@1000.service/app.slice/deadlock-brain-test.service";
const MAPS: &str = "00400000-00401000 r-xp 00000000 08:01 42 /private/NEVER_PRINT_THIS/brain\n";

#[derive(Default)]
struct Reader {
    texts: BTreeMap<String, VecDeque<String>>,
    links: BTreeMap<String, VecDeque<String>>,
    dirs: BTreeMap<String, Vec<String>>,
    calls: Vec<String>,
}

fn next(values: &mut BTreeMap<String, VecDeque<String>>, path: &str) -> AuditResult<String> {
    let sequence = values.get_mut(path).ok_or("NEVER_PRINT_THIS")?;
    if sequence.len() > 1 {
        Ok(sequence.pop_front().unwrap())
    } else {
        sequence.front().cloned().ok_or("NEVER_PRINT_THIS")
    }
}

impl RuntimeReader for Reader {
    fn text(&mut self, path: &str) -> AuditResult<String> {
        self.calls.push(path.to_owned());
        next(&mut self.texts, path)
    }
    fn link(&mut self, path: &str) -> AuditResult<String> {
        self.calls.push(path.to_owned());
        next(&mut self.links, path)
    }
    fn directories(&mut self, path: &str) -> AuditResult<Vec<String>> {
        self.calls.push(path.to_owned());
        self.dirs.get(path).cloned().ok_or("NEVER_PRINT_THIS")
    }
}

fn unit(pid: u32, group: Option<&str>) -> Unit {
    let group = group
        .map(|v| format!("ControlGroup={v}\n"))
        .unwrap_or_default();
    parse_unit(NAME, &format!("Id={NAME}\nLoadState=loaded\nActiveState={}\nSubState={}\nUnitFileState=static\nResult=success\nExecMainStatus=0\nMainPID={pid}\nMemoryMax=infinity\nCPUQuotaPerSecUSec=infinity\nTasksMax=infinity\nTimeoutStopUSec=90s\nExecStart={{ path=/bin/bash ; }}\n{group}", if pid == 0 { "inactive" } else { "active" }, if pid == 0 { "dead" } else { "running" })).unwrap()
}

fn stat(pid: u32, start: u64) -> String {
    format!(
        "{pid} (private (NEVER_PRINT_THIS)) S {} {start} 0 0\n",
        ["0"; 18].join(" ")
    )
}

impl Reader {
    fn set(&mut self, path: &str, value: &str) {
        self.texts
            .insert(path.to_owned(), [value.to_owned()].into());
    }
    fn process(&mut self, pid: u32, group: &str, exe: &str) {
        self.set(&format!("/proc/{pid}/stat"), &stat(pid, 123));
        self.set(&format!("/proc/{pid}/cgroup"), &format!("0::{group}\n"));
        self.set(&format!("/proc/{pid}/maps"), MAPS);
        self.links
            .insert(format!("/proc/{pid}/exe"), [exe.to_owned()].into());
    }
    fn group(&mut self, group: &str, pids: &str, children: &[&str]) {
        let path = format!("/sys/fs/cgroup{group}");
        self.set(&format!("{path}/cgroup.procs"), pids);
        self.dirs
            .insert(path, children.iter().map(|v| (*v).to_owned()).collect());
    }
}

fn fixture() -> Reader {
    let mut r = Reader::default();
    r.group(GROUP, "101\n", &[]);
    r.process(101, GROUP, "/private/NEVER_PRINT_THIS/brain");
    r
}

fn report(r: &mut Reader) -> AuditResult<String> {
    collect_runtime(&[unit(101, Some(GROUP))], r).map(|v| render_runtime(&v))
}

#[test]
fn native_process_is_observed_without_claiming_rust_or_cutover() {
    let output = report(&mut fixture()).unwrap();
    for text in [
        "processes_observed=1",
        "other_unverified",
        "cutover_authorized=false",
        "full_runtime_verification=not_performed",
        "point_in_time_only=true",
    ] {
        assert!(output.contains(text), "missing {text}");
    }
    assert!(!output.contains("NEVER_PRINT_THIS"));
    assert!(!output.contains("/private"));
}

#[test]
fn nested_cgroup_python_child_is_not_hidden_by_native_main() {
    let mut r = fixture();
    let child = format!("{GROUP}/workers");
    r.group(GROUP, "101\n", &["workers"]);
    r.group(&child, "102\n", &[]);
    r.process(102, &child, "/opt/venv/bin/python3.12");
    let output = report(&mut r).unwrap();
    assert!(output.contains("processes_observed=2"));
    assert!(output.contains("python_candidate"));
    assert!(output.contains("python_candidates=1"));
}

#[test]
fn embedded_python_mapping_is_visible() {
    let mut r = fixture();
    r.set(
        "/proc/101/maps",
        "00400000-00401000 r-xp 00000000 08:01 42 /private/libpython3.12.so.1.0\n",
    );
    let output = report(&mut r).unwrap();
    assert!(output.contains("python_candidates=1"));
    assert!(output.contains("mapped_python_candidate"));
}

#[test]
fn deleted_executable_and_library_are_reported_without_paths() {
    let mut r = fixture();
    r.links.insert(
        "/proc/101/exe".to_owned(),
        ["/private/NEVER_PRINT_THIS/brain (deleted)".to_owned()].into(),
    );
    r.set(
        "/proc/101/maps",
        &(MAPS.trim_end().to_owned() + " (deleted)\n"),
    );
    let output = report(&mut r).unwrap();
    assert!(output.contains("deleted_executables=1"));
    assert!(output.contains("processes_with_deleted_mappings=1"));
    assert!(!output.contains("NEVER_PRINT_THIS"));
}

#[test]
fn duplicate_pids_within_one_cgroup_are_deduplicated() {
    let mut r = fixture();
    r.group(GROUP, "101\n101\n", &[]);
    assert!(report(&mut r).unwrap().contains("processes_observed=1"));
}

#[test]
fn inactive_service_is_unobserved_not_python_free() {
    let output =
        render_runtime(&collect_runtime(&[unit(0, Some(""))], &mut Reader::default()).unwrap());
    assert!(output.contains("services_without_process_observation=1"));
    assert!(output.contains("full_runtime_verification=not_performed"));
}

#[test]
fn empty_scope_is_an_error() {
    assert!(collect_runtime(&[], &mut Reader::default()).is_err());
}

#[test]
fn active_service_without_cgroup_fails() {
    for group in [None, Some("")] {
        assert!(collect_runtime(&[unit(101, group)], &mut fixture()).is_err());
    }
}

#[test]
fn unsafe_or_broader_group_is_rejected() {
    for group in [
        "/",
        "/user.slice",
        "/../deadlock-brain-test.service",
        "/user.slice//deadlock-brain-test.service",
        "/user.slice/./deadlock-brain-test.service",
        "/user.slice/other.service",
    ] {
        let mut r = fixture();
        assert!(collect_runtime(&[unit(101, Some(group))], &mut r).is_err());
        assert!(r.calls.is_empty());
    }
}

#[test]
fn cgroup_membership_change_invalidates_snapshot() {
    let mut r = fixture();
    r.texts.insert(
        format!("/sys/fs/cgroup{GROUP}/cgroup.procs"),
        ["101\n".to_owned(), "101\n102\n".to_owned()].into(),
    );
    assert!(report(&mut r).is_err());
}

#[test]
fn recycled_pid_is_rejected() {
    let mut r = fixture();
    r.texts.insert(
        "/proc/101/stat".to_owned(),
        [stat(101, 123), stat(101, 456)].into(),
    );
    assert!(report(&mut r).is_err());
}

#[test]
fn migrated_or_foreign_process_is_rejected() {
    let mut r = fixture();
    r.set("/proc/101/cgroup", "0::/user.slice/other.service\n");
    assert!(report(&mut r).is_err());
}

#[test]
fn executable_change_is_rejected() {
    let mut r = fixture();
    r.links.insert(
        "/proc/101/exe".to_owned(),
        ["/bin/brain".to_owned(), "/usr/bin/python3".to_owned()].into(),
    );
    assert!(report(&mut r).is_err());
}

#[test]
fn missing_or_invalid_maps_never_becomes_clean_success() {
    let mut r = fixture();
    r.texts.remove("/proc/101/maps");
    assert!(report(&mut r).is_err());
    for maps in [
        "",
        "NEVER_PRINT_THIS",
        "00400000-00401000 garbage 0 08:01 42 /lib/native.so\n",
    ] {
        r.set("/proc/101/maps", maps);
        assert!(report(&mut r).is_err());
    }
}

#[test]
fn malformed_or_excessive_pid_list_is_rejected() {
    for pids in [
        "0\n".to_owned(),
        "-1\n".to_owned(),
        "NEVER_PRINT_THIS\n".to_owned(),
        "101 102\n".to_owned(),
        (1..=4097).map(|n| format!("{n}\n")).collect(),
    ] {
        let mut r = fixture();
        r.group(GROUP, &pids, &[]);
        assert!(report(&mut r).is_err());
    }
}

#[test]
fn unsafe_child_directory_is_rejected_before_traversal() {
    let mut r = fixture();
    r.group(GROUP, "101\n", &["../other.service"]);
    assert!(report(&mut r).is_err());
    assert!(!r.calls.iter().any(|p| p.contains("..")));
}

#[test]
fn raw_reader_errors_are_redacted() {
    let error = report(&mut Reader::default()).unwrap_err();
    assert!(!error.contains("NEVER_PRINT_THIS"));
}

#[test]
fn process_collection_never_reads_command_lines_or_environment() {
    let mut r = fixture();
    report(&mut r).unwrap();
    assert!(r.calls.iter().any(|p| p == "/proc/101/maps"));
    assert!(r
        .calls
        .iter()
        .all(|p| !p.ends_with("/environ") && !p.ends_with("/cmdline")));
}

#[test]
fn main_pid_must_be_in_the_observed_group() {
    let mut r = fixture();
    r.group(GROUP, "102\n", &[]);
    r.process(102, GROUP, "/bin/brain");
    assert!(report(&mut r).is_err());
}

#[test]
fn host_systemd_v1_mount_is_detected() {
    use brain_runtime_audit::runtime::{detect_layout, CgroupLayout};
    assert_eq!(detect_layout("7690 6723 0:27 / /sys/fs/cgroup/systemd rw,nosuid - cgroup cgroup rw,xattr,name=systemd\n").unwrap(), CgroupLayout::SystemdV1);
}

#[test]
fn unified_v2_mount_is_detected() {
    use brain_runtime_audit::runtime::{detect_layout, CgroupLayout};
    assert_eq!(
        detect_layout("29 20 0:26 / /sys/fs/cgroup rw,nosuid - cgroup2 cgroup rw\n").unwrap(),
        CgroupLayout::UnifiedV2
    );
}

#[test]
fn unknown_ambiguous_or_namespace_limited_mounts_are_rejected() {
    use brain_runtime_audit::runtime::detect_layout;
    for mounts in [
        "",
        "29 20 0:26 / /sys/fs/cgroup rw - tmpfs tmpfs rw\n",
        "29 20 0:26 /user.slice /sys/fs/cgroup rw - cgroup2 cgroup rw\n",
        "29 20 0:26 / /arbitrary/private/root rw - cgroup2 cgroup rw\n",
        "29 20 0:26 / /sys/fs/cgroup/systemd rw - cgroup cgroup rw,name=systemd-other\n",
        "29 20 0:26 / /sys/fs/cgroup rw - cgroup2 cgroup rw\n30 20 0:27 / /sys/fs/cgroup/systemd rw - cgroup cgroup rw,name=systemd\n",
    ] {
        assert!(detect_layout(mounts).is_err());
    }
}

struct V1Reader {
    inner: Reader,
    controller: &'static str,
}

impl RuntimeReader for V1Reader {
    fn layout(&mut self) -> AuditResult<brain_runtime_audit::runtime::CgroupLayout> {
        Ok(brain_runtime_audit::runtime::CgroupLayout::SystemdV1)
    }
    fn text(&mut self, path: &str) -> AuditResult<String> {
        let mapped = path.replace("/sys/fs/cgroup/systemd/", "/sys/fs/cgroup/");
        if path.starts_with("/sys/fs/cgroup/") && mapped == path {
            return Err("wrong_v1_mount");
        }
        let text = self.inner.text(&mapped)?;
        if path.ends_with("/cgroup") {
            Ok(text.replace("0::", &format!("5:{}:", self.controller)))
        } else {
            Ok(text)
        }
    }
    fn link(&mut self, path: &str) -> AuditResult<String> {
        self.inner.link(path)
    }
    fn directories(&mut self, path: &str) -> AuditResult<Vec<String>> {
        let mapped = path.replace("/sys/fs/cgroup/systemd/", "/sys/fs/cgroup/");
        if mapped == path {
            return Err("wrong_v1_mount");
        }
        self.inner.directories(&mapped)
    }
}

#[test]
fn systemd_v1_processes_use_the_named_hierarchy() {
    let mut inner = fixture();
    inner.process(101, GROUP, "/usr/bin/python3");
    let output = render_runtime(
        &collect_runtime(
            &[unit(101, Some(GROUP))],
            &mut V1Reader {
                inner,
                controller: "name=systemd",
            },
        )
        .unwrap(),
    );
    assert!(output.contains("cgroup_layout=systemd_v1"));
    assert!(output.contains("python_candidates=1"));
}

#[test]
fn matching_path_in_wrong_v1_controller_is_not_membership_proof() {
    for controller in ["cpu", "name=systemd-other"] {
        assert!(collect_runtime(
            &[unit(101, Some(GROUP))],
            &mut V1Reader {
                inner: fixture(),
                controller
            }
        )
        .is_err());
    }
}

#[test]
fn deleted_python_and_pypy_executables_are_candidates() {
    for exe in [
        "python",
        "python3.12",
        "python3.13t",
        "python3.12d",
        "pypy3",
        "pypy3.10",
    ] {
        let mut r = fixture();
        r.process(
            101,
            GROUP,
            &format!("/private/NEVER_PRINT_THIS/{exe} (deleted)"),
        );
        let output = report(&mut r).unwrap();
        assert!(output.contains("python_candidates=1"), "{exe}");
        assert!(output.contains("deleted_executables=1"));
        assert!(!output.contains("NEVER_PRINT_THIS"));
    }
}

#[test]
fn renamed_deleted_interpreter_is_detected_from_stat_comm() {
    let mut r = fixture();
    r.process(
        101,
        GROUP,
        "/private/NEVER_PRINT_THIS/removed-runtime (deleted)",
    );
    r.set(
        "/proc/101/stat",
        &format!("101 (python3.13t) S {} 123 0 0\n", ["0"; 18].join(" ")),
    );
    let output = report(&mut r).unwrap();
    assert!(output.contains("python_candidates=1"));
    assert!(!output.contains("NEVER_PRINT_THIS"));
}

#[test]
fn deleted_python_mappings_cover_embedded_and_renamed_runtimes() {
    for name in ["libpython3.12.so.1.0", "libpypy3-c.so", "python3.13t"] {
        let mut r = fixture();
        r.process(101, GROUP, "/private/NEVER_PRINT_THIS/host (deleted)");
        r.set(
            "/proc/101/maps",
            &format!("00400000-00401000 r-xp 00000000 08:01 42 /private/{name} (deleted)\n"),
        );
        let output = report(&mut r).unwrap();
        assert!(output.contains("python_candidates=1"), "{name}");
        assert!(output.contains("processes_with_deleted_mappings=1"));
        assert!(output.contains("mapped_python_candidate"));
    }
}

#[test]
fn misleading_names_and_nonexecutable_python_files_are_not_candidates() {
    for name in [
        "python-helper",
        "python3.txt",
        "pypython",
        "libpython-not-a-library.so",
        "python.",
    ] {
        let mut r = fixture();
        r.process(101, GROUP, &format!("/private/{name} (deleted)"));
        r.set(
            "/proc/101/maps",
            &format!("00400000-00401000 r-xp 00000000 08:01 42 /private/{name} (deleted)\n"),
        );
        assert!(
            report(&mut r).unwrap().contains("python_candidates=0"),
            "{name}"
        );
    }
    let mut r = fixture();
    r.set(
        "/proc/101/maps",
        "00400000-00401000 r--p 00000000 08:01 42 /private/python3 (deleted)\n",
    );
    assert!(report(&mut r).unwrap().contains("python_candidates=0"));
}

#[test]
fn deleted_runtime_collection_reads_only_allowlisted_metadata() {
    let mut r = fixture();
    r.process(101, GROUP, "/private/python3 (deleted)");
    report(&mut r).unwrap();
    for path in r.calls.iter().filter(|p| p.starts_with("/proc/")) {
        assert!(["stat", "cgroup", "exe", "maps"].contains(&path.rsplit('/').next().unwrap()));
    }
}
