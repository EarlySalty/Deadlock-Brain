#![forbid(unsafe_code)]
use brain_serve::{log_event, Config, Error, Prepared, Secrets};
use std::{path::PathBuf, process::ExitCode};

fn config_path() -> Result<Option<PathBuf>, Error> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    match args.as_slice() {
        [arg] if arg == "--help" || arg == "-h" => {
            println!("brain-serve --config /path/to/brain-serve.json\n\nWithout --config, BRAIN_SERVE_CONFIG is required.\nSecrets are read only from environment names declared in the configuration.\nGET /healthz, GET /readyz, POST /v1/answer. SIGTERM/SIGINT drain the service.");
            Ok(None)
        }
        [arg] if arg == "--version" => {
            println!("brain-serve {}", env!("CARGO_PKG_VERSION"));
            Ok(None)
        }
        [arg, path] if arg == "--config" && !path.is_empty() => Ok(Some(path.into())),
        [] => std::env::var_os("BRAIN_SERVE_CONFIG")
            .filter(|p| !p.is_empty())
            .map(PathBuf::from)
            .map(Some)
            .ok_or(Error::ConfigMissing),
        _ => Err(Error::Arguments),
    }
}

fn execute() -> Result<(), Error> {
    let Some(path) = config_path()? else {
        return Ok(());
    };
    let config = Config::load(&path)?;
    let secrets = Secrets::from_environment(&config)?;
    let prepared = Prepared::new(config, secrets)?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .max_blocking_threads(18)
        .enable_all()
        .build()
        .map_err(|_| Error::Runtime)?;
    let result = runtime.block_on(brain_serve::run(&prepared));
    let remaining = prepared.remaining_shutdown();
    // Detached blocking API work cannot keep the process alive beyond the drain budget.
    runtime.shutdown_timeout(remaining);
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
