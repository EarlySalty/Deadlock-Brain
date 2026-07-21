---
title: "Inhibitor"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_inhibitor"
canonical_name: "Inhibitor"
snapshot_id: 40106
source_document_id: 7072
payload_hash: "308e90e199c2ec6d0a14eb89cb147c136da8cf81f25ad8645fe13b47548b64ec"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.886833+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Inhibitor

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_inhibitor`
- Snapshot ID: `40106`
- Source-Dokument: `7072`
- Kurzinfo: Inhibitor aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 10,
  "BonusHealth": 150,
  "BuildUpDuration": 5,
  "BuildUpPerShot": 0.77,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "DebuffDuration": 5,
  "Description": "Your bullets build up to reduce the target's <span class=\"highlight\">outgoing damage</span> and apply <span class=\"highlight\">healing reduction</span>.",
  "HealAmpReceivePenaltyPercent": -40,
  "HealAmpRegenPenaltyPercent": -40,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_inhibitor",
  "Name": "Inhibitor",
  "OutgoingDamagePenaltyPercent": -30,
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 20,
    "BonusHealth": 125,
    "HealAmpReceivePenaltyPercent": -20,
    "HealAmpRegenPenaltyPercent": -20,
    "OutgoingDamagePenaltyPercent": -20
  },
  "ShopFilters": [
    "WeaponDamage",
    "Disruption",
    "FireRate"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
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
      "lookup": "inhibitor",
      "name": "Inhibitor",
      "type": "item"
    }
  ]
}
````
