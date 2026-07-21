---
title: "Full Auto"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_wraith_rapidfire"
canonical_name: "Full Auto"
snapshot_id: 39945
source_document_id: 7071
payload_hash: "bbeb64f93171247bca540720c6724a81980a97e0d74af03185bef26f403ee8f5"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.508766+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Full Auto

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_wraith_rapidfire`
- Snapshot ID: `39945`
- Source-Dokument: `7071`
- Kurzinfo: Full Auto aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 45
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5
  },
  "DescKey": "citadel_ability_wraith_rapidfire_desc",
  "HeroKey": "hero_wraith",
  "HeroName": "Wraith",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityLifestealPercent",
        "Name": "Spirit Lifesteal",
        "Type": "healing",
        "Value": 0
      },
      {
        "Key": "BulletLifestealPercent",
        "Name": "Bullet Lifesteal",
        "Type": "healing",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_wraith_rapidfire_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Type": "fire_rate",
          "Value": 20
        },
        {
          "Key": "MagicDamagePerBullet",
          "Name": "Spirit Damage Per Bullet",
          "Scale": {
            "Type": "spirit",
            "Value": 0.03
          },
          "Type": "tech_damage",
          "Value": 2
        }
      ]
    }
  },
  "Key": "citadel_ability_wraith_rapidfire",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Full Auto",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -20
    },
    {
      "AbilityDuration": 3,
      "BonusFireRate": 10,
      "DescKey": "citadel_ability_wraith_rapidfire_t2_desc"
    },
    {
      "DescKey": "citadel_ability_wraith_rapidfire_t3_desc",
      "MagicDamagePerBullet": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.06
        },
        "Value": 0
      },
      "UnlimitedAmmo": 1
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
      "hero_key": "hero_wraith",
      "hero_name": "Wraith",
      "lookup": "full auto",
      "name": "Full Auto",
      "type": "ability"
    }
  ]
}
````
