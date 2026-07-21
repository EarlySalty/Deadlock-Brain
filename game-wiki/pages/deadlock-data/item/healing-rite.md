---
title: "Healing Rite"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_health_stimpak"
canonical_name: "Healing Rite"
snapshot_id: 40093
source_document_id: 7072
payload_hash: "8ce7cc703c17cdef06da36b4abcdfa1d5e99659550c0e098ce423e6e3491ae92"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.854687+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Healing Rite

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_health_stimpak`
- Snapshot ID: `40093`
- Source-Dokument: `7072`
- Kurzinfo: Healing Rite aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": "30m",
  "AbilityCooldown": 70,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusSprintSpeed": "2m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": "Grant <span class=\"highlight\">Regen</span> and <span class=\"highlight\">Sprint Speed</span> to the target. Gets dispelled if you take damage from enemy players or objectives. Can be self-cast.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_health_stimpak",
  "Name": "Healing Rite",
  "PropertyUpgrades": {
    "AbilityCooldown": -60,
    "BonusSprintSpeed": "6m",
    "TotalHealthRegen": 600
  },
  "RegenDuration": 20,
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 1,
  "TotalHealthRegen": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.1
    },
    "Value": 300
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
      "lookup": "healing rite",
      "name": "Healing Rite",
      "type": "item"
    }
  ]
}
````
