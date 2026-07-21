---
title: "Soul Exchange"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_health_swap"
canonical_name: "Soul Exchange"
snapshot_id: 39810
source_document_id: 7071
payload_hash: "d610d0155ba65e45e6e0202eac3c2f1e4fbb334a7955bcdd9d85edf625f21845"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.247917+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Soul Exchange

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_health_swap`
- Snapshot ID: `39810`
- Source-Dokument: `7071`
- Kurzinfo: Soul Exchange aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 5.5
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 220.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 0.25
  },
  "DescKey": "ability_health_swap_desc",
  "HeroKey": "hero_ghost",
  "HeroName": "Lady Geist",
  "Info1": {
    "Alt": [
      {
        "Key": "SelfBuffDuration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "SilenceDuration",
        "Name": "Silence Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "SilenceRadius",
        "Name": "Silence Radius",
        "Type": "distance",
        "Value": 0
      }
    ],
    "DescKey": "ability_health_swap_desc",
    "Main": {
      "Props": [
        {
          "Key": "EnemyMinHealthPct",
          "Name": "Enemy Min Health",
          "Type": "health",
          "Value": 30
        },
        {
          "Key": "MinHealthTakenPct",
          "Name": "Min Health Received",
          "Type": "health",
          "Value": 30
        }
      ]
    }
  },
  "Key": "ability_health_swap",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 2
    }
  },
  "Name": "Soul Exchange",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "EnemySlowPct": {
      "Name": "Enemy Move Speed",
      "Value": 70
    },
    "InitialUpSpeed": {
      "Name": null,
      "Value": 150
    },
    "MinDiffToCast": {
      "Name": null,
      "Value": 0.1
    },
    "PostCastHoldTime": {
      "Name": null,
      "Value": 0.2
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -50.0
    },
    {
      "DescKey": "ability_health_swap_t2_desc",
      "SilenceDuration": 3,
      "SilenceRadius": 25
    },
    {
      "BonusFireRate": 40,
      "BonusSpirit": 60,
      "DescKey": "ability_health_swap_t3_desc",
      "SelfBuffDuration": 8,
      "TechResist": 50
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
      "lookup": "soul exchange",
      "name": "Soul Exchange",
      "type": "ability"
    }
  ]
}
````
