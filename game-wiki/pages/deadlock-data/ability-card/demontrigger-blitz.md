---
title: "Demontrigger Blitz"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_gunslinger_salvo"
canonical_name: "Demontrigger Blitz"
snapshot_id: 39890
source_document_id: 7071
payload_hash: "6c5c7afabf298c4039c0660caed4f8b0cbf4444860ca53f7dfbd6b1ba73d6bea"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.402686+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Demontrigger Blitz

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_gunslinger_salvo`
- Snapshot ID: `39890`
- Source-Dokument: `7071`
- Kurzinfo: Demontrigger Blitz aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 60
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 90
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_gunslinger_salvo_desc",
  "HeroKey": "hero_skyrunner",
  "HeroName": "Skyrunner",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityCastDelay",
        "Name": "Cast Delay",
        "Type": "cast",
        "Value": 1
      },
      {
        "Key": "AbilityChannelTime",
        "Name": "Channel Duration",
        "Type": "cast",
        "Value": 1
      }
    ],
    "DescKey": "ability_gunslinger_salvo_desc",
    "Main": {
      "Props": [
        {
          "Key": "TotalShots",
          "Name": "Total Shots Fired",
          "Type": "bullet_damage",
          "Value": 4
        },
        {
          "Key": "ProcDamagePercentage",
          "Name": "Bonus Bullet Damage",
          "Type": "tech_damage",
          "Value": 220
        }
      ]
    }
  },
  "Key": "ability_gunslinger_salvo",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 3.8
    }
  },
  "Name": "Demontrigger Blitz",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "Damage": {
      "Name": "Damage",
      "Value": null
    },
    "OverrideBulletRadius": {
      "Name": null,
      "Value": 0.3
    },
    "ProcChance": {
      "Name": "Proc Chance",
      "Value": 100
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -20
    },
    {
      "DebuffDuration": 6
    },
    {
      "TotalShots": 2
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
      "hero_key": "hero_gunslinger",
      "hero_name": "Gunslinger",
      "lookup": "demontrigger blitz",
      "name": "Demontrigger Blitz",
      "type": "ability"
    }
  ]
}
````
