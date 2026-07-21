---
title: "Rabbit Hex"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_magician_animalhexarea"
canonical_name: "Rabbit Hex"
snapshot_id: 39473
source_document_id: 7070
payload_hash: "d57ce14fd2a88affc8f1246ad6a72ad7f36f0de09c90c189875d0fc3f551d856"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.423736+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Rabbit Hex

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_animalhexarea`
- Snapshot ID: `39473`
- Source-Dokument: `7070`
- Kurzinfo: Rabbit Hex aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 24,
  "AbilityCooldown": 26,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AirDampingDuration": 1,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DamageAmpPercentage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0558
    },
    "Value": 15
  },
  "DetonationDelay": 0.9,
  "HexAreaModifier": {
    "Class": "MagicianAnimalcurseHexarea",
    "HexModifier": {
      "Class": "CitadelAnimalcurse",
      "ModelScale": 0.75,
      "Subclass": "AnimalhexCurseModifier"
    },
    "Subclass": "HexareaModifier"
  },
  "HexDuration": 2,
  "HexMoveSpeedLimit": 6,
  "IsDisabled": false,
  "Key": "ability_magician_animalhexarea",
  "MoveSpeedBonusPct": 36,
  "Name": "Rabbit Hex",
  "Radius": 6.5,
  "SelfBumpImpulse": 500,
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "HexDuration": 1
    },
    {
      "DamageAmpPercentage": 7,
      "Radius": 3
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
  "_deadlock_data_card": {
    "card_name": "Rabbit Hex",
    "hero_key": "hero_magician",
    "hero_name": "Sinclair",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_magician",
      "hero_name": "Sinclair",
      "lookup": "rabbit hex",
      "name": "Rabbit Hex",
      "type": "ability"
    }
  ]
}
````
