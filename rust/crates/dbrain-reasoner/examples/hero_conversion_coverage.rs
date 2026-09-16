//! Roster-Abdeckung der Spirit->Waffen-Feuerraten-Konversion (Modell-Ebene).
//!
//! Liest die eingefrorene Phase-0-Datei speicherarm (nur das Feld `heroes` wird
//! typisiert deserialisiert, alle anderen Felder verwirft serde). Grundlage ist
//! die bereits normalisierte `HeroModel.scaling`, NICHT der rohe Asset-Snapshot.
//! Der Bericht ist damit eine Modell-Abdeckung: er belegt, was der Loader je Held
//! ins Modell gezogen hat, und ist kein Beweis fuer die Vollstaendigkeit des
//! Asset-Parsings selbst.
//!
//! Bewusst getrennt ausgewiesen werden die zwei heute im Produktivcode
//! vorhandenen Auslegungen derselben Rohdaten, weil sie sich bei negativen oder
//! nicht endlichen Skalen unterscheiden:
//!   - `combat_rate_per_spirit`  wie combat.rs (Stand vor Phase A): Wert
//!     ungefiltert, Vorzeichen bleibt erhalten.
//!   - `score_rate_per_spirit`   wie item.rs::spirit_fire_rate_value: negative
//!     oder nicht endliche Skalen werden als 0 behandelt.
//! `sign_divergence` markiert Helden, bei denen beide Pfade heute abweichen.
//!
//! Aufruf: hero_conversion_coverage FROZEN.json [AUSGABE.json]

use dbrain_reasoner::HeroModel;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fs::File, io::BufReader, path::Path};

mod support;

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
    combat_rate_per_spirit: f64,
    score_rate_per_spirit: f64,
    converter: bool,
    negative_or_nonfinite: bool,
    sign_divergence: bool,
}

fn per_spirit(hero: &HeroModel, stat: &str) -> Option<f64> {
    hero.scaling
        .iter()
        .find(|s| s.stat == stat)
        .and_then(|s| s.per_spirit)
}

// combat.rs (Stand vor Phase A): ungefiltert, Vorzeichen bleibt.
fn combat_rate(hero: &HeroModel, rounds: Option<f64>, fire_rate: Option<f64>) -> f64 {
    rounds
        .or_else(|| fire_rate.map(|v| v * hero.weapon.shots_per_second / 100.0))
        .unwrap_or(0.0)
}

// item.rs::spirit_fire_rate_value: negativ/nicht endlich -> als fehlend behandelt.
fn score_rate(hero: &HeroModel, rounds: Option<f64>, fire_rate: Option<f64>) -> f64 {
    let usable = |value: Option<f64>| value.filter(|v| v.is_finite() && *v >= 0.0);
    usable(rounds)
        .or_else(|| usable(fire_rate).map(|v| hero.weapon.shots_per_second * v / 100.0))
        .unwrap_or(0.0)
}

// Ein- und Ausgabepfad duerfen nicht identisch sein, sonst wuerde die Eingabe
// als Ausgabe behandelt (support::write_new lehnt zusaetzlich bestehende Dateien ab).
fn validate_paths(input: &Path, output: &Path) -> Result<(), Error> {
    if input == output {
        return Err("Ausgabepfad darf nicht die Eingabedatei sein".into());
    }
    Ok(())
}

fn coverage_row(hero: &HeroModel) -> HeroCoverage {
    let rounds = per_spirit(hero, "ERoundsPerSecond");
    let fire_rate = per_spirit(hero, "EFireRate");
    let combat = combat_rate(hero, rounds, fire_rate);
    let score = score_rate(hero, rounds, fire_rate);
    let negative_or_nonfinite = [rounds, fire_rate]
        .into_iter()
        .flatten()
        .any(|v| !v.is_finite() || v < 0.0);
    HeroCoverage {
        hero_id: hero.hero_id,
        name: hero.name.clone(),
        shots_per_second: hero.weapon.shots_per_second,
        e_rounds_per_second_per_spirit: rounds,
        e_fire_rate_per_spirit: fire_rate,
        combat_rate_per_spirit: combat,
        score_rate_per_spirit: score,
        // Konverter: der Score-Pfad zieht positiven Waffennutzen aus Spirit.
        converter: score > 0.0,
        negative_or_nonfinite,
        // Endliche Differenz oder NaN-Ungleichheit zwischen beiden Auslegungen.
        sign_divergence: combat != score || combat.is_nan() != score.is_nan(),
    }
}

fn main() -> Result<(), Error> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let input = args
        .first()
        .ok_or("Aufruf: hero_conversion_coverage FROZEN.json [AUSGABE.json]")?;
    if let Some(output) = args.get(1) {
        // Eingabe darf nicht die Ausgabe sein und eine bestehende Baseline nie
        // ueberschrieben werden (support::write_new nutzt create_new).
        validate_paths(Path::new(input), Path::new(output))?;
    }
    let frozen: CoverageFrozen = serde_json::from_reader(BufReader::new(File::open(input)?))?;

    let mut rows = frozen
        .heroes
        .iter()
        .map(|entry| coverage_row(&entry.hero))
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| {
        b.score_rate_per_spirit
            .total_cmp(&a.score_rate_per_spirit)
            .then_with(|| a.hero_id.cmp(&b.hero_id))
    });

    let converters = rows.iter().filter(|r| r.converter).count();
    let flagged = rows.iter().filter(|r| r.negative_or_nonfinite).count();
    let divergent = rows.iter().filter(|r| r.sign_divergence).count();
    let summary = json!({
        "coverage_kind": "normalisierte HeroModel.scaling (Modell-Abdeckung, kein Roh-Asset-Parsebeweis)",
        "hero_count": rows.len(),
        "converters": converters,
        "null_converters": rows.len() - converters,
        "negative_or_nonfinite": flagged,
        "sign_divergence_combat_vs_score": divergent,
        "heroes": rows,
    });
    let text = serde_json::to_vec_pretty(&summary)?;
    if let Some(output) = args.get(1) {
        support::write_new(Path::new(output), &text)?;
        eprintln!(
            "Abdeckung: {} Helden, {converters} Konverter, {} Nullkonverter, {flagged} negativ/nicht endlich, {divergent} Vorzeichen-Divergenz",
            rows.len(),
            rows.len() - converters
        );
    } else {
        println!("{}", String::from_utf8(text)?);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dbrain_reasoner::{DamagePlan, DamageType, PurchaseBonuses, ScalingStat, WeaponProfile};

    fn hero_with(scaling: Vec<ScalingStat>, shots_per_second: f64) -> HeroModel {
        HeroModel {
            base_spirit_power: 0.0,
            standard_level_up_upgrades: Default::default(),
            standard_upgrade_levels: Default::default(),
            level_rewards: Default::default(),
            cost_bonuses: Default::default(),
            hero_id: 1,
            name: "Fixture".into(),
            archetype: String::new(),
            base_health: 600.0,
            level_curve: Vec::new(),
            purchase_bonuses: PurchaseBonuses {
                spirit: Vec::new(),
                weapon: Vec::new(),
                vitality: Vec::new(),
            },
            scaling,
            weapon: WeaponProfile {
                bullet_damage: 20.0,
                shots_per_second,
                clip_size: 16.0,
                reload_duration: 2.0,
                range: 20.0,
                falloff_start_range: 20.0,
                falloff_end_range: 50.0,
                sustained_dps: 0.0,
            },
            abilities: Vec::new(),
            damage_plan: DamagePlan {
                weapon_dps: 0.0,
                spirit_dps: 0.0,
                weapon_share: 1.0,
                primary_axis: DamageType::Weapon,
            },
        }
    }

    fn scaling(stat: &str, per_spirit: Option<f64>) -> ScalingStat {
        ScalingStat {
            stat: stat.into(),
            per_level: 0.0,
            per_spirit,
        }
    }

    #[test]
    fn negative_scale_diverges_between_combat_and_score() {
        let hero = hero_with(vec![scaling("ERoundsPerSecond", Some(-0.1))], 4.0);
        let row = coverage_row(&hero);
        // combat.rs behaelt das Vorzeichen, item.rs verwirft es -> Divergenz.
        assert_eq!(row.combat_rate_per_spirit, -0.1);
        assert_eq!(row.score_rate_per_spirit, 0.0);
        assert!(row.negative_or_nonfinite);
        assert!(row.sign_divergence);
        assert!(!row.converter);
    }

    #[test]
    fn positive_rounds_take_precedence_and_agree() {
        let hero = hero_with(
            vec![
                scaling("ERoundsPerSecond", Some(0.25)),
                scaling("EFireRate", Some(101.0)),
            ],
            4.0,
        );
        let row = coverage_row(&hero);
        // ERoundsPerSecond direkt, EFireRate nie zusaetzlich addiert.
        assert_eq!(row.combat_rate_per_spirit, 0.25);
        assert_eq!(row.score_rate_per_spirit, 0.25);
        assert!(!row.sign_divergence);
        assert!(row.converter);
    }

    #[test]
    fn fire_rate_percent_is_only_a_fallback() {
        let hero = hero_with(vec![scaling("EFireRate", Some(50.0))], 4.0);
        let row = coverage_row(&hero);
        // 50 % von 4 Schuss/s = 2.0 Schuss/s je Spirit.
        assert_eq!(row.combat_rate_per_spirit, 2.0);
        assert_eq!(row.score_rate_per_spirit, 2.0);
        assert!(row.converter);
    }

    #[test]
    fn missing_conversion_is_a_null_converter() {
        let hero = hero_with(vec![scaling("ESomethingElse", Some(1.0))], 4.0);
        let row = coverage_row(&hero);
        assert_eq!(row.combat_rate_per_spirit, 0.0);
        assert_eq!(row.score_rate_per_spirit, 0.0);
        assert!(!row.converter);
        assert!(!row.negative_or_nonfinite);
    }

    #[test]
    fn output_path_must_differ_from_input() {
        // Gleicher Pfad fuer Ein- und Ausgabe wird abgelehnt, bevor geschrieben wird.
        let same = std::env::temp_dir().join("dbrain-coverage-same.json");
        assert!(validate_paths(&same, &same).is_err());
        let other = std::env::temp_dir().join("dbrain-coverage-other.json");
        assert!(validate_paths(&same, &other).is_ok());
    }
}
