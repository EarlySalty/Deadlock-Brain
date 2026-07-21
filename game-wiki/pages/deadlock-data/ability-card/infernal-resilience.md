---
title: "Infernal Resilience"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_passive_beefy"
canonical_name: "Infernal Resilience"
snapshot_id: 39754
source_document_id: 7071
payload_hash: "c3594cb9eaab5768b0931ce634996242acd0954b9eaf191ca2eb1c4aafcb33cd"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.137750+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Infernal Resilience

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_passive_beefy`
- Snapshot ID: `39754`
- Source-Dokument: `7071`
- Kurzinfo: Infernal Resilience aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_passive_beefy_desc",
  "HeroKey": "hero_atlas",
  "HeroName": "Abrams",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealthRegen",
        "Name": "Health Regen",
        "Type": "health",
        "Value": 1
      },
      {
        "Key": "BonusMaxHealth",
        "Name": "Max Health",
        "Type": "health",
        "Value": 0
      },
      {
        "Key": "MeleeLifesteal",
        "Name": "Melee Lifesteal",
        "Type": "healing",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_passive_beefy_desc",
    "Main": {
      "Props": [
        {
          "Key": "RegenIncomingDamagePercent",
          "Name": "Damage Regenerated",
          "Title": "Passive:",
          "Type": "healing",
          "Value": 13
        },
        {
          "Key": "RegenIncomingDamageDuration",
          "Name": "Regeneration Time",
          "Title": "Passive:",
          "Type": "duration",
          "Value": 20
        }
      ]
    }
  },
  "Key": "citadel_ability_passive_beefy",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Infernal Resilience",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "NonHeroHealPct": {
      "Name": null,
      "Value": 40
    },
    "RegenDamageInterval": {
      "Name": null,
      "Value": 1.0
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BonusMaxHealth": 200
    },
    {
      "MeleeLifesteal": 18
    },
    {
      "DescKey": "citadel_ability_passive_beefy_t3_desc",
      "RegenIncomingDamagePercent": 8,
      "StatusResistancePercent": 20
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
      "hero_key": "hero_atlas",
      "hero_name": "Abrams",
      "lookup": "infernal resilience",
      "name": "Infernal Resilience",
      "type": "ability"
    }
  ]
}
````
