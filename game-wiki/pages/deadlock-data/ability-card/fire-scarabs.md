---
title: "Fire Scarabs"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "mirage_fire_beetles"
canonical_name: "Fire Scarabs"
snapshot_id: 39851
source_document_id: 7071
payload_hash: "2b0631a478b4a26de107836d0a38b6f0b781ef50a427973edfcdc7452bf5e90c"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.328192+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Fire Scarabs

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `mirage_fire_beetles`
- Snapshot ID: `39851`
- Source-Dokument: `7071`
- Kurzinfo: Fire Scarabs aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.05
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 35
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 1
  },
  "DescKey": "mirage_fire_beetles_desc",
  "HeroKey": "hero_mirage",
  "HeroName": "Mirage",
  "Info1": {
    "Alt": [
      {
        "Key": "StealDuration",
        "Name": "Steal Duration",
        "Type": "duration",
        "Value": 7
      }
    ],
    "DescKey": "mirage_fire_beetles_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.1
          },
          "Type": "healing",
          "Value": 8
        },
        {
          "Key": "OutgoingDamagePenaltyPercent",
          "Name": "Damage Penalty",
          "Type": "damage",
          "Value": -20
        }
      ]
    }
  },
  "Key": "mirage_fire_beetles",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Fire Scarabs",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "MaxStacks": {
      "Name": "Max Stacks",
      "Value": 100
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "DPS": 7
    },
    {
      "AbilityCharges": 2,
      "DescKey": "mirage_fire_beetles_t2_desc"
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.17
        },
        "Value": 0
      },
      "OutgoingDamagePenaltyPercent": -15
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
      "hero_key": "hero_mirage",
      "hero_name": "Mirage",
      "lookup": "fire scarabs",
      "name": "Fire Scarabs",
      "type": "ability"
    }
  ]
}
````
