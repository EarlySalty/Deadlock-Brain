---
title: "Flog"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_lash_flog"
canonical_name: "Flog"
snapshot_id: 39845
source_document_id: 7071
payload_hash: "8dd1a42d2b9df3a058c4dbadbc4922d39980032e42316d601194028b40d41bfb"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.316539+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Flog

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_lash_flog`
- Snapshot ID: `39845`
- Source-Dokument: `7071`
- Kurzinfo: Flog aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 20
  },
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
  "DescKey": "ability_lash_flog_desc",
  "HeroKey": "hero_lash",
  "HeroName": "Lash",
  "Info1": {
    "Alt": [
      {
        "Key": "HealPctVsNonHeroes",
        "Name": "Heal vs non-heroes",
        "Type": "healing",
        "Value": 16
      },
      {
        "Key": "EnemySlowPct",
        "Name": "Enemy Move Speed",
        "Type": "slow",
        "Value": 0
      },
      {
        "Key": "EnemySlowDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "FireRateSlow",
        "Name": "Fire Rate",
        "Type": "fire_rate",
        "Value": 0
      }
    ],
    "DescKey": "ability_lash_flog_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.85
          },
          "Type": "tech_damage",
          "Value": 65
        },
        {
          "Key": "HealPctVsHeroes",
          "Name": "Heal vs heroes",
          "Type": "healing",
          "Value": 50
        },
        {
          "Key": "TargetingConeAngle",
          "Name": "Attack Angle",
          "Type": "distance",
          "Value": 38
        }
      ]
    }
  },
  "Key": "ability_lash_flog",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Flog",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 30
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DescKey": "ability_lash_flog_t1_desc",
      "EnemySlowDuration": 3,
      "EnemySlowPct": 35
    },
    {
      "AbilityCooldown": -16.0,
      "DescKey": "ability_lash_flog_t2_desc",
      "FireRateSlow": 30
    },
    {
      "Damage": 80,
      "DescKey": "ability_lash_flog_t3_desc",
      "HealPctVsHeroes": 20,
      "HealPctVsNonHeroes": 6,
      "TargetingConeAngle": 40
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
      "hero_key": "hero_lash",
      "hero_name": "Lash",
      "lookup": "flog",
      "name": "Flog",
      "type": "ability"
    }
  ]
}
````
