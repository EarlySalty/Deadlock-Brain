---
title: "Phantom Strike"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_phantom_strike"
canonical_name: "Phantom Strike"
snapshot_id: 40139
source_document_id: 7072
payload_hash: "260b6a31b5869231c363675141b51cc886988617d999b705e830d910761402ce"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.985311+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Phantom Strike

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_phantom_strike`
- Snapshot ID: `40139`
- Source-Dokument: `7072`
- Kurzinfo: Phantom Strike aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.35,
  "AbilityCastRange": "25m",
  "AbilityCooldown": 35.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BaseAttackDamagePercent": 15,
  "ChannelMoveSpeed": "1.3m",
  "Components": null,
  "Cost": 6400,
  "Description": "<span class=\"highlight\">Teleport</span> to an enemy target and pull them to the ground. Dealing <span class=\"highlight\">damage</span>, <span class=\"highlight\">Move speed</span> reduction and <span class=\"highlight\">Disarm</span>.",
  "ImpactDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.93
    },
    "Value": 75
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_phantom_strike",
  "Name": "Phantom Strike",
  "PropertyUpgrades": {
    "AbilityCooldown": -20,
    "BaseAttackDamagePercent": 20,
    "ImpactDamage": 100,
    "TechPower": 12
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability",
    "Movement",
    "Disruption"
  ],
  "Slot": "Armor",
  "SlowDuration": 3,
  "SlowPercent": 50,
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "TechPower": 8,
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
      "lookup": "phantom strike",
      "name": "Phantom Strike",
      "type": "item"
    }
  ]
}
````
