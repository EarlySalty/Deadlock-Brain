---
title: "Burrow"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_burrow"
canonical_name: "Burrow"
snapshot_id: 39413
source_document_id: 7070
payload_hash: "4ce73ce37f5e3481f0830c14543909abbbabc49251ecf0a1b853aeb850e4e58f"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.262106+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Burrow

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_burrow`
- Snapshot ID: `39413`
- Source-Dokument: `7070`
- Kurzinfo: Burrow aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1,
  "AbilityChannelTime": 5,
  "AbilityCooldown": 40.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BonusMoveSpeed": 5,
  "BulletResist": 60,
  "BurrowModifier": {
    "Class": "Burrow",
    "DesatAmount": 0.3,
    "EnabledStateMask": [
      "Sprinting",
      "DamageMovementPenaltyImmune",
      "MantleDisabled",
      "MeleeDisabled",
      "DashDisabled",
      "DuckingDisabled",
      "ZiplineDisabled",
      "AllowInTunnelsNoDuck",
      "ShrunkCharacter",
      "IsTinyCharacter"
    ],
    "Subclass": "Burrow"
  },
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.488
    },
    "Value": 75
  },
  "EnemyDamageSpeedPenalty": 0.5,
  "IsDisabled": false,
  "Key": "ability_burrow",
  "Name": "Burrow",
  "Radius": 5,
  "SpeedLostDuration": 1,
  "SpinDuration": 1.5,
  "SpinModifier": {
    "Class": "Spin",
    "SlowModifier": {
      "Class": "SlowBase",
      "Subclass": "Slow"
    },
    "Subclass": "Spin"
  },
  "SpinSlowDuration": 0.3,
  "SpinSlowPercent": 10,
  "TechResist": 30,
  "TickRate": 0.1,
  "TossDuration": 1,
  "UpForce": 250,
  "Upgrades": [
    {
      "DPS": 50
    },
    {
      "AbilityChannelTime": 4,
      "Radius": 2
    },
    {
      "AbilityCooldown": -20.0,
      "BonusMoveSpeed": 4
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
    "card_name": "Burrow",
    "hero_key": "hero_krill",
    "hero_name": "Mo & Krill",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_krill",
      "hero_name": "Mo & Krill",
      "lookup": "burrow",
      "name": "Burrow",
      "type": "ability"
    }
  ]
}
````
