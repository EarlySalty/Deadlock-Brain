---
title: "Willpower"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_warden_high_alert"
canonical_name: "Willpower"
snapshot_id: 39570
source_document_id: 7070
payload_hash: "459868ba390f817e0690a05ea7488084ba2644710d97714bd5b6a72e43b1b203"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.663809+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Willpower

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_warden_high_alert`
- Snapshot ID: `39570`
- Source-Dokument: `7070`
- Kurzinfo: Willpower aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 40,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "BuffModifier": {
    "Class": "WardenHighAlert",
    "StatusEffectPriority": 100,
    "Subclass": "WardenHighAlert"
  },
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.8
    },
    "Value": 125
  },
  "IsDisabled": false,
  "Key": "ability_warden_high_alert",
  "MoveSpeedBonusPct": 15,
  "Name": "Willpower",
  "Upgrades": [
    {
      "MoveSpeedBonusPct": 20
    },
    {
      "AbilityCooldown": -24,
      "AbilityDuration": 2
    },
    {
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 2.7
        },
        "Value": 0
      },
      "StatusResistancePercent": 40
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
    "card_name": "Willpower",
    "hero_key": "hero_warden",
    "hero_name": "Warden",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_warden",
      "hero_name": "Warden",
      "lookup": "willpower",
      "name": "Willpower",
      "type": "ability"
    }
  ]
}
````
