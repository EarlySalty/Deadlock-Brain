---
title: "Express Shot"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_express_shot"
canonical_name: "Express Shot"
snapshot_id: 40062
source_document_id: 7072
payload_hash: "7211485234d99276e59fd9b9c1e6610dc192be1becd1c6a8a2ccc259c2dd6e9f"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.782025+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Express Shot

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_express_shot`
- Snapshot ID: `40062`
- Source-Dokument: `7072`
- Kurzinfo: Express Shot aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 8,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 8,
  "BonusBulletSpeedPercent": 60,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_high_velocity_mag"
  ],
  "Cost": 3200,
  "Description": "Your next attack will <span class=\"highlight\">fire twice</span> in quick succession with <span class=\"highlight\">increased damage</span> and velocity. This attack consumes extra ammo.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_express_shot",
  "Name": "Express Shot",
  "ProcAmmoConsumed": 2,
  "ProcBaseAttackDamagePercent": {
    "Scale": {
      "Type": "power_increase",
      "Value": 2
    },
    "Value": 125
  },
  "ProcBaseAttackDamagePercentAltFire": {
    "Scale": {
      "Type": "power_increase",
      "Value": 1.3
    },
    "Value": 40
  },
  "ProcBulletVelocity": 100,
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 15,
    "BonusBulletSpeedPercent": 45,
    "ProcBaseAttackDamagePercent": 75,
    "ProcBaseAttackDamagePercentAltFire": 25
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-data.json",
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
      "lookup": "express shot",
      "name": "Express Shot",
      "type": "item"
    }
  ]
}
````
