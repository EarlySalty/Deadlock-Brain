---
title: "Abrams"
entity_type: "hero"
source: "deadlock_data"
external_id: "hero_atlas"
canonical_name: "Abrams"
snapshot_id: 39333
source_document_id: 7069
payload_hash: "ddd70e832303279695bb8cc85b5508520e8bba1c0c90966b3ed178b1402e638a"
source_content_hash: "74a78bbafe26f260f0b01f8d431b88986d874c3bef714da77698b48d6862a424"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/hero-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_hero-data.json.74a78bbafe26f260.json"
fetched_at: "2026-07-09T19:36:21.664512+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "hero"]
---

# Abrams

## Kurzueberblick

- Typ: `hero`
- Quelle: `deadlock_data`
- External ID: `hero_atlas`
- Snapshot ID: `39333`
- Source-Dokument: `7069`
- Kurzinfo: Abrams aus `deadlock_data` / `hero` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityResourceMax": 0,
  "AbilityResourceRegenPerSecond": 0,
  "AirDashDistanceInMeters": 8.0,
  "AirDashDuration": 0.49,
  "AirDashSpeed": 16.3,
  "BaseHealthRegen": 1.5,
  "BaseWeaponDamageIncrease": 0,
  "BoundAbilities": {
    "1": {
      "Key": "citadel_ability_bull_heal",
      "Name": "Siphon Life"
    },
    "2": {
      "Key": "citadel_ability_bull_charge",
      "Name": "Shoulder Charge"
    },
    "3": {
      "Key": "citadel_ability_passive_beefy",
      "Name": "Infernal Resilience"
    },
    "4": {
      "Key": "citadel_ability_bull_leap",
      "Name": "Seismic Impact"
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
  "Key": "hero_atlas",
  "LevelScaling": {
    "BulletDamage": 0.1,
    "DPS": 1.4286,
    "HeavyMeleeDamage": 4.0322,
    "LightMeleeDamage": 1.738,
    "MaxHealth": 49.0,
    "PowerIncreases": 1,
    "SustainedDPS": 0.84839,
    "TechPower": 1.1
  },
  "LightMeleeDamage": 50,
  "Lore": "hero_atlas_lore",
  "MaxHealth": 800.0,
  "MaxMoveSpeed": 6.4,
  "MoveAcceleration": 4,
  "Name": "Abrams",
  "Playstyle": "hero_atlas_playstyle",
  "ProcBuildUpRateScale": 1,
  "ReloadSpeed": 0,
  "Role": "hero_atlas_role",
  "SpiritScaling": {},
  "SprintSpeed": 1.6,
  "Stamina": 3,
  "StaminaCooldown": 4.5000045000045,
  "StaminaRegenPerSecond": 0.222222,
  "TechDuration": 0,
  "TechRange": 0,
  "Type": "Brawler",
  "Weapon": {
    "AmmoConsumedPerShot": 1,
    "BulletDamage": 3.6,
    "BulletGravityScale": 0.8,
    "BulletRadius": 0.0762,
    "BulletSpeed": 609.6,
    "BulletsPerBurst": 1,
    "BulletsPerShot": 9,
    "BurstInterShotInterval": 0,
    "CanCrit": true,
    "ClipSize": 9,
    "DPS": 51.429,
    "DescKey": "citadel_weapon_hero_atlas_set_desc",
    "FalloffBias": 0.5,
    "FalloffEndRange": 40.0,
    "FalloffEndScale": 0.1,
    "FalloffStartRange": 17.0,
    "FalloffStartScale": 1.0,
    "HitOnceAcrossAllBullets": false,
    "NameKey": "citadel_weapon_hero_atlas_set",
    "ReloadDelay": 0.705,
    "ReloadMovespeed": 1.0,
    "ReloadSingle": true,
    "ReloadTime": 0.3525,
    "RoundsPerSecond": 1.5873,
    "ShootMoveSpeed": 0.75,
    "SustainedDPS": 30.542,
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
      "lookup": "abrams",
      "name": "Abrams",
      "type": "hero"
    }
  ]
}
````
