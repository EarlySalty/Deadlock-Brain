---
title: "Divine Barrier"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_divine_barrier"
canonical_name: "Divine Barrier"
snapshot_id: 40049
source_document_id: 7072
payload_hash: "577d61fc9c176d759f941222711252cbbcff426b75e519aee19716984d15ca5a"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.749474+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Divine Barrier

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_divine_barrier`
- Snapshot ID: `40049`
- Source-Dokument: `7072`
- Kurzinfo: Divine Barrier aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": "40m",
  "AbilityCooldown": 45,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusMoveSpeed": "2.75m",
  "BuffDuration": 6,
  "ChannelMoveSpeed": -1,
  "CombatBarrier": 600,
  "Components": [
    "upgrade_guardian_ward"
  ],
  "CooldownReductionPctOnOthers": 50,
  "Cost": 6400,
  "Description": "<span class=\"highlight\">Remove all non-stun debuffs</span> from the target and provide them with a <span class=\"highlight\">Barrier</span> and <span class=\"highlight\">Move Speed</span>. <span class=\"diminish\"><br>Can be self-cast. Cooldown is reduced by half when cast on someone else.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_divine_barrier",
  "Name": "Divine Barrier",
  "OutOfCombatHealthRegen": 1.5,
  "PropertyUpgrades": {
    "AbilityCooldown": -27,
    "TechRadiusMultiplier": 10,
    "TechRangeMultiplier": 10
  },
  "ShopFilters": [
    "Durability",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "TechRadiusMultiplier": 10,
  "TechRangeMultiplier": 10,
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
      "lookup": "divine barrier",
      "name": "Divine Barrier",
      "type": "item"
    }
  ]
}
````
