---
title: "Sleep Dagger"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_sleep_dagger"
canonical_name: "Sleep Dagger"
snapshot_id: 39823
source_document_id: 7071
payload_hash: "bef24b601eaf5a964fb4b7a47817ed73b5585a02c5892926732d2df46ffaee2a"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.272910+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Sleep Dagger

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_sleep_dagger`
- Snapshot ID: `39823`
- Source-Dokument: `7071`
- Kurzinfo: Sleep Dagger aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_sleep_dagger_desc",
  "HeroKey": "hero_haze",
  "HeroName": "Haze",
  "Info1": {
    "Alt": [
      {
        "Key": "MinimumSleepTime",
        "Name": "Min Sleep Time",
        "Type": "duration",
        "Value": 0.2
      },
      {
        "Key": "SleepWakeUpDelay",
        "Name": "Wake Up Delay",
        "Scale": {
          "Type": "spirit",
          "Value": 0.003
        },
        "Type": "duration",
        "Value": 0.1
      },
      {
        "Key": "SleepMoveSpeed",
        "Name": "Sleep Movespeed",
        "Type": "move_speed",
        "Value": 1.5
      },
      {
        "Key": "RicochetRadius",
        "Name": "Ricochet Range",
        "Value": 0
      }
    ],
    "DescKey": "ability_sleep_dagger_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.8
          },
          "Type": "tech_damage",
          "Value": 65
        },
        {
          "Key": "SleepDuration",
          "Name": "Sleep Duration",
          "Type": "duration",
          "Value": 2.75
        }
      ]
    }
  },
  "Key": "ability_sleep_dagger",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Sleep Dagger",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DoesNotBreakInvis": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "BulletResistReduction": -10.0,
      "BulletResistReductionDuration": 6,
      "DescKey": "ability_sleep_dagger_t1_desc"
    },
    {
      "AbilityCooldown": -18.0,
      "DescKey": "ability_sleep_dagger_t2_desc"
    },
    {
      "DebuffDuration": 3,
      "DescKey": "ability_sleep_dagger_t3_desc",
      "GroundDashReductionPercent": -50,
      "SleepDuration": 1,
      "SlowPercent": 50
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
      "lookup": "sleep dagger",
      "name": "Sleep Dagger",
      "type": "ability"
    }
  ]
}
````
