---
title: "Hex-Sealed Knuckles"
entity_type: "item_special"
source: "deadlock_data"
external_id: "upgrade_superacolytes_glove"
canonical_name: "Hex-Sealed Knuckles"
snapshot_id: 40192
source_document_id: 7072
payload_hash: "86b5b889273d21ca587cd74f35f2812312e3ffb588d345371890a6b6498e05f2"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.131382+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_special"]
---

# Hex-Sealed Knuckles

## Kurzueberblick

- Typ: `item_special`
- Quelle: `deadlock_data`
- External ID: `upgrade_superacolytes_glove`
- Snapshot ID: `40192`
- Source-Dokument: `7072`
- Kurzinfo: Hex-Sealed Knuckles aus `deadlock_data` / `item_special` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 3,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 13,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 75,
  "BonusHealthRegen": 1,
  "BonusMeleeDamagePercent": 30,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "After dealing an accumulated <span class=\"highlight\">{s:StoredSpiritDamage}</span> spirit damage, your next <span class=\"highlight\">Heavy Melee Attack</span> deals an additional <span class=\"highlight\">{s:StoredSpiritDamage} spirit damage</span>.",
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_superacolytes_glove",
  "Name": "Hex-Sealed Knuckles",
  "ShopFilters": [
    "MagicDamage",
    "Melee"
  ],
  "Slot": "Tech",
  "StoredSpiritDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.93
    },
    "Value": 200
  },
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
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
  }
}
````
