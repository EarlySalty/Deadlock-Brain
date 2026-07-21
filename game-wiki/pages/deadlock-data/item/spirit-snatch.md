---
title: "Spirit Snatch"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_spirit_snatch"
canonical_name: "Spirit Snatch"
snapshot_id: 40186
source_document_id: 7072
payload_hash: "7fd5fd88024c83f73b5237bcf2395ca32d0f79640616d3154841ef8a12f85655"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.115270+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Spirit Snatch

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_spirit_snatch`
- Snapshot ID: `40186`
- Source-Dokument: `7072`
- Kurzinfo: Spirit Snatch aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 6,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 10,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 75,
  "BonusMeleeDamagePercent": 7,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_acolytes_glove"
  ],
  "Cost": 3200,
  "Description": "When you perform a <span class=\"highlight\">Light or Heavy Melee</span> attack against a hero, the attack deals extra {g:citadel_inline_attribute:'SpiritDamage'} and steals <span class=\"highlight\">Spirit Resist</span> and <span class=\"highlight\">Spirit Power</span>.<span class=\"diminish\"><br><br>Effects are reduced by 30% for Light Melee hits.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spirit_snatch",
  "LightMeleeReduction": 30,
  "Name": "Spirit Snatch",
  "PropertyUpgrades": {
    "SpiritDamage": 50,
    "TechArmorDamageReduction": -5,
    "TechArmorGain": 5,
    "TechPowerGain": 35,
    "TechPowerReduction": -35
  },
  "ShopFilters": [
    "MagicDamage",
    "Melee"
  ],
  "Slot": "Tech",
  "SpiritDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.84
    },
    "Value": 50
  },
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "TechArmorDamageReduction": -12,
  "TechArmorGain": 12,
  "TechPowerGain": 25,
  "TechPowerReduction": -25,
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
      "lookup": "spirit snatch",
      "name": "Spirit Snatch",
      "type": "item"
    }
  ]
}
````
