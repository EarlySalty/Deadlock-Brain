---
title: "Toxic Bullets"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_toxic_bullets"
canonical_name: "Toxic Bullets"
snapshot_id: 40488
source_document_id: 7073
payload_hash: "ff82e2d46946cb467c0c7a186222ec996b17c0348b3ce8f6c1bb8c95aad474c5"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.698892+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Toxic Bullets

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_toxic_bullets`
- Snapshot ID: `40488`
- Source-Dokument: `7073`
- Kurzinfo: Toxic Bullets aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "Your bullets build up a <span class=\"highlight\">Bleed</span> on enemies, causing them to lose a <span class=\"highlight\">percentage</span> of their <span class=\"highlight\">Max Health</span> over time. Also applies <span class=\"highlight\">Healing Reduction</span> on the bleeding target.",
  "Info1": {
    "Alt": [
      {
        "Key": "DotDuration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "BuildUpPerShot",
        "Value": 1.28
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_toxic_bullets_desc",
    "Main": [
      {
        "Key": "DotHealthPercent",
        "Scale": {
          "Type": "spirit",
          "Value": 0.005
        },
        "Type": "tech_damage",
        "Value": 1.9
      },
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -35
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_toxic_bullets",
  "Name": "Toxic Bullets",
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
    "DotMultiplerTroopers": {
      "Key": "DotMultiplerTroopers",
      "Value": 0.5
    },
    "HealAmpRegenPenaltyPercent": {
      "Key": "HealAmpRegenPenaltyPercent",
      "Type": "damage",
      "UsageFlags": "ConditionallyApplied",
      "Value": -35
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 0.5
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy",
    "MinionEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "DotHealthPercent": 0.7,
    "HealAmpReceivePenaltyPercent": -30,
    "HealAmpRegenPenaltyPercent": -30
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
      "lookup": "toxic bullets",
      "name": "Toxic Bullets",
      "type": "item"
    }
  ]
}
````
