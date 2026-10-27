use std::{
    io::{BufRead, BufReader, Read, Write},
    os::unix::process::ExitStatusExt,
    path::PathBuf,
    process::{Child, ChildStdin, Command, Stdio},
};

mod postmaster {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../test-support/postmaster.rs"
    ));
}

pub struct ScratchPg {
    pub directory: PathBuf,
    supervisor: Child,
    lifetime: Option<ChildStdin>,
    _ram: postmaster::RamDir,
}

fn test_name(name: &str) -> String {
    let module = module_path!().split_once("::").expect("test module").1;
    format!("{module}::{name}")
}

impl ScratchPg {
    pub fn start() -> Self {
        Self::start_at("brain-release-pg", 55439)
    }

    pub fn start_at(name: &str, port: u16) -> Self {
        let ram = postmaster::RamDir::new(name);
        let directory = ram.path().to_owned();
        let mut supervisor = Command::new(std::env::current_exe().unwrap())
            .args([
                "--ignored",
                "--exact",
                &test_name("supervise_cluster"),
                "--nocapture",
            ])
            .env("BRAIN_SCRATCH_PG_DIRECTORY", &directory)
            .env("BRAIN_SCRATCH_PG_PORT", port.to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
            .unwrap();
        let lifetime = supervisor.stdin.take();
        let mut instance = Self {
            directory,
            supervisor,
            lifetime,
            _ram: ram,
        };
        let started = std::time::Instant::now();
        while !instance.directory.join("ready").exists() {
            assert!(
                instance.supervisor.try_wait().unwrap().is_none(),
                "scratch postgres supervisor failed to start"
            );
            assert!(
                started.elapsed() < std::time::Duration::from_secs(30),
                "scratch postgres supervisor startup timed out"
            );
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        instance
    }
}

impl Drop for ScratchPg {
    fn drop(&mut self) {
        self.lifetime.take();
        let _ = self.supervisor.wait();
    }
}

struct ClusterDirectory(PathBuf);

impl Drop for ClusterDirectory {
    fn drop(&mut self) {
        if let Err(error) = std::fs::remove_dir_all(&self.0) {
            eprintln!("scratch postgres cleanup failed: {error}");
        }
    }
}

#[test]
#[ignore = "Internal scratch cluster supervisor"]
fn supervise_cluster() {
    let directory = PathBuf::from(std::env::var_os("BRAIN_SCRATCH_PG_DIRECTORY").unwrap());
    let port: u16 = std::env::var("BRAIN_SCRATCH_PG_PORT")
        .unwrap()
        .parse()
        .unwrap();
    let cluster = ClusterDirectory(directory);
    let data = cluster.0.join("data");
    let socket = cluster.0.join("socket");
    std::fs::create_dir(&socket).unwrap();
    assert!(Command::new("/usr/lib/postgresql/16/bin/initdb")
        .arg("-D")
        .arg(&data)
        .args([
            "-A",
            "trust",
            "-U",
            "brain_core_test",
            "--no-locale",
            "--encoding=UTF8"
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap()
        .success());
    let port = format!("port={port}");
    let _postmaster = postmaster::Postmaster::start(
        &data,
        &socket,
        &[&port, "shared_buffers=16MB"],
        Some(&cluster.0.join("postgres.log")),
    );
    std::fs::write(cluster.0.join("ready"), []).unwrap();
    let mut byte = [0];
    let _ = std::io::stdin().read(&mut byte);
}

#[test]
#[ignore = "Internal SIGKILL regression child"]
fn hold_cluster() {
    let pg = ScratchPg::start();
    let pid = std::fs::read_to_string(pg.directory.join("data/postmaster.pid")).unwrap();
    println!(
        "SCRATCH_PG_HOLD {} {}",
        pg.directory.display(),
        pid.lines().next().unwrap()
    );
    std::io::stdout().flush().unwrap();
    let mut byte = [0];
    let _ = std::io::stdin().read(&mut byte);
}

#[test]
fn sigkill_removes_postmaster_and_directory() {
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args([
            "--ignored",
            "--exact",
            &test_name("hold_cluster"),
            "--nocapture",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = child.stdout.take().unwrap();
    let ready = BufReader::new(output)
        .lines()
        .map(|line| line.unwrap())
        .find(|line| line.starts_with("SCRATCH_PG_HOLD "))
        .expect("cluster started");
    let mut fields = ready.split_whitespace().skip(1);
    let directory = PathBuf::from(fields.next().unwrap());
    let pid = fields.next().unwrap();
    let started = std::time::Instant::now();
    child.kill().unwrap();
    assert_eq!(child.wait().unwrap().signal(), Some(9));
    while directory.exists() && started.elapsed() < std::time::Duration::from_secs(10) {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    assert!(!directory.exists(), "cluster directory survived SIGKILL");
    if let Ok(stat) = std::fs::read_to_string(format!("/proc/{pid}/stat")) {
        assert_eq!(
            stat.split_once(") ").unwrap().1.split_whitespace().next(),
            Some("Z")
        );
    }
    eprintln!(
        "SIGKILL cleanup verified in {} ms",
        started.elapsed().as_millis()
    );
}
