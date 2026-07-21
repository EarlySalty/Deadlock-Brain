---
title: "Naptime"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_familiar_ability01"
canonical_name: "Naptime"
snapshot_id: 39790
source_document_id: 7071
payload_hash: "c91589175bbf076f6eaf528cda20189c04667c4a993b430c383261f075e405c7"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.209228+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Naptime

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_familiar_ability01`
- Snapshot ID: `39790`
- Source-Dokument: `7071`
- Kurzinfo: Naptime aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.18
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "distance",
    "Value": 24
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Scale": {
      "Type": "duration",
      "Value": 0.0
    },
    "Type": "cast",
    "Value": 1.9
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 200.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_familiar_ability01_desc",
  "HeroKey": "hero_familiar",
  "HeroName": "Rem",
  "Info1": {
    "Alt": [
      {
        "Key": "Radius",
        "Name": "Radius",
        "Type": "distance",
        "Value": 19.0
      },
      {
        "Key": "SleepDuration",
        "Name": "Sleep Duration",
        "Type": "duration",
        "Value": 4.0
      },
      {
        "Key": "MinSleepTime",
        "Type": "duration",
        "Value": 0.5
      },
      {
        "Key": "DamageResistPctWhileChanneling",
        "Value": 30
      }
    ],
    "DescKey": "ability_familiar_ability01_desc",
    "Main": {
      "Props": [
        {
          "Key": "AwakeDamage",
          "Name": "Wake Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.6
          },
          "Type": "tech_damage",
          "Value": 120
        },
        {
          "Key": "SleepDamageThreshold",
          "Name": "Sleep Damage Threshold",
          "Scale": {
            "Type": "power_increase",
            "Value": 3.1
          },
          "Type": "damage",
          "Value": 100
        },
        {
          "Key": "MoveSpeedAndDashSlowPct",
          "Name": "Move/Dash Slow",
          "Type": "slow",
          "Value": 25
        }
      ]
    }
  },
  "Key": "ability_familiar_ability01",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    },
    "SleepMoveSpeed": {
      "Name": "Sleep Movespeed",
      "Type": "move_speed",
      "Value": 1.5
    }
  },
  "Name": "Naptime",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "Height": {
      "Name": null,
      "Value": 20
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 19.0
  },
  "Slot": "4",
  "Upgrades": [
    {
      "ConsumeStaminaOnWake": 1,
      "DescKey": "ability_familiar_ability01_t1_desc",
      "NoStaminaRegenDuringSleep": 1
    },
    {
      "DescKey": "ability_familiar_ability01_t2_desc",
      "Radius": 3,
      "SleepDuration": 0.75
    },
    {
      "AbilityCooldown": -55,
      "DamageResistPctWhileChanneling": 50,
      "DescKey": "ability_familiar_ability01_t3_desc",
      "UnstoppableWhileChanneling": 1
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
      "hero_key": "hero_familiar",
      "hero_name": "Rem",
      "lookup": "naptime",
      "name": "Naptime",
      "type": "ability"
    }
  ]
}
````
