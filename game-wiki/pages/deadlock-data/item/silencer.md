---
title: "Silencer"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_proc_silence"
canonical_name: "Silencer"
snapshot_id: 40144
source_document_id: 7072
payload_hash: "6b0889c48678c3874ee40dfbe273c770721cfcca4ba968e80402181165ba1ea6"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.998006+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Silencer

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_proc_silence`
- Snapshot ID: `40144`
- Source-Dokument: `7072`
- Kurzinfo: Silencer aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BuildUpDuration": 5,
  "BuildUpPerShot": 1.04,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "DebuffDuration": 6,
  "Description": "Your bullets build up to a <span class=\"highlight\">Silence</span>. Victims are immune to the build up for <span class=\"highlight\">10s</span> after silence expires.",
  "ImmunityDuration": 10,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_proc_silence",
  "Name": "Silencer",
  "PropertyUpgrades": {
    "SilenceDuration": 1.25,
    "TechDamageReduction": -15,
    "TechResist": 15
  },
  "ShopFilters": [
    "Disruption",
    "WeaponDamage"
  ],
  "SilenceDuration": 2.5,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "TechDamageReduction": -25,
  "TechResist": 12,
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
      "lookup": "silencer",
      "name": "Silencer",
      "type": "item"
    }
  ]
}
````
