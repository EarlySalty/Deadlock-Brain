---
title: "Flakshot"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_skyrunner_flakshot"
canonical_name: "Flakshot"
snapshot_id: 39889
source_document_id: 7071
payload_hash: "d3282ffb056a982754f0fe488b7591cd116ed81198198fb46646c2f26f572e35"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.400826+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Flakshot

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_skyrunner_flakshot`
- Snapshot ID: `39889`
- Source-Dokument: `7071`
- Kurzinfo: Flakshot aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_skyrunner_flakshot_desc",
  "HeroKey": "hero_skyrunner",
  "HeroName": "Skyrunner",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_skyrunner_flakshot_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusDamage",
          "Name": "Bonus Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.093
          },
          "Type": "bullet_damage",
          "Value": 3
        },
        {
          "Key": "RicochetChance",
          "Name": "Ricochet Chance",
          "Type": "cast",
          "Value": 50
        },
        {
          "Key": "RicochetDamagePercent",
          "Name": "Ricochet Damage",
          "Type": "bullet_damage",
          "Value": 100
        }
      ]
    }
  },
  "Key": "ability_skyrunner_flakshot",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Flakshot",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DirectionVariance": {
      "Name": null,
      "Value": 0.02
    },
    "MinEffectiveness": {
      "Name": null,
      "Value": -1
    },
    "RicochetAssistRatio": {
      "Name": null,
      "Value": 0.5
    },
    "RicochetRadius": {
      "Name": "Ricochet Range",
      "Value": 20
    }
  },
  "Radius": {
    "Name": "Radius",
    "Value": 5
  },
  "Slot": "3",
  "Upgrades": [
    {
      "RicochetChance": 50
    },
    {
      "BonusDamage": 5
    },
    {
      "RicochetDamagePercent": 50
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
      "hero_key": "hero_skyrunner",
      "hero_name": "Skyrunner",
      "lookup": "flakshot",
      "name": "Flakshot",
      "type": "ability"
    }
  ]
}
````
