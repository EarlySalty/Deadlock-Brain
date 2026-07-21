---
title: "Unstoppable"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_unstoppable"
canonical_name: "Unstoppable"
snapshot_id: 40214
source_document_id: 7072
payload_hash: "3e9c60e6de4c1689395685a8188b573ee299743cee164a44441a41ea52947dd8"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.190130+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Unstoppable

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_unstoppable`
- Snapshot ID: `40214`
- Source-Dokument: `7072`
- Kurzinfo: Unstoppable aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 60.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5.5,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BonusHealth": 125,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_debuff_reducer"
  ],
  "Cost": 6400,
  "Description": "Temporarily suppress <span class=\"highlight\">negative status effects</span> and become <span class=\"highlight\">immune</span> to <span class=\"highlight\">Stun, Silence, Sleep, Root, and Disarm</span>. <br>Cannot be used while <span class=\"highlight\">Stunned</span> or <span class=\"highlight\">Slept</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_unstoppable",
  "Name": "Unstoppable",
  "PropertyUpgrades": {
    "AbilityCooldown": -35,
    "AbilityDuration": 1.25,
    "BonusHealth": 75,
    "StatusResistancePercent": 15
  },
  "ShopFilters": [
    "Durability",
    "Movement"
  ],
  "Slot": "Armor",
  "StatusResistancePercent": 25,
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "unstoppable",
      "name": "Unstoppable",
      "type": "item"
    }
  ]
}
````
