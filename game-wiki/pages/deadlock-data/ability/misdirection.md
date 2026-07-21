---
title: "Misdirection"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_magewalk"
canonical_name: "Misdirection"
snapshot_id: 39627
source_document_id: 7070
payload_hash: "4c07648e0122cbebfc8e3302d8481efa305f93592f685265be82dcd9e074bcae"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.809460+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Misdirection

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_magewalk`
- Snapshot ID: `39627`
- Source-Dokument: `7070`
- Kurzinfo: Misdirection aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 10,
  "AbilityCooldown": 15,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventBotUsage",
    "BehaviorAllowAltCast",
    "BehaviorMovement",
    "BehaviorCanSetQuickCast"
  ],
  "BubbleModifier": {
    "Class": "Magewalk",
    "StatusEffectPriority": 100,
    "Subclass": "Magewalk"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.465
    },
    "Value": 30
  },
  "EmpoweredAttackCount": 2,
  "FireRateBonus": 25,
  "FireRateBonusDurationMax": 8,
  "ImpulseStrength": -3000,
  "IsDisabled": false,
  "Key": "citadel_ability_magewalk",
  "MageTime": 1.5,
  "Name": "Misdirection",
  "ProcChance": 100,
  "TrailInterval": 0.01,
  "TurretAttackDelay": 1,
  "TurretAttackFalloffEnd": null,
  "TurretAttackFalloffStart": null,
  "TurretBaseHealth": 400,
  "TurretHealthScaling": null,
  "TurretLifetime": 8,
  "TurretModifier": {
    "Class": "Base",
    "Subclass": "TurretModifier"
  },
  "Upgrades": [
    {
      "AbilityCastRange": 4
    },
    {
      "FireRateBonus": 25
    },
    {
      "AbilityCooldown": -4,
      "BonusClipSizePercent": 120
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
  }
}
````
