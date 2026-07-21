---
title: "Slowing Hex"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_containment"
canonical_name: "Slowing Hex"
snapshot_id: 40037
source_document_id: 7072
payload_hash: "d3e403174ad51aa1c0fc8c50aba52080c3fa9b23a2097c42430545a7f546f66e"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.720397+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Slowing Hex

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_containment`
- Snapshot ID: `40037`
- Source-Dokument: `7072`
- Kurzinfo: Slowing Hex aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": "25m",
  "AbilityCooldown": 27,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 3.5,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusSprintSpeed": "0.5m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "<span class=\"highlight\">Slows movement</span> of enemy target. Also <span class=\"highlight\">Silences</span> their <span class=\"highlight\">movement-based items and abilities</span>.<br><span class=\"diminish\">Increases the target's gravity.<br>Does not affect target's stamina usage.</span>",
  "GroundDashReductionPercent": -30,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_containment",
  "Name": "Slowing Hex",
  "PropertyUpgrades": {
    "AbilityCooldown": -18,
    "GroundDashReductionPercent": -6,
    "SlowPercent": 10
  },
  "ShopFilters": [
    "Movement",
    "Disruption",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "SlowPercent": 20,
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
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
      "lookup": "slowing hex",
      "name": "Slowing Hex",
      "type": "item"
    }
  ]
}
````
