use super::{assertion, leaves as document_leaves, push_assertion, Assertion, Atom, Leaf};
use crate::git::Source;
use anyhow::{ensure, Context, Result};
use serde_json::json;

pub fn supported(source: Source, path: &str) -> bool {
    matches!(
        (source.key, path),
        (
            "deadlock-data",
            "data/json/misc-data.json" | "data/json/generic-data.json"
        ) | (
            "gametracking",
            "game/citadel/pak01_dir/scripts/generic_data.vdata"
                | "game/citadel/pak01_dir/scripts/misc.vdata"
        )
    )
}

fn field(path: &str, pointer: &str) -> Option<(&'static str, &'static str)> {
    if path.ends_with("generic-data.json") {
        return match pointer {
            "/RejuvParams/RejuvinatorBuffDuration" => {
                Some(("rejuvenator", "buff_duration_seconds"))
            }
            "/RejuvParams/RejuvinatorDropDuration" => {
                Some(("rejuvenator", "drop_duration_seconds"))
            }
            "/RejuvParams/RejuvinatorExpirationWarningTiming" => {
                Some(("rejuvenator", "expiration_warning_seconds"))
            }
            _ => None,
        };
    }
    if path.ends_with("generic_data.vdata") {
        return match pointer {
            "/m_RejuvParams/0/m_flRejuvinatorBuffDuration/0" => {
                Some(("rejuvenator", "buff_duration_seconds"))
            }
            "/m_RejuvParams/0/m_flRejuvinatorDropDuration/0" => {
                Some(("rejuvenator", "drop_duration_seconds"))
            }
            "/m_RejuvParams/0/m_flRejuvinatorExpirationWarningTiming/0" => {
                Some(("rejuvenator", "expiration_warning_seconds"))
            }
            _ => None,
        };
    }
    let (entity, field) = if path.ends_with(".json") {
        let mut parts = pointer.strip_prefix('/')?.split('/');
        let entity = parts.next()?;
        let field = parts.next()?;
        if parts.next().is_some() {
            return None;
        }
        (entity, field)
    } else {
        let mut parts = pointer.strip_prefix('/')?.split('/');
        let entity = parts.next()?;
        if parts.next()? != "0" {
            return None;
        }
        let field = parts.next()?;
        if parts.next()? != "0" || parts.next().is_some() {
            return None;
        }
        (entity, field)
    };
    match (entity, field) {
        (
            "citadel_breakable_prop_drop_gold"
            | "citadel_breakable_prop_box_multi"
            | "citadel_breakable_prop_tough_crate"
            | "citadel_breakable_prop_wooden_crate",
            "InitialSpawnTime" | "m_flInitialSpawnTime",
        ) => Some(("soul_box", "first_spawn_seconds")),
        (
            "citadel_breakable_prop_drop_gold"
            | "citadel_breakable_prop_box_multi"
            | "citadel_breakable_prop_tough_crate"
            | "citadel_breakable_prop_wooden_crate",
            "RespawnTime" | "m_flRespawnTime",
        ) => Some(("soul_box", "respawn_interval_seconds")),
        ("neutral_camp_midboss", "SpawnIntervalInSeconds" | "m_iSpawnIntervalInSeconds") => {
            Some(("mid_boss", "respawn_interval_seconds"))
        }
        ("neutral_camp_midboss", "SpawnIntervalChange" | "m_iSpawnIntervalChange") => {
            Some(("mid_boss", "respawn_interval_change_seconds"))
        }
        ("neutral_camp_midboss", "SpawnIntervalMin" | "m_iSpawnIntervalMin") => {
            Some(("mid_boss", "respawn_interval_min_seconds"))
        }
        _ => None,
    }
}

fn leaves(path: &str, bytes: &[u8]) -> Result<Vec<Leaf>> {
    ensure!(bytes.len() <= 16 * 1024 * 1024, "extraction_input_limit");
    let content = std::str::from_utf8(bytes)
        .context("extraction_utf8")?
        .trim_start_matches('\u{feff}')
        .trim_start();
    if path.ends_with(".json") || !(content.starts_with("<!--") || content.contains(" = ")) {
        return document_leaves(path, bytes);
    }
    let parsed = super::kv3::parse(content)
        .map_err(anyhow::Error::msg)
        .context("kv3_parse")?;
    Ok(parsed
        .into_iter()
        .filter(|leaf| {
            leaf.numeric_representation.is_some() && field(path, &leaf.pointer).is_some()
        })
        .map(|leaf| Leaf {
            pointer: leaf.pointer,
            text: leaf.lexeme.clone(),
            lexeme: leaf.lexeme,
            numeric: true,
            flags: leaf.flags,
        })
        .collect())
}

pub fn extract(source: Source, path: &str, bytes: &[u8]) -> Result<Vec<Assertion>> {
    if !supported(source, path) {
        return Ok(Vec::new());
    }
    let mut output = Vec::new();
    let mut remaining = 32 * 1024 * 1024;
    for leaf in leaves(path, bytes)? {
        let Some((timer, stat)) = field(path, &leaf.pointer) else {
            continue;
        };
        if !leaf.numeric {
            continue;
        }
        let root = leaf.pointer.split('/').nth(1).unwrap_or_default();
        let condition = json!({"source_entity":root});
        let mut fact = assertion(
            timer,
            "timer",
            &leaf,
            path.ends_with(".json"),
            "stats",
            stat,
            condition,
        );
        fact.stat = format!("timer.{stat}");
        fact.unit = Some("s");
        fact.order = match stat {
            "first_spawn_seconds" => 0,
            "respawn_interval_seconds" => 1,
            _ => 2,
        };
        push_assertion(&mut output, &mut remaining, fact)?;
    }
    Ok(output)
}

pub fn prove(source: Source, path: &str, bytes: &[u8], facts: &[Assertion]) -> Result<()> {
    let expected = extract(source, path, bytes)?;
    ensure!(expected.len() == facts.len(), "timer_proof_count");
    for (actual, expected) in facts.iter().zip(expected) {
        let value_matches = match (&actual.value, &expected.value) {
            (Atom::Number(a), Atom::Number(b)) | (Atom::Text(a), Atom::Text(b)) => a == b,
            _ => false,
        };
        ensure!(
            actual.entity == expected.entity
                && actual.kind == expected.kind
                && actual.stat == expected.stat
                && actual.section == expected.section
                && actual.unit == expected.unit
                && actual.order == expected.order
                && actual.locator_kind == expected.locator_kind
                && actual.locator == expected.locator
                && actual.hash == expected.hash
                && actual.condition == expected.condition
                && value_matches
                && actual.alias.is_none()
                && actual.relation.is_none(),
            "timer_proof_mismatch"
        );
    }
    Ok(())
}
