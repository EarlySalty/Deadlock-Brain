---
title: "Capacitor"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_capacitor"
canonical_name: "Capacitor"
snapshot_id: 40017
source_document_id: 7072
payload_hash: "a744342a237ff2ed60e733b37e7cd581d33dae7dc68bdddda0f844c5cc77cb1a"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.673340+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Capacitor

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_capacitor`
- Snapshot ID: `40017`
- Source-Dokument: `7072`
- Kurzinfo: Capacitor aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 40,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BonusFireRate": 5,
  "BonusPerChain": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.19
    },
    "Value": 43
  },
  "ChainCount": 6,
  "ChainRadius": "10m",
  "ChainTickRate": 0.4,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_chain_lightning"
  ],
  "Cost": 6400,
  "Damage": 100,
  "DamagePerChain": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.19
    },
    "Value": 43
  },
  "Description": "Launch a projectile that deals <span class=\"highlight\">{g:citadel_inline_attribute:'SpiritIcon'}damage</span>, applies a strong slow that recovers over time, <span class=\"highlight\">prevents Stamina usage</span> and <span class=\"highlight\">Silences</span> their <span class=\"highlight\">movement-based items and abilities</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_capacitor",
  "MaxSlowPercent": 75,
  "Name": "Capacitor",
  "ProcChance": 20,
  "ProcCooldown": 0.2,
  "PropertyUpgrades": {
    "AbilityCooldown": -32,
    "BonusFireRate": 15,
    "Damage": 25,
    "DamagePerChain": 25,
    "ProcChance": 5
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Weapon",
  "SlowDuration": 3,
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy",
    "BossEnemy",
    "MinionEnemy"
  ],
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
      "lookup": "capacitor",
      "name": "Capacitor",
      "type": "item"
    }
  ]
}
````
