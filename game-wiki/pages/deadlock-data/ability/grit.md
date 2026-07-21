---
title: "Grit"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_priest_stackingdefense"
canonical_name: "Grit"
snapshot_id: 39517
source_document_id: 7070
payload_hash: "4da0ef908a1ddc04d2b3714390029e6893537b393b269c7ae89b51be9acb58c1"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.526346+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Grit

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_stackingdefense`
- Snapshot ID: `39517`
- Source-Dokument: `7070`
- Kurzinfo: Grit aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "ChannelMoveSpeed": -1,
  "CooldownPerStack": 0.5,
  "IsDisabled": false,
  "Key": "ability_priest_stackingdefense",
  "MaxStacks": 20,
  "Name": "Grit",
  "ResistancePerStack": 1,
  "StackDuration": 8,
  "StackingModifier": {
    "Class": "PriestStackingdefense",
    "Subclass": "Stackingdefense"
  },
  "Upgrades": [
    {
      "StackDuration": 2
    },
    {
      "MaxStacks": 10
    },
    {
      "WeaponDamagePerStack": 1.5
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
