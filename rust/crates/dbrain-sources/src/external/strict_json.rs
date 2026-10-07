use serde_json::Value;

pub(super) fn parse(raw: &[u8]) -> Result<Value, serde_json::Error> {
    brain_contracts::provider_input::parse_unique_json(raw)
}
