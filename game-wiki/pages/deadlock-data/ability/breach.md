---
title: "Breach"
entity_type: "ability"
source: "deadlock_data"
external_id: "fathom_breach"
canonical_name: "Breach"
snapshot_id: 39675
source_document_id: 7070
payload_hash: "1a4b640dd0c743efe98943d6ff52ae2dcca963b8e74d28ff0c35343cd42f5ac2"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.921019+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Breach

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `fathom_breach`
- Snapshot ID: `39675`
- Source-Dokument: `7070`
- Kurzinfo: Breach aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 20,
  "AbilityCooldown": 22.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorMovement"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.651
    },
    "Value": 80
  },
  "ExplosionRadius": 6,
  "GravityScale": 1.4,
  "InFlightModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "DashDisabled"
    ],
    "Subclass": "InFlight"
  },
  "IsDisabled": false,
  "Key": "fathom_breach",
  "Name": "Breach",
  "TossSpeed": 350,
  "Upgrades": [
    {
      "ExplosionRadius": 3
    },
    {
      "AbilityCooldown": -8
    },
    {
      "Damage": 120
    }
  ],
  "WallImpactLookAheadDistance": 100,
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
    "card_name": "Breach",
    "hero_key": "hero_slork",
    "hero_name": "Fathom",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_slork",
      "hero_name": "Fathom",
      "lookup": "breach",
      "name": "Breach",
      "type": "ability"
    }
  ]
}
````
