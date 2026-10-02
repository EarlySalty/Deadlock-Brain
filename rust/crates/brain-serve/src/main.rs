#![forbid(unsafe_code)]
use brain_serve::{log_event, Config, Error, Prepared, Secrets};
use std::{
    path::PathBuf,
    process::ExitCode,
    time::{Duration, Instant},
};

fn config_paths() -> Result<Option<(PathBuf, PathBuf)>, Error> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    match args.as_slice() {
        [arg] if arg == "--help" || arg == "-h" => {
            println!("brain-serve --config /pfad/brain-serve.json --infisical-config /pfad/infisical.json\n\nBeide normalen Konfigurationsdateien sind ausdrücklich anzugeben. Geheimnisse kommen aus dem vorhandenen Infisical-Snapshot. Die historischen *_env-Felder benennen dort Geheimnisse, keine Umgebungsvariablen.\nGET /healthz, GET /readyz, POST /v1/answer, POST /v1/retrieve. SIGTERM/SIGINT beenden den Dienst geordnet.");
            Ok(None)
        }
        [arg] if arg == "--version" => {
            println!("brain-serve {}", env!("CARGO_PKG_VERSION"));
            Ok(None)
        }
        [config, path, infisical, secret_path]
            if config == "--config"
                && infisical == "--infisical-config"
                && !path.is_empty()
                && !secret_path.is_empty() =>
        {
            Ok(Some((path.into(), secret_path.into())))
        }
        [infisical, secret_path, config, path]
            if config == "--config"
                && infisical == "--infisical-config"
                && !path.is_empty()
                && !secret_path.is_empty() =>
        {
            Ok(Some((path.into(), secret_path.into())))
        }
        [] => Err(Error::ConfigMissing),
        _ => Err(Error::Arguments),
    }
}

fn execute() -> Result<(), Error> {
    let Some((path, infisical_path)) = config_paths()? else {
        return Ok(());
    };
    let config = Config::load(&path)?;
    let deadline = Instant::now() + Duration::from_millis(config.timeouts.startup_ms);
    let secrets = Secrets::load_until(&config, &infisical_path, deadline)?;
    let prepared = Prepared::new_until(config, secrets, deadline)?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .max_blocking_threads(66)
        .enable_all()
        .build()
        .map_err(|_| Error::Runtime)?;
    let result = runtime.block_on(brain_serve::run(&prepared));
    let remaining = prepared.remaining_shutdown();
    // HTTP drain already happens inside run(). Detached synchronous DB/provider workers must not
    // keep an unbound/startup-aborted process alive for the full service drain budget.
    runtime.shutdown_timeout(remaining.min(std::time::Duration::from_secs(2)));
    result?;
    if prepared.remaining_shutdown().is_zero() {
        return Err(Error::ShutdownTimeout);
    }
    log_event("stopped");
    Ok(())
}

fn main() -> ExitCode {
    // Dependency panic payloads may contain URLs, headers or database errors. Do not print them.
    std::panic::set_hook(Box::new(|_| log_event("panic_redacted")));
    match execute() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!(
                "{}",
                serde_json::json!({"event": "service_failed", "reason": error.to_string()})
            );
            ExitCode::FAILURE
        }
    }
}
