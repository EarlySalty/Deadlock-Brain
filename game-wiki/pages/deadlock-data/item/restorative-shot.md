---
title: "Restorative Shot"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_medic_bullets"
canonical_name: "Restorative Shot"
snapshot_id: 40124
source_document_id: 7072
payload_hash: "062505acb32557b477da6a1296fdd5034bb689f4c4a116dfc5f5b3e46df5c187"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.936872+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Restorative Shot

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_medic_bullets`
- Snapshot ID: `40124`
- Source-Dokument: `7072`
- Kurzinfo: Restorative Shot aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 6,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 6,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": "Your next bullet will <span class=\"highlight\">heal</span> you based on what target you hit.",
  "HealFromHero": 50,
  "HealFromNPC": 20,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_medic_bullets",
  "Name": "Restorative Shot",
  "ProcChance": 100,
  "PropertyUpgrades": {
    "HealFromHero": 100,
    "HealFromNPC": 40
  },
  "Radius": "1m",
  "ShopFilters": [
    "WeaponDamage",
    "Healing"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
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
      "lookup": "restorative shot",
      "name": "Restorative Shot",
      "type": "item"
    }
  ]
}
````
