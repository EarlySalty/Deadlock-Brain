---
title: "Scorn"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_intimidate"
canonical_name: "Scorn"
snapshot_id: 39839
source_document_id: 7071
payload_hash: "02fad13e846853307bde01333e47e4709e2bea38c5560e677a7e9394cdb0fc9d"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.303355+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Scorn

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_intimidate`
- Snapshot ID: `39839`
- Source-Dokument: `7071`
- Kurzinfo: Scorn aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 13
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_intimidate_desc",
  "HeroKey": "hero_krill",
  "HeroName": "Mo & Krill",
  "Info1": {
    "Alt": [
      {
        "Key": "DamageHealMultNonHero",
        "Name": "Damage to Heal",
        "Type": "healing",
        "Value": 0.35
      }
    ],
    "DescKey": "ability_intimidate_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.75
          },
          "Type": "tech_damage",
          "Value": 50
        },
        {
          "Key": "DamageHealMult",
          "Name": "Hero Damage to Heal",
          "Type": "healing",
          "Value": 1.2
        }
      ]
    }
  },
  "Key": "ability_intimidate",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Scorn",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickRate": {
      "Name": null,
      "Value": 0.1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 9
  },
  "Slot": "1",
  "Upgrades": [
    {
      "Damage": 35
    },
    {
      "AbilityCooldown": -5,
      "DescKey": "ability_intimidate_t2_desc",
      "Radius": 1
    },
    {
      "DamageBonus": 15,
      "DebuffDuration": 16,
      "DescKey": "ability_intimidate_t3_desc"
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
      "hero_key": "hero_krill",
      "hero_name": "Mo & Krill",
      "lookup": "scorn",
      "name": "Scorn",
      "type": "ability"
    }
  ]
}
````
