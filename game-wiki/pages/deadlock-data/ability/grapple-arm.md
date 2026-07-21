---
title: "Grapple Arm"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_hook"
canonical_name: "Grapple Arm"
snapshot_id: 39614
source_document_id: 7070
payload_hash: "9a5355d581f9ce2abf28d027407778a11bfa14503fdccf9fdb3bb5b25c83536b"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.778180+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Grapple Arm

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hook`
- Snapshot ID: `39614`
- Source-Dokument: `7070`
- Kurzinfo: Grapple Arm aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 30,
  "AbilityCooldown": 23.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorPreventBotUsage",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BulletAmpModifier": {
    "Class": "BebopHookBulletAmp",
    "Subclass": "BebopHookBulletAmp"
  },
  "CancelHookDuration": 0.2,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "melee",
      "Value": 0.7
    },
    "Value": 0
  },
  "FriendlyHookIgnoreRange": 8,
  "HookImpactDelay": 0.5,
  "HookingSlowSpeedLimit": 5,
  "IsDisabled": false,
  "Key": "citadel_ability_hook",
  "Name": "Grapple Arm",
  "RestrictionDuration": 0.5,
  "SelfModifier": {
    "Class": "CitadelHookself",
    "EnabledStateMask": [
      "CastingHook"
    ],
    "Subclass": "CitadelHookself"
  },
  "SlowPercent": 90,
  "TargetModifier": {
    "Class": "CitadelHooktarget",
    "CloseEnoughDistance": 30.0,
    "FailSafeDurationMult": 2.0,
    "FailSafeMinTime": 1.0,
    "RestrictionModifier": {
      "Class": "SlowBase",
      "EnabledStateMask": [
        "SilenceMovementAbilites",
        "MovementAbilityRestricted",
        "DashDisabled",
        "Slowed"
      ],
      "Subclass": "SlowBases"
    },
    "ReturnPositionForwardOffset": 100.0,
    "ReturnSpeed": 2000.0,
    "ReturnSpeedFail": 100.0,
    "ReturnStuckTime": 0.2,
    "Subclass": "CitadelHooktarget",
    "TossUpSpeed": 0.0
  },
  "Upgrades": [
    {
      "BulletAmp": 20,
      "BulletAmpDuration": 6
    },
    {
      "AbilityCastRange": 30
    },
    {
      "AbilityCooldown": -11.5
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
    "card_name": "Grapple Arm",
    "hero_key": "hero_bebop",
    "hero_name": "Bebop",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_bebop",
      "hero_name": "Bebop",
      "lookup": "grapple arm",
      "name": "Grapple Arm",
      "type": "ability"
    }
  ]
}
````
