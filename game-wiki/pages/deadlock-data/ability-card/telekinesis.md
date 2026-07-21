---
title: "Telekinesis"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_psychic_lift"
canonical_name: "Telekinesis"
snapshot_id: 39946
source_document_id: 7071
payload_hash: "b977e1b28611e42dd1cd47a29248f6b8f4687a8c4ef06f52b4bd33419b1822d0"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.510659+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Telekinesis

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_psychic_lift`
- Snapshot ID: `39946`
- Source-Dokument: `7071`
- Kurzinfo: Telekinesis aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.45
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 10
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.65
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 150
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
  "DescKey": "citadel_ability_psychic_lift_desc",
  "HeroKey": "hero_wraith",
  "HeroName": "Wraith",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_psychic_lift_desc",
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
          "Value": 100
        },
        {
          "Key": "TossDistance",
          "Name": "Throw Range",
          "Value": 13
        },
        {
          "Key": "AbilityChannelTime",
          "Name": "Channel Duration",
          "StatusEffect": "Stun",
          "Type": "cast",
          "Value": 0.65
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "citadel_ability_psychic_lift_slam_desc",
    "Main": {
      "Props": [
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 2.25
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 40
        }
      ]
    }
  },
  "Key": "citadel_ability_psychic_lift",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Telekinesis",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DampingFactor": {
      "Name": null,
      "Value": 0.3
    },
    "LiftChainRadius": {
      "Name": null,
      "Value": 20
    },
    "LiftDuration": {
      "Name": null,
      "Value": 2
    },
    "LiftHeight": {
      "Name": null,
      "Value": 80
    },
    "TossUpStrength": {
      "Name": null,
      "Value": 220
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "Damage": 100
    },
    {
      "AbilityCooldown": -45
    },
    {
      "AbilityCastRange": 6,
      "AbilityDuration": 1.5,
      "DescKey": "citadel_ability_psychic_lift_t3_desc",
      "TossDistance": 6
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
      "lookup": "telekinesis",
      "name": "Telekinesis",
      "type": "ability"
    }
  ]
}
````
