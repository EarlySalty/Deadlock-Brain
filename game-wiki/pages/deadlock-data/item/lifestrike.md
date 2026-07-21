---
title: "Lifestrike"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_boxing_glove"
canonical_name: "Lifestrike"
snapshot_id: 40008
source_document_id: 7072
payload_hash: "e7b8a1b088cc527ebafe8a8eb04d02dd2afa993cf4aa3a727db211b34ca506bd"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.653122+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Lifestrike

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_boxing_glove`
- Snapshot ID: `40008`
- Source-Dokument: `7072`
- Kurzinfo: Lifestrike aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 4,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 125,
  "BonusMeleeDamagePercent": 16,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_lifestrike_gauntlets"
  ],
  "Cost": 3200,
  "Description": "Your <span class=\"highlight\">Melee Attack</span> applies <span class=\"highlight\">Movement Slow</span> and <span class=\"highlight\">heals you</span> for a percentage of the <span class=\"highlight\">Melee Damage</span> dealt plus a fixed amount. <span class=\"diminish\"><br><br>This heal is 40% effective vs non-heroes. <br>Cooldown is 1.5x as long for Light Melee hits.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_boxing_glove",
  "LifestealHeal": {
    "Scale": {
      "Type": "power_increase",
      "Value": 1.5
    },
    "Value": 100
  },
  "LifestealHealPercent": {
    "Scale": {
      "Type": "power_increase",
      "Value": 0.5
    },
    "Value": 30
  },
  "LightMeleeCooldownMult": 1.5,
  "Name": "Lifestrike",
  "NonHeroHealPct": 40,
  "PropertyUpgrades": {
    "AbilityCooldown": -3,
    "BonusHealth": 125,
    "BonusMeleeDamagePercent": 10
  },
  "ShopFilters": [
    "Durability",
    "Melee",
    "Healing"
  ],
  "Slot": "Armor",
  "SlowDuration": 2.5,
  "SlowPercent": 60,
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
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
      "lookup": "lifestrike",
      "name": "Lifestrike",
      "type": "item"
    }
  ]
}
````
