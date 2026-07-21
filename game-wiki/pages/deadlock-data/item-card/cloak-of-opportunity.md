---
title: "Cloak of Opportunity"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_cloak_of_opportunity"
canonical_name: "Cloak of Opportunity"
snapshot_id: 40309
source_document_id: 7073
payload_hash: "c6a5b20eaf95ae8a3e3f1c04cb3675a41339b6fd3fb195831c828d9c83937b68"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.364128+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Cloak of Opportunity

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_cloak_of_opportunity`
- Snapshot ID: `40309`
- Source-Dokument: `7073`
- Kurzinfo: Cloak of Opportunity aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "Block the next debuff that would apply <span class=\"highlight\">movement lock, Stun, Chained, Immobilize, or Sleep</span> and become <span class=\"highlight\">Unstoppable</span>. Also gain a <span class=\"highlight\">Barrier</span> and bonus {g:citadel_inline_attribute:'MoveSpeed'}",
  "Info1": {
    "Alt": [
      {
        "Key": "StatusImmuneDuration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "BuffDuration",
        "Type": "duration",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": 12.0,
    "DescKey": "#upgrade_cloak_of_opportunity_desc",
    "Main": [
      {
        "Key": "CombatBarrier",
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 500
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "3m"
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_cloak_of_opportunity",
  "Name": "Cloak of Opportunity",
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
  "TargetTypes": [
    "AllFriendly"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityCooldown": -4,
    "BonusMoveSpeed": "4m",
    "CombatBarrier": 300
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
      "lookup": "cloak of opportunity",
      "name": "Cloak of Opportunity",
      "type": "item"
    }
  ]
}
````
