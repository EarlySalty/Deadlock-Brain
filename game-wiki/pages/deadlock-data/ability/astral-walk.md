---
title: "Astral Walk"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_wrecker_teleport"
canonical_name: "Astral Walk"
snapshot_id: 39587
source_document_id: 7070
payload_hash: "8464e77073bdceccc401fa3082bdd1221d1771aac93468490664b23fefec425b"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.704085+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Astral Walk

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_wrecker_teleport`
- Snapshot ID: `39587`
- Source-Dokument: `7070`
- Kurzinfo: Astral Walk aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1.5,
  "AbilityChannelTime": 8,
  "AbilityCooldown": 138.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "DamagePerSecondFlown": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.487469
    },
    "Value": 16
  },
  "DebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "SlowBase"
  },
  "EnemyMoveSlowDuration": 1,
  "EnemySlowPct": 60,
  "ExplosionRadius": 8,
  "GuidingModifier": {
    "Class": "Base",
    "Subclass": "GuidingModifier"
  },
  "IsDisabled": false,
  "Key": "ability_wrecker_teleport",
  "Name": "Astral Walk",
  "Upgrades": [
    {
      "AbilityChannelTime": 8
    },
    {
      "AbilityCooldown": -47.0
    },
    {
      "DamagePerSecondFlown": 16
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
