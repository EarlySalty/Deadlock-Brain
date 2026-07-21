---
title: "Guided Owl"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_guided_arrow"
canonical_name: "Guided Owl"
snapshot_id: 39870
source_document_id: 7071
payload_hash: "4daa41a4aa0816c81bb3fdcb6d39454dbcf16c9cb10828a82dc25ee68b5dfa8c"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.364824+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Guided Owl

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_guided_arrow`
- Snapshot ID: `39870`
- Source-Dokument: `7071`
- Kurzinfo: Guided Owl aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 1.5
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 20
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 125.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_guided_arrow_desc",
  "HeroKey": "hero_orion",
  "HeroName": "Grey Talon",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusTechPowerPerKill",
        "Name": "Spirit Power Per Kill",
        "Type": "tech_damage",
        "Value": 8
      }
    ],
    "DescKey": "ability_guided_arrow_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.93744
          },
          "Type": "tech_damage",
          "Value": 230.0
        },
        {
          "Key": "ExplosionRadius",
          "Name": "Explosion Radius",
          "Type": "distance",
          "Value": 12
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "Type": "duration",
          "Value": 0.75
        }
      ]
    }
  },
  "Key": "ability_guided_arrow",
  "Name": "Guided Owl",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "Damage": 85.0
    },
    {
      "AbilityCooldown": -40.0
    },
    {
      "DescKey": "ability_guided_arrow_t3_desc",
      "LowHealthEnemyThresholdPct": 22
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
      "hero_key": "hero_orion",
      "hero_name": "Grey Talon",
      "lookup": "guided owl",
      "name": "Guided Owl",
      "type": "ability"
    }
  ]
}
````
