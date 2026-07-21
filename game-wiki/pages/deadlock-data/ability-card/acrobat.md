---
title: "Acrobat"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_swan_acrobat"
canonical_name: "Acrobat"
snapshot_id: 39897
source_document_id: 7071
payload_hash: "837f8a8e6f5e20b170f54ce7695909d6713a41ee857d471ac57b488b5cdd34fd"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.417725+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Acrobat

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_swan_acrobat`
- Snapshot ID: `39897`
- Source-Dokument: `7071`
- Kurzinfo: Acrobat aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 7
  },
  "DescKey": "ability_swan_acrobat_desc",
  "HeroKey": "hero_swan",
  "HeroName": "Swan",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_swan_acrobat_desc",
    "Main": {
      "Props": [
        {
          "Key": "MaxStacks",
          "Name": "Max Stacks",
          "Type": "cast",
          "Value": 4
        },
        {
          "Key": "BurstBonusPerStack",
          "Name": "Weapon Burst Bonus",
          "Title": "On Buff:",
          "Type": "bullet_damage",
          "Value": 1
        },
        {
          "Key": "FireRatePerStack",
          "Name": "Fire Rate per Stack",
          "Title": "On Buff:",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_swan_acrobat",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Acrobat",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityDuration": 3
    },
    {
      "FireRatePerStack": 4
    },
    {
      "MaxStacks": 4
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
      "hero_key": "hero_swan",
      "hero_name": "Swan",
      "lookup": "acrobat",
      "name": "Acrobat",
      "type": "ability"
    }
  ]
}
````
