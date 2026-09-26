//! Reviewed source configuration. Loading or validating it never fetches a ref,
//! selects HEAD, contacts a database, or reads credentials.
use std::{fs::File, io::Read, path::Path};

use serde::{Deserialize, Serialize};

use crate::{deadlock_data::PARSER_REVISION, external::IR_VERSION, Result, SourcesError};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourcePins {
    pub schema_version: u32,
    pub deadlock_data: DeadlockDataPin,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeadlockDataPin {
    pub commit: String,
    pub parser_revision: String,
    /// Expected SourceIr schema, not an invented upstream schema version.
    pub schema_version: Option<u32>,
    /// Optional exact ClientVersion from this commit's data/version.txt.
    pub data_version: Option<String>,
}

impl SourcePins {
    pub fn read(path: &Path) -> Result<Self> {
        let mut bytes = Vec::new();
        File::open(path)?.take(65_537).read_to_end(&mut bytes)?;
        if bytes.len() > 65_536 {
            return Err(SourcesError::invalid_input(
                "source pin configuration exceeds 64 KiB",
            ));
        }
        // Reject duplicate JSON keys as well as unknown config fields.
        let value = crate::external::parse_json_strict(&bytes)?;
        let config: Self = serde_json::from_value(value)?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1 {
            return Err(SourcesError::invalid_input(
                "unsupported source pin configuration schema_version; expected 1",
            ));
        }
        self.deadlock_data.validate()
    }
}

impl DeadlockDataPin {
    pub fn validate(&self) -> Result<()> {
        crate::git_source::validate_commit(&self.commit)?;
        if self.parser_revision != PARSER_REVISION {
            return Err(SourcesError::invalid_input(format!(
                "deadlock_data.parser_revision missing or unsupported; this binary requires {PARSER_REVISION}"
            )));
        }
        if self
            .schema_version
            .is_some_and(|version| version != IR_VERSION)
        {
            return Err(SourcesError::invalid_input(
                "deadlock_data.schema_version does not match the SourceIr schema",
            ));
        }
        if self.data_version.as_ref().is_some_and(|value| {
            value.is_empty()
                || value.len() > 128
                || value.trim() != value
                || value.chars().any(char::is_control)
        }) {
            return Err(SourcesError::invalid_input(
                "deadlock_data.data_version must be an exact nonempty ClientVersion",
            ));
        }
        Ok(())
    }

    pub fn verify_data_version(&self, actual: Option<&str>) -> Result<()> {
        if self
            .data_version
            .as_deref()
            .is_some_and(|expected| Some(expected) != actual)
        {
            return Err(SourcesError::invalid_input(
                "deadlock_data.data_version does not match the pinned commit",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn config() -> serde_json::Value {
        json!({"schema_version": 1, "deadlock_data": {
            "commit": "a".repeat(40), "parser_revision": PARSER_REVISION,
            "schema_version": IR_VERSION, "data_version": "123"
        }})
    }

    #[test]
    fn valid_pin_roundtrips_and_version_checks_are_enforced() {
        let pin: SourcePins = serde_json::from_value(config()).unwrap();
        pin.validate().unwrap();
        pin.deadlock_data.verify_data_version(Some("123")).unwrap();
        assert!(pin.deadlock_data.verify_data_version(Some("124")).is_err());
        assert!(pin.deadlock_data.verify_data_version(None).is_err());
        assert_eq!(serde_json::to_value(&pin).unwrap(), config());
    }

    #[test]
    fn invalid_and_missing_pins_fail_closed() {
        for field in ["commit", "parser_revision"] {
            let mut value = config();
            value["deadlock_data"]
                .as_object_mut()
                .unwrap()
                .remove(field);
            assert!(serde_json::from_value::<SourcePins>(value).is_err());
        }
        for commit in [
            "",
            "HEAD",
            "main",
            "aabbccdd",
            &"A".repeat(40),
            &"z".repeat(40),
        ] {
            let mut value = config();
            value["deadlock_data"]["commit"] = json!(commit);
            assert!(serde_json::from_value::<SourcePins>(value)
                .unwrap()
                .validate()
                .is_err());
        }
        let mut value = config();
        value["deadlock_data"]["parser_revision"] = json!("future-parser");
        assert!(serde_json::from_value::<SourcePins>(value)
            .unwrap()
            .validate()
            .is_err());
        let mut value = config();
        value["deadlock_data"]["schema_version"] = json!(999);
        assert!(serde_json::from_value::<SourcePins>(value)
            .unwrap()
            .validate()
            .is_err());
        let mut value = config();
        value["schema_version"] = json!(2);
        assert!(serde_json::from_value::<SourcePins>(value)
            .unwrap()
            .validate()
            .is_err());
    }

    #[test]
    fn configuration_is_bounded_and_rejects_duplicate_keys() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("pins.json");
        std::fs::write(&file, serde_json::to_vec(&config()).unwrap()).unwrap();
        SourcePins::read(&file).unwrap();
        std::fs::write(&file, b"{\"schema_version\":1,\"schema_version\":2}").unwrap();
        assert!(SourcePins::read(&file).is_err());
        std::fs::write(&file, vec![b' '; 65_537]).unwrap();
        assert!(SourcePins::read(&file).is_err());
    }
}
