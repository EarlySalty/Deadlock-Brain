---
title: "Dust Devil"
entity_type: "ability"
source: "deadlock_data"
external_id: "mirage_tornado"
canonical_name: "Dust Devil"
snapshot_id: 39695
source_document_id: 7070
payload_hash: "b277dea249250d142c1eeab0674032c741eaef2d22260c63dd7d21600e4e80d0"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.972262+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Dust Devil

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `mirage_tornado`
- Snapshot ID: `39695`
- Source-Dokument: `7070`
- Kurzinfo: Dust Devil aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 20,
  "AbilityCooldown": 36.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontInterruptSprint",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "ClimbHeight": 1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.3
    },
    "Value": 65
  },
  "DampingFactor": 0.1,
  "DistanceAboveGround": 0.5,
  "DropDownRate": 10,
  "EnemyLiftDuration": 0.2,
  "HoldInPlaceDuration": 0.3,
  "IsDisabled": false,
  "Key": "mirage_tornado",
  "LiftHeight": 3,
  "MaxDeltaMovementControl": 2,
  "Name": "Dust Devil",
  "OpenHeight": 8,
  "ProjectileThinkInterval": 0.01,
  "Radius": 4,
  "SlowDuration": 3,
  "SlowPercent": 30,
  "TickRate": 0.25,
  "TornadoSpeed": 24,
  "Upgrades": [
    {
      "Damage": 60
    },
    {
      "AbilityCooldown": -12,
      "WhirlwindEvasionChance": 30
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 0
      },
      "HoldInPlaceDuration": 0.3,
      "RecastWindow": 6
    }
  ],
  "WhirlwindDuration": 4,
  "WhirlwindEvasionChance": 30,
  "WhirlwindEvasionModifier": {
    "Class": "MirageTornadoEvasion",
    "Subclass": "MirageTornadoEvasion"
  },
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
    "card_name": "Dust Devil",
    "hero_key": "hero_mirage",
    "hero_name": "Mirage",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_mirage",
      "hero_name": "Mirage",
      "lookup": "dust devil",
      "name": "Dust Devil",
      "type": "ability"
    }
  ]
}
````
