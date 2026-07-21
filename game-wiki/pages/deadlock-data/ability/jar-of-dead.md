---
title: "Jar of Dead"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_necro_hauntingskull"
canonical_name: "Jar of Dead"
snapshot_id: 39498
source_document_id: 7070
payload_hash: "adeadeba6368553eacaedde525d6e8f1336d23fd0cd6459d8b38c84780d391d3"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.483938+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Jar of Dead

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_necro_hauntingskull`
- Snapshot ID: `39498`
- Source-Dokument: `7070`
- Kurzinfo: Jar of Dead aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCharges": 4,
  "AbilityChargesConditionally": 1,
  "AbilityCooldownBetweenCharge": 13,
  "AbilityUnitTargetLimit": 1,
  "AreaModifier": {
    "Class": "NecroHauntingskullArea",
    "InitialRandomVariance": 30.0,
    "SpawnPositionNavMeshSearchRange": 60.0,
    "Subclass": "Area"
  },
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.25
    },
    "Value": 16
  },
  "DelayBeforeRespawning": 1,
  "HealPerPickup": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "IsDisabled": false,
  "Key": "ability_necro_hauntingskull",
  "KillTime": 0.2,
  "MaxHits": -1,
  "Name": "Jar of Dead",
  "PickupsPerBossDeath": 5,
  "PickupsPerDeath": 1,
  "PickupsPerHeroDeath": 5,
  "PickupsPerNeutralTrooperDeath": 2,
  "ResourceCost": {
    "Scale": {
      "Type": "cooldown",
      "Value": 1.0
    },
    "Value": 120
  },
  "ResourceGenerationPercent": 100,
  "ResourcePerPickup": 10,
  "ResourceRadius": 40,
  "SkullCount": 4,
  "SkullImmuneDuration": 0.15,
  "SkullKillGold": {
    "Scale": {
      "Type": "power_increase",
      "Value": 0.5
    },
    "Value": 7
  },
  "SkullLifetime": 10,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "SpawnRadius": 2,
  "StackingDebuffModifier": {
    "Class": "NecroHauntingskullStackingdebuff",
    "Subclass": "Debuff"
  },
  "SummonBuffModifier": {
    "Class": "Base",
    "Subclass": "Summonbuff"
  },
  "SummonHealth": {
    "Scale": {
      "Type": "power_increase",
      "Value": 1.3
    },
    "Value": 20
  },
  "SummonModifier": {
    "Class": "BarrierTracker",
    "EnabledStateMask": [
      "IgnoredByNpcTargeting",
      "IsSmallDeployable"
    ],
    "Subclass": "Barriertracker"
  },
  "SummonTakesDamage": 1,
  "TargetDashRadius": {
    "Scale": {
      "Type": "range",
      "Value": 1.0
    },
    "Value": 15
  },
  "TargetSearchDelayMax": 1.25,
  "TargetSearchDelayMin": 1.5,
  "TargetSearchInitialDelayMax": 0.2,
  "TargetSearchInitialDelayMin": 0.15,
  "TargetSearchInitialStagger": 0.125,
  "TargetSearchRadius": 7,
  "TickRate": 0.2,
  "Upgrades": [
    {
      "HealPerPickup": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.1
        },
        "Value": 5
      }
    },
    {
      "SlowDuration": 1,
      "SlowPercent": 30
    },
    {
      "SkullCount": 2,
      "SkullLifetime": 4
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
    "card_name": "Jar of Dead",
    "hero_key": "hero_necro",
    "hero_name": "Graves",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_necro",
      "hero_name": "Graves",
      "lookup": "jar of dead",
      "name": "Jar of Dead",
      "type": "ability"
    }
  ]
}
````
