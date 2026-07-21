---
title: "Infernus"
entity_type: "hero"
source: "deadlock_data"
external_id: "hero_inferno"
canonical_name: "Infernus"
snapshot_id: 39355
source_document_id: 7069
payload_hash: "9a74e09f798a73fa1ff92d4a3ab7c9751a64e925ae2379170d2e27cbdab00880"
source_content_hash: "74a78bbafe26f260f0b01f8d431b88986d874c3bef714da77698b48d6862a424"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/hero-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_hero-data.json.74a78bbafe26f260.json"
fetched_at: "2026-07-09T19:36:22.247242+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "hero"]
---

# Infernus

## Kurzueberblick

- Typ: `hero`
- Quelle: `deadlock_data`
- External ID: `hero_inferno`
- Snapshot ID: `39355`
- Source-Dokument: `7069`
- Kurzinfo: Infernus aus `deadlock_data` / `hero` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityResourceMax": 0,
  "AbilityResourceRegenPerSecond": 0,
  "AirDashDistanceInMeters": 8.0,
  "AirDashDuration": 0.47,
  "AirDashSpeed": 17.0,
  "BaseHealthRegen": 2.0,
  "BaseWeaponDamageIncrease": 0,
  "BoundAbilities": {
    "1": {
      "Key": "ability_incendiary_projectile",
      "Name": "Napalm"
    },
    "2": {
      "Key": "ability_flame_dash",
      "Name": "Flame Dash"
    },
    "3": {
      "Key": "ability_afterburn",
      "Name": "Afterburn"
    },
    "4": {
      "Key": "ability_fire_bomb",
      "Name": "Concussive Combustion"
    }
  },
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
  "Key": "hero_inferno",
  "LevelScaling": {
    "BulletDamage": 0.088,
    "DPS": 0.83809,
    "HeavyMeleeDamage": 3.6656,
    "LightMeleeDamage": 1.58,
    "MaxHealth": 39.0,
    "PowerIncreases": 1,
    "SustainedDPS": 0.46726,
    "TechPower": 1.1
  },
  "LightMeleeDamage": 50,
  "Lore": "hero_inferno_lore",
  "MaxHealth": 830.0,
  "MaxMoveSpeed": 6.7,
  "MoveAcceleration": 4,
  "Name": "Infernus",
  "Playstyle": "hero_inferno_playstyle",
  "ProcBuildUpRateScale": 1,
  "ReloadSpeed": 0,
  "Role": "hero_inferno_role",
  "SpiritScaling": {},
  "SprintSpeed": 1.6,
  "Stamina": 3,
  "StaminaCooldown": 4.5000045000045,
  "StaminaRegenPerSecond": 0.222222,
  "TechDuration": 0,
  "TechRange": 0,
  "Type": "Marksman",
  "Weapon": {
    "AmmoConsumedPerShot": 1,
    "BulletDamage": 5.5,
    "BulletGravityScale": 0.8,
    "BulletRadius": 0.1016,
    "BulletSpeed": 660.4,
    "BulletsPerBurst": 1,
    "BulletsPerShot": 1,
    "BurstInterShotInterval": 0,
    "CanCrit": true,
    "ClipSize": 27,
    "DPS": 52.381,
    "DescKey": "citadel_weapon_hero_inferno_set_desc",
    "FalloffBias": 0.5,
    "FalloffEndRange": 55.0,
    "FalloffEndScale": 0.1,
    "FalloffStartRange": 18.0,
    "FalloffStartScale": 1.0,
    "HitOnceAcrossAllBullets": false,
    "NameKey": "citadel_weapon_hero_inferno_set",
    "ReloadDelay": 0.0,
    "ReloadMovespeed": 1.0,
    "ReloadSingle": false,
    "ReloadTime": 2.25,
    "RoundsPerSecond": 9.5238,
    "ShootMoveSpeed": 0.75,
    "SustainedDPS": 29.204,
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
      "lookup": "infernus",
      "name": "Infernus",
      "type": "hero"
    }
  ]
}
````
