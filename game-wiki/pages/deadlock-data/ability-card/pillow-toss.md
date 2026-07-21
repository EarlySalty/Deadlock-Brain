---
title: "Pillow Toss"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_familiar_ability02"
canonical_name: "Pillow Toss"
snapshot_id: 39787
source_document_id: 7071
payload_hash: "daf5e34b951570fcaa9443c1704c9180a18386a2e6b7a3f54414f84a88b71116"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.203374+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Pillow Toss

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_familiar_ability02`
- Snapshot ID: `39787`
- Source-Dokument: `7071`
- Kurzinfo: Pillow Toss aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.3
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 40
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 25.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 8
  },
  "DescKey": "ability_familiar_ability02_desc",
  "HeroKey": "hero_familiar",
  "HeroName": "Rem",
  "Info1": {
    "Alt": [
      {
        "Key": "EffectDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 3.0
      },
      {
        "Key": "CDReduceOnPillowHit",
        "Value": 5
      }
    ],
    "DescKey": "ability_familiar_ability02_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.6
          },
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "FadingSlowPercent",
          "Name": "Fading Move Speed",
          "Type": "slow",
          "Value": 45
        },
        {
          "Key": "TossDuration",
          "Name": "Duration",
          "StatusEffect": "Displacement",
          "Type": "duration",
          "Value": 0.4
        }
      ]
    }
  },
  "Key": "ability_familiar_ability02",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Pillow Toss",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "OrbsToFire": {
      "Name": null,
      "Value": 1
    },
    "TossForce": {
      "Name": null,
      "Value": 300
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 5
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -7
    },
    {
      "DescKey": "ability_familiar_ability02_t2_desc",
      "FireRateSlow": 35,
      "Radius": 2
    },
    {
      "AbilityCharges": 1,
      "Damage": 100,
      "DescKey": "ability_familiar_ability02_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_familiar",
      "hero_name": "Rem",
      "lookup": "pillow toss",
      "name": "Pillow Toss",
      "type": "ability"
    }
  ]
}
````
