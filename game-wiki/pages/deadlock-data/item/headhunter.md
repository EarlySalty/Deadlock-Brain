---
title: "Headhunter"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_headhunter"
canonical_name: "Headhunter"
snapshot_id: 40080
source_document_id: 7072
payload_hash: "2387da7b38b96a620d782c1bb3d0b5bdd161234885acfa0b1c9082432ec8fb25"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.824259+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Headhunter

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_headhunter`
- Snapshot ID: `40080`
- Source-Dokument: `7072`
- Kurzinfo: Headhunter aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 8,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 5,
  "BonusHealth": 50,
  "BonusMoveSpeed": "1.75m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_headshot_booster"
  ],
  "Cost": 3200,
  "Description": "Your next <span class=\"highlight\">headshot</span> against an enemy Hero deals {g:citadel_inline_attribute:'BonusWeaponDamage'}, {g:citadel_inline_attribute:'Heal'} you, and briefly grants {g:citadel_inline_attribute:'BonusMoveSpeed'}.",
  "HeadShotBonusDamage": {
    "Scale": {
      "Type": "power_increase",
      "Value": 4
    },
    "Value": 75
  },
  "HealPercentPerHeadshot": {
    "Scale": {
      "Type": "power_increase",
      "Value": 0.06
    },
    "Value": 4
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_headhunter",
  "MovementSpeedBonusDuration": 3,
  "Name": "Headhunter",
  "ProcChance": 100,
  "PropertyUpgrades": {
    "AbilityCooldown": -3,
    "HeadShotBonusDamage": 75,
    "HealPercentPerHeadshot": 4
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
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
      "lookup": "headhunter",
      "name": "Headhunter",
      "type": "item"
    }
  ]
}
````
