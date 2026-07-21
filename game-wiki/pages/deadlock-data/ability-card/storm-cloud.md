---
title: "Storm Cloud"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_storm_cloud"
canonical_name: "Storm Cloud"
snapshot_id: 39814
source_document_id: 7071
payload_hash: "d5b73b35696e558a94e0f9e75bb9182cd47ba6c51375fad50a8861d4cff156a9"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.255830+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Storm Cloud

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_storm_cloud`
- Snapshot ID: `39814`
- Source-Dokument: `7071`
- Kurzinfo: Storm Cloud aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 7
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 205.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Cooldown": {
    "ExpandTime": {
      "Name": "Expand time",
      "Type": "cooldown",
      "Value": 3.5
    }
  },
  "DescKey": "citadel_ability_storm_cloud_desc",
  "HeroKey": "hero_gigawatt",
  "HeroName": "Seven",
  "Info1": {
    "Alt": [
      {
        "Key": "InitialRadius",
        "Name": "Initial Radius",
        "Type": "distance",
        "Value": 10
      },
      {
        "Key": "BulletResistOnActive",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_up",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_storm_cloud_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.6
          },
          "Type": "tech_damage",
          "Value": 95
        },
        {
          "Key": "FlightControlEnabled",
          "Name": "Flight Speed",
          "Value": 1.5
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "LightningStrikeDamage",
        "Name": "Strike Damage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Type": "tech_damage",
        "Value": 75.0
      },
      {
        "Key": "LightningStrikeRadius",
        "Name": "Strike Radius",
        "Type": "distance",
        "Value": 7
      }
    ],
    "DescKey": "citadel_ability_storm_cloud_lightning_strike_desc",
    "Main": {
      "Props": []
    }
  },
  "Key": "citadel_ability_storm_cloud",
  "Name": "Storm Cloud",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraDistance": {
      "Name": null,
      "Value": 600
    },
    "CloudHeight": {
      "Name": null,
      "Value": 120
    },
    "DamageInterval": {
      "Name": null,
      "Value": 0.3
    },
    "EndingSoonTime": {
      "Name": null,
      "Value": 2
    },
    "LightningStrikeDelay": {
      "Name": null,
      "Value": 0.25
    },
    "LightningStrikeKnockBackForce": {
      "Name": null,
      "Value": 500
    },
    "LightningStrikes": {
      "Name": null,
      "Value": 1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 30
  },
  "Slot": "4",
  "Upgrades": [
    {
      "BulletResistOnActive": 55,
      "DescKey": "citadel_ability_storm_cloud_t1_desc"
    },
    {
      "AbilityChannelTime": 7,
      "DescKey": "citadel_ability_storm_cloud_t2_desc",
      "InitialRadius": 5,
      "Radius": 10
    },
    {
      "DPS": 65.0,
      "DescKey": "citadel_ability_storm_cloud_t3_desc",
      "FlightControlEnabled": 4
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
      "hero_key": "hero_gigawatt",
      "hero_name": "Seven",
      "lookup": "storm cloud",
      "name": "Storm Cloud",
      "type": "ability"
    }
  ]
}
````
