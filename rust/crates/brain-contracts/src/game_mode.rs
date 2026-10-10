use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameMode {
    #[default]
    Normal,
    StreetBrawl,
}

impl GameMode {
    pub fn from_question(text: &str) -> Self {
        let terms = crate::lexical::terms(&text.replace('_', " "));
        if terms.windows(2).any(|pair| pair == ["street", "brawl"])
            || terms.iter().any(|term| term == "streetbrawl")
        {
            Self::StreetBrawl
        } else {
            Self::Normal
        }
    }

    pub fn api_value(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::StreetBrawl => "street_brawl",
        }
    }

    pub fn match_mode(self) -> &'static str {
        match self {
            Self::Normal => "ranked",
            Self::StreetBrawl => "unranked",
        }
    }

    pub fn population(self) -> &'static str {
        match self {
            Self::Normal => "Ranked, Standardmodus",
            Self::StreetBrawl => "Unranked, Street Brawl",
        }
    }

    pub fn allows_item(self, row: &Value) -> bool {
        let tier = row["item_tier"].as_i64();
        let brawl_only = tier == Some(5)
            || ["StreetBrawl", "street_brawl", "is_street_brawl"]
                .iter()
                .any(|key| row[*key].as_bool() == Some(true));
        row["type"] == "upgrade"
            && row["shopable"].as_bool() == Some(true)
            && ["disabled", "IsDisabled"]
                .iter()
                .all(|key| match row.get(*key) {
                    None | Some(Value::Null) => true,
                    Some(value) => value.as_bool() == Some(false),
                })
            && matches!(
                row["item_slot_type"].as_str(),
                Some("weapon" | "vitality" | "spirit")
            )
            && row["cost"].as_i64().is_some_and(|cost| cost > 0)
            && tier.is_some_and(|tier| (1..=5).contains(&tier))
            && (self == Self::StreetBrawl || !brawl_only)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn mode_requires_an_explicit_street_brawl_reference() {
        for text in [
            "Items für Abrams",
            "Shadow Strike",
            "Brawl",
            "Standardmodus",
        ] {
            assert_eq!(GameMode::from_question(text), GameMode::Normal);
        }
        for text in [
            "Items für Abrams in Street Brawl",
            "Street-Brawl",
            "street_brawl",
            "StreetBrawl",
        ] {
            assert_eq!(GameMode::from_question(text), GameMode::StreetBrawl);
        }
    }

    #[test]
    fn availability_uses_shop_fields_and_brawl_category_not_names() {
        let item = json!({"id":2319629810_i64,"name":"Shadow Strike","type":"upgrade",
            "item_slot_type":"vitality","item_tier":5,"cost":9999,"shopable":true});
        assert!(!GameMode::Normal.allows_item(&item));
        assert!(GameMode::StreetBrawl.allows_item(&item));
        let mut shared = item.clone();
        shared["name"] = json!("Different name");
        shared["item_tier"] = json!(2);
        assert!(GameMode::Normal.allows_item(&shared));
        assert!(GameMode::StreetBrawl.allows_item(&shared));
        for (field, value) in [
            ("type", json!("ability")),
            ("shopable", json!(false)),
            ("disabled", json!(true)),
            ("IsDisabled", json!(true)),
            ("disabled", json!("false")),
            ("item_slot_type", json!("component")),
            ("cost", json!(0)),
            ("item_tier", json!(6)),
        ] {
            let mut unavailable = shared.clone();
            unavailable[field] = value;
            for mode in [GameMode::Normal, GameMode::StreetBrawl] {
                assert!(!mode.allows_item(&unavailable), "{mode:?}: {field}");
            }
        }
        for field in ["type", "shopable", "cost", "item_tier", "item_slot_type"] {
            let mut missing = shared.clone();
            missing.as_object_mut().unwrap().remove(field);
            assert!(!GameMode::Normal.allows_item(&missing), "{field}");
        }
        shared["StreetBrawl"] = json!(true);
        assert!(!GameMode::Normal.allows_item(&shared));
        assert!(GameMode::StreetBrawl.allows_item(&shared));
    }
}
