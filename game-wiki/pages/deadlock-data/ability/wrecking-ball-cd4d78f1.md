---
title: "Wrecking Ball"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_wrecker_bouldergrenade"
canonical_name: "Wrecking Ball"
snapshot_id: 39666
source_document_id: 7070
payload_hash: "ba673d82f0b8ec267a2b1f1c61a89b41b6919a311b3d1c9d7806a6ab635c6964"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.900247+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Wrecking Ball

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_wrecker_bouldergrenade`
- Snapshot ID: `39666`
- Source-Dokument: `7070`
- Kurzinfo: Wrecking Ball aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 50,
  "AbilityChannelTime": 1.2,
  "AbilityCooldown": 31,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": 2.5,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.302
    },
    "Value": 150
  },
  "ExplosionDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.744
    },
    "Value": 80
  },
  "ExplosionPushForce": 1200,
  "IsDisabled": false,
  "Key": "citadel_ability_wrecker_bouldergrenade",
  "Name": "Wrecking Ball",
  "Radius": 7,
  "StunDuration": 1,
  "Upgrades": [
    {
      "AbilityCooldown": -7.5
    },
    {
      "Damage": 50
    },
    {
      "StunDuration": 0.5
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
    "card_name": "Wrecking Ball",
    "hero_key": "hero_wrecker",
    "hero_name": "Wrecker",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_wrecker",
      "hero_name": "Wrecker",
      "lookup": "wrecking ball",
      "name": "Wrecking Ball",
      "type": "ability"
    }
  ]
}
````
