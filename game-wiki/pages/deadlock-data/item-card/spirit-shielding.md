---
title: "Spirit Shielding"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_spirit_bubble"
canonical_name: "Spirit Shielding"
snapshot_id: 40462
source_document_id: 7073
payload_hash: "5aa245d24a7641e44405115fe050fbf312ad2ad7642c42f81008459e35bcc628"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.634401+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Spirit Shielding

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_spirit_bubble`
- Snapshot ID: `40462`
- Source-Dokument: `7073`
- Kurzinfo: Spirit Shielding aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_grit"
  ],
  "Cost": 1600,
  "Description": "Gain a <span class=\"highlight\">Barrier</span> whenever you take significant {g:citadel_inline_attribute:'SpiritDamage'} from enemy Heroes in a small time frame.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 2.5
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
        "Key": "DamageThreshold",
        "Type": "bullet_armor_up",
        "Value": 225
      },
      {
        "Key": "DamageWindow",
        "Type": "duration",
        "Value": 3.5
      },
      {
        "Key": "BarrierDuration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": 45,
    "DescKey": "#upgrade_spirit_bubble_desc",
    "Main": [
      {
        "Key": "CombatBarrier",
        "Scale": {
          "Type": "power_increase",
          "Value": 5.0
        },
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 300.0
      },
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "UsageFlags": "ConditionallyApplied",
        "Value": 18
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spirit_bubble",
  "Name": "Spirit Shielding",
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
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "AbilityCooldown": -20,
    "CombatBarrier": 175,
    "OutOfCombatHealthRegen": 3,
    "TechResist": 20
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
      "lookup": "spirit shielding",
      "name": "Spirit Shielding",
      "type": "item"
    }
  ]
}
````
