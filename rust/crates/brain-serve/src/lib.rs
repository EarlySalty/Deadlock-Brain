#![forbid(unsafe_code)]
//! Composition root for the integrated Rust core. No legacy transports or schema writes.

pub mod config;
mod health;
mod secrets;
mod service;

pub use config::Config;
pub use secrets::Secrets;
pub use service::{run, Prepared};

/// Deliberately contains only static, allowlisted diagnostics, never dependency errors or input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Arguments,
    ConfigMissing,
    ConfigIo,
    ConfigSyntax,
    ConfigInvalid(&'static str),
    SecretMissing(&'static str),
    SecretInvalid(&'static str),
    ProviderConfig,
    ReaderConfig,
    Runtime,
    Signal,
    DatabaseUnavailable,
    DatabasePermissions,
    ReleaseUnavailable,
    KnowledgeVersion,
    SchemaIncompatible,
    ReaderUnavailable,
    StartupTimeout,
    Bind,
    Serve,
    ShutdownTimeout,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigInvalid(section) => write!(f, "invalid_config:{section}"),
            Self::SecretMissing(role) => write!(f, "required_secret_missing:{role}"),
            Self::SecretInvalid(role) => write!(f, "invalid_secret:{role}"),
            other => f.write_str(match other {
                Self::Arguments => "invalid_arguments",
                Self::ConfigMissing => "config_missing",
                Self::ConfigIo => "config_unreadable",
                Self::ConfigSyntax => "config_schema_invalid",
                Self::ProviderConfig => "provider_configuration_failed",
                Self::ReaderConfig => "reader_configuration_failed",
                Self::Runtime => "runtime_failed",
                Self::Signal => "signal_registration_failed",
                Self::DatabaseUnavailable => "database_unavailable",
                Self::DatabasePermissions => "database_permissions_missing",
                Self::ReleaseUnavailable => "release_unavailable",
                Self::KnowledgeVersion => "knowledge_version_mismatch",
                Self::SchemaIncompatible => "core_schema_incompatible",
                Self::ReaderUnavailable => "reader_unavailable",
                Self::StartupTimeout => "startup_timeout",
                Self::Bind => "bind_failed",
                Self::Serve => "server_failed",
                Self::ShutdownTimeout => "shutdown_timeout",
                _ => unreachable!(),
            }),
        }
    }
}
impl std::error::Error for Error {}

pub fn log_event(event: &'static str) {
    eprintln!("{}", serde_json::json!({"event": event}));
}
