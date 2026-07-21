---
title: "Celeste"
entity_type: "hero"
source: "deadlock_data"
external_id: "hero_unicorn"
canonical_name: "Celeste"
snapshot_id: 39380
source_document_id: 7069
payload_hash: "057f79990a35482d3b4844dbc53fc383a79c38fb8f82923ea6c501cd0083360e"
source_content_hash: "74a78bbafe26f260f0b01f8d431b88986d874c3bef714da77698b48d6862a424"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/hero-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_hero-data.json.74a78bbafe26f260.json"
fetched_at: "2026-07-09T19:36:22.875757+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "hero"]
---

# Celeste

## Kurzueberblick

- Typ: `hero`
- Quelle: `deadlock_data`
- External ID: `hero_unicorn`
- Snapshot ID: `39380`
- Source-Dokument: `7069`
- Kurzinfo: Celeste aus `deadlock_data` / `hero` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityResourceMax": 0,
  "AbilityResourceRegenPerSecond": 0,
  "AirControlAccelPercent": 10,
  "AirControlPercent": 44,
  "AirDashDistanceInMeters": 8.0,
  "AirDashDuration": 0.43,
  "AirDashSpeed": 18.6,
  "BaseHealthRegen": 1,
  "BaseWeaponDamageIncrease": 0,
  "BoundAbilities": {
    "1": {
      "Key": "ability_unicorn_radiantblast",
      "Name": "Light Eater"
    },
    "2": {
      "Key": "ability_unicorn_prismaticguard",
      "Name": "Dazzling Trick"
    },
    "3": {
      "Key": "ability_unicorn_luminousstrike",
      "Name": "Radiant Daggers"
    },
    "4": {
      "Key": "ability_unicorn_dazzlingorb",
      "Name": "Shining Wonder"
    }
  },
  "CritDamageBonusPercent": -25.0,
  "CritDamageReceivedPercent": 0.0,
  "CrouchSpeed": 4.75,
  "GravityChange": -25,
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
  "Key": "hero_unicorn",
  "LevelScaling": {
    "BulletDamage": 0.82,
    "DPS": 1.4138,
    "HeavyMeleeDamage": 3.6656,
    "LightMeleeDamage": 1.58,
    "MaxHealth": 33.0,
    "PowerIncreases": 1,
    "SustainedDPS": 0.98794,
    "TechPower": 1.1
  },
  "LightMeleeDamage": 50,
  "Lore": "hero_unicorn_lore",
  "MaxHealth": 690.0,
  "MaxMoveSpeed": 6.2,
  "MoveAcceleration": 4,
  "Name": "Celeste",
  "Playstyle": "hero_unicorn_playstyle",
  "ProcBuildUpRateScale": 1,
  "ReloadSpeed": 0,
  "Role": "hero_unicorn_role",
  "SpiritScaling": {},
  "SprintSpeed": 1.6,
  "Stamina": 4,
  "StaminaCooldown": 5.0,
  "StaminaRegenPerSecond": 0.2,
  "TechDuration": 0,
  "TechRange": 0,
  "Type": "Marksman",
  "Weapon": {
    "AmmoConsumedPerShot": 1,
    "BulletDamage": 18,
    "BulletGravityScale": 0.2,
    "BulletRadius": 0.4572,
    "BulletSpeed": 50.0,
    "BulletsPerBurst": 1,
    "BulletsPerShot": 1,
    "BurstInterShotInterval": 0,
    "CanCrit": true,
    "ClipSize": 8,
    "DPS": 31.034,
    "DescKey": "citadel_weapon_hero_unicorn_set_desc",
    "ExplosionRadius": 0.0,
    "FalloffBias": 0.5,
    "FalloffEndRange": 60.0,
    "FalloffEndScale": 0.1,
    "FalloffStartRange": 22.0,
    "FalloffStartScale": 1.0,
    "HitOnceAcrossAllBullets": false,
    "NameKey": "citadel_weapon_hero_unicorn_set",
    "ReloadDelay": 0.0,
    "ReloadMovespeed": 1.0,
    "ReloadSingle": false,
    "ReloadTime": 2.0,
    "RoundsPerSecond": 1.7241,
    "ShootMoveSpeed": 0.75,
    "SustainedDPS": 21.686,
    "WeaponTypes": [
      "Attribute_EWeaponAttribute_MediumRange",
      "Attribute_EWeaponAttribute_Projectile",
      "Attribute_EWeaponAttribute_Bouncy"
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
      "lookup": "celeste",
      "name": "Celeste",
      "type": "hero"
    }
  ]
}
````
