---
title: "Veil Walker"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_veil_walker"
canonical_name: "Veil Walker"
snapshot_id: 40216
source_document_id: 7072
payload_hash: "9188bbf33a0b11701cc5541971421d442658d2f8202aea695ba573461efb83c9"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.196162+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Veil Walker

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_veil_walker`
- Snapshot ID: `40216`
- Source-Dokument: `7072`
- Kurzinfo: Veil Walker aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 15.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 16,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 125,
  "BonusMoveSpeed": "3.5m",
  "BonusSprintSpeed": "2.0m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_sprint_booster"
  ],
  "Cost": 3200,
  "Description": "Walking through a <span class=\"highlight\">cosmic veil</span> grants you <span class=\"highlight\">Stealth</span>, <span class=\"highlight\">Heal</span> and increased <span class=\"highlight\">Move Speed</span>.",
  "HealOnVeil": {
    "Scale": {
      "Type": "power_increase",
      "Value": 8
    },
    "Value": 85
  },
  "InvisAlertWhenFading": 1,
  "InvisDuration": 8,
  "InvisFadeToDuration": 0.25,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_veil_walker",
  "Name": "Veil Walker",
  "OutOfCombatHealthRegen": 2,
  "PropertyUpgrades": {
    "AbilityCooldown": -9,
    "BonusMoveSpeed": "4m",
    "BonusSprintSpeed": "12m",
    "HealOnVeil": 300,
    "InvisDuration": 4,
    "InvisMoveSpeedMod": "6m",
    "OutOfCombatHealthRegen": 8,
    "SpiritPower": 25
  },
  "RevealOnDamageDuration": 0.5,
  "RevealOnSpottedDuration": 1.25,
  "ShopFilters": [
    "Durability",
    "ClipSize"
  ],
  "Slot": "Armor",
  "SpiritPower": 10,
  "SpottedRadius": "20m",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "veil walker",
      "name": "Veil Walker",
      "type": "item"
    }
  ]
}
````
