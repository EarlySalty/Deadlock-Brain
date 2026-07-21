---
title: "Charged Shot"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_charged_shot"
canonical_name: "Charged Shot"
snapshot_id: 39867
source_document_id: 7071
payload_hash: "e06a29f6c28b00e339b7834f842af6d6353f35ae4af9990ab6a601b88f8f2b99"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.359350+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Charged Shot

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_charged_shot`
- Snapshot ID: `39867`
- Source-Dokument: `7071`
- Kurzinfo: Charged Shot aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.5
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 9999
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 17.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 4
  },
  "DescKey": "ability_charged_shot_desc",
  "HeroKey": "hero_orion",
  "HeroName": "Grey Talon",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_charged_shot_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.0
          },
          "Type": "tech_damage",
          "Value": 80
        }
      ]
    }
  },
  "Key": "ability_charged_shot",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.5
    }
  },
  "Name": "Charged Shot",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 4.1
    },
    "CameraHeightOffset": {
      "Name": null,
      "Value": 20
    },
    "CameraHorizontalOffset": {
      "Name": null,
      "Value": 15
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 1.524
    },
    "TechCleaveExpireTime": {
      "Name": null,
      "Value": 0.2
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "Damage": 54.0,
      "DescKey": "ability_charged_shot_t2_desc"
    },
    {
      "AbilityCooldownBetweenCharge": -3,
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 0
      },
      "DescKey": "ability_charged_shot_t3_desc"
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
      "hero_key": "hero_orion",
      "hero_name": "Grey Talon",
      "lookup": "charged shot",
      "name": "Charged Shot",
      "type": "ability"
    }
  ]
}
````
