---
title: "Kinetic Pulse"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_stomp"
canonical_name: "Kinetic Pulse"
snapshot_id: 39783
source_document_id: 7071
payload_hash: "4f22e646e03f54b62c11d3ecc6ae1b44b4fc4d66ed9fda7f21b2fbee60fee48d"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.196279+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Kinetic Pulse

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_stomp`
- Snapshot ID: `39783`
- Source-Dokument: `7071`
- Kurzinfo: Kinetic Pulse aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.42
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 5
  },
  "DescKey": "citadel_ability_stomp_desc",
  "HeroKey": "hero_dynamo",
  "HeroName": "Dynamo",
  "Info1": {
    "Alt": [
      {
        "Key": "StompRange",
        "Name": "Pulse Range",
        "Type": "distance",
        "Value": 16
      },
      {
        "Key": "StompWidth",
        "Name": "Pulse Width",
        "Type": "distance",
        "Value": 5.5
      }
    ],
    "DescKey": "citadel_ability_stomp_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.55
          },
          "Type": "tech_damage",
          "Value": 115.0
        },
        {
          "Key": "TossDuration",
          "Name": "Duration",
          "StatusEffect": "Displacement",
          "Value": 1
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "BulletResistReduction",
          "Name": "Bullet Resist",
          "Title": "On Hit:",
          "Type": "bullet_armor_down",
          "Value": 0
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hit:",
          "Type": "slow",
          "Value": 0
        },
        {
          "Key": "SlowDuration",
          "Name": "Slow Duration",
          "Title": "On Hit:",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_stomp",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Kinetic Pulse",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ClimbHeight": {
      "Name": null,
      "Value": 1.0
    },
    "DistanceAboveGround": {
      "Name": null,
      "Value": 1.0
    },
    "DropDownRate": {
      "Name": null,
      "Value": 20
    },
    "ImpactInterval": {
      "Name": null,
      "Value": 0.1
    },
    "TechCleaveExpireTime": {
      "Name": null,
      "Value": 0.2
    },
    "TossSpeed": {
      "Name": null,
      "Value": 450
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCharges": 1,
      "DescKey": "citadel_ability_stomp_t1_desc"
    },
    {
      "BulletResistReduction": -15,
      "DescKey": "citadel_ability_stomp_t2_desc",
      "SlowDuration": 4,
      "SlowPercent": 30
    },
    {
      "Damage": 135,
      "DescKey": "citadel_ability_stomp_t3_desc",
      "StompRange": 20
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
      "hero_key": "hero_dynamo",
      "hero_name": "Dynamo",
      "lookup": "kinetic pulse",
      "name": "Kinetic Pulse",
      "type": "ability"
    }
  ]
}
````
