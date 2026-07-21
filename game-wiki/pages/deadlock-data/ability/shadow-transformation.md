---
title: "Shadow Transformation"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_infinity_slash"
canonical_name: "Shadow Transformation"
snapshot_id: 39619
source_document_id: 7070
payload_hash: "0995c11e3658eaf887f1fde51f1dfba78765ac37eaa272daee6c9c2019ef12e2"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.789780+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Shadow Transformation

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_infinity_slash`
- Snapshot ID: `39619`
- Source-Dokument: `7070`
- Kurzinfo: Shadow Transformation aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChannelTime": 1.5,
  "AbilityCooldown": 150.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilitySpeedPct": 60,
  "AbilityUnitTargetLimit": 1,
  "AutoChannelModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "Invulnerable",
      "TechUntargetable",
      "UnitStatusHealthHidden",
      "IgnoreBullets",
      "IgnoreMelee",
      "StatusImmune",
      "SlowImmune",
      "KnockdownImmune",
      "Unstoppable"
    ],
    "Subclass": "Channel"
  },
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCleaveDisabled",
    "BehaviorCanCancelDuringCastDelay",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BuffModifier": {
    "Class": "Base",
    "DisabledStateMask": [
      "Disarmed",
      "Muted",
      "Silenced",
      "SilenceMovementAbilites",
      "Slowed",
      "Glitched",
      "MeleeDisabledDebuff",
      "DashDisabledDebuff"
    ],
    "EnabledStateMask": [
      "YamatoShadowForm",
      "StatusImmune",
      "SlowImmune",
      "KnockdownImmune",
      "Unstoppable"
    ],
    "StatusEffectPriority": 0,
    "Subclass": "Buff"
  },
  "BuffTimerModifier": {
    "Class": "YamatoInfinitySlashBuffTimer",
    "Subclass": "Timer"
  },
  "BulletResist": 30,
  "IsDisabled": false,
  "Key": "citadel_ability_infinity_slash",
  "MaxHealthRegen": 15,
  "Name": "Shadow Transformation",
  "ShadowFormDurationOnKill": 2.0,
  "TechResist": 30,
  "Upgrades": [
    {
      "WeaponDamageBonus": 7
    },
    {
      "AbilityCooldown": -20,
      "BonusMoveSpeed": 4
    },
    {
      "AbilityDuration": 3.0,
      "BulletResist": 30,
      "TechResist": 30
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
    "card_name": "Shadow Transformation",
    "hero_key": "hero_yamato",
    "hero_name": "Yamato",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_yamato",
      "hero_name": "Yamato",
      "lookup": "shadow transformation",
      "name": "Shadow Transformation",
      "type": "ability"
    }
  ]
}
````
