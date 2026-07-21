---
title: "Slowing Hex"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_containment"
canonical_name: "Slowing Hex"
snapshot_id: 40316
source_document_id: 7073
payload_hash: "7a1dc5741d52d58891b6c829121ca6dabe38cbb883beba59271975fe5bf34773"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.377498+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Slowing Hex

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_containment`
- Snapshot ID: `40316`
- Source-Dokument: `7073`
- Kurzinfo: Slowing Hex aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 1600,
  "Description": "<span class=\"highlight\">Slows movement</span> of enemy target. Also <span class=\"highlight\">Silences</span> their <span class=\"highlight\">movement-based items and abilities</span>.<br><span class=\"diminish\">Increases the target's gravity.<br>Does not affect target's stamina usage.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.5m"
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
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "25m"
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 3.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 27,
    "DescKey": "#upgrade_containment_desc",
    "Main": [
      {
        "Key": "SlowPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 20
      },
      {
        "Key": "GroundDashReductionPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": -30
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_containment",
  "Name": "Slowing Hex",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
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
    "Movement",
    "Disruption",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "AbilityCooldown": -18,
    "GroundDashReductionPercent": -6,
    "SlowPercent": 10
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
      "lookup": "slowing hex",
      "name": "Slowing Hex",
      "type": "item"
    }
  ]
}
````
