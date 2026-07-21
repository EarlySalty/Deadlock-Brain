---
title: "Vindicta"
entity_type: "hero"
source: "deadlock_data"
external_id: "hero_hornet"
canonical_name: "Vindicta"
snapshot_id: 39354
source_document_id: 7069
payload_hash: "8426cd9d86234f9e674d6bb11235f7f5fb3f65a1fb1138b95a3510f069edd6bd"
source_content_hash: "74a78bbafe26f260f0b01f8d431b88986d874c3bef714da77698b48d6862a424"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/hero-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_hero-data.json.74a78bbafe26f260.json"
fetched_at: "2026-07-09T19:36:22.221151+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "hero"]
---

# Vindicta

## Kurzueberblick

- Typ: `hero`
- Quelle: `deadlock_data`
- External ID: `hero_hornet`
- Snapshot ID: `39354`
- Source-Dokument: `7069`
- Kurzinfo: Vindicta aus `deadlock_data` / `hero` mit vollstaendiger Payload.

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
      "Key": "citadel_ability_hornet_chain",
      "Name": "Stake"
    },
    "2": {
      "Key": "citadel_ability_hornet_leap",
      "Name": "Flight"
    },
    "3": {
      "Key": "citadel_ability_hornet_sting",
      "Name": "Crow Familiar"
    },
    "4": {
      "Key": "citadel_ability_hornet_snipe",
      "Name": "Assassinate"
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
  "Key": "hero_hornet",
  "LevelScaling": {
    "BulletDamage": 0.495,
    "DPS": 2.1429,
    "HeavyMeleeDamage": 3.6656,
    "LightMeleeDamage": 1.58,
    "MaxHealth": 28.0,
    "PowerIncreases": 1,
    "SustainedDPS": 1.2878,
    "TechPower": 1.1
  },
  "LightMeleeDamage": 50,
  "Lore": "hero_hornet_lore",
  "MaxHealth": 755.0,
  "MaxMoveSpeed": 7.9,
  "MoveAcceleration": 4,
  "Name": "Vindicta",
  "Playstyle": "hero_hornet_playstyle",
  "ProcBuildUpRateScale": 1,
  "ReloadSpeed": 0,
  "Role": "hero_hornet_role",
  "SpiritScaling": {
    "BulletDamage": 0.022,
    "DPS": 0.095238,
    "SustainedDPS": 0.057237
  },
  "SprintSpeed": 1.6,
  "Stamina": 2,
  "StaminaCooldown": 4.5000045000045,
  "StaminaRegenPerSecond": 0.222222,
  "TechDuration": 0,
  "TechRange": 0,
  "Type": "Marksman",
  "Weapon": {
    "AmmoConsumedPerShot": 1,
    "BulletDamage": 12.33,
    "BulletGravityScale": 0.8,
    "BulletRadius": 0.0762,
    "BulletSpeed": 660.0,
    "BulletsPerBurst": 1,
    "BulletsPerShot": 1,
    "BurstInterShotInterval": 0,
    "CanCrit": true,
    "ClipSize": 19,
    "DPS": 53.377,
    "DescKey": "citadel_weapon_hero_hornet_set_desc",
    "FalloffBias": 0.5,
    "FalloffEndRange": 64.0,
    "FalloffEndScale": 0.1,
    "FalloffStartRange": 19.99,
    "FalloffStartScale": 1.0,
    "HitOnceAcrossAllBullets": false,
    "NameKey": "citadel_weapon_hero_hornet_set",
    "ReloadDelay": 0.0,
    "ReloadMovespeed": 1.0,
    "ReloadSingle": false,
    "ReloadTime": 2.914,
    "RoundsPerSecond": 4.329,
    "ShootMoveSpeed": 0.75,
    "SustainedDPS": 32.079,
    "WeaponTypes": [
      "Attribute_EWeaponAttribute_RapidFire",
      "Attribute_EWeaponAttribute_LongRange"
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
      "lookup": "vindicta",
      "name": "Vindicta",
      "type": "hero"
    }
  ]
}
````
