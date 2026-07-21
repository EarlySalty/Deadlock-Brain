---
title: "Leech"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_damage_recycler"
canonical_name: "Leech"
snapshot_id: 40044
source_document_id: 7072
payload_hash: "7e3cecedd05edb2b65f5e6080f23c57fd0a7dada41d371d218cfae8adaf0d1de"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.738460+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Leech

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_damage_recycler`
- Snapshot ID: `40044`
- Source-Dokument: `7072`
- Kurzinfo: Leech aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityLifestealPercentHero": 25,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 12,
  "BonusHealth": 180,
  "BulletLifestealPercent": 25,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_vampire",
    "upgrade_health_stealing_magic"
  ],
  "Cost": 6400,
  "Description": "Reduces the effect of enemy applied <span class=\"highlight\">healing reduction</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_damage_recycler",
  "Name": "Leech",
  "PropertyUpgrades": {
    "AbilityLifestealPercentHero": 15,
    "BaseAttackDamagePercent": 15,
    "BonusHealth": 200,
    "BulletLifestealPercent": 15,
    "TechPower": 15
  },
  "ShopFilters": [
    "MagicDamage",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechPower": 12,
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
      "lookup": "leech",
      "name": "Leech",
      "type": "item"
    }
  ]
}
````
