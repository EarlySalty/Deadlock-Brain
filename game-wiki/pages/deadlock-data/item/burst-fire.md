---
title: "Burst Fire"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_burst_fire"
canonical_name: "Burst Fire"
snapshot_id: 40014
source_document_id: 7072
payload_hash: "60b960c6b733700760b41f5e0d45214111d6c271777577f2d316aada981c16a3"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.667219+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Burst Fire

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_burst_fire`
- Snapshot ID: `40014`
- Source-Dokument: `7072`
- Kurzinfo: Burst Fire aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 9,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4.5,
  "AbilityUnitTargetLimit": 1,
  "ActivatedFireRate": 32,
  "Activation": "Passive",
  "BonusFireRate": 10,
  "BonusMoveSpeed": "1.25m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_rapid_rounds"
  ],
  "Cost": 3200,
  "Description": "Briefly gain <span class=\"highlight\">Fire Rate</span> and <span class=\"highlight\">Move Speed</span> when one of your bullets hits an enemy hero.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_burst_fire",
  "Name": "Burst Fire",
  "PropertyUpgrades": {
    "AbilityCooldown": -1,
    "ActivatedFireRate": 15,
    "BonusFireRate": 14,
    "BonusMoveSpeed": "1.5m",
    "SlideScale": 50
  },
  "ShopFilters": [
    "FireRate"
  ],
  "SlideScale": 50,
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
      "lookup": "burst fire",
      "name": "Burst Fire",
      "type": "item"
    }
  ]
}
````
