---
title: "Blinding Radiance"
entity_type: "ability"
source: "deadlock_data"
external_id: "tokamak_radiance"
canonical_name: "Blinding Radiance"
snapshot_id: 39728
source_document_id: 7070
payload_hash: "ab04c3df080b2cf6ca9fe7663b91e8b88b3eb34168c46a8106b6744df06e9880"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.064865+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Blinding Radiance

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `tokamak_radiance`
- Snapshot ID: `39728`
- Source-Dokument: `7070`
- Kurzinfo: Blinding Radiance aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCooldown": 48.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact"
  ],
  "BlindScale": 0.5,
  "ChannelMoveSpeed": -1,
  "EvasionChance": 20,
  "IsDisabled": false,
  "Key": "tokamak_radiance",
  "LookDotMin": 0.866,
  "LookRadiusScale": 1,
  "MaxDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.097494
    },
    "Value": 16
  },
  "Name": "Blinding Radiance",
  "RadianceModifier": {
    "Class": "TokamakRadiance",
    "Subclass": "TokamakRadiance"
  },
  "Radius": 40,
  "TickRate": 0.25,
  "Upgrades": [
    {
      "AbilityCooldown": -14.0
    },
    {
      "EvasionChance": 20
    },
    {
      "MaxDPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.146241
        },
        "Value": 24
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
      "lookup": "blinding radiance",
      "name": "Blinding Radiance",
      "type": "ability"
    }
  ]
}
````
