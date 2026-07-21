---
title: "Venator"
entity_type: "hero"
source: "deadlock_data"
external_id: "hero_priest"
canonical_name: "Venator"
snapshot_id: 39366
source_document_id: 7069
payload_hash: "f18d425ce5a6fa17960c5b42b29fca8b872f0061e2dbb72d45011122535606d7"
source_content_hash: "74a78bbafe26f260f0b01f8d431b88986d874c3bef714da77698b48d6862a424"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/hero-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_hero-data.json.74a78bbafe26f260.json"
fetched_at: "2026-07-09T19:36:22.543577+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "hero"]
---

# Venator

## Kurzueberblick

- Typ: `hero`
- Quelle: `deadlock_data`
- External ID: `hero_priest`
- Snapshot ID: `39366`
- Source-Dokument: `7069`
- Kurzinfo: Venator aus `deadlock_data` / `hero` mit vollstaendiger Payload.

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
      "Key": "ability_priest_flashbang",
      "Name": "Consecrating Grenade"
    },
    "2": {
      "Key": "ability_priest_knockback",
      "Name": "Gutshot"
    },
    "3": {
      "Key": "ability_priest_beartrap",
      "Name": "Hex-Lined Snap Trap"
    },
    "4": {
      "Key": "ability_priest_weaponswap",
      "Name": "Ira Domini"
    }
  },
  "CritDamageBonusPercent": 0.0,
  "CritDamageReceivedPercent": 0.0,
  "CrouchSpeed": 4.75,
  "GroundDashDistanceInMeters": 10.0,
  "GroundDashDuration": 0.7,
  "GroundDashSpeed": 14.3,
  "HeavyMeleeDamage": 125.0,
  "HeroBulletLifestealEffectiveness": 1.0,
  "HeroSpiritLifestealEffectiveness": 1.0,
  "InDevelopment": false,
  "InHeroLabs": false,
  "IsDisabled": false,
  "IsRecommended": true,
  "IsSelectable": true,
  "Key": "hero_priest",
  "LevelScaling": {
    "BulletDamage": 0.27,
    "DPS": 2.1429,
    "HeavyMeleeDamage": 4.25,
    "LightMeleeDamage": 1.7,
    "MaxHealth": 43.0,
    "PowerIncreases": 1,
    "SustainedDPS": 1.2805,
    "TechPower": 1.1
  },
  "LightMeleeDamage": 50,
  "Lore": "hero_priest_lore",
  "MaxHealth": 820.0,
  "MaxMoveSpeed": 6.4,
  "MoveAcceleration": 4,
  "Name": "Venator",
  "Playstyle": "hero_priest_playstyle",
  "ProcBuildUpRateScale": 1,
  "ReloadSpeed": 0,
  "Role": "hero_priest_role",
  "SpiritScaling": {
    "BulletResist": 0.12178,
    "TechResist": 0.12178
  },
  "SprintSpeed": 1.5,
  "Stamina": 3,
  "StaminaCooldown": 4.5000045000045,
  "StaminaRegenPerSecond": 0.222222,
  "TechDuration": 0,
  "TechRange": 0,
  "Type": "Marksman",
  "Weapon": {
    "AmmoConsumedPerShot": 1,
    "BulletDamage": 8.0,
    "BulletGravityScale": 0,
    "BulletRadius": 0.0762,
    "BulletSpeed": 1588.0,
    "BulletsPerBurst": 1,
    "BulletsPerShot": 1,
    "BurstInterShotInterval": 0,
    "CanCrit": true,
    "ClipSize": 33,
    "DPS": 63.492,
    "DescKey": "citadel_weapon_hero_priest_set_desc",
    "FalloffBias": 0.5,
    "FalloffEndRange": 50.8,
    "FalloffEndScale": 0.1,
    "FalloffStartRange": 19.99,
    "FalloffStartScale": 1.0,
    "HitOnceAcrossAllBullets": false,
    "NameKey": "citadel_weapon_hero_priest_set",
    "ReloadDelay": 0.0,
    "ReloadMovespeed": 1.0,
    "ReloadSingle": false,
    "ReloadTime": 2.8,
    "RoundsPerSecond": 7.9365,
    "ShootMoveSpeed": 0.75,
    "SustainedDPS": 37.942,
    "WeaponTypes": [
      "Attribute_EWeaponAttribute_RapidFire",
      "Attribute_EWeaponAttribute_MediumRange"
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
      "lookup": "venator",
      "name": "Venator",
      "type": "hero"
    }
  ]
}
````
