---
title: "Healing Tempo"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_healbuff"
canonical_name: "Healing Tempo"
snapshot_id: 40364
source_document_id: 7073
payload_hash: "8c2cb965b3c14cfd7702183768f2a78530eddfe8d5312c835cf376e46fb3becb"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.467152+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Healing Tempo

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_healbuff`
- Snapshot ID: `40364`
- Source-Dokument: `7073`
- Kurzinfo: Healing Tempo aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_healing_booster"
  ],
  "Cost": 6400,
  "Description": "Applying {g:citadel_inline_attribute:'Heal'} to yourself or an ally grants the target {g:citadel_inline_attribute:'BonusFireRate'} and {g:citadel_inline_attribute:'BonusMoveSpeed'}.<br><br><span class=\"diminish\">Does not apply on innate Regen or passive Bullet/Spirit Lifesteals.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "Value": 10
      },
      {
        "Key": "BonusHealthRegen",
        "Type": "healing",
        "Value": 6
      },
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 4
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "HealAmpCastPercent",
        "Value": 25
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BuffDuration",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_healbuff_desc",
    "Main": [
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "Value": 35
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "1.25m"
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_healbuff",
  "Name": "Healing Tempo",
  "Other": {
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 1
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
    "HealAmpRegenPercent": {
      "Key": "HealAmpRegenPercent",
      "Value": 25
    },
    "MinimumHealAmount": {
      "Key": "MinimumHealAmount",
      "Value": 1
    }
  },
  "ShopFilters": [
    "FireRate",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "BonusFireRate": 20,
    "BonusHealthRegen": 6,
    "BonusMoveSpeed": "2m",
    "HealAmpCastPercent": 10,
    "HealAmpRegenPercent": 10,
    "TechResist": 10
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
      "lookup": "healing tempo",
      "name": "Healing Tempo",
      "type": "item"
    }
  ]
}
````
