use brain_contracts::Principal;
use dbrain_replay::{
    ReplayDecoder, ReplayFailure, ReplayRequest, WorkerDecoder,
    import::{import_report, query_example},
};
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::{
    collections::BTreeSet,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use dbrain_replay::validation;

fn request(path: &Path) -> Result<ReplayRequest, ReplayFailure> {
    let mut bytes = Vec::new();
    File::open(path)
        .map_err(|_| ReplayFailure::InputIo)?
        .take(32_769)
        .read_to_end(&mut bytes)
        .map_err(|_| ReplayFailure::InputIo)?;
    if bytes.len() > 32_768 {
        return Err(ReplayFailure::InvalidRequest);
    }
    serde_json::from_slice(&bytes).map_err(|_| ReplayFailure::InvalidRequest)
}

async fn pool(
    args: &mut impl Iterator<Item = String>,
    read_only: bool,
) -> Result<PgPool, ReplayFailure> {
    match args.next().as_deref() {
        Some("--infisical") => {
            let config = args.next().ok_or(ReplayFailure::InvalidRequest)?;
            deadlock_brain_core::pg::pg_pool_from_config(Path::new(&config), read_only)
                .await
                .map_err(|_| ReplayFailure::StoreFailure)
        }
        Some("--peer-test") => {
            let socket = args.next().ok_or(ReplayFailure::InvalidRequest)?;
            let port: u16 = args
                .next()
                .ok_or(ReplayFailure::InvalidRequest)?
                .parse()
                .map_err(|_| ReplayFailure::InvalidRequest)?;
            let database = args.next().ok_or(ReplayFailure::InvalidRequest)?;
            if !Path::new(&socket).is_absolute() || !database.ends_with("_test") || port == 0 {
                return Err(ReplayFailure::InvalidRequest);
            }
            let socket = Path::new(&socket)
                .canonicalize()
                .map_err(|_| ReplayFailure::InvalidRequest)?;
            if !socket.starts_with("/tmp") || !socket.is_dir() {
                return Err(ReplayFailure::InvalidRequest);
            }
            let socket = socket.to_str().ok_or(ReplayFailure::InvalidRequest)?;
            let user = std::env::var("USER").map_err(|_| ReplayFailure::InvalidRequest)?;
            let options = PgConnectOptions::new()
                .host(socket)
                .port(port)
                .database(&database)
                .username(&user)
                .options([(
                    "default_transaction_read_only",
                    if read_only { "on" } else { "off" },
                )]);
            PgPoolOptions::new()
                .max_connections(2)
                .connect_with(options)
                .await
                .map_err(|_| ReplayFailure::StoreFailure)
        }
        _ => Err(ReplayFailure::InvalidRequest),
    }
}

async fn run() -> Result<(), ReplayFailure> {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("import") => {
            let raw = PathBuf::from(args.next().ok_or(ReplayFailure::InvalidRequest)?);
            let request_path = PathBuf::from(args.next().ok_or(ReplayFailure::InvalidRequest)?);
            let worker = PathBuf::from(args.next().ok_or(ReplayFailure::InvalidRequest)?);
            let request_path = validation::private_input_path(&request_path)?;
            let request = request(&request_path)?;
            dbrain_replay::validate_request(&request)?;
            if request.source.expected_sha256.is_none() {
                return Err(ReplayFailure::InvalidRequest);
            }
            let raw = validation::private_input_path(&raw)?;
            validation::outside_git(&std::env::temp_dir())?;
            let report = WorkerDecoder::new(worker).decode(&raw, &request)?;
            let pool = pool(&mut args, false).await?;
            if args.next().is_some() {
                return Err(ReplayFailure::InvalidRequest);
            }
            let receipt = import_report(&pool, &report, &request).await?;
            println!(
                "{}",
                serde_json::to_string(&receipt).map_err(|_| ReplayFailure::StoreFailure)?
            );
        }
        Some("query") => {
            let release = args.next().ok_or(ReplayFailure::InvalidRequest)?;
            let scope = args.next().ok_or(ReplayFailure::InvalidRequest)?;
            let pool = pool(&mut args, true).await?;
            if args.next().is_some() {
                return Err(ReplayFailure::InvalidRequest);
            }
            let principal = Principal {
                actor_id: "replay-local-cli".into(),
                channel: "local".into(),
                scopes: BTreeSet::from([scope]),
                provider_egress: BTreeSet::new(),
            };
            let example = query_example(&pool, &release, &principal).await?;
            println!(
                "{}",
                serde_json::to_string(&example).map_err(|_| ReplayFailure::StoreFailure)?
            );
        }
        _ => return Err(ReplayFailure::InvalidRequest),
    }
    Ok(())
}

fn main() {
    let result = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| ReplayFailure::StoreFailure)
        .and_then(|rt| rt.block_on(run()));
    if let Err(error) = result {
        eprintln!("Replay-Import oder interne Abfrage fehlgeschlagen: {error}");
        std::process::exit(2);
    }
}
