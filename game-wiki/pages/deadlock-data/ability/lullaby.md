---
title: "Lullaby"
entity_type: "ability"
source: "deadlock_data"
external_id: "cadence_ability_lullaby"
canonical_name: "Lullaby"
snapshot_id: 39594
source_document_id: 7070
payload_hash: "9848f6cfd174725224779a809092c2cd534306745ff53a9ae4029495d527691e"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.724217+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Lullaby

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `cadence_ability_lullaby`
- Snapshot ID: `39594`
- Source-Dokument: `7070`
- Kurzinfo: Lullaby aus `deadlock_data` / `ability` mit vollstaendiger Payload.

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
  "ChannelMoveSpeed": -1,
  "ExternalBonusHealthRegen": 15,
  "IsDisabled": false,
  "Key": "cadence_ability_lullaby",
  "LingerDuration": 0.25,
  "MinimumSleepTime": 2,
  "Name": "Lullaby",
  "Radius": 12,
  "SleepAOEModifier": {
    "Class": "CadenceSleepAoe",
    "ProvidedByAura": {
      "Class": "CadenceSleeping",
      "Subclass": "CadenceSleeping"
    },
    "Subclass": "CadenceSleepAoe"
  },
  "SleepWakeUpDelay": 0.25,
  "Upgrades": [
    {
      "ExternalBonusHealthRegen": 15
    },
    {
      "Radius": 2
    },
    {
      "LingerDuration": 0.75
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
      "lookup": "lullaby",
      "name": "Lullaby",
      "type": "ability"
    }
  ]
}
````
