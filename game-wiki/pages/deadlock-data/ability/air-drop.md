---
title: "Air Drop"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_tengu_airlift"
canonical_name: "Air Drop"
snapshot_id: 39652
source_document_id: 7070
payload_hash: "8f7829cc86d23692abde3eaae3f643fd8989dd8cddac201dbd618a8a3dc78f18"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.867495+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Air Drop

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tengu_airlift`
- Snapshot ID: `39652`
- Source-Dokument: `7070`
- Kurzinfo: Air Drop aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 22,
  "AbilityCooldown": 100.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 21.0,
  "AbilityUnitTargetLimit": 1,
  "AirDropBulletShield": {
    "Scale": {
      "Type": "spirit",
      "Value": 0
    },
    "Value": 0
  },
  "AirDropOutgoingDamagePercent": 20,
  "AllyCastDelay": 0.1,
  "AllyOutgoingDamagePercent": -20,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorAllowSelfCast",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorMovement",
    "BehaviorRequireAbilityButtonToCancel",
    "BehaviorCanSetQuickCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BuffDuration": 8,
  "BulletResistModifier": {
    "Class": "Base",
    "Subclass": "AirdropBulletresist"
  },
  "ChannelMoveSpeed": 1.3,
  "CooldownReductionPctOnOthers": 30,
  "DroppedBuffModifier": {
    "Class": "Base",
    "Subclass": "BarrierModifier"
  },
  "ExplodeDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.7
    },
    "Value": 115.0
  },
  "ExplodingAllyModifier": {
    "Class": "AirliftExplodingAlly",
    "Subclass": "AirliftExplodingAlly"
  },
  "FlyingModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "AbilityMovement",
      "Sprinting",
      "DashDisabled",
      "ShootingDisabled",
      "JumpDisabled",
      "DuckingDisabled",
      "MantleDisabled",
      "AimForwardWithPitch",
      "MeleeDisabled",
      "SlidingDisabled",
      "HideCrosshair"
    ],
    "Subclass": "Base"
  },
  "GrabModifier": {
    "AllyGrabCancelTime": 1.0,
    "AllyPossibleStuckDistance": 320,
    "Class": "AirliftGrab",
    "EnabledStateMask": [
      "AbilityMovement",
      "ShootingDisabled",
      "MeleeDisabled",
      "IgnorePortals"
    ],
    "FollowDampingFactor": 20,
    "FollowDistance": -60,
    "LiftHeight": -120,
    "LiftHorizontal": 0,
    "Subclass": "AirliftGrab"
  },
  "InterruptCooldown": 3.5,
  "IsDisabled": false,
  "Key": "citadel_ability_tengu_airlift",
  "Name": "Air Drop",
  "OnLandDamageRadius": 20,
  "OnLandDamageRadiusStart": 16,
  "SilenceBombSpeed": 12,
  "SilenceModifier": {
    "Class": "CitadelSilenced",
    "Subclass": "CitadelSilenced"
  },
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "AirliftSlow"
  },
  "Upgrades": [
    {
      "AirDropBulletShield": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.7
        },
        "Value": 300
      }
    },
    {
      "DebuffDuration": 3,
      "SlowPercent": 40
    },
    {
      "AirDropBulletShield": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 0
      },
      "ExplodeDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.5
        },
        "Value": 0
      },
      "SilenceDuration": 3
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
    "card_name": "Air Drop",
    "hero_key": "hero_tengu",
    "hero_name": "Ivy",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_tengu",
      "hero_name": "Ivy",
      "lookup": "air drop",
      "name": "Air Drop",
      "type": "ability"
    }
  ]
}
````
