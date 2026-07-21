---
title: "Healing Tempo"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_healbuff"
canonical_name: "Healing Tempo"
snapshot_id: 40085
source_document_id: 7072
payload_hash: "8a538d6f4a7fd6bfda4a3b3a006244f002cc6e94669608aed99727275e3750bf"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.837227+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Healing Tempo

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_healbuff`
- Snapshot ID: `40085`
- Source-Dokument: `7072`
- Kurzinfo: Healing Tempo aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 1,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusFireRate": 35,
  "BonusHealthRegen": 6,
  "BonusMoveSpeed": "1.25m",
  "BuffDuration": 7,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_healing_booster"
  ],
  "Cost": 6400,
  "Description": "Applying {g:citadel_inline_attribute:'Heal'} to yourself or an ally grants the target {g:citadel_inline_attribute:'BonusFireRate'} and {g:citadel_inline_attribute:'BonusMoveSpeed'}.<br><br><span class=\"diminish\">Does not apply on innate Regen or passive Bullet/Spirit Lifesteals.</span>",
  "HealAmpCastPercent": 25,
  "HealAmpRegenPercent": 25,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_healbuff",
  "MinimumHealAmount": 1,
  "Name": "Healing Tempo",
  "OutOfCombatHealthRegen": 4,
  "PropertyUpgrades": {
    "BonusFireRate": 20,
    "BonusHealthRegen": 6,
    "BonusMoveSpeed": "2m",
    "HealAmpCastPercent": 10,
    "HealAmpRegenPercent": 10,
    "TechResist": 10
  },
  "ShopFilters": [
    "FireRate",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechResist": 10,
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
      "lookup": "healing tempo",
      "name": "Healing Tempo",
      "type": "item"
    }
  ]
}
````
