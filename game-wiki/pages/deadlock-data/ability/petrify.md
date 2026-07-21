---
title: "Petrify"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_viper_ult"
canonical_name: "Petrify"
snapshot_id: 39567
source_document_id: 7070
payload_hash: "43bdc221f4945243bca51d6730578e6c73681caad25c0c6e5bf54a1f0ab59c60"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.654058+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Petrify

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_viper_ult`
- Snapshot ID: `39567`
- Source-Dokument: `7070`
- Kurzinfo: Petrify aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 20,
  "AbilityCooldown": 70,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.15,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0695
    },
    "Value": 200
  },
  "HalfHeight": 6,
  "IsDisabled": false,
  "Key": "ability_viper_ult",
  "Name": "Petrify",
  "PetrifyDamageBreakThreshold": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.116
    },
    "Value": 200
  },
  "PetrifyDuration": 3,
  "PetrifyModifier": {
    "Class": "CitadelPetrify",
    "Subclass": "ViperUltPetrify"
  },
  "PreDetonateDuration": 1,
  "Radius": 4,
  "Upgrades": [
    {
      "AbilityCooldown": -15
    },
    {
      "PetrifyDuration": 1.5
    },
    {
      "Radius": 3
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
