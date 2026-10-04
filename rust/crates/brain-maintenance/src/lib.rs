#![forbid(unsafe_code)]

pub mod author;
pub mod config;
pub mod entity_profile_render;
pub mod html;
pub mod integration;
pub mod lease;
pub mod process;
pub mod scanner;
pub mod triage;

pub fn digest(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    hex::encode(Sha256::digest(bytes))
}
