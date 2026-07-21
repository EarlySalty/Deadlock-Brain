---
title: "Quantum Entanglement"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_void_sphere"
canonical_name: "Quantum Entanglement"
snapshot_id: 39784
source_document_id: 7071
payload_hash: "5100d21ac6cd51f32598301575cfeb5449882459d36710ad4843b194e484eb75"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.198150+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Quantum Entanglement

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_void_sphere`
- Snapshot ID: `39784`
- Source-Dokument: `7071`
- Kurzinfo: Quantum Entanglement aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 10
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 20
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 1.4
  },
  "DescKey": "citadel_ability_void_sphere_desc",
  "HeroKey": "hero_dynamo",
  "HeroName": "Dynamo",
  "Info1": {
    "Alt": [
      {
        "Key": "StaminaRestore",
        "Name": "Stamina Restored",
        "Value": 1
      },
      {
        "Key": "AllyDistance",
        "Name": "Ally Distance",
        "Type": "distance",
        "Value": 13
      }
    ],
    "DescKey": "citadel_ability_void_sphere_desc",
    "Main": {
      "Props": [
        {
          "Key": "AbilityCastRange",
          "Name": "Cast Range",
          "Type": "range",
          "Value": 10
        },
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 1.4
        }
      ]
    }
  },
  "Key": "citadel_ability_void_sphere",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Quantum Entanglement",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TrailInterval": {
      "Name": null,
      "Value": 0.01
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCastRange": 6
    },
    {
      "AbilityCooldown": -6
    },
    {
      "ChargeReplenish": 1,
      "DescKey": "citadel_ability_void_sphere_t3_desc",
      "ReduceDebuffs": 50
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
      "hero_key": "hero_dynamo",
      "hero_name": "Dynamo",
      "lookup": "quantum entanglement",
      "name": "Quantum Entanglement",
      "type": "ability"
    }
  ]
}
````
