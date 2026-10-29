use crate::{
    limits, Alias, Entity, Extractor, Fact, FactSet, FactValue, Outcome, RawObject, Receipt,
    Rejection,
};
use brain_storage::entity_profile::derivation::game_file_facts::json_text;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub const SOURCE: &str = "deadlock-data";
pub const PATHS: &[&str] = &[CONVARS, NPC_DATA, GENERIC_DATA, SOUL_UNLOCK_DATA];
const CONVARS: &str = "data/json/convars.json";
const NPC_DATA: &str = "data/json/npc-data.json";
const GENERIC_DATA: &str = "data/json/generic-data.json";
const SOUL_UNLOCK_DATA: &str = "data/json/soul-unlock-data.json";

struct Selector {
    mechanic: &'static str,
    path: &'static str,
    roots: &'static [&'static str],
    fields: Option<&'static [&'static str]>,
    stat_prefix: &'static str,
}

const NEUTRAL_FIELDS: &[&str] = &[
    "GoldReward",
    "GoldRewardBonusPercentPerMinute",
    "InitialSpawnDelay",
    "SpawnInterval",
    "MaxHealth",
];
const OBJECTIVE_FIELDS: &[&str] = &[
    "MaxHealth",
    "Phase2Health",
    "PlayerDPS",
    "TrooperDPS",
    "PlayerDamageResistPct",
    "TrooperDamageResistPct",
];
const STREET_BRAWL_FIELDS: &[&str] = &[
    "m_vecRespawnTimes",
    "m_vecGoldPerRound",
    "m_vecAPPerRound",
    "m_vecRoundLengthMinutes",
    "m_vecBuyTime",
    "m_iScoreToWin",
    "m_iLaneNumber",
    "m_vecObjectiveMaxHealth",
    "TrooperSpawnTimer",
    "OvertimeRespawnTimeIncrease",
    "OvertimeTrooperHealthScale",
    "m_iUltimateUnlockRound",
];

const SELECTORS: &[Selector] = &[
    Selector {
        mechanic: "lane_troopers",
        path: CONVARS,
        roots: &[
            "citadel_trooper_gold_reward",
            "citadel_trooper_gold_reward_bonus_per_minute",
            "citadel_trooper_squad_size",
            "citadel_trooper_spawn_initial",
            "citadel_trooper_spawn_interval_early",
            "citadel_trooper_spawn_interval_late",
            "citadel_trooper_spawn_interval_late_time",
            "citadel_trooper_spawn_interval_very_late",
            "citadel_trooper_spawn_interval_very_late_time",
            "citadel_trooper_health_mult",
            "citadel_trooper_health_mult_gametime",
            "citadel_trooper_instant_gold_ratio_laning",
            "citadel_trooper_instant_gold_ratio_postlaning",
            "citadel_trooper_laning_gold_rules_end_time",
            "citadel_trooper_siege_instant_gold_ratio",
            "citadel_minion_gold_reward",
            "citadel_minion_gold_reward_bonus_per_minute",
            "citadel_deny_denier_percentage",
            "citadel_deny_denied_percentage",
        ],
        fields: None,
        stat_prefix: "",
    },
    Selector {
        mechanic: "lane_troopers",
        path: NPC_DATA,
        roots: &["trooper_normal", "trooper_melee", "trooper_medic"],
        fields: Some(&["MaxHealth", "MeleeDamage", "DPSPctGrowthPerMinute"]),
        stat_prefix: "",
    },
    Selector {
        mechanic: "soul_sharing",
        path: GENERIC_DATA,
        roots: &["TrooperKillGoldShareFrac", "HeroKillGoldShareFrac"],
        fields: None,
        stat_prefix: "",
    },
    Selector {
        mechanic: "denizens",
        path: CONVARS,
        roots: &["citadel_neutral_gold_ratio"],
        fields: None,
        stat_prefix: "",
    },
    Selector {
        mechanic: "denizens",
        path: NPC_DATA,
        roots: &[
            "neutral_trooper_weak",
            "neutral_trooper_normal",
            "neutral_trooper_strong",
        ],
        fields: Some(NEUTRAL_FIELDS),
        stat_prefix: "",
    },
    Selector {
        mechanic: "objectives",
        path: NPC_DATA,
        roots: &[
            "npc_boss_tier1",
            "npc_boss_tier2",
            "npc_boss_tier3",
            "npc_barrack_boss",
        ],
        fields: Some(OBJECTIVE_FIELDS),
        stat_prefix: "",
    },
    Selector {
        mechanic: "objectives",
        path: NPC_DATA,
        roots: &["trooper_normal", "trooper_melee", "trooper_medic"],
        fields: Some(&["ShrinesDownBuffModifier"]),
        stat_prefix: "",
    },
    Selector {
        mechanic: "objectives",
        path: CONVARS,
        roots: &[
            "citadel_super_trooper_gold_mult",
            "citadel_no_orbs_on_objective_kill",
        ],
        fields: None,
        stat_prefix: "",
    },
    Selector {
        mechanic: "objectives",
        path: GENERIC_DATA,
        roots: &["ObjectiveParams"],
        fields: None,
        stat_prefix: "",
    },
    Selector {
        mechanic: "mid_boss",
        path: CONVARS,
        roots: &[
            "citadel_super_neutral_gold_reward",
            "citadel_super_neutral_gold_reward_bonus_per_minute",
        ],
        fields: None,
        stat_prefix: "",
    },
    Selector {
        mechanic: "mid_boss",
        path: NPC_DATA,
        roots: &["npc_super_neutral"],
        fields: Some(&[
            "StartingHealth",
            "HealthGainPerMinute",
            "InitialSpawnDelay",
            "SpawnInterval",
        ]),
        stat_prefix: "",
    },
    Selector {
        mechanic: "rejuvenator",
        path: GENERIC_DATA,
        roots: &["RejuvParams"],
        fields: None,
        stat_prefix: "",
    },
    Selector {
        mechanic: "hero_kills",
        path: CONVARS,
        roots: &[
            "citadel_player_starting_gold",
            "citadel_player_gold_reward_min",
            "citadel_player_gold_reward_max",
            "citadel_player_gold_reward_time",
            "citadel_player_gold_reward_first_kill_bonus",
            "citadel_player_gold_killer_to_assist_ratio",
            "citadel_player_gold_comeback_multiplier",
            "citadel_player_gold_comeback_killer_fraction",
            "citadel_player_gold_comeback_assister_ratio",
        ],
        fields: None,
        stat_prefix: "",
    },
    Selector {
        mechanic: "respawn",
        path: CONVARS,
        roots: &[
            "citadel_player_spawn_time_min_ramp_1",
            "citadel_player_spawn_time_min_ramp_1_time",
            "citadel_player_spawn_time_min_ramp_2",
            "citadel_player_spawn_time_min_ramp_2_time",
            "citadel_player_spawn_time_min_ramp_3",
            "citadel_player_spawn_time_min_ramp_3_time",
            "citadel_player_spawn_time_max_ramp_1",
            "citadel_player_spawn_time_max_ramp_1_time",
            "citadel_player_spawn_time_max_ramp_2",
            "citadel_player_spawn_time_max_ramp_2_time",
            "citadel_player_spawn_time_max_ramp_3",
            "citadel_player_spawn_time_max_ramp_3_time",
            "citadel_player_spawn_time_max_respawn_time",
            "citadel_player_spawn_time_in_base_penalty",
            "citadel_tier3_phase1_kill_respawn_reduction",
            "citadel_tier3_phase1_kill_respawn_reduction_limit",
        ],
        fields: None,
        stat_prefix: "",
    },
    Selector {
        mechanic: "urn",
        path: CONVARS,
        roots: &[
            "citadel_crate_spawn_initial_delay",
            "citadel_crate_respawn_interval",
            "citadel_crate_reward_base",
            "citadel_crate_reward_time_multiplier",
            "citadel_crate_delivery_overtime_bonus",
            "citadel_idol_cashin_buff_count",
            "citadel_idol_orbs_to_spawn",
            "citadel_idol_duration_until_decay",
            "citadel_idol_decay_duration",
        ],
        fields: None,
        stat_prefix: "",
    },
    Selector {
        mechanic: "breakables",
        path: GENERIC_DATA,
        roots: &["BreakableSpawnTimeDesc"],
        fields: None,
        stat_prefix: "",
    },
    Selector {
        mechanic: "sinners_sacrifice",
        path: NPC_DATA,
        roots: &["neutral_sinners_sacrifice"],
        fields: Some(NEUTRAL_FIELDS),
        stat_prefix: "",
    },
    Selector {
        mechanic: "item_tiers",
        path: GENERIC_DATA,
        roots: &["ItemPricePerTier", "ItemCorruptionPricePerTier"],
        fields: None,
        stat_prefix: "",
    },
    Selector {
        mechanic: "levels",
        path: SOUL_UNLOCK_DATA,
        roots: &[],
        fields: None,
        stat_prefix: "level_",
    },
    Selector {
        mechanic: "street_brawl",
        path: GENERIC_DATA,
        roots: &["StreetBrawl"],
        fields: Some(STREET_BRAWL_FIELDS),
        stat_prefix: "",
    },
];

pub struct MechanicsExtractor;

impl Extractor for MechanicsExtractor {
    fn name(&self) -> &'static str {
        "mechanics_v1"
    }

    fn accepts(&self, source: &str, path: &str) -> bool {
        source == SOURCE && PATHS.contains(&path)
    }

    fn extract(&self, raw: &RawObject) -> Outcome {
        if !self.accepts(&raw.source, &raw.path) {
            return Outcome::Ignored;
        }
        let timers = match timer_facts(raw) {
            Ok(timers) => timers,
            Err(rejection) => return Outcome::Rejected(rejection),
        };
        let rules = facts(&raw.path, &raw.bytes).and_then(|rules| combined(rules, timers.clone()));
        match (rules, timers) {
            (Ok(set), _) => Outcome::Accepted(set),
            (Err(_), Some(timers)) => Outcome::Accepted(timers),
            (Err(rejection), None) => Outcome::Rejected(rejection),
        }
    }
}

fn timer_facts(raw: &RawObject) -> Result<Option<FactSet>, Rejection> {
    if crate::timers::TimerExtractor.accepts(&raw.source, &raw.path) {
        crate::timers::facts(raw).map(Some)
    } else {
        Ok(None)
    }
}

fn combined(mut set: FactSet, timers: Option<FactSet>) -> Result<FactSet, Rejection> {
    if let Some(timers) = timers {
        set.entities.extend(timers.entities);
        set.aliases.extend(timers.aliases);
        set.facts.extend(timers.facts);
        set.relations.extend(timers.relations);
        limits::validate(&set)?;
    }
    Ok(set)
}

struct Selected<'a> {
    selector: &'a Selector,
    root: &'a str,
    field: String,
}

fn select(path: &str, pointer: &str) -> Option<Selected<'static>> {
    let mut segments = pointer.strip_prefix('/')?.split('/');
    let first = segments.next()?;
    SELECTORS
        .iter()
        .filter(|selector| selector.path == path)
        .find_map(|selector| {
            if selector.roots.is_empty() {
                return Some(Selected {
                    selector,
                    root: "",
                    field: pointer.trim_start_matches('/').to_owned(),
                });
            }
            let root = selector.roots.iter().find(|root| **root == first)?;
            let rest: Vec<&str> = segments.clone().collect();
            if path == CONVARS {
                return matches!(rest.as_slice(), [] | ["value"]).then(|| Selected {
                    selector,
                    root,
                    field: (*root).to_owned(),
                });
            }
            if let Some(fields) = selector.fields {
                rest.first().filter(|field| fields.contains(field))?;
            }
            Some(Selected {
                selector,
                root,
                field: pointer.trim_start_matches('/').to_owned(),
            })
        })
}

fn slug(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}

fn file_slug(path: &str) -> String {
    slug(
        path.rsplit('/')
            .next()
            .unwrap_or(path)
            .trim_end_matches(".json"),
    )
}

fn rejection(code: &'static str) -> Rejection {
    Rejection::new(code, "Spielregelobjekt entspricht nicht dem Quellvertrag")
}

pub fn alias(mechanic: &str) -> String {
    format!("mechanic_{mechanic}")
}

fn facts(path: &str, bytes: &[u8]) -> Result<FactSet, Rejection> {
    let input = std::str::from_utf8(bytes).map_err(|_| rejection("invalid_utf8"))?;
    let leaves = json_text::parse(input).map_err(rejection)?;
    let mut set = FactSet::default();
    let mut entities = BTreeSet::new();
    for leaf in leaves {
        let Some(selected) = select(path, &leaf.pointer) else {
            continue;
        };
        match leaf.value {
            Value::Bool(_) => {}
            Value::Number(_) | Value::String(_) if leaf.numeric_representation.is_some() => {
                limits::validate_numeric_lexeme(&leaf.lexeme)?;
            }
            _ => continue,
        }
        let mechanic = selected.selector.mechanic;
        let entity_key = format!("rule_{mechanic}__{}", file_slug(path));
        let receipt = Receipt {
            locator_kind: "json_pointer".into(),
            locator: leaf.pointer.clone(),
            value_sha256: Sha256::digest(leaf.lexeme.as_bytes()).into(),
        };
        if entities.insert(entity_key.clone()) {
            set.entities.push(Entity {
                kind: "rule".into(),
                key: entity_key.clone(),
                receipt: receipt.clone(),
            });
            set.aliases.push(Alias {
                entity_key: entity_key.clone(),
                alias: alias(mechanic),
                lang: "und".into(),
                receipt: receipt.clone(),
            });
        }
        let stat = format!("{}{}", selected.selector.stat_prefix, slug(&selected.field));
        if !stat.starts_with(|c: char| c.is_ascii_lowercase()) {
            return Err(rejection("invalid_key"));
        }
        let payload = json!({
            "value":leaf.lexeme,
            "unit":Value::Null,
            "condition":{"source_entity":selected.root,"field":leaf.pointer}
        });
        set.facts.push(Fact {
            entity_key,
            stat_key: stat,
            value: FactValue::Text(payload.to_string()),
            receipt,
        });
    }
    limits::validate(&set)?;
    Ok(set)
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_storage::gamefacts::receipts;

    fn run(path: &str, input: &str) -> FactSet {
        let set = facts(path, input.as_bytes()).unwrap();
        receipts::verify(input.as_bytes(), &set).unwrap();
        set
    }

    fn value(set: &FactSet, entity: &str, stat: &str) -> Value {
        let fact = set
            .facts
            .iter()
            .find(|fact| fact.entity_key == entity && fact.stat_key == stat)
            .unwrap();
        let FactValue::Text(text) = &fact.value else {
            panic!("text payload expected");
        };
        serde_json::from_str::<Value>(text).unwrap()["value"].clone()
    }

    #[test]
    fn convars_keep_plain_and_described_values_with_verified_receipts() {
        let input = r#"{"citadel_trooper_gold_reward":100,
            "citadel_trooper_gold_reward_bonus_per_minute":{"value":2,"description":"x"},
            "citadel_trooper_spawn_enabled":true,"citadel_neutral_gold_ratio":1,
            "citadel_super_neutral_gold_reward":3000,"citadel_player_gold_reward_min":200,
            "citadel_player_spawn_time_min_ramp_1":8,"citadel_crate_reward_base":250,
            "unrelated":5}"#;
        let set = run(CONVARS, input);
        let entity = "rule_lane_troopers__convars";
        assert_eq!(value(&set, entity, "citadel_trooper_gold_reward"), "100");
        assert_eq!(
            value(&set, entity, "citadel_trooper_gold_reward_bonus_per_minute"),
            "2"
        );
        assert!(set.facts.iter().all(|fact| fact.stat_key != "unrelated"));
        assert!(set
            .aliases
            .iter()
            .any(|alias| alias.alias == "mechanic_urn" && alias.lang == "und"));
    }

    #[test]
    fn nested_arrays_and_field_filters_follow_the_catalogue() {
        let input = r#"{"TrooperKillGoldShareFrac":[1.0,0.54],"HeroKillGoldShareFrac":[1.25],
            "ObjectiveParams":{"Tier1GoldKill":1375},"RejuvParams":{"TrooperHealthMult":[1.7]},
            "BreakableSpawnTimeDesc":[{"InitialSpawnTime":180}],"ItemPricePerTier":[0,800],
            "StreetBrawl":{"m_iScoreToWin":3,"m_vecItemDraftRoundsPerGameRound":[{"x":1}]}}"#;
        let set = run(GENERIC_DATA, input);
        assert_eq!(
            value(
                &set,
                "rule_soul_sharing__generic_data",
                "trooperkillgoldsharefrac_1"
            ),
            "0.54"
        );
        assert_eq!(
            value(
                &set,
                "rule_objectives__generic_data",
                "objectiveparams_tier1goldkill"
            ),
            "1375"
        );
        assert!(set
            .facts
            .iter()
            .all(|fact| !fact.stat_key.contains("itemdraft")));
    }

    #[test]
    fn level_rows_get_a_letter_prefix() {
        let input = r#"[{"RequiredSouls":0,"AbilityUnlocks":1},{"RequiredSouls":400,"PowerIncrease":true}]"#;
        let set = run(SOUL_UNLOCK_DATA, input);
        assert_eq!(
            value(
                &set,
                "rule_levels__soul_unlock_data",
                "level_1_requiredsouls"
            ),
            "400"
        );
        assert_eq!(
            value(
                &set,
                "rule_levels__soul_unlock_data",
                "level_1_powerincrease"
            ),
            "true"
        );
    }

    #[test]
    fn rule_rejections_keep_the_timer_facts_of_shared_files() {
        let raw = |bytes: &[u8]| RawObject {
            id: 1,
            source: SOURCE.into(),
            revision: "fixture".into(),
            path: GENERIC_DATA.into(),
            content_sha256: Sha256::digest(bytes).into(),
            bytes: bytes.to_vec(),
        };
        let shared = format!(
            r#"{{"RejuvParams":{{"RejuvinatorBuffDuration":120}},"ItemPricePerTier":[1.{}]}}"#,
            "1".repeat(limits::TEXT_BYTES + 1)
        );
        assert!(facts(GENERIC_DATA, shared.as_bytes()).is_err());
        let Outcome::Accepted(set) = MechanicsExtractor.extract(&raw(shared.as_bytes())) else {
            panic!("timer facts must survive a rule rejection");
        };
        assert!(set.entities.iter().all(|entity| entity.kind == "timer"));
        assert!(!set.facts.is_empty());
        let both = br#"{"RejuvParams":{"RejuvinatorBuffDuration":120}}"#;
        let Outcome::Accepted(set) = MechanicsExtractor.extract(&raw(both)) else {
            panic!("shared file must be accepted");
        };
        let kinds: BTreeSet<&str> = set
            .entities
            .iter()
            .map(|entity| entity.kind.as_str())
            .collect();
        assert_eq!(kinds, BTreeSet::from(["rule", "timer"]));
    }

    #[test]
    fn shrine_buffs_and_objective_convars_join_the_objectives_rule() {
        let set = run(
            NPC_DATA,
            r#"{"trooper_normal":{"MaxHealth":300,"ShrinesDownBuffModifier":{"HealthScale":1.4,
                "DamageScale":60.0,"EnabledStateMask":"MODIFIER_STATE_SUPER_TROOPER"}}}"#,
        );
        assert_eq!(
            value(
                &set,
                "rule_objectives__npc_data",
                "trooper_normal_shrinesdownbuffmodifier_damagescale"
            ),
            "60.0"
        );
        assert_eq!(
            value(
                &set,
                "rule_lane_troopers__npc_data",
                "trooper_normal_maxhealth"
            ),
            "300"
        );
        let set = run(
            CONVARS,
            r#"{"citadel_super_trooper_gold_mult":{"value":0.85,"description":"x"},
                "citadel_no_orbs_on_objective_kill":true}"#,
        );
        assert_eq!(
            value(
                &set,
                "rule_objectives__convars",
                "citadel_super_trooper_gold_mult"
            ),
            "0.85"
        );
        assert_eq!(
            value(
                &set,
                "rule_objectives__convars",
                "citadel_no_orbs_on_objective_kill"
            ),
            "true"
        );
    }

    #[test]
    fn missing_catalogue_roots_yield_only_present_rules() {
        let set = run(NPC_DATA, r#"{"trooper_normal":{"MaxHealth":300}}"#);
        assert_eq!(set.entities.len(), 1);
        assert_eq!(set.facts.len(), 1);
        assert!(facts(CONVARS, b"{").is_err());
    }
}
