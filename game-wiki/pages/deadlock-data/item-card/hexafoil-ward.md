---
title: "Hexafoil Ward"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_spellshield"
canonical_name: "Hexafoil Ward"
snapshot_id: 40460
source_document_id: 7073
payload_hash: "981c24c6532ddcdef2d1ff2e37b35dff44a1a2794151be2a9e6343db245068a7"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.630898+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Hexafoil Ward

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_spellshield`
- Snapshot ID: `40460`
- Source-Dokument: `7073`
- Kurzinfo: Hexafoil Ward aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "Block the next {g:citadel_inline_attribute:'SpiritDamage'} or <span class=\"highlight\">Debuff</span>, preventing its effects. Only regenerates outside of combat.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "SpellShieldFlavorText",
        "Type": "time",
        "Value": "asdasd"
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 12,
    "DescKey": "#upgrade_spellshield_desc",
    "Main": [
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "Value": 15
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_spellshield",
  "Name": "Hexafoil Ward",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": "1.3m"
    },
    "SpellShieldLingerDuration": {
      "Key": "SpellShieldLingerDuration",
      "Value": 0.3
    }
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
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
  }
}
````
