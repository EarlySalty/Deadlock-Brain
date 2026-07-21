---
title: "Dispel Magic"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_reduce_debuff_duration"
canonical_name: "Dispel Magic"
snapshot_id: 40152
source_document_id: 7072
payload_hash: "389dd6b83e51d17f4c38d9cf91e90f0a506614577f6e3275bcf2fa8745ab5fd8"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.019826+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Dispel Magic

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_reduce_debuff_duration`
- Snapshot ID: `40152`
- Source-Dokument: `7072`
- Kurzinfo: Dispel Magic aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 45.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "ActiveBonusMoveSpeed": "2m",
  "BuffDuration": 3,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "HealOnActivate": 250,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_reduce_debuff_duration",
  "Name": "Dispel Magic",
  "PropertyUpgrades": {
    "AbilityCooldown": -25,
    "HealOnActivate": 150,
    "TechResist": 20
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechResist": 10,
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
      "lookup": "dispel magic",
      "name": "Dispel Magic",
      "type": "item"
    }
  ]
}
````
