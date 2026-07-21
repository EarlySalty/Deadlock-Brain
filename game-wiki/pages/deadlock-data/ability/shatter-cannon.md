---
title: "Shatter Cannon"
entity_type: "ability"
source: "deadlock_data"
external_id: "thumper_ability_1"
canonical_name: "Shatter Cannon"
snapshot_id: 39718
source_document_id: 7070
payload_hash: "b0ca016d9720bc8c5328ab89f1f95a022a7109afc8231cbd11ecaee07f352e0d"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.031890+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Shatter Cannon

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `thumper_ability_1`
- Snapshot ID: `39718`
- Source-Dokument: `7070`
- Kurzinfo: Shatter Cannon aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 40,
  "AbilityCharges": 1,
  "AbilityCooldown": 17.0,
  "AbilityCooldownBetweenCharge": 4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDontTriggerSpellBlock",
    "BehaviorDisplaysDamageImpact",
    "BehaviorAlwaysPreviewRadius"
  ],
  "BounceRadians": 0.5,
  "BounceRange": 20,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 120
  },
  "IsDisabled": false,
  "Key": "thumper_ability_1",
  "MaxPlaneDistance": 1,
  "Name": "Shatter Cannon",
  "PlaneSpread": 30,
  "PushSpeedMax": 1000,
  "PushSpeedMid": 600,
  "PushSpeedMin": 100,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "Damage": 60
    },
    {
      "AbilityCooldown": -3.75,
      "AbilityCooldownBetweenCharge": -3
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_thumper",
      "hero_name": "Thumper",
      "lookup": "shatter cannon",
      "name": "Shatter Cannon",
      "type": "ability"
    }
  ]
}
````
