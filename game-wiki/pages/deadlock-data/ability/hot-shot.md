---
title: "Hot Shot"
entity_type: "ability"
source: "deadlock_data"
external_id: "tokamak_hot_shot"
canonical_name: "Hot Shot"
snapshot_id: 39727
source_document_id: 7070
payload_hash: "39623f5939354fcef20454c96136fe892e3bef5ccae058aa1bd559cbdb2f41c9"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.061618+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Hot Shot

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `tokamak_hot_shot`
- Snapshot ID: `39727`
- Source-Dokument: `7070`
- Kurzinfo: Hot Shot aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityChannelTime": 1.0,
  "AbilityCharges": 2,
  "AbilityCooldown": 32.0,
  "AbilityCooldownBetweenCharge": 1,
  "AbilityUnitTargetLimit": 1,
  "BeamLength": 30,
  "BeamWidth": 4.0,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanCancelDuringCastDelay",
    "BehaviorShowCastRangeAsSatSphereWhileCasting"
  ],
  "ChannelMoveSpeed": 1.8,
  "HotDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.548402
    },
    "Value": 90
  },
  "IsDisabled": false,
  "Key": "tokamak_hot_shot",
  "Name": "Hot Shot",
  "NormalDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.335135
    },
    "Value": 55
  },
  "TickRate": 0.1,
  "TrackingSpeed": 180,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "AbilityChannelTime": 0.5
    },
    {
      "HotDPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.292481
        },
        "Value": 48
      },
      "NormalDPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.097494
        },
        "Value": 16
      }
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
      "hero_key": "hero_tokamak",
      "hero_name": "Tokamak",
      "lookup": "hot shot",
      "name": "Hot Shot",
      "type": "ability"
    }
  ]
}
````
