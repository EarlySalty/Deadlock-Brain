---
title: "Scourge"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_discord"
canonical_name: "Scourge"
snapshot_id: 40327
source_document_id: 7073
payload_hash: "812f0b9a1e804150481512c00b767be11330970a8cc0d4df10fb30912a031ef6"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.399613+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Scourge

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_discord`
- Snapshot ID: `40327`
- Source-Dokument: `7073`
- Kurzinfo: Scourge aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 6400,
  "Description": "Apply <span class=\"highlight\">Spirit Resist</span> and an aura on a friendly target that deals <span class=\"highlight\">{g:citadel_inline_attribute:'SpiritIcon'}damage</span> to enemies proportional to their max health. <br>Can be self cast.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 100
      },
      {
        "Key": "StatusResistancePercent",
        "Type": "duration",
        "Value": 17
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
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 10
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "35m"
      },
      {
        "Key": "AuraRadius",
        "Type": "distance",
        "Value": "10m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 35.0,
    "DescKey": "#upgrade_discord_desc",
    "Main": [
      {
        "Key": "MaxHealthPercentAsDPS",
        "Scale": {
          "Type": "spirit",
          "Value": 0.0055
        },
        "Type": "tech_damage",
        "Value": 2.6
      },
      {
        "Key": "TechResist",
        "UsageFlags": "ConditionallyApplied",
        "Value": 40
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_discord",
  "Name": "Scourge",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.25
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
    "TickRate": {
      "Key": "TickRate",
      "Value": 0.25
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityDuration": 3,
    "BonusHealth": 125,
    "CombatBarrier": 300,
    "MaxHealthPercentAsDPS": 2,
    "StatusResistancePercent": 20
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
      "lookup": "scourge",
      "name": "Scourge",
      "type": "item"
    }
  ]
}
````
