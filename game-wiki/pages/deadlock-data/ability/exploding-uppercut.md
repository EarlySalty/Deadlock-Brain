---
title: "Exploding Uppercut"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_uppercut"
canonical_name: "Exploding Uppercut"
snapshot_id: 39661
source_document_id: 7070
payload_hash: "e6dbfd8035e8683d9259d173d00e49c5372dfa40d673197453903c5b4d519db0"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.889639+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Exploding Uppercut

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_uppercut`
- Snapshot ID: `39661`
- Source-Dokument: `7070`
- Kurzinfo: Exploding Uppercut aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 22.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BonusFireRate": {
    "Scale": {
      "Type": "spirit",
      "Value": -0.186
    },
    "Value": -14
  },
  "BuffGunRangePercent": 100,
  "BuffModifier": {
    "Class": "UppercutBuff",
    "EnabledStateMask": [
      "NoWindup"
    ],
    "Subclass": "UppercutBuff"
  },
  "ChannelMoveSpeed": -1,
  "ClipModifier": {
    "Class": "UppercutClipsize",
    "Subclass": "UppercutClipsize"
  },
  "EnemyHeroTossVelocity": 20,
  "ExplodeDebuffDuration": 5,
  "ForceReductionOnAngleDown": 0.75,
  "IsDisabled": false,
  "Key": "citadel_ability_uppercut",
  "LandingDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 75
  },
  "MeleeAttackLength": 6,
  "MeleeHalfAngle": 60,
  "MeleeRadius": 2.5,
  "Name": "Exploding Uppercut",
  "OnLandDamageRadius": 14,
  "TossDuration": 0.5,
  "TossDurationFriendly": 0.3,
  "TossVelocity": 25,
  "Upgrades": [
    {
      "AbilityCooldown": -11
    },
    {
      "BuffBaseWeaponPct": 30,
      "UppercutBuffOnHit": 9
    },
    {
      "MissingHPHeal": 18,
      "RestoreHookCooldown": 1
    }
  ],
  "UppercutDamage": {
    "Scale": {
      "Type": "melee",
      "Value": 1.0
    },
    "Value": 0.01
  },
  "UppercutModifier": {
    "Class": "CitadelUppercutted",
    "ExplodeDebuffModifier": {
      "Class": "Base",
      "Subclass": "UppercutDebuff"
    },
    "NoExplodeModifier": {
      "Class": "Base",
      "Subclass": "NoUppercutExplosionDamage"
    },
    "StatusEffectPriority": 20,
    "Subclass": "CitadelUppercutted"
  },
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
    "card_name": "Exploding Uppercut",
    "hero_key": "hero_bebop",
    "hero_name": "Bebop",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_bebop",
      "hero_name": "Bebop",
      "lookup": "exploding uppercut",
      "name": "Exploding Uppercut",
      "type": "ability"
    }
  ]
}
````
