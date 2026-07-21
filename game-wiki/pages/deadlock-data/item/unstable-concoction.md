---
title: "Unstable Concoction"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_unstable_concoction"
canonical_name: "Unstable Concoction"
snapshot_id: 40213
source_document_id: 7072
payload_hash: "65baf8dce1c0b8ed76a1b7046210684c01bf680ce08fc121b24d73ebace9fad1"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.186393+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Unstable Concoction

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_unstable_concoction`
- Snapshot ID: `40213`
- Source-Dokument: `7072`
- Kurzinfo: Unstable Concoction aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 25.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BaseAttackDamagePercent": 150,
  "BonusHealth": 3000,
  "BonusMoveSpeed": "10m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "Description": "Consume a concoction that grants you <span class=\"highlight\">Unstoppable</span> and increased <span class=\"highlight\">speed, health, spirit and weapon damage</span>. After a short duration <span class=\"highlight\">you die and explode</span>, stunning nearby enemies and dealing damage based on your maximum health. Dying this way reduces your respawn time by <span class=\"highlight\">50%</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_unstable_concoction",
  "MaxHPDamage": 30,
  "Name": "Unstable Concoction",
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 50,
    "BonusHealth": 1300,
    "StunDuration": 0.5,
    "TechPower": 50
  },
  "Radius": "22m",
  "RespawnTimeMod": 50,
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": true,
  "StunDuration": 3.0,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechPower": 150,
  "Tier": 5,
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
      "lookup": "unstable concoction",
      "name": "Unstable Concoction",
      "type": "item"
    }
  ]
}
````
