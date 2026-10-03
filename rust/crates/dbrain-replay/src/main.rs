#![forbid(unsafe_code)]
use dbrain_replay::validation;
use dbrain_replay::{ReplayDecoder, ReplayFailure, ReplayRequest, WorkerDecoder};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    process::ExitCode,
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
fn write_json(value: &impl serde::Serialize) -> Result<(), ReplayFailure> {
    let mut output = std::io::stdout().lock();
    serde_json::to_writer(&mut output, value).map_err(|_| ReplayFailure::InputIo)?;
    output.flush().map_err(|_| ReplayFailure::InputIo)
}
fn run() -> Result<ExitCode, ReplayFailure> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() == 1 && args[0] == "--worker" {
        dbrain_replay::worker_stdio()?;
        return Ok(ExitCode::SUCCESS);
    }
    if args.len() == 1 && args[0] == "manifest" {
        write_json(&serde_json::json!({
            "contract": dbrain_replay::CONTRACT_VERSION,
            "parser_revision": dbrain_replay::parser_revision(),
            "haste_revision": dbrain_replay::HASTE_REVISION,
            "schema_revision": dbrain_replay::SCHEMA_REVISION,
            "dungers_revision": dbrain_replay::DUNGERS_REVISION,
            "capabilities": dbrain_replay::capabilities(),
            "real_replay_verified": false,
            "coaching_eligible": false,
        }))?;
        return Ok(ExitCode::SUCCESS);
    }
    if args.len() != 3 || (args[0] != "decode" && args[0] != "validate") {
        eprintln!(
            "Aufruf: dbrain-replay-worker manifest | <decode|validate> <lokale-Demo> <freigegebene-Anfrage.json>"
        );
        return Err(ReplayFailure::InvalidRequest);
    }
    let executable = std::env::current_exe().map_err(|_| ReplayFailure::InputIo)?;
    let decoder = WorkerDecoder::new(executable);
    if args[0] == "validate" {
        let config = validation::private_input_path(Path::new(&args[2]))?;
        let request = request_file(&config)?;
        dbrain_replay::validate_request(&request)?;
        if request.source.expected_sha256.is_none() {
            return Err(ReplayFailure::InvalidRequest);
        }
        let raw = validation::private_input_path(Path::new(&args[1]))?;
        write_json(&validation::validate(&raw, &request, &decoder)?)?;
        return Ok(ExitCode::from(2));
    }
    let request = request_file(Path::new(&args[2]))?;
    write_json(&decoder.decode(Path::new(&args[1]), &request)?)?;
    Ok(ExitCode::SUCCESS)
}
fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(reason) => {
            eprintln!("replay_quarantined:{reason}");
            ExitCode::from(2)
        }
    }
}
