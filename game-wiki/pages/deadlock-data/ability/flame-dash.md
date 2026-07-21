---
title: "Flame Dash"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_flame_dash"
canonical_name: "Flame Dash"
snapshot_id: 39438
source_document_id: 7070
payload_hash: "7830481a9528a914e4d4f94af295cae847c6f2a99233b17439a87df2b67a0a1e"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.324939+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Flame Dash

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_flame_dash`
- Snapshot ID: `39438`
- Source-Dokument: `7070`
- Kurzinfo: Flame Dash aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 38.0,
  "AbilityDuration": 3.0,
  "AbilityUnitTargetLimit": 1,
  "AuraLingerDuration": 1.0,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventBotUsage",
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": 18,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.7
    },
    "Value": 30
  },
  "DashAirSpeed": 8,
  "DashSpeed": 12,
  "FlameAuraRadius": 4.5,
  "FlameDashJumpBonus": 50,
  "FlameDashModifier": {
    "Class": "Flamedash",
    "EnabledStateMask": [
      "AbilityMovement",
      "MantleDisabled",
      "MeleeDisabled",
      "SlidingDisabled",
      "DuckingDisabled"
    ],
    "GroundAuraModifier": {
      "Class": "FlamedashGroundAura",
      "Height": 80.0,
      "ProvidedByAura": {
        "Class": "FlamedashBurn",
        "DebuffModifier": {
          "Class": "Base",
          "Subclass": "FlamedashDebuff"
        },
        "Subclass": "FlamedashBurn"
      },
      "Subclass": "FlamedashGroundAura"
    },
    "ProgressModifier": {
      "Class": "Base",
      "Subclass": "ProgressWatcher"
    },
    "Subclass": "Flamedash"
  },
  "GroundAuraSpacing": 1,
  "GroundFlameDuration": 4,
  "IsDisabled": false,
  "Key": "ability_flame_dash",
  "Name": "Flame Dash",
  "SideMoveSpeedReduction": -65,
  "SlowResistance": 50,
  "SpeedBurstSpeed": 20,
  "TickRate": 0.5,
  "Upgrades": [
    {
      "AbilityCooldown": -12.0
    },
    {
      "DPS": 20.0,
      "GroundFlameDuration": 1
    },
    {
      "AbilityCharges": 2,
      "AbilityCooldownBetweenCharge": 14
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
    "card_name": "Flame Dash",
    "hero_key": "hero_inferno",
    "hero_name": "Infernus",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_inferno",
      "hero_name": "Infernus",
      "lookup": "flame dash",
      "name": "Flame Dash",
      "type": "ability"
    }
  ]
}
````
