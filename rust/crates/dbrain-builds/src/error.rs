use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildEngineError {
    #[error("Held nicht gefunden: {0}")]
    HeroNotFound(String),
    #[error("Build-Daten fuer Hero-ID {0} fehlen. Bitte zuerst `pull build-data --hero {0}` ausfuehren.")]
    MissingBuildData(i64),
    #[error("Ungueltiger Playstyle: {0}")]
    InvalidPlaystyle(String),
    #[error("API-Antwort enthaelt keine erwartete Hero-Zeile fuer Hero-ID {0}")]
    MissingHeroStats(i64),
}
