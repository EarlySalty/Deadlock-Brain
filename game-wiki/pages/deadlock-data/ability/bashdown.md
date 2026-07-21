---
title: "Bashdown"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_punkgoat_ult"
canonical_name: "Bashdown"
snapshot_id: 39523
source_document_id: 7070
payload_hash: "610cc57a330595d607cdacf59eafa5101fe034dbec19b38c044bfa7574a0f52a"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.539927+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Bashdown

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_punkgoat_ult`
- Snapshot ID: `39523`
- Source-Dokument: `7070`
- Kurzinfo: Bashdown aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 4,
  "AbilityChannelTime": 0.3,
  "AbilityCharges": 1,
  "AbilityCooldown": 35,
  "AbilityCooldownBetweenCharge": 8,
  "AbilityPostCastDuration": 0.3,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorSilentCastFailureFeedback",
    "BehaviorChannelled",
    "BehaviorExclusiveUse",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorHoldsAtMaxChannel"
  ],
  "CameraTurnRateMax": 2000,
  "ChannelMoveSpeed": 4.826,
  "CountsAsLightMelee": 1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.1
    },
    "Value": 35
  },
  "DiminishingSlowModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "Slow"
  },
  "ExplodeDelay": 0.5,
  "FireRateModifier": {
    "Class": "Base",
    "Subclass": "SlamFirerateSlow"
  },
  "GroundAuraModifier": {
    "Class": "PunkgoatSigilAura",
    "Height": 100.0,
    "ProvidedByAura": {
      "Class": "Base",
      "Subclass": "SigilModifier"
    },
    "Subclass": "SigilAura"
  },
  "HeavyMeleeDamage": {
    "Scale": {
      "Type": "heavy_melee",
      "Value": 0
    },
    "Value": 0
  },
  "IsDisabled": false,
  "Key": "ability_punkgoat_ult",
  "MeleeDamage": {
    "Scale": {
      "Type": "melee",
      "Value": 0.9
    },
    "Value": 0
  },
  "Name": "Bashdown",
  "PlaceDistanceInFrontOfCaster": 6.2,
  "PullDownDuration": 0.75,
  "PullDownRange": 3,
  "PullToGroundModifier": {
    "Class": "ChargeDragEnemy",
    "ForceDistScale": 11,
    "ForwardOffset": 200,
    "Subclass": "ChargeDragEnemy",
    "VerticalOffset": 0
  },
  "TossDuration": 0.4,
  "TossForce": 350,
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "AbilityCastRange": 2,
      "AbilityCharges": 1
    },
    {
      "AbilityCooldownBetweenCharge": -3,
      "CountsAsHeavyMelee": 1,
      "CountsAsLightMelee": -1,
      "HeavyMeleeDamage": {
        "Scale": {
          "Type": "heavy_melee",
          "Value": 0.5
        },
        "Value": 0.0
      },
      "MeleeDamage": {
        "Scale": {
          "Multiply": true,
          "Type": "melee",
          "Value": 0.0
        },
        "Value": 0.0
      }
    }
  ],
  "WaveEndRadius": 8.0,
  "WaveStartRadius": 0.5,
  "WaveThickness": 1,
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
    "card_name": "Bashdown",
    "hero_key": "hero_punkgoat",
    "hero_name": "Billy",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_punkgoat",
      "hero_name": "Billy",
      "lookup": "bashdown",
      "name": "Bashdown",
      "type": "ability"
    }
  ]
}
````
