---
title: "Cursed Relic"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_glitch"
canonical_name: "Cursed Relic"
snapshot_id: 40353
source_document_id: 7073
payload_hash: "ee8a912184cae44c59a8d225ecdafb1760222b69b950ac8aa17faccd350e12db"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.447341+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Cursed Relic

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_glitch`
- Snapshot ID: `40353`
- Source-Dokument: `7073`
- Kurzinfo: Cursed Relic aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 6400,
  "Description": "Curses an enemy - <span class=\"highlight\">interrupting, Silencing, Disarming</span>, and <span class=\"highlight\">preventing item usage</span>. <span class=\"highlight\">Removes all non-ultimate buffs</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutgoingDamagePenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -14
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
        "Value": 3.25
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "20m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 55.0,
    "DescKey": "#upgrade_glitch_desc",
    "Main": [
      {
        "Key": "StatusEffectEMP",
        "Value": null
      },
      {
        "Key": "StatusEffectDisarmed",
        "Value": null
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_glitch",
  "Name": "Cursed Relic",
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
    "SkipFrames": {
      "Key": "SkipFrames",
      "UsageFlags": "ConditionallyApplied",
      "Value": 6
    }
  },
  "ShopFilters": [
    "Disruption",
    "WeaponDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -40,
    "AbilityDuration": 0.25
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
      "lookup": "cursed relic",
      "name": "Cursed Relic",
      "type": "item"
    }
  ]
}
````
