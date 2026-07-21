---
title: "Spirit Lasso"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_gravity_lasso"
canonical_name: "Spirit Lasso"
snapshot_id: 39751
source_document_id: 7071
payload_hash: "964ab08484c34870c7a161c50003e3f178801769cff57640c83c729c6581ad76"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.131870+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Spirit Lasso

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_gravity_lasso`
- Snapshot ID: `39751`
- Source-Dokument: `7071`
- Kurzinfo: Spirit Lasso aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.5
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 20
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 130
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 2.25
  },
  "DescKey": "ability_gravity_lasso_desc",
  "HeroKey": "hero_astro",
  "HeroName": "Holliday",
  "Info1": {
    "Alt": [
      {
        "Key": "BouncePadExtendDuration",
        "Value": 1.0
      }
    ],
    "DescKey": "ability_gravity_lasso_desc",
    "Main": {
      "Props": [
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "StatusEffect": "Stun",
          "Type": "duration",
          "Value": 2.25
        },
        {
          "Key": "AbilityCastRange",
          "Name": "Cast Range",
          "Type": "range",
          "Value": 20
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.93
          },
          "Type": "tech_damage",
          "Value": 80
        }
      ]
    }
  },
  "Key": "ability_gravity_lasso",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Spirit Lasso",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraPreviewDistance": {
      "Name": null,
      "Value": 200
    },
    "CameraPreviewOffset": {
      "Name": null,
      "Value": 25
    },
    "CameraPreviewSpeed": {
      "Name": null,
      "Value": 0.6
    },
    "ExtraTargetConeAngle": {
      "Name": "Targetting Cone Angle",
      "Value": 60
    },
    "ExtraTargetHorizontalOffset": {
      "Name": null,
      "Value": 30
    },
    "FollowDampingFactor": {
      "Name": null,
      "Value": 8
    },
    "FollowDistance": {
      "Name": null,
      "Value": 60
    },
    "GrabExtraTargetsRadiusMult": {
      "Name": null,
      "Value": 2
    },
    "LassoTargetMaxSpeed": {
      "Name": null,
      "Value": 55
    },
    "LiftHeight": {
      "Name": null,
      "Value": 7
    },
    "LiftHorizontal": {
      "Name": null,
      "Value": -30
    },
    "LiftInitialDelay": {
      "Name": null,
      "Value": 0.5
    },
    "LiftInitialRisingSpeed": {
      "Name": null,
      "Value": 100
    },
    "LiftInitialVelocityStart": {
      "Name": null,
      "Value": 500
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "Damage": 80
    },
    {
      "AbilityDuration": 0.75
    },
    {
      "AbilityCooldown": -40
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
      "hero_key": "hero_astro",
      "hero_name": "Holliday",
      "lookup": "spirit lasso",
      "name": "Spirit Lasso",
      "type": "ability"
    }
  ]
}
````
