---
title: "Pulse"
entity_type: "ability"
source: "deadlock_data"
external_id: "rutger_pulse"
canonical_name: "Pulse"
snapshot_id: 39703
source_document_id: 7070
payload_hash: "5a58648d52d35c1288957998219dbf957a64180c7aeb9bfb831b5c3a23169b30"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.996078+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Pulse

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `rutger_pulse`
- Snapshot ID: `39703`
- Source-Dokument: `7070`
- Kurzinfo: Pulse aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 127.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "AuraModifier": {
    "Class": "RutgerPulseAura",
    "ProvidedByAura": {
      "Class": "RutgerPulseTarget",
      "Subclass": "RutgerPulseTargetSubclass"
    },
    "Subclass": "RutgerPulseAuraSubclass"
  },
  "BehaviourBits": [
    "BehaviorNoTarget"
  ],
  "ChannelMoveSpeed": -1,
  "DamageMax": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 350
  },
  "DamageMax_DistanceFuzz": 2,
  "DamageMin": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 50
  },
  "EndRadius": 30,
  "IsDisabled": false,
  "Key": "rutger_pulse",
  "MovementSlow": 25,
  "Name": "Pulse",
  "SpreadDuration": 0.6,
  "StartRadius": 1,
  "Upgrades": [
    {
      "MovementSlow": 25
    },
    {
      "AbilityCooldown": -47.0
    },
    {
      "DamageMax": 200
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
      "hero_key": "hero_rutger",
      "hero_name": "Rutger",
      "lookup": "pulse",
      "name": "Pulse",
      "type": "ability"
    }
  ]
}
````
