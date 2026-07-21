---
title: "Card Trick"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_card_toss"
canonical_name: "Card Trick"
snapshot_id: 39600
source_document_id: 7070
payload_hash: "7484c13c05ccb32174ccbe4e462692d70bd72f027b62ff179a8eaacaf82b89f0"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.741259+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Card Trick

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_card_toss`
- Snapshot ID: `39600`
- Source-Dokument: `7070`
- Kurzinfo: Card Trick aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Scale": {
      "Type": "range",
      "Value": 0.0
    },
    "Value": 500
  },
  "AbilityCharges": 2,
  "AbilityChargesConditionally": 1,
  "AbilityCooldown": {
    "Scale": {
      "Type": "cooldown",
      "Value": 0.0
    },
    "Value": 0.6
  },
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.1,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorAllowAltCast",
    "BehaviorAllowAltCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusAbilityResource": 100,
  "CardResourceGenPctScale": {
    "Scale": {
      "Type": "cooldown",
      "Value": -1.0
    },
    "Value": 85
  },
  "CardResourcePerBulletCrit": 6,
  "CardResourcePerBulletHit": 4,
  "CardResourcePerHeavyMelee": 25,
  "CardResourcePerLightMelee": 10,
  "ChannelMoveSpeed": -1,
  "ClubModifier": {
    "Class": "SlowBase",
    "Subclass": "ClubDebuff"
  },
  "ClubSlowDuration": 3,
  "ClubSlowPercent": -30,
  "CooldownBetweenCards": 0.5,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.55
    },
    "Value": 45
  },
  "DiamondModifier": {
    "Class": "CardtossStackingResistShred",
    "Subclass": "DiamondCardDebuff"
  },
  "DiamondResistShred": -8.0,
  "DiamondResistShredDuration": 5,
  "HeartHeal": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 75
  },
  "HeartHealNonHeroRatio": 0.5,
  "IsDisabled": false,
  "JokerExtraCardSearchRadius": 20,
  "Key": "citadel_ability_card_toss",
  "Name": "Card Trick",
  "NonPlayerCardResourceScale": 0.35,
  "ProjectileOriginHeightOffset": 50,
  "Radius": 4,
  "ResourcePerCard": 100,
  "SpadeDamageBonus": 60,
  "Upgrades": [
    {
      "AbilityCharges": 2
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.4
        },
        "Value": 40
      }
    },
    {
      "ClubSlowPercent": -20,
      "DiamondResistShred": -5,
      "HeartHeal": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 75
      },
      "ImprovedJokerChance": 1,
      "SpadeDamageBonus": 40
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
    "card_name": "Card Trick",
    "hero_key": "hero_wraith",
    "hero_name": "Wraith",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_wraith",
      "hero_name": "Wraith",
      "lookup": "card trick",
      "name": "Card Trick",
      "type": "ability"
    }
  ]
}
````
