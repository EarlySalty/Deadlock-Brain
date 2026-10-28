use super::*;

fn fact(stat: &str, value: &str, citation: &str) -> Value {
    json!({"stat_key":stat,"value":value,"condition":{},"citation":citation,"source_revision":"rev","patch":null})
}

fn request(mechanic: MechanicKind, minute: Option<u16>) -> GetMechanicRequest {
    GetMechanicRequest {
        mechanic,
        minute,
        patch: None,
    }
}

fn value_of<'a>(facts: &'a [Value], stat: &str) -> Option<&'a str> {
    facts
        .iter()
        .find(|row| row["stat_key"] == stat)
        .and_then(|row| row["value"].as_str())
}

#[test]
fn trooper_wave_souls_scale_with_minute_and_cite_inputs() {
    let rows = vec![
        fact("citadel_trooper_gold_reward", "100", "1"),
        fact("citadel_trooper_gold_reward_bonus_per_minute", "2", "2"),
        fact("citadel_trooper_squad_size", "4", "3"),
        fact("citadel_trooper_instant_gold_ratio_laning", "0.5", "4"),
        fact("citadel_trooper_instant_gold_ratio_postlaning", "0.25", "5"),
        fact("citadel_trooper_laning_gold_rules_end_time", "10", "6"),
        fact("citadel_trooper_spawn_interval_early", "30", "7"),
        fact("citadel_trooper_spawn_interval_late", "25", "8"),
        fact("citadel_trooper_spawn_interval_late_time", "15", "9"),
    ];
    let output = present(&request(MechanicKind::LaneTroopers, Some(16)), rows);
    let facts = output["facts"].as_array().cloned().unwrap_or_default();
    assert_eq!(
        value_of(&facts, "derived_trooper_souls_at_minute"),
        Some("132")
    );
    assert_eq!(
        value_of(&facts, "derived_squad_souls_at_minute"),
        Some("528")
    );
    assert_eq!(
        value_of(&facts, "derived_trooper_souls_instant_at_minute"),
        Some("33")
    );
    assert_eq!(
        value_of(&facts, "derived_wave_interval_seconds_at_minute"),
        Some("25")
    );
    assert_eq!(facts[0]["citation"], "1");
    assert_eq!(facts[0]["condition"]["minute"], 16);
    assert_eq!(
        value_of(&facts, "derived_wave_souls_instant_at_minute"),
        Some("132")
    );
    assert_eq!(
        value_of(&facts, "derived_wave_souls_orb_at_minute"),
        Some("396")
    );
    assert_eq!(facts.len(), 16);
    let unused = [
        "citadel_trooper_instant_gold_ratio_laning",
        "citadel_trooper_spawn_interval_early",
    ];
    assert!(facts[..14]
        .iter()
        .all(|row| !unused.contains(&row["stat_key"].as_str().unwrap_or(""))));
}

#[test]
fn respawn_interpolates_between_ramp_points_and_clamps() {
    let rows = vec![
        fact("citadel_player_spawn_time_min_ramp_1", "8", "1"),
        fact("citadel_player_spawn_time_min_ramp_1_time", "300", "2"),
        fact("citadel_player_spawn_time_min_ramp_2", "38", "3"),
        fact("citadel_player_spawn_time_min_ramp_2_time", "1200", "4"),
    ];
    let at = |minute| {
        let output = present(&request(MechanicKind::Respawn, Some(minute)), rows.clone());
        value_of(
            output["facts"].as_array().map_or(&[][..], Vec::as_slice),
            "derived_respawn_min_seconds_at_minute",
        )
        .map(str::to_owned)
    };
    assert_eq!(at(0).as_deref(), Some("8"));
    assert_eq!(at(10).as_deref(), Some("18"));
    assert_eq!(at(20).as_deref(), Some("38"));
    assert_eq!(at(60).as_deref(), Some("38"));
}

#[test]
fn shares_levels_and_camps_are_derived_without_hiding_raw_rows() {
    let shares = present(
        &request(MechanicKind::SoulSharing, None),
        vec![
            fact("trooperkillgoldsharefrac_0", "1", "1"),
            fact("trooperkillgoldsharefrac_1", "0.54", "2"),
        ],
    );
    let facts = shares["facts"].as_array().cloned().unwrap_or_default();
    assert_eq!(
        value_of(&facts, "derived_trooper_share_percent_2_players"),
        Some("54")
    );
    let camps = present(
        &request(MechanicKind::Denizens, Some(10)),
        vec![
            fact("neutral_trooper_weak_goldreward", "41", "1"),
            fact(
                "neutral_trooper_weak_goldrewardbonuspercentperminute",
                "1.08",
                "2",
            ),
        ],
    );
    let facts = camps["facts"].as_array().cloned().unwrap_or_default();
    assert_eq!(
        value_of(&facts, "derived_neutral_trooper_weak_souls_at_minute"),
        Some("45.43")
    );
    let levels = present(
        &request(MechanicKind::Levels, None),
        vec![
            fact("level_0_requiredsouls", "0", "1"),
            fact("level_1_requiredsouls", "400", "2"),
            fact("level_1_abilitypoints", "true", "3"),
        ],
    );
    let facts = levels["facts"].as_array().cloned().unwrap_or_default();
    assert_eq!(
        value_of(&facts, "derived_level_2_required_souls"),
        Some("400")
    );
    assert_eq!(facts[1]["condition"]["ability_points"], "true");
    assert_eq!(facts.len(), 5);
}

#[test]
fn raw_rows_are_capped_and_lower_bound_is_reported() {
    let rows: Vec<Value> = (0..QUERY_LIMIT)
        .map(|index| fact(&format!("row_{index}"), "1", "1"))
        .collect();
    let output = present(&request(MechanicKind::StreetBrawl, None), rows);
    assert_eq!(output["facts"].as_array().map(Vec::len), Some(MAX_ROWS));
    assert_eq!(output["truncated"], true);
    assert_eq!(output["omitted_count_is_lower_bound"], true);
    let definition = definition();
    assert_eq!(definition.name, ToolName::GetMechanic);
    assert_eq!(
        definition.input_schema["properties"]["mechanic"]["enum"]
            .as_array()
            .map(Vec::len),
        Some(MechanicKind::ALL.len())
    );
}

#[test]
fn missing_minute_derives_reference_minutes_and_explains_concepts() {
    let kills = present(
        &request(MechanicKind::HeroKills, None),
        vec![
            fact("citadel_player_gold_reward_min", "200", "1"),
            fact("citadel_player_gold_reward_max", "2200", "2"),
            fact("citadel_player_gold_reward_time", "2400", "3"),
        ],
    );
    let facts = kills["facts"].as_array().cloned().unwrap_or_default();
    let bounties: Vec<(u64, &str)> = facts
        .iter()
        .filter(|row| row["stat_key"] == "derived_hero_kill_bounty_at_minute")
        .filter_map(|row| Some((row["condition"]["minute"].as_u64()?, row["value"].as_str()?)))
        .collect();
    assert_eq!(
        bounties,
        vec![(0, "200"), (10, "700"), (20, "1200"), (30, "1700")]
    );
    assert_eq!(kills["reference_minutes"], json!([0, 10, 20, 30]));
    let urn = present(
        &request(MechanicKind::Urn, Some(10)),
        vec![
            fact("citadel_crate_spawn_initial_delay", "600", "1"),
            fact("citadel_crate_respawn_interval", "300", "2"),
            fact("citadel_idol_orbs_to_spawn", "10", "3"),
        ],
    );
    let facts = urn["facts"].as_array().cloned().unwrap_or_default();
    assert_eq!(
        value_of(&facts, "derived_urn_first_spawn_minute"),
        Some("10")
    );
    assert_eq!(
        value_of(&facts, "derived_urn_respawn_interval_minutes"),
        Some("5")
    );
    assert!(facts
        .iter()
        .all(|row| row["condition"]["meaning"].is_string()));
    assert!(urn["reference_minutes"].is_null());
}
