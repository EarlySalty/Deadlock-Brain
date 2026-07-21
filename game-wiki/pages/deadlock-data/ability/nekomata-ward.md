---
title: "Nekomata Ward"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_nano_proximity_ritual"
canonical_name: "Nekomata Ward"
snapshot_id: 39491
source_document_id: 7070
payload_hash: "d7afee31ebbe8d965ae614867d4650e23b4e101636073017dffaf01ea9b1acb9"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.468114+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Nekomata Ward

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_nano_proximity_ritual`
- Snapshot ID: `39491`
- Source-Dokument: `7070`
- Kurzinfo: Nekomata Ward aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 30.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 60,
  "AbilityUnitTargetLimit": 1,
  "ActiveRadius": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.186
    },
    "Value": 40
  },
  "AttackRadius": 30,
  "BehaviourBits": [
    "BehaviorProjectileFiredAsBullet",
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorAllowSelfCast",
    "BehaviorCanSetQuickCast",
    "BehaviorCanCancelDuringCastDelay"
  ],
  "CatActivateDuration": 2.0,
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.279
    },
    "Value": 40
  },
  "DamageTick": 1.0,
  "HealAmpReceivePenaltyPercent": -20,
  "HealAmpRegenPenaltyPercent": -20,
  "InvisFadeToDuration": 1.0,
  "IsDisabled": false,
  "Key": "ability_nano_proximity_ritual",
  "Name": "Nekomata Ward",
  "PredatoryStatueModifier": {
    "Class": "NanoPredatoryStatue",
    "MinRevealTime": 2.0,
    "NewTargetAttackTime": 0.5,
    "RevealModifier": {
      "Class": "Base",
      "EnabledStateMask": [
        "VisibleToEnemy"
      ],
      "Subclass": "RevealModifier"
    },
    "SelfHealScale": 0.2,
    "Subclass": "NanoPredatoryStatue",
    "TargetModifier": {
      "Class": "NanoPredatoryStatueTarget",
      "EnabledStateMask": [
        "PredatoryStatueTarget"
      ],
      "Subclass": "NanoPredatoryStatueTarget"
    }
  },
  "RecentDamageMarkDuration": 1.5,
  "RecentDamageModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "NanoRecentDamage"
    ],
    "Subclass": "NanoRecentDamage"
  },
  "RevealOnDamageDuration": 1.5,
  "RevealOnSpottedDuration": 1.5,
  "SpottedRadius": 20,
  "StatueArmTime": 0.5,
  "StatueHealth": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.86
    },
    "Value": 300
  },
  "TargetLifesteal": 30,
  "TargetLifestealNonHero": 10,
  "TickInterval": 0.1,
  "Upgrades": [
    {
      "MakeInvisible": 1
    },
    {
      "AbilityDuration": 30,
      "DPS": 20
    },
    {
      "HealAmpReceivePenaltyPercent": -30,
      "HealAmpRegenPenaltyPercent": -30,
      "VictimDamageReduction": -30
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
