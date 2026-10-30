use brain_contracts::{
    tools::{GetMechanicRequest, MechanicKind, ObjectiveKind},
    PortError, ToolDefinition, ToolName,
};
use serde_json::{json, Value};
use sqlx::PgConnection;
use std::collections::BTreeSet;

#[path = "mechanic_history.rs"]
mod history;

pub(super) const QUERY: &str = include_str!("mechanic_query.sql");
const ORB_HISTORY: &str = include_str!("mechanic_orb_history.sql");
const ORB_SHARE_MEANING: &str = "Verlauf aus den Patchnotes: Prozentanteil der Seelen eines Lane-Troopers, der im fliegenden Seelenorb landet und gesichert oder gedeniet werden kann. Bei kill/orb-Angaben wie 60/40 ist die zweite Zahl der Orb-Anteil. old_orb_share_percent gilt vor, value und new_orb_share_percent ab released_at.";
const MAX_ROWS: usize = 120;
const QUERY_LIMIT: usize = 401;
const REFERENCE_MINUTES: [u16; 4] = [0, 10, 20, 30];
const LANE_REFERENCE_MINUTES: [u16; 1] = [0];
const MEANINGS: &[(&str, &str)] = &[
    ("citadel_deny_denier_percentage", "Denien (deny): Anteil des Seelenorb-Werts für den Spieler, der einen gegnerischen Seelenorb zuerst abschießt (1 = 100 %)"),
    ("citadel_deny_denied_percentage", "Denien (deny): Anteil des Seelenorb-Werts für den Gegner, dessen Orb gedeniet wurde (0 = nichts)"),
    ("citadel_deny_", "Denien (deny): gegnerischen Seelenorb zuerst abschießen"),
    ("citadel_crate_spawn_initial_delay", "Urne (Urn, Soul Urn, Seelenurne): erster Spawn in Sekunden Spielzeit"),
    ("citadel_crate_respawn_interval", "Urne (Urn, Soul Urn, Seelenurne): Sekunden bis zum nächsten Spawn"),
    ("citadel_crate_reward_base", "Urne (Urn, Soul Urn, Seelenurne): Grundwert der Belohnung beim Abgeben"),
    ("citadel_crate_reward_time_multiplier", "Urne (Urn, Soul Urn, Seelenurne): Zuwachs der Belohnung je Spielminute"),
    ("citadel_crate_delivery_overtime_bonus", "Urne (Urn, Soul Urn, Seelenurne): Bonus beim Abgeben"),
    ("citadel_crate_", "Urne (Urn, Soul Urn, Seelenurne)"),
    ("citadel_idol_", "Idol: interner Name der Urne, solange sie getragen und zum Abgabepunkt gebracht wird"),
    ("derived_urn_", "Urne (Urn, Soul Urn, Seelenurne)"),
    ("derived_trooper_", "Wert je einzelnem Trooper"),
    ("derived_wave_souls_", "Wert je ganzer Welle, also alle Trooper eines Trupps zusammen"),
    ("derived_squad_", "Wert je ganzer Welle, also alle Trooper eines Trupps zusammen"),
    ("derived_hero_kill_", "Kopfgeld für einen Heldenkill"),
    ("objectiveparams_tier1goldkill", "Guardian (npc_boss_tier1): Seelen beim Zerstören"),
    ("objectiveparams_tier2goldkill", "Walker (npc_boss_tier2): Seelen beim Zerstören"),
    ("objectiveparams_baseguardiansgoldkill", "Base Guardian (npc_barrack_boss): Seelen beim Zerstören"),
    ("objectiveparams_shrinesgoldkill", "Shrine (Schrein): Seelen beim Zerstören"),
    ("objectiveparams_patronphase1goldkill", "Patron (npc_boss_tier3): Seelen beim Besiegen der ersten Phase; der Wert ist die Seelenzahl, auch wenn er 0 ist, und wird dann als 0 Seelen genannt"),
    ("objectiveparams_nearplayersplitpct", "Prozentwert der Objective-Seelen für Spieler in der Nähe des Ziels; wie Team und Spieler in der Nähe genau aufgeteilt werden, steht nicht in den Daten"),
    ("objectiveparams_", "Objective-Seelen: *GoldKill = Seelen beim Zerstören, *GoldOrbs und GoldPerOrb = Seelen als Seelenorbs"),
    ("citadel_no_orbs_on_objective_kill", "true = beim Zerstören von Objectives entstehen keine Seelenorbs"),
    ("citadel_super_trooper_gold_mult", "Super Trooper (nach zerstörtem Shrine der Lane): Faktor auf das Kopfgeld je Super Trooper; 0,85 = 15 % weniger Seelen"),
    ("shrinesdownbuffmodifier_damagescale", "Super Trooper (nach zerstörtem Shrine der Lane): Bonus auf den Schaden pro Sekunde in Prozent; 60 = 60 % mehr Schaden"),
    ("citadel_ability_melee_parry_parriedstuntime", "Parry (Parieren): Sekunden, die die Parry-Wirkung auf einen erfolgreich parierten Gegner anhält; währenddessen nimmt er mehr Schaden"),
    ("citadel_ability_melee_parry_victimdamagetakenscale", "Parry (Parieren): der parierte Gegner nimmt während der Parry-Wirkung so viel Prozent mehr Schaden aus allen Quellen"),
    ("citadel_ability_melee_parry_abilitycooldown_value", "Parry (Parieren): Abklingzeit in Sekunden"),
    ("citadel_player_dash_down_stamina_cost", "Dash nach unten (Down Dash): Ausdauerkosten in Ausdauerpunkten"),
    ("citadel_player_dash_stamina_cost", "Dash: Ausdauerkosten in Ausdauerpunkten"),
    ("citadel_initial_wall_jump_stamina_cost", "Wall Jump (Wandsprung): Ausdauerkosten des ersten Wandsprungs in Ausdauerpunkten"),
    ("citadel_wall_jump_num_free_wall_jumps", "Wall Jump (Wandsprung): Zahl der Wandsprünge hintereinander ohne Ausdauerkosten"),
    ("citadel_wall_jump_stamina_cost", "Wall Jump (Wandsprung): Ausdauerkosten je weiterem Wandsprung in Ausdauerpunkten"),
    ("citadel_wall_slide_terminal_velocity", "Wall Slide (an der Wand rutschen): höchste Fallgeschwindigkeit an der Wand in Source-Einheiten pro Sekunde"),
];

const OBJECTIVE_NAMES: &[(&str, &str)] = &[
    ("npc_boss_tier1_", "Guardian (npc_boss_tier1)"),
    ("npc_boss_tier2_", "Walker (npc_boss_tier2)"),
    ("npc_boss_tier3_", "Patron (npc_boss_tier3)"),
    ("npc_barrack_boss_", "Base Guardian (npc_barrack_boss)"),
    (
        "destroyable_building_",
        "Shrine (Schrein, destroyable_building)",
    ),
];
const OBJECTIVE_ROWS: &[(ObjectiveKind, &[&str])] = &[
    (
        ObjectiveKind::Guardian,
        &["npc_boss_tier1_", "objectiveparams_tier1"],
    ),
    (
        ObjectiveKind::Walker,
        &["npc_boss_tier2_", "objectiveparams_tier2"],
    ),
    (
        ObjectiveKind::Patron,
        &["npc_boss_tier3_", "objectiveparams_patron"],
    ),
    (
        ObjectiveKind::BaseGuardian,
        &["npc_barrack_boss_", "objectiveparams_baseguardians"],
    ),
    (
        ObjectiveKind::Shrine,
        &[
            "destroyable_building_",
            "objectiveparams_shrines",
            "shrinesdownbuffmodifier_",
            "citadel_super_trooper_",
        ],
    ),
];
const OBJECTIVE_FIELDS: &[(&str, &str)] = &[
    ("maxhealth", "Leben"),
    ("phase2health", "Leben der zweiten Phase"),
    ("maxhealthgenerator", "Leben der Shrines, solange noch kein Shrine des Teams zerstört ist"),
    ("maxhealthgeneratorsecond", "Leben des verbliebenen Shrines, nachdem der erste Shrine des Teams zerstört ist"),
    ("empoweredmodifierlevel1_maxhealth", "Leben nach dem ersten zerstörten Walker; Walker bekommen nach jedem zerstörten Walker mehr Leben"),
    ("empoweredmodifierlevel2_maxhealth", "Leben nach dem zweiten zerstörten Walker; Walker bekommen nach jedem zerstörten Walker mehr Leben"),
    ("intrinsicmodifiers_0_bullet_armor_damage_resist", "Kugelresistenz in Prozent: nimmt so viel Prozent weniger Schaden durch Kugeln"),
    ("nearbyenemyresist_nearbyenemyresistrange", "Reichweite in Metern, in der gegnerische Helden für die Resistenz je nahem Gegner zählen"),
    ("nearbyenemyresist_resistvalues_", "Resistenz je Zahl naher gegnerischer Helden: die Zahl am Ende des stat_key ist die Zahl der Gegner in Reichweite, der Wert ist, wie viel Prozent weniger Schaden das Ziel dann nimmt"),
    ("friendlyauramodifier_auraradius", "Aura für verbündete Helden: Radius in Metern"),
    ("friendlyauramodifier_modifierprovidedbyaura_bullet_armor_damage_resist", "Aura: verbündete Helden im Auraradius nehmen so viel Prozent weniger Schaden durch Kugeln"),
    ("friendlyauramodifier_modifierprovidedbyaura_tech_resist", "Aura: verbündete Helden im Auraradius nehmen so viel Prozent weniger Spirit-Schaden"),
    ("backdoorbulletresistmodifier_bulletresist", "zusätzliche Kugelresistenz in Prozent ohne gegnerische Helden in der Nähe; sinkt je nahem gegnerischem Held, mindestens 0"),
    ("backdoorbulletresistmodifier_bulletresistreductionperhero", "Prozentpunkte, um die die zusätzliche Kugelresistenz je nahem gegnerischem Held sinkt"),
];

pub(super) fn definition() -> ToolDefinition {
    ToolDefinition {
        name: ToolName::GetMechanic,
        description: "Liest belegte Spielmechanik-Werte aus den Spieldateien und rechnet sie auf eine Spielminute hoch. Fragt der Nutzer nach einer Spielzeit, minute angeben; Anfang, Spielbeginn oder Start = Minute 0. Ohne minute kommen berechnete Werte für die Referenzminuten 0, 10, 20 und 30 (condition.minute), bei lane_troopers nur für Minute 0; für eine andere Spielzeit minute angeben. Für allgemeine Spielfragen ohne Held oder Item zuerst get_mechanic verwenden, nicht find_entity oder search_docs. Mechaniken: lane_troopers (Lane-Trooper, Wellen, Creeps: Seelen je Trooper und je Welle, Seelen pro Minute, Truppgröße, Wellenabstand, Anteil sofort vs. Seelenorb je Trooper und je Welle, Denien bzw. deny = gegnerischen Seelenorb zuerst abschießen, Trooper-Leben, Verlauf des Orb-Anteils aus den Patchnotes als patch_orb_share_change mit released_at sowie altem und neuem Orb-Anteil; bei Fragen nach früheren Anteilen wie 60/40 alle diese Zeilen mit Datum nennen), soul_sharing (Seelenteilung mit Mitspielern auf der Lane, Anteil je Spieleranzahl), denizens (Jungle-Camps, Neutrals: Seelen, Spawn, Respawn, Leben je Camp-Stufe weak/normal/strong), objectives (Guardian = npc_boss_tier1, Walker = npc_boss_tier2, Patron = npc_boss_tier3, Base Guardians = npc_barrack_boss, Shrines bzw. Schreine = destroyable_building: Leben, Kugel- und Spirit-Resistenz, Resistenz je Zahl naher gegnerischer Helden, Aura für verbündete Helden, zusätzliche Kugelresistenz je Zahl naher Helden, Seelen beim Zerstören, Seelenorbs, Anteil für Spieler in der Nähe, Super Trooper nach zerstörtem Shrine mit Schaden und Seelen; geht die Frage um ein einzelnes Objective, objective angeben (guardian, walker, patron, base_guardian, shrine), sonst werden nicht alle Werte geliefert; bei Belohnungsfragen alle Zeilen mit condition.meaning zum gefragten Objective nennen; Resistenzwerte sind Prozent weniger erlittener Schaden), core_abilities (Grundfähigkeiten aller Helden: Parry bzw. Parieren mit Betäubungsdauer, Schadensbonus auf den parierten Gegner aus allen Quellen und Abklingzeit; Dash und Down Dash mit Ausdauerkosten; Wall Jump und Wall Slide. Parry-Regeln zu Troopern, Objectives wie dem Guardian und Urnenträgern kommen als patch_note_change-Zeilen mit released_at; nenne sie mit Datum. Bezeichne die Parry-Wirkung nie als Stun oder Betäubung, auch wenn eine Patchzeile stun schreibt; sprich von der Parry-Wirkung), mid_boss (Midboss: Seelen, Leben, Spawn, Respawn), rejuvenator (Rejuvenator: Buffdauer, Trooper-Lebensbonus und Respawn-Faktor je erstem/zweitem/drittem Rejuvenator), hero_kills (Startseelen, Kopfgeld für Heldenkills nach Spielzeit, Assist-Anteil, Comeback, Erstkill-Bonus), respawn (Respawnzeit nach Spielzeit), urn (Urne, Urn, Soul Urn, Seelenurne, Idol: erster Spawn, Respawn, Belohnung und Bonus beim Abgeben), breakables (Kisten und zerstörbare Objekte: Spawnzeiten), sinners_sacrifice (Sinner's Sacrifice), item_tiers (Itemkosten je Stufe 1 bis 4, Index 1 = Stufe 1), levels (Seelen pro Level, Ability Points, Ability-Freischaltungen, Power Increase), street_brawl (Spielmodus Street Brawl: Runden, Seelen je Runde, Respawn, Punkte zum Sieg). Zeilen mit derived_ sind aus den belegten Rohwerten berechnet; derived_trooper_* gilt je einzelnem Trooper, derived_wave_* und derived_squad_* je ganzer Welle; condition.meaning erklärt den Spielbegriff; condition.formula nennt die Rechnung und condition.inputs die Rohwerte. Diese Werte darfst du unverändert nennen und mit der Formel erklären. Einheiten: Zeiten in Sekunden, *_time-Grenzen der Trooper-Wellen in Minuten, Spieldauer-Grenzen der Respawn-Rampen in Sekunden. Index 0 in Listen ist der erste Eintrag. Fehlender Patch oder current bedeutet aktueller beobachteter Quellenstand. Zeilen mit stat_key patch_note_change sind datierte Patchnotiz-Zeilen zur Mechanik (neueste zuerst, old_text und new_text aus from X to Y); nennt der Nutzer einen früheren Wert oder fragt nach Änderungen, diese Zeilen mit Datum nennen. Regeln, die nur dort stehen, gelten ab released_at. Fehlende Werte nicht ergänzen.".into(),
        input_schema: json!({
            "type":"object", "additionalProperties":false, "required":["mechanic"],
            "properties":{
                "mechanic":{"type":"string","enum":MechanicKind::ALL.map(MechanicKind::as_str)},
                "minute":{"type":"integer","minimum":0,"maximum":120},
                "patch":{"type":"string","minLength":1,"maxLength":128},
                "objective":{"type":"string","enum":ObjectiveKind::ALL.map(ObjectiveKind::as_str)}
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
        .fetch_all(&mut *connection)
        .await
        .map_err(super::database_error)?;
    if rows.is_empty() {
        return Ok(None);
    }
    let mut value = present(request, rows);
    if request.mechanic == MechanicKind::LaneTroopers {
        let history = orb_history(connection)
            .await
            .ok_or_else(history_unavailable)?;
        insert_orb_history(&mut value, &history);
    }
    if let Some(scope) = history::scope(request) {
        let lines = history::lines(connection, scope)
            .await
            .ok_or_else(history_unavailable)?;
        let at = primary_len(&value);
        history::insert(&mut value, &lines, at);
    }
    Ok(Some(value))
}

pub(super) fn primary_len(value: &Value) -> usize {
    let Some(facts) = value["facts"].as_array() else {
        return 0;
    };
    let inputs: BTreeSet<&str> = facts
        .iter()
        .filter_map(|row| row["condition"]["inputs"].as_array())
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    facts
        .iter()
        .rposition(|row| {
            row["stat_key"].as_str().is_some_and(|stat| {
                stat.starts_with("derived_")
                    || stat.starts_with("patch_")
                    || inputs.contains(stat)
                    || meaning(row).is_some()
            })
        })
        .map_or(0, |last| last + 1)
}

fn history_unavailable() -> PortError {
    PortError::Unavailable("Patchwerte können derzeit nicht sicher gelesen werden.".into())
}

async fn orb_history(connection: &mut PgConnection) -> Option<Vec<Value>> {
    for _ in 0..2 {
        if let Some(rows) = attempt_orb_history(connection).await {
            return Some(rows);
        }
    }
    None
}

async fn attempt_orb_history(connection: &mut PgConnection) -> Option<Vec<Value>> {
    let mut savepoint = sqlx::Connection::begin(&mut *connection).await.ok()?;
    let started = std::time::Instant::now();
    let history = match sqlx::query_scalar::<_, Value>(ORB_HISTORY)
        .fetch_all(&mut *savepoint)
        .await
    {
        Ok(rows) => savepoint.commit().await.map(|_| rows),
        Err(error) => {
            let _ = savepoint.rollback().await;
            Err(error)
        }
    };
    eprintln!(
        "{}",
        json!({
            "event": "brain_orb_history",
            "ok": history.is_ok(),
            "elapsed_ms": started.elapsed().as_millis(),
        })
    );
    history.ok()
}

fn insert_orb_history(value: &mut Value, history: &[Value]) {
    let Some(facts) = value["facts"].as_array_mut() else {
        return;
    };
    let at = facts
        .iter()
        .position(|row| {
            !row["stat_key"]
                .as_str()
                .is_some_and(|stat| stat.starts_with("derived_"))
        })
        .unwrap_or(facts.len());
    let rows: Vec<Value> = history.iter().filter_map(orb_share_change).collect();
    facts.splice(at..at, rows);
}

fn orb_share_change(line: &Value) -> Option<Value> {
    let text = line["line_text"].as_str()?;
    line["citation"].as_str()?;
    line["source_revision"]
        .as_str()
        .filter(|revision| !revision.is_empty())?;
    let released_at = line["released_at"]
        .as_str()
        .filter(|released| !released.is_empty())?;
    let lower = text.to_lowercase();
    let (_, values) = lower.split_once("from ")?;
    let (old, new) = values.split_once(" to ")?;
    let token = |part: &str| {
        part.split_whitespace()
            .next()
            .map(|word| word.trim_end_matches(['.', ',', ';']).to_owned())
    };
    let (old, new) = (token(old)?, token(new)?);
    let ratio = lower.contains("kill/orb");
    let share = |value: &str| {
        let number = value.trim_end_matches('%');
        let part = if ratio {
            let (kill, orb) = number.split_once('/')?;
            kill.parse::<f64>().ok()?;
            orb
        } else {
            number
        };
        part.parse::<f64>()
            .ok()
            .filter(|share| (0.0..=100.0).contains(share))
    };
    let (old_share, new_share) = (share(&old)?, share(&new)?);
    Some(json!({
        "stat_key":"patch_orb_share_change",
        "value":format(new_share),
        "condition":{
            "meaning":ORB_SHARE_MEANING,
            "patch_label":line["patch"],
            "line_text":text,
            "old_text":old,
            "new_text":new,
            "old_orb_share_percent":format(old_share),
            "new_orb_share_percent":format(new_share)
        },
        "released_at":released_at,
        "citation":line["citation"],
        "source_revision":line["source_revision"],
        "patch":null
    }))
}

fn present(request: &GetMechanicRequest, rows: Vec<Value>) -> Value {
    let reference = reference_minutes(request.mechanic);
    let minutes = request
        .minute
        .map_or_else(|| reference.to_vec(), |minute| vec![minute]);
    let rows: Vec<Value> = rows
        .into_iter()
        .filter(|row| belongs_to(row, request.objective))
        .collect();
    let derived = derive(request.mechanic, request.minute, &minutes, &rows);
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
    let (mut concepts, mut rest): (Vec<Value>, Vec<Value>) =
        rest.into_iter().partition(|row| meaning(row).is_some());
    if verified_only(request.mechanic) {
        rest.clear();
    }
    concepts.sort_by_key(meaning_rank);
    let omitted = (used.len() + concepts.len() + rest.len()).saturating_sub(MAX_ROWS);
    let facts: Vec<Value> = derived
        .iter()
        .cloned()
        .chain(used.into_iter().chain(concepts).chain(rest).take(MAX_ROWS))
        .map(explain)
        .collect();
    json!({
        "mechanic":request.mechanic.as_str(),
        "minute":request.minute,
        "objective":request.objective.map(ObjectiveKind::as_str),
        "reference_minutes":request.minute.is_none().then_some(reference),
        "facts":facts,
        "truncated":omitted>0,
        "omitted_count":omitted,
        "omitted_count_is_lower_bound":lower_bound
    })
}

fn belongs_to(row: &Value, objective: Option<ObjectiveKind>) -> bool {
    let Some(objective) = objective else {
        return true;
    };
    let Some(stat) = row["stat_key"].as_str() else {
        return true;
    };
    OBJECTIVE_ROWS
        .iter()
        .find(|(_, prefixes)| prefixes.iter().any(|prefix| stat.contains(prefix)))
        .is_none_or(|(owner, _)| *owner == objective)
}

fn reference_minutes(mechanic: MechanicKind) -> &'static [u16] {
    match mechanic {
        MechanicKind::LaneTroopers => &LANE_REFERENCE_MINUTES,
        _ => &REFERENCE_MINUTES,
    }
}

fn meaning(row: &Value) -> Option<String> {
    let stat = row["stat_key"].as_str()?;
    objective_meaning(stat).or_else(|| {
        meaning_rank(row)
            .and_then(|rank| MEANINGS.get(rank))
            .map(|(_, meaning)| (*meaning).to_owned())
    })
}

fn objective_meaning(stat: &str) -> Option<String> {
    OBJECTIVE_NAMES.iter().find_map(|(prefix, name)| {
        let field = stat.strip_prefix(prefix)?;
        OBJECTIVE_FIELDS
            .iter()
            .find(|(key, _)| match key.strip_suffix('_') {
                Some(list) => field
                    .strip_prefix(list)
                    .and_then(|index| index.strip_prefix('_'))
                    .is_some_and(|index| index.parse::<u8>().is_ok()),
                None => field == *key,
            })
            .map(|(_, meaning)| format!("{name}: {meaning}"))
    })
}

fn meaning_rank(row: &Value) -> Option<usize> {
    row["stat_key"].as_str().and_then(|stat| {
        MEANINGS
            .iter()
            .position(|(pattern, _)| stat.contains(pattern))
            .or_else(|| objective_meaning(stat).map(|_| MEANINGS.len()))
    })
}

fn verified_only(mechanic: MechanicKind) -> bool {
    matches!(
        mechanic,
        MechanicKind::Objectives | MechanicKind::CoreAbilities
    )
}

fn explain(mut row: Value) -> Value {
    if let (Some(meaning), Some(condition)) = (meaning(&row), row["condition"].as_object_mut()) {
        condition.insert("meaning".into(), meaning.into());
    }
    row
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

fn derive(
    mechanic: MechanicKind,
    asked: Option<u16>,
    minutes: &[u16],
    rows: &[Value],
) -> Vec<Value> {
    let rows = Rows(rows);
    match mechanic {
        MechanicKind::SoulSharing => soul_sharing(&rows),
        MechanicKind::Levels => levels(&rows),
        MechanicKind::Urn => urn_timing(&rows)
            .into_iter()
            .chain(minutes.iter().flat_map(|minute| urn(&rows, *minute)))
            .collect(),
        MechanicKind::LaneTroopers => {
            let schedule = asked.is_none().then(|| wave_schedule(&rows)).flatten();
            let scheduled = schedule.is_some();
            let per_minute = minutes
                .iter()
                .flat_map(|minute| lane_troopers(&rows, *minute))
                .filter(|row| {
                    !scheduled || row["stat_key"] != "derived_wave_interval_seconds_at_minute"
                });
            schedule.into_iter().chain(per_minute).collect()
        }
        _ => minutes
            .iter()
            .flat_map(|minute| at_minute(mechanic, &rows, *minute))
            .collect(),
    }
}

fn at_minute(mechanic: MechanicKind, rows: &Rows, minute: u16) -> Vec<Value> {
    match mechanic {
        MechanicKind::Denizens => [
            "neutral_trooper_weak",
            "neutral_trooper_normal",
            "neutral_trooper_strong",
        ]
        .into_iter()
        .filter_map(|camp| camp_souls(rows, camp, minute))
        .collect(),
        MechanicKind::SinnersSacrifice => camp_souls(rows, "neutral_sinners_sacrifice", minute)
            .into_iter()
            .collect(),
        MechanicKind::MidBoss => mid_boss(rows, minute),
        MechanicKind::Respawn => respawn(rows, minute),
        MechanicKind::HeroKills => hero_kill_bounty(rows, minute).into_iter().collect(),
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
    let size = rows.get("citadel_trooper_squad_size");
    if let Some((source, size)) = size {
        output.push(row(
            "derived_squad_souls_at_minute".into(),
            souls * size,
            source,
            "Seelen einer ganzen Welle je Lane = Seelen je Trooper × Truppgröße",
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
        let inputs = [
            "derived_trooper_souls_at_minute",
            instant_key,
            "citadel_trooper_laning_gold_rules_end_time",
        ];
        output.push(row(
            "derived_trooper_souls_instant_at_minute".into(),
            souls * ratio,
            source,
            "Je Trooper sofort beim letzten Treffer = Seelen je Trooper × Sofortanteil",
            &inputs,
            json!({"minute":minute}),
        ));
        output.push(row(
            "derived_trooper_souls_orb_at_minute".into(),
            souls * (1.0 - ratio),
            source,
            "Je Trooper als Seelenorb = Seelen je Trooper × (1 - Sofortanteil)",
            &inputs,
            json!({"minute":minute}),
        ));
        if let Some((_, size)) = size {
            let inputs = [
                "derived_trooper_souls_at_minute",
                instant_key,
                "citadel_trooper_squad_size",
            ];
            output.push(row(
                "derived_wave_souls_instant_at_minute".into(),
                souls * ratio * size,
                source,
                "Je ganzer Welle sofort = Seelen je Trooper × Sofortanteil × Truppgröße",
                &inputs,
                json!({"minute":minute}),
            ));
            output.push(row(
                "derived_wave_souls_orb_at_minute".into(),
                souls * (1.0 - ratio) * size,
                source,
                "Je ganzer Welle als Seelenorbs = Seelen je Trooper × (1 - Sofortanteil) × Truppgröße",
                &inputs,
                json!({"minute":minute}),
            ));
        }
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

fn wave_schedule(rows: &Rows) -> Option<Value> {
    let (source, early) = rows.get("citadel_trooper_spawn_interval_early")?;
    let later = [
        (
            "citadel_trooper_spawn_interval_late_time",
            "citadel_trooper_spawn_interval_late",
        ),
        (
            "citadel_trooper_spawn_interval_very_late_time",
            "citadel_trooper_spawn_interval_very_late",
        ),
    ]
    .into_iter()
    .filter_map(|(time, value)| {
        let (_, start) = rows.get(time)?;
        let (_, seconds) = rows.get(value)?;
        Some(format!("{} s ab Minute {}", format(seconds), format(start)))
    });
    let steps: Vec<String> = std::iter::once(format!("{} s ab Minute 0", format(early)))
        .chain(later)
        .collect();
    Some(json!({
        "stat_key":"derived_wave_interval_schedule",
        "value":steps.join("; "),
        "condition":{
            "formula":"Abstand der Trooper-Wellen, je Stufe gültig ab der Minute",
            "inputs":[
                "citadel_trooper_spawn_interval_early",
                "citadel_trooper_spawn_interval_late",
                "citadel_trooper_spawn_interval_late_time",
                "citadel_trooper_spawn_interval_very_late",
                "citadel_trooper_spawn_interval_very_late_time"
            ]
        },
        "citation":source["citation"],
        "source_revision":source["source_revision"],
        "patch":null
    }))
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

fn urn_timing(rows: &Rows) -> Vec<Value> {
    [
        (
            "citadel_crate_spawn_initial_delay",
            "derived_urn_first_spawn_minute",
            "Erster Urnen-Spawn in Spielminuten = Verzögerung in Sekunden / 60",
        ),
        (
            "citadel_crate_respawn_interval",
            "derived_urn_respawn_interval_minutes",
            "Abstand bis zum nächsten Urnen-Spawn in Minuten = Sekunden / 60",
        ),
    ]
    .into_iter()
    .filter_map(|(input, stat, formula)| {
        let (source, seconds) = rows.get(input)?;
        Some(row(
            stat.into(),
            seconds / 60.0,
            source,
            formula,
            &[input],
            json!({}),
        ))
    })
    .collect()
}

fn hero_kill_bounty(rows: &Rows, minute: u16) -> Option<Value> {
    let (source, low) = rows.get("citadel_player_gold_reward_min")?;
    let (_, high) = rows.get("citadel_player_gold_reward_max")?;
    let (_, time) = rows.get("citadel_player_gold_reward_time")?;
    if time <= 0.0 {
        return None;
    }
    let seconds = f64::from(minute) * 60.0;
    Some(row(
        "derived_hero_kill_bounty_at_minute".into(),
        low + (high - low) * (seconds / time).min(1.0),
        source,
        "Grundkopfgeld eines Heldenkills ohne Comeback- und Erstkill-Bonus = Minimum + (Maximum - Minimum) × min(Spielzeit in Sekunden / citadel_player_gold_reward_time, 1); linearer Verlauf zwischen Minimum und Maximum angenommen",
        &[
            "citadel_player_gold_reward_min",
            "citadel_player_gold_reward_max",
            "citadel_player_gold_reward_time",
        ],
        json!({"minute":minute,"game_seconds":seconds}),
    ))
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
