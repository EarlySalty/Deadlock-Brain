use std::collections::BTreeMap;

use serde_json::Value;

use crate::{DeltaTarget, HeroModel, ItemModel, PatchApplication, PatchDelta, PatchSnapshot};

const MAX_FACTOR: f64 = 2.0;

fn text(value: Option<&Value>) -> String {
    value
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

fn normalized(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '.')
        .flat_map(char::to_lowercase)
        .collect()
}

fn line_key(value: &str) -> String {
    value
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}

fn numeric_tokens(value: &str) -> Vec<f64> {
    value
        .split(|c: char| !c.is_ascii_digit() && !matches!(c, '.' | '-' | '+'))
        .filter_map(|part| part.parse::<f64>().ok())
        .filter(|value| value.is_finite())
        .collect()
}

fn number(value: Option<&Value>) -> Option<f64> {
    value
        .and_then(|value| {
            value.as_f64().or_else(|| {
                let numbers = numeric_tokens(value.as_str()?);
                (numbers.len() == 1).then(|| numbers[0])
            })
        })
        .filter(|value| value.is_finite())
}

fn line(event: &Value) -> String {
    let raw = text(event.get("raw_line"));
    let raw = if raw.is_empty() {
        text(event.get("normalized_line"))
    } else {
        raw
    };
    let raw = raw.trim_start_matches(|c: char| c.is_whitespace() || matches!(c, '-' | '*' | '•'));
    let name = text(event.get("entity_name"));
    raw.split_once(':')
        .filter(|(prefix, _)| normalized(prefix) == normalized(&name))
        .map_or(raw, |(_, rest)| rest)
        .trim()
        .to_string()
}

fn values(event: &Value) -> Option<(f64, f64)> {
    let line = line(event).to_ascii_lowercase();
    if let Some((_, after)) = line.split_once(" from ") {
        let (old, new) = after.split_once(" to ")?;
        let old = numeric_tokens(old);
        let new = numeric_tokens(new);
        return (old.len() == 1 && new.len() == 1).then(|| (old[0], new[0]));
    }
    if let Some((_, after)) = line.split_once(" by ") {
        let numbers = numeric_tokens(after);
        if numbers.len() == 1 && after.contains('%') {
            let direction = if line.contains("increased") {
                1.0
            } else if line.contains("reduced") || line.contains("decreased") {
                -1.0
            } else {
                return None;
            };
            return Some((100.0, 100.0 + direction * numbers[0]));
        }
    }
    if let Some((_, after)) = line.split_once(" is ") {
        let numbers = numeric_tokens(after);
        if numbers.len() == 1 && after.contains('%') {
            let direction = if after.contains("faster") || after.contains("higher") {
                1.0
            } else if after.contains("slower") || after.contains("lower") {
                -1.0
            } else {
                return None;
            };
            return Some((100.0, 100.0 + direction * numbers[0]));
        }
    }
    Some((
        number(event.get("old_value"))?,
        number(event.get("new_value"))?,
    ))
}

fn event_id(event: &Value, keys: &[&str]) -> Option<i64> {
    keys.iter().find_map(|key| {
        event
            .get(key)
            .and_then(|value| value.as_i64().or_else(|| value.as_str()?.parse().ok()))
            .filter(|value| *value > 0)
    })
}

fn target(hero: &HeroModel, event: &Value, snapshots: &[PatchSnapshot]) -> Option<DeltaTarget> {
    let name = text(event.get("entity_name"));
    if name.eq_ignore_ascii_case(&hero.name) {
        let patch_line = normalized(&line(event));
        if let Some(snapshot) = snapshots.iter().find(|snapshot| {
            matches!(snapshot.target, DeltaTarget::Ability(_))
                && !snapshot.name.is_empty()
                && patch_line.starts_with(&normalized(&snapshot.name))
        }) {
            return Some(snapshot.target.clone());
        }
        return Some(DeltaTarget::Hero(hero.hero_id));
    }
    if let Some(snapshot) = snapshots
        .iter()
        .find(|snapshot| snapshot.name.eq_ignore_ascii_case(&name))
    {
        return Some(snapshot.target.clone());
    }
    if let Some(id) = event_id(event, &["ability_id"]) {
        return Some(DeltaTarget::Ability(id));
    }
    event_id(event, &["item_id", "target_item_id", "secondary_entity_id"])
        .or_else(|| {
            event_id(
                event.get("enrichment").unwrap_or(&Value::Null),
                &["item_id", "target_item_id", "secondary_entity_id"],
            )
        })
        .map(DeltaTarget::Item)
}

fn mechanic(line: &str) -> String {
    let line = line.to_ascii_lowercase();
    for (name, needles) in [
        (
            "spirit_scaling",
            &["spirit scaling", "spirit power scaling"][..],
        ),
        ("weapon_scaling", &["weapon scaling"][..]),
        ("bullet_damage", &["bullet damage", "weapon damage"][..]),
        ("fire_rate", &["fire rate", "rounds per second"][..]),
        ("reload", &["reload"][..]),
        ("cooldown", &["cooldown"][..]),
        ("health", &["health"][..]),
        ("damage", &["damage"][..]),
    ] {
        if needles.iter().any(|needle| line.contains(needle)) {
            return name.into();
        }
    }
    "other".into()
}

fn stat_name(event: &Value) -> String {
    let line = line(event).to_ascii_lowercase();
    let prefix = [
        " increased",
        " reduced",
        " decreased",
        " changed",
        " from ",
        " is ",
    ]
    .iter()
    .filter_map(|needle| line.find(needle))
    .min()
    .map_or(line.as_str(), |end| &line[..end]);
    normalized(prefix)
}

fn matches_field(name: &str, field: &str, label: &str) -> bool {
    let field_name = normalized(field.rsplit('.').next().unwrap_or(field));
    let label_name = normalized(label);
    if name == field_name || (!label_name.is_empty() && name == label_name) {
        return true;
    }
    let aliases: &[&str] = match field {
        "weapon.bullet_damage" => &["basebulletdamage", "bulletdamage"],
        "weapon.shots_per_second" => &["basefirerate", "firerate", "roundspersecond"],
        "weapon.reload_duration" => &["reloadtime", "basereloadtime", "reloadduration"],
        "weapon.clip_size" => &["ammocapacity", "baseammo", "clipsize"],
        "base_health" => &["health", "basehealth"],
        "scaling.EFireRate" => &[
            "fireratespiritscaling",
            "fireratespiritpowerscaling",
            "fireratescalingwithspirit",
        ],
        "scaling.ERoundsPerSecond" => &[
            "roundsperspiritscaling",
            "roundspersecondspiritscaling",
            "roundsperspirit",
        ],
        "standard_level_up_upgrades.MODIFIER_VALUE_BASE_BULLET_DAMAGE_FROM_LEVEL" => &[
            "bulletdamageperboon",
            "basebulletdamageperboon",
            "bulletdamagegrowthperboon",
        ],
        "properties.ForwardVelocity" => &[
            "projectilespeed",
            "projecticalrangeandspeed",
            "projectilerangeandspeed",
            "travelspeed",
        ],
        "properties.BonusSpirit" => &["spiritpoweronproc", "bonusspirit"],
        "properties.BuildUpPerShot" => &["buildup", "buildupershot"],
        "property_spirit_scaling.BulletsBonusMagicDamage" => {
            &["basebulletdamagescaling", "basebulletdamagespiritscaling"]
        }
        "cooldown" | "properties.AbilityCooldown" => &["cooldown"],
        _ if field.ends_with(".StatusResistancePercent.bonus") => &[
            "debuffresistance",
            "statusresistance",
            "statusresistancepercent",
        ],
        _ if field.ends_with(".CombatBarrier.bonus") => &[
            "spiritscaling",
            "spiritpowerscaling",
            "barrierspiritscaling",
        ],
        _ => &[],
    };
    aliases
        .iter()
        .any(|alias| name == *alias || name.ends_with(alias))
}

pub fn compute_patch_delta(hero: &HeroModel, events: &[Value]) -> Vec<PatchDelta> {
    compute_patch_delta_with_snapshots(hero, events, &[])
}

pub fn compute_patch_delta_with_snapshots(
    hero: &HeroModel,
    events: &[Value],
    snapshots: &[PatchSnapshot],
) -> Vec<PatchDelta> {
    let mut groups: BTreeMap<(String, String, String), Vec<&Value>> = BTreeMap::new();
    for event in events {
        let Some(target) = target(hero, event, snapshots) else {
            continue;
        };
        let posted = text(event.get("posted_at"));
        let patch = posted
            .get(..10)
            .map_or_else(|| text(event.get("patch_external_id")), str::to_string);
        groups
            .entry((patch, format!("{target:?}"), line_key(&line(event))))
            .or_default()
            .push(event);
    }
    let mut result = Vec::new();
    for group in groups.values() {
        let event = group[0];
        let target = target(hero, event, snapshots).unwrap();
        let parsed = values(event);
        let (sign, magnitude) = parsed.map_or((0, 0.0), |(old, new)| {
            let sign = if new > old {
                1
            } else if new < old {
                -1
            } else {
                0
            };
            let magnitude = (new / old - 1.0).abs();
            (
                sign,
                if magnitude.is_finite() {
                    magnitude
                } else {
                    0.0
                },
            )
        });
        let mut delta = PatchDelta {
            target: target.clone(),
            mechanic: mechanic(&line(event)),
            sign,
            magnitude,
            note: line(event),
            application: None,
        };
        let candidates = snapshots
            .iter()
            .filter(|snapshot| snapshot.target == target)
            .flat_map(|snapshot| &snapshot.fields)
            .filter(|(field, value)| matches_field(&stat_name(event), field, &value.label))
            .collect::<Vec<_>>();
        let posted_at = if group
            .iter()
            .all(|event| number(event.get("posted_at_epoch")).is_some())
        {
            group
                .iter()
                .filter_map(|event| number(event.get("posted_at_epoch")))
                .min_by(f64::total_cmp)
        } else {
            None
        };
        let reason = if candidates.len() != 1 {
            "kein eindeutig modelliertes Snapshot-Feld"
        } else {
            let (field, snapshot) = candidates[0];
            match (posted_at, snapshot.fetched_at, parsed) {
                (Some(posted), Some(fetched), _) if posted <= fetched => {
                    "Historie, bereits im Snapshot enthalten"
                }
                (None, _, _) | (_, None, _) => "Zeitstempel fehlt",
                (_, _, None) => "keine eindeutigen Ausgangs- und Endwerte",
                (Some(posted), Some(fetched), Some((old, new))) => {
                    let ratio = new / old;
                    if old <= 0.0
                        || new <= 0.0
                        || !ratio.is_finite()
                        || !(1.0 / MAX_FACTOR..=MAX_FACTOR).contains(&ratio)
                        || snapshot.value <= 0.0
                        || sign == 0
                    {
                        "unplausibler Faktor oder nicht positiver Ausgangs-/Endwert"
                    } else if group.iter().any(|other| values(other) != parsed) {
                        "widersprüchliche Zahlen für dieselbe Patchzeile"
                    } else {
                        delta.application = Some(PatchApplication {
                            field: field.clone(),
                            snapshot_value: snapshot.value,
                            fetched_at: fetched,
                            posted_at: posted,
                            source: snapshot.source.clone(),
                        });
                        "anwendbar auf Snapshot-Feld"
                    }
                }
            }
        };
        delta.note = format!(
            "{}: {} ({reason}; {} Fassungen)",
            if delta.application.is_some() {
                "Anwendbar"
            } else {
                "Nicht anwendbar"
            },
            delta.note,
            group.len()
        );
        result.push(delta);
    }
    result.sort_by(|left, right| {
        left.application
            .as_ref()
            .map_or(0.0, |app| app.posted_at)
            .total_cmp(&right.application.as_ref().map_or(0.0, |app| app.posted_at))
            .then_with(|| left.note.cmp(&right.note))
    });
    result
}

fn apply_one(hero: &mut HeroModel, items: &mut [ItemModel], delta: &PatchDelta) -> bool {
    let Some(app) = &delta.application else {
        return false;
    };
    if app.posted_at <= app.fetched_at || !app.posted_at.is_finite() || !app.fetched_at.is_finite()
    {
        return false;
    }
    let factor = 1.0 + f64::from(delta.sign) * delta.magnitude;
    if !factor.is_finite() || !(1.0 / MAX_FACTOR..=MAX_FACTOR).contains(&factor) {
        return false;
    }
    if let DeltaTarget::Ability(id) = delta.target {
        if let Some(rest) = app.field.strip_prefix("upgrade.") {
            let mut parts = rest.splitn(3, '.');
            let Some(upgrade_index) = parts.next().and_then(|part| part.parse::<usize>().ok())
            else {
                return false;
            };
            let Some(property_name) = parts.next() else {
                return false;
            };
            if parts.next() != Some("bonus") {
                return false;
            }
            let Some(ability) = hero
                .abilities
                .iter_mut()
                .find(|ability| ability.ability_id == id)
            else {
                return false;
            };
            let Some(property) = ability
                .upgrades
                .get_mut(upgrade_index)
                .and_then(|upgrade| upgrade.get_mut("property_upgrades"))
                .and_then(Value::as_array_mut)
                .and_then(|properties| {
                    properties.iter_mut().find(|property| {
                        property.get("name").and_then(Value::as_str) == Some(property_name)
                    })
                })
            else {
                return false;
            };
            let Some(current) = number(property.get("bonus")) else {
                return false;
            };
            let updated = current * factor;
            let cumulative = updated / app.snapshot_value;
            if !updated.is_finite()
                || updated <= 0.0
                || !cumulative.is_finite()
                || !(1.0 / MAX_FACTOR..=MAX_FACTOR).contains(&cumulative)
            {
                return false;
            }
            property["bonus"] = Value::from(updated);
            if let Some(step) = ability.scaling_step.as_mut().filter(|step| {
                step.upgrade_index == upgrade_index as i64 && step.stat == property_name
            }) {
                step.to = step.from + updated;
            }
            return true;
        }
    }
    let value = match delta.target {
        DeltaTarget::Hero(id) if id == hero.hero_id => match app.field.as_str() {
            "weapon.bullet_damage" => Some(&mut hero.weapon.bullet_damage),
            "weapon.shots_per_second" => Some(&mut hero.weapon.shots_per_second),
            "weapon.reload_duration" => Some(&mut hero.weapon.reload_duration),
            "weapon.clip_size" => Some(&mut hero.weapon.clip_size),
            "weapon.range" => Some(&mut hero.weapon.range),
            "base_health" => Some(&mut hero.base_health),
            field => field
                .strip_prefix("scaling.")
                .and_then(|name| hero.scaling.iter_mut().find(|stat| stat.stat == name))
                .and_then(|stat| stat.per_spirit.as_mut())
                .or_else(|| {
                    field
                        .strip_prefix("standard_level_up_upgrades.")
                        .and_then(|name| hero.standard_level_up_upgrades.get_mut(name))
                }),
        },
        DeltaTarget::Ability(id) => hero
            .abilities
            .iter_mut()
            .find(|ability| ability.ability_id == id)
            .and_then(|ability| {
                if app.field == "cooldown" {
                    Some(&mut ability.cooldown)
                } else {
                    app.field
                        .strip_prefix("properties.")
                        .and_then(|name| ability.properties.get_mut(name))
                }
            }),
        DeltaTarget::Item(id) => {
            items
                .iter_mut()
                .find(|item| item.item_id == id)
                .and_then(|item| {
                    if app.field == "proc_cooldown" {
                        return item.proc_cooldown.as_mut();
                    }
                    if let Some(name) = app.field.strip_prefix("properties.") {
                        return item.properties.get_mut(name);
                    }
                    app.field
                        .strip_prefix("property_spirit_scaling.")
                        .and_then(|name| item.property_spirit_scaling.get_mut(name))
                })
        }
        _ => None,
    };
    let Some(value) = value else { return false };
    let updated = *value * factor;
    let cumulative = updated / app.snapshot_value;
    if !updated.is_finite()
        || updated <= 0.0
        || !cumulative.is_finite()
        || !(1.0 / MAX_FACTOR..=MAX_FACTOR).contains(&cumulative)
    {
        return false;
    }
    *value = updated;
    if let DeltaTarget::Item(id) = delta.target {
        if let Some(name) = app.field.strip_prefix("properties.") {
            let item = items.iter_mut().find(|item| item.item_id == id).unwrap();
            if let Some(passive) = item.passive_properties.get_mut(name) {
                *passive = updated;
            }
            if name.to_ascii_lowercase().contains("proccooldown") {
                item.proc_cooldown = Some(updated);
            }
        }
    }
    if let DeltaTarget::Ability(id) = delta.target {
        if app.field.starts_with("properties.") {
            if let Some(ability) = hero
                .abilities
                .iter_mut()
                .find(|ability| ability.ability_id == id)
            {
                crate::data::refresh_ability_derived(ability);
            }
        }
    }
    if app.field.starts_with("weapon.") {
        hero.weapon.sustained_dps = 0.0;
        hero.weapon.sustained_dps = crate::mechanics::weapon_dps(&hero.weapon, 40.0);
    }
    true
}

pub fn apply_patch_delta(hero: &mut HeroModel, items: &mut [ItemModel], deltas: &[PatchDelta]) {
    let meta = crate::MetaIndex {
        by_item: BTreeMap::new(),
        sample_ok: Default::default(),
    };
    apply_scored_patch_delta(
        hero,
        items,
        &mut deltas.to_vec(),
        &meta,
        &crate::ReasonerConfig::default(),
    );
}

pub fn apply_scored_patch_delta(
    hero: &mut HeroModel,
    items: &mut [ItemModel],
    deltas: &mut [PatchDelta],
    meta: &crate::MetaIndex,
    cfg: &crate::ReasonerConfig,
) {
    let original = crate::item::score_items(hero, items, meta, &[], cfg)
        .into_iter()
        .map(|item| (item.item.item_id, item.score.total))
        .collect::<BTreeMap<_, _>>();
    for delta in deltas {
        if delta.application.is_none() {
            continue;
        }
        let mut candidate_hero = hero.clone();
        let mut candidate_items = items.to_vec();
        let changed = apply_one(&mut candidate_hero, &mut candidate_items, delta);
        candidate_hero.damage_plan = crate::hero::damage_plan(&candidate_hero, cfg);
        let valid = changed
            && crate::item::score_items(&candidate_hero, &candidate_items, meta, &[], cfg)
                .iter()
                .all(|item| {
                    let old = original[&item.item.item_id];
                    let new = item.score.total;
                    if new == old {
                        true
                    } else {
                        new.is_finite()
                            && new >= 0.0
                            && old > 0.0
                            && (1.0 / MAX_FACTOR..=MAX_FACTOR).contains(&(new / old))
                    }
                });
        if valid {
            *hero = candidate_hero;
            items.clone_from_slice(&candidate_items);
        } else {
            delta.application = None;
            delta.note = format!(
                "Nicht anwendbar: Feld- oder Score-Faktor außerhalb 0,5 bis 2; {}",
                delta.note
            );
        }
    }
}

#[cfg(test)]
#[path = "patch_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "patch_current_tests.rs"]
mod current_patch_tests;
