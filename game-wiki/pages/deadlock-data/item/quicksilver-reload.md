---
title: "Quicksilver Reload"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_quick_silver"
canonical_name: "Quicksilver Reload"
snapshot_id: 40147
source_document_id: 7072
payload_hash: "f3f3b2c97485abca84d10c53f8513844b50bae10692ebf1cef1fbdf4c3302e54"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.005999+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Quicksilver Reload

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_quick_silver`
- Snapshot ID: `40147`
- Source-Dokument: `7072`
- Kurzinfo: Quicksilver Reload aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChargeUpTime": 18,
  "AbilityCooldown": 18,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "AmmoReloadPercent": 100,
  "BonusFireRate": 10,
  "BuffDuration": 12,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.16
    },
    "Value": 44
  },
  "Description": "Your imbued ability charges up over time with {g:citadel_inline_attribute:'BonusSpiritDamage'}, {g:citadel_inline_attribute:'BonusFireRate'}, and <span class=\"highlight\">reloads bullets</span> on use.",
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_quick_silver",
  "Name": "Quicksilver Reload",
  "PropertyUpgrades": {
    "AbilityChargeUpTime": -4,
    "AbilityCooldown": -4,
    "BonusFireRate": 20,
    "Damage": 56
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
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
      "lookup": "quicksilver reload",
      "name": "Quicksilver Reload",
      "type": "item"
    }
  ]
}
````
