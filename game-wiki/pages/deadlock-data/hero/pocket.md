---
title: "Pocket"
entity_type: "hero"
source: "deadlock_data"
external_id: "hero_synth"
canonical_name: "Pocket"
snapshot_id: 39374
source_document_id: 7069
payload_hash: "47fc096f28dadb67747a0e613f5c5f999215b513669d0df467331d2a8ccea2b1"
source_content_hash: "74a78bbafe26f260f0b01f8d431b88986d874c3bef714da77698b48d6862a424"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/hero-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_hero-data.json.74a78bbafe26f260.json"
fetched_at: "2026-07-09T19:36:22.737662+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "hero"]
---

# Pocket

## Kurzueberblick

- Typ: `hero`
- Quelle: `deadlock_data`
- External ID: `hero_synth`
- Snapshot ID: `39374`
- Source-Dokument: `7069`
- Kurzinfo: Pocket aus `deadlock_data` / `hero` mit vollstaendiger Payload.

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
      "Key": "synth_barrage",
      "Name": "Barrage"
    },
    "2": {
      "Key": "synth_plasma_flux",
      "Name": "Flying Cloak"
    },
    "3": {
      "Key": "synth_pulse",
      "Name": "Enchanter's Satchel"
    },
    "4": {
      "Key": "synth_affliction",
      "Name": "Affliction"
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
  "Key": "hero_synth",
  "LevelScaling": {
    "BulletDamage": 0.14,
    "DPS": 1.8667,
    "HeavyMeleeDamage": 3.0547,
    "LightMeleeDamage": 1.58,
    "MaxHealth": 36.0,
    "PowerIncreases": 1,
    "SustainedDPS": 1.2542,
    "TechPower": 1.1
  },
  "LightMeleeDamage": 60,
  "Lore": "hero_synth_lore",
  "MaxHealth": 780.0,
  "MaxMoveSpeed": 7.2,
  "MoveAcceleration": 4,
  "Name": "Pocket",
  "Playstyle": "hero_synth_playstyle",
  "ProcBuildUpRateScale": 1,
  "ReloadSpeed": 0,
  "Role": "hero_synth_role",
  "SpiritScaling": {},
  "SprintSpeed": 1.6,
  "Stamina": 3,
  "StaminaCooldown": 4.5000045000045,
  "StaminaRegenPerSecond": 0.222222,
  "TechDuration": 0,
  "TechRange": 0,
  "TechResist": -15.0,
  "Type": "Assassin",
  "Weapon": {
    "AmmoConsumedPerShot": 1,
    "BulletDamage": 4.28,
    "BulletGravityScale": 0.8,
    "BulletRadius": 0.0762,
    "BulletSpeed": 558.8,
    "BulletsPerBurst": 1,
    "BulletsPerShot": 7,
    "BurstInterShotInterval": 0,
    "CanCrit": true,
    "ClipSize": 11,
    "DPS": 57.068,
    "DescKey": "citadel_weapon_hero_synth_set_desc",
    "FalloffBias": 0.5,
    "FalloffEndRange": 45.72,
    "FalloffEndScale": 0.1,
    "FalloffStartRange": 16.0,
    "FalloffStartScale": 1.0,
    "HitOnceAcrossAllBullets": false,
    "NameKey": "citadel_weapon_hero_synth_set",
    "ReloadDelay": 0.0,
    "ReloadMovespeed": 1.0,
    "ReloadSingle": false,
    "ReloadTime": 2.82,
    "RoundsPerSecond": 1.9048,
    "ShootMoveSpeed": 0.75,
    "SustainedDPS": 38.344,
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
      "lookup": "pocket",
      "name": "Pocket",
      "type": "hero"
    }
  ]
}
````
