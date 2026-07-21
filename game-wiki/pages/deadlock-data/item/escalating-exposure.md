---
title: "Escalating Exposure"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_escalating_exposure"
canonical_name: "Escalating Exposure"
snapshot_id: 40059
source_document_id: 7072
payload_hash: "e9a51317287477cfeda2f19b641c3ab38f30b89d6c2dc170b3b3b801d4542b4a"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.774617+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Escalating Exposure

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_escalating_exposure`
- Snapshot ID: `40059`
- Source-Dokument: `7072`
- Kurzinfo: Escalating Exposure aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 12,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_magic_vulnerability"
  ],
  "Cost": 6400,
  "Description": "Dealing {g:citadel_inline_attribute:'SpiritDamage'} applies a stacking <span class=\"highlight\">Spirit Amp</span> that increases your {g:citadel_inline_attribute:'SpiritDamage'} to the target.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_escalating_exposure",
  "MagicIncreasePerStack": 4.5,
  "MaxStacks": 12,
  "Name": "Escalating Exposure",
  "ProcCooldown": 0.7,
  "PropertyUpgrades": {
    "MagicIncreasePerStack": 1.5,
    "MaxStacks": 6,
    "TechArmorDamageReduction": -10,
    "TechResist": 8
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
  "TechArmorDamageReduction": -8,
  "TechResist": 17,
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
      "lookup": "escalating exposure",
      "name": "Escalating Exposure",
      "type": "item"
    }
  ]
}
````
