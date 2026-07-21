---
title: "Bullet Dance"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_bullet_flurry"
canonical_name: "Bullet Dance"
snapshot_id: 39412
source_document_id: 7070
payload_hash: "79765ede01872654ef7835f0d93c8106fc45c69eb33b96583fff0b534f9eb4c9"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.259969+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Bullet Dance

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_bullet_flurry`
- Snapshot ID: `39412`
- Source-Dokument: `7070`
- Kurzinfo: Bullet Dance aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.4,
  "AbilityCooldown": 165.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.03
    },
    "Value": 3.5
  },
  "AbilityUnitTargetLimit": 1,
  "AutoCastDelayModifier": {
    "Class": "CitadelBulletFlurryWindup",
    "Subclass": "Cast"
  },
  "BehaviourBits": [
    "BehaviorExclusiveUse",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCleaveDisabled",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BonusFireRate": 25,
  "BulletFlurryModifier": {
    "Class": "CitadelBulletFlurry",
    "EnabledStateMask": [
      "SilencedHidden",
      "InfiniteClip"
    ],
    "Subclass": "CitadelBulletFlurry"
  },
  "ChannelMoveSpeed": 4,
  "EvasionPercent": 30,
  "IsDisabled": false,
  "Key": "ability_bullet_flurry",
  "Name": "Bullet Dance",
  "OverrideBulletRadius": 10,
  "ProcChance": 100,
  "Radius": 16,
  "RadiusMin": 0.75,
  "TargetsPerTick": 1,
  "Upgrades": [
    {
      "WeaponDamageBonus": 7
    },
    {
      "BonusFireRate": 10,
      "ChannelMoveSpeed": 3
    },
    {
      "AbilityCooldown": -65,
      "EvasionPercent": 40.0
    }
  ],
  "WeaponDamageBonus": 7,
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
    "card_name": "Bullet Dance",
    "hero_key": "hero_haze",
    "hero_name": "Haze",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_haze",
      "hero_name": "Haze",
      "lookup": "bullet dance",
      "name": "Bullet Dance",
      "type": "ability"
    }
  ]
}
````
