---
title: "Unknown(ability_haunt)"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_haunt"
canonical_name: "Unknown(ability_haunt)"
snapshot_id: 39921
source_document_id: 7071
payload_hash: "fef8fea87652fc76b03dbbed9e3425504ae16b5f5657556e85e6ac9faff61b21"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.463291+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Unknown(ability_haunt)

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_haunt`
- Snapshot ID: `39921`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_haunt) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_vandal",
  "HeroName": "Vandal",
  "Info1": {
    "Alt": [
      {
        "Key": "BuildUpBulletPercentPerHit",
        "Name": "Buildup Per Bullet",
        "Value": 8.33
      },
      {
        "Key": "CritBuildup",
        "Name": "Buildup Per Headshot",
        "Value": 16
      },
      {
        "Key": "BuildUpDuration",
        "Type": "duration",
        "Value": 0.1
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
            "Value": 0.465
          },
          "Type": "tech_damage",
          "Value": 15
        },
        {
          "Key": "BurnDuration",
          "Name": "Burn Duration",
          "Type": "duration",
          "Value": 0.3
        }
      ]
    }
  },
  "Key": "ability_haunt",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_haunt)",
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
      "AfterburnSpiritDamageReduction": -30
    },
    {
      "BurnDuration": 1
    },
    {
      "DPS": 30
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
  }
}
````
