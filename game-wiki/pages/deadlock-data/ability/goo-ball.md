---
title: "Goo Ball"
entity_type: "ability"
source: "deadlock_data"
external_id: "viscous_goo_bowling_ball"
canonical_name: "Goo Ball"
snapshot_id: 39734
source_document_id: 7070
payload_hash: "a53b7d4bbb07e6433debcf6916165acc081c2b317cefaf9123f4383f1a8777d7"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.079863+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Goo Ball

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `viscous_goo_bowling_ball`
- Snapshot ID: `39734`
- Source-Dokument: `7070`
- Kurzinfo: Goo Ball aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.55,
  "AbilityCooldown": 150,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 11,
  "AbilityUnitTargetLimit": 1,
  "AccelerationPercentage": -60,
  "AirJumpForce": 500,
  "BallHitRadius": 1.8,
  "BallOffset": 50,
  "BallRadius": 1.4,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast",
    "BehaviorRequireAbilityButtonToCancel",
    "BehaviorInhibitSoftCameraCollision"
  ],
  "BreakablePropDamageRadius": 75,
  "BulletResist": 35,
  "CastWhileRolling": 1,
  "ChannelMoveSpeed": 7,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1
    },
    "Value": 110
  },
  "DamagePreventionModifier": {
    "Class": "Base",
    "Duration": 1.2,
    "Subclass": "ViscousBallDamagePrevention"
  },
  "FrictionPercentage": -85,
  "IsDisabled": false,
  "JumpForce": 500,
  "Key": "viscous_goo_bowling_ball",
  "KnockForce": 400,
  "MoveSpeedMax": 7,
  "Name": "Goo Ball",
  "ParticleRadiusMultiplier": 1.2,
  "RollingModifier": {
    "Class": "ViscousBall",
    "Subclass": "ViscousRollingResist"
  },
  "StunDuration": 0.5,
  "TechResist": 35,
  "TickRate": 0.25,
  "Upgrades": [
    {
      "AbilityCooldown": -25
    },
    {
      "BulletResist": 10,
      "Damage": 70,
      "TechResist": 10
    },
    {
      "AbilityDuration": 7,
      "StunDuration": 0.3
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
    "card_name": "Goo Ball",
    "hero_key": "hero_viscous",
    "hero_name": "Viscous",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_viscous",
      "hero_name": "Viscous",
      "lookup": "goo ball",
      "name": "Goo Ball",
      "type": "ability"
    }
  ]
}
````
