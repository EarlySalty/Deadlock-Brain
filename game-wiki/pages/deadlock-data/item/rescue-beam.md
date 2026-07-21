---
title: "Rescue Beam"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_rescue_beam"
canonical_name: "Rescue Beam"
snapshot_id: 40157
source_document_id: 7072
payload_hash: "c895637fd29291e4712f2cf20d4240689e4ab9cc0401658ae539a55e9142adc1"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.032052+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Rescue Beam

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_rescue_beam`
- Snapshot ID: `40157`
- Source-Dokument: `7072`
- Kurzinfo: Rescue Beam aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": "35m",
  "AbilityChannelTime": 2.5,
  "AbilityCooldown": 60.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusMoveSpeed": "0m",
  "BonusSprintSpeed": "0.75m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_health_stimpak"
  ],
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Heals</span> a target allied hero and yourself for a percentage of <span class=\"highlight\">Max Health</span>. Once while healing, you can <span class=\"highlight\">Pull</span> the target towards you. Can be self-cast.",
  "HealInterval": 0.2,
  "HealPercentAmount": 20,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rescue_beam",
  "Name": "Rescue Beam",
  "PropertyUpgrades": {
    "AbilityCooldown": -45,
    "HealPercentAmount": 15,
    "TechRadiusMultiplier": 20,
    "TechRangeMultiplier": 20
  },
  "SelfModifier": 100,
  "ShopFilters": [
    "MagicDamage",
    "Durability",
    "Healing",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "TechRadiusMultiplier": 6,
  "TechRangeMultiplier": 6,
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
      "lookup": "rescue beam",
      "name": "Rescue Beam",
      "type": "item"
    }
  ]
}
````
