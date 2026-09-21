//! Independent input/holdout audit. Reads an existing freeze without DB access,
//! AI calls, catalog filtering, score changes, or any publication side effects.
use dbrain_reasoner::{families, ItemModel, PopulationPrior};
use families::{BuildObservation, ObservationSource};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
mod support;

type Error = Box<dyn std::error::Error>;

fn participant_overlap(rows: &[BuildObservation]) -> usize {
    let training = rows
        .iter()
        .filter(|row| !families::is_holdout(row))
        .map(|row| row.participant.as_str())
        .collect::<BTreeSet<_>>();
    let validation = rows
        .iter()
        .filter(|row| families::is_holdout(row))
        .map(|row| row.participant.as_str())
        .collect::<BTreeSet<_>>();
    training.intersection(&validation).count()
}

fn cross_source_probe() -> Value {
    let mut mismatches = 0;
    let samples = 1000;
    for participant in 0..samples {
        let player = BuildObservation {
            key: format!("match:{participant}"),
            participant: participant.to_string(),
            source: ObservationSource::Player,
            items: vec![1, 2, 3],
            imbues: BTreeMap::new(),
            skill_order: Vec::new(),
            buy_times_s: BTreeMap::new(),
            sold_times_s: BTreeMap::new(),
            observed_at: None,
            won: None,
            warnings: Vec::new(),
        };
        let mut author = player.clone();
        author.source = ObservationSource::Author;
        author.key = format!("author:{participant}");
        mismatches += usize::from(families::is_holdout(&player) != families::is_holdout(&author));
    }
    json!({"participants_tested": samples, "cross_source_partition_mismatches": mismatches,
        "passed": mismatches == 0,
        "requirement": "The same account must stay on one side even when it is both a match participant and a build author."})
}

fn legacy_publish_probe() -> Value {
    // Call only the pure validator. Never call the queue or open a database.
    let build = dbrain_reasoner::BuildObject {
        hero_id: 777,
        hero_name: "Synthetic validation probe".into(),
        patch_tag: "unverified".into(),
        name: "Unverified empty legacy input".into(),
        core: Vec::new(),
        situations: Vec::new(),
        ability_order: Vec::new(),
        confidence: dbrain_reasoner::Confidence::Low,
        rationale: String::new(),
        family: None,
        variants: Vec::new(),
        family_discovery: None,
    };
    let result = dbrain_reasoner::publish::validate_publish_input(&build);
    json!({"passed": result.is_err(), "validator_accepted_empty_unverified_build": result.is_ok(),
        "validation_error": result.err().map(|error| error.to_string()),
        "database_or_queue_called": false,
        "requirement": "Missing family metadata must not permit an empty, Low-confidence, unverified build to bypass publication validation."})
}

fn catalog_gaps(rows: &[BuildObservation], items: &[ItemModel]) -> Value {
    let models = items
        .iter()
        .map(|item| (item.item_id, item))
        .collect::<BTreeMap<_, _>>();
    let prior = families::observed_population(rows);
    let mut counts = BTreeMap::<i64, usize>::new();
    let mut keys = BTreeSet::new();
    let mut duplicate_rows = 0;
    for row in rows
        .iter()
        .filter(|row| row.source == ObservationSource::Player)
    {
        if !keys.insert(row.key.as_str()) {
            duplicate_rows += 1;
            continue;
        }
        for id in row.items.iter().copied().collect::<BTreeSet<_>>() {
            *counts.entry(id).or_default() += 1;
        }
    }
    let describe = |id: &i64, count: &usize| {
        json!({
            "item_id": id, "observed_player_rows": count,
            "prevalence": prior.prevalence(*id), "staple": prior.is_staple(*id),
            "name": models.get(id).map(|item| item.name.as_str()),
        })
    };
    let absent = counts
        .iter()
        .filter(|(id, _)| !models.contains_key(id))
        .map(|(id, count)| describe(id, count))
        .collect::<Vec<_>>();
    let unavailable = counts
        .iter()
        .filter(|(id, _)| {
            models
                .get(id)
                .is_some_and(|item| !item.shopable || item.disabled)
        })
        .map(|(id, count)| describe(id, count))
        .collect::<Vec<_>>();
    json!({"catalog_models": items.len(), "distinct_player_rows": keys.len(),
        "duplicate_rows": duplicate_rows, "absent_from_catalog": absent,
        "present_but_not_shopable": unavailable,
        "interpretation": "Missing catalog IDs are retained as data-coverage failures. This audit does not remove missing staples or claim that the IDs are obsolete. Raw row counts require conflict review when duplicate_rows is nonzero."})
}

fn skill_order_review(
    input: &Value,
    config: &dbrain_reasoner::ReasonerConfig,
) -> Result<Value, Error> {
    let mut hero: dbrain_reasoner::HeroModel = serde_json::from_value(input["hero"].clone())?;
    let mut items: Vec<ItemModel> = serde_json::from_value(input["items"].clone())?;
    let meta: dbrain_reasoner::meta::MetaIndexWithSources =
        serde_json::from_value(input["meta"].clone())?;
    let snapshots: Vec<dbrain_reasoner::PatchSnapshot> =
        serde_json::from_value(input["snapshots"].clone())?;
    let events: Vec<Value> = serde_json::from_value(input["events"].clone())?;
    let mut deltas =
        dbrain_reasoner::patch::compute_patch_delta_with_snapshots(&hero, &events, &snapshots);
    dbrain_reasoner::patch::apply_scored_patch_delta(
        &mut hero,
        &mut items,
        &mut deltas,
        &meta.index,
        config,
    );
    let discovery = families::detect_families(
        &meta.observations,
        &items,
        &meta.population,
        &families::FamilyPolicy::for_patch(&events, config),
    );
    let mut review = Vec::new();
    for family in discovery
        .families
        .iter()
        .filter(|family| family.eligible_for_planning)
    {
        let context = families::conditioned_meta(&meta, family, config);
        let (selected, raw_source) = context.ability_order(hero.hero_id);
        let (accepted, source, notes) = context.coherent_ability_order(&hero);
        let author_prefixes = context
            .author_builds
            .iter()
            .filter_map(|author| {
                let order = families::author_observation(author).skill_order;
                let (valid, _) = dbrain_reasoner::progression::coherent_order(&hero, &order);
                (!valid.is_empty()).then_some(valid.len())
            })
            .collect::<Vec<_>>();
        let global_prefix = meta.hero_ability_orders.get(&hero.hero_id).map(|order| {
            dbrain_reasoner::progression::coherent_order(&hero, order)
                .0
                .len()
        });
        let observed_prefix =
            dbrain_reasoner::progression::coherent_order(&hero, &family.skill_order)
                .0
                .len();
        let ignored_fallback = accepted.is_empty()
            && (!author_prefixes.is_empty() || global_prefix.is_some_and(|length| length > 0));
        println!("  skill {}: selected={} accepted={} alternative-authors={} global-prefix={:?} ignored-fallback={}",
            family.id, selected.len(), accepted.len(), author_prefixes.len(), global_prefix, ignored_fallback);
        review.push(json!({"family_id": family.id, "label": family.label,
            "source": source, "first_raw_source": raw_source,
            "selected_steps": selected.len(), "accepted_steps": accepted.len(),
            "validation_notes": notes, "usable_author_prefixes": author_prefixes,
            "global_prefix_steps": global_prefix, "observed_family_prefix_steps": observed_prefix,
            "family_skill_order_support": family.skill_order_support,
            "empty_despite_usable_fallback": ignored_fallback}));
    }
    Ok(Value::Array(review))
}

fn audit(input: &Path, output: &Path) -> Result<(), Error> {
    if output.exists() {
        return Err("output already exists".into());
    }
    let bytes = fs::read(input)?;
    let frozen: Value = serde_json::from_slice(&bytes)?;
    if frozen["format_version"] != 1 {
        return Err("unsupported frozen input version".into());
    }
    let heroes = frozen["heroes"].as_array().ok_or("heroes array missing")?;
    let config: dbrain_reasoner::ReasonerConfig = serde_json::from_value(frozen["config"].clone())?;
    let mut reports = Vec::new();
    for input in heroes {
        let items: Vec<ItemModel> = serde_json::from_value(input["items"].clone())?;
        let rows: Vec<BuildObservation> =
            serde_json::from_value(input["meta"]["observations"].clone())?;
        let global: PopulationPrior = serde_json::from_value(input["meta"]["population"].clone())?;
        let training = rows
            .iter()
            .filter(|row| !families::is_holdout(row))
            .cloned()
            .collect::<Vec<_>>();
        let holdout = rows
            .iter()
            .filter(|row| families::is_holdout(row))
            .cloned()
            .collect::<Vec<_>>();
        let gaps = catalog_gaps(&rows, &items);
        let absent_staples = gaps["absent_from_catalog"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|item| item["staple"] == true)
            .map(|item| item["item_id"].clone())
            .collect::<Vec<_>>();
        let absent_global_staples = gaps["absent_from_catalog"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|item| item["item_id"].as_i64())
            .filter(|id| global.is_staple(*id))
            .collect::<Vec<_>>();
        let overlap = participant_overlap(&rows);
        println!("{}: catalog={} absent={} missing-player-staples={:?} missing-global-staples={:?} participant-leaks={}",
            input["hero"]["name"], items.len(), gaps["absent_from_catalog"].as_array().map_or(0, Vec::len),
            absent_staples, absent_global_staples, overlap);
        reports.push(
            json!({"hero_id": input["hero"]["hero_id"], "hero_name": input["hero"]["name"],
            "all_observations": gaps, "training": catalog_gaps(&training, &items),
            "holdout": catalog_gaps(&holdout, &items), "participant_overlap": overlap,
            "missing_raw_population_staples": absent_staples,
            "missing_aggregated_global_staples": absent_global_staples,
            "skill_order_review": skill_order_review(input, &config)?}),
        );
    }
    let probe = cross_source_probe();
    println!("cross-source holdout probe: {probe}");
    let publish_probe = legacy_publish_probe();
    println!("legacy publication probe: {publish_probe}");
    let report = json!({"format_version": 1, "input_sha256": format!("{:x}", Sha256::digest(&bytes)),
        "patch_tag": frozen["config"]["patch_tag"], "snapshot_guard": frozen["snapshot_guard"],
        "cross_source_partition_probe": probe, "legacy_publication_probe": publish_probe,
        "heroes": reports,
        "scope": "Frozen-input correctness review, not current-patch verification or expert release approval."});
    support::write_new(output, &serde_json::to_vec_pretty(&report)?)?;
    Ok(())
}

fn summarize(path: &Path) -> Result<(), Error> {
    let report: Value = serde_json::from_slice(&fs::read(path)?)?;
    if report["format_version"] != 1 {
        return Err("unsupported review report version".into());
    }
    println!("input_sha256={}", report["input_sha256"]);
    println!(
        "cross_source_partition_probe={}",
        report["cross_source_partition_probe"]
    );
    println!(
        "legacy_publication_probe={}",
        report["legacy_publication_probe"]
    );
    for hero in report["heroes"]
        .as_array()
        .ok_or("heroes missing from review")?
    {
        println!(
            "{}: participant-overlap={} catalog-gaps={}",
            hero["hero_name"],
            hero["participant_overlap"],
            hero["all_observations"]["absent_from_catalog"]
        );
        for skill in hero["skill_order_review"].as_array().into_iter().flatten() {
            if skill["empty_despite_usable_fallback"] == true {
                println!("  BLOCKED: {}", skill);
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [mode, path] if mode == "summary" => summarize(Path::new(path)),
        [input, output] => audit(Path::new(input), Path::new(output)),
        _ => Err("usage: family_input_review FROZEN_INPUT NEW_REPORT | summary REPORT".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_unverified_legacy_build_is_rejected_without_queue_access() {
        let report = legacy_publish_probe();
        assert_eq!(report["passed"], true);
        assert_eq!(report["validator_accepted_empty_unverified_build"], false);
        assert_eq!(report["database_or_queue_called"], false);
    }

    #[test]
    fn the_same_account_is_not_split_by_source_role() {
        let report = cross_source_probe();
        assert_eq!(report["participants_tested"], 1000);
        assert_eq!(report["cross_source_partition_mismatches"], 0);
        assert_eq!(report["passed"], true);
    }

    #[test]
    fn unknown_items_are_visible_and_repeated_purchases_are_not_extra_matches() {
        let row = BuildObservation {
            key: "match:1".into(),
            participant: "account:1".into(),
            source: ObservationSource::Player,
            items: vec![12, 12, 13],
            imbues: BTreeMap::new(),
            skill_order: Vec::new(),
            buy_times_s: BTreeMap::new(),
            sold_times_s: BTreeMap::new(),
            observed_at: None,
            won: None,
            warnings: Vec::new(),
        };
        let report = catalog_gaps(&[row], &[]);
        assert_eq!(report["distinct_player_rows"], 1);
        assert_eq!(report["absent_from_catalog"].as_array().unwrap().len(), 2);
        assert_eq!(report["absent_from_catalog"][0]["observed_player_rows"], 1);
        assert_eq!(report["absent_from_catalog"][0]["prevalence"], 1.0);
        assert_eq!(report["absent_from_catalog"][0]["staple"], true);
        assert_eq!(report["absent_from_catalog"][0]["name"], Value::Null);
    }

    #[test]
    fn authors_do_not_count_as_player_catalog_evidence() {
        let row = BuildObservation {
            key: "author:1".into(),
            participant: "account:1".into(),
            source: ObservationSource::Author,
            items: vec![12, 13],
            imbues: BTreeMap::new(),
            skill_order: Vec::new(),
            buy_times_s: BTreeMap::new(),
            sold_times_s: BTreeMap::new(),
            observed_at: None,
            won: None,
            warnings: Vec::new(),
        };
        let report = catalog_gaps(&[row], &[]);
        assert_eq!(report["distinct_player_rows"], 0);
        assert_eq!(report["absent_from_catalog"], json!([]));
    }
}
