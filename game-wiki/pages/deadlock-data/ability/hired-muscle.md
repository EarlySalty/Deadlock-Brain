---
title: "Hired Muscle"
entity_type: "ability"
source: "deadlock_data"
external_id: "yakuza_kobun"
canonical_name: "Hired Muscle"
snapshot_id: 39741
source_document_id: 7070
payload_hash: "db69478d97cc10b6aa0877ed6f6fb149f558157fb47c050e7fe28c6fd4d138e9"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.095342+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Hired Muscle

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `yakuza_kobun`
- Snapshot ID: `39741`
- Source-Dokument: `7070`
- Kurzinfo: Hired Muscle aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 30,
  "AbilityCooldown": 32.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAllowSelfCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "CloneModifier": {
    "Class": "HeroClone",
    "Subclass": "HeroClone"
  },
  "IsDisabled": false,
  "Key": "yakuza_kobun",
  "Name": "Hired Muscle",
  "SummonCount": 1,
  "SummonDPS": 60,
  "SummonHealth": 450,
  "SummonLifetime": 45,
  "SummonMoveSpeed": 200,
  "Upgrades": [
    {
      "SummonCasterHealthPct": 20
    },
    {
      "SummonDPS": 45
    },
    {
      "SummonCount": 1
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
      "hero_key": "hero_yakuza",
      "hero_name": "The Boss",
      "lookup": "hired muscle",
      "name": "Hired Muscle",
      "type": "ability"
    }
  ]
}
````
