---
title: "Torment Pulse"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_tech_damage_pulse"
canonical_name: "Torment Pulse"
snapshot_id: 40200
source_document_id: 7072
payload_hash: "88b7a5dcfd600d5b0248d67cf917513ecdf76043796bbf82dc1d2a6f4eb3df38"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.152957+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Torment Pulse

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_tech_damage_pulse`
- Snapshot ID: `40200`
- Source-Dokument: `7072`
- Kurzinfo: Torment Pulse aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 1.4,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 100,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "DamagePulseAmount": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.23
    },
    "Value": 25
  },
  "DamagePulseRadius": "9m",
  "Description": "Periodically deals {g:citadel_inline_attribute:'SpiritDamage'} to the closest two enemies nearby.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_tech_damage_pulse",
  "MeleeResistPercent": 18,
  "Name": "Torment Pulse",
  "PropertyUpgrades": {
    "BonusHealth": 75,
    "DamagePulseAmount": 30
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Tech",
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
      "lookup": "torment pulse",
      "name": "Torment Pulse",
      "type": "item"
    }
  ]
}
````
