---
title: "Split Shot"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_split_shot"
canonical_name: "Split Shot"
snapshot_id: 40187
source_document_id: 7072
payload_hash: "7abcd056a1bc7c7707f9a98c6f24d0d4c1b3d0fb86f029e372a97b48e0c1ae22"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.118208+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Split Shot

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_split_shot`
- Snapshot ID: `40187`
- Source-Dokument: `7072`
- Kurzinfo: Split Shot aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 27,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BonusShotsDuration": 5,
  "BulletSplitShot": 5,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "Make your weapon fire <span class=\"highlight\">multishot</span>. <br><br> Hitting more than one Hero per attack will grant a <span class=\"highlight\">stacking weapon damage bonus</span>. <br><br><span class=\"diminish\">Targets can only be hit once per multishot.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_split_shot",
  "MaxStacks": 5,
  "Name": "Split Shot",
  "PropertyUpgrades": {
    "AbilityCooldown": -8,
    "BulletSplitShot": 4,
    "WeaponDamagePerStack": 8
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "SpreadAngleDegrees": 45,
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "WeaponDamageBonusDuration": 12,
  "WeaponDamagePerStack": 8,
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
      "lookup": "split shot",
      "name": "Split Shot",
      "type": "item"
    }
  ]
}
````
