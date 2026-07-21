---
title: "Cursed Relic"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_glitch"
canonical_name: "Cursed Relic"
snapshot_id: 40074
source_document_id: 7072
payload_hash: "39636a95b836a2f613b5e13493af53b88441f228f432678614b7135311ab82eb"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.809490+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Cursed Relic

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_glitch`
- Snapshot ID: `40074`
- Source-Dokument: `7072`
- Kurzinfo: Cursed Relic aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": "20m",
  "AbilityCooldown": 55.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 3.25,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "Description": "Curses an enemy - <span class=\"highlight\">interrupting, Silencing, Disarming</span>, and <span class=\"highlight\">preventing item usage</span>. <span class=\"highlight\">Removes all non-ultimate buffs</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_glitch",
  "Name": "Cursed Relic",
  "OutgoingDamagePenaltyPercent": -14,
  "PropertyUpgrades": {
    "AbilityCooldown": -40,
    "AbilityDuration": 0.25
  },
  "ShopFilters": [
    "Disruption",
    "WeaponDamage"
  ],
  "SkipFrames": 6,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
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
      "lookup": "cursed relic",
      "name": "Cursed Relic",
      "type": "item"
    }
  ]
}
````
