---
title: "Life Drain"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_life_drain"
canonical_name: "Life Drain"
snapshot_id: 39808
source_document_id: 7071
payload_hash: "85a5ea40e828a019c969a50fd60102fb62a1d81c33812dd2b3810fbc61dcecdc"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.244084+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Life Drain

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_life_drain`
- Snapshot ID: `39808`
- Source-Dokument: `7071`
- Kurzinfo: Life Drain aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Type": "distance",
    "Value": 18
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 34.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 2.5
  },
  "Debuff": {
    "MoveSpeedReduction": {
      "Name": null,
      "Type": "slow",
      "Value": 40
    }
  },
  "DescKey": "ability_life_drain_desc",
  "HeroKey": "hero_ghost",
  "HeroName": "Lady Geist",
  "Info1": {
    "Alt": [
      {
        "Key": "LifeDrainHealthMult",
        "Name": "Damage to Heal",
        "Type": "healing",
        "Value": 75
      },
      {
        "Key": "MaxRange",
        "Name": "Max Tether Range",
        "Type": "distance",
        "Value": 28
      }
    ],
    "DescKey": "ability_life_drain_desc",
    "Main": {
      "Props": [
        {
          "Key": "LifeDrainPerSecond",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.43
          },
          "Type": "tech_damage",
          "Value": 32
        }
      ]
    }
  },
  "Key": "ability_life_drain",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Life Drain",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 10
    },
    "TickRate": {
      "Name": null,
      "Value": 0.1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "LifeDrainPerSecond": 18
    },
    {
      "AbilityDuration": 2.5
    },
    {
      "AbilityCharges": 3,
      "AbilityCooldownBetweenCharge": 0.1,
      "DescKey": "ability_life_drain_t3_desc",
      "LifeDrainPerSecond": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.4
        },
        "Value": 0
      }
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
      "lookup": "life drain",
      "name": "Life Drain",
      "type": "ability"
    }
  ]
}
````
