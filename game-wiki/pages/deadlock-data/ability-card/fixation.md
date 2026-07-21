---
title: "Fixation"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_stacking_damage"
canonical_name: "Fixation"
snapshot_id: 39825
source_document_id: 7071
payload_hash: "c11fd00fef1e5a6e2d5348c9af0079902ff06eec5c3378f1c1dcf9d535a3ab30"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.276717+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Fixation

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_stacking_damage`
- Snapshot ID: `39825`
- Source-Dokument: `7071`
- Kurzinfo: Fixation aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6
  },
  "DescKey": "ability_stacking_damage_desc",
  "HeroKey": "hero_haze",
  "HeroName": "Haze",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_stacking_damage_desc",
    "Main": {
      "Props": [
        {
          "Key": "DamageBonusFixedPerStack",
          "Name": "Weapon Damage",
          "Type": "bullet_damage",
          "Value": 0.2
        },
        {
          "Key": "MaxStacks",
          "Name": "Max Stacks",
          "Value": 40
        },
        {
          "Key": "ProcDamage",
          "Name": "Spirit Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Type": "tech_damage",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_stacking_damage",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Fixation",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DescKey": "ability_stacking_damage_t1_desc",
      "ProcDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.8
        },
        "Value": 40
      },
      "ProcDamageStackCount": 20,
      "SlowDuration": 2,
      "SlowPercent": 15
    },
    {
      "AbilityDuration": 5,
      "DescKey": "ability_stacking_damage_t2_desc",
      "MaxStacks": 40
    },
    {
      "DamageBonusFixedPerStack": 0.14
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
      "hero_key": "hero_haze",
      "hero_name": "Haze",
      "lookup": "fixation",
      "name": "Fixation",
      "type": "ability"
    }
  ]
}
````
