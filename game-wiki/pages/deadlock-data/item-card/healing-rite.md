---
title: "Healing Rite"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_health_stimpak"
canonical_name: "Healing Rite"
snapshot_id: 40372
source_document_id: 7073
payload_hash: "c95dbdc9dcf8b014a56ab9a50a843719b119aa98e2c895ac3f33b6356348f957"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.480675+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Healing Rite

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_health_stimpak`
- Snapshot ID: `40372`
- Source-Dokument: `7073`
- Kurzinfo: Healing Rite aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 800,
  "Description": "Grant <span class=\"highlight\">Regen</span> and <span class=\"highlight\">Sprint Speed</span> to the target. Gets dispelled if you take damage from enemy players or objectives. Can be self-cast.",
  "Info1": {
    "Alt": [
      {
        "Key": "RegenDuration",
        "Type": "duration",
        "Value": 20
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "30m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 70,
    "DescKey": "#upgrade_health_stimpak_desc",
    "Main": [
      {
        "Key": "TotalHealthRegen",
        "Scale": {
          "Type": "spirit",
          "Value": 1.1
        },
        "Type": "healing",
        "Value": 300
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "2m"
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_health_stimpak",
  "Name": "Healing Rite",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.2
    },
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
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 1,
  "Upgrades": {
    "AbilityCooldown": -60,
    "BonusSprintSpeed": "6m",
    "TotalHealthRegen": 600
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
      "lookup": "healing rite",
      "name": "Healing Rite",
      "type": "item"
    }
  ]
}
````
