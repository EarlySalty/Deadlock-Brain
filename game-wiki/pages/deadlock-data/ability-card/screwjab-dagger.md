---
title: "Screwjab Dagger"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_viper_debuffdagger"
canonical_name: "Screwjab Dagger"
snapshot_id: 39923
source_document_id: 7071
payload_hash: "fa6cd4ae72204e1d4381479beb339c0596bd8c1fd278d96fb4baccccb158d3e1"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.466975+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Screwjab Dagger

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_viper_debuffdagger`
- Snapshot ID: `39923`
- Source-Dokument: `7071`
- Kurzinfo: Screwjab Dagger aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 10
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 4.0
  },
  "DescKey": "ability_viper_debuffdagger_desc",
  "Duration": {
    "SlowDuration": {
      "Name": "Slow Duration",
      "Type": "duration",
      "Value": 2
    }
  },
  "HeroKey": "hero_viper",
  "HeroName": "Vyper",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_viper_debuffdagger_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.8
          },
          "Type": "tech_damage",
          "Value": 50
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hit:",
          "Type": "slow",
          "Value": 35
        },
        {
          "Key": "BulletResistReduction",
          "Name": "Bullet Resist",
          "Title": "On Hit:",
          "Type": "tech_armor_down",
          "Value": 0
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "StackDuration",
        "Name": "Stack Duration",
        "Value": 10
      },
      {
        "Key": "MaxStacks",
        "Name": "Max Stacks",
        "Value": 3
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "DamagePerStack",
          "Name": "Damage per Stack",
          "Scale": {
            "Type": "spirit",
            "Value": 0.4
          },
          "Title": "On Stack:",
          "Type": "tech_damage",
          "Value": 25
        },
        {
          "Key": "SlowPercentPerStack",
          "Name": "Move Speed per Stack",
          "Title": "On Stack:",
          "Type": "slow",
          "Value": 15
        },
        {
          "Key": "BulletResistReductionPerStack",
          "Name": "Bullet Resist per Stack",
          "Title": "On Stack:",
          "Type": "tech_armor_down",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_viper_debuffdagger",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Screwjab Dagger",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "BulletResistReduction": -8,
      "BulletResistReductionPerStack": -6,
      "DescKey": "ability_viper_debuffdagger_t2_desc"
    },
    {
      "AbilityCooldownBetweenCharge": -2,
      "CooldownRefundPercent": 55,
      "DescKey": "ability_viper_debuffdagger_t3_desc",
      "MaxStacks": 2
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
      "hero_key": "hero_viper",
      "hero_name": "Vyper",
      "lookup": "screwjab dagger",
      "name": "Screwjab Dagger",
      "type": "ability"
    }
  ]
}
````
