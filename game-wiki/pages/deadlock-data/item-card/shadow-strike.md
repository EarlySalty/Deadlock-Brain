---
title: "Shadow Strike"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_shadow_strike"
canonical_name: "Shadow Strike"
snapshot_id: 40448
source_document_id: 7073
payload_hash: "e9cf0fda828751fa3f7965b2d2a604aefe226f9b671388491ea7f5feb3695ca3"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.609253+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Shadow Strike

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_shadow_strike`
- Snapshot ID: `40448`
- Source-Dokument: `7073`
- Kurzinfo: Shadow Strike aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "Go <span class=\"highlight\">Invisible</span> on <span class=\"highlight\">Stamina use</span> with no detection range. Doing a <span class=\"highlight\">melee attack</span> while invisible will cause you to <span class=\"highlight\">steal bullet and spirit resistance</span> from them and deal <span class=\"highlight\">damage over time</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "Stamina",
        "Value": 3
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 350
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
        "Key": "InvisFadeToDuration",
        "Type": "duration",
        "Value": 0.2
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_shadow_strike_desc",
    "Main": [
      {
        "Key": "DPS",
        "Scale": {
          "Type": "spirit",
          "Value": 0.4
        },
        "Type": "tech_damage",
        "Value": 125.0
      },
      {
        "Key": "AbilityDuration",
        "LocTokenOverride": "InvisDuration",
        "Type": "duration",
        "Value": 3
      },
      {
        "Key": "StealDuration",
        "Type": "duration",
        "Value": 6
      },
      {
        "Key": "ResistStealAmount",
        "Type": "tech_armor_up",
        "Value": 40
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_shadow_strike",
  "Name": "Shadow Strike",
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
    "InvisAlertWhenFading": {
      "Key": "InvisAlertWhenFading",
      "Value": 1
    },
    "MaxStacks": {
      "Key": "MaxStacks",
      "Value": 1
    },
    "RevealOnDamageDuration": {
      "Key": "RevealOnDamageDuration",
      "Type": "duration",
      "Value": 1.5
    },
    "RevealOnSpottedDuration": {
      "Key": "RevealOnSpottedDuration",
      "Type": "duration",
      "Value": 0.25
    },
    "SpottedRadius": {
      "Key": "SpottedRadius",
      "Type": "distance",
      "Value": "0m"
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityDuration": 3,
    "BonusHealth": 250,
    "DPS": 125,
    "ResistStealAmount": 20,
    "Stamina": 1
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
      "lookup": "shadow strike",
      "name": "Shadow Strike",
      "type": "item"
    }
  ]
}
````
