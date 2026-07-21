---
title: "Shiv"
entity_type: "hero"
source: "deadlock_data"
external_id: "hero_shiv"
canonical_name: "Shiv"
snapshot_id: 39370
source_document_id: 7069
payload_hash: "2d17a31ca6d8c22c48a84cc816b6905ae45a893717477b4a8c2081a7d4fab4e3"
source_content_hash: "74a78bbafe26f260f0b01f8d431b88986d874c3bef714da77698b48d6862a424"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/hero-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_hero-data.json.74a78bbafe26f260.json"
fetched_at: "2026-07-09T19:36:22.647496+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "hero"]
---

# Shiv

## Kurzueberblick

- Typ: `hero`
- Quelle: `deadlock_data`
- External ID: `hero_shiv`
- Snapshot ID: `39370`
- Source-Dokument: `7069`
- Kurzinfo: Shiv aus `deadlock_data` / `hero` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityResourceMax": 0,
  "AbilityResourceRegenPerSecond": 0,
  "AirDashDistanceInMeters": 8.0,
  "AirDashDuration": 0.49,
  "AirDashSpeed": 16.3,
  "BaseHealthRegen": 2.0,
  "BaseWeaponDamageIncrease": 0,
  "BoundAbilities": {
    "1": {
      "Key": "citadel_ability_shiv_dagger",
      "Name": "Serrated Knives"
    },
    "2": {
      "Key": "citadel_ability_shiv_dash",
      "Name": "Slice and Dice"
    },
    "3": {
      "Key": "citadel_ability_shiv_defer_damage",
      "Name": "Bloodletting"
    },
    "4": {
      "Key": "citadel_ability_shiv_killing_blow",
      "Name": "Killing Blow"
    }
  },
  "CritDamageBonusPercent": 0.0,
  "CritDamageReceivedPercent": 0.0,
  "CrouchSpeed": 4.75,
  "GroundDashDistanceInMeters": 10.0,
  "GroundDashDuration": 0.7,
  "GroundDashSpeed": 14.3,
  "HeavyMeleeDamage": 116,
  "HeroBulletLifestealEffectiveness": 1.0,
  "HeroSpiritLifestealEffectiveness": 1.0,
  "InDevelopment": false,
  "InHeroLabs": false,
  "IsDisabled": false,
  "IsRecommended": false,
  "IsSelectable": true,
  "Key": "hero_shiv",
  "LevelScaling": {
    "BulletDamage": 0.165,
    "BulletDamageAltFire": 0.2,
    "DPS": 1.796,
    "HeavyMeleeDamage": 3.6656,
    "LightMeleeDamage": 1.58,
    "MaxHealth": 46.0,
    "PowerIncreases": 1,
    "SustainedDPS": 1.191,
    "TechPower": 1.1
  },
  "LightMeleeDamage": 50,
  "Lore": "hero_shiv_lore",
  "MaxHealth": 830.0,
  "MaxMoveSpeed": 6.5,
  "MoveAcceleration": 4,
  "Name": "Shiv",
  "Playstyle": "hero_shiv_playstyle",
  "ProcBuildUpRateScale": 1,
  "ReloadSpeed": 0,
  "Role": "hero_shiv_role",
  "SpiritScaling": {},
  "SprintSpeed": 1.6,
  "Stamina": 3,
  "StaminaCooldown": 5.500247511138001,
  "StaminaRegenPerSecond": 0.18181,
  "TechDuration": 0,
  "TechRange": 0,
  "Type": "Brawler",
  "Weapon": {
    "AltFire": {
      "AmmoConsumedPerShot": 4,
      "BulletDamage": 4.36,
      "BulletGravityScale": 0.8,
      "BulletRadius": 0.0762,
      "BulletSpeed": 609.6,
      "BulletsPerBurst": 1,
      "BulletsPerShot": 12,
      "BurstInterShotInterval": 0.05,
      "CanCrit": true,
      "ClipSize": 8,
      "DPS": 41.524,
      "DescKey": "citadel_weapon_shiv_alt_desc",
      "FalloffBias": 0.5,
      "FalloffEndRange": 57.51,
      "FalloffEndScale": 0.1,
      "FalloffStartRange": 19.99,
      "FalloffStartScale": 1.0,
      "HitOnceAcrossAllBullets": false,
      "NameKey": "citadel_weapon_shiv_alt",
      "ReloadDelay": 0.0,
      "ReloadMovespeed": 1.0,
      "ReloadSingle": false,
      "ReloadTime": 2.0,
      "RoundsPerSecond": 0.79365,
      "ShootMoveSpeed": 0.75,
      "SustainedDPS": 34.649
    },
    "AmmoConsumedPerShot": 1,
    "BulletDamage": 4.8,
    "BulletGravityScale": 0.8,
    "BulletRadius": 0.0762,
    "BulletSpeed": 609.6,
    "BulletsPerBurst": 1,
    "BulletsPerShot": 6,
    "BurstInterShotInterval": 0,
    "CanCrit": true,
    "ClipSize": 10,
    "DPS": 52.246,
    "DescKey": "citadel_weapon_hero_shiv_set_desc",
    "FalloffBias": 0.5,
    "FalloffEndRange": 41.15,
    "FalloffEndScale": 0.1,
    "FalloffStartRange": 19.79,
    "FalloffStartScale": 1.0,
    "HitOnceAcrossAllBullets": false,
    "NameKey": "citadel_weapon_hero_shiv_set",
    "ReloadDelay": 0.0,
    "ReloadMovespeed": 1.0,
    "ReloadSingle": false,
    "ReloadTime": 2.8,
    "RoundsPerSecond": 1.8141,
    "ShootMoveSpeed": 0.75,
    "SustainedDPS": 34.647,
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
      "lookup": "shiv",
      "name": "Shiv",
      "type": "hero"
    }
  ]
}
````
