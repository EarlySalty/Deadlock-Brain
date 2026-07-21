---
title: "Lightning Scroll"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_ultimate_burst"
canonical_name: "Lightning Scroll"
snapshot_id: 40491
source_document_id: 7073
payload_hash: "31ad2cfb213ad16774c9c380407aed48f0bd6d8832526ced0f7dd3d806716fcc"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.704345+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Lightning Scroll

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_ultimate_burst`
- Snapshot ID: `40491`
- Source-Dokument: `7073`
- Kurzinfo: Lightning Scroll aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_magic_slow"
  ],
  "Cost": 6400,
  "Description": "Damage from your ultimate applies a {g:citadel_inline_attribute:'Stun'} and deals {g:citadel_inline_attribute:'BonusSpiritDamage'} after a short delay.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "MovementSpeedSlow",
        "LocTokenOverride": "LightningScrollMysticSlow",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 50
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.75m"
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
        "Key": "DelayBeforeStun",
        "Type": "duration",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_ultimate_burst_desc",
    "Main": [
      {
        "Key": "StatusEffectStun",
        "Value": null
      },
      {
        "Key": "StunDuration",
        "Type": "duration",
        "Value": 0.75
      },
      {
        "Key": "Damage",
        "Type": "tech_damage",
        "Value": 150
      }
    ],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_ultimate_burst_hint",
    "Main": [],
    "Type": "Passive"
  },
  "Info4": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_ultimate_burst_upgrade",
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_ultimate_burst",
  "Name": "Lightning Scroll",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 2
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
    "GroundDashReductionPercent": {
      "Key": "GroundDashReductionPercent",
      "Type": "slow",
      "UsageFlags": "ConditionallyApplied",
      "Value": -12
    },
    "SlowPercent": {
      "Key": "SlowPercent",
      "Type": "slow",
      "Value": 80
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "BonusHealth": 100,
    "BonusSprintSpeed": "5m",
    "Damage": 100,
    "StunDuration": 0.75
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
      "lookup": "lightning scroll",
      "name": "Lightning Scroll",
      "type": "item"
    }
  ]
}
````
