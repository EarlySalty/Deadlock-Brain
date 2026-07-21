---
title: "Smoke Bomb"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_smoke_bomb"
canonical_name: "Smoke Bomb"
snapshot_id: 39824
source_document_id: 7071
payload_hash: "92910b2c0169677ed73adebb3463add2f1964c2e5c48e7a6b24da5b1c4e9e08c"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.274874+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Smoke Bomb

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_smoke_bomb`
- Snapshot ID: `39824`
- Source-Dokument: `7071`
- Kurzinfo: Smoke Bomb aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 33.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Scale": {
      "Type": "spirit",
      "Value": 0.1
    },
    "Type": "duration",
    "Value": 8
  },
  "DescKey": "ability_smoke_bomb_desc",
  "Duration": {
    "RevealOnDamageDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 1.5
    },
    "RevealOnSpottedDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 0.5
    }
  },
  "HeroKey": "hero_haze",
  "HeroName": "Haze",
  "Info1": {
    "Alt": [
      {
        "Key": "SpottedRadius",
        "Name": "Spot Radius",
        "Type": "distance",
        "Value": 18
      },
      {
        "Key": "InvisMoveSpeedMod",
        "Name": "Invis Sprint Speed",
        "Type": "move_speed",
        "Value": 0
      },
      {
        "Key": "PostInvisBuffDuration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "PhaseOutDuration",
        "Name": "Invincible Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_smoke_bomb_desc",
    "Main": {
      "Props": [
        {
          "Key": "InvisFadeToDuration",
          "Name": "Fade Time",
          "Type": "duration",
          "Value": 1.5
        }
      ]
    }
  },
  "Key": "ability_smoke_bomb",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Smoke Bomb",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "FullInvisDistance": {
      "Name": null,
      "Value": 50
    },
    "InvisAlertWhenFading": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "InvisMoveSpeedMod": 7
    },
    {
      "AbilityCharges": 2,
      "AbilityCooldownBetweenCharge": 7,
      "DescKey": "ability_smoke_bomb_t2_desc"
    },
    {
      "BulletLifesteal": 50,
      "DescKey": "ability_smoke_bomb_t3_desc",
      "DispelOnUse": 1,
      "PostInvisBuffDuration": 5
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
      "lookup": "smoke bomb",
      "name": "Smoke Bomb",
      "type": "ability"
    }
  ]
}
````
