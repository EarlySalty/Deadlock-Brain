---
title: "Doorway"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_doorman_doorway"
canonical_name: "Doorway"
snapshot_id: 39776
source_document_id: 7071
payload_hash: "e9c909c9c736b8799c2fe87dd27065123239bfc822766323b0decf5556d593e5"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.182388+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Doorway

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_doorman_doorway`
- Snapshot ID: `39776`
- Source-Dokument: `7071`
- Kurzinfo: Doorway aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 50
  },
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
    "Value": 20
  },
  "DescKey": "ability_doorman_doorway_desc",
  "HeroKey": "hero_doorman",
  "HeroName": "The Doorman",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_doorman_doorway_desc",
    "Main": {
      "Props": [
        {
          "Key": "DoorwayDistance",
          "Name": "Doorway Distance",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Type": "distance",
          "Value": 70
        },
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Title": "On Portal",
          "Type": "bullet_armor_up",
          "Value": 0
        },
        {
          "Key": "BarrierDuration",
          "Name": "Barrier Duration",
          "Title": "On Portal",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_doorman_doorway",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Doorway",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DoorwayCloseCooldown": {
      "Name": null,
      "Value": 8
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityDuration": 15
    },
    {
      "BarrierDuration": 12,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.5
        },
        "Value": 250
      },
      "DescKey": "ability_doorman_doorway_t2_desc"
    },
    {
      "AbilityCastRange": 30,
      "DescKey": "ability_doorman_doorway_t3_desc",
      "DoorwayDistance": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.15
        },
        "Value": 45
      }
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
      "hero_key": "hero_doorman",
      "hero_name": "The Doorman",
      "lookup": "doorway",
      "name": "Doorway",
      "type": "ability"
    }
  ]
}
````
