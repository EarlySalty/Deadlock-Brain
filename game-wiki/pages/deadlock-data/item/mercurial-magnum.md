---
title: "Mercurial Magnum"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_ethereal_bullets"
canonical_name: "Mercurial Magnum"
snapshot_id: 40061
source_document_id: 7072
payload_hash: "de3a2836f26b7bc1ef9b415b5577b3c877cf9a3df10f31e571ae58f8605edb03"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.779811+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Mercurial Magnum

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_ethereal_bullets`
- Snapshot ID: `40061`
- Source-Dokument: `7072`
- Kurzinfo: Mercurial Magnum aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChargeUpTime": 14,
  "AbilityCooldown": 15,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "AmmoReloadPercent": 100,
  "BonusClipSizePercent": 20,
  "BonusFireRate": 22,
  "BuffDuration": 12,
  "BulletsBonusMagicDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.49
    },
    "Value": 25
  },
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_quick_silver"
  ],
  "Cost": 6400,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.16
    },
    "Value": 60
  },
  "Description": "Your imbued ability charges up over time with {g:citadel_inline_attribute:'BonusSpiritDamage'}, {g:citadel_inline_attribute:'BonusFireRate'}, and <span class=\"highlight\">reloads bullets</span> on use. Until your next reload, your <span class=\"highlight\">bullets deal {g:citadel_inline_attribute:'BonusSpiritDamage'}</span> based on your Spirit Power.",
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_ethereal_bullets",
  "Name": "Mercurial Magnum",
  "PropertyUpgrades": {
    "BonusClipSizePercent": 60,
    "BonusFireRate": 20,
    "BulletsBonusMagicDamage": 20,
    "Damage": 120,
    "TechPower": 15
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
  "TechPower": 7,
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
      "lookup": "mercurial magnum",
      "name": "Mercurial Magnum",
      "type": "item"
    }
  ]
}
````
