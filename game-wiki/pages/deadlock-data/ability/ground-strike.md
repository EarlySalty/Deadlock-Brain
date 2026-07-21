---
title: "Ground Strike"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_lash_down_strike"
canonical_name: "Ground Strike"
snapshot_id: 39624
source_document_id: 7070
payload_hash: "cacc7bf37fd3c6a1e52f1d5bc28538853f837c0587f5d89bcfbd0e88ba4f4b62"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.802181+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Ground Strike

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_lash_down_strike`
- Snapshot ID: `39624`
- Source-Dokument: `7070`
- Kurzinfo: Ground Strike aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCooldown": 18.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanCastOnZipline"
  ],
  "ChannelMoveSpeed": -1,
  "DownStrikeModifier": {
    "Class": "Base",
    "Subclass": "LashDownStrikeEmbedded"
  },
  "DragModifier": {
    "Class": "ChargeDragEnemy",
    "ForceDistScale": 10,
    "ForwardOffset": 200,
    "Subclass": "ChargeDragEnemy",
    "VerticalOffset": -200
  },
  "ImpactModifier": {
    "Class": "SlowBase",
    "Subclass": "SlowBase"
  },
  "IsDisabled": false,
  "Key": "citadel_ability_lash_down_strike",
  "MinAimAngle": 60,
  "Name": "Ground Strike",
  "Radius": 10,
  "StompDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.7905
    },
    "Value": 60.0
  },
  "StompDamagePerMeterPrimary": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.04
    },
    "Value": 5.5
  },
  "StompDamagePerMeterSecondary": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.008137
    },
    "Value": 4.2
  },
  "StompDamagePrimaryRange": 25,
  "StompVerticalThreshold": 118,
  "StrikeVelocity": 50,
  "Upgrades": [
    {
      "AbilityCooldown": -10.0
    },
    {
      "EnemySlowPct": 50,
      "SlowDuration": 3,
      "StompBounceHeight": 400,
      "TossDuration": 1
    },
    {
      "StompDamagePerMeterPrimary": {
        "Multiply": true,
        "Scale": {
          "Type": "spirit",
          "Value": 0.03255
        },
        "Value": 2.13
      },
      "StompDamagePerMeterSecondary": {
        "Multiply": true,
        "Scale": {
          "Type": "spirit",
          "Value": 0.008137
        },
        "Value": 2.13
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
    "card_name": "Ground Strike",
    "hero_key": "hero_lash",
    "hero_name": "Lash",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_lash",
      "hero_name": "Lash",
      "lookup": "ground strike",
      "name": "Ground Strike",
      "type": "ability"
    }
  ]
}
````
