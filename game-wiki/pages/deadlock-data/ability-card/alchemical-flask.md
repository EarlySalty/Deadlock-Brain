---
title: "Alchemical Flask"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_warden_crowd_control"
canonical_name: "Alchemical Flask"
snapshot_id: 39931
source_document_id: 7071
payload_hash: "5bd06508781ba8d1dea2a16cad3bfe67b83560db4725dbe173d585c8b0a237a1"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.482986+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Alchemical Flask

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_warden_crowd_control`
- Snapshot ID: `39931`
- Source-Dokument: `7071`
- Kurzinfo: Alchemical Flask aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 12.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_warden_crowd_control_desc",
  "HeroKey": "hero_warden",
  "HeroName": "Warden",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 3
      },
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 7
      },
      {
        "Key": "StaminaReduction",
        "Name": "Stamina Reduction",
        "Value": 0
      }
    ],
    "DescKey": "ability_warden_crowd_control_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.63
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "MoveSpeedSlowPct",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 20
        },
        {
          "Key": "WeaponPowerDebuff",
          "Name": "Weapon Damage",
          "Type": "bullet_damage",
          "Value": -25
        }
      ]
    }
  },
  "Key": "ability_warden_crowd_control",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Alchemical Flask",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ForwardVelocity": {
      "Name": null,
      "Value": 800
    },
    "ProjectileLifetime": {
      "Name": null,
      "Value": 60
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 5.5
  },
  "Slot": "1",
  "Upgrades": [
    {
      "DescKey": "ability_warden_crowd_control_t1_desc",
      "StaminaReduction": 1
    },
    {
      "Damage": 35,
      "DescKey": "ability_warden_crowd_control_t2_desc",
      "WeaponPowerDebuff": -25
    },
    {
      "AbilityCooldown": -7,
      "DescKey": "ability_warden_crowd_control_t3_desc",
      "FireRateSlow": 30,
      "Radius": 2
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
      "hero_key": "hero_warden",
      "hero_name": "Warden",
      "lookup": "alchemical flask",
      "name": "Alchemical Flask",
      "type": "ability"
    }
  ]
}
````
