//! Alle Text-KI-Aufrufe nutzen die zentral geprüfte Fireworks-Auswahl.
//! Der tägliche Katalogabruf und die Probe gehören ausschließlich zum Resolver-Dienst.

use crate::{CoreError, Result};

/// Liest die atomare Auswahl vor jedem Aufruf neu, damit laufende Clients wechseln.
/// Fehlende oder ungültige Auswahl wird gemeldet, niemals durch ein ungeprüftes Modell ersetzt.
pub fn model_for_request() -> Result<String> {
    fireworks_model_selection::selected_model()
        .map_err(|error| CoreError::ModelSelection(error.to_string()))
}
