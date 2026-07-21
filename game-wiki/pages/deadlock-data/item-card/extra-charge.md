---
title: "Extra Charge"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_extra_charge"
canonical_name: "Extra Charge"
snapshot_id: 40342
source_document_id: 7073
payload_hash: "31498c63dd606f080795f6dbab7099266357e24cd8dc32483dd2500cd6e5969c"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.428241+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Extra Charge

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_extra_charge`
- Snapshot ID: `40342`
- Source-Dokument: `7073`
- Kurzinfo: Extra Charge aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": "Adds one to your ability max <span class=\"highlight\">charges</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusSpiritForChargedAbilities",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_extra_charge_desc",
    "Main": [
      {
        "Key": "BonusAbilityCharges",
        "Type": "cast",
        "Value": 1
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_extra_charge",
  "Name": "Extra Charge",
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
  "ShopFilters": [
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "BonusAbilityCharges": 1,
    "BonusSpiritForChargedAbilities": 7
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
      "lookup": "extra charge",
      "name": "Extra Charge",
      "type": "item"
    }
  ]
}
````
