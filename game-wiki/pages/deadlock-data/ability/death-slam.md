---
title: "Death Slam"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_lash_ultimate"
canonical_name: "Death Slam"
snapshot_id: 39625
source_document_id: 7070
payload_hash: "6ecfa24784fbef976d4146dace5503f8a481fbceca2f84032576a09018c71771"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.804729+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Death Slam

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_lash_ultimate`
- Snapshot ID: `39625`
- Source-Dokument: `7070`
- Kurzinfo: Death Slam aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.3,
  "AbilityCastRange": 20,
  "AbilityChannelTime": 2.3,
  "AbilityCooldown": 170.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 6,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast",
    "BehaviorCastRangeIs2d"
  ],
  "BoostTime": 1.0,
  "ChannelMoveSpeed": -1,
  "GrappleEnemyModifier": {
    "Class": "LashGrappleEnemyDebuff",
    "DebuffModifier": {
      "Class": "SlowBase",
      "Subclass": "Slow"
    },
    "EnabledStateMask": [
      "AbilityMovementDebuff"
    ],
    "Subclass": "LashGrappleEnemyDebuff"
  },
  "GrappleTargetModifier": {
    "Class": "LashGrappleTarget",
    "Subclass": "Target"
  },
  "HangTime": 0.6,
  "ImpactDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.974938
    },
    "Value": 105
  },
  "ImpactRadius": 6,
  "IsDisabled": false,
  "Key": "citadel_ability_lash_ultimate",
  "LiftHeight": 6,
  "LockonConeAngle": 40,
  "LosingLockGraceTime": 0.4,
  "MaxLockonStacks": 1,
  "Name": "Death Slam",
  "NotInConeLosesLock": 1,
  "SlamSpeed": 1600,
  "SlowDuration": 4,
  "SlowPercent": 50,
  "TargetModifier": {
    "Class": "LashGrappleTarget",
    "Subclass": "Lockon"
  },
  "ThrowDistance": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.14
    },
    "Value": 14
  },
  "ThrowStraightDuration": 1.5,
  "TimeToGainLockonStack": 0.7,
  "TimeToLoseLockonStack": 2,
  "UpBoostSpeed": 400,
  "Upgrades": [
    {
      "ThrowDistance": 12
    },
    {
      "AbilityCooldown": -35
    },
    {
      "AbilityCastRange": 6,
      "StunDuration": 1.2
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
    "card_name": "Death Slam",
    "hero_key": "hero_lash",
    "hero_name": "Lash",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_lash",
      "hero_name": "Lash",
      "lookup": "death slam",
      "name": "Death Slam",
      "type": "ability"
    }
  ]
}
````
