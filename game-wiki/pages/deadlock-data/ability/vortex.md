---
title: "Vortex"
entity_type: "ability"
source: "deadlock_data"
external_id: "thumper_ability_4"
canonical_name: "Vortex"
snapshot_id: 39721
source_document_id: 7070
payload_hash: "cd1bbaf0d186ecdfcb6fa5510818d8fe0c2a0fb0c2781b6886b0be21dfd1fba8"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.043407+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Vortex

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `thumper_ability_4`
- Snapshot ID: `39721`
- Source-Dokument: `7070`
- Kurzinfo: Vortex aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 10.5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "ClimbHeight": 1,
  "DistanceAboveGround": 2,
  "DropDownRate": 2,
  "Duration": 4,
  "InitialForce": 300,
  "IsDisabled": false,
  "Key": "thumper_ability_4",
  "Name": "Vortex",
  "PullAOEModifier": {
    "Class": "ThumperPullAoe",
    "ProvidedByAura": {
      "Class": "ThumperEnemyPulled",
      "Subclass": "ThumperEnemyPulled"
    },
    "Subclass": "ThumperPullAoe"
  },
  "PushAccel": 1000,
  "PushNPCSpeed": 800,
  "Radius": 15,
  "TornadoSpeed": 350,
  "Upgrades": [
    {
      "AbilityCooldown": -0.75
    },
    {
      "AbilityCooldown": -0.75
    },
    {
      "AbilityCooldown": -0.75
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
      "hero_key": "hero_thumper",
      "hero_name": "Thumper",
      "lookup": "vortex",
      "name": "Vortex",
      "type": "ability"
    }
  ]
}
````
