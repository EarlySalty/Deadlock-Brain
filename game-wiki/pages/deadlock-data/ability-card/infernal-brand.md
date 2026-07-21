---
title: "Infernal Brand"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "gunslinger_demonMark"
canonical_name: "Infernal Brand"
snapshot_id: 39821
source_document_id: 7071
payload_hash: "438e4823524049ec804912d57977bbd12b62ea1c3b938897b635d7735a8f4916"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.268410+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Infernal Brand

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `gunslinger_demonMark`
- Snapshot ID: `39821`
- Source-Dokument: `7071`
- Kurzinfo: Infernal Brand aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 40
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 14
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Damage": {
    "Damage": {
      "Name": "Damage",
      "Scale": {
        "Type": "weapon_damage",
        "Value": 0.93
      },
      "Type": "bullet_damage",
      "Value": 100
    }
  },
  "DescKey": "gunslinger_demonMark_desc",
  "HeroKey": "hero_gunslinger",
  "HeroName": "Gunslinger",
  "Info1": {
    "Alt": [
      {
        "Key": "SearchRadius",
        "Name": "Search Radius",
        "Value": 20
      },
      {
        "Key": "MarkDuration",
        "Name": "Mark Duration",
        "Value": 5
      },
      {
        "Key": "BuffDuration",
        "Name": "Buff Duration",
        "Value": 3
      }
    ],
    "DescKey": "gunslinger_demonMark_desc",
    "Main": {
      "Props": [
        {
          "Key": "ProcDamage",
          "Name": "Spirit Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.744
          },
          "Type": "tech_damage",
          "Value": 100
        },
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Type": "fire_rate",
          "Value": 25
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 2
        }
      ]
    }
  },
  "Key": "gunslinger_demonMark",
  "Name": "Infernal Brand",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "SearchAngle": {
      "Name": null,
      "Value": 20
    },
    "SearchRate": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -3
    },
    {
      "ProcDamage": 50
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
      "hero_key": "hero_gunslinger",
      "hero_name": "Gunslinger",
      "lookup": "infernal brand",
      "name": "Infernal Brand",
      "type": "ability"
    }
  ]
}
````
