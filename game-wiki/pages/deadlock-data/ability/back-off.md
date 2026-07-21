---
title: "Back Off!"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_necro_coffin"
canonical_name: "Back Off!"
snapshot_id: 39494
source_document_id: 7070
payload_hash: "2eca31158636da8b4e6861bcb313661f1d8303e232b36f4bc5e2a77b496f86f2"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.474537+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Back Off!

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_necro_coffin`
- Snapshot ID: `39494`
- Source-Dokument: `7070`
- Kurzinfo: Back Off! aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 28,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BonusMoveSpeedPercent": 15,
  "BuffDuration": {
    "Scale": {
      "Type": "duration",
      "Value": 1.0
    },
    "Value": 4
  },
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "Buff"
  },
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 65
  },
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 65
  },
  "DebuffDuration": 3,
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "Debuff"
  },
  "ImmobilizeModifier": {
    "Class": "CitadelRoot",
    "Subclass": "Immobilize"
  },
  "IsDisabled": false,
  "Key": "ability_necro_coffin",
  "Name": "Back Off!",
  "Radius": 9,
  "SlowPercent": 40,
  "Upgrades": [
    {
      "CombatBarrier": 50
    },
    {
      "AbilityCooldown": -6
    },
    {
      "BonusMoveSpeedPercent": 15
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
