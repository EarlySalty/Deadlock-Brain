---
title: "Spellbreaker"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_spellbreaker"
canonical_name: "Spellbreaker"
snapshot_id: 40180
source_document_id: 7072
payload_hash: "1a02adfbac7346bcf7f2e8d87f0851ffb60e0e97acb75f530963f082f502d0a7"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.099389+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Spellbreaker

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_spellbreaker`
- Snapshot ID: `40180`
- Source-Dokument: `7072`
- Kurzinfo: Spellbreaker aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 9,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 90,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_debuff_reducer"
  ],
  "Cost": 6400,
  "DamageThreshold": 175,
  "Description": "The next instance of high {g:citadel_inline_attribute:'SpiritDamage'} you take is significantly reduced.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spellbreaker",
  "Name": "Spellbreaker",
  "PropertyUpgrades": {
    "AbilityCooldown": -3,
    "StatusResistancePercent": 15,
    "TechResist": 15
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "SpiritDamageReductionProc": 65,
  "StatusResistancePercent": 25,
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechResist": 18,
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
      "lookup": "spellbreaker",
      "name": "Spellbreaker",
      "type": "item"
    }
  ]
}
````
