---
title: "Rabbit Hex"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_magician_animalcurse"
canonical_name: "Rabbit Hex"
snapshot_id: 39472
source_document_id: 7070
payload_hash: "0ad91c04ac5471de4b2347c5f6112f44ccf95041fe43d65b7c475378c5314adb"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.420794+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Rabbit Hex

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_animalcurse`
- Snapshot ID: `39472`
- Source-Dokument: `7070`
- Kurzinfo: Rabbit Hex aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.35,
  "AbilityCastRange": 20,
  "AbilityCooldown": 45,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 2,
  "AbilityPostCastDuration": 0.1,
  "AbilityUnitTargetLimit": 1,
  "AirDampingDuration": 1,
  "AirDampingModifier": {
    "Class": "Airdamp",
    "Subclass": "AirdampAnimalcurse"
  },
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorAllowSelfCast",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "CurseModifier": {
    "Class": "CitadelAnimalcurse",
    "ModelScale": 0.75,
    "Subclass": "AnimalhexCurseModifier"
  },
  "DamageAmpPercentage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.093
    },
    "Value": 25
  },
  "HexMoveSpeedLimit": 6,
  "IsDisabled": false,
  "Key": "ability_magician_animalcurse",
  "MoveSpeedBonusPct": 35,
  "Name": "Rabbit Hex",
  "SelfBumpImpulse": 500,
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "AbilityDuration": 1
    },
    {
      "Radius": 6
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
