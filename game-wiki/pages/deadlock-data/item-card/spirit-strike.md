---
title: "Spirit Strike"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_acolytes_glove"
canonical_name: "Spirit Strike"
snapshot_id: 40258
source_document_id: 7073
payload_hash: "a16ca4b520f3589340c1dff0fcb22bdfa90d61546308206f1af056d82f681190"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.280662+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Spirit Strike

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_acolytes_glove`
- Snapshot ID: `40258`
- Source-Dokument: `7073`
- Kurzinfo: Spirit Strike aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": "When you perform a <span class=\"highlight\">Light or Heavy Melee</span> attack against a hero, deal extra {g:citadel_inline_attribute:'SpiritDamage'} with the attack and reduce the target's <span class=\"highlight\">Spirit Resist</span>.<span class=\"diminish\"><br><br>Cooldown is 2x longer for Light Melee hits.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": 8,
    "DescKey": "#upgrade_acolytes_glove_desc",
    "Main": [
      {
        "Key": "SpiritDamage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.372
        },
        "Type": "tech_damage",
        "Value": 40.0
      },
      {
        "Key": "TechArmorDamageReduction",
        "Type": "tech_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -6
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_acolytes_glove",
  "Name": "Spirit Strike",
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
      "Value": 2
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Melee"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 1,
  "Upgrades": {
    "SpiritDamage": 80,
    "TechArmorDamageReduction": -5
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
      "lookup": "spirit strike",
      "name": "Spirit Strike",
      "type": "item"
    }
  ]
}
````
