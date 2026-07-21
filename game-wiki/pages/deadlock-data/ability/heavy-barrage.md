---
title: "Heavy Barrage"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_rocket_barrage"
canonical_name: "Heavy Barrage"
snapshot_id: 39636
source_document_id: 7070
payload_hash: "dd5019e6f31103d6bfd188628ea9c420a79eb91524c3917246cd231cce45cd16"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.830399+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Heavy Barrage

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_rocket_barrage`
- Snapshot ID: `39636`
- Source-Dokument: `7070`
- Kurzinfo: Heavy Barrage aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 36,
  "AbilityCooldown": 200.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 100,
  "BarrageModifier": {
    "Class": "CitadelRocketBarrageVolley",
    "EnabledStateMask": [
      "AllowDashWhenChanneling",
      "SinclairTaxUltActive",
      "Disarmed"
    ],
    "Subclass": "CitadelRocketBarrageVolley"
  },
  "BehaviourBits": [
    "BehaviorExclusiveUse",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCleaveDisabled",
    "BehaviorDeactivateCrouchToggleOnCast",
    "BehaviorCanSetQuickCast",
    "BehaviorRequireAbilityButtonToCancel",
    "BehaviorDontSwitchAwayOnCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DamagePerRocket": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.2
    },
    "Value": 21
  },
  "DetonateTimer": 5,
  "ExplosionFalloffDisabled": 1,
  "ExplosionRadius": 4.5,
  "GrenadesPerSecond": 6,
  "GroundDashReductionPercent": -35,
  "IntervalRampUpStart": 0.35,
  "IntervalRampUpTime": 0.3,
  "IsDisabled": false,
  "Key": "citadel_ability_rocket_barrage",
  "MaxSpread": 5,
  "MinDistance": 8.5,
  "MoveSlowModifier": {
    "Class": "SlowBase",
    "Subclass": "ForgeRocketBarrageSlow"
  },
  "Name": "Heavy Barrage",
  "ProjectileIgnoreCollisionTime": 0.2,
  "TrackSpeedFar": 100,
  "TrackSpeedNear": 150,
  "TrackingTime": 0.4,
  "Upgrades": [
    {
      "EnemyDashSlowPercent": -18,
      "MoveSlowDuration": 1,
      "MoveSlowPercent": 30
    },
    {
      "AbilityCooldown": -45.0,
      "AbilityDuration": 6
    },
    {
      "DamagePerRocket": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.16
        },
        "Value": 15
      },
      "ExplosionRadius": 2
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
    "card_name": "Heavy Barrage",
    "hero_key": "hero_forge",
    "hero_name": "McGinnis",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_forge",
      "hero_name": "McGinnis",
      "lookup": "heavy barrage",
      "name": "Heavy Barrage",
      "type": "ability"
    }
  ]
}
````
