---
title: "Medicinal Specter"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_mobile_resupply"
canonical_name: "Medicinal Specter"
snapshot_id: 39796
source_document_id: 7071
payload_hash: "43266a106dfc574ea3fe406b96c165aa66bf63eb96448df03242bf85faa0c024"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.221935+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Medicinal Specter

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_mobile_resupply`
- Snapshot ID: `39796`
- Source-Dokument: `7071`
- Kurzinfo: Medicinal Specter aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 50.0
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
  "DescKey": "citadel_ability_mobile_resupply_desc",
  "Health": {
    "TurretHealMult": {
      "Name": null,
      "Type": "healing",
      "Value": 1.0
    }
  },
  "HeroKey": "hero_forge",
  "HeroName": "McGinnis",
  "Info1": {
    "Alt": [
      {
        "Key": "StaminaCooldownReduction",
        "Name": "Stamina Recovery",
        "Type": "stamina_recovery",
        "Value": 0
      },
      {
        "Key": "MaxHealthRegenPct",
        "Name": "Max Health Regen",
        "Type": "healing",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_mobile_resupply_desc",
    "Main": {
      "Props": [
        {
          "Key": "ExternalBonusHealthRegen",
          "Name": "Health Regen",
          "Scale": {
            "Type": "spirit",
            "Value": 0.3
          },
          "Type": "healing",
          "Value": 25
        },
        {
          "Key": "HealRadius",
          "Name": "Heal Radius",
          "Type": "distance",
          "Value": 6
        },
        {
          "Key": "AuraFireRateBonus",
          "Name": "Unknown(AuraFireRateBonus)",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_mobile_resupply",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Medicinal Specter",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "HealInterval": {
      "Name": "Heal Interval",
      "Value": 0.1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "DescKey": "citadel_ability_mobile_resupply_t1_desc",
      "SpiritResist": 40
    },
    {
      "AbilityCooldown": -20.0,
      "DescKey": "citadel_ability_mobile_resupply_t2_desc",
      "StaminaCooldownReduction": 100.0
    },
    {
      "AbilityDuration": 1.5,
      "DescKey": "citadel_ability_mobile_resupply_t3_desc",
      "HealRadius": 3,
      "MaxHealthRegenPct": 2.0
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
      "hero_key": "hero_forge",
      "hero_name": "McGinnis",
      "lookup": "medicinal specter",
      "name": "Medicinal Specter",
      "type": "ability"
    }
  ]
}
````
