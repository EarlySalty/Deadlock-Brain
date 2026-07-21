---
title: "Powder Keg"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_explosive_barrel"
canonical_name: "Powder Keg"
snapshot_id: 39426
source_document_id: 7070
payload_hash: "16fcd582436d759ef60b2df7e3b17130585df69ef75950cefc3c45144d7f6c61"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.291324+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Powder Keg

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_explosive_barrel`
- Snapshot ID: `39426`
- Source-Dokument: `7070`
- Kurzinfo: Powder Keg aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.125,
  "AbilityCharges": 2,
  "AbilityCooldown": 28.0,
  "AbilityCooldownBetweenCharge": 7.5,
  "AbilityUnitTargetLimit": 1,
  "ArmTime": 0.1,
  "BarrelDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.2
    },
    "Value": 80
  },
  "BarrelHeavyMeleeForceForward": 1800,
  "BarrelHeavyMeleeForceUp": 300,
  "BarrelLifetime": 8,
  "BarrelLightMeleeForceForward": 1400,
  "BarrelLightMeleeForceUp": 300,
  "BarrelPitchMax": 90,
  "BarrelPitchMin": 2,
  "BarrelRollSpeedMoveAir": 10,
  "BarrelRollSpeedMoveMin": 20,
  "BarrelScale": 1.3,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorIgnoreSelectionMashProtection",
    "BehaviorDontInterruptSlideOnCast",
    "BehaviorAllowAltCast",
    "BehaviorCastImmediateOnOtherAbility"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_explosive_barrel",
  "MinTimeBeforeDestroy": 0.1,
  "Name": "Powder Keg",
  "Radius": 6,
  "TossDuration": 0.4,
  "TossSpeed": 3.556,
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "AbilityCharges": 1
    },
    {
      "AbilityCooldownBetweenCharge": -5,
      "BarrelDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 80
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
    "card_name": "Powder Keg",
    "hero_key": "hero_astro",
    "hero_name": "Holliday",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_astro",
      "hero_name": "Holliday",
      "lookup": "powder keg",
      "name": "Powder Keg",
      "type": "ability"
    }
  ]
}
````
