---
title: "Mystic Shot"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_crackshot"
canonical_name: "Mystic Shot"
snapshot_id: 40041
source_document_id: 7072
payload_hash: "77bbad67be252de63e89dac56b5715a66eef18804a8c4aa9a31d600d6a634fc4"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.732027+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Mystic Shot

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_crackshot`
- Snapshot ID: `40041`
- Source-Dokument: `7072`
- Kurzinfo: Mystic Shot aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 9,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "Your next bullet deals bonus {g:citadel_inline_attribute:'SpiritDamage'}.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_crackshot",
  "Name": "Mystic Shot",
  "ProcBonusMagicDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.2
    },
    "Value": 40
  },
  "ProcChance": 100,
  "ProcCooldown": 1,
  "PropertyUpgrades": {
    "ProcBonusMagicDamage": 109,
    "SpiritPower": 14
  },
  "Radius": "1m",
  "ShopFilters": [
    "WeaponDamage",
    "MagicDamage"
  ],
  "Slot": "Weapon",
  "SpiritPower": 7,
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
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
      "lookup": "mystic shot",
      "name": "Mystic Shot",
      "type": "item"
    }
  ]
}
````
