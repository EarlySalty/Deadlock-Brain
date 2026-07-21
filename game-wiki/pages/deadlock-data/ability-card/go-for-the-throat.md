---
title: "Go For The Throat"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_werewolf_frenzy"
canonical_name: "Go For The Throat"
snapshot_id: 39939
source_document_id: 7071
payload_hash: "546167dde00a1c206890f51cf4360270fec356d257930bb3a9fda7998c23172a"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.498+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Go For The Throat

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_frenzy`
- Snapshot ID: `39939`
- Source-Dokument: `7071`
- Kurzinfo: Go For The Throat aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 7.5
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 6.5
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_werewolf_frenzy_desc",
  "HeroKey": "hero_werewolf_transformed",
  "HeroName": "Silver (Transformed)",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_werewolf_frenzy_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "melee",
            "Value": 1.5
          },
          "Type": "melee_damage",
          "Value": 0.0
        },
        {
          "Key": "MissingHealthDamagePercentage",
          "Name": "Missing Health Damage",
          "Type": "melee_damage",
          "Value": 6
        },
        {
          "Key": "LifeStealPercentOnHit",
          "Name": "Unknown(LifeStealPercentOnHit)",
          "Type": "healing",
          "Value": 40
        }
      ]
    }
  },
  "Key": "ability_werewolf_frenzy",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Go For The Throat",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.5
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 16
    }
  },
  "Range": {
    "TargetingConeAngle": {
      "Name": "Attack Angle",
      "Type": "distance",
      "Value": 40
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "Damage": 30
    },
    {
      "LifeStealPercentOnHit": 25
    },
    {
      "MissingHealthDamagePercentage": 4
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
      "hero_key": "hero_werewolf_transformed",
      "hero_name": "Silver (Transformed)",
      "lookup": "go for the throat",
      "name": "Go For The Throat",
      "type": "ability"
    }
  ]
}
````
