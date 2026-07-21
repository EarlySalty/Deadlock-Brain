---
title: "Soul Explosion"
entity_type: "item_special"
source: "deadlock_data"
external_id: "upgrade_corpse_explosion"
canonical_name: "Soul Explosion"
snapshot_id: 40039
source_document_id: 7072
payload_hash: "b5bd5e5c897a6d30828156fbbe70fdbd1bf76a73d9ea5ce1b51392b6a76d9b24"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.726684+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_special"]
---

# Soul Explosion

## Kurzueberblick

- Typ: `item_special`
- Quelle: `deadlock_data`
- External ID: `upgrade_corpse_explosion`
- Snapshot ID: `40039`
- Source-Dokument: `7072`
- Kurzinfo: Soul Explosion aus `deadlock_data` / `item_special` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ArmingTime": 0.1,
  "BonusHealth": 110,
  "BonusHealthRegen": 3,
  "BonusSprintSpeed": "1m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "Kills or assists cause an <span class=\"highlight\">explosion</span> where the victim dies. Kills against heroes have greater radius and damage.",
  "ExplosionDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.93
    },
    "Value": 125
  },
  "ExplosionRadius": "4m",
  "HeroMultiplier": 150,
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_corpse_explosion",
  "Name": "Soul Explosion",
  "ShopFilters": [
    "MagicDamage",
    "Movement"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechPower": 6,
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
