---
title: "Stalker"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_weapon_backstabber"
canonical_name: "Stalker"
snapshot_id: 40219
source_document_id: 7072
payload_hash: "a5eaf0029f086141601716ecdc2f2ede4ec5b7122c4d0e3adee233030eb847c1"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.204699+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Stalker

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_weapon_backstabber`
- Snapshot ID: `40219`
- Source-Dokument: `7072`
- Kurzinfo: Stalker aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 6,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 50,
  "BonusMoveSpeed": "1.5m",
  "BulletResistReduction": -6,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "DPS": 17,
  "DebuffDuration": 5,
  "DebuffRadius": "25m",
  "Description": "Dealing {g:citadel_inline_attribute:'WeaponDamage'} at close range opens a wound and grants you {g:citadel_inline_attribute:'BonusMoveSpeed'}. <br><br>Wounded enemies take {g:citadel_inline_attribute:'SpiritDPS'}, have reduced {g:citadel_inline_attribute:'BulletResist'}, and are revealed <span class=\"highlight\">through walls</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_weapon_backstabber",
  "Name": "Stalker",
  "ProcRadius": "8m",
  "PropertyUpgrades": {
    "BonusMoveSpeed": "2m",
    "BulletResistReduction": -10,
    "DPS": 20,
    "ReduceFootstepSound": -50
  },
  "ReduceFootstepSound": -50,
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TickRate": 0.5,
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
      "lookup": "stalker",
      "name": "Stalker",
      "type": "item"
    }
  ]
}
````
