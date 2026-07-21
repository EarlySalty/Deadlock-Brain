---
title: "Ricochet"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_ricochet"
canonical_name: "Ricochet"
snapshot_id: 40162
source_document_id: 7072
payload_hash: "f886875c09c8287a1a6e8bde3d94b1ac31ff48cc944ac437a6a37bc03002554f"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.045885+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Ricochet

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_ricochet`
- Snapshot ID: `40162`
- Source-Dokument: `7072`
- Kurzinfo: Ricochet aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusFireRate": 18,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "Description": "Your bullets will <span class=\"highlight\">ricochet</span> on enemies near your target, <span class=\"highlight\">applying any bullet procs</span> and <span class=\"highlight\">dealing a percentage of the original damage.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_ricochet",
  "Name": "Ricochet",
  "PropertyUpgrades": {
    "BonusFireRate": 25,
    "RicochetDamagePercent": 15
  },
  "RicochetDamagePercent": 65,
  "RicochetRadius": "13m",
  "RicochetTargetsTooltipOnly": 2,
  "ShopFilters": [
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "BossEnemy",
    "TrooperEnemy",
    "PropEnemy",
    "MinionEnemy",
    "Neutral"
  ],
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
      "lookup": "ricochet",
      "name": "Ricochet",
      "type": "item"
    }
  ]
}
````
