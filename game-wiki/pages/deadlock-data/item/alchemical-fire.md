---
title: "Alchemical Fire"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_thermal_detonator"
canonical_name: "Alchemical Fire"
snapshot_id: 40205
source_document_id: 7072
payload_hash: "33a49678ce9a4859035cdd1a31042889ae0e9603c3876198a6d299a012a006e4"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.164857+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Alchemical Fire

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_thermal_detonator`
- Snapshot ID: `40205`
- Source-Dokument: `7072`
- Kurzinfo: Alchemical Fire aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 30.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BulletArmorReduction": {
    "Scale": {
      "Type": "spirit",
      "Value": -0.055
    },
    "Value": -7
  },
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.2
    },
    "Value": 45
  },
  "DPSIncrease": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.04
    },
    "Value": 7
  },
  "DPSMax": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.4
    },
    "Value": 95
  },
  "Description": "Throw a flask that explodes on contact, creating an area that does increasing {g:citadel_inline_attribute:'SpiritDamage'} <span class=\"highlight\">per second</span> and reduces enemy <span class=\"highlight\">Bullet Resist</span>.<br><br>50% less effective vs non-heroes.",
  "HeightOffGround": 50,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_thermal_detonator",
  "Name": "Alchemical Fire",
  "NonHeroReductionPercent": 50,
  "PropertyUpgrades": {
    "BulletArmorReduction": -8,
    "DPS": 30,
    "DPSMax": 30,
    "SpiritPower": 15
  },
  "Radius": "10m",
  "ShopFilters": [
    "WeaponDamage",
    "MagicDamage"
  ],
  "Slot": "Weapon",
  "SpiritPower": 10,
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TickRate": 0.5,
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
      "lookup": "alchemical fire",
      "name": "Alchemical Fire",
      "type": "item"
    }
  ]
}
````
