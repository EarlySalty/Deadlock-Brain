---
title: "Spirit Snatch"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_spirit_snatch"
canonical_name: "Spirit Snatch"
snapshot_id: 40465
source_document_id: 7073
payload_hash: "c9a2b9c3d9067b7b678848128ffc0d2c30d8c630e3e43a644eb852989bd404bd"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.650283+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Spirit Snatch

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_spirit_snatch`
- Snapshot ID: `40465`
- Source-Dokument: `7073`
- Kurzinfo: Spirit Snatch aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_acolytes_glove"
  ],
  "Cost": 3200,
  "Description": "When you perform a <span class=\"highlight\">Light or Heavy Melee</span> attack against a hero, the attack deals extra {g:citadel_inline_attribute:'SpiritDamage'} and steals <span class=\"highlight\">Spirit Resist</span> and <span class=\"highlight\">Spirit Power</span>.<span class=\"diminish\"><br><br>Effects are reduced by 30% for Light Melee hits.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusMeleeDamagePercent",
        "Type": "melee_damage",
        "Value": 7
      },
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
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": 6,
    "DescKey": "#upgrade_spirit_snatch_desc",
    "Main": [
      {
        "Key": "SpiritDamage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.84
        },
        "Type": "tech_damage",
        "Value": 50.0
      },
      {
        "Key": "TechArmorGain",
        "LocTokenOverride": "SpiritSnatch_TechArmorSteal",
        "Type": "tech_armor_up",
        "UsageFlags": "ConditionallyApplied",
        "Value": 12
      },
      {
        "Key": "TechPowerGain",
        "LocTokenOverride": "SpiritSnatch_TechPowerSteal",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 25
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spirit_snatch",
  "Name": "Spirit Snatch",
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
    "LightMeleeReduction": {
      "Key": "LightMeleeReduction",
      "Value": 30
    },
    "TechArmorDamageReduction": {
      "Key": "TechArmorDamageReduction",
      "Type": "tech_armor_down",
      "UsageFlags": "ConditionallyApplied",
      "Value": -12
    },
    "TechPowerReduction": {
      "Key": "TechPowerReduction",
      "Type": "tech_damage",
      "UsageFlags": "ConditionallyApplied",
      "Value": -25
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Melee"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "SpiritDamage": 50,
    "TechArmorDamageReduction": -5,
    "TechArmorGain": 5,
    "TechPowerGain": 35,
    "TechPowerReduction": -35
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
      "lookup": "spirit snatch",
      "name": "Spirit Snatch",
      "type": "item"
    }
  ]
}
````
