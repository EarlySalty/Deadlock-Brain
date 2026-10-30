use brain_contracts::tools::{GetMechanicRequest, MechanicKind, ObjectiveKind};
use serde_json::{json, Value};
use sqlx::PgConnection;

const QUERY: &str = include_str!("mechanic_patch_history.sql");
const LIMIT: i64 = 12;
const CORE_ABILITY_LIMIT: i64 = 16;
const ATTEMPTS: u8 = 2;
pub(super) const MEANING: &str = "patch_note_change-Zeilen sind Patchnotiz-Zeilen zu dieser Mechanik mit Datum, neueste zuerst. Steht in line_text from X to Y, galt old_text bis released_at und new_text gilt ab released_at. Nennt der Nutzer einen früheren Wert, mit diesen Zeilen sagen, wann er galt und seit wann der aktuelle Wert gilt. Regeln, die nur in diesen Zeilen stehen, gelten ab released_at, solange keine neuere Zeile sie ändert. Die aktuellen Werte stehen in allen übrigen Zeilen und haben Vorrang: nenne sie immer vollständig, die Patchzeilen nur ergänzend als Verlauf.";

#[derive(Clone, Copy)]
pub(super) struct Scope {
    pattern: &'static str,
    exclude: &'static str,
    section_exclude: &'static str,
    buckets: &'static [(&'static str, i32)],
    limit: i64,
}

const STREET_BRAWL: &str = r"street brawl";
const URN_BUCKETS: &[(&str, i32)] = &[
    (
        r"deposit.*(timer|phase|immediately)|drop ?off channel|capture point|king of the hill",
        5,
    ),
    (r"\mparr", 1),
    (r"spawn", 1),
    (r"bounty|reward|\msouls\M|buffs?\M", 2),
    (r"runner|carrier", 1),
    (r"pick ?up|dropped|\mdrops?\M|fumble", 1),
    (r"decay|damage|held timer|give up", 1),
];
const CORE_ABILITY_BUCKETS: &[(&str, i32)] = &[
    (r"parr.*troopers?|troopers?.*parr", 3),
    (r"parr.*(objective|guardian|boss)", 1),
    (r"parr.*reload", 1),
    (r"\murn\M.*parr|parr.*\murn\M", 2),
    (r"parr.*dash|dash.*parr", 2),
    (r"\mparr", 1),
    (r"wall (jump|slide)", 2),
    (r"\mdash", 2),
    (r"stamina", 2),
];

pub(super) fn scope(request: &GetMechanicRequest) -> Option<Scope> {
    let (pattern, exclude) = scope_terms(request)?;
    let section_exclude = match request.mechanic {
        MechanicKind::StreetBrawl => "",
        _ => STREET_BRAWL,
    };
    let (buckets, limit) = match request.mechanic {
        MechanicKind::Urn => (URN_BUCKETS, LIMIT),
        MechanicKind::CoreAbilities => (CORE_ABILITY_BUCKETS, CORE_ABILITY_LIMIT),
        _ => (&[][..], LIMIT),
    };
    Some(Scope {
        pattern,
        exclude,
        section_exclude,
        buckets,
        limit,
    })
}

fn scope_terms(request: &GetMechanicRequest) -> Option<(&'static str, &'static str)> {
    let troopers = r"\mtroopers?\M.*(bounty|souls?|gold|health|\mhp\M|damage|resist|interval|spawn)|wave interval|\mwaves?\M.*(every|interval)";
    Some(match request.mechanic {
        MechanicKind::LaneTroopers => (troopers, r"kill/orb|flying orb|neutral|\murn\M|\mrift\M"),
        MechanicKind::SoulSharing => (r"soul shar|split efficiency|bounty split", ""),
        MechanicKind::HeroKills => (
            r"hero kill|kill bounty|kill value|kill souls|kill gold|starting souls|first blood|comeback|assister|unsecured",
            "",
        ),
        MechanicKind::Respawn => (
            r"respawn",
            r"neutral|\mcamps?\M|denizen|sinner|mid ?boss|\murn\M|breakable|crystal|rejuv|music|sound",
        ),
        MechanicKind::Objectives => objective(request.objective),
        MechanicKind::MidBoss => (r"mid ?boss", ""),
        MechanicKind::Rejuvenator => (r"rejuv|crystal", r"rejuvenating"),
        MechanicKind::Urn => (r"\murn\M|\midol\M", r"^unstable rift"),
        MechanicKind::Denizens => (r"neutrals?\M|denizen|\mcamps?\M", r"neutral shop"),
        MechanicKind::Breakables => (r"breakable|\mcrates?\M", ""),
        MechanicKind::SinnersSacrifice => (r"sinner", ""),
        MechanicKind::Levels => (r"\mboons?\M|ability points?|power increase", ""),
        MechanicKind::CoreAbilities => (
            r"\mparry|\mparried|\mdash|wall jump|wall slide|stamina",
            r"parr.*(to be stunned|repeatedly melee)",
        ),
        MechanicKind::StreetBrawl => (STREET_BRAWL, ""),
        MechanicKind::ItemTiers => return None,
    })
}

fn objective(kind: Option<ObjectiveKind>) -> (&'static str, &'static str) {
    match kind {
        Some(ObjectiveKind::Guardian) => (
            r"\mguardians?\M",
            r"base guardian|guardian ward|barrack|parr.*(to be stunned|repeatedly melee)",
        ),
        Some(ObjectiveKind::Walker) => (r"\mwalkers?\M", r"veil ?walker"),
        Some(ObjectiveKind::Patron) => (r"\mpatron", ""),
        Some(ObjectiveKind::BaseGuardian) => (r"base guardians?|barrack", ""),
        Some(ObjectiveKind::Shrine) => (r"\mshrines?\M|super troopers?", ""),
        None => (
            r"\mguardians?\M|\mwalkers?\M|\mpatron|\mshrines?\M|objective|super troopers?",
            r"guardian ward|veil ?walker|parr.*(to be stunned|repeatedly melee)",
        ),
    }
}

pub(super) async fn lines(connection: &mut PgConnection, scope: Scope) -> Option<Vec<Value>> {
    for attempt in 1..=ATTEMPTS {
        if let Some(rows) = attempt_lines(connection, scope, attempt).await {
            return Some(rows);
        }
    }
    None
}

async fn attempt_lines(
    connection: &mut PgConnection,
    scope: Scope,
    attempt: u8,
) -> Option<Vec<Value>> {
    let mut savepoint = sqlx::Connection::begin(&mut *connection).await.ok()?;
    let started = std::time::Instant::now();
    let (patterns, keeps): (Vec<&str>, Vec<i32>) = scope.buckets.iter().copied().unzip();
    let history = match sqlx::query_scalar::<_, Value>(QUERY)
        .bind(scope.pattern)
        .bind(scope.exclude)
        .bind(scope.limit)
        .bind(scope.section_exclude)
        .bind(patterns)
        .bind(keeps)
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
            "event": "brain_mechanic_patch_history",
            "ok": history.is_ok(),
            "attempt": attempt,
            "rows": history.as_ref().map_or(0, Vec::len),
            "elapsed_ms": started.elapsed().as_millis(),
        })
    );
    history.ok()
}

pub(super) fn insert(value: &mut Value, history: &[Value], at: usize) {
    let rows: Vec<Value> = history.iter().filter_map(change).collect();
    if rows.is_empty() {
        return;
    }
    let Some(facts) = value["facts"].as_array_mut() else {
        return;
    };
    let at = at.min(facts.len());
    facts.splice(at..at, rows);
    value["patch_history_meaning"] = json!(MEANING);
}

fn change(line: &Value) -> Option<Value> {
    let text = line["line_text"].as_str()?;
    line["citation"].as_str()?;
    line["source_revision"]
        .as_str()
        .filter(|revision| !revision.is_empty())?;
    let released_at = line["released_at"]
        .as_str()
        .filter(|released| !released.is_empty())?;
    let mut condition = json!({"patch_label":line["patch"],"line_text":text});
    if let Some((old, new)) = from_to(text) {
        condition["old_text"] = old.into();
        condition["new_text"] = new.into();
    }
    Some(json!({
        "stat_key":"patch_note_change",
        "condition":condition,
        "released_at":released_at.get(..10).unwrap_or(released_at),
        "citation":line["citation"],
        "source_revision":line["source_revision"],
        "patch":null
    }))
}

pub(super) fn from_to(text: &str) -> Option<(String, String)> {
    let lower = text.to_lowercase();
    let start = lower.find(" from ")? + " from ".len();
    let rest = text.get(start..)?;
    let split = rest.to_lowercase().find(" to ")?;
    let old = rest.get(..split)?.trim();
    let tail = rest.get(split + " to ".len()..)?;
    let end = tail.find(['(', ';']).unwrap_or(tail.len());
    let new = tail
        .get(..end)?
        .trim()
        .trim_end_matches(['.', ',', ';'])
        .trim();
    let numeric = |part: &str| part.bytes().any(|byte| byte.is_ascii_digit());
    (numeric(old) && numeric(new) && old.len() <= 60 && new.len() <= 60)
        .then(|| (old.to_owned(), new.to_owned()))
}
