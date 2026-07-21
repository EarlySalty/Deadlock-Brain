---
title: "Disarming Hex"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_greater_withering_whip"
canonical_name: "Disarming Hex"
snapshot_id: 40076
source_document_id: 7072
payload_hash: "f544578f0950bfd7e96f3b25557362cc1e4b6617bba97ab173edada3a85f8183"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.814991+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Disarming Hex

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_greater_withering_whip`
- Snapshot ID: `40076`
- Source-Dokument: `7072`
- Kurzinfo: Disarming Hex aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": "32m",
  "AbilityCooldown": 16.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4.25,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusHealth": 75,
  "BonusSprintSpeed": "0.75m",
  "BulletArmorReduction": -13,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_withering_whip"
  ],
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Disarms</span> enemy target and reduces their <span class=\"highlight\">Bullet Resist</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_greater_withering_whip",
  "Name": "Disarming Hex",
  "PropertyUpgrades": {
    "AbilityCooldown": -8,
    "BonusHealth": 175,
    "BulletArmorReduction": -7
  },
  "ShopFilters": [
    "Disruption",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
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
      "lookup": "disarming hex",
      "name": "Disarming Hex",
      "type": "item"
    }
  ]
}
````
