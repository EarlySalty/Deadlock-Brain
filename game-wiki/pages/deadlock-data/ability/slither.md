---
title: "Slither"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_viper_snakedash"
canonical_name: "Slither"
snapshot_id: 39566
source_document_id: 7070
payload_hash: "1946a645b39e26972ea3ba6290b6e788417bf7a5c3c5c4c6041cca9c80c941a5"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.650886+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Slither

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_viper_snakedash`
- Snapshot ID: `39566`
- Source-Dokument: `7070`
- Kurzinfo: Slither aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "Class": "Base",
      "Subclass": "ViperslideIntrinsic"
    },
    {
      "BuffModifier": {
        "Class": "Base",
        "Subclass": "Slidebarrier"
      },
      "Class": "ViperSlidebuff",
      "Subclass": "ViperSlidebuffModifier"
    }
  ],
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventBotUsage",
    "BehaviorMovement"
  ],
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "IsDisabled": false,
  "Key": "ability_viper_snakedash",
  "Name": "Slither",
  "SlideScale": 15,
  "Upgrades": [
    {
      "SlideScale": 20
    },
    {
      "Stamina": 2
    },
    {
      "AbilityCooldown": 8,
      "BuffDuration": 5,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.8
        },
        "Value": 180
      }
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-data.json",
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
  "_deadlock_data_card": {
    "card_name": "Slither",
    "hero_key": "hero_viper",
    "hero_name": "Vyper",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_viper",
      "hero_name": "Vyper",
      "lookup": "slither",
      "name": "Slither",
      "type": "ability"
    }
  ]
}
````
