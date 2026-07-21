---
title: "Essence Bomb"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_blood_bomb"
canonical_name: "Essence Bomb"
snapshot_id: 39807
source_document_id: 7071
payload_hash: "9a06288f1e050cf6c6301570a9c91b93ef19d098b8ee16e2f0dc45a4857bd361"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.242236+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Essence Bomb

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_blood_bomb`
- Snapshot ID: `39807`
- Source-Dokument: `7071`
- Kurzinfo: Essence Bomb aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 14.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_blood_bomb_desc",
  "HeroKey": "hero_ghost",
  "HeroName": "Lady Geist",
  "Info1": {
    "Alt": [
      {
        "Key": "ArmingDuration",
        "Name": "Arming Duration",
        "Type": "duration",
        "Value": 0.65
      }
    ],
    "DescKey": "ability_blood_bomb_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.22
          },
          "Type": "tech_damage",
          "Value": 90
        },
        {
          "Key": "SelfDamagePct",
          "Name": "Health Cost",
          "Value": 30
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "ability_blood_bomb_t3_desc",
    "Main": {
      "Props": [
        {
          "Key": "BloodSpillDuration",
          "Name": "Toxic Mess Duration",
          "Type": "tech_damage",
          "Value": 0
        }
      ]
    },
    "RequiresUpgradeIndex": 2
  },
  "Key": "ability_blood_bomb",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Essence Bomb",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BeepSoundBuildupCount": {
      "Name": null,
      "Value": 4
    },
    "BeepSoundIntervalBias": {
      "Name": null,
      "Value": 0.55
    },
    "BeepSoundMaxFrequency": {
      "Name": null,
      "Value": 0.1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 7
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -5
    },
    {
      "Damage": 50,
      "DescKey": "ability_blood_bomb_t2_desc",
      "Radius": 2
    },
    {
      "BloodSpillDPSPercent": 26,
      "BloodSpillDuration": 6,
      "DescKey": "ability_blood_bomb_t3_desc"
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
      "hero_key": "hero_ghost",
      "hero_name": "Lady Geist",
      "lookup": "essence bomb",
      "name": "Essence Bomb",
      "type": "ability"
    }
  ]
}
````
