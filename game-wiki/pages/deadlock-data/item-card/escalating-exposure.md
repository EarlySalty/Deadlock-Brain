---
title: "Escalating Exposure"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_escalating_exposure"
canonical_name: "Escalating Exposure"
snapshot_id: 40338
source_document_id: 7073
payload_hash: "49a38ac1d29181ff1e906348e8e7e38ea1f80014b1edba53a9ffc461af76f603"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.420726+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Escalating Exposure

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_escalating_exposure`
- Snapshot ID: `40338`
- Source-Dokument: `7073`
- Kurzinfo: Escalating Exposure aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_magic_vulnerability"
  ],
  "Cost": 6400,
  "Description": "Dealing {g:citadel_inline_attribute:'SpiritDamage'} applies a stacking <span class=\"highlight\">Spirit Amp</span> that increases your {g:citadel_inline_attribute:'SpiritDamage'} to the target.",
  "Info1": {
    "Alt": [
      {
        "Key": "TechArmorDamageReduction",
        "LocTokenOverride": "EscalatingExposureTechArmorDamageReduction",
        "Type": "tech_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -8
      },
      {
        "Key": "TechResist",
        "Value": 17
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "MaxStacks",
        "Value": 12
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": 0.7,
    "DescKey": "#upgrade_escalating_exposure_desc",
    "Main": [
      {
        "Key": "MagicIncreasePerStack",
        "Type": "tech_armor_down",
        "Value": 4.5
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_escalating_exposure",
  "Name": "Escalating Exposure",
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
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "MagicIncreasePerStack": 1.5,
    "MaxStacks": 6,
    "TechArmorDamageReduction": -10,
    "TechResist": 8
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
      "lookup": "escalating exposure",
      "name": "Escalating Exposure",
      "type": "item"
    }
  ]
}
````
