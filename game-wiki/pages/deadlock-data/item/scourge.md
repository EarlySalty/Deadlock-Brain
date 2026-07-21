---
title: "Scourge"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_discord"
canonical_name: "Scourge"
snapshot_id: 40048
source_document_id: 7072
payload_hash: "fcec22a2167b448b7ef153e5cd1298cc858a3dea98a5c2828af59e1a1d823c8d"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.747027+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Scourge

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_discord`
- Snapshot ID: `40048`
- Source-Dokument: `7072`
- Kurzinfo: Scourge aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCastRange": "35m",
  "AbilityCooldown": 35.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 10,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "AuraRadius": "10m",
  "BonusHealth": 100,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "Description": "Apply <span class=\"highlight\">Spirit Resist</span> and an aura on a friendly target that deals <span class=\"highlight\">{g:citadel_inline_attribute:'SpiritIcon'}damage</span> to enemies proportional to their max health. <br>Can be self cast.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_discord",
  "MaxHealthPercentAsDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0055
    },
    "Value": 2.6
  },
  "Name": "Scourge",
  "PropertyUpgrades": {
    "AbilityDuration": 3,
    "BonusHealth": 125,
    "CombatBarrier": 300,
    "MaxHealthPercentAsDPS": 2,
    "StatusResistancePercent": 20
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StatusResistancePercent": 17,
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "TechResist": 40,
  "TickRate": 0.25,
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
      "lookup": "scourge",
      "name": "Scourge",
      "type": "item"
    }
  ]
}
````
