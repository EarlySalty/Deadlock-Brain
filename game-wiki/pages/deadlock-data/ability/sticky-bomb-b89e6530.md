---
title: "Sticky Bomb"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_sticky_bomb"
canonical_name: "Sticky Bomb"
snapshot_id: 39647
source_document_id: 7070
payload_hash: "3d395adb21b545a41f83a0ecdc0dd82e1045fe22ea3fc44bf6562abd8c90f712"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.855987+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Sticky Bomb

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_sticky_bomb`
- Snapshot ID: `39647`
- Source-Dokument: `7070`
- Kurzinfo: Sticky Bomb aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 6,
  "AbilityCooldown": 18.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 3.5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAllowSelfCast",
    "BehaviorDisplaysDamageImpact",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "BombAttachedModifier": {
    "Class": "CitadelStickyBombAttached",
    "OnGroundModifier": {
      "Class": "CitadelStickyBombOnGround",
      "OnGroundModifier": {
        "Class": "Base",
        "Subclass": "Empty"
      },
      "Subclass": "CitadelStickyBombOnGround"
    },
    "Subclass": "CitadelStickyBombAttached"
  },
  "BonusDamagePctPerPlayerHit": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0015
    },
    "Value": 1.0
  },
  "BonusDamagePctPerPlayerKilled": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.01
    },
    "Value": 2.5
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.5
    },
    "Value": 85
  },
  "FuseTime": 3.5,
  "IsDisabled": false,
  "Key": "citadel_ability_sticky_bomb",
  "KillCheckModifier": {
    "Class": "Base",
    "Subclass": "KillcheckModifier"
  },
  "KillCheckWindow": 10.0,
  "Name": "Sticky Bomb",
  "OnHitDiminish": 60,
  "OnKillDiminish": 7,
  "Radius": 8,
  "SelfBuffModifier": {
    "Class": "Base",
    "Subclass": "CitadelStickyBombSelfBuff"
  },
  "SelfDamagePercent": 20,
  "Upgrades": [
    {
      "AbilityCooldown": -8
    },
    {
      "Damage": 85
    },
    {
      "MovementSpeedBonus": 5,
      "MovementSpeedBonusDuration": 6,
      "StatusResistancePercent": 25
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
    "card_name": "Sticky Bomb",
    "hero_key": "hero_bebop",
    "hero_name": "Bebop",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_bebop",
      "hero_name": "Bebop",
      "lookup": "sticky bomb",
      "name": "Sticky Bomb",
      "type": "ability"
    }
  ]
}
````
