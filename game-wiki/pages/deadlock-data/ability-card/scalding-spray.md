---
title: "Scalding Spray"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "fathom_scalding_spray"
canonical_name: "Scalding Spray"
snapshot_id: 39891
source_document_id: 7071
payload_hash: "2eb3f19b2dc06f16306b005b829ea1eb00b57a66459044e3c7c8c49f06090a39"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.404578+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Scalding Spray

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `fathom_scalding_spray`
- Snapshot ID: `39891`
- Source-Dokument: `7071`
- Kurzinfo: Scalding Spray aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 40.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 8
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 3
  },
  "DescKey": "fathom_scalding_spray_desc",
  "HeroKey": "hero_slork",
  "HeroName": "Fathom",
  "Info1": {
    "Alt": [],
    "DescKey": "fathom_scalding_spray_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.372
          },
          "Type": "tech_damage",
          "Value": 40
        },
        {
          "Key": "WeaponDamageBonusPerSec",
          "Name": "Weapon Damage Gain Per Sec",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0372
          },
          "Type": "bullet_damage",
          "Value": 5
        },
        {
          "Key": "WeaponDamageBonusDuration",
          "Name": "Bonus Duration",
          "Type": "duration",
          "Value": 12
        }
      ]
    }
  },
  "Key": "fathom_scalding_spray",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Scalding Spray",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 12
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -15.0
    },
    {
      "AbilityDuration": 2
    },
    {
      "DPS": 55
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
      "hero_key": "hero_slork",
      "hero_name": "Fathom",
      "lookup": "scalding spray",
      "name": "Scalding Spray",
      "type": "ability"
    }
  ]
}
````
