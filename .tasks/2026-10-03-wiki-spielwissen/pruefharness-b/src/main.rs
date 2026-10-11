#[path = "../../../../rust/crates/dbrain-sources/src/game_files.rs"]
pub mod game_files;

use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    time::Instant,
};

fn report_resources(started: Instant, success: bool) -> io::Result<()> {
    let reader = BufReader::new(File::open("/proc/self/status")?);
    let mut peak_kib = None;
    for line in reader.lines() {
        let line = line?;
        if let Some(value) = line.strip_prefix("VmHWM:") {
            let mut fields = value.split_whitespace();
            let peak = fields
                .next()
                .and_then(|value| value.parse::<u64>().ok())
                .ok_or_else(|| io::Error::other("Ungültiger VmHWM-Wert"))?;
            if fields.next() != Some("kB") || fields.next().is_some() {
                return Err(io::Error::other("Ungültige VmHWM-Einheit"));
            }
            peak_kib = Some(peak);
            break;
        }
    }
    let peak_kib = peak_kib.ok_or_else(|| io::Error::other("VmHWM fehlt im Prozessstatus"))?;
    let mut stderr = io::stderr().lock();
    writeln!(
        stderr,
        "RESOURCE {}",
        serde_json::json!({
            "duration_seconds": started.elapsed().as_secs_f64(),
            "peak_rss_kib": peak_kib,
            "pid": std::process::id(),
            "success": success,
            "scope": "linux_process_vmhwm"
        })
    )?;
    stderr.flush()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let started = Instant::now();
    let result = run();
    let measurement = report_resources(started, result.is_ok());
    match (result, measurement) {
        (Err(error), Err(measurement)) => {
            Err(format!("{error}; Ressourcenmessung fehlgeschlagen: {measurement}").into())
        }
        (Err(error), Ok(())) => Err(error),
        (Ok(()), Err(error)) => Err(error.into()),
        (Ok(()), Ok(())) => Ok(()),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        return Err(
            "Aufruf: brain-game-files-pruefharness OPTIONS.json DOCUMENTS.jsonl INVENTORY.json"
                .into(),
        );
    }
    let options: game_files::GameFileOptions = serde_json::from_reader(File::open(&args[1])?)?;
    let output_file = File::options()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    let mut output = BufWriter::new(output_file);
    let extraction = game_files::extract_game_files(&options, &mut output);
    let flushed = output.flush();
    let inventory = match (extraction, flushed) {
        (Err(error), Err(flush)) => {
            return Err(format!("{error}; JSONL-Flush fehlgeschlagen: {flush}").into());
        }
        (Err(error), Ok(())) | (Ok(_), Err(error)) => return Err(error.into()),
        (Ok(inventory), Ok(())) => inventory,
    };
    let inventory_file = File::options()
        .write(true)
        .create_new(true)
        .open(&args[3])?;
    serde_json::to_writer_pretty(inventory_file, &inventory)?;
    let mut stdout = io::stdout().lock();
    writeln!(
        stdout,
        "documents={} facts={} unknown_revisions={} gaps={}",
        inventory.documents,
        inventory.facts,
        inventory.unknown_revisions,
        inventory.gaps.len()
    )?;
    stdout.flush()?;
    Ok(())
}
