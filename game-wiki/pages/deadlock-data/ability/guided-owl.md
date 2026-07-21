---
title: "Guided Owl"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_guided_arrow"
canonical_name: "Guided Owl"
snapshot_id: 39453
source_document_id: 7070
payload_hash: "4cca66047aec9e9bae150486f9683af5ede35756fa58aedf2d391f9b66b3c043"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.361684+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Guided Owl

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_guided_arrow`
- Snapshot ID: `39453`
- Source-Dokument: `7070`
- Kurzinfo: Guided Owl aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1.5,
  "AbilityChannelTime": 20,
  "AbilityCooldown": 125.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BonusTechPowerPerKill": 8,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.93744
    },
    "Value": 230.0
  },
  "ExplosionRadius": 12,
  "GuidingModifier": {
    "Class": "GuidingArrow",
    "EnabledStateMask": [
      "JumpDisabled",
      "ZiplineDisabled",
      "DashDisabled"
    ],
    "GlowEnemeyModifier": {
      "Class": "LowHealthGlow",
      "Subclass": "LowHealthGlow"
    },
    "Subclass": "GuidingArrow"
  },
  "IsDisabled": false,
  "Key": "ability_guided_arrow",
  "KillCheckModifier": {
    "Class": "GuidingArrowKillcheck",
    "Subclass": "KillcheckModifier"
  },
  "Name": "Guided Owl",
  "StunDuration": 0.75,
  "Upgrades": [
    {
      "Damage": 85.0
    },
    {
      "AbilityCooldown": -40.0
    },
    {
      "LowHealthEnemyThresholdPct": 22
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
    "card_name": "Guided Owl",
    "hero_key": "hero_orion",
    "hero_name": "Grey Talon",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_orion",
      "hero_name": "Grey Talon",
      "lookup": "guided owl",
      "name": "Guided Owl",
      "type": "ability"
    }
  ]
}
````
