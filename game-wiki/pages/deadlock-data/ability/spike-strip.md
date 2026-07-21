---
title: "Spike Strip"
entity_type: "ability"
source: "deadlock_data"
external_id: "thumper_ability_2"
canonical_name: "Spike Strip"
snapshot_id: 39719
source_document_id: 7070
payload_hash: "ba10ef3954fa65015b5864b3a8747589febdce0bc625d77a24e84ffe432e7ea7"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.036199+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Spike Strip

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `thumper_ability_2`
- Snapshot ID: `39719`
- Source-Dokument: `7070`
- Kurzinfo: Spike Strip aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.42,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "BarbedWireAuraModifier": {
    "Class": "ThumperAbility2Aura",
    "ProvidedByAura": {
      "Class": "ThumperAbility2",
      "EnabledStateMask": [
        "Slowed",
        "GlowThroughWallsToEnemy"
      ],
      "StatusEffectPriority": 100,
      "Subclass": "ThumperAbility2"
    },
    "Subclass": "ThumperAbility2Aura"
  },
  "BarbedWireDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 10
  },
  "BarbedWireDamagePerMeter": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 30
  },
  "BarbedWireHeightOffGround": 1,
  "BarbedWireRadius": 4,
  "BarbedWireSlow": 50,
  "BarbedWireTickRate": 0.5,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 125
  },
  "ImpactInterval": 0.1,
  "IsDisabled": false,
  "Key": "thumper_ability_2",
  "Name": "Spike Strip",
  "StompRange": 25,
  "TechCleaveExpireTime": 0.2,
  "Upgrades": [
    {
      "AbilityDuration": 2
    },
    {
      "BarbedWireRadius": 3
    },
    {
      "BarbedWireDamagePerMeter": 30
    }
  ],
  "VerticalDifferenceTolerance": 2.5,
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
      "hero_key": "hero_thumper",
      "hero_name": "Thumper",
      "lookup": "spike strip",
      "name": "Spike Strip",
      "type": "ability"
    }
  ]
}
````
