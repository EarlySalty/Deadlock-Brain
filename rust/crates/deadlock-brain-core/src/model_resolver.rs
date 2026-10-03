//! Text-KI-Aufrufe nutzen das freigegebene Modell aus der normalen Konfiguration.

use crate::Result;

pub fn model_for_request() -> Result<String> {
    Ok(crate::config::load_ai_settings()?.model)
}
