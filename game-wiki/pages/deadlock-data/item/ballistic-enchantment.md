---
title: "Ballistic Enchantment"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_bulletshredimbue"
canonical_name: "Ballistic Enchantment"
snapshot_id: 40013
source_document_id: 7072
payload_hash: "fdfb0b18555f514df85eebfbb4bdd641a440e3f2c6b99f06c553dcd8d7cd33f6"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.664929+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Ballistic Enchantment

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_bulletshredimbue`
- Snapshot ID: `40013`
- Source-Dokument: `7072`
- Kurzinfo: Ballistic Enchantment aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 14,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_magic_reach"
  ],
  "Cost": 3200,
  "Description": "Imbue an ability with increased <span class=\"highlight\">range</span>. Dealing damage with that ability grants you increased <span class=\"highlight\">weapon damage</span> per <span class=\"highlight\">unique hero hit</span>. Has reduced effect on non-heroes.",
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_bulletshredimbue",
  "Name": "Ballistic Enchantment",
  "NonHeroStackLimit": 8,
  "PropertyUpgrades": {
    "TechRadiusMultiplier": 15,
    "TechRangeMultiplier": 15,
    "WeaponPowerPerStack": 15
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechRadiusMultiplier": 22,
  "TechRangeMultiplier": 22,
  "Tier": 3,
  "WeaponPowerPerStack": 20,
  "WeaponPowerPerStackNonHero": 5,
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
      "lookup": "ballistic enchantment",
      "name": "Ballistic Enchantment",
      "type": "item"
    }
  ]
}
````
