---
title: "Acrobat"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_swan_acrobat"
canonical_name: "Acrobat"
snapshot_id: 39537
source_document_id: 7070
payload_hash: "e09e8e6e96ecf48fa0dd29342e0ade9206fa90ca1470cbbdec6a2f44dfc0a998"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.573224+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Acrobat

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_swan_acrobat`
- Snapshot ID: `39537`
- Source-Dokument: `7070`
- Kurzinfo: Acrobat aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 7,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "BurstBonusPerStack": 1,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_swan_acrobat",
  "MaxStacks": 4,
  "Name": "Acrobat",
  "StackingModifier": {
    "Class": "SwanAcrobat",
    "Subclass": "Stack"
  },
  "Upgrades": [
    {
      "AbilityDuration": 3
    },
    {
      "FireRatePerStack": 4
    },
    {
      "MaxStacks": 4
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
    "card_name": "Acrobat",
    "hero_key": "hero_swan",
    "hero_name": "Swan",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_swan",
      "hero_name": "Swan",
      "lookup": "acrobat",
      "name": "Acrobat",
      "type": "ability"
    }
  ]
}
````
