use brain_storage::entity_profile::derivation::game_file_facts::{json_text, kv3};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

const BOXES: &[&str] = &[
    "citadel_breakable_prop_drop_gold",
    "citadel_breakable_prop_box_multi",
    "citadel_breakable_prop_tough_crate",
    "citadel_breakable_prop_wooden_crate",
];

#[derive(Debug, PartialEq)]
pub struct TimerValue {
    pub timer: &'static str,
    pub stat: &'static str,
    pub source_entity: String,
    pub numeric: String,
    pub locator_kind: &'static str,
    pub locator: String,
    pub value_sha256: [u8; 32],
    pub flags: Vec<String>,
}

pub fn accepts(source: &str, path: &str) -> bool {
    matches!(
        (source, path),
        (
            "deadlock-data",
            "data/json/misc-data.json" | "data/json/generic-data.json"
        ) | (
            "gametracking",
            "game/citadel/pak01_dir/scripts/misc.vdata"
                | "game/citadel/pak01_dir/scripts/generic_data.vdata"
        )
    )
}

struct Mapping {
    root: String,
    field: String,
    timer: &'static str,
    stat: &'static str,
    required: bool,
}

fn mappings(
    path: &str,
    normalize: &impl Fn(&str) -> Result<String, &'static str>,
) -> Result<Vec<Mapping>, &'static str> {
    let json = path.ends_with(".json");
    let mut values = Vec::new();
    let mut add = |root: &str,
                   prefix: &str,
                   timer,
                   fields: &[(&'static str, &'static str)]|
     -> Result<(), &'static str> {
        for (index, (field, stat)) in fields.iter().enumerate() {
            let field = if json {
                (*field).to_owned()
            } else {
                format!("{prefix}{field}")
            };
            values.push(Mapping {
                root: normalize(root)?,
                field: normalize(&field)?,
                timer,
                stat,
                required: index == 0 && (timer != "soul_box" || root == BOXES[0]),
            });
        }
        Ok(())
    };
    if path.ends_with("generic-data.json") || path.ends_with("generic_data.vdata") {
        add(
            if json { "RejuvParams" } else { "m_RejuvParams" },
            "m_fl",
            "rejuvenator",
            &[
                ("RejuvinatorBuffDuration", "timer_buff_duration_seconds"),
                ("RejuvinatorDropDuration", "timer_drop_duration_seconds"),
                (
                    "RejuvinatorExpirationWarningTiming",
                    "timer_expiration_warning_seconds",
                ),
            ],
        )?;
    } else {
        for root in BOXES {
            add(
                root,
                "m_fl",
                "soul_box",
                &[
                    ("InitialSpawnTime", "timer_first_spawn_seconds"),
                    ("RespawnTime", "timer_respawn_interval_seconds"),
                ],
            )?;
        }
        add(
            "neutral_camp_midboss",
            "m_i",
            "mid_boss",
            &[
                ("SpawnIntervalInSeconds", "timer_respawn_interval_seconds"),
                (
                    "SpawnIntervalChange",
                    "timer_respawn_interval_change_seconds",
                ),
                ("SpawnIntervalMin", "timer_respawn_interval_min_seconds"),
            ],
        )?;
    }
    Ok(values)
}

struct SourceKeys {
    original: String,
    occurrence: usize,
    fields: BTreeSet<String>,
}

fn parts(pointer: &str, json: bool) -> Option<(&str, usize, &str, bool)> {
    let mut parts = pointer.strip_prefix('/')?.split('/');
    let root = parts.next()?;
    let occurrence = if json { 0 } else { parts.next()?.parse().ok()? };
    let field = parts.next()?;
    if !json {
        parts.next()?.parse::<usize>().ok()?;
    }
    Some((root, occurrence, field, parts.next().is_none()))
}

pub fn parse(
    source: &str,
    path: &str,
    bytes: &[u8],
    max_input_bytes: usize,
    normalize: impl Fn(&str) -> Result<String, &'static str>,
) -> Result<Vec<TimerValue>, &'static str> {
    if !accepts(source, path) {
        return Ok(Vec::new());
    }
    if bytes.len() > max_input_bytes {
        return Err("raw_object_limit");
    }
    let input = std::str::from_utf8(bytes).map_err(|_| "invalid_utf8")?;
    let json = path.ends_with(".json");
    let input = if json {
        input
    } else {
        input.trim_start_matches('\u{feff}').trim_start()
    };
    let leaves = if json {
        json_text::parse(input)
    } else {
        kv3::parse(input)
    }
    .map_err(|code| {
        if code == "duplicate_json_keys_preserved_as_text" {
            "key_collision"
        } else {
            code
        }
    })?;
    let mappings = mappings(path, &normalize)?;
    let mut objects: BTreeMap<String, SourceKeys> = BTreeMap::new();
    let mut output = Vec::new();
    for leaf in leaves {
        let Some((root, occurrence, field, scalar)) = parts(&leaf.pointer, json) else {
            continue;
        };
        let root_key = normalize(root)?;
        let field_key = normalize(field)?;
        let mapping = mappings
            .iter()
            .find(|mapping| mapping.root == root_key && mapping.field == field_key);
        if !scalar {
            if mapping.is_some() {
                return Err("timer_property_type");
            }
            continue;
        }
        let object = objects.entry(root_key).or_insert_with(|| SourceKeys {
            original: root.to_owned(),
            occurrence,
            fields: BTreeSet::new(),
        });
        if object.original != root
            || object.occurrence != occurrence
            || !object.fields.insert(field_key)
        {
            return Err("key_collision");
        }
        let Some(mapping) = mapping else {
            continue;
        };
        if leaf.numeric_representation.is_none() {
            return Err("timer_property_type");
        }
        if !matches!(leaf.value, Value::Number(_) | Value::String(_)) {
            return Err("timer_property_type");
        }
        output.push(TimerValue {
            timer: mapping.timer,
            stat: mapping.stat,
            source_entity: root.to_owned(),
            value_sha256: Sha256::digest(leaf.lexeme.as_bytes()).into(),
            numeric: leaf.lexeme,
            locator_kind: if json { "json_pointer" } else { "kv_path" },
            locator: leaf.pointer,
            flags: leaf.flags,
        });
    }
    if mappings.iter().any(|mapping| {
        mapping.required
            && !objects
                .get(&mapping.root)
                .is_some_and(|object| object.fields.contains(&mapping.field))
    }) {
        return Err("timer_property_missing");
    }
    Ok(output)
}
