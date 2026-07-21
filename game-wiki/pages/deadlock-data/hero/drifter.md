---
title: "Drifter"
entity_type: "hero"
source: "deadlock_data"
external_id: "hero_drifter"
canonical_name: "Drifter"
snapshot_id: 39341
source_document_id: 7069
payload_hash: "c2100a7e009954cd212615c7ebc2f4b522d0bd59841774442f52f04d615826de"
source_content_hash: "74a78bbafe26f260f0b01f8d431b88986d874c3bef714da77698b48d6862a424"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/hero-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_hero-data.json.74a78bbafe26f260.json"
fetched_at: "2026-07-09T19:36:21.883268+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "hero"]
---

# Drifter

## Kurzueberblick

- Typ: `hero`
- Quelle: `deadlock_data`
- External ID: `hero_drifter`
- Snapshot ID: `39341`
- Source-Dokument: `7069`
- Kurzinfo: Drifter aus `deadlock_data` / `hero` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityResourceMax": 0,
  "AbilityResourceRegenPerSecond": 0,
  "AirDashDistanceInMeters": 8.0,
  "AirDashDuration": 0.47,
  "AirDashSpeed": 17.0,
  "BaseHealthRegen": 3.5,
  "BaseWeaponDamageIncrease": 0,
  "BoundAbilities": {
    "1": {
      "Key": "drifter_blood_blast",
      "Name": "Rend"
    },
    "2": {
      "Key": "drifter_shadow_mark",
      "Name": "Stalker's Mark"
    },
    "3": {
      "Key": "ability_drifter_hunger",
      "Name": "Bloodscent"
    },
    "4": {
      "Key": "drifter_darkness",
      "Name": "Eternal Night"
    }
  },
  "CritDamageBonusPercent": -45.0,
  "CritDamageReceivedPercent": 0.0,
  "CrouchSpeed": 4.75,
  "GroundDashDistanceInMeters": 10.0,
  "GroundDashDuration": 0.68,
  "GroundDashSpeed": 14.7,
  "HeavyMeleeDamage": 120,
  "HeroBulletLifestealEffectiveness": 1.0,
  "HeroSpiritLifestealEffectiveness": 1.0,
  "InDevelopment": false,
  "InHeroLabs": false,
  "IsDisabled": false,
  "IsRecommended": false,
  "IsSelectable": true,
  "Key": "hero_drifter",
  "LevelScaling": {
    "BulletDamage": 0.616,
    "DPS": 1.3968,
    "HeavyMeleeDamage": 3.6816,
    "LightMeleeDamage": 1.58,
    "MaxHealth": 41.0,
    "PowerIncreases": 1,
    "SustainedDPS": 0.95554,
    "TechPower": 1.1
  },
  "LightMeleeDamage": 51.5,
  "Lore": "hero_drifter_lore",
  "MaxHealth": 755.0,
  "MaxMoveSpeed": 6.9,
  "MoveAcceleration": 4,
  "Name": "Drifter",
  "Playstyle": "hero_drifter_playstyle",
  "ProcBuildUpRateScale": 1,
  "ReloadSpeed": 0,
  "Role": "hero_drifter_role",
  "SpiritScaling": {},
  "SprintSpeed": 1.6,
  "Stamina": 3,
  "StaminaCooldown": 4.5000045000045,
  "StaminaRegenPerSecond": 0.222222,
  "TechDuration": 0,
  "TechRange": 0,
  "Type": "Assassin",
  "Weapon": {
    "AmmoConsumedPerShot": 1,
    "BulletDamage": 19.5,
    "BulletGravityScale": 0.0,
    "BulletRadius": 0.1524,
    "BulletSpeed": 508.0,
    "BulletsPerBurst": 1,
    "BulletsPerShot": 3,
    "BurstInterShotInterval": 0,
    "CanCrit": true,
    "ClipSize": 12,
    "DPS": 44.218,
    "DescKey": "citadel_weapon_hero_drifter_set_desc",
    "FalloffBias": 0.5,
    "FalloffEndRange": 27.0,
    "FalloffEndScale": 0.6,
    "FalloffStartRange": 22.0,
    "FalloffStartScale": 1.0,
    "HitOnceAcrossAllBullets": true,
    "NameKey": "citadel_weapon_hero_drifter_set",
    "ReloadDelay": 0.0,
    "ReloadMovespeed": 1.0,
    "ReloadSingle": false,
    "ReloadTime": 2.444,
    "RoundsPerSecond": 2.2676,
    "ShootMoveSpeed": 0.75,
    "SustainedDPS": 30.248,
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
      "lookup": "drifter",
      "name": "Drifter",
      "type": "hero"
    }
  ]
}
````
