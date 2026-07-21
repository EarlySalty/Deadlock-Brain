---
title: "Pulse Cannon"
entity_type: "ability"
source: "deadlock_data"
external_id: "tokamak_crimson_cannon"
canonical_name: "Pulse Cannon"
snapshot_id: 39723
source_document_id: 7070
payload_hash: "380c21fe37817fabed90df775d36e947446c242d00703e7b1e1cbc5e87bbea95"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.049374+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Pulse Cannon

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `tokamak_crimson_cannon`
- Snapshot ID: `39723`
- Source-Dokument: `7070`
- Kurzinfo: Pulse Cannon aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 2.0,
  "AbilityCastRange": 100,
  "AbilityChannelTime": 3.6,
  "AbilityCooldown": 127.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AimFOV": 60,
  "AimZoomDuration": 0.15,
  "AirSpeedMax": 70,
  "AutoCastDelayModifier": {
    "Class": "Base",
    "Subclass": "CastDelay"
  },
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.462406
    },
    "Value": 190
  },
  "DelayBetweenShots": 0.6,
  "FallSpeedMax": 1,
  "IsDisabled": false,
  "Key": "tokamak_crimson_cannon",
  "Name": "Pulse Cannon",
  "TargetingWidth": 0.8,
  "Upgrades": [
    {
      "DelayBetweenShots": -0.2
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.462406
        },
        "Value": 50
      }
    },
    {
      "AbilityCooldown": -47.0
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
      "hero_key": "hero_tokamak",
      "hero_name": "Tokamak",
      "lookup": "pulse cannon",
      "name": "Pulse Cannon",
      "type": "ability"
    }
  ]
}
````
