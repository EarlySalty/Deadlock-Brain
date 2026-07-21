---
title: "Celestial Blessing"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_celestial_guidance"
canonical_name: "Celestial Blessing"
snapshot_id: 40019
source_document_id: 7072
payload_hash: "b1f44d7c36f234287824f37259034602b463a0395a1ef18745c229587ac49377"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.677691+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Celestial Blessing

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_celestial_guidance`
- Snapshot ID: `40019`
- Source-Dokument: `7072`
- Kurzinfo: Celestial Blessing aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 30,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BuffDuration": 6,
  "BuffMoveSpeedBonus": "5m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "Description": "Applies an extremely powerful cleanse that replenishes your allies globally.",
  "HealPercentAmount": 60,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_celestial_guidance",
  "MinHeal": 400,
  "Name": "Celestial Blessing",
  "PropertyUpgrades": {
    "AbilityCooldown": -20,
    "BuffDuration": 2,
    "MinHeal": 200
  },
  "Radius": "999m",
  "ShopFilters": null,
  "Slot": "Armor",
  "StaminaCooldownReduction": 100,
  "StreetBrawl": true,
  "TargetTypes": [
    "HeroFriendly",
    "TrooperFriendly",
    "MinionFriendly"
  ],
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
      "lookup": "celestial blessing",
      "name": "Celestial Blessing",
      "type": "item"
    }
  ]
}
````
