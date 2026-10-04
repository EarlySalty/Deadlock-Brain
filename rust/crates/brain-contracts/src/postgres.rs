use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres {
    pub socket_dir: PathBuf,
    pub port: u16,
    pub username: String,
    pub database: String,
    pub auth: DatabaseAuth,
    pub password_env: Option<String>,
    pub max_connections: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseAuth {
    Peer,
    Password,
}
