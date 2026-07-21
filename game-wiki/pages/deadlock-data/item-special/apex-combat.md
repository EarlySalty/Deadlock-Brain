---
title: "Apex Combat"
entity_type: "item_special"
source: "deadlock_data"
external_id: "upgrade_apex_combat"
canonical_name: "Apex Combat"
snapshot_id: 39990
source_document_id: 7072
payload_hash: "1dbb04de0f0e645dd7783567dbc563e6e8a1384f9ed6ad3c1af79eb1e272b342"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.616297+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_special"]
---

# Apex Combat

## Kurzueberblick

- Typ: `item_special`
- Quelle: `deadlock_data`
- External ID: `upgrade_apex_combat`
- Snapshot ID: `39990`
- Source-Dokument: `7072`
- Kurzinfo: Apex Combat aus `deadlock_data` / `item_special` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_ricochet"
  ],
  "Cost": 9999,
  "CritDamagePercent": 125,
  "Description": "Your bullets will <span class=\"highlight\">ricochet</span> on enemies near your target, <span class=\"highlight\">applying any bullet procs</span> and <span class=\"highlight\">dealing a percentage of the original damage.</span>",
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_apex_combat",
  "Name": "Apex Combat",
  "ProcChance": 40,
  "PropertyUpgrades": {
    "CritDamagePercent": 30,
    "ProcChance": 5,
    "RicochetDamagePercent": 25
  },
  "RicochetDamagePercent": 65,
  "RicochetRadius": "13m",
  "RicochetTargetsTooltipOnly": 4,
  "ShopFilters": [
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": true,
  "TargetTypes": [
    "HeroEnemy",
    "BossEnemy",
    "TrooperEnemy",
    "PropEnemy",
    "MinionEnemy",
    "Neutral"
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
  }
}
````
