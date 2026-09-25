//! Optional, offline S05 audit. Not linked into any production crate.
//! Compiles the exact pure function from the checked-out source, not a rewrite.
use std::{env, error::Error, fs, path::PathBuf, process::{Command, ExitCode}, time::{Duration, Instant}};
const SOURCE: &str = include_str!("../../../../rust/crates/dbrain-normalize/src/util.rs");
const FIXTURES: &str = include_str!("alias_cases.rs");
const CHECKS: &str = include_str!("checks.rs");

fn extract(source: &str) -> Result<&str, &'static str> {
    let marker = "pub fn normalize_alias(value: &str) -> String {\n";
    if source.matches(marker).count() != 1 { return Err("function signature missing or ambiguous"); }
    let start = source.find(marker).ok_or("function missing")?;
    let end = source[start..].find("\n}\n").ok_or("function boundary missing")?;
    Ok(&source[start..start + end + 3])
}

fn bounded(command: &mut Command) -> Result<std::process::ExitStatus, Box<dyn Error>> {
    let mut child = command.spawn()?;
    let started = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? { return Ok(status); }
        if started.elapsed() > Duration::from_secs(60) {
            child.kill()?;
            let _ = child.wait();
            return Err("audit subprocess exceeded 60-second limit".into());
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn run() -> Result<bool, Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let output = PathBuf::from(args.next().ok_or("usage: s05-probe OUTPUT_DIR --characterize|--strict")?);
    let mode = args.next().ok_or("missing --characterize or --strict")?;
    if args.next().is_some() || (mode != "--characterize" && mode != "--strict") {
        return Err("usage: s05-probe OUTPUT_DIR --characterize|--strict".into());
    }
    // A dedicated fresh directory prevents overwriting existing work or binaries.
    fs::create_dir(&output)?;
    let output = output.canonicalize()?;
    let source = output.join("alias_check.rs");
    let binary = output.join("alias-check");
    let function = extract(SOURCE)?;
    fs::write(&source, format!("#![forbid(unsafe_code)]\n{function}\n{FIXTURES}\n{CHECKS}"))?;
    let compiled = bounded(Command::new("rustc").arg("--edition=2021").arg("-Dwarnings").arg(&source).arg("-o").arg(&binary))?;
    if !compiled.success() { return Err("compilation failed; no parity result".into()); }
    let result = bounded(Command::new(&binary).arg(mode))?;
    Ok(result.success())
}
fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(error) => { eprintln!("S05 audit error: {error}"); ExitCode::from(2) }
    }
}

#[cfg(test)]
mod tests {
    use super::extract;
    #[test] fn extracts_actual_function() { assert!(extract(super::SOURCE).unwrap().contains(".to_lowercase()")); }
    #[test] fn rejects_missing_signature() { assert!(extract("fn other() {}\n").is_err()); }
    #[test] fn rejects_ambiguous_signature() { assert!(extract(&format!("{}{}", super::SOURCE, super::SOURCE)).is_err()); }
    #[test] fn rejects_missing_boundary() { assert!(extract("pub fn normalize_alias(value: &str) -> String {\n").is_err()); }
}
