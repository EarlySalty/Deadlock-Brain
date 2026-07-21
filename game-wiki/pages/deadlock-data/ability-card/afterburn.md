---
title: "Afterburn"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_afterburn"
canonical_name: "Afterburn"
snapshot_id: 39833
source_document_id: 7071
payload_hash: "f1283b2ac32ec372c8aba104078c4833f0aed2fc00dbd86a5e0caa397c741a0a"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.290826+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Afterburn

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_afterburn`
- Snapshot ID: `39833`
- Source-Dokument: `7071`
- Kurzinfo: Afterburn aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_afterburn_desc",
  "Duration": {
    "BuildUpDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 17
    }
  },
  "HeroKey": "hero_inferno",
  "HeroName": "Infernus",
  "Info1": {
    "Alt": [
      {
        "Key": "BuildUpBulletPercentPerHit",
        "Name": "Buildup Per Bullet",
        "Type": "cast",
        "Value": 8.1
      },
      {
        "Key": "CritBuildup",
        "Name": "Buildup Per Headshot",
        "Type": "cast",
        "Value": 15.4
      },
      {
        "Key": "RefillDuration",
        "Type": "duration",
        "Value": 0.5
      },
      {
        "Key": "RefillDurationCrit",
        "Name": "Extend Per Headshot",
        "Type": "duration",
        "Value": 1.0
      }
    ],
    "DescKey": "ability_afterburn_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.66
          },
          "Title": "Burn Effect:",
          "Type": "tech_damage",
          "Value": 14.0
        },
        {
          "Key": "BurnDurationBase",
          "Name": "Unknown(BurnDurationBase)",
          "Title": "Burn Effect:",
          "Type": "duration",
          "Value": 3
        },
        {
          "Key": "BurnDuration",
          "Name": "Burn Duration",
          "Title": "Burn Effect:",
          "Type": "duration",
          "Value": 3
        },
        {
          "Key": "OutgoingTechDamagePercent",
          "Name": "Spirit Damage",
          "Title": "Burn Effect:",
          "Type": "tech_damage",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_afterburn",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Afterburn",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DPS": 16
    },
    {
      "DescKey": "ability_afterburn_t2_desc",
      "OutgoingTechDamagePercent": -35
    },
    {
      "BurnDuration": 3,
      "DescKey": "ability_afterburn_t3_desc"
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
      "lookup": "afterburn",
      "name": "Afterburn",
      "type": "ability"
    }
  ]
}
````
