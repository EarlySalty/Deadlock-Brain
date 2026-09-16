//! Roster-Abdeckung der Spirit->Waffen-Feuerraten-Konversion.
//!
//! Liest die eingefrorene Phase-0-Datei speicherarm (nur das Feld `heroes` wird
//! typisiert deserialisiert, alle anderen Felder verwirft serde). Fuer jeden
//! Helden wird belegt, ob und mit welchem Wert die Assets eine Spirit->Feuerrate-
//! Konversion tragen (`ERoundsPerSecond` direkt, sonst `EFireRate` als Prozent-
//! Fallback). Namen dienen nur dem Bericht, nicht der Bewertung.
//!
//! Aufruf: hero_conversion_coverage FROZEN.json [AUSGABE.json]

use dbrain_reasoner::HeroModel;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fs::File, io::BufReader, path::Path};

type Error = Box<dyn std::error::Error>;

#[derive(Deserialize)]
struct CoverageHero {
    hero: HeroModel,
}

#[derive(Deserialize)]
struct CoverageFrozen {
    heroes: Vec<CoverageHero>,
}

#[derive(Serialize)]
struct HeroCoverage {
    hero_id: i64,
    name: String,
    shots_per_second: f64,
    e_rounds_per_second_per_spirit: Option<f64>,
    e_fire_rate_per_spirit: Option<f64>,
    // Effektive Konversion: Schuss/s je 1 Spirit nach derselben Regel wie
    // combat.rs/item.rs (ERoundsPerSecond direkt, EFireRate nur als Fallback).
    effective_rate_per_spirit: f64,
    converter: bool,
    negative_or_nonfinite: bool,
}

fn per_spirit(hero: &HeroModel, stat: &str) -> Option<f64> {
    hero.scaling
        .iter()
        .find(|s| s.stat == stat)
        .and_then(|s| s.per_spirit)
}

fn main() -> Result<(), Error> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let input = args
        .first()
        .ok_or("Aufruf: hero_conversion_coverage FROZEN.json [AUSGABE.json]")?;
    let frozen: CoverageFrozen =
        serde_json::from_reader(BufReader::new(File::open(input)?))?;

    let mut rows = Vec::new();
    for entry in &frozen.heroes {
        let hero = &entry.hero;
        let rounds = per_spirit(hero, "ERoundsPerSecond");
        let fire_rate = per_spirit(hero, "EFireRate");
        let usable = |value: Option<f64>| value.filter(|v| v.is_finite() && *v >= 0.0);
        let effective = usable(rounds)
            .or_else(|| usable(fire_rate).map(|v| hero.weapon.shots_per_second * v / 100.0))
            .unwrap_or(0.0);
        let negative_or_nonfinite = [rounds, fire_rate]
            .into_iter()
            .flatten()
            .any(|v| !v.is_finite() || v < 0.0);
        rows.push(HeroCoverage {
            hero_id: hero.hero_id,
            name: hero.name.clone(),
            shots_per_second: hero.weapon.shots_per_second,
            e_rounds_per_second_per_spirit: rounds,
            e_fire_rate_per_spirit: fire_rate,
            effective_rate_per_spirit: effective,
            converter: effective > 0.0,
            negative_or_nonfinite,
        });
    }
    rows.sort_by(|a, b| {
        b.effective_rate_per_spirit
            .total_cmp(&a.effective_rate_per_spirit)
            .then_with(|| a.hero_id.cmp(&b.hero_id))
    });

    let converters = rows.iter().filter(|r| r.converter).count();
    let flagged = rows.iter().filter(|r| r.negative_or_nonfinite).count();
    let summary = json!({
        "hero_count": rows.len(),
        "converters": converters,
        "null_converters": rows.len() - converters,
        "negative_or_nonfinite": flagged,
        "heroes": rows,
    });
    let text = serde_json::to_string_pretty(&summary)?;
    if let Some(output) = args.get(1) {
        std::fs::write(Path::new(output), &text)?;
        eprintln!(
            "Abdeckung: {} Helden, {converters} Konverter, {} Nullkonverter, {flagged} negativ/nicht endlich",
            rows.len(),
            rows.len() - converters
        );
    } else {
        println!("{text}");
    }
    Ok(())
}
