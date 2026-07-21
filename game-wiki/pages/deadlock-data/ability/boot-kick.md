---
title: "Boot Kick"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_werewolf_kickflip"
canonical_name: "Boot Kick"
snapshot_id: 39576
source_document_id: 7070
payload_hash: "547d4274e35a7af95464961546eff048b2f6aa59d216366b6dc3d227d8e13f01"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.680966+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Boot Kick

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_kickflip`
- Snapshot ID: `39576`
- Source-Dokument: `7070`
- Kurzinfo: Boot Kick aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCastRange": 10.6,
  "AbilityChannelTime": 0.35,
  "AbilityCooldown": 21,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.25,
  "AbilityUnitTargetLimit": 1,
  "AirDrag": 0.8,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BonusDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.0
    },
    "Value": 25
  },
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "Buff"
  },
  "CameraTurnRateMax": 188,
  "Damage": {
    "Scale": {
      "Type": "melee",
      "Value": 0.9
    },
    "Value": 0
  },
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "Debuff"
  },
  "DisarmModifier": {
    "Class": "CitadelDisarmed",
    "Subclass": "Disarm"
  },
  "EnemyPushForceAway": 300,
  "EnemyPushForceUp": 300,
  "FallSpeedMax": 20,
  "IsDisabled": false,
  "Key": "ability_werewolf_kickflip",
  "LeapForwardOffset": 2.5,
  "LeapRadius": 1.7,
  "LeapingModifier": {
    "Class": "CitadelWerewolfLeaping",
    "Subclass": "Leaping"
  },
  "MarkDuration": {
    "Scale": {
      "Type": "duration",
      "Value": 1.0
    },
    "Value": 3
  },
  "MarkModifier": {
    "Class": "WerewolfKickflipBonusdamage",
    "Subclass": "Bonusdamage"
  },
  "Name": "Boot Kick",
  "SelfPushForceCameraAway": 600,
  "SelfPushForceUp": 200,
  "SlowDuration": 0.1,
  "SuccessEnemyModifier": {
    "Class": "CitadelRoot",
    "Subclass": "Enemysuccess"
  },
  "SuccessInputWindow": 0.3,
  "SuccessSelfModifier": {
    "Class": "WerewolfKickflipSuccessSelf",
    "EnabledStateMask": [
      "Immobilized",
      "CommandRestricted"
    ],
    "Subclass": "Selfsuccess"
  },
  "TimeScaleDebuff": 95,
  "Upgrades": [
    {
      "AbilityCooldown": -6
    },
    {
      "StaminaRestore": 2
    },
    {
      "BonusDamage": 80,
      "DebuffDuration": 5,
      "OutgoingDamagePercent": -35
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
    "card_name": "Boot Kick",
    "hero_key": "hero_werewolf",
    "hero_name": "Silver",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_werewolf",
      "hero_name": "Silver",
      "lookup": "boot kick",
      "name": "Boot Kick",
      "type": "ability"
    }
  ]
}
````
