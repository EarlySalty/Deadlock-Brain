---
title: "Arcane Surge"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_arcane_surge"
canonical_name: "Arcane Surge"
snapshot_id: 40274
source_document_id: 7073
payload_hash: "dc02a7ef1c33d0e1d4bb228c10b8f4447f3edc39b3d2e15055d0a982f2e196dc"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.305798+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Arcane Surge

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_arcane_surge`
- Snapshot ID: `40274`
- Source-Dokument: `7073`
- Kurzinfo: Arcane Surge aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_improved_stamina"
  ],
  "Cost": 1600,
  "Description": "After you <span class=\"highlight\">Dash-Jump</span>, the <span class=\"highlight\">next ability you use</span> within 7s will have bonus <span class=\"highlight\">Range, Duration,</span> and <span class=\"highlight\">Spirit Power</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "Stamina",
        "Value": 1
      },
      {
        "Key": "StaminaCooldownReduction",
        "Value": 12
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
        "Key": "AbilityDuration",
        "LocTokenOverride": "ArcaneSurgeWindow",
        "Type": "duration",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_arcane_surge_desc",
    "Main": [
      {
        "Key": "TechRangeMultiplierBuff",
        "Type": "distance",
        "UsageFlags": "ConditionallyApplied",
        "Value": 12
      },
      {
        "Key": "BonusAbilityDurationPercent",
        "Type": "duration",
        "UsageFlags": "ConditionallyApplied",
        "Value": 15
      },
      {
        "Key": "SpiritPower",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 20
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_arcane_surge",
  "Name": "Arcane Surge",
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
    },
    "TechRadiusMultiplierBuff": {
      "Key": "TechRadiusMultiplierBuff",
      "Type": "distance",
      "UsageFlags": "ConditionallyApplied",
      "Value": 12
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate",
    "ClipSize",
    "Movement",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusAbilityDurationPercent": 15,
    "SpiritPower": 25,
    "Stamina": 1,
    "StaminaCooldownReduction": 14,
    "TechRadiusMultiplierBuff": 15,
    "TechRangeMultiplierBuff": 15
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
      "lookup": "arcane surge",
      "name": "Arcane Surge",
      "type": "item"
    }
  ]
}
````
