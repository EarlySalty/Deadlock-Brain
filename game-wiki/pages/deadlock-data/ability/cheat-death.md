---
title: "Cheat Death"
entity_type: "ability"
source: "deadlock_data"
external_id: "rutger_cheat_death"
canonical_name: "Cheat Death"
snapshot_id: 39701
source_document_id: 7070
payload_hash: "7bd154d551fba86871eef3ea52a7c79b0221ce3f028204c87f2b80ba0f615343"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.989684+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Cheat Death

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `rutger_cheat_death`
- Snapshot ID: `39701`
- Source-Dokument: `7070`
- Kurzinfo: Cheat Death aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "BonusHealthRegen": 2,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "rutger_cheat_death",
  "ModifierCheatDeathActivated": {
    "Class": "RutgerCheatDeathActivated",
    "Duration": -1.0,
    "Subclass": "RutgerCheatDeathActivated"
  },
  "Name": "Cheat Death",
  "Upgrades": [
    {
      "AbilityDuration": 2
    },
    {
      "BonusMoveSpeed": 50
    },
    {
      "BulletLifestealPercent": 100
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
  }
}
````
