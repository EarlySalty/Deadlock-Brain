---
title: "Crushing Leap"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_werewolf_leap"
canonical_name: "Crushing Leap"
snapshot_id: 39577
source_document_id: 7070
payload_hash: "1e1157dd8959f8d8f0a8491f0dd2757f6a0aa50c656bdfd011a1400570356d76"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.683314+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Crushing Leap

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_leap`
- Snapshot ID: `39577`
- Source-Dokument: `7070`
- Kurzinfo: Crushing Leap aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.35,
  "AbilityChannelTime": 3,
  "AbilityCooldown": 16,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 0.25,
  "AbilityUnitTargetLimit": 1,
  "AirDrag": 0.1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": 5.08,
  "Damage": {
    "Scale": {
      "Type": "melee",
      "Value": 1.9
    },
    "Value": 0
  },
  "DebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "Debuff"
  },
  "EnemyPushForceAway": 200,
  "EnemyPushForceUp": 300,
  "GravityScale": 1,
  "Height": 4,
  "IsDisabled": false,
  "Key": "ability_werewolf_leap",
  "LandingBonusesModifier": {
    "Class": "Base",
    "Subclass": "Buff"
  },
  "LeapCameraSpeed": 300,
  "LeapForwardSpeed": 800,
  "LeapInputSpeed": 250,
  "LeapMultiHitRadius": 1.5,
  "LeapRadius": 2.54,
  "LeapUpSpeed": 200,
  "LeapingModifier": {
    "Class": "CitadelWerewolfLeaping",
    "Subclass": "Leaping"
  },
  "Name": "Crushing Leap",
  "Radius": 7.5,
  "SlowPercent": 30,
  "Upgrades": [
    {
      "BonusMoveSpeed": 2,
      "LandingBonusesDuration": 5
    },
    {
      "AbilityCooldown": -9
    },
    {
      "StunDuration": 1.2
    }
  ],
  "WorldImpactRadius": 25,
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
