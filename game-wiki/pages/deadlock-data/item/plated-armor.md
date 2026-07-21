---
title: "Plated Armor"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_deflecting_armor"
canonical_name: "Plated Armor"
snapshot_id: 40046
source_document_id: 7072
payload_hash: "43feac162a72b0c7e5b39fdd1381caf0fed7a15c91b923ce2924a2eb66255e1e"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.742895+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Plated Armor

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_deflecting_armor`
- Snapshot ID: `40046`
- Source-Dokument: `7072`
- Kurzinfo: Plated Armor aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 1,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 130,
  "BulletProcDeflectionPercent": 50,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "DeflectionPercent": 30,
  "DeflectionRandomness": 1,
  "Description": "Gain a chance to either deflect incoming bullets, preventing all {g:citadel_inline_attribute:'WeaponDamage'} or prevent all <span class=\"highlight\">on-hit effects</span> from bullets.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_deflecting_armor",
  "Name": "Plated Armor",
  "PropertyUpgrades": {
    "BulletProcDeflectionPercent": 15,
    "DeflectionPercent": 15
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
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
      "lookup": "plated armor",
      "name": "Plated Armor",
      "type": "item"
    }
  ]
}
````
