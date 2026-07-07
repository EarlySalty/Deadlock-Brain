#![forbid(unsafe_code)]

//! Gemeinsamer Kern fuer den schrittweisen Rust-Port von Deadlock-Brain.
//!
//! Diese Crate enthaelt nur Querschnittsvertraege: Pfade/Config,
//! HTTP/MiniMax, Postgres und gemeinsame Datenmodelle. Feature-Logik lebt in den
//! nachgelagerten `dbrain-*`-Crates.

pub mod build_narration;
pub mod config;
pub mod error;
pub mod http;
pub mod minimax;
pub mod models;
pub mod pg;

pub use error::{CoreError, Result};

pub fn now_epoch_seconds() -> Result<i64> {
    use std::time::{SystemTime, UNIX_EPOCH};

    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64)
}
