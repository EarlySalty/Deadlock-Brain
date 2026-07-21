---
title: "Eternal Gift"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_eternal_gift"
canonical_name: "Eternal Gift"
snapshot_id: 40339
source_document_id: 7073
payload_hash: "110ee854c75e16138a7d971ac02fdf7f985237fb2c6b18af2d2a63751618e854"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.422820+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Eternal Gift

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_eternal_gift`
- Snapshot ID: `40339`
- Source-Dokument: `7073`
- Kurzinfo: Eternal Gift aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "Periodically gain a random permanent stat buff.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_eternal_gift_desc",
    "Main": [
      {
        "Key": "BuffFrequency",
        "Type": "duration",
        "Value": 2
      }
    ],
    "Type": "Passive"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 165,
    "DescKey": "#upgrade_eternal_gift_respawn_desc",
    "Main": [
      {
        "Key": "RespawnTime",
        "UsageFlags": "ConditionallyApplied",
        "Value": -70
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_eternal_gift",
  "Name": "Eternal Gift",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
  "Upgrades": {
    "BuffFrequency": -0.5
  },
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
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "eternal gift",
      "name": "Eternal Gift",
      "type": "item"
    }
  ]
}
````
