---
title: "Bullet Resilience"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_improved_bullet_armor"
canonical_name: "Bullet Resilience"
snapshot_id: 40100
source_document_id: 7072
payload_hash: "87e7f01d4901f71cd50618cd865c30aaebca6ca2666456c0eab652596a72c099"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.870963+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Bullet Resilience

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_improved_bullet_armor`
- Snapshot ID: `40100`
- Source-Dokument: `7072`
- Kurzinfo: Bullet Resilience aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BulletResist": 30,
  "BulletResistBelowThreshold": 15,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "When below <span class=\"highlight\">50% health</span>, gain additional <span class=\"highlight\">Bullet Resist</span>.",
  "HealthThreshold": 50,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_improved_bullet_armor",
  "Name": "Bullet Resilience",
  "OutOfCombatHealthRegen": 3,
  "PropertyUpgrades": {
    "BulletResist": 10,
    "BulletResistBelowThreshold": 10
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
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
      "lookup": "bullet resilience",
      "name": "Bullet Resilience",
      "type": "item"
    }
  ]
}
````
