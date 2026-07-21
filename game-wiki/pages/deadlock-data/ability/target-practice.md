---
title: "Target Practice"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_target_practice"
canonical_name: "Target Practice"
snapshot_id: 39541
source_document_id: 7070
payload_hash: "a744c903f4169fe0f6c0c8da1d4dd9c2c6215f12905637e35dacb53fa55b6c2f"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.585344+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Target Practice

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_target_practice`
- Snapshot ID: `39541`
- Source-Dokument: `7070`
- Kurzinfo: Target Practice aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 48.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSprint"
  ],
  "BonusPerHeadshot": 25,
  "ChannelMoveSpeed": -1,
  "DamageOnBuildup": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.066338
    },
    "Value": 125
  },
  "IsDisabled": false,
  "Key": "ability_target_practice",
  "Name": "Target Practice",
  "ShotsToProc": 4,
  "TargetOffSetScale": 2.0,
  "TargetPracticeDuration": 10,
  "TargetPracticeEnemyModifier": {
    "BuildupCompleteModifier": {
      "Class": "Base",
      "Subclass": "Base"
    },
    "BuildupModifier": {
      "BuildUpDecayDelay": 20.0,
      "Class": "CitadelBaseBuildup",
      "Subclass": "CitadelBaseBuildup"
    },
    "Class": "TargetPracticeEnemy",
    "DebuffModifier": {
      "Class": "Base",
      "Subclass": "TargetPracticeBulletResist"
    },
    "Subclass": "TargetPracticeEnemy"
  },
  "TargetPracticeSelfModifier": {
    "Class": "TargetPracticeSelf",
    "Subclass": "TargetPracticeSelf"
  },
  "Upgrades": [
    {
      "AbilityCooldown": -19.0
    },
    {
      "DamageOnBuildup": 50
    },
    {
      "BulletArmorReduction": -30,
      "DebuffDuration": 6
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
