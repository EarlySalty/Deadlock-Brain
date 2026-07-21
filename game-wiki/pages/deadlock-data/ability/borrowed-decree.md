---
title: "Borrowed Decree"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_necro_gravestone"
canonical_name: "Borrowed Decree"
snapshot_id: 39496
source_document_id: 7070
payload_hash: "a45b694ef64dc05a196105ac9434e479ec7d06b425f06e9b64f823f3e8218b5e"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.478907+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Borrowed Decree

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_necro_gravestone`
- Snapshot ID: `39496`
- Source-Dokument: `7070`
- Kurzinfo: Borrowed Decree aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 20,
  "AbilityChannelTime": 0.66,
  "AbilityCooldown": 140,
  "AbilityDuration": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.04
    },
    "Value": 16
  },
  "AbilityUnitTargetLimit": 1,
  "AuraRadius": 8,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorCanSetQuickCast",
    "BehaviorRefundFullCooldownOnChannelInterrupt",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BlockerScaleFactor": 1,
  "BonusHealthRegen": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "BonusSpiritDamagePercentage": 15,
  "BuffDuration": -1,
  "BulletResist": 12.5,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.27
    },
    "Value": 115
  },
  "DamageSlowDuration": 0.5,
  "DamageSlowPercent": 20,
  "DecayDuration": 1,
  "DecayTickRate": 0.1,
  "ExplodeDelay": 0.23,
  "ExplosionRadius": 6.5,
  "GraveStoneModifier": {
    "Class": "Gravestone",
    "EnabledStateMask": [
      "TechUntargetableByEnemies",
      "IgnoredByNpcTargeting"
    ],
    "GravestoneCriticalModifier": {
      "Class": "Base",
      "Subclass": "Gravestonecritical"
    },
    "ProvidedByAura": {
      "Class": "Base",
      "EnabledStateMask": [
        "NearHeavyPunchableDestroy"
      ],
      "Subclass": "Nearby"
    },
    "Subclass": "Gravestone"
  },
  "GravestoneHealth": 100,
  "GravestoneTakesDamage": 1,
  "GrowTime": 0.1,
  "IsDisabled": false,
  "Key": "ability_necro_gravestone",
  "KnockupRadius": 4,
  "KnockupSideRatio": 1,
  "KnockupSpeed": 240,
  "MaxGravestones": 3,
  "MaxStacks": 40,
  "Name": "Borrowed Decree",
  "PushForce": 300,
  "ReplicateZombieCast": 1,
  "SlowDuration": 1.25,
  "SlowPercent": 80,
  "SlowPercentPerStack": 0.5,
  "SpawnDuration": 1.5,
  "StackDuration": 5,
  "StackingDebuffTickRate": 0.25,
  "SummonBurstCount": 2,
  "SummonBurstFrequency": 0.1,
  "SummonFrequency": 4,
  "SummonHealth": {
    "Scale": {
      "Type": "power_increase",
      "Value": 8.0
    },
    "Value": 180
  },
  "SummonInitialDelay": 0.3,
  "SummonLifetime": 20,
  "SummonMaxCount": 32,
  "SummonMeleeDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.5
    },
    "Value": 40
  },
  "SummonSearchRadius": 4,
  "TechArmorDamageReductionPerStack": -0.5,
  "TechPower": {
    "Scale": {
      "Type": "power_increase",
      "Value": 1.0
    },
    "Value": 0
  },
  "TickRate": 0.4,
  "Upgrades": [
    {
      "AbilityCooldown": -15
    },
    {
      "AbilityDuration": 10,
      "MoveSpeedPercent": 25
    },
    {
      "CurrentHealthDamagePercentage": 5
    }
  ],
  "ZombieSummonModifier": {
    "Class": "NecroSummonzombiesArea",
    "ForwardWalkDistance": 0.0,
    "SpawningInModifier": {
      "Class": "Base",
      "EnabledStateMask": [
        "Immobilized",
        "CommandRestricted",
        "UnitStatusHealthHidden",
        "UnitStatusHidden",
        "DoNotDrawModel"
      ],
      "Subclass": "Spawningin"
    },
    "SpawningInTime": 0.1,
    "Subclass": "Summonzombiesarea",
    "SummonDecayModifier": {
      "Class": "NecroSummondecay",
      "Subclass": "SummonDecay"
    },
    "SummonModifier": {
      "Class": "Base",
      "Subclass": "SummonBuffs"
    },
    "ZombieSpawnForwardOffset": 100.0,
    "ZombieSpawnNavMeshSearchDistance": 300.0
  },
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
    "card_name": "Borrowed Decree",
    "hero_key": "hero_necro",
    "hero_name": "Graves",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_necro",
      "hero_name": "Graves",
      "lookup": "borrowed decree",
      "name": "Borrowed Decree",
      "type": "ability"
    }
  ]
}
````
