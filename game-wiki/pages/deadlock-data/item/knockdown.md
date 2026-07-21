---
title: "Knockdown"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_target_stun"
canonical_name: "Knockdown"
snapshot_id: 40196
source_document_id: 7072
payload_hash: "c724b14ce85676fcd81b72671074e9e7ec36c3abcf1a164baa8a548a5c478d2d"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.142251+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Knockdown

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_target_stun`
- Snapshot ID: `40196`
- Source-Dokument: `7072`
- Kurzinfo: Knockdown aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": "45m",
  "AbilityCooldown": 35.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusHealth": 75,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "Apply a <span class=\"highlight\">Stun</span> after <span class=\"highlight\">2s</span>. Stun duration is increased against <span class=\"highlight\">airborne</span> targets.<br><br><span class=\"diminish\">Increases the target's gravity for the duration of the stun.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_target_stun",
  "MaxBonusDuration": 1.5,
  "MaxHeightForBonus": "30m",
  "Name": "Knockdown",
  "PropertyUpgrades": {
    "BonusHealth": 75,
    "StunDuration": 0.75,
    "TechRadiusMultiplier": 6,
    "TechRangeMultiplier": 6
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "StunDelay": 2,
  "StunDuration": 0.5,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "TechRadiusMultiplier": 5,
  "TechRangeMultiplier": 5,
  "Tier": 3,
  "VisualContractRadius": "3m",
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
      "lookup": "knockdown",
      "name": "Knockdown",
      "type": "item"
    }
  ]
}
````
