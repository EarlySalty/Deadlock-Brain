#![forbid(unsafe_code)]
use dbrain_replay::{ReplayDecoder, ReplayFailure, ReplayRequest, WorkerDecoder};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

fn request_file(path: &Path) -> Result<ReplayRequest, ReplayFailure> {
    let mut data = Vec::new();
    File::open(path)
        .map_err(|_| ReplayFailure::InputIo)?
        .take(32_769)
        .read_to_end(&mut data)
        .map_err(|_| ReplayFailure::InputIo)?;
    if data.len() > 32_768 {
        return Err(ReplayFailure::InvalidRequest);
    }
    serde_json::from_slice(&data).map_err(|_| ReplayFailure::InvalidRequest)
}
fn run() -> Result<(), ReplayFailure> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() == 1 && args[0] == "--worker" {
        return dbrain_replay::worker_stdio();
    }
    if args.len() == 1 && args[0] == "manifest" {
        let manifest = serde_json::json!({
            "contract": dbrain_replay::CONTRACT_VERSION,
            "parser_revision": dbrain_replay::parser_revision(),
            "haste_revision": dbrain_replay::HASTE_REVISION,
            "schema_revision": dbrain_replay::SCHEMA_REVISION,
            "dungers_revision": dbrain_replay::DUNGERS_REVISION,
            "capabilities": dbrain_replay::capabilities(),
            "real_replay_verified": false,
            "coaching_eligible": false,
        });
        serde_json::to_writer(std::io::stdout().lock(), &manifest)
            .map_err(|_| ReplayFailure::InputIo)?;
        return Ok(());
    }
    if args.len() != 3 || args[0] != "decode" {
        eprintln!(
            "Usage: dbrain-replay-worker manifest | decode <local-raw-file> <authorized-request.json>"
        );
        return Err(ReplayFailure::InvalidRequest);
    }
    let request = request_file(Path::new(&args[2]))?;
    let executable = std::env::current_exe().map_err(|_| ReplayFailure::InputIo)?;
    let report = WorkerDecoder::new(executable).decode(Path::new(&args[1]), &request)?;
    serde_json::to_writer(std::io::stdout().lock(), &report).map_err(|_| ReplayFailure::InputIo)?;
    std::io::stdout()
        .flush()
        .map_err(|_| ReplayFailure::InputIo)?;
    Ok(())
}
fn main() {
    if let Err(reason) = run() {
        // Never print upstream error strings, raw bytes, player names, or private paths.
        eprintln!("replay_quarantined:{reason}");
        std::process::exit(2);
    }
}
