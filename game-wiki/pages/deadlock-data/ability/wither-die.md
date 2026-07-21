---
title: "Wither & Die"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_necro_wither"
canonical_name: "Wither & Die"
snapshot_id: 39502
source_document_id: 7070
payload_hash: "f75a506edd1437a45ea2ee1195b4fd5e7bd003fcd4a9834b87a1d00bf125c899"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.492154+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Wither & Die

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_necro_wither`
- Snapshot ID: `39502`
- Source-Dokument: `7070`
- Kurzinfo: Wither & Die aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AreaModifier": {
    "Class": "NecroHauntingskullArea",
    "InitialRandomVariance": 30.0,
    "SpawnPositionNavMeshSearchRange": 60.0,
    "Subclass": "Area"
  },
  "BehaviourBits": null,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.3
    },
    "Value": 20
  },
  "DelayBeforeRespawning": 1,
  "IsDisabled": false,
  "Key": "ability_necro_wither",
  "MaxHits": -1,
  "MaxStacks": 40,
  "Name": "Wither & Die",
  "SkullImmuneDuration": 0.15,
  "SkullLifetime": 10,
  "SkullsOnKill": 1,
  "SkullsOnPlayerKill": 3,
  "SlowPercentPerStack": 0.5,
  "SpawnRadius": 2,
  "StackDuration": 4,
  "StackingDebuffModifier": {
    "Class": "NecroHauntingskullStackingdebuff",
    "Subclass": "Stackingdebuff"
  },
  "SummonBuffModifier": {
    "Class": "Base",
    "Subclass": "Summonbuff"
  },
  "SummonHealth": {
    "Scale": {
      "Type": "power_increase",
      "Value": 2.0
    },
    "Value": 20
  },
  "SummonModifier": {
    "Class": "BarrierTracker",
    "Subclass": "Barriertracker"
  },
  "TargetDashRadius": 13,
  "TargetSearchDelayMax": 0.88,
  "TargetSearchDelayMin": 0.65,
  "TargetSearchRadius": 5,
  "TechArmorDamageReductionPerStack": -0.5,
  "TickRate": 0.3,
  "Upgrades": [
    {
      "SlowPercentPerStack": 0.5
    },
    {
      "MaxStacks": 20
    },
    {
      "StacksToProcSkull": 10
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
