---
title: "Cultist Sacrifice"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_non_player_bonus_sacrifice"
canonical_name: "Cultist Sacrifice"
snapshot_id: 40134
source_document_id: 7072
payload_hash: "b88605b42a95e4156412dac823409e08aa37ada95d334135016a2036eefd232b"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.967416+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Cultist Sacrifice

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_non_player_bonus_sacrifice`
- Snapshot ID: `40134`
- Source-Dokument: `7072`
- Kurzinfo: Cultist Sacrifice aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": "7m",
  "AbilityCooldown": 270,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 160,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BaseAttackDamagePercent": {
    "Scale": {
      "Type": "power_increase",
      "Value": 0.8
    },
    "Value": 10
  },
  "BonusAbilityCharges": 1,
  "BonusHealth": {
    "Scale": {
      "Type": "power_increase",
      "Value": 4
    },
    "Value": 50
  },
  "BonusSoulsPct": 180,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_non_player_bonus"
  ],
  "Cost": 3200,
  "Description": "Target an enemy NPC and consume it for <span class=\"highlight\">180% Bonus Souls</span> and grants a powerful long lasting buff.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_non_player_bonus_sacrifice",
  "Name": "Cultist Sacrifice",
  "NonPlayerBonusWeaponPower": 30,
  "NonPlayerBulletResist": 30,
  "OutOfCombatHealthRegen": 2,
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 47,
    "BonusHealth": 300,
    "NonPlayerBonusWeaponPower": 30,
    "NonPlayerBulletResist": 30,
    "TechRadiusMultiplier": 40,
    "TechRangeMultiplier": 40
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability",
    "Healing"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "TrooperEnemy",
    "Neutral",
    "MinionEnemy"
  ],
  "TechRadiusMultiplier": 12,
  "TechRangeMultiplier": 12,
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
      "lookup": "cultist sacrifice",
      "name": "Cultist Sacrifice",
      "type": "item"
    }
  ]
}
````
