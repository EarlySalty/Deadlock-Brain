---
title: "Warden"
entity_type: "hero"
source: "deadlock_data"
external_id: "hero_warden"
canonical_name: "Warden"
snapshot_id: 39385
source_document_id: 7069
payload_hash: "01f945f4983b51e41bb8204bd26c645d30593c9bd23b47d49e2a24c976fbf993"
source_content_hash: "74a78bbafe26f260f0b01f8d431b88986d874c3bef714da77698b48d6862a424"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/hero-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_hero-data.json.74a78bbafe26f260.json"
fetched_at: "2026-07-09T19:36:23.012667+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "hero"]
---

# Warden

## Kurzueberblick

- Typ: `hero`
- Quelle: `deadlock_data`
- External ID: `hero_warden`
- Snapshot ID: `39385`
- Source-Dokument: `7069`
- Kurzinfo: Warden aus `deadlock_data` / `hero` mit vollstaendiger Payload.

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
      "Key": "ability_warden_crowd_control",
      "Name": "Alchemical Flask"
    },
    "2": {
      "Key": "ability_warden_high_alert",
      "Name": "Willpower"
    },
    "3": {
      "Key": "ability_warden_lock_down",
      "Name": "Binding Word"
    },
    "4": {
      "Key": "ability_warden_riot_protocol",
      "Name": "Last Stand"
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
  "IsRecommended": false,
  "IsSelectable": true,
  "Key": "hero_warden",
  "LevelScaling": {
    "BulletDamage": 0.28,
    "DPS": 1.0667,
    "HeavyMeleeDamage": 3.6656,
    "LightMeleeDamage": 1.58,
    "MaxHealth": 60.0,
    "PowerIncreases": 1,
    "SustainedDPS": 0.64529,
    "TechPower": 1.1
  },
  "LightMeleeDamage": 50,
  "Lore": "hero_warden_lore",
  "MaxHealth": 805.0,
  "MaxMoveSpeed": 6.3,
  "MoveAcceleration": 4,
  "Name": "Warden",
  "Playstyle": "hero_warden_playstyle",
  "ProcBuildUpRateScale": 1,
  "ReloadSpeed": 0,
  "Role": "hero_warden_role",
  "SpiritScaling": {
    "DPS": 0.1734,
    "FireRate": 0.25,
    "RoundsPerSecond": 0.01,
    "SustainedDPS": 0.063395
  },
  "SprintSpeed": 1.6,
  "Stamina": 3,
  "StaminaCooldown": 4.5000045000045,
  "StaminaRegenPerSecond": 0.222222,
  "TechDuration": 0,
  "TechRange": 0,
  "Type": "Brawler",
  "Weapon": {
    "AmmoConsumedPerShot": 1,
    "BulletDamage": 17.34,
    "BulletGravityScale": 0.25,
    "BulletRadius": 0.127,
    "BulletSpeed": 290.0,
    "BulletsPerBurst": 1,
    "BulletsPerShot": 1,
    "BurstInterShotInterval": 0,
    "CanCrit": true,
    "ClipSize": 17,
    "DPS": 66.057,
    "DescKey": "citadel_weapon_hero_warden_set_desc",
    "FalloffBias": 0.5,
    "FalloffEndRange": 47.0,
    "FalloffEndScale": 0.1,
    "FalloffStartRange": 18.0,
    "FalloffStartScale": 1.0,
    "HitOnceAcrossAllBullets": false,
    "NameKey": "citadel_weapon_hero_warden_set",
    "ReloadDelay": 0.0,
    "ReloadMovespeed": 1.0,
    "ReloadSingle": false,
    "ReloadTime": 2.914,
    "RoundsPerSecond": 3.8095,
    "ShootMoveSpeed": 0.75,
    "SustainedDPS": 39.962,
    "WeaponTypes": [
      "Attribute_EWeaponAttribute_MediumRange",
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
      "lookup": "warden",
      "name": "Warden",
      "type": "hero"
    }
  ]
}
````
