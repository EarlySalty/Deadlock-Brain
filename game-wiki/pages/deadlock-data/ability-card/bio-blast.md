---
title: "Bio Blast"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_scrap_blast"
canonical_name: "Bio Blast"
snapshot_id: 39949
source_document_id: 7071
payload_hash: "818a2d5c295d8bc769e8b825b470dd437722bf0f5269bd8ab452d4b7e75406d7"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.516755+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Bio Blast

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_scrap_blast`
- Snapshot ID: `39949`
- Source-Dokument: `7071`
- Kurzinfo: Bio Blast aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.25
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 15
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 64.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 3
  },
  "DescKey": "ability_scrap_blast_desc",
  "HeroKey": "hero_wrecker",
  "HeroName": "Wrecker",
  "Info1": {
    "Alt": [
      {
        "Key": "EnemyMoveSlowDuration",
        "Name": "Enemy Slow Duration",
        "Type": "duration",
        "Value": 5
      }
    ],
    "DescKey": "ability_scrap_blast_desc",
    "Main": {
      "Props": [
        {
          "Key": "ScrapDamage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.731203
          },
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "EnemyMoveSlow",
          "Name": "Enemy Slow per hit",
          "Type": "slow",
          "Value": 10
        }
      ]
    }
  },
  "Key": "ability_scrap_blast",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Bio Blast",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Range": {
    "BlastRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 10
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCharges": 2
    },
    {
      "ScrapDamage": 55
    },
    {
      "EnemyMoveSlow": 20
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
      "hero_key": "hero_wrecker",
      "hero_name": "Wrecker",
      "lookup": "bio blast",
      "name": "Bio Blast",
      "type": "ability"
    }
  ]
}
````
