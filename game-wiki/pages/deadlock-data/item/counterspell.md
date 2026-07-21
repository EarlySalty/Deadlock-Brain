---
title: "Counterspell"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_counterspell"
canonical_name: "Counterspell"
snapshot_id: 40040
source_document_id: 7072
payload_hash: "b00708db209afd3838c75c0da08490c2dbf5e44c72bddd9248ae4b3b67672e0e"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.729548+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Counterspell

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_counterspell`
- Snapshot ID: `40040`
- Source-Dokument: `7072`
- Kurzinfo: Counterspell aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 23.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 50,
  "BonusMoveSpeed": "1.75m",
  "BuffDuration": 6,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "Your next parry <span class=\"highlight\">protects you from the damage and effects of enemy abilities and items</span>. On a successful spell parry {g:citadel_inline_attribute:'Heal'} and gain {g:citadel_inline_attribute:'MoveSpeed'} and {g:citadel_inline_attribute:'Spirit'}.",
  "HealOnSuccess": 150,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_counterspell",
  "Name": "Counterspell",
  "PropertyUpgrades": {
    "AbilityCooldown": -8,
    "BonusHealth": 150,
    "BonusMoveSpeed": "2m",
    "HealOnSuccess": 250,
    "SpiritPower": 20
  },
  "ShopFilters": [
    "Durability",
    "Melee",
    "Healing"
  ],
  "Slot": "Armor",
  "SpellParryDuration": 0.8,
  "SpiritPower": 20,
  "SpiritPowerInnate": 5,
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
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
      "lookup": "counterspell",
      "name": "Counterspell",
      "type": "item"
    }
  ]
}
````
