---
title: "The Cube"
entity_type: "ability"
source: "deadlock_data"
external_id: "viscous_restorative_goo"
canonical_name: "The Cube"
snapshot_id: 39736
source_document_id: 7070
payload_hash: "71cf6613c4615a4a79fcf94a593565775a6755a22218c576768f50e0c1063c0f"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.085247+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# The Cube

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `viscous_restorative_goo`
- Snapshot ID: `39736`
- Source-Dokument: `7070`
- Kurzinfo: The Cube aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 26,
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 3,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorEqualUnitTargetPriority",
    "BehaviorAllowSelfCast",
    "BehaviorCanHealPlayers",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusHealthRegen": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.3
    },
    "Value": 40
  },
  "BreakoutTime": 1,
  "BulletForce": 600,
  "ChannelMoveSpeed": -1,
  "CubeScale": 1.5,
  "Friction": -80,
  "HeavyMeleeForce": 700,
  "IsDisabled": false,
  "Key": "viscous_restorative_goo",
  "LightMeleeForce": 300,
  "Name": "The Cube",
  "PostCubeBuffDuration": 8,
  "PushBackForce": 250,
  "PushBackRadius": 50,
  "RestorativeGooModifier": {
    "BreakoutProgressBarModifier": {
      "Class": "Base",
      "Subclass": "Breakoutprogress"
    },
    "Class": "RestorativeGoo",
    "DistanceCameraOffset": 450.0,
    "DistanceCameraOffsetBias": 0.75,
    "DistanceCameraOffsetLerpTime": 0.5,
    "PostCubeBuffModifier": {
      "Class": "Base",
      "Subclass": "CubePostSpeedBuff"
    },
    "Subclass": "RestorativeGoo"
  },
  "SelfCubeModelSwapModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "DoNotDrawModel"
    ],
    "Subclass": "ViscousSelfCubeModelSwap"
  },
  "SlideForce": 70,
  "Upgrades": [
    {
      "BonusMoveSpeed": 2.5,
      "PostCubeBuff": 1,
      "StaminaCooldownReduction": 30
    },
    {
      "AbilityDuration": 1,
      "BonusHealthRegen": 25
    },
    {
      "AbilityCooldown": -25.0,
      "PurgeDebuffs": 1
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
    "card_name": "The Cube",
    "hero_key": "hero_viscous",
    "hero_name": "Viscous",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_viscous",
      "hero_name": "Viscous",
      "lookup": "the cube",
      "name": "The Cube",
      "type": "ability"
    }
  ]
}
````
