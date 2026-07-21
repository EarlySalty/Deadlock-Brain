---
title: "Graves"
entity_type: "hero"
source: "deadlock_data"
external_id: "hero_necro"
canonical_name: "Graves"
snapshot_id: 39363
source_document_id: 7069
payload_hash: "d9f7436d2441b1c27b70c0de5ec35b06ab24644e9f7716b36654d1d4ff0c88f1"
source_content_hash: "74a78bbafe26f260f0b01f8d431b88986d874c3bef714da77698b48d6862a424"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/hero-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_hero-data.json.74a78bbafe26f260.json"
fetched_at: "2026-07-09T19:36:22.458704+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "hero"]
---

# Graves

## Kurzueberblick

- Typ: `hero`
- Quelle: `deadlock_data`
- External ID: `hero_necro`
- Snapshot ID: `39363`
- Source-Dokument: `7069`
- Kurzinfo: Graves aus `deadlock_data` / `hero` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityResourceMax": 0,
  "AbilityResourceRegenPerSecond": 0,
  "AirDashDistanceInMeters": 8.0,
  "AirDashDuration": 0.47,
  "AirDashSpeed": 17.0,
  "BaseHealthRegen": 1.0,
  "BaseWeaponDamageIncrease": 0,
  "BoundAbilities": {
    "1": {
      "Key": "ability_necro_hauntingskull",
      "Name": "Jar of Dead"
    },
    "2": {
      "Key": "ability_necro_zombiewall",
      "Name": "Grasping Hands"
    },
    "3": {
      "Key": "ability_necro_fear",
      "Name": "Essence Theft"
    },
    "4": {
      "Key": "ability_necro_gravestone",
      "Name": "Borrowed Decree"
    }
  },
  "BuildUpRate": -50.0,
  "BulletLifesteal": 8.0,
  "CritDamageBonusPercent": 0.0,
  "CritDamageReceivedPercent": 0.0,
  "CrouchSpeed": 4.75,
  "GroundDashDistanceInMeters": 10.0,
  "GroundDashDuration": 0.68,
  "GroundDashSpeed": 14.7,
  "HeavyMeleeDamage": 116,
  "HeroBulletLifestealEffectiveness": 1.0,
  "HeroSpiritLifestealEffectiveness": 1.0,
  "InDevelopment": false,
  "InHeroLabs": false,
  "IsDisabled": false,
  "IsRecommended": true,
  "IsSelectable": true,
  "Key": "hero_necro",
  "LevelScaling": {
    "BulletDamage": 0.054,
    "DPS": 0.52941,
    "HeavyMeleeDamage": 3.6656,
    "LightMeleeDamage": 1.58,
    "MaxHealth": 33.0,
    "PowerIncreases": 1,
    "SustainedDPS": 0.31395,
    "TechPower": 1.1
  },
  "LightMeleeDamage": 50,
  "Lore": "hero_necro_lore",
  "MaxHealth": 730.0,
  "MaxMoveSpeed": 7.0,
  "MoveAcceleration": 4,
  "Name": "Graves",
  "Playstyle": "hero_necro_playstyle",
  "ProcBuildUpRateScale": 1,
  "ReloadSpeed": 0,
  "Role": "hero_necro_role",
  "SpiritScaling": {},
  "SprintSpeed": 2.2,
  "Stamina": 2.0,
  "StaminaCooldown": 4.5000045000045,
  "StaminaRegenPerSecond": 0.222222,
  "TechDuration": 0,
  "TechRange": 0,
  "Type": "Marksman",
  "Weapon": {
    "AmmoConsumedPerShot": 1,
    "BulletDamage": 3.6,
    "BulletGravityScale": 0.0,
    "BulletRadius": 0.0762,
    "BulletSpeed": 635.0,
    "BulletsPerBurst": 1,
    "BulletsPerShot": 1,
    "BurstInterShotInterval": 0,
    "CanCrit": true,
    "ClipSize": 40,
    "DPS": 35.294,
    "DescKey": "citadel_weapon_hero_necro_set_desc",
    "FalloffBias": 0.5,
    "FalloffEndRange": 17.02,
    "FalloffEndScale": 0.5,
    "FalloffStartRange": 7.62,
    "FalloffStartScale": 1.0,
    "HitOnceAcrossAllBullets": false,
    "NameKey": "citadel_weapon_hero_necro_set",
    "ReloadDelay": 0.0,
    "ReloadMovespeed": 1.0,
    "ReloadSingle": false,
    "ReloadTime": 2.8,
    "RoundsPerSecond": 9.8039,
    "ShootMoveSpeed": 0.75,
    "SustainedDPS": 20.93,
    "WeaponTypes": [
      "Attribute_EWeaponAttribute_BeamWeapon",
      "Attribute_EWeaponAttribute_CloseRange"
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
      "lookup": "graves",
      "name": "Graves",
      "type": "hero"
    }
  ]
}
````
