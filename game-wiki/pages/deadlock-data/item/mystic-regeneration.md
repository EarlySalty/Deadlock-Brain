---
title: "Mystic Regeneration"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_mystic_regeneration"
canonical_name: "Mystic Regeneration"
snapshot_id: 40129
source_document_id: 7072
payload_hash: "e4e97b28bf3cdec2547999118d60d011f9e3ebe42c2f99bd782395fa308219c9"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.951223+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Mystic Regeneration

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_mystic_regeneration`
- Snapshot ID: `40129`
- Source-Dokument: `7072`
- Kurzinfo: Mystic Regeneration aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 50,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": "Dealing {g:citadel_inline_attribute:'SpiritDamage'} to enemy Heroes grants you Bonus {g:citadel_inline_attribute:'Regen'}. Stacks when dealing damage to different heroes.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_mystic_regeneration",
  "Name": "Mystic Regeneration",
  "PropertyUpgrades": {
    "BonusHealth": 150,
    "Regeneration": 8
  },
  "Regeneration": 4,
  "RegenerationDuration": 7,
  "ShopFilters": [
    "MagicDamage",
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
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
      "lookup": "mystic regeneration",
      "name": "Mystic Regeneration",
      "type": "item"
    }
  ]
}
````
