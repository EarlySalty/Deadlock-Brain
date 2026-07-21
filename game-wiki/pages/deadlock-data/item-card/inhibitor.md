---
title: "Inhibitor"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_inhibitor"
canonical_name: "Inhibitor"
snapshot_id: 40385
source_document_id: 7073
payload_hash: "4c0a81eb1168b55572e6c5bfa90ffd5796a12de8a2867fa207844f9f2ecdd1d3"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.502982+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Inhibitor

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_inhibitor`
- Snapshot ID: `40385`
- Source-Dokument: `7073`
- Kurzinfo: Inhibitor aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Your bullets build up to reduce the target's <span class=\"highlight\">outgoing damage</span> and apply <span class=\"highlight\">healing reduction</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 10
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 150
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
        "Key": "DebuffDuration",
        "Type": "duration",
        "Value": 5
      },
      {
        "Key": "BuildUpPerShot",
        "Value": 0.77
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_inhibitor_desc",
    "Main": [
      {
        "Key": "OutgoingDamagePenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -30
      },
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -40
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_inhibitor",
  "Name": "Inhibitor",
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
    "BuildUpDuration": {
      "Key": "BuildUpDuration",
      "Type": "duration",
      "Value": 5
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HealAmpRegenPenaltyPercent": {
      "Key": "HealAmpRegenPenaltyPercent",
      "Type": "damage",
      "UsageFlags": "ConditionallyApplied",
      "Value": -40
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Disruption",
    "FireRate"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "BaseAttackDamagePercent": 20,
    "BonusHealth": 125,
    "HealAmpReceivePenaltyPercent": -20,
    "HealAmpRegenPenaltyPercent": -20,
    "OutgoingDamagePenaltyPercent": -20
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
      "lookup": "inhibitor",
      "name": "Inhibitor",
      "type": "item"
    }
  ]
}
````
