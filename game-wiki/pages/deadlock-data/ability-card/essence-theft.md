---
title: "Essence Theft"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_necro_fear"
canonical_name: "Essence Theft"
snapshot_id: 39861
source_document_id: 7071
payload_hash: "157ba1ae66a6c696498d85998c44a9003f4ffb5bc9543d843cf29f145e0af142"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.347934+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Essence Theft

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_necro_fear`
- Snapshot ID: `39861`
- Source-Dokument: `7071`
- Kurzinfo: Essence Theft aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 4
  },
  "DescKey": "ability_necro_fear_desc",
  "HeroKey": "hero_necro",
  "HeroName": "Graves",
  "Info1": {
    "Alt": [
      {
        "Key": "MaxStolenTargets",
        "Name": "Max Steal Targets",
        "Value": 3
      },
      {
        "Key": "ShootDurationForMax",
        "Name": "Time for Max Damage",
        "Type": "duration",
        "Value": 4
      }
    ],
    "DescKey": "ability_necro_fear_desc",
    "Main": {
      "Props": [
        {
          "Key": "MaxStolenAttackDamage",
          "Name": "Max Weapon Damage Stolen",
          "Scale": {
            "Type": "spirit",
            "Value": 0.25
          },
          "Type": "bullet_damage",
          "Value": 25
        },
        {
          "Key": "MaxStolenSpiritResist",
          "Name": "Max Spirit Resist Stolen",
          "Type": "tech_armor_down",
          "Value": 10
        },
        {
          "Key": "MaxStolenFireRate",
          "Name": "Max Fire Rate Stolen",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_necro_fear",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Essence Theft",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DelayBeforeLoss": {
      "Name": null,
      "Value": 0.5
    },
    "ProgressLossMultiplier": {
      "Name": null,
      "Value": 2.3
    },
    "ProgressLossPerSecond": {
      "Name": null,
      "Value": 1
    },
    "TickInterval": {
      "Name": null,
      "Value": 0.15
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "MaxStolenSpiritResist": 5
    },
    {
      "MaxStolenAttackDamage": 20
    },
    {
      "DescKey": "ability_necro_fear_t3_desc",
      "SkullBuildUp": 0.15,
      "ZombieExplosionBuildUp": 1.0,
      "ZombieMeleeBuildUp": 0.15
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
      "hero_key": "hero_necro",
      "hero_name": "Graves",
      "lookup": "essence theft",
      "name": "Essence Theft",
      "type": "ability"
    }
  ]
}
````
