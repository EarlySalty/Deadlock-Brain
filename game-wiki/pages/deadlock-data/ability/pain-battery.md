---
title: "Pain Battery"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_frank_shocktarget2"
canonical_name: "Pain Battery"
snapshot_id: 39446
source_document_id: 7070
payload_hash: "d72af38ffa6f5dd698bfda8c23af3115d211ee3289ee0e4e2dfd1318e2ec8d68"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.345214+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Pain Battery

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_frank_shocktarget2`
- Snapshot ID: `39446`
- Source-Dokument: `7070`
- Kurzinfo: Pain Battery aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.35,
  "AbilityCastRange": 28,
  "AbilityChargesConditionally": 1,
  "AbilityCooldown": 2,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "BatteryGenerationPercent": {
    "Scale": {
      "Type": "cooldown",
      "Value": -1.0
    },
    "Value": 100
  },
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BoltCount": 7,
  "BonusShocksDelay": 0.2,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.6
    },
    "Value": 100
  },
  "FullyChargedFXModifier": {
    "Class": "FrankShockfullycharged",
    "Subclass": "ChargedFx"
  },
  "HealOnHit": {
    "Scale": {
      "Type": "healing",
      "Value": 1.0
    },
    "Value": 0
  },
  "IsDisabled": false,
  "Key": "ability_frank_shocktarget2",
  "MissingHealthPercentHeal": {
    "Scale": {
      "Type": "healing",
      "Value": 1.0
    },
    "Value": 0
  },
  "Name": "Pain Battery",
  "ShockModifier": {
    "Class": "FrankShocktarget",
    "Subclass": "Shock"
  },
  "SlowModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "ZapSlow"
  },
  "SpreadAngle": 40,
  "SpreadRandomness": 0.005,
  "StoredDamageHealthPercentRequired": 40,
  "Upgrades": [
    {
      "SlowDuration": "2s",
      "SlowPercent": 40
    },
    {
      "Damage": 50.0
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 0
      },
      "MissingHealthPercentHeal": 15
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
    "card_name": "Pain Battery",
    "hero_key": "hero_frank",
    "hero_name": "Victor",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_frank",
      "hero_name": "Victor",
      "lookup": "pain battery",
      "name": "Pain Battery",
      "type": "ability"
    }
  ]
}
````
