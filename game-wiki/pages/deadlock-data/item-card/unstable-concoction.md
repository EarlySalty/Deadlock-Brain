---
title: "Unstable Concoction"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_unstable_concoction"
canonical_name: "Unstable Concoction"
snapshot_id: 40492
source_document_id: 7073
payload_hash: "c60a18a16d78b86719974cc8b36f5a4e05a8290f14e68353feffbf61ca11eeb9"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.706189+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Unstable Concoction

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_unstable_concoction`
- Snapshot ID: `40492`
- Source-Dokument: `7073`
- Kurzinfo: Unstable Concoction aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 9999,
  "Description": "Consume a concoction that grants you <span class=\"highlight\">Unstoppable</span> and increased <span class=\"highlight\">speed, health, spirit and weapon damage</span>. After a short duration <span class=\"highlight\">you die and explode</span>, stunning nearby enemies and dealing damage based on your maximum health. Dying this way reduces your respawn time by <span class=\"highlight\">50%</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 4.0
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 150
      },
      {
        "Key": "Radius",
        "Value": "22m"
      },
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 150
      }
    ],
    "ChargeUp": null,
    "Cooldown": 25.0,
    "DescKey": "#upgrade_unstable_concoction_desc",
    "Main": [
      {
        "Key": "MaxHPDamage",
        "Type": "tech_damage",
        "Value": 30
      },
      {
        "Key": "StunDuration",
        "Type": "duration",
        "Value": 3.0
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "10m"
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "UsageFlags": "ConditionallyApplied",
        "Value": 3000
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_unstable_concoction",
  "Name": "Unstable Concoction",
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
    "RespawnTimeMod": {
      "Key": "RespawnTimeMod",
      "Value": 50
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "BaseAttackDamagePercent": 50,
    "BonusHealth": 1300,
    "StunDuration": 0.5,
    "TechPower": 50
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
      "lookup": "unstable concoction",
      "name": "Unstable Concoction",
      "type": "item"
    }
  ]
}
````
