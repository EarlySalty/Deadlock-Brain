---
title: "Swish"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_boho_doublehit"
canonical_name: "Swish"
snapshot_id: 39403
source_document_id: 7070
payload_hash: "facfad02418ab30345355bb56d464e95f691cab135249f7237ea0a2c41ff4955"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.238054+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Swish

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_boho_doublehit`
- Snapshot ID: `39403`
- Source-Dokument: `7070`
- Kurzinfo: Swish aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 11,
  "AbilityCooldown": 14,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.7,
  "AbilityUnitTargetLimit": 16,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusMoveSpeed": 2,
  "BuffDuration": 4,
  "BuffModifier": {
    "Class": "BohoDoublehitBuff",
    "Subclass": "Buff"
  },
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.8
    },
    "Value": 80
  },
  "CombatBarrierPerStack": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.2
    },
    "Value": 20
  },
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 50
  },
  "IsDisabled": false,
  "Key": "ability_boho_doublehit",
  "MaxStacks": 6,
  "Name": "Swish",
  "TargetingConeAngle": 100,
  "TimeBetweenAttacks": 0.35,
  "Upgrades": [
    {
      "Damage": 18.0
    },
    {
      "BonusMoveSpeed": 2
    },
    {
      "CombatBarrier": 80
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
    "card_name": "Swish",
    "hero_key": "hero_boho",
    "hero_name": "Boho",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_boho",
      "hero_name": "Boho",
      "lookup": "swish",
      "name": "Swish",
      "type": "ability"
    }
  ]
}
````
