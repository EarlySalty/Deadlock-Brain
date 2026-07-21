---
title: "Healing Nova"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_health_nova"
canonical_name: "Healing Nova"
snapshot_id: 40089
source_document_id: 7072
payload_hash: "e9e7ba4b2f2d0a0afd8085fbb5db76e7b5c632fb41acb72b7f3c4d36808c5cb4"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.846528+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Healing Nova

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_health_nova`
- Snapshot ID: `40089`
- Source-Dokument: `7072`
- Kurzinfo: Healing Nova aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCooldown": 60,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "AuraRadius": "18m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_health_stimpak"
  ],
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Heal</span> yourself and nearby allies.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_health_nova",
  "Name": "Healing Nova",
  "PropertyUpgrades": {
    "TechPower": 12,
    "TechRadiusMultiplier": 12,
    "TechRangeMultiplier": 12,
    "TotalHealthRegen": 425
  },
  "RegenDuration": 2,
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Armor",
  "SpiritPower": 8,
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly",
    "TrooperFriendly",
    "MinionFriendly"
  ],
  "TechRadiusMultiplier": 5,
  "TechRangeMultiplier": 5,
  "Tier": 3,
  "TotalHealthRegen": {
    "Scale": {
      "Type": "power_increase",
      "Value": 6
    },
    "Value": 325
  },
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
      "lookup": "healing nova",
      "name": "Healing Nova",
      "type": "item"
    }
  ]
}
````
