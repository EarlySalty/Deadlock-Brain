---
title: "Malice"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_blood_shards"
canonical_name: "Malice"
snapshot_id: 39397
source_document_id: 7070
payload_hash: "3a84248ceb1079cd470e131f3f3c79bdd4076412c0f9bf910d5b0b0e1e1fa3f5"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.224679+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Malice

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_blood_shards`
- Snapshot ID: `39397`
- Source-Dokument: `7070`
- Kurzinfo: Malice aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.12,
  "AbilityCooldown": 6,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.3,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DebuffDuration": 9,
  "DebuffModifier": {
    "Class": "BloodShardDebuff",
    "Subclass": "BloodShardDebuff"
  },
  "HealthToDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.558
    },
    "Value": 23.0
  },
  "IsDisabled": false,
  "Key": "ability_blood_shards",
  "MaxStacks": 5,
  "MoveSpeedPenaltyPerStack": 15,
  "Name": "Malice",
  "NumBloodShards": 3,
  "SelfDamagePct": 9,
  "SlowDuration": 4,
  "SpreadAngleDegrees": 6,
  "Upgrades": [
    {
      "AbilityCooldown": -3
    },
    {
      "HealthToDamage": 25.2,
      "NumBloodShards": 4,
      "SpreadAngleDegrees": 22
    },
    {
      "VulnerabilityPerStack": 7
    }
  ],
  "VulnerabilityPerStack": 8,
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
  "_deadlock_data_card": {
    "card_name": "Malice",
    "hero_key": "hero_ghost",
    "hero_name": "Lady Geist",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_ghost",
      "hero_name": "Lady Geist",
      "lookup": "malice",
      "name": "Malice",
      "type": "ability"
    }
  ]
}
````
