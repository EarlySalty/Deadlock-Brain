---
title: "Spirit Burn"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_spirit_burn"
canonical_name: "Spirit Burn"
snapshot_id: 40463
source_document_id: 7073
payload_hash: "95a01475532e31ba1252f908e178bf24a4f8235d692f4257183dbe3c9feaa784"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.636033+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Spirit Burn

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_spirit_burn`
- Snapshot ID: `40463`
- Source-Dokument: `7073`
- Kurzinfo: Spirit Burn aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Dealing significant {g:citadel_inline_attribute:'SpiritDamage'} to an enemy within 5s causes an explosion dealing damage and a burn to nearby enemies. While burning, enemies take damage over time and receive reduced healing.<br><span class=\"diminish\">Deals half-damage and has half-cooldown on non-heroes.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 6
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
        "Key": "ExplosionRadius",
        "Value": "12m"
      },
      {
        "Key": "DebuffDuration",
        "Type": "tech_damage",
        "Value": 8
      },
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "UsageFlags": "ConditionallyApplied",
        "Value": -70
      }
    ],
    "ChargeUp": null,
    "Cooldown": 20,
    "DescKey": "#upgrade_spirit_burn_desc",
    "Main": [
      {
        "Key": "DamageThreshold",
        "Type": "tech_armor_up",
        "Value": 500
      },
      {
        "Key": "ExplosionDamage",
        "Type": "tech_damage",
        "Value": 110
      },
      {
        "Key": "DPS",
        "Scale": {
          "Type": "spirit",
          "Value": 0.06
        },
        "Type": "tech_damage",
        "Value": 24.0
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spirit_burn",
  "Name": "Spirit Burn",
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
    "CooldownReductionPctOnNonHeroes": {
      "Key": "CooldownReductionPctOnNonHeroes",
      "Value": 50
    },
    "DamagePctVsNonHeroes": {
      "Key": "DamagePctVsNonHeroes",
      "Value": 50
    },
    "DamageThresholdDuration": {
      "Key": "DamageThresholdDuration",
      "Value": 5
    },
    "HealAmpRegenPenaltyPercent": {
      "Key": "HealAmpRegenPenaltyPercent",
      "UsageFlags": "ConditionallyApplied",
      "Value": -70
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 6
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 0.5
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
    "AbilityCooldown": -6,
    "DPS": 20,
    "ExplosionDamage": 160,
    "TechRadiusMultiplier": 12,
    "TechRangeMultiplier": 12
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
      "lookup": "spirit burn",
      "name": "Spirit Burn",
      "type": "item"
    }
  ]
}
````
