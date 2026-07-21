---
title: "Restorative Locket"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_restorative_locket"
canonical_name: "Restorative Locket"
snapshot_id: 40160
source_document_id: 7072
payload_hash: "cdd317a9e0db065af8bd69ed1e088699fcf5fed5b82952d62a12bab49b7f8faa"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.040660+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Restorative Locket

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_restorative_locket`
- Snapshot ID: `40160`
- Source-Dokument: `7072`
- Kurzinfo: Restorative Locket aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": "35m",
  "AbilityCooldown": 20.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "HealPerStack": {
    "Scale": {
      "Type": "power_increase",
      "Value": 0.32
    },
    "Value": 16
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_restorative_locket",
  "MaxStacks": 25,
  "MaxStaminaRestore": 3,
  "Name": "Restorative Locket",
  "PropertyUpgrades": {
    "AbilityCooldown": -8,
    "HealPerStack": 30,
    "MaxStaminaRestore": 2,
    "MinStaminaRestore": 2,
    "TechResist": 10
  },
  "Radius": "35m",
  "ShopFilters": [
    "Healing",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "TechResist": 10,
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
      "lookup": "restorative locket",
      "name": "Restorative Locket",
      "type": "item"
    }
  ]
}
````
