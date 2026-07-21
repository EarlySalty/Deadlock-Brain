---
title: "Tankbuster"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_magic_shock"
canonical_name: "Tankbuster"
snapshot_id: 40119
source_document_id: 7072
payload_hash: "65fda72a00d971e3779f7b91f1f0e60c0c84ec5764bf3a4ffbb6ba88cd0a77e9"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.921647+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Tankbuster

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_shock`
- Snapshot ID: `40119`
- Source-Dokument: `7072`
- Kurzinfo: Tankbuster aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChargeUpTime": 14,
  "AbilityCooldown": 14,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 50,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_magic_burst"
  ],
  "Cost": 3200,
  "CurrentHealthDamage": 8,
  "Damage": 40,
  "Description": "Charges up over time with {g:citadel_inline_attribute:'BonusSpiritDamage'}, causing abilities dealing more than <span class=\"highlight\">165</span> damage to deal additional damage. <span class=\"highlight\">Ignores Spirit Resistance.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_magic_shock",
  "MinimumDamage": 165,
  "Name": "Tankbuster",
  "PropertyUpgrades": {
    "BonusHealth": 100,
    "CurrentHealthDamage": 5,
    "Damage": 60
  },
  "ReProcLockoutTime": 5,
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "WatcherMaxDuration": 30,
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
      "lookup": "tankbuster",
      "name": "Tankbuster",
      "type": "item"
    }
  ]
}
````
