---
title: "Combo"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_ult_combo"
canonical_name: "Combo"
snapshot_id: 39550
source_document_id: 7070
payload_hash: "5f92b77537074ed8c8fe1bc2ad7c699ea21582c208bef7e14f9a4bd2d7694a45"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.612158+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Combo

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_ult_combo`
- Snapshot ID: `39550`
- Source-Dokument: `7070`
- Kurzinfo: Combo aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 4.0,
  "AbilityChannelTime": 2.4,
  "AbilityCooldown": 150.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorDisplaysDamageImpact",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BonusHealthOnKill": {
    "Scale": {
      "Type": "power_increase",
      "Value": 2
    },
    "Value": 40
  },
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 40
  },
  "IsDisabled": false,
  "Key": "ability_ult_combo",
  "KillCheckModifier": {
    "Class": "Base",
    "Subclass": "KillcheckModifier"
  },
  "Name": "Combo",
  "SelfModifier": {
    "Class": "UltComboSelf",
    "EnabledStateMask": [
      "ForceCanParry"
    ],
    "Subclass": "UltComboSelf"
  },
  "TargetModifier": {
    "Class": "UltComboTarget",
    "StatusEffectPriority": 50,
    "Subclass": "UltComboTarget",
    "TargetPosDistance": 100.0,
    "TargetPosRange": 20.0
  },
  "Upgrades": [
    {
      "LifeStealPercentOnHit": 100
    },
    {
      "AbilityCooldown": -30,
      "BulletResist": 50
    },
    {
      "AbilityChannelTime": 0.7,
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.4
        },
        "Value": 40
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
    "card_name": "Combo",
    "hero_key": "hero_krill",
    "hero_name": "Mo & Krill",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_krill",
      "hero_name": "Mo & Krill",
      "lookup": "combo",
      "name": "Combo",
      "type": "ability"
    }
  ]
}
````
