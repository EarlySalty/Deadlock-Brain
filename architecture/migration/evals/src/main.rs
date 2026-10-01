#![forbid(unsafe_code)]
use dbrain_s10_evals::*;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::Path,
    process::{Command, ExitCode},
};
mod baseline;
mod support_inventory;
const LIMIT: u64 = 4 * 1024 * 1024;

fn read_bytes(path: &Path) -> Result<Vec<u8>, String> {
    let file = fs::File::open(path).map_err(|_| "input cannot be opened")?;
    if !file
        .metadata()
        .map_err(|_| "input metadata unavailable")?
        .is_file()
    {
        return Err("input must be a regular file".into());
    }
    let mut bytes = vec![];
    file.take(LIMIT + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "input read failed")?;
    if bytes.len() as u64 > LIMIT {
        return Err("input exceeds 4 MiB limit".into());
    }
    Ok(bytes)
}
fn parse<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, String> {
    // Do not echo source text, credentials or a malformed JSON value into logs.
    serde_json::from_slice(bytes).map_err(|_| "invalid JSON or artifact schema".into())
}
fn write_new(path: &Path, value: &impl serde::Serialize) -> Result<(), String> {
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|_| "report serialization failed")?;
    bytes.push(b'\n');
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(path)
        .map_err(|_| "output exists or cannot be created")?;
    file.write_all(&bytes)
        .and_then(|_| file.sync_all())
        .map_err(|_| "report write failed".to_owned())
}
fn git_text(root: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .output()
        .map_err(|_| "git read failed")?;
    if !output.status.success() {
        return Err("git read failed".into());
    }
    String::from_utf8(output.stdout)
        .map(|s| s.trim().into())
        .map_err(|_| "git output invalid".into())
}
fn execute(args: &[String]) -> Result<u8, String> {
    match args {
        [command] if command == "help" || command == "--help" => {
            println!("S10 offline evaluation tools\n  design PROFILE CATALOG OUTPUT\n  assess PROFILE CATALOG RUN EXPECTED_SHA OUTPUT\n  baseline OUTPUT\n  support-inventory DOCS_REPO OUTPUT\nNo network, database access or production load. assess: 0=pass proposal, 2=blocked/failed; invalid input=64. No command authorizes release.");
            Ok(0)
        }
        [command, p, c, out] if command == "design" => {
            let pb = read_bytes(Path::new(p))?;
            let cb = read_bytes(Path::new(c))?;
            let profile: Profile = parse(&pb)?;
            let catalog: Catalog = parse(&cb)?;
            validate_catalog(&catalog)?;
            if profile.schema_version != 1 || profile.dataset_sha256 != digest(&cb) {
                return Err("profile/catalogue version or digest mismatch".into());
            }
            let report = json!({"schema_version":1,"kind":"test_design","case_count":catalog.cases.len(),"dataset_sha256":digest(&cb),"profile_sha256":digest(&pb),"profile_approved":profile.approved,"release_approved":false,"e2e_executed":false});
            write_new(Path::new(out), &report)?;
            Ok(0)
        }
        [command, p, c, r, sha, out] if command == "assess" => {
            let pb = read_bytes(Path::new(p))?;
            let cb = read_bytes(Path::new(c))?;
            let rb = read_bytes(Path::new(r))?;
            let profile: Profile = parse(&pb)?;
            let catalog: Catalog = parse(&cb)?;
            let run: Run = parse(&rb)?;
            let report = assess(&profile, &catalog, &run, sha, &digest(&pb), &digest(&cb));
            let code = if report.status == "pass" { 0 } else { 2 };
            write_new(Path::new(out), &report)?;
            Ok(code)
        }
        [command, out] if command == "baseline" => {
            write_new(Path::new(out), &baseline::measure()?)?;
            Ok(0)
        }
        [command, repo, out] if command == "support-inventory" => {
            write_new(
                Path::new(out),
                &support_inventory::inventory(Path::new(repo))?,
            )?;
            Ok(0)
        }
        _ => Err("invalid arguments; use --help".into()),
    }
}
fn main() -> ExitCode {
    match execute(&std::env::args().skip(1).collect::<Vec<_>>()) {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("S10: {error}");
            ExitCode::from(64)
        }
    }
}
