---
title: "Flight"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_hornet_leap"
canonical_name: "Flight"
snapshot_id: 39828
source_document_id: 7071
payload_hash: "b74cb95c20694d42601f7bbccdc4f3f8b661b5295640eb01495ef1189088c248"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.281817+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Flight

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hornet_leap`
- Snapshot ID: `39828`
- Source-Dokument: `7071`
- Kurzinfo: Flight aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 42.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 13
  },
  "DescKey": "citadel_ability_hornet_leap_desc",
  "HeroKey": "hero_hornet",
  "HeroName": "Vindicta",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_hornet_leap_desc",
    "Main": {
      "Props": [
        {
          "Key": "MagicDamagePerBullet",
          "Name": "Spirit Damage Per Bullet",
          "Scale": {
            "Type": "spirit",
            "Value": 0.18
          },
          "Type": "tech_damage",
          "Value": 10
        },
        {
          "Key": "FlyingItemCastRange",
          "Name": "Item Range",
          "Type": "distance",
          "Value": 50
        }
      ]
    }
  },
  "Key": "citadel_ability_hornet_leap",
  "Name": "Flight",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirSideMoveSpeedPercentage": {
      "Name": null,
      "Value": -35
    },
    "JumpVelocity": {
      "Name": "Jump Velocity",
      "Value": 1000
    },
    "MaxFlyHeight": {
      "Name": null,
      "Value": 1720
    },
    "MinVelocityZ": {
      "Name": null,
      "Value": -20.0
    },
    "WeaponRecoilReduction": {
      "Name": "Recoil Reduction",
      "Value": 40
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "BonusClipSizePercent": 50,
      "DescKey": "citadel_ability_hornet_leap_t1_desc"
    },
    {
      "AbilityDuration": 10
    },
    {
      "DescKey": "citadel_ability_hornet_leap_t3_desc",
      "MagicDamagePerBullet": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.1
        },
        "Value": 10
      },
      "RefreshOnKill": 1
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
      "hero_key": "hero_hornet",
      "hero_name": "Vindicta",
      "lookup": "flight",
      "name": "Flight",
      "type": "ability"
    }
  ]
}
````
