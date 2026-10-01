#![forbid(unsafe_code)]

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::ExitCode;

use s14_replay_audit::{audit, MAX_MANIFEST_BYTES};

fn read_manifest(path: &Path) -> Result<String, &'static str> {
    // Local trusted inventory paths only. This is not an adversarial filesystem
    // sandbox; a productive replay worker must supply its own descriptor/cgroup
    // isolation. Symlinks and special files are never accepted intentionally.
    let before = std::fs::symlink_metadata(path).map_err(|_| "manifest_unreadable")?;
    if !before.is_file() {
        return Err("manifest_not_regular");
    }
    if before.len() > MAX_MANIFEST_BYTES as u64 {
        return Err("manifest_too_large");
    }
    let file = File::open(path).map_err(|_| "manifest_unreadable")?;
    if !file
        .metadata()
        .map_err(|_| "manifest_unreadable")?
        .is_file()
    {
        return Err("manifest_not_regular");
    }
    let mut bytes = Vec::new();
    file.take((MAX_MANIFEST_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| "manifest_unreadable")?;
    if bytes.len() > MAX_MANIFEST_BYTES {
        return Err("manifest_too_large");
    }
    String::from_utf8(bytes).map_err(|_| "manifest_not_utf8")
}

fn run() -> Result<u8, String> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() == 1 && args[0] == "--help" {
        println!("Usage: s14-replay-audit CORPUS.tsv CAPABILITIES.tsv [--cutoff-epoch EPOCH]\nOffline metadata checks only; never grants implementation or release approval.\nExit 0: metadata complete, 2: evidence missing, 1: invalid input.\nManifest inputs must be trusted local regular files, at most 256 KiB each.");
        return Ok(0);
    }
    if args.len() != 2 && args.len() != 4 {
        return Err("invalid_arguments".into());
    }
    let cutoff = if args.len() == 4 {
        if args[2] != "--cutoff-epoch" {
            return Err("invalid_arguments".into());
        }
        let value = args[3].to_str().ok_or("invalid_cutoff")?;
        if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
            return Err("invalid_cutoff".into());
        }
        Some(value.parse::<u64>().map_err(|_| "invalid_cutoff")?)
    } else {
        None
    };
    let corpus = read_manifest(Path::new(&args[0]))?;
    let caps = read_manifest(Path::new(&args[1]))?;
    let report = audit(&corpus, &caps, cutoff).map_err(|e| e.to_string())?;
    println!("{}", report.json());
    Ok(if report.metadata_complete() { 0 } else { 2 })
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            // Only fixed error codes/table names/line numbers, never raw values,
            // input paths, OS error strings, replay identities, or credentials.
            eprintln!("s14_replay_audit:{error}");
            ExitCode::FAILURE
        }
    }
}
