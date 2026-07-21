---
title: "Grey Talon"
entity_type: "hero"
source: "deadlock_data"
external_id: "hero_orion"
canonical_name: "Grey Talon"
snapshot_id: 39365
source_document_id: 7069
payload_hash: "37aeac4bc97a45f4dceff5bd73d807906ea4dc87c687adb8b9482565a68f736d"
source_content_hash: "74a78bbafe26f260f0b01f8d431b88986d874c3bef714da77698b48d6862a424"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/hero-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_hero-data.json.74a78bbafe26f260.json"
fetched_at: "2026-07-09T19:36:22.513762+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "hero"]
---

# Grey Talon

## Kurzueberblick

- Typ: `hero`
- Quelle: `deadlock_data`
- External ID: `hero_orion`
- Snapshot ID: `39365`
- Source-Dokument: `7069`
- Kurzinfo: Grey Talon aus `deadlock_data` / `hero` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityResourceMax": 0,
  "AbilityResourceRegenPerSecond": 0,
  "AirDashDistanceInMeters": 8.0,
  "AirDashDuration": 0.43,
  "AirDashSpeed": 18.6,
  "BaseHealthRegen": 1.5,
  "BaseWeaponDamageIncrease": 0,
  "BoundAbilities": {
    "1": {
      "Key": "ability_charged_shot",
      "Name": "Charged Shot"
    },
    "2": {
      "Key": "ability_power_jump",
      "Name": "Rain of Arrows"
    },
    "3": {
      "Key": "ability_immobilize_trap",
      "Name": "Spirit Snare"
    },
    "4": {
      "Key": "ability_guided_arrow",
      "Name": "Guided Owl"
    }
  },
  "CritDamageBonusPercent": 0.0,
  "CritDamageReceivedPercent": 0.0,
  "CrouchSpeed": 4.75,
  "GroundDashDistanceInMeters": 10.0,
  "GroundDashDuration": 0.62,
  "GroundDashSpeed": 16.1,
  "HeavyMeleeDamage": 116,
  "HeroBulletLifestealEffectiveness": 1.0,
  "HeroSpiritLifestealEffectiveness": 1.0,
  "InDevelopment": false,
  "InHeroLabs": false,
  "IsDisabled": false,
  "IsRecommended": false,
  "IsSelectable": true,
  "Key": "hero_orion",
  "LevelScaling": {
    "BulletDamage": 0.85,
    "DPS": 1.4167,
    "HeavyMeleeDamage": 3.6656,
    "LightMeleeDamage": 1.58,
    "MaxHealth": 38.0,
    "PowerIncreases": 1,
    "SustainedDPS": 1.1514,
    "TechPower": 1.6
  },
  "LightMeleeDamage": 50,
  "Lore": "hero_orion_lore",
  "MaxHealth": 780.0,
  "MaxMoveSpeed": 6.3,
  "MoveAcceleration": 4,
  "Name": "Grey Talon",
  "Playstyle": "hero_orion_playstyle",
  "ProcBuildUpRateScale": 1,
  "ReloadSpeed": 0,
  "Role": "hero_orion_role",
  "SpiritScaling": {
    "BulletDamage": 0.08,
    "DPS": 0.13334,
    "MaxMoveSpeed": 0.0084,
    "SustainedDPS": 0.10837
  },
  "SprintSpeed": 1.6,
  "Stamina": 4,
  "StaminaCooldown": 4.5000045000045,
  "StaminaRegenPerSecond": 0.222222,
  "TechDuration": 0,
  "TechRange": 0,
  "Type": "Marksman",
  "Weapon": {
    "AmmoConsumedPerShot": 1,
    "BulletDamage": 23.51,
    "BulletGravityScale": 0.8,
    "BulletRadius": 0.127,
    "BulletSpeed": 495.3,
    "BulletsPerBurst": 1,
    "BulletsPerShot": 1,
    "BurstInterShotInterval": 0,
    "CanCrit": true,
    "ClipSize": 17,
    "DPS": 39.184,
    "DescKey": "citadel_weapon_hero_orion_set_desc",
    "FalloffBias": 0.5,
    "FalloffEndRange": 54.0,
    "FalloffEndScale": 0.1,
    "FalloffStartRange": 18.0,
    "FalloffStartScale": 1.0,
    "HitOnceAcrossAllBullets": true,
    "NameKey": "citadel_weapon_hero_orion_set",
    "ReloadDelay": 0.0,
    "ReloadMovespeed": 1.0,
    "ReloadSingle": false,
    "ReloadTime": 2.35,
    "RoundsPerSecond": 1.6667,
    "ShootMoveSpeed": 0.75,
    "SustainedDPS": 31.847,
    "WeaponTypes": [
      "Attribute_EWeaponAttribute_MediumRange",
      "Attribute_EWeaponAttribute_HeavyHitter",
      "Attribute_EWeaponAttribute_Projectile"
    ]
  },
  "WeaponPowerScale": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/hero-data.json",
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
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "grey talon",
      "name": "Grey Talon",
      "type": "hero"
    }
  ]
}
````
