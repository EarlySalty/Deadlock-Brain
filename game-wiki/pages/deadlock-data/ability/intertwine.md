---
title: "Intertwine"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_boho_damageshare"
canonical_name: "Intertwine"
snapshot_id: 39402
source_document_id: 7070
payload_hash: "62240ed2e68fab80f461a53b10540dd4c939dba4233b069500eafe1aab2f6414"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.235035+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Intertwine

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_boho_damageshare`
- Snapshot ID: `39402`
- Source-Dokument: `7070`
- Kurzinfo: Intertwine aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "DamageShareModifier": {
    "Class": "BohoDamageshare",
    "Subclass": "Damageshare"
  },
  "DamageSharePercentage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.186
    },
    "Value": 30
  },
  "DamageShareRadius": 8,
  "DebuffDuration": 6,
  "IsDisabled": false,
  "Key": "ability_boho_damageshare",
  "LinkDuration": 0.5,
  "MaxLinks": 6,
  "Name": "Intertwine",
  "TickRate": 0.25,
  "Upgrades": [
    {
      "DamageShareRadius": 3
    },
    {
      "AbilityDuration": 3
    },
    {
      "DamageSharePercentage": 22.5
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
    "card_name": "Intertwine",
    "hero_key": "hero_boho",
    "hero_name": "Boho",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_boho",
      "hero_name": "Boho",
      "lookup": "intertwine",
      "name": "Intertwine",
      "type": "ability"
    }
  ]
}
````
