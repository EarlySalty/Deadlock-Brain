---
title: "Rake"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_vampirebat_steallife"
canonical_name: "Rake"
snapshot_id: 39560
source_document_id: 7070
payload_hash: "392dbeee49723c35446282f1ce25593789c982fc4fd3acebbe18ee3daf107403"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.636630+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Rake

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_vampirebat_steallife`
- Snapshot ID: `39560`
- Source-Dokument: `7070`
- Kurzinfo: Rake aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 10,
  "AbilityCooldown": 16,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.4,
  "AbilityUnitTargetLimit": 6,
  "AirDrag": 0.2,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast",
    "BehaviorUseLagCompensationForUnitTargeting"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "stats_count",
      "Value": 1.0
    },
    "Value": 60
  },
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "Debuff"
  },
  "FallSpeedMax": 3,
  "FallingDrag": 20,
  "FloatingModifier": {
    "Class": "Base",
    "Subclass": "Floatingmodifier"
  },
  "IsDisabled": false,
  "Key": "ability_vampirebat_steallife",
  "MaxFloatTime": 4.0,
  "MiniJumpVelocity": 200,
  "MissingHealthDamagePercentage": 6,
  "Name": "Rake",
  "RakeHealPerKill": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.3
    },
    "Value": 25
  },
  "TargetingConeAngle": 60,
  "TimeBetweenAttacks": 0.04,
  "TrooperExecuteThreshold": 60,
  "Upgrades": [
    {
      "Damage": 60
    },
    {
      "AbilityCooldown": -8,
      "RakeHealPerKill": 30
    },
    {
      "MissingHealthDamagePercentage": 7.0,
      "RakeHealPerKill": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.2
        },
        "Value": 0
      }
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-data.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_card": {
    "card_name": "Rake",
    "hero_key": "hero_vampirebat",
    "hero_name": "Mina",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_vampirebat",
      "hero_name": "Mina",
      "lookup": "rake",
      "name": "Rake",
      "type": "ability"
    }
  ]
}
````
