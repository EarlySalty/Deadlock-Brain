---
title: "Grand Finale"
entity_type: "ability"
source: "deadlock_data"
external_id: "cadence_ability_grandfinale"
canonical_name: "Grand Finale"
snapshot_id: 39593
source_document_id: 7070
payload_hash: "c94818ab62ddfe585cd7e5c3a3f693efe54fd8d55c16e1ea9ba713ab66fefaac"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.721351+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Grand Finale

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `cadence_ability_grandfinale`
- Snapshot ID: `39593`
- Source-Dokument: `7070`
- Kurzinfo: Grand Finale aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCooldown": 95.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact"
  ],
  "BuildUpDuration": 10,
  "BuildUpMaxDamage": 150,
  "BuildUpMaxDamageNonHero": 300,
  "ChannelMoveSpeed": -1,
  "ExplosiveDamage": 120,
  "FireRateBonus": 30,
  "GrandFinaleAOEModifier": {
    "Class": "CadenceGrandfinaleAoe",
    "ProvidedByAura": {
      "BuildUpModifier": {
        "Class": "CitadelBaseBuildup",
        "Subclass": "CitadelBaseBuildup"
      },
      "Class": "CadenceGrandfinaleBuff",
      "Subclass": "CadenceGrandfinaleBuff"
    },
    "Subclass": "CadenceGrandfinaleAoe"
  },
  "IsDisabled": false,
  "Key": "cadence_ability_grandfinale",
  "Name": "Grand Finale",
  "Radius": 12,
  "StageDuration": 12,
  "StageRadius": 15,
  "Upgrades": [
    {
      "AbilityCooldown": -19.0
    },
    {
      "ExplosiveDamage": 120
    },
    {
      "FireRateBonus": 20
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
