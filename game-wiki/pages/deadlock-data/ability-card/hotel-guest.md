---
title: "Hotel Guest"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_doorman_hotel"
canonical_name: "Hotel Guest"
snapshot_id: 39778
source_document_id: 7071
payload_hash: "f110b18964cba0e2f2cc0b449836e48dc2b509633b5ce3dbed3f3e40f28bf930"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.186447+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Hotel Guest

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_doorman_hotel`
- Snapshot ID: `39778`
- Source-Dokument: `7071`
- Kurzinfo: Hotel Guest aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 7
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 140
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6.5
  },
  "DescKey": "ability_doorman_hotel_desc",
  "HeroKey": "hero_doorman",
  "HeroName": "The Doorman",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_doorman_hotel_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.0
          },
          "Title": "Cost of Stay",
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "LateCheckoutDamage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.5
          },
          "Title": "Failure to Check-Out",
          "Type": "tech_damage",
          "Value": 125
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Title": "While a Guest",
          "Type": "duration",
          "Value": 6.5
        },
        {
          "Key": "HotelTimeScale",
          "Name": "Slow",
          "Title": "While a Guest",
          "Type": "slow",
          "Value": null
        }
      ]
    }
  },
  "Key": "ability_doorman_hotel",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Hotel Guest",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.7
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TimeSlowDuration": {
      "Name": null,
      "Value": 1.0
    },
    "TimeSlowPercentage": {
      "Name": null,
      "Value": 100
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -20,
      "DescKey": "ability_doorman_hotel_t1_desc",
      "StaminaDrain": 1
    },
    {
      "Damage": 150,
      "DescKey": "ability_doorman_hotel_t2_desc",
      "LateCheckoutDamage": 150,
      "LateCheckoutStun": 1.5
    },
    {
      "DescKey": "ability_doorman_hotel_t3_desc",
      "LateCheckoutCooldown": 13,
      "UnstoppableWhileHotelOccupied": 1
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
      "lookup": "hotel guest",
      "name": "Hotel Guest",
      "type": "ability"
    }
  ]
}
````
