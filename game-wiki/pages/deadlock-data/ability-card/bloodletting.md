---
title: "Bloodletting"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_shiv_defer_damage"
canonical_name: "Bloodletting"
snapshot_id: 39885
source_document_id: 7071
payload_hash: "59ee33ce0a960674b747834ef3e8547f823e498c05c0d9f55ca5a3081f2cd74a"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.393677+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Bloodletting

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shiv_defer_damage`
- Snapshot ID: `39885`
- Source-Dokument: `7071`
- Kurzinfo: Bloodletting aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.25
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 20.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_shiv_defer_damage_desc",
  "HeroKey": "hero_shiv",
  "HeroName": "Shiv",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_shiv_defer_damage_desc",
    "Main": {
      "Props": [
        {
          "Key": "DamagePctDeferred",
          "Name": "Incoming Damage Deferred",
          "Type": "damage",
          "Value": 25
        },
        {
          "Key": "DeferredDamageDuration",
          "Name": "Deferred Damage Duration",
          "Type": "duration",
          "Value": 6
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "DeferClearPct",
          "Name": "Deferred Damage Cleared",
          "Title": "On Activate",
          "Type": "healing",
          "Value": 35
        }
      ]
    }
  },
  "Info3": {
    "Alt": [],
    "DescKey": "citadel_ability_shiv_defer_damage_max_rage_desc",
    "Main": {
      "Props": [
        {
          "Key": "DamagePctDeferredMaxRage",
          "Name": "Incoming Damage Deferred",
          "Type": "damage",
          "Value": 15
        }
      ]
    }
  },
  "Key": "citadel_ability_shiv_defer_damage",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Bloodletting",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "DeferClearPct": 35
    },
    {
      "DamagePctDeferred": 15
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
      "hero_key": "hero_shiv",
      "hero_name": "Shiv",
      "lookup": "bloodletting",
      "name": "Bloodletting",
      "type": "ability"
    }
  ]
}
````
