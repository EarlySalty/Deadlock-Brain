---
title: "Lifestrike"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_boxing_glove"
canonical_name: "Lifestrike"
snapshot_id: 40287
source_document_id: 7073
payload_hash: "7ce3431ec098d9c3835ce50c21947a1f2c80c34e9ff8fc2f862e7c341343f442"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.330002+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Lifestrike

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_boxing_glove`
- Snapshot ID: `40287`
- Source-Dokument: `7073`
- Kurzinfo: Lifestrike aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_lifestrike_gauntlets"
  ],
  "Cost": 3200,
  "Description": "Your <span class=\"highlight\">Melee Attack</span> applies <span class=\"highlight\">Movement Slow</span> and <span class=\"highlight\">heals you</span> for a percentage of the <span class=\"highlight\">Melee Damage</span> dealt plus a fixed amount. <span class=\"diminish\"><br><br>This heal is 40% effective vs non-heroes. <br>Cooldown is 1.5x as long for Light Melee hits.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 125
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusMeleeDamagePercent",
        "Type": "melee_damage",
        "Value": 16
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "SlowDuration",
        "Type": "duration",
        "Value": 2.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 4,
    "DescKey": "#upgrade_boxing_glove_desc",
    "Main": [
      {
        "Key": "SlowPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 60
      },
      {
        "Key": "LifestealHeal",
        "Scale": {
          "Type": "power_increase",
          "Value": 1.5
        },
        "Type": "healing",
        "Value": 100
      },
      {
        "Key": "LifestealHealPercent",
        "Scale": {
          "Type": "power_increase",
          "Value": 0.5
        },
        "Type": "healing",
        "Value": 30
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_boxing_glove",
  "Name": "Lifestrike",
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
    "LightMeleeCooldownMult": {
      "Key": "LightMeleeCooldownMult",
      "Value": 1.5
    },
    "NonHeroHealPct": {
      "Key": "NonHeroHealPct",
      "Value": 40
    }
  },
  "ShopFilters": [
    "Durability",
    "Melee",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -3,
    "BonusHealth": 125,
    "BonusMeleeDamagePercent": 10
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
      "lookup": "lifestrike",
      "name": "Lifestrike",
      "type": "item"
    }
  ]
}
````
