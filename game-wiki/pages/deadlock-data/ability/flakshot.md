---
title: "Flakshot"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_skyrunner_flakshot"
canonical_name: "Flakshot"
snapshot_id: 39530
source_document_id: 7070
payload_hash: "82f7b1585f48f3e263e6eff2e5152dc82f4bffea1bdbafc6dae88525304c7f37"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.556576+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Flakshot

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_skyrunner_flakshot`
- Snapshot ID: `39530`
- Source-Dokument: `7070`
- Kurzinfo: Flakshot aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "Class": "Base",
      "Subclass": "Flakshotbase"
    }
  ],
  "BehaviourBits": [
    "BehaviorProjectileFiredAsBullet"
  ],
  "BonusDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.093
    },
    "Value": 3
  },
  "ChannelMoveSpeed": -1,
  "DirectionVariance": 0.02,
  "IsDisabled": false,
  "Key": "ability_skyrunner_flakshot",
  "MinEffectiveness": -1,
  "Name": "Flakshot",
  "Radius": 5,
  "RicochetAssistRatio": 0.5,
  "RicochetChance": 50,
  "RicochetDamagePercent": 100,
  "RicochetRadius": 20,
  "Upgrades": [
    {
      "RicochetChance": 50
    },
    {
      "BonusDamage": 5
    },
    {
      "RicochetDamagePercent": 50
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
    "card_name": "Flakshot",
    "hero_key": "hero_skyrunner",
    "hero_name": "Skyrunner",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_skyrunner",
      "hero_name": "Skyrunner",
      "lookup": "flakshot",
      "name": "Flakshot",
      "type": "ability"
    }
  ]
}
````
