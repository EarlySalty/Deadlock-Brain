---
title: "Rend"
entity_type: "ability"
source: "deadlock_data"
external_id: "drifter_blood_blast"
canonical_name: "Rend"
snapshot_id: 39671
source_document_id: 7070
payload_hash: "00c1c6ec6e9302dba43a439366ec12a78b433846b9e76858319da05a9f4ea953"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.910565+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Rend

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `drifter_blood_blast`
- Snapshot ID: `39671`
- Source-Dokument: `7070`
- Kurzinfo: Rend aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.4,
  "AbilityCastRange": 16,
  "AbilityCooldown": 16.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.4,
  "AbilityUnitTargetLimit": 30,
  "AirSpeedMax": 70,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorNoTarget",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.8
    },
    "Value": 40.0
  },
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "melee",
      "Value": 1.2
    },
    "Value": 0.0
  },
  "DamageHeavyMelee": {
    "Scale": {
      "Type": "heavy_melee",
      "Value": 0
    },
    "Value": 0
  },
  "DebuffModifier": {
    "Class": "CitadelSilenced",
    "Subclass": "Debuff"
  },
  "ExtraSweepConeAngle": 60,
  "ExtraSweepOffsetBehindCaster": 80,
  "ExtraSweepRange": 3,
  "FallSpeedMax": 1,
  "IsDisabled": false,
  "Key": "drifter_blood_blast",
  "Name": "Rend",
  "RangeForBonusDamage": 8,
  "TargetModifier": {
    "Class": "DrifterRendBulletLifesteal",
    "Subclass": "DrifterRendBulletLifesteal"
  },
  "TargetingConeAngle": 50,
  "Upgrades": [
    {
      "BonusDamage": 40
    },
    {
      "AbilityCooldown": -8.0
    },
    {
      "Damage": {
        "Scale": {
          "Multiply": true,
          "Type": "melee",
          "Value": 0.0
        },
        "Value": 0
      },
      "DamageHeavyMelee": {
        "Scale": {
          "Type": "heavy_melee",
          "Value": 0.55
        },
        "Value": 0
      },
      "DebuffDuration": 2.3,
      "UseHeavyMelee": 1
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
    "card_name": "Rend",
    "hero_key": "hero_drifter",
    "hero_name": "Drifter",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_drifter",
      "hero_name": "Drifter",
      "lookup": "rend",
      "name": "Rend",
      "type": "ability"
    }
  ]
}
````
