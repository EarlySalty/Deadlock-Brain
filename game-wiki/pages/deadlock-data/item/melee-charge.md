---
title: "Melee Charge"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_melee_charge"
canonical_name: "Melee Charge"
snapshot_id: 40125
source_document_id: 7072
payload_hash: "8bffcaaff3845b0e1a07cb6ae591c91faf63e7303632a97658e17b8d21098778"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.940355+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Melee Charge

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_melee_charge`
- Snapshot ID: `40125`
- Source-Dokument: `7072`
- Kurzinfo: Melee Charge aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHeavyMeleeDamage": 25,
  "BonusMeleeDamagePercent": 10,
  "BulletResist": 6,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "Your next <span class=\"highlight\">Heavy Melee</span> attack against an enemy <span class=\"highlight\">deals increased damage</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_melee_charge",
  "MeleeDistanceScale": 50,
  "Name": "Melee Charge",
  "PropertyUpgrades": {
    "BonusHeavyMeleeDamage": 15,
    "BulletResist": 12,
    "MeleeDistanceScale": 30
  },
  "ShopFilters": [
    "Melee"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "TrooperEnemy",
    "MinionEnemy",
    "Neutral"
  ],
  "Tier": 2,
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
      "lookup": "melee charge",
      "name": "Melee Charge",
      "type": "item"
    }
  ]
}
````
