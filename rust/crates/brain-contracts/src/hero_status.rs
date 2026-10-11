use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeroStatus {
    Released,
    InDevelopment,
    HeroLabs,
}

impl HeroStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Released => "released",
            Self::InDevelopment => "in_development",
            Self::HeroLabs => "hero_labs",
        }
    }
}

pub fn hero_status(payload: &Map<String, Value>) -> Option<HeroStatus> {
    if let Some(key) = payload.get("Key").and_then(Value::as_str) {
        if !hero_class(key) {
            return None;
        }
        if let Some(catalog) = payload.get("_hero_catalog").and_then(Value::as_object) {
            if catalog.get("class_name").and_then(Value::as_str) != Some(key) {
                return None;
            }
            let status = catalog_status(catalog)?;
            return Some(
                if payload.get("InHeroLabs").and_then(Value::as_bool) == Some(true) {
                    HeroStatus::HeroLabs
                } else if status == HeroStatus::Released
                    && [
                        ("IsDisabled", false),
                        ("IsSelectable", true),
                        ("InHeroLabs", false),
                    ]
                    .iter()
                    .any(|(field, expected)| {
                        payload
                            .get(*field)
                            .is_some_and(|value| value.as_bool() != Some(*expected))
                    })
                {
                    HeroStatus::InDevelopment
                } else {
                    status
                },
            );
        }
        let development = payload.get("InDevelopment").and_then(Value::as_bool) == Some(true);
        let labs = payload.get("InHeroLabs").and_then(Value::as_bool) == Some(true);
        let named_ability = payload
            .get("BoundAbilities")
            .and_then(Value::as_object)
            .is_some_and(|abilities| {
                abilities.values().any(|ability| {
                    ability["Name"]
                        .as_str()
                        .is_some_and(|name| !name.trim().is_empty())
                })
            });
        return (named_ability && (development || labs)).then_some(if labs {
            HeroStatus::HeroLabs
        } else {
            HeroStatus::InDevelopment
        });
    }
    catalog_status(payload)
}

pub fn is_released_hero(payload: &Value) -> bool {
    payload.as_object().and_then(hero_status) == Some(HeroStatus::Released)
}

fn hero_class(class: &str) -> bool {
    class
        .strip_prefix("hero_")
        .is_some_and(|suffix| !suffix.trim().is_empty())
}

fn catalog_status(payload: &Map<String, Value>) -> Option<HeroStatus> {
    let id = payload.get("id")?.as_u64()?;
    let class = payload.get("class_name")?.as_str()?;
    let name = payload.get("name")?.as_str()?.trim();
    if id == 0 || id > i64::MAX as u64 || !hero_class(class) || name.is_empty() || name == class {
        return None;
    }
    let labs = ["in_hero_labs", "InHeroLabs"]
        .iter()
        .any(|field| payload.get(*field).and_then(Value::as_bool) == Some(true));
    Some(if labs {
        HeroStatus::HeroLabs
    } else if payload.get("player_selectable").and_then(Value::as_bool) == Some(true)
        && payload.get("disabled").and_then(Value::as_bool) == Some(false)
        && [
            "prerelease_only",
            "assigned_players_only",
            "in_hero_labs",
            "InHeroLabs",
        ]
        .iter()
        .all(|field| match payload.get(*field) {
            None => true,
            Some(value) => value.as_bool() == Some(false),
        })
    {
        HeroStatus::Released
    } else {
        HeroStatus::InDevelopment
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn catalog(class: &str, name: &str, selectable: bool, disabled: bool) -> Value {
        json!({"id":88,"class_name":class,"name":name,"player_selectable":selectable,
            "disabled":disabled,"in_development":true})
    }

    #[test]
    fn nonhero_entities_do_not_receive_hero_status() {
        for class in ["upgrade_fixture", "ability_fixture", "", "hero_", "hero_ "] {
            let catalog = catalog(class, "Fixture", true, false);
            assert_eq!(hero_status(catalog.as_object().unwrap()), None);
            let raw = json!({"Key":class,"InDevelopment":true,
                "BoundAbilities":{"1":{"Name":"Fixture Ability"}}});
            assert_eq!(hero_status(raw.as_object().unwrap()), None);
        }
    }

    #[test]
    fn release_uses_playability_instead_of_the_development_flag() {
        let baba = catalog("hero_baba", "Baba", true, false);
        assert!(is_released_hero(&baba));
        for (class, name) in [("hero_bomber", "Bomber"), ("hero_boho", "Boho")] {
            let hero = catalog(class, name, false, true);
            assert_eq!(
                hero_status(hero.as_object().unwrap()),
                Some(HeroStatus::InDevelopment)
            );
            assert!(!is_released_hero(&hero));
        }
    }

    #[test]
    fn placeholders_and_unrepresented_forms_are_not_independent_heroes() {
        let test = catalog("hero_testhero", "hero_testhero", false, true);
        assert_eq!(hero_status(test.as_object().unwrap()), None);
        for key in [
            "hero_genericperson",
            "hero_targetdummy",
            "hero_werewolf_transformed",
        ] {
            let hero = json!({"Key":key,"Name":"A display name","IsSelectable":true,
                "IsDisabled":false,"InDevelopment":false,"BoundAbilities":{}});
            assert_eq!(hero_status(hero.as_object().unwrap()), None);
        }
        let transformed = json!({"Key":"hero_werewolf_transformed","Name":"Silver",
            "IsSelectable":true,"IsDisabled":false,"InDevelopment":false,"InHeroLabs":false,
            "BoundAbilities":{"1":{"Name":"Slash"},"2":{"Name":"Wild Leap"},
                "3":{"Name":"Takedown"},"4":{"Name":"Blood Frenzy"}}});
        assert_eq!(hero_status(transformed.as_object().unwrap()), None);
        let generic = json!({"Key":"hero_genericperson","InDevelopment":true,
            "BoundAbilities":{"1":{"Name":null}}});
        assert_eq!(hero_status(generic.as_object().unwrap()), None);
    }

    #[test]
    fn uncatalogued_development_heroes_remain_answerable_but_cannot_publish() {
        for (key, name) in [
            ("hero_airheart", "Rocket Launcher"),
            ("hero_tokamak", "Hot Shot"),
        ] {
            let hero = json!({"Key":key,"InDevelopment":true,
                "BoundAbilities":{"1":{"Name":name}}});
            assert_eq!(
                hero_status(hero.as_object().unwrap()),
                Some(HeroStatus::InDevelopment)
            );
            assert!(!is_released_hero(&hero));
        }
    }

    #[test]
    fn source_identity_and_labs_are_preserved_without_granting_release() {
        let mut raw = json!({"Key":"hero_baba","InDevelopment":true,
            "IsDisabled":false,"InHeroLabs":false,"_hero_catalog":catalog("hero_baba", "Baba", true, false)});
        assert!(is_released_hero(&raw));
        for (field, value) in [
            ("IsDisabled", json!(true)),
            ("IsSelectable", json!(false)),
            ("InHeroLabs", json!("false")),
        ] {
            let mut blocked = raw.clone();
            blocked[field] = value;
            assert!(!is_released_hero(&blocked), "{field}");
        }
        raw["InHeroLabs"] = json!(true);
        assert_eq!(
            hero_status(raw.as_object().unwrap()),
            Some(HeroStatus::HeroLabs)
        );
        raw["_hero_catalog"]["class_name"] = json!("hero_other");
        assert_eq!(hero_status(raw.as_object().unwrap()), None);
        for field in ["in_hero_labs", "InHeroLabs"] {
            let mut labs = catalog("hero_baba", "Baba", true, false);
            labs[field] = json!(true);
            assert_eq!(
                hero_status(labs.as_object().unwrap()),
                Some(HeroStatus::HeroLabs)
            );
            assert!(!is_released_hero(&labs));
        }
        for field in [
            "player_selectable",
            "disabled",
            "prerelease_only",
            "assigned_players_only",
            "in_hero_labs",
            "InHeroLabs",
        ] {
            let mut malformed = catalog("hero_baba", "Baba", true, false);
            malformed[field] = json!("false");
            assert!(!is_released_hero(&malformed), "{field}");
            malformed[field] = Value::Null;
            assert!(!is_released_hero(&malformed), "{field}");
        }
    }
}
