use super::*;

fn fact(stat: &str, value: &str, citation: &str) -> Value {
    json!({"stat_key":stat,"value":value,"condition":{},"citation":citation,"source_revision":"rev","patch":null})
}

fn request(mechanic: MechanicKind, minute: Option<u16>) -> GetMechanicRequest {
    GetMechanicRequest {
        mechanic,
        minute,
        patch: None,
        objective: None,
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

#[test]
fn live_lane_trooper_rows_fit_one_tool_result_with_concept_rows() {
    let rows: Vec<Value> = serde_json::from_str(include_str!(
        "../tests/fixtures/mechanic_lane_troopers.json"
    ))
    .unwrap_or_default();
    assert_eq!(rows.len(), 29);
    let evidence = rows.len() * 96 + 1100;
    for minute in [None, Some(16)] {
        let mut result = present(&request(MechanicKind::LaneTroopers, minute), rows.clone());
        let facts = result["facts"].as_array().cloned().unwrap_or_default();
        let needed = facts
            .iter()
            .rposition(|row| {
                row["stat_key"]
                    .as_str()
                    .is_some_and(|stat| stat.starts_with("derived_"))
                    || row["condition"]["meaning"].is_string()
            })
            .map_or(0, |last| last + 1);
        assert!(facts[..needed]
            .iter()
            .any(|row| row["stat_key"] == "derived_wave_souls_orb_at_minute"));
        assert!(facts[..needed]
            .iter()
            .any(|row| row["stat_key"] == "citadel_deny_denied_percentage"));
        result["facts"] = Value::Array(facts[..needed].to_vec());
        let bytes = serde_json::to_vec(&result).map_or(usize::MAX, |v| v.len());
        assert!(bytes + evidence < 16_384, "{bytes}");
    }
}

#[test]
fn objective_reward_rows_precede_boss_stat_rows() {
    let mut rows = vec![
        fact("npc_boss_tier3_maxhealth", "12000", "c"),
        fact("npc_boss_tier3_phase2health", "12000", "c"),
    ];
    rows.extend([
        fact(
            "trooper_normal_shrinesdownbuffmodifier_damagescale",
            "60.0",
            "c",
        ),
        fact("objectiveparams_patronphase1goldkill", "0", "c"),
        fact("objectiveparams_shrinesgoldkill", "2000", "c"),
    ]);
    let output = present(&request(MechanicKind::Objectives, None), rows);
    let stats: Vec<&str> = output["facts"]
        .as_array()
        .map(|facts| {
            facts
                .iter()
                .filter_map(|row| row["stat_key"].as_str())
                .collect()
        })
        .unwrap_or_default();
    let first_boss = stats.iter().position(|stat| stat.starts_with("npc_boss_"));
    let last_reward = stats
        .iter()
        .rposition(|stat| stat.starts_with("objectiveparams_"));
    assert!(last_reward < first_boss, "{stats:?}");
    assert_eq!(stats.first(), Some(&"objectiveparams_shrinesgoldkill"));
}

#[test]
fn objective_resists_carry_signed_meanings_and_unverified_rows_stay_out() {
    let rows = vec![
        fact(
            "npc_boss_tier2_intrinsicmodifiers_0_bullet_armor_damage_resist",
            "35",
            "c",
        ),
        fact("npc_boss_tier2_nearbyenemyresist_resistvalues_3", "20", "c"),
        fact(
            "npc_boss_tier2_friendlyauramodifier_modifierprovidedbyaura_tech_resist",
            "15",
            "c",
        ),
        fact(
            "npc_boss_tier2_empoweredmodifierlevel1_maxhealth",
            "9000",
            "c",
        ),
        fact("npc_barrack_boss_playerdamageresistpct", "-25.0", "c"),
        fact("npc_boss_tier1_playerdps", "116.0", "c"),
        fact(
            "trooper_normal_shrinesdownbuffmodifier_healthscale",
            "1.4",
            "c",
        ),
    ];
    let output = present(&request(MechanicKind::Objectives, None), rows);
    let facts = output["facts"].as_array().cloned().unwrap_or_default();
    let meaning = |stat: &str| {
        facts
            .iter()
            .find(|row| row["stat_key"] == stat)
            .and_then(|row| row["condition"]["meaning"].as_str())
            .map(str::to_owned)
    };
    assert!(
        meaning("npc_boss_tier2_intrinsicmodifiers_0_bullet_armor_damage_resist").is_some_and(
            |text| text.starts_with("Walker") && text.contains("weniger Schaden durch Kugeln")
        )
    );
    assert!(meaning("npc_boss_tier2_nearbyenemyresist_resistvalues_3")
        .is_some_and(|text| text.contains("Zahl der Gegner")));
    assert!(
        meaning("npc_boss_tier2_friendlyauramodifier_modifierprovidedbyaura_tech_resist")
            .is_some_and(|text| text.contains("verbündete Helden"))
    );
    assert!(meaning("npc_boss_tier2_empoweredmodifierlevel1_maxhealth").is_some());
    assert_eq!(facts.len(), 4, "{facts:?}");
}

#[test]
fn parry_rows_explain_stun_and_damage_bonus() {
    let rows = vec![
        fact("citadel_ability_melee_parry_parriedstuntime", "2.75", "c"),
        fact(
            "citadel_ability_melee_parry_victimdamagetakenscale",
            "25",
            "c",
        ),
        fact(
            "citadel_ability_melee_parry_abilitycooldown_value",
            "4.5",
            "c",
        ),
        fact("citadel_player_dash_down_stamina_cost", "0.5", "c"),
        fact("citadel_unexplained_value", "1", "c"),
    ];
    let output = present(&request(MechanicKind::CoreAbilities, None), rows);
    let facts = output["facts"].as_array().cloned().unwrap_or_default();
    assert_eq!(facts.len(), 4);
    assert!(facts
        .iter()
        .all(|row| row["condition"]["meaning"].is_string()));
    let down = facts
        .iter()
        .find(|row| row["stat_key"] == "citadel_player_dash_down_stamina_cost")
        .and_then(|row| row["condition"]["meaning"].as_str());
    assert!(down.is_some_and(|text| text.starts_with("Dash nach unten")));
}

#[test]
fn every_live_objective_fits_one_tool_result_with_all_its_rows() {
    let rows: Vec<Value> =
        serde_json::from_str(include_str!("../tests/fixtures/mechanic_objectives.json"))
            .unwrap_or_default();
    assert_eq!(rows.len(), 56);
    for objective in ObjectiveKind::ALL {
        let mut request = request(MechanicKind::Objectives, None);
        request.objective = Some(objective);
        let result = present(&request, rows.clone());
        let facts = result["facts"].as_array().cloned().unwrap_or_default();
        assert!(!facts.is_empty());
        assert!(
            facts
                .iter()
                .all(|row| belongs_to(row, Some(objective))
                    && row["condition"]["meaning"].is_string())
        );
        let evidence = facts.len() * 96 + 1100;
        let bytes = serde_json::to_vec(&result).map_or(usize::MAX, |v| v.len());
        assert!(bytes + evidence < 16_384, "{objective:?} {bytes}");
    }
    let mut walker = request(MechanicKind::Objectives, None);
    walker.objective = Some(ObjectiveKind::Walker);
    let facts = present(&walker, rows)["facts"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    for (stat, value) in [
        (
            "npc_boss_tier2_intrinsicmodifiers_0_bullet_armor_damage_resist",
            "35",
        ),
        ("npc_boss_tier2_nearbyenemyresist_resistvalues_6", "50"),
        (
            "npc_boss_tier2_friendlyauramodifier_modifierprovidedbyaura_tech_resist",
            "15",
        ),
        ("objectiveparams_tier2goldkill", "3675"),
    ] {
        assert_eq!(value_of(&facts, stat), Some(value), "{stat}");
    }
    assert!(value_of(&facts, "npc_boss_tier1_maxhealth").is_none());
}

fn orb_line(patch: &str, released_at: &str, text: &str, citation: &str) -> Value {
    json!({"patch":patch,"released_at":released_at,"line_text":text,"citation":citation,"source_revision":"rev"})
}

#[test]
fn orb_share_history_lists_dated_kill_orb_changes_after_derived_rows() {
    let rows: Vec<Value> = serde_json::from_str(include_str!(
        "../tests/fixtures/mechanic_lane_troopers.json"
    ))
    .unwrap_or_default();
    let history = [
        orb_line(
            "11-21-2024 Update",
            "2024-11-21T23:21:49+00:00",
            "Trooper soul changed from 60/40% in the kill/orb to 65/35%",
            "11",
        ),
        orb_line(
            "11-21-2025 Update",
            "2025-11-21T22:53:12+00:00",
            "Amount of souls in the flying orb that can be secured/denied reduced from 50% to 40%",
            "12",
        ),
        orb_line(
            " Minor Update - 06-30-2026",
            "2026-06-30T17:22:14+00:00",
            "Trooper bounty ratio in the deniable flying orb increased from 40% to 50%",
            "13",
        ),
        orb_line(
            "x",
            "2026-01-01T00:00:00+00:00",
            "Flying orb now glows brighter",
            "14",
        ),
        json!({"patch":"y","released_at":"2026-01-02T00:00:00+00:00","line_text":"Amount in the flying orb reduced from 50% to 40%","citation":"15","source_revision":""}),
        json!({"patch":"z","released_at":"","line_text":"Amount in the flying orb reduced from 50% to 40%","citation":"16","source_revision":"rev"}),
        json!({"patch":"z","line_text":"Amount in the flying orb reduced from 50% to 40%","citation":"17","source_revision":"rev"}),
    ];
    let mut result = present(&request(MechanicKind::LaneTroopers, None), rows);
    insert_orb_history(&mut result, &history);
    let facts = result["facts"].as_array().cloned().unwrap_or_default();
    let changes: Vec<(usize, &Value)> = facts
        .iter()
        .enumerate()
        .filter(|(_, row)| row["stat_key"] == "patch_orb_share_change")
        .collect();
    let shares: Vec<(&str, &str, &str)> = changes
        .iter()
        .map(|(_, row)| {
            (
                row["released_at"].as_str().unwrap_or_default(),
                row["condition"]["old_orb_share_percent"]
                    .as_str()
                    .unwrap_or_default(),
                row["condition"]["new_orb_share_percent"]
                    .as_str()
                    .unwrap_or_default(),
            )
        })
        .collect();
    assert_eq!(
        shares,
        [
            ("2024-11-21T23:21:49+00:00", "40", "35"),
            ("2025-11-21T22:53:12+00:00", "50", "40"),
            ("2026-06-30T17:22:14+00:00", "40", "50"),
        ]
    );
    let first = changes.first().map_or(usize::MAX, |(index, _)| *index);
    assert!(facts[..first].iter().all(|row| row["stat_key"]
        .as_str()
        .is_some_and(|stat| stat.starts_with("derived_"))));
    assert!(changes.iter().all(|(_, row)| row["patch"].is_null()
        && row["citation"].is_string()
        && row["condition"]["meaning"].is_string()));
    let needed = facts
        .iter()
        .rposition(|row| {
            row["stat_key"]
                .as_str()
                .is_some_and(|stat| stat.starts_with("derived_"))
                || row["condition"]["meaning"].is_string()
        })
        .map_or(0, |last| last + 1);
    result["facts"] = Value::Array(facts[..needed].to_vec());
    let evidence = (29 + changes.len()) * 96 + 1100;
    let bytes = serde_json::to_vec(&result).map_or(usize::MAX, |v| v.len());
    assert!(bytes + evidence < 16_384, "{bytes}");
}

#[test]
fn orb_history_failure_keeps_the_core_transaction_usable() {
    let pg = crate::scratch_pg::ScratchPg::start();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("runtime");
    runtime.block_on(async {
        let options = sqlx::postgres::PgConnectOptions::new_without_pgpass()
            .host(&pg.directory.join("socket").to_string_lossy())
            .port(55439)
            .username("brain_core_test")
            .database("postgres")
            .password("");
        let admin = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect_with(options.clone())
            .await
            .expect("scratch DB");
        sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service LOGIN; CREATE ROLE brain_readonly LOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate")
            .execute(&admin)
            .await
            .expect("scratch roles");
        brain_storage::PgStore::new(admin.clone())
            .migrate_gamefacts()
            .await
            .expect("migrations");
        let pool = super::super::tool_pool(
            options.username("brain_readonly"),
            std::time::Duration::from_secs(2),
            1,
        );
        let mut blocker = admin.begin().await.expect("blocking transaction");
        sqlx::query("LOCK TABLE canon.fact IN ACCESS EXCLUSIVE MODE")
            .execute(&mut *blocker)
            .await
            .expect("block history source");
        let mut connection = pool.begin().await.expect("tool transaction");
        assert_eq!(orb_history(&mut connection).await, None);
        let scope = history::scope(&request(MechanicKind::Respawn, None)).expect("respawn scope");
        assert_eq!(history::lines(&mut connection, scope).await, None);
        blocker.rollback().await.expect("release history source");
        assert_eq!(orb_history(&mut connection).await, Some(Vec::new()));
        assert_eq!(history::lines(&mut connection, scope).await, Some(Vec::new()));
        let alive: i32 = sqlx::query_scalar("SELECT 1")
            .fetch_one(&mut *connection)
            .await
            .expect("core transaction survives history failure");
        assert_eq!(alive, 1);
        connection.commit().await.expect("core commit");
    });
}

#[test]
fn patch_note_changes_expose_old_and_new_values() {
    let cases = [
        (
            "Respawn time at 30 minutes reduced from 75s to 70s",
            Some(("75s", "70s")),
        ),
        (
            "Mid Walkers HP increased from 8,000 to 9,000.",
            Some(("8,000", "9,000")),
        ),
        (
            "Mystic Burst: Cooldown increased from 12s to 14s",
            Some(("12s", "14s")),
        ),
        (
            "Base kill bounty reduced from 250 to 200 (max is still 2200 at 40m)",
            Some(("250", "200")),
        ),
        ("Urn no longer requires a drop off channel time", None),
        (
            "Flex Slot changed from \"Two Guardians\" to \"Three Guardians\"",
            None,
        ),
    ];
    for (text, expected) in cases {
        let parsed = history::from_to(text);
        assert_eq!(
            parsed
                .as_ref()
                .map(|(old, new)| (old.as_str(), new.as_str())),
            expected,
            "{text}"
        );
    }
}

#[test]
fn patch_note_changes_follow_explained_rows_and_precede_raw_rows() {
    let rows = vec![
        fact("citadel_respawn_time_min", "8", "1"),
        fact("citadel_unexplained_raw", "3", "2"),
    ];
    let mut result = present(&request(MechanicKind::Respawn, Some(20)), rows);
    let lines = [
        json!({"patch":"Minor Update - 09-16-2026","released_at":"2026-09-16T20:00:00+00:00","line_text":"Respawn Time at 20 minutes increased from 35s to 38s","citation":"7","source_revision":"rev"}),
        json!({"patch":"x","released_at":"","line_text":"Respawn time at 30 minutes reduced from 75s to 70s","citation":"8","source_revision":"rev"}),
    ];
    history::insert(&mut result, &lines);
    let facts = result["facts"].as_array().cloned().unwrap_or_default();
    let changes: Vec<usize> = facts
        .iter()
        .enumerate()
        .filter(|(_, row)| row["stat_key"] == "patch_note_change")
        .map(|(index, _)| index)
        .collect();
    assert_eq!(changes.len(), 1);
    let change = &facts[changes[0]];
    assert_eq!(change["condition"]["old_text"], "35s");
    assert_eq!(change["condition"]["new_text"], "38s");
    assert_eq!(change["released_at"], "2026-09-16T20:00:00+00:00");
    assert!(facts[changes[0] + 1..]
        .iter()
        .all(|row| row["condition"]["meaning"].is_null()
            && !row["stat_key"]
                .as_str()
                .is_some_and(|stat| stat.starts_with("derived_"))));
    assert_eq!(result["patch_history_meaning"], history::MEANING);
}

#[test]
fn every_mechanic_except_item_tiers_has_a_patch_scope() {
    for mechanic in MechanicKind::ALL {
        let scope = history::scope(&request(mechanic, None));
        assert_eq!(
            scope.is_none(),
            mechanic == MechanicKind::ItemTiers,
            "{mechanic:?}"
        );
    }
}
