---
title: "Itani Lo Sahn"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_fencer_ultimate"
canonical_name: "Itani Lo Sahn"
snapshot_id: 39436
source_document_id: 7070
payload_hash: "a99120a97a6a1cc8de8cc853c53a36bdb3ec8f0f93b5065ad8e3ab5d7b719b3a"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.319945+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Itani Lo Sahn

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_fencer_ultimate`
- Snapshot ID: `39436`
- Source-Dokument: `7070`
- Kurzinfo: Itani Lo Sahn aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1.5,
  "AbilityChannelTime": 9999,
  "AbilityCooldown": 145,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "AirSpeedMax": 70,
  "AutoCastDelayModifier": {
    "Class": "Base",
    "Subclass": "FencerUltCastDelay"
  },
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorNoTarget",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusDamagePercent": 60,
  "CameraDistance": 250,
  "CasterArrivalModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "SinclairTaxUltActive",
      "HideCrosshair",
      "HideStamina",
      "HideAmmo",
      "CameraTransitionAlways"
    ],
    "Subclass": "FencerUltimateCasterArrival"
  },
  "CasterLockDuration": 1.8,
  "CasterModifier": {
    "Class": "FencerUltimateCaster",
    "EnabledStateMask": [
      "SinclairTaxUltActive",
      "HideCrosshair",
      "HideStamina",
      "HideAmmo",
      "CameraTransitionAlways"
    ],
    "Subclass": "FencerUltimateCaster"
  },
  "DashAngleThreshold": 89,
  "DashRadius": 7,
  "DashRange": 27,
  "DashSpeed": 254.0,
  "DebuffDuration": 1.8,
  "DelayedDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.6
    },
    "Value": 200
  },
  "FallSpeedMax": 1,
  "GapDistanceToWall": 180,
  "GroundDashReductionPercent": -30,
  "ImpactDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.77
    },
    "Value": 70
  },
  "IncomingDamageReductionPercent": 70,
  "IsDisabled": false,
  "Key": "ability_fencer_ultimate",
  "LowHealthEnemyThresholdPct": 50,
  "MoveSpeedPenaltyMaxSpeed": 200,
  "Name": "Itani Lo Sahn",
  "SideMoveSpeedReduction": -100,
  "TargetModifier": {
    "Class": "FencerUltimateTarget",
    "DamageTimeOffset": 0.3,
    "EnabledStateMask": [
      "ModNoCleanse",
      "HealingDisabled",
      "Disarmed",
      "Silenced",
      "Muted",
      "DashDisabledDebuff",
      "SilenceMovementAbilites"
    ],
    "EndTimeScaleForFlinch": 0.9,
    "StatusEffectPriority": 50,
    "Subclass": "Cursed"
  },
  "TargetNonHeroModifier": {
    "Class": "FencerUltimateTarget",
    "DamageTimeOffset": 0.3,
    "EnabledStateMask": [
      "HealingDisabled",
      "Disarmed",
      "Silenced",
      "Muted",
      "DashDisabledDebuff",
      "SilenceMovementAbilites"
    ],
    "EndTimeScaleForFlinch": 0.9,
    "StatusEffectPriority": 50,
    "Subclass": "CursedNonHero"
  },
  "TechCleaveExpireTime": 0.35,
  "TimeScaleDebuff": 70,
  "TimerSoundDuration": 1,
  "TravelDistPctBeforeWallGapCheck": 70,
  "TurnRateMaxDuringCast": 999,
  "Upgrades": [
    {
      "DashRange": 8
    },
    {
      "AbilityCooldown": -35
    },
    {
      "BonusDamagePercent": 50
    }
  ],
  "VacuumSpeed": 10.16,
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
    "card_name": "Itani Lo Sahn",
    "hero_key": "hero_fencer",
    "hero_name": "Apollo",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_fencer",
      "hero_name": "Apollo",
      "lookup": "itani lo sahn",
      "name": "Itani Lo Sahn",
      "type": "ability"
    }
  ]
}
````
