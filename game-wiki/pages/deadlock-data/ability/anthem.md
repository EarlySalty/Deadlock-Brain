---
title: "Anthem"
entity_type: "ability"
source: "deadlock_data"
external_id: "cadence_ability_anthem"
canonical_name: "Anthem"
snapshot_id: 39591
source_document_id: 7070
payload_hash: "0158944c8a0247d783740b92adff2255ea78b2e77388295273e2448d8595d52c"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.714245+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Anthem

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `cadence_ability_anthem`
- Snapshot ID: `39591`
- Source-Dokument: `7070`
- Kurzinfo: Anthem aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.75,
  "AbilityCooldown": 37.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 1,
  "AnthemAOEModifier": {
    "Class": "CadenceAnthemAoe",
    "ProvidedByAura": {
      "Class": "CadenceAnthemBuff",
      "Subclass": "CadenceAnthemBuff"
    },
    "Subclass": "CadenceAnthemAoe"
  },
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": 1.3,
  "ExtraLargeClip": 25,
  "IsDisabled": false,
  "Key": "cadence_ability_anthem",
  "LingerDuration": 0.5,
  "Name": "Anthem",
  "PeakFireRateBonus": 100,
  "Radius": 12,
  "Upgrades": [
    {
      "ExtraLargeClip": 75
    },
    {
      "Radius": 4
    },
    {
      "PeakFireRateBonus": 100
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
      "hero_key": "hero_cadence",
      "hero_name": "Cadence",
      "lookup": "anthem",
      "name": "Anthem",
      "type": "ability"
    }
  ]
}
````
