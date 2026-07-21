---
title: "Static Charge"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_static_charge"
canonical_name: "Static Charge"
snapshot_id: 39812
source_document_id: 7071
payload_hash: "eacd72c3fcd62e18bb432489f84d6d972498eb959618fe1bc2a078324db442b9"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.251791+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Static Charge

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_static_charge`
- Snapshot ID: `39812`
- Source-Dokument: `7071`
- Kurzinfo: Static Charge aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 42.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_static_charge_desc",
  "HeroKey": "hero_gigawatt",
  "HeroName": "Seven",
  "Info1": {
    "Alt": [
      {
        "Key": "ShockRadius",
        "Name": "Radius",
        "Type": "distance",
        "Value": 5
      }
    ],
    "DescKey": "citadel_ability_static_charge_desc",
    "Main": {
      "Props": [
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Type": "duration",
          "Value": 0.9
        },
        {
          "Key": "ShockDelay",
          "Name": "Delay Before Stun",
          "Type": "cast",
          "Value": 3.5
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.792137
          },
          "Type": "tech_damage",
          "Value": 35
        }
      ]
    }
  },
  "Key": "citadel_ability_static_charge",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Static Charge",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -20.0
    },
    {
      "AbilityCastRange": 5,
      "DescKey": "citadel_ability_static_charge_t2_desc",
      "ShockRadius": 7
    },
    {
      "Damage": 160,
      "DescKey": "citadel_ability_static_charge_t3_desc",
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
      "hero_key": "hero_gigawatt",
      "hero_name": "Seven",
      "lookup": "static charge",
      "name": "Static Charge",
      "type": "ability"
    }
  ]
}
````
