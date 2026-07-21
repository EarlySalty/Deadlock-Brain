---
title: "Hunter's Aura"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_bullet_armor_reduction_aura"
canonical_name: "Hunter's Aura"
snapshot_id: 40010
source_document_id: 7072
payload_hash: "54b8a449dda59af6ae7be301a52bb792d1ec5ec395434464e34347e134542ce5"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.658204+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Hunter's Aura

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_bullet_armor_reduction_aura`
- Snapshot ID: `40010`
- Source-Dokument: `7072`
- Kurzinfo: Hunter's Aura aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 100,
  "BonusSprintSpeed": "0.75m",
  "BulletArmorReduction": -10,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "Reduces nearby enemies' <span class=\"highlight\">Bullet Resist and Fire Rate</span>. If there is only one enemy hero nearby, this <span class=\"highlight\">effect is doubled</span>.",
  "FireRateSlow": 15,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_bullet_armor_reduction_aura",
  "Name": "Hunter's Aura",
  "PropertyUpgrades": {
    "BonusHealth": 125,
    "BonusSprintSpeed": "3m",
    "BulletArmorReduction": -6,
    "FireRateSlow": 5
  },
  "Radius": "15m",
  "ShopFilters": [
    "WeaponDamage",
    "Disruption",
    "ClipSize"
  ],
  "SingleTargetPlayerMultiplier": 2,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy",
    "MinionEnemy"
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
      "lookup": "hunter's aura",
      "name": "Hunter's Aura",
      "type": "item"
    }
  ]
}
````
