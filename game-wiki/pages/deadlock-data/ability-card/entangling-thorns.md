---
title: "Entangling Thorns"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_tengu_urn"
canonical_name: "Entangling Thorns"
snapshot_id: 39903
source_document_id: 7071
payload_hash: "33df161c3965430ae130acf8d833433240750f8fbf82c8e2642a7cc9ad37730d"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.427983+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Entangling Thorns

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tengu_urn`
- Snapshot ID: `39903`
- Source-Dokument: `7071`
- Kurzinfo: Entangling Thorns aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 32.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 5
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 4
  },
  "DescKey": "citadel_ability_tengu_urn_desc",
  "HeroKey": "hero_tengu",
  "HeroName": "Ivy",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_tengu_urn_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.55
          },
          "Type": "tech_damage",
          "Value": 40
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 35
        }
      ]
    }
  },
  "Key": "citadel_ability_tengu_urn",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Entangling Thorns",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "Height": {
      "Name": null,
      "Value": 2
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 6
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 0
      },
      "DescKey": "citadel_ability_tengu_urn_t2_desc",
      "Radius": 2
    },
    {
      "DescKey": "citadel_ability_tengu_urn_t3_desc",
      "EntangleDuration": 1.6,
      "TimeToEntangle": 2
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
      "hero_key": "hero_tengu",
      "hero_name": "Ivy",
      "lookup": "entangling thorns",
      "name": "Entangling Thorns",
      "type": "ability"
    }
  ]
}
````
