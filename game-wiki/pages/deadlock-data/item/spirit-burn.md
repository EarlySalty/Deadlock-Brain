---
title: "Spirit Burn"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_spirit_burn"
canonical_name: "Spirit Burn"
snapshot_id: 40184
source_document_id: 7072
payload_hash: "ba24442524a3b3acc7e78418a08023479c56c89cec039c1f05ff390a14162b3d"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.109801+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Spirit Burn

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_spirit_burn`
- Snapshot ID: `40184`
- Source-Dokument: `7072`
- Kurzinfo: Spirit Burn aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 20,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "CooldownReductionPctOnNonHeroes": 50,
  "Cost": 6400,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.06
    },
    "Value": 24
  },
  "DamagePctVsNonHeroes": 50,
  "DamageThreshold": 500,
  "DamageThresholdDuration": 5,
  "DebuffDuration": 8,
  "Description": "Dealing significant {g:citadel_inline_attribute:'SpiritDamage'} to an enemy within 5s causes an explosion dealing damage and a burn to nearby enemies. While burning, enemies take damage over time and receive reduced healing.<br><span class=\"diminish\">Deals half-damage and has half-cooldown on non-heroes.</span>",
  "ExplosionDamage": 110,
  "ExplosionRadius": "12m",
  "HealAmpReceivePenaltyPercent": -70,
  "HealAmpRegenPenaltyPercent": -70,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spirit_burn",
  "Name": "Spirit Burn",
  "PropertyUpgrades": {
    "AbilityCooldown": -6,
    "DPS": 20,
    "ExplosionDamage": 160,
    "TechRadiusMultiplier": 12,
    "TechRangeMultiplier": 12
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechRadiusMultiplier": 6,
  "TechRangeMultiplier": 6,
  "TickRate": 0.5,
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
      "lookup": "spirit burn",
      "name": "Spirit Burn",
      "type": "item"
    }
  ]
}
````
