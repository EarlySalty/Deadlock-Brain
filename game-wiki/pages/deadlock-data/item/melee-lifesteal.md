---
title: "Melee Lifesteal"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_lifestrike_gauntlets"
canonical_name: "Melee Lifesteal"
snapshot_id: 40110
source_document_id: 7072
payload_hash: "c12636ba0815bbe2595551ccc4e5bf50717a70a313954997c95b61b9722b17a2"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.897575+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Melee Lifesteal

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_lifestrike_gauntlets`
- Snapshot ID: `40110`
- Source-Dokument: `7072`
- Kurzinfo: Melee Lifesteal aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 8,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusMeleeDamagePercent": 12,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": "Your next <span class=\"highlight\">Melee</span> attack <span class=\"highlight\">heals you</span>. <span class=\"diminish\"><br><br>This heal is 30% effective vs non-heroes. <br>Cooldown is 1.5x as long for Light Melee hits.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_lifestrike_gauntlets",
  "LifestrikeHeal": 100,
  "LightMeleeCooldownMult": 1.5,
  "Name": "Melee Lifesteal",
  "NonHeroHealPct": 30,
  "PropertyUpgrades": {
    "AbilityCooldown": -6,
    "BonusMeleeDamagePercent": 12
  },
  "ShopFilters": [
    "Durability",
    "Melee",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 1,
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
      "lookup": "melee lifesteal",
      "name": "Melee Lifesteal",
      "type": "item"
    }
  ]
}
````
