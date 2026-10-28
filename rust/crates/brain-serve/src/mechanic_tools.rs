use brain_contracts::{
    tools::{GetMechanicRequest, MechanicKind},
    PortError, ToolDefinition, ToolName,
};
use serde_json::{json, Value};
use sqlx::PgConnection;
use std::collections::BTreeSet;

pub(super) const QUERY: &str = include_str!("mechanic_query.sql");
const MAX_ROWS: usize = 120;
const QUERY_LIMIT: usize = 401;

pub(super) fn definition() -> ToolDefinition {
    ToolDefinition {
        name: ToolName::GetMechanic,
        description: "Liest belegte Spielmechanik-Werte aus den Spieldateien und rechnet sie bei Angabe von minute auf eine Spielminute hoch. Für allgemeine Spielfragen ohne Held oder Item zuerst get_mechanic verwenden, nicht find_entity oder search_docs. Mechaniken: lane_troopers (Lane-Trooper, Wellen, Creeps: Seelen je Trooper und Welle, Seelen pro Minute, Truppgröße, Wellenabstand, Anteil sofort vs. Seelenorb, Denien, Trooper-Leben), soul_sharing (Seelenteilung mit Mitspielern auf der Lane, Anteil je Spieleranzahl), denizens (Jungle-Camps, Neutrals: Seelen, Spawn, Respawn, Leben je Camp-Stufe weak/normal/strong), objectives (Guardian = npc_boss_tier1, Walker = npc_boss_tier2, Patron = npc_boss_tier3, Base Guardians = npc_barrack_boss, Shrines: Leben, Schaden, Seelen beim Zerstören), mid_boss (Midboss: Seelen, Leben, Spawn, Respawn), rejuvenator (Rejuvenator: Buffdauer, Trooper-Lebensbonus und Respawn-Faktor je erstem/zweitem/drittem Rejuvenator), hero_kills (Startseelen, Kopfgeld für Heldenkills, Assist-Anteil, Comeback), respawn (Respawnzeit nach Spielzeit), urn (Urne: erster Spawn, Respawn, Belohnung), breakables (Kisten und zerstörbare Objekte: Spawnzeiten), sinners_sacrifice (Sinner's Sacrifice), item_tiers (Itemkosten je Stufe 1 bis 4, Index 1 = Stufe 1), levels (Seelen pro Level, Ability Points, Ability-Freischaltungen, Power Increase), street_brawl (Spielmodus Street Brawl: Runden, Seelen je Runde, Respawn, Punkte zum Sieg). Zeilen mit derived_ sind aus den belegten Rohwerten berechnet; condition.formula nennt die Rechnung und condition.inputs die Rohwerte. Diese Werte darfst du unverändert nennen und mit der Formel erklären. Einheiten: Zeiten in Sekunden, *_time-Grenzen der Trooper-Wellen in Minuten, Spieldauer-Grenzen der Respawn-Rampen in Sekunden. Index 0 in Listen ist der erste Eintrag. Fehlender Patch oder current bedeutet aktueller beobachteter Quellenstand. Fehlende Werte nicht ergänzen.".into(),
        input_schema: json!({
            "type":"object", "additionalProperties":false, "required":["mechanic"],
            "properties":{
                "mechanic":{"type":"string","enum":MechanicKind::ALL.map(MechanicKind::as_str)},
                "minute":{"type":"integer","minimum":0,"maximum":120},
                "patch":{"type":"string","minLength":1,"maxLength":128}
            }
        }),
    }
}

pub(super) async fn read(
    connection: &mut PgConnection,
    request: &GetMechanicRequest,
) -> Result<Option<Value>, PortError> {
    let key = format!("mechanic_{}", request.mechanic.as_str());
    let rows = sqlx::query_scalar::<_, Value>(QUERY)
        .bind(key)
        .fetch_all(connection)
        .await
        .map_err(super::database_error)?;
    if rows.is_empty() {
        return Ok(None);
    }
    Ok(Some(present(request, rows)))
}

fn present(request: &GetMechanicRequest, rows: Vec<Value>) -> Value {
    let derived = derive(request.mechanic, request.minute, &rows);
    let omitted = rows.len().saturating_sub(MAX_ROWS);
    let lower_bound = rows.len() >= QUERY_LIMIT;
    let inputs: BTreeSet<&str> = derived
        .iter()
        .filter_map(|row| row["condition"]["inputs"].as_array())
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    let (used, rest): (Vec<Value>, Vec<Value>) = rows.iter().cloned().partition(|row| {
        row["stat_key"]
            .as_str()
            .is_some_and(|stat| inputs.contains(stat))
    });
    let facts: Vec<Value> = derived
        .iter()
        .cloned()
        .chain(used.into_iter().chain(rest).take(MAX_ROWS))
        .collect();
    json!({
        "mechanic":request.mechanic.as_str(),
        "minute":request.minute,
        "facts":facts,
        "truncated":omitted>0,
        "omitted_count":omitted,
        "omitted_count_is_lower_bound":lower_bound
    })
}

struct Rows<'a>(&'a [Value]);

impl<'a> Rows<'a> {
    fn get(&self, stat: &str) -> Option<(&'a Value, f64)> {
        let row = self.0.iter().find(|row| row["stat_key"] == stat)?;
        let number = match &row["value"] {
            Value::String(text) => text.parse::<f64>().ok(),
            Value::Number(number) => number.as_f64(),
            _ => None,
        }?;
        number.is_finite().then_some((row, number))
    }

    fn list(&self, prefix: &str) -> Vec<(&'a Value, f64)> {
        (0..)
            .map_while(|index| self.get(&format!("{prefix}_{index}")))
            .collect()
    }
}

fn format(value: f64) -> String {
    let rounded = (value * 100.0).round() / 100.0;
    let text = format!("{rounded:.2}");
    text.trim_end_matches('0').trim_end_matches('.').to_owned()
}

fn row(
    stat: String,
    value: f64,
    source: &Value,
    formula: &str,
    inputs: &[&str],
    extra: Value,
) -> Value {
    let mut condition = json!({"formula":formula,"inputs":inputs});
    if let (Some(condition), Value::Object(extra)) = (condition.as_object_mut(), extra) {
        condition.extend(extra);
    }
    json!({
        "stat_key":stat,
        "value":format(value),
        "condition":condition,
        "citation":source["citation"],
        "source_revision":source["source_revision"],
        "patch":null
    })
}

fn derive(mechanic: MechanicKind, minute: Option<u16>, rows: &[Value]) -> Vec<Value> {
    let rows = Rows(rows);
    match (mechanic, minute) {
        (MechanicKind::LaneTroopers, Some(minute)) => lane_troopers(&rows, minute),
        (MechanicKind::Denizens, Some(minute)) => [
            "neutral_trooper_weak",
            "neutral_trooper_normal",
            "neutral_trooper_strong",
        ]
        .into_iter()
        .filter_map(|camp| camp_souls(&rows, camp, minute))
        .collect(),
        (MechanicKind::SinnersSacrifice, Some(minute)) => {
            camp_souls(&rows, "neutral_sinners_sacrifice", minute)
                .into_iter()
                .collect()
        }
        (MechanicKind::MidBoss, Some(minute)) => mid_boss(&rows, minute),
        (MechanicKind::Respawn, Some(minute)) => respawn(&rows, minute),
        (MechanicKind::Urn, Some(minute)) => urn(&rows, minute),
        (MechanicKind::SoulSharing, _) => soul_sharing(&rows),
        (MechanicKind::Levels, _) => levels(&rows),
        _ => Vec::new(),
    }
}

fn linear(
    rows: &Rows,
    base: &str,
    bonus: &str,
    minute: u16,
    stat: &str,
    formula: &str,
) -> Option<(Value, f64)> {
    let (source, base_value) = rows.get(base)?;
    let (_, bonus_value) = rows.get(bonus)?;
    let value = base_value + bonus_value * f64::from(minute);
    Some((
        row(
            stat.into(),
            value,
            source,
            formula,
            &[base, bonus],
            json!({"minute":minute}),
        ),
        value,
    ))
}

fn lane_troopers(rows: &Rows, minute: u16) -> Vec<Value> {
    let mut output = Vec::new();
    let Some((trooper, souls)) = linear(
        rows,
        "citadel_trooper_gold_reward",
        "citadel_trooper_gold_reward_bonus_per_minute",
        minute,
        "derived_trooper_souls_at_minute",
        "Seelen je Lane-Trooper = Grundwert + Bonus je Minute × Minute",
    ) else {
        return output;
    };
    output.push(trooper);
    if let Some((source, size)) = rows.get("citadel_trooper_squad_size") {
        output.push(row(
            "derived_squad_souls_at_minute".into(),
            souls * size,
            source,
            "Seelen eines Trooper-Trupps je Lane = Seelen je Trooper × Truppgröße",
            &[
                "derived_trooper_souls_at_minute",
                "citadel_trooper_squad_size",
            ],
            json!({"minute":minute}),
        ));
    }
    let instant_key = match rows.get("citadel_trooper_laning_gold_rules_end_time") {
        Some((_, end)) if f64::from(minute) >= end => {
            "citadel_trooper_instant_gold_ratio_postlaning"
        }
        _ => "citadel_trooper_instant_gold_ratio_laning",
    };
    if let Some((source, ratio)) = rows.get(instant_key) {
        output.push(row(
            "derived_trooper_souls_instant_at_minute".into(),
            souls * ratio,
            source,
            "Sofort beim letzten Treffer = Seelen je Trooper × Sofortanteil; der Rest fällt als Seelenorb",
            &[
                "derived_trooper_souls_at_minute",
                instant_key,
                "citadel_trooper_laning_gold_rules_end_time",
            ],
            json!({"minute":minute}),
        ));
        output.push(row(
            "derived_trooper_souls_orb_at_minute".into(),
            souls * (1.0 - ratio),
            source,
            "Seelenorb = Seelen je Trooper × (1 - Sofortanteil)",
            &["derived_trooper_souls_at_minute", instant_key],
            json!({"minute":minute}),
        ));
    }
    let interval = [
        (
            "citadel_trooper_spawn_interval_very_late_time",
            "citadel_trooper_spawn_interval_very_late",
        ),
        (
            "citadel_trooper_spawn_interval_late_time",
            "citadel_trooper_spawn_interval_late",
        ),
    ]
    .into_iter()
    .find_map(|(time, value)| {
        let (_, start) = rows.get(time)?;
        (f64::from(minute) >= start).then_some(value)
    })
    .unwrap_or("citadel_trooper_spawn_interval_early");
    if let Some((source, seconds)) = rows.get(interval) {
        output.push(row(
            "derived_wave_interval_seconds_at_minute".into(),
            seconds,
            source,
            "Abstand zwischen Trooper-Wellen ab der jeweiligen Minutengrenze",
            &[
                interval,
                "citadel_trooper_spawn_interval_late_time",
                "citadel_trooper_spawn_interval_very_late_time",
            ],
            json!({"minute":minute}),
        ));
    }
    output.extend(
        linear(
            rows,
            "citadel_minion_gold_reward",
            "citadel_minion_gold_reward_bonus_per_minute",
            minute,
            "derived_minion_souls_at_minute",
            "Seelen je Minion = Grundwert + Bonus je Minute × Minute",
        )
        .map(|(value, _)| value),
    );
    output
}

fn camp_souls(rows: &Rows, camp: &str, minute: u16) -> Option<Value> {
    let base = format!("{camp}_goldreward");
    let percent = format!("{camp}_goldrewardbonuspercentperminute");
    let (source, souls) = rows.get(&base)?;
    let (_, growth) = rows.get(&percent)?;
    Some(row(
        format!("derived_{camp}_souls_at_minute"),
        souls * (1.0 + growth / 100.0 * f64::from(minute)),
        source,
        "Seelen = Grundwert × (1 + Prozent je Minute / 100 × Minute)",
        &[&base, &percent],
        json!({"minute":minute}),
    ))
}

fn mid_boss(rows: &Rows, minute: u16) -> Vec<Value> {
    [
        (
            "citadel_super_neutral_gold_reward",
            "citadel_super_neutral_gold_reward_bonus_per_minute",
            "derived_mid_boss_souls_at_minute",
            "Seelen = Grundwert + Bonus je Minute × Minute",
        ),
        (
            "npc_super_neutral_startinghealth",
            "npc_super_neutral_healthgainperminute",
            "derived_mid_boss_health_at_minute",
            "Leben = Startleben + Lebenszuwachs je Minute × Minute",
        ),
    ]
    .into_iter()
    .filter_map(|(base, bonus, stat, formula)| {
        linear(rows, base, bonus, minute, stat, formula).map(|(value, _)| value)
    })
    .collect()
}

fn urn(rows: &Rows, minute: u16) -> Vec<Value> {
    linear(
        rows,
        "citadel_crate_reward_base",
        "citadel_crate_reward_time_multiplier",
        minute,
        "derived_urn_reward_at_minute",
        "Urnenbelohnung = Grundwert + Zeitfaktor × Minute",
    )
    .map(|(value, _)| value)
    .into_iter()
    .collect()
}

fn respawn(rows: &Rows, minute: u16) -> Vec<Value> {
    let seconds = f64::from(minute) * 60.0;
    ["min", "max"]
        .into_iter()
        .filter_map(|bound| {
            let points: Vec<(String, f64, f64, &Value)> = (1..=3)
                .filter_map(|step| {
                    let value_key = format!("citadel_player_spawn_time_{bound}_ramp_{step}");
                    let time_key = format!("{value_key}_time");
                    let (source, value) = rows.get(&value_key)?;
                    let (_, time) = rows.get(&time_key)?;
                    Some((value_key, time, value, source))
                })
                .collect();
            let first = points.first()?;
            let value = match points.windows(2).find(|pair| seconds < pair[1].1) {
                _ if seconds <= first.1 => first.2,
                Some(pair) => {
                    let (from, to) = (&pair[0], &pair[1]);
                    from.2 + (to.2 - from.2) * (seconds - from.1) / (to.1 - from.1)
                }
                None => points.last()?.2,
            };
            let inputs: Vec<&str> = points.iter().map(|point| point.0.as_str()).collect();
            Some(row(
                format!("derived_respawn_{bound}_seconds_at_minute"),
                value,
                first.3,
                "Respawnzeit linear zwischen den Rampenpunkten (Spielzeit in Sekunden), vor dem ersten Punkt dessen Wert, nach dem letzten dessen Wert",
                &inputs,
                json!({"minute":minute,"game_seconds":seconds}),
            ))
        })
        .collect()
}

fn soul_sharing(rows: &Rows) -> Vec<Value> {
    [
        ("trooperkillgoldsharefrac", "trooper"),
        ("herokillgoldsharefrac", "hero"),
    ]
    .into_iter()
    .flat_map(|(prefix, kind)| {
        rows.list(prefix)
            .into_iter()
            .enumerate()
            .map(move |(index, (source, fraction))| {
                let players = index + 1;
                row(
                    format!("derived_{kind}_share_percent_{players}_players"),
                    fraction * 100.0,
                    source,
                    "Anteil je Spieler in Prozent bei dieser Spieleranzahl = Teilungsfaktor × 100",
                    &[&format!("{prefix}_{index}")],
                    json!({"players":players}),
                )
            })
    })
    .collect()
}

fn levels(rows: &Rows) -> Vec<Value> {
    (0..)
        .map_while(|index| {
            let prefix = format!("level_{index}_");
            let (source, souls) = rows.get(&format!("{prefix}requiredsouls"))?;
            let mut extra = json!({"level":index + 1});
            for (key, field) in [
                ("ability_points", "abilitypoints"),
                ("ability_unlocks", "abilityunlocks"),
                ("power_increase", "powerincrease"),
            ] {
                if let Some(value) = rows.0.iter().find_map(|row| {
                    (row["stat_key"] == format!("{prefix}{field}").as_str())
                        .then(|| row["value"].clone())
                }) {
                    extra[key] = value;
                }
            }
            Some(row(
                format!("derived_level_{}_required_souls", index + 1),
                souls,
                source,
                "Benötigte Seelen für dieses Level; Index 0 der Quelle ist Level 1",
                &[&format!("{prefix}requiredsouls")],
                extra,
            ))
        })
        .collect()
}

#[cfg(test)]
#[path = "mechanic_tools_tests.rs"]
mod tests;
