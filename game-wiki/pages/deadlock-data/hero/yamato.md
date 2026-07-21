---
title: "Yamato"
entity_type: "hero"
source: "deadlock_data"
external_id: "hero_yamato"
canonical_name: "Yamato"
snapshot_id: 39391
source_document_id: 7069
payload_hash: "1e262efcc5b23ec182ad0c52bd50c4110cc2fda03adbc6c5ccde9e87c357642b"
source_content_hash: "74a78bbafe26f260f0b01f8d431b88986d874c3bef714da77698b48d6862a424"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/hero-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_hero-data.json.74a78bbafe26f260.json"
fetched_at: "2026-07-09T19:36:23.177451+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "hero"]
---

# Yamato

## Kurzueberblick

- Typ: `hero`
- Quelle: `deadlock_data`
- External ID: `hero_yamato`
- Snapshot ID: `39391`
- Source-Dokument: `7069`
- Kurzinfo: Yamato aus `deadlock_data` / `hero` mit vollstaendiger Payload.

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
      "Key": "citadel_ability_power_slash",
      "Name": "Power Slash"
    },
    "2": {
      "Key": "citadel_ability_flying_strike",
      "Name": "Flying Slash"
    },
    "3": {
      "Key": "citadel_ability_healing_slash",
      "Name": "Crimson Slash"
    },
    "4": {
      "Key": "citadel_ability_infinity_slash",
      "Name": "Shadow Transformation"
    }
  },
  "CritDamageBonusPercent": 0.0,
  "CritDamageReceivedPercent": 0.0,
  "CrouchSpeed": 4.75,
  "GroundDashDistanceInMeters": 10.0,
  "GroundDashDuration": 0.68,
  "GroundDashSpeed": 14.7,
  "HeavyMeleeDamage": 128,
  "HeroBulletLifestealEffectiveness": 1.0,
  "HeroSpiritLifestealEffectiveness": 1.0,
  "InDevelopment": false,
  "InHeroLabs": false,
  "IsDisabled": false,
  "IsRecommended": false,
  "IsSelectable": true,
  "Key": "hero_yamato",
  "LevelScaling": {
    "BulletDamage": 0.154,
    "BulletDamageAltFire": 0.7,
    "DPS": 1.8334,
    "HeavyMeleeDamage": 3.6771,
    "LightMeleeDamage": 1.58,
    "MaxHealth": 45.0,
    "PowerIncreases": 1,
    "SustainedDPS": 1.2347,
    "TechPower": 1.1
  },
  "LightMeleeDamage": 55,
  "Lore": "hero_yamato_lore",
  "MaxHealth": 730.0,
  "MaxMoveSpeed": 8.2,
  "MoveAcceleration": 4,
  "Name": "Yamato",
  "Playstyle": "hero_yamato_playstyle",
  "ProcBuildUpRateScale": 1,
  "ReloadSpeed": 0,
  "Role": "hero_yamato_role",
  "SpiritScaling": {
    "ClipSize": 0.15,
    "SustainedDPS": 0.17233
  },
  "SprintSpeed": 1.6,
  "Stamina": 3,
  "StaminaCooldown": 4.5000045000045,
  "StaminaRegenPerSecond": 0.222222,
  "TechDuration": 0,
  "TechRange": 0,
  "Type": "Assassin",
  "Weapon": {
    "AltFire": {
      "AmmoConsumedPerShot": 3,
      "BulletDamage": 50,
      "BulletGravityScale": 1.0,
      "BulletRadius": 0.3302,
      "BulletSpeed": 76.2,
      "BulletsPerBurst": 1,
      "BulletsPerShot": 1,
      "BurstInterShotInterval": 0,
      "CanCrit": false,
      "ClipSize": 9,
      "DPS": 47.619,
      "DescKey": "citadel_weapon_yamato_alt_desc",
      "ExplosionRadius": 3.048,
      "FalloffBias": 0.5,
      "FalloffEndRange": 57.51,
      "FalloffEndScale": 0.1,
      "FalloffStartRange": 19.99,
      "FalloffStartScale": 1.0,
      "HitOnceAcrossAllBullets": false,
      "NameKey": "citadel_weapon_yamato_alt",
      "ReloadDelay": 0.0,
      "ReloadMovespeed": 1.0,
      "ReloadSingle": false,
      "ReloadTime": 2.1,
      "RoundsPerSecond": 0.95238,
      "ShootMoveSpeed": 0.75,
      "SustainedDPS": 38.961
    },
    "AmmoConsumedPerShot": 1,
    "BulletDamage": 5.31,
    "BulletGravityScale": 0.0,
    "BulletRadius": 0.0762,
    "BulletSpeed": 254.0,
    "BulletsPerBurst": 1,
    "BulletsPerShot": 5,
    "BurstInterShotInterval": 0,
    "CanCrit": true,
    "ClipSize": 12,
    "DPS": 63.216,
    "DescKey": "citadel_weapon_hero_yamato_set_desc",
    "FalloffBias": 0.5,
    "FalloffEndRange": 45.72,
    "FalloffEndScale": 0.1,
    "FalloffStartRange": 19.99,
    "FalloffStartScale": 1.0,
    "HitOnceAcrossAllBullets": false,
    "NameKey": "citadel_weapon_hero_yamato_set",
    "ReloadDelay": 0.0,
    "ReloadMovespeed": 1.0,
    "ReloadSingle": false,
    "ReloadTime": 2.444,
    "RoundsPerSecond": 2.381,
    "ShootMoveSpeed": 0.75,
    "SustainedDPS": 42.571,
    "WeaponTypes": [
      "Attribute_EWeaponAttribute_Spreadshot",
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
      "lookup": "yamato",
      "name": "Yamato",
      "type": "hero"
    }
  ]
}
````
