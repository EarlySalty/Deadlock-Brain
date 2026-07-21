---
title: "Crushing Fists"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_crushing_fists"
canonical_name: "Crushing Fists"
snapshot_id: 40043
source_document_id: 7072
payload_hash: "32f3f2b006d9e6c48c78ff7f1bd1c6ba96480f3b3dfb98b2871358d2c3af6aeb"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.736287+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Crushing Fists

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_crushing_fists`
- Snapshot ID: `40043`
- Source-Dokument: `7072`
- Kurzinfo: Crushing Fists aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHeavyMeleeDamage": 25,
  "BonusMeleeDamagePercent": 22,
  "BulletResist": 12,
  "BulletResistReduction": -4,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_melee_charge"
  ],
  "Cost": 6400,
  "DebuffDuration": 8,
  "Description": "Your <span class=\"highlight\">{g:citadel_inline_attribute:'MeleeDamage'}</span> will <span class=\"highlight\">restore ammo</span> and apply a <span class=\"highlight\">stacking bullet resist debuff</span> on enemies. Heavy melee applies 2 stacks. <br><br>If the target reaches max stacks, they will be <span class=\"highlight\">stunned</span>.",
  "HeavyMeleeMultiplier": 2,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_crushing_fists",
  "LightMeleeAmmo": 15,
  "LightMeleeStacks": 1,
  "MaxStacks": 6,
  "MeleeDistanceScale": 60,
  "Name": "Crushing Fists",
  "PropertyUpgrades": {
    "BonusHeavyMeleeDamage": 15,
    "BonusMeleeDamagePercent": 15,
    "BulletResist": 12,
    "BulletResistReduction": -4,
    "MeleeDistanceScale": 40
  },
  "ShopFilters": [
    "Melee"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "StunDuration": 0.5,
  "TargetTypes": [
    "HeroEnemy",
    "TrooperEnemy",
    "MinionEnemy",
    "Neutral"
  ],
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
      "lookup": "crushing fists",
      "name": "Crushing Fists",
      "type": "item"
    }
  ]
}
````
