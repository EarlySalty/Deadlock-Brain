---
title: "Blood Tribute"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_blood_tribute"
canonical_name: "Blood Tribute"
snapshot_id: 40005
source_document_id: 7072
payload_hash: "da5fbda709b6c9a51bcc688ded00e2e17d9c4cbf0493c06aad32c31cc4459864"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.647654+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Blood Tribute

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_blood_tribute`
- Snapshot ID: `40005`
- Source-Dokument: `7072`
- Kurzinfo: Blood Tribute aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCastToggle",
  "BonusFireRate": 35,
  "BonusMoveSpeed": "2.0m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "Toggle: Continually sacrifice Health to improve {g:citadel_inline_attribute:'FireRate'}, <span class=\"highlight\">Debuff Resistance</span> and <span class=\"highlight\">Move Speed</span>.",
  "HealthDrainedPerSecond": 50,
  "InnateStatusResistancePercent": 8,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_blood_tribute",
  "Name": "Blood Tribute",
  "OutOfCombatHealthRegen": 4,
  "PropertyUpgrades": {
    "BonusFireRate": 30,
    "HealthDrainedPerSecond": -20,
    "OutOfCombatHealthRegen": 8,
    "TechResist": 14
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StatusResistancePercent": 35,
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechResist": 8,
  "TickRate": 0.1,
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
      "lookup": "blood tribute",
      "name": "Blood Tribute",
      "type": "item"
    }
  ]
}
````
