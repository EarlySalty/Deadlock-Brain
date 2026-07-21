---
title: "Weapon Shielding"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_weapon_shielding"
canonical_name: "Weapon Shielding"
snapshot_id: 40225
source_document_id: 7072
payload_hash: "17843652070c40db54f318228c836af4aa77f37b0ed03a4d09f39928820bc6ec"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.218061+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Weapon Shielding

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_weapon_shielding`
- Snapshot ID: `40225`
- Source-Dokument: `7072`
- Kurzinfo: Weapon Shielding aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 35,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BarrierDuration": 8,
  "BulletResist": 18,
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "power_increase",
      "Value": 5
    },
    "Value": 300
  },
  "Components": [
    "upgrade_grit"
  ],
  "Cost": 1600,
  "DamageThreshold": 250,
  "DamageWindow": 4.0,
  "Description": "Gain a <span class=\"highlight\">Barrier</span> whenever you take significant {g:citadel_inline_attribute:'WeaponDamage'} from enemy Heroes in a small time frame.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_weapon_shielding",
  "Name": "Weapon Shielding",
  "OutOfCombatHealthRegen": 2.5,
  "PropertyUpgrades": {
    "AbilityCooldown": -20,
    "BulletResist": 15,
    "CombatBarrier": 225,
    "OutOfCombatHealthRegen": 3
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "weapon shielding",
      "name": "Weapon Shielding",
      "type": "item"
    }
  ]
}
````
