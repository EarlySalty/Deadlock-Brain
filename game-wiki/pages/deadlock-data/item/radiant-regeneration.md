---
title: "Radiant Regeneration"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_resonant_healing"
canonical_name: "Radiant Regeneration"
snapshot_id: 40159
source_document_id: 7072
payload_hash: "583c4cb3df12106df55f91286bde34d5b8ac9a314e4ec67aa816d85387ec2f08"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.037816+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Radiant Regeneration

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_resonant_healing`
- Snapshot ID: `40159`
- Source-Dokument: `7072`
- Kurzinfo: Radiant Regeneration aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 6,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 3,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 90,
  "BonusMoveSpeed": "1.75m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_mystic_regeneration"
  ],
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Heal</span> and gain bonus <span class=\"highlight\">Movement Speed</span> for a short duration when you cast an ability.",
  "HealingPerCast": {
    "Scale": {
      "Type": "power_increase",
      "Value": 2.0
    },
    "Value": 70
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_resonant_healing",
  "Name": "Radiant Regeneration",
  "PropertyUpgrades": {
    "BonusHealth": 110,
    "BonusMoveSpeed": "1m",
    "HealingPerCast": 60,
    "Regeneration": 9
  },
  "Regeneration": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.04
    },
    "Value": 4
  },
  "RegenerationDuration": 7,
  "ShopFilters": [
    "MagicDamage",
    "Movement",
    "Healing"
  ],
  "Slot": "Tech",
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
      "lookup": "radiant regeneration",
      "name": "Radiant Regeneration",
      "type": "item"
    }
  ]
}
````
