---
title: "Concussive Combustion"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_fire_bomb"
canonical_name: "Concussive Combustion"
snapshot_id: 39834
source_document_id: 7071
payload_hash: "ff64ff7c0c1d3b3425c7a1d3606d4182a527af30c6d82cffdc4a39c9b683f8de"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.292466+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Concussive Combustion

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_fire_bomb`
- Snapshot ID: `39834`
- Source-Dokument: `7071`
- Kurzinfo: Concussive Combustion aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 190.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_fire_bomb_desc",
  "HeroKey": "hero_inferno",
  "HeroName": "Infernus",
  "Info1": {
    "Alt": [
      {
        "Key": "Radius",
        "Name": "Radius",
        "Type": "distance",
        "Value": 12
      }
    ],
    "DescKey": "ability_fire_bomb_desc",
    "Main": {
      "Props": [
        {
          "Key": "ExplodeDelay",
          "Name": "Explosion Delay",
          "Type": "duration",
          "Value": 3.25
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.974938
          },
          "Type": "tech_damage",
          "Value": 125
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Title": "On Hit:",
          "Type": "duration",
          "Value": 1.25
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "FireRatePerHero",
          "Name": "Unknown(FireRatePerHero)",
          "Title": "On Hero Hit:",
          "Type": "fire_rate",
          "Value": 0
        },
        {
          "Key": "BuffDuration",
          "Name": "Buff Duration",
          "Title": "On Hero Hit:",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_fire_bomb",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Concussive Combustion",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 12
  },
  "Slot": "4",
  "Upgrades": [
    {
      "Damage": 100
    },
    {
      "AbilityCooldown": -65.0,
      "DescKey": "ability_fire_bomb_t2_desc",
      "LifeStealPercentOnHit": 100
    },
    {
      "DescKey": "ability_fire_bomb_t3_desc",
      "Radius": 10,
      "StunDuration": 0.9
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
      "lookup": "concussive combustion",
      "name": "Concussive Combustion",
      "type": "ability"
    }
  ]
}
````
