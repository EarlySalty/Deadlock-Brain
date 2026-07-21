---
title: "Dying Star"
entity_type: "ability"
source: "deadlock_data"
external_id: "tokamak_dying_star"
canonical_name: "Dying Star"
snapshot_id: 39724
source_document_id: 7070
payload_hash: "3adbaab781dd8ca075415ebfadedffe54ee44d25bf9641c0c57de4f46b00105c"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.053516+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Dying Star

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `tokamak_dying_star`
- Snapshot ID: `39724`
- Source-Dokument: `7070`
- Kurzinfo: Dying Star aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 20,
  "AbilityCooldown": 25.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.487469
    },
    "Value": 80
  },
  "ExplosionRadius": 6,
  "GravityScale": 1.4,
  "InFlightModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "CommandRestricted"
    ],
    "Subclass": "InFlight"
  },
  "IsDisabled": false,
  "Key": "tokamak_dying_star",
  "Name": "Dying Star",
  "TossSpeed": 350,
  "Upgrades": [
    {
      "ExplosionRadius": 3
    },
    {
      "AbilityCooldown": -7.5
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.487469
        },
        "Value": 80
      }
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
      "lookup": "dying star",
      "name": "Dying Star",
      "type": "ability"
    }
  ]
}
````
