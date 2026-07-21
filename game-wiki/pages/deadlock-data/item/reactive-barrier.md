---
title: "Reactive Barrier"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_vex_barrier"
canonical_name: "Reactive Barrier"
snapshot_id: 40217
source_document_id: 7072
payload_hash: "4d05fb9c1e49e0661f24e0715eba37e64f90197bfcd70b93cfb1667820d94aa3"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.199243+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Reactive Barrier

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_vex_barrier`
- Snapshot ID: `40217`
- Source-Dokument: `7072`
- Kurzinfo: Reactive Barrier aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 55,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 10,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_grit"
  ],
  "Cost": 1600,
  "Description": "Gain a <span class=\"highlight\">Barrier</span> when you are <span class=\"highlight\">Stunned, Chained, Immobilized, Slept or Silenced</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_vex_barrier",
  "Name": "Reactive Barrier",
  "OutOfCombatHealthRegen": 1.0,
  "PropertyUpgrades": {
    "AbilityCooldown": -15,
    "VexBarrierCombatBarrier": 375
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "VexBarrierCombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.8
    },
    "Value": 325
  },
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
      "lookup": "reactive barrier",
      "name": "Reactive Barrier",
      "type": "item"
    }
  ]
}
````
