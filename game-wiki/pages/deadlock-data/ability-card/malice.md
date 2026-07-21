---
title: "Malice"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_blood_shards"
canonical_name: "Malice"
snapshot_id: 39809
source_document_id: 7071
payload_hash: "5a47dc55404d6e99177ddaea5ec118821032840c20d53f553aed5b912813a029"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.245779+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Malice

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_blood_shards`
- Snapshot ID: `39809`
- Source-Dokument: `7071`
- Kurzinfo: Malice aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.12
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 6
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_blood_shards_desc",
  "HeroKey": "hero_ghost",
  "HeroName": "Lady Geist",
  "Info1": {
    "Alt": [
      {
        "Key": "NumBloodShards",
        "Name": "Blood Shards",
        "Value": 3
      },
      {
        "Key": "MaxStacks",
        "Name": "Max Stacks",
        "Value": 5
      }
    ],
    "DescKey": "ability_blood_shards_desc",
    "Main": {
      "Props": [
        {
          "Key": "HealthToDamage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.558
          },
          "Type": "tech_damage",
          "Value": 23.0
        },
        {
          "Key": "SelfDamagePct",
          "Name": "Health Cost",
          "Type": "tech_damage",
          "Value": 9
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 9
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 4
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "VulnerabilityPerStack",
          "Name": "Damage Amplification",
          "Title": "Effect Per Stack:",
          "Type": "damage",
          "Value": 8
        },
        {
          "Key": "MoveSpeedPenaltyPerStack",
          "Name": "Movement Slow",
          "Title": "Effect Per Stack:",
          "Type": "slow",
          "Value": 15
        }
      ]
    }
  },
  "Key": "ability_blood_shards",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Malice",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.3
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "SpreadAngleDegrees": {
      "Name": null,
      "Value": 6
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -3
    },
    {
      "DescKey": "ability_blood_shards_t2_desc",
      "HealthToDamage": 25.2,
      "NumBloodShards": 4,
      "SpreadAngleDegrees": 22
    },
    {
      "DescKey": "ability_blood_shards_t3_desc",
      "VulnerabilityPerStack": 7
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
      "hero_key": "hero_ghost",
      "hero_name": "Lady Geist",
      "lookup": "malice",
      "name": "Malice",
      "type": "ability"
    }
  ]
}
````
