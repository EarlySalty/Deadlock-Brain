//! Read-only test surface for the existing planner. No AI, persistence or publish.
use crate::{ReasonerConfig, ReasonerCtx, ReasonerError, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LabRequest {
    pub hero: String,
    #[serde(default = "default_window")]
    pub combat_window_seconds: f64,
    #[serde(default = "default_channel")]
    pub channel_uptime: f64,
    #[serde(default = "default_weapon_share")]
    pub incoming_weapon_share: f64,
    #[serde(default)]
    pub incoming_pressure_dps: Option<f64>,
}
fn default_window() -> f64 {
    40.0
}
fn default_channel() -> f64 {
    0.55
}
fn default_weapon_share() -> f64 {
    0.5
}

impl LabRequest {
    pub fn config(&self) -> Result<ReasonerConfig> {
        if self.hero.trim().is_empty()
            || self.hero.len() > 80
            || self.hero.chars().any(char::is_control)
        {
            return Err(ReasonerError::Data(
                "Bitte einen gültigen Helden wählen.".into(),
            ));
        }
        for (value, min, max, name) in [
            (self.combat_window_seconds, 5.0, 120.0, "Kampffenster"),
            (self.channel_uptime, 0.0, 1.0, "Kanal-Uptime"),
            (
                self.incoming_weapon_share,
                0.0,
                1.0,
                "Eingehender Waffenanteil",
            ),
        ] {
            if !value.is_finite() || !(min..=max).contains(&value) {
                return Err(ReasonerError::Data(format!(
                    "{name}: Wert muss zwischen {min} und {max} liegen."
                )));
            }
        }
        if self
            .incoming_pressure_dps
            .is_some_and(|v| !v.is_finite() || !(0.0..=2000.0).contains(&v))
        {
            return Err(ReasonerError::Data(
                "Eingehender Schaden muss zwischen 0 und 2000 DPS liegen.".into(),
            ));
        }
        Ok(ReasonerConfig {
            patch_tag: "current".into(),
            use_ai: false,
            combat_window_seconds: self.combat_window_seconds,
            channel_uptime: self.channel_uptime,
            incoming_weapon_share: self.incoming_weapon_share,
            incoming_pressure_dps: self.incoming_pressure_dps,
            ..Default::default()
        })
    }
}

#[derive(Debug, Serialize)]
pub struct SourceStatus {
    pub source: String,
    pub oldest_fetched_at: Option<f64>,
    pub newest_fetched_at: Option<f64>,
    pub fields: usize,
    pub fields_without_timestamp: usize,
}

#[derive(Serialize)]
pub struct LabReport {
    pub schema_version: u32,
    pub mode: &'static str,
    pub publish_allowed: bool,
    pub model_fingerprint: String,
    pub config: ReasonerConfig,
    pub sources: Vec<SourceStatus>,
    pub warnings: Vec<String>,
    pub build: crate::BuildObject,
    pub hero: crate::HeroModel,
    pub scored: Vec<crate::ScoredItem>,
    pub variant_scores: BTreeMap<String, Vec<crate::ScoredItem>>,
    pub patch_deltas: Vec<crate::PatchDelta>,
}

fn source_status(snapshots: &[crate::PatchSnapshot]) -> Vec<SourceStatus> {
    let mut sources = BTreeMap::<String, SourceStatus>::new();
    for field in snapshots.iter().flat_map(|s| s.fields.values()) {
        let status = sources
            .entry(field.source.clone())
            .or_insert_with(|| SourceStatus {
                source: field.source.clone(),
                oldest_fetched_at: None,
                newest_fetched_at: None,
                fields: 0,
                fields_without_timestamp: 0,
            });
        status.fields += 1;
        if let Some(time) = field.fetched_at.filter(|v| v.is_finite() && *v > 0.0) {
            status.oldest_fetched_at =
                Some(status.oldest_fetched_at.map_or(time, |old| old.min(time)));
            status.newest_fetched_at =
                Some(status.newest_fetched_at.map_or(time, |old| old.max(time)));
        } else {
            status.fields_without_timestamp += 1;
        }
    }
    sources.into_values().collect()
}

/// Always recalculates from the loaded current models. The semaphore is shared
/// with the normal reasoner, and stays held until blocking CPU work has ended,
/// including when an HTTP caller disconnects or times out.
pub async fn build(pool: sqlx::PgPool, request: LabRequest) -> Result<LabReport> {
    let config = request.config()?;
    let permit = crate::PLANNING_SLOTS.try_acquire().map_err(|_| {
        ReasonerError::Data("Build-Labor ist ausgelastet; bitte erneut versuchen.".into())
    })?;
    let ctx = crate::effective_context(&ReasonerCtx {
        pool,
        ai: None,
        config,
    })
    .await?;
    let (hero, items, meta, snapshots) =
        crate::load_reasoning_inputs(&ctx, request.hero.trim(), None).await?;
    let events = crate::load_patch_events_for_snapshots(&ctx, hero.hero_id, &snapshots).await?;
    let bytes = serde_json::to_vec(&(&hero, &items, &snapshots, &ctx.config))
        .map_err(|e| ReasonerError::Data(format!("Modell-Fingerprint: {e}")))?;
    let model_fingerprint = format!("{:x}", Sha256::digest(bytes));
    let sources = source_status(&snapshots);
    tokio::task::spawn_blocking(move || {
        let _permit = permit;
        let mut plan = crate::plan_build(&hero, &items, &meta, &events, &snapshots, &ctx.config)?;
        crate::annotate_missing_authors(&mut plan.build, &meta);
        for variant in &mut plan.build.variants { crate::annotate_missing_authors(variant, &meta); }
        let mut warnings = vec![
            "Experimentelles Testlabor: keine fachliche Releasefreigabe und keine Veröffentlichung in Steam.".into(),
            "Scores sind Modellwerte, keine Winrate und keine vollständige In-Game-Simulation. Unbekannte Effekte stehen in den Item-Belegen.".into(),
            "Der Modell-Fingerprint umfasst Helden-, Item-, Snapshot- und Szenariodaten, nicht die gesamte Population.".into(),
        ];
        let patch_start = crate::families::FamilyPolicy::for_patch(&events, &ctx.config).patch_started_at;
        if patch_start.is_none() {
            warnings.push("Der Patchbeginn ist nicht sicher zugeordnet; Aktualität der Population ist nicht nachgewiesen.".into());
        }
        for source in &sources {
            if source.fields_without_timestamp > 0 {
                warnings.push(format!("{}: {} Modellfelder ohne Quellzeitstempel.", source.source, source.fields_without_timestamp));
            }
            if let (Some(oldest), Some(start)) = (source.oldest_fetched_at, patch_start) {
                if oldest < start as f64 {
                    warnings.push(format!("{} enthält Werte vor dem Patch. Nur nachweislich zuordenbare Patch-Deltas werden angewandt; dies ersetzt keinen vollständigen aktuellen Datenimport.", source.source));
                }
            }
        }
        for build in std::iter::once(&plan.build).chain(&plan.build.variants) {
            if let Some(family) = &build.family {
                if family.post_patch_player_matches.unwrap_or(0) < 100 {
                    warnings.push(format!("{}: nur {} belegte Spieler-Matches nach dem Patch; historische Daten sind kein aktueller Wirksamkeitsnachweis.", family.label, family.post_patch_player_matches.unwrap_or(0)));
                }
            } else {
                warnings.push(format!("{}: keine empirisch belegte Buildfamilie; mechanischer Testvorschlag.", build.name));
            }
        }
        Ok(LabReport {
            schema_version: 1, mode: "read_only_experimental", publish_allowed: false,
            model_fingerprint, config: ctx.config, sources, warnings,
            build: plan.build, hero: plan.hero, scored: plan.scored,
            variant_scores: plan.variant_scores, patch_deltas: plan.deltas,
        })
    }).await.map_err(|_| ReasonerError::Data("Buildberechnung wurde abgebrochen.".into()))?
}

#[derive(Serialize)]
pub struct HeroChoice {
    pub id: i64,
    pub name: String,
}
#[derive(Serialize)]
pub struct CatalogSource {
    pub source: String,
    pub entity_type: String,
    pub fetched_at: Option<f64>,
}
#[derive(Serialize)]
pub struct LabCatalog {
    pub heroes: Vec<HeroChoice>,
    pub patch_tag: String,
    pub sources: Vec<CatalogSource>,
    pub mode: &'static str,
}
pub async fn catalog(pool: &sqlx::PgPool) -> Result<LabCatalog> {
    let heroes = sqlx::query_as::<_, (i64, String)>(
        "SELECT hero_id, name FROM brain.hero_catalog ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(ReasonerError::Db)?
    .into_iter()
    .map(|(id, name)| HeroChoice { id, name })
    .collect();
    let sources = sqlx::query_as::<_, (String, String, Option<f64>)>(
        "SELECT source, entity_type, EXTRACT(EPOCH FROM MAX(fetched_at))::float8 FROM brain.entity_snapshots WHERE source='deadlock_assets_api' GROUP BY source, entity_type ORDER BY source, entity_type")
        .fetch_all(pool).await.map_err(ReasonerError::Db)?
        .into_iter().map(|(source, entity_type, fetched_at)| CatalogSource { source, entity_type, fetched_at }).collect();
    let patch_tag = dbrain_builds::latest_patch_tag(pool)
        .await
        .map_err(ReasonerError::Db)?;
    Ok(LabCatalog {
        heroes,
        patch_tag,
        sources,
        mode: "read_only_experimental",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn request() -> LabRequest {
        serde_json::from_value(json!({"hero":"Warden"})).unwrap()
    }
    #[test]
    fn lab_is_current_and_never_enables_ai() {
        let cfg = request().config().unwrap();
        assert!(!cfg.use_ai);
        assert_eq!(cfg.patch_tag, "current");
        assert_eq!(cfg.combat_window_seconds, 40.0);
    }
    #[test]
    fn cannot_smuggle_publish_persistence_patch_or_ai() {
        for field in ["publish", "persist", "use_ai", "patch_tag", "seed_path"] {
            let mut v = json!({"hero":"Warden"});
            v[field] = json!(true);
            assert!(serde_json::from_value::<LabRequest>(v).is_err(), "{field}");
        }
    }
    #[test]
    fn invalid_numeric_inputs_are_not_silently_clamped() {
        for value in [f64::NAN, f64::INFINITY, -1.0, 121.0] {
            let mut r = request();
            r.combat_window_seconds = value;
            assert!(r.config().is_err());
        }
        let mut r = request();
        r.channel_uptime = 1.01;
        assert!(r.config().is_err());
        let mut r = request();
        r.incoming_weapon_share = -0.01;
        assert!(r.config().is_err());
        let mut r = request();
        r.incoming_pressure_dps = Some(f64::NAN);
        assert!(r.config().is_err());
    }
    #[test]
    fn valid_boundary_scenarios_are_preserved() {
        let mut r = request();
        r.incoming_pressure_dps = Some(0.0);
        r.incoming_weapon_share = 1.0;
        r.channel_uptime = 0.0;
        let cfg = r.config().unwrap();
        assert_eq!(cfg.incoming_pressure_dps, Some(0.0));
        assert_eq!(cfg.channel_uptime, 0.0);
        assert_eq!(cfg.incoming_weapon_share, 1.0);
    }
    #[test]
    fn invalid_hero_is_rejected() {
        for hero in ["", "  ", "Warden\n"] {
            let mut r = request();
            r.hero = hero.into();
            assert!(r.config().is_err());
        }
    }
}
