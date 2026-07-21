---
title: "Shining Wonder"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_unicorn_dazzlingorb"
canonical_name: "Shining Wonder"
snapshot_id: 39914
source_document_id: 7071
payload_hash: "7469a13ca355289ea7fd699a0ebd5727f63a12e5b46220304523e338c85b6895"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.449645+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Shining Wonder

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_unicorn_dazzlingorb`
- Snapshot ID: `39914`
- Source-Dokument: `7071`
- Kurzinfo: Shining Wonder aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.75
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 9999
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 160
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_unicorn_dazzlingorb_desc",
  "HeroKey": "hero_unicorn",
  "HeroName": "Celeste",
  "Info1": {
    "Alt": [
      {
        "Key": "GroundDashReductionPercent",
        "Name": "Dash Distance",
        "Type": "slow",
        "Value": -25
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 1.5
      },
      {
        "Key": "BounceRadius",
        "Name": "Bounce Range",
        "Type": "distance",
        "Value": 16.5
      },
      {
        "Key": "BounceGrace",
        "Value": 3
      }
    ],
    "DescKey": "ability_unicorn_dazzlingorb_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.8
          },
          "Type": "tech_damage",
          "Value": 150
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 40
        },
        {
          "Key": "MaxBounces",
          "Name": "Bounces",
          "Value": 8
        }
      ]
    }
  },
  "Key": "ability_unicorn_dazzlingorb",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Shining Wonder",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "NextTargetDuration": {
      "Name": null,
      "Value": 4
    },
    "PriorityBounceRadius": {
      "Name": null,
      "Value": 12.5
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "DescKey": "ability_unicorn_dazzlingorb_t1_desc",
      "GroundDashReductionPercent": -15,
      "SlowPercent": 20
    },
    {
      "Damage": 80
    },
    {
      "AbilityCooldown": -30,
      "DescKey": "ability_unicorn_dazzlingorb_t3_desc",
      "MaxBounces": 8
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
      "hero_key": "hero_unicorn",
      "hero_name": "Celeste",
      "lookup": "shining wonder",
      "name": "Shining Wonder",
      "type": "ability"
    }
  ]
}
````
