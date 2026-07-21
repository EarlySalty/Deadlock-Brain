---
title: "Arctic Blast"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_arctic_blast"
canonical_name: "Arctic Blast"
snapshot_id: 40275
source_document_id: 7073
payload_hash: "35f19230b8fa7280074f2f84bcdbd78f7c21f624845a29389d127a7ce7317830"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.308076+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Arctic Blast

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_arctic_blast`
- Snapshot ID: `40275`
- Source-Dokument: `7073`
- Kurzinfo: Arctic Blast aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": [
    "upgrade_cold_front"
  ],
  "Cost": 6400,
  "Description": "Release an expanding ice blast that deals {g:citadel_inline_attribute:'SpiritDamage'}, <span class=\"highlight\">Freezing</span> and then <span class=\"highlight\">Slowing</span> targets it hits.<br><br>Slowed targets have their <span class=\"highlight\">stamina regen frozen</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "TechResist",
        "Value": 10
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
        "Key": "EndRadius",
        "Type": "distance",
        "Value": "16m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 24.0,
    "DescKey": "#upgrade_arctic_blast_desc",
    "Main": [
      {
        "Key": "Damage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.6975
        },
        "Type": "tech_damage",
        "Value": 175.0
      },
      {
        "Key": "FreezeDuration",
        "Type": "duration",
        "Value": 1
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_arctic_blast",
  "Name": "Arctic Blast",
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
    "DamageHeight": {
      "Key": "DamageHeight",
      "Value": "7m"
    },
    "NPCDamageMult": {
      "Key": "NPCDamageMult",
      "Type": "tech_damage",
      "Value": 1
    },
    "SlowDuration": {
      "Key": "SlowDuration",
      "LocTokenOverride": "ArcticBlastSlowDuration",
      "Type": "duration",
      "Value": 4
    },
    "SlowPercent": {
      "Key": "SlowPercent",
      "Type": "slow",
      "UsageFlags": "ConditionallyApplied",
      "Value": 60
    },
    "SpreadDuration": {
      "Key": "SpreadDuration",
      "Type": "duration",
      "Value": 0.6
    },
    "StartRadius": {
      "Key": "StartRadius",
      "Type": "distance",
      "Value": "2m"
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -12,
    "Damage": 150,
    "FreezeDuration": 0.25,
    "TechResist": 15
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
      "lookup": "arctic blast",
      "name": "Arctic Blast",
      "type": "item"
    }
  ]
}
````
