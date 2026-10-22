#![forbid(unsafe_code)]

pub mod author;
pub use brain_compare::{
    compare_artifact, hero_compare_render, hero_curve_artifact, hero_curve_release,
};
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

pub fn require_maintenance_enabled() -> anyhow::Result<()> {
    anyhow::bail!("Die bisherige Dokumentpflege und Kandidatenaktivierung sind deaktiviert.")
}

#[cfg(test)]
mod frozen_tests {
    #[test]
    fn maintenance_is_frozen_without_configuration_or_io() {
        assert!(super::require_maintenance_enabled().is_err());
    }

    #[tokio::test]
    async fn runner_rejects_before_loading_configuration_secrets_or_git() {
        let runtime = serde_json::from_str(include_str!(
            "../../../../ops/brain-maintenance/runtime.example.json"
        ))
        .unwrap();
        let result = super::integration::runner::Runner::open(runtime).await;
        let error = result.err().expect("retired runner must reject startup");
        assert_eq!(
            error.to_string(),
            super::require_maintenance_enabled()
                .unwrap_err()
                .to_string()
        );
    }
}
