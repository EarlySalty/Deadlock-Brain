---
title: "Kinetic Dash"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_kinetic_sash"
canonical_name: "Kinetic Dash"
snapshot_id: 40109
source_document_id: 7072
payload_hash: "8bc6d2ac069d36e257f6c700ec43fe0be6304587747f998aa79500cfaf7d53e2"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.895075+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Kinetic Dash

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_kinetic_sash`
- Snapshot ID: `40109`
- Source-Dokument: `7072`
- Kurzinfo: Kinetic Dash aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 7,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusClipSize": 6,
  "BonusFireRate": 25,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_improved_stamina"
  ],
  "Cost": 1600,
  "Description": "When you <span class=\"highlight\">Dash-Jump</span> you gain <span class=\"highlight\">Fire Rate</span> and bonus <span class=\"highlight\">Ammo</span> until your next reload. Lasts up to 7s.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_kinetic_sash",
  "Name": "Kinetic Dash",
  "PropertyUpgrades": {
    "BonusClipSize": 6,
    "BonusFireRate": 20,
    "Stamina": 1,
    "StaminaCooldownReduction": 14
  },
  "ShopFilters": [
    "FireRate",
    "ClipSize",
    "Movement",
    "Durability"
  ],
  "Slot": "Weapon",
  "Stamina": 1,
  "StaminaCooldownReduction": 12,
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "kinetic dash",
      "name": "Kinetic Dash",
      "type": "item"
    }
  ]
}
````
