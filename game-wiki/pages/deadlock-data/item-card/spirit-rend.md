---
title: "Spirit Rend"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_spellslinger_headshots"
canonical_name: "Spirit Rend"
snapshot_id: 40461
source_document_id: 7073
payload_hash: "bb22095afbcff6a65fc1f27334684c729e070aabc26309e575f9caa4a6b6183b"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.632420+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Spirit Rend

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_spellslinger_headshots`
- Snapshot ID: `40461`
- Source-Dokument: `7073`
- Kurzinfo: Spirit Rend aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_tech_defense_shredders"
  ],
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 75
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_spellslinger_headshots_part1_desc",
    "Main": [
      {
        "Key": "TechArmorDamageReduction",
        "Type": "tech_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -8
      },
      {
        "Key": "AbilityLifestealPercentHero",
        "Type": "healing",
        "UsageFlags": "ConditionallyApplied",
        "Value": 10
      }
    ],
    "Type": null
  },
  "Info3": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Type": "duration",
        "Value": 8
      },
      {
        "Key": "MaxStacks",
        "Value": 4
      }
    ],
    "ChargeUp": null,
    "Cooldown": 2,
    "DescKey": "#upgrade_spellslinger_headshots_part2_desc",
    "Main": [
      {
        "Key": "MagicResistReduction",
        "LocTokenOverride": "SpellSlingerHeadshots_SpiritShredPerStack",
        "Type": "tech_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -7
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spellslinger_headshots",
  "Name": "Spirit Rend",
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
    "WeaponDamage",
    "MagicDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "AbilityLifestealPercentHero": 10,
    "MagicResistReduction": -5,
    "TechArmorDamageReduction": -10
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
      "lookup": "spirit rend",
      "name": "Spirit Rend",
      "type": "item"
    }
  ]
}
````
