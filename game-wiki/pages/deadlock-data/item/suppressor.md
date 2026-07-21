---
title: "Suppressor"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_suppressor"
canonical_name: "Suppressor"
snapshot_id: 40194
source_document_id: 7072
payload_hash: "984d82f9f1426f06cf4b7d59a9935f637053092b4fe148e6bb0e4be02da68928"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.137276+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Suppressor

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_suppressor`
- Snapshot ID: `40194`
- Source-Dokument: `7072`
- Kurzinfo: Suppressor aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BulletResist": 8,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "When you deal {g:citadel_inline_attribute:'SpiritDamage'} to enemies, you also reduce their <span class=\"highlight\">Fire Rate</span>.",
  "FireRateSlow": 28,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_suppressor",
  "Name": "Suppressor",
  "PropertyUpgrades": {
    "BulletResist": 16,
    "FireRateSlow": 20,
    "TechPower": 12
  },
  "ShopFilters": [
    "Healing",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechPower": 6,
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
      "lookup": "suppressor",
      "name": "Suppressor",
      "type": "item"
    }
  ]
}
````
