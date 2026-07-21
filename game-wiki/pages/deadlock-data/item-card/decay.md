---
title: "Decay"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_rupture"
canonical_name: "Decay"
snapshot_id: 40445
source_document_id: 7073
payload_hash: "96d232248818a54bbb59f25b2a07b1aa394175095112a66d315bf71908662032"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.604127+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Decay

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_rupture`
- Snapshot ID: `40445`
- Source-Dokument: `7073`
- Kurzinfo: Decay aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 3200,
  "Description": "Inflict <span class=\"highlight\">damage over time</span> to a target, dealing damage based on their current health.<br>Decay's damage is non-lethal and does not apply item procs.",
  "Info1": {
    "Alt": [
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "Value": 8
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 65
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
        "Scale": {
          "Type": "spirit",
          "Value": 0.1
        },
        "Type": "range",
        "Value": "20m"
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": 30.0,
    "DescKey": "#upgrade_rupture_desc",
    "Main": [
      {
        "Key": "DotHealthPercent",
        "Scale": {
          "Type": "spirit",
          "Value": 0.004
        },
        "Type": "tech_damage",
        "Value": 2.6
      },
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -50
      },
      {
        "Key": "AbilityCastRange",
        "Scale": {
          "Type": "spirit",
          "Value": 0.1
        },
        "Type": "range",
        "Value": "20m"
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rupture",
  "Name": "Decay",
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
    },
    "HealAmpRegenPenaltyPercent": {
      "Key": "HealAmpRegenPenaltyPercent",
      "Type": "damage",
      "UsageFlags": "ConditionallyApplied",
      "Value": -50
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 1.0
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -10,
    "BonusHealth": 90,
    "DotHealthPercent": 0.5,
    "HealAmpReceivePenaltyPercent": -20,
    "HealAmpRegenPenaltyPercent": -20,
    "TechPower": 12
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
      "lookup": "decay",
      "name": "Decay",
      "type": "item"
    }
  ]
}
````
