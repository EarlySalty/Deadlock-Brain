---
title: "Cold Front"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_cold_front"
canonical_name: "Cold Front"
snapshot_id: 40035
source_document_id: 7072
payload_hash: "e7d5f0c34f88f607cbfb5f3a98ec0f2bf8b376c9557f876264d2a84a66b71c73"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.714915+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Cold Front

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_cold_front`
- Snapshot ID: `40035`
- Source-Dokument: `7072`
- Kurzinfo: Cold Front aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 25.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.465
    },
    "Value": 95
  },
  "DamageHeight": "7m",
  "Description": "Release an expanding ice blast that deals {g:citadel_inline_attribute:'SpiritDamage'} and <span class=\"highlight\">Slows</span> targets it hits.",
  "EndRadius": "10m",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_cold_front",
  "MovementSpeedSlow": 60,
  "NPCDamageMult": 1,
  "Name": "Cold Front",
  "PropertyUpgrades": {
    "AbilityCooldown": -13,
    "Damage": 60,
    "TechResist": 8
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption",
    "Durability"
  ],
  "Slot": "Tech",
  "SpreadDuration": 0.6,
  "StartRadius": "2m",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechResist": 6,
  "Tier": 2,
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
      "lookup": "cold front",
      "name": "Cold Front",
      "type": "item"
    }
  ]
}
````
