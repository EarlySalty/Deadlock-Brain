use dbrain_s12_wiki_probe::{analyze, compare, model::MAX_INPUT_BYTES, Result};
use serde::Serialize;
use std::{
    env,
    fs::File,
    io::{Read, Write},
    path::Path,
    process::ExitCode,
};

const USAGE: &str = "S12 offline wiki preparation (no network, database, provider or publication)\n\n  dbrain-s12-wiki-probe analyze CAPTURE.json [--require-production-ready]\n  dbrain-s12-wiki-probe delta BEFORE.json AFTER.json\n\nExit: 0 = analysis completed (NOT production acceptance), 2 = invalid capture,\n      3 = production acceptance requested but blocked.\n";

fn read_capture(path: &Path) -> Result<Vec<u8>> {
    let metadata = std::fs::symlink_metadata(path).map_err(|_| "cannot inspect input file")?;
    if !metadata.is_file() || metadata.len() > MAX_INPUT_BYTES as u64 {
        return Err("input must be a regular bounded file (no symlink/device/pipe)".into());
    }
    let file = File::open(path).map_err(|_| "cannot open input file")?;
    let mut bytes = Vec::new();
    file.take(MAX_INPUT_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "cannot read input file")?;
    if bytes.len() > MAX_INPUT_BYTES {
        return Err("input byte budget exceeded".into());
    }
    Ok(bytes)
}

fn print_json(value: &impl Serialize) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|_| "report serialization failed")?;
    if bytes.len() > 32 * 1024 * 1024 {
        return Err("report byte budget exceeded".into());
    }
    let mut out = std::io::stdout().lock();
    out.write_all(&bytes)
        .and_then(|_| out.write_all(b"\n"))
        .map_err(|_| "cannot write report".into())
}

fn run() -> Result<u8> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.is_empty() || (args.len() == 1 && (args[0] == "--help" || args[0] == "-h")) {
        print!("{USAGE}");
        return Ok(0);
    }
    match args[0].to_str() {
        Some("analyze")
            if args.len() == 2 || (args.len() == 3 && args[2] == "--require-production-ready") =>
        {
            let report = analyze(&read_capture(Path::new(&args[1]))?)?;
            let blocked = args.len() == 3 && !report.production_publishable;
            print_json(&report)?;
            Ok(if blocked { 3 } else { 0 })
        }
        Some("delta") if args.len() == 3 => {
            let before = analyze(&read_capture(Path::new(&args[1]))?)?;
            let after = analyze(&read_capture(Path::new(&args[2]))?)?;
            print_json(&compare(&before, &after)?)?;
            Ok(0)
        }
        _ => Err("invalid arguments; use --help".into()),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("S12: {error}");
            ExitCode::from(2)
        }
    }
}
