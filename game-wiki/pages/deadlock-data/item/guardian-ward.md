---
title: "Guardian Ward"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_guardian_ward"
canonical_name: "Guardian Ward"
snapshot_id: 40078
source_document_id: 7072
payload_hash: "db8d1434556daec39d7f60e5fbc3aef1d32cabb53e0abf622fd8272963999f7f"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.819449+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Guardian Ward

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_guardian_ward`
- Snapshot ID: `40078`
- Source-Dokument: `7072`
- Kurzinfo: Guardian Ward aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": "40m",
  "AbilityCooldown": 60,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusMoveSpeed": "2.75m",
  "BuffDuration": 6,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_grit"
  ],
  "CooldownReductionPctOnOthers": 50,
  "Cost": 1600,
  "Description": "Provide the target with a <span class=\"highlight\">Barrier</span> and temporary <span class=\"highlight\">Move Speed</span>. <span class=\"diminish\"><br>Can be self-cast.<br>Cooldown is reduced by half when cast on someone else.</span>",
  "GuardianWardCombatBarrier": 250,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_guardian_ward",
  "Name": "Guardian Ward",
  "OutOfCombatHealthRegen": 1.5,
  "PropertyUpgrades": {
    "AbilityCooldown": -12,
    "ChannelMoveSpeed": 2,
    "GuardianWardCombatBarrier": 250,
    "TechRadiusMultiplier": 12,
    "TechRangeMultiplier": 12
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
  "TechRadiusMultiplier": 8,
  "TechRangeMultiplier": 8,
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
      "lookup": "guardian ward",
      "name": "Guardian Ward",
      "type": "item"
    }
  ]
}
````
