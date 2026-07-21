---
title: "Flame Dash"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_flame_dash"
canonical_name: "Flame Dash"
snapshot_id: 39832
source_document_id: 7071
payload_hash: "9a0d27eee7158f1bbde1f56d8006cb91eb9feee31f375c1e5c8a006d2e50a22f"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.289042+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Flame Dash

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_flame_dash`
- Snapshot ID: `39832`
- Source-Dokument: `7071`
- Kurzinfo: Flame Dash aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 38.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 3.0
  },
  "DescKey": "ability_flame_dash_desc",
  "HeroKey": "hero_inferno",
  "HeroName": "Infernus",
  "Info1": {
    "Alt": [
      {
        "Key": "GroundFlameDuration",
        "Name": "Trail Duration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "FlameAuraRadius",
        "Name": "Trail Width",
        "Type": "distance",
        "Value": 4.5
      },
      {
        "Key": "SlowResistance",
        "Name": "Slow Resistance",
        "Type": "move_speed",
        "Value": 50
      }
    ],
    "DescKey": "ability_flame_dash_desc",
    "Main": {
      "Props": [
        {
          "Key": "SpeedBurstSpeed",
          "Name": "Max Dash Speed",
          "Type": "move_speed",
          "Value": 20
        },
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.7
          },
          "Type": "tech_damage",
          "Value": 30
        },
        {
          "Key": "FireRateSlow",
          "Name": "Fire Rate",
          "Title": "On Hit:",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_flame_dash",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 18
    },
    "DashAirSpeed": {
      "Name": null,
      "Type": "move_speed",
      "Value": 8
    },
    "DashSpeed": {
      "Name": "Dash Speed",
      "Type": "move_speed",
      "Value": 12
    }
  },
  "Name": "Flame Dash",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AuraLingerDuration": {
      "Name": null,
      "Value": 1.0
    },
    "FlameDashJumpBonus": {
      "Name": null,
      "Value": 50
    },
    "GroundAuraSpacing": {
      "Name": null,
      "Value": 1
    },
    "SideMoveSpeedReduction": {
      "Name": null,
      "Value": -65
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -12.0
    },
    {
      "DPS": 20.0,
      "DescKey": "ability_flame_dash_t2_desc",
      "GroundFlameDuration": 1
    },
    {
      "AbilityCharges": 2,
      "AbilityCooldownBetweenCharge": 14,
      "DescKey": "ability_flame_dash_t3_desc"
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
      "hero_key": "hero_inferno",
      "hero_name": "Infernus",
      "lookup": "flame dash",
      "name": "Flame Dash",
      "type": "ability"
    }
  ]
}
````
