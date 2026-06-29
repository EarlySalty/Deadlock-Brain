#![forbid(unsafe_code)]

//! Gemeinsamer Kern fuer den schrittweisen Rust-Port von Deadlock-Brain.
//!
//! Diese Crate enthaelt nur Querschnittsvertraege: Pfade/Config, SQLite,
//! HTTP/MiniMax und gemeinsame Datenmodelle. Feature-Logik lebt in den
//! nachgelagerten `dbrain-*`-Crates.

pub mod build_narration;
pub mod config;
pub mod db;
pub mod error;
pub mod http;
pub mod minimax;
pub mod models;
pub mod schema;

pub use error::{CoreError, Result};
