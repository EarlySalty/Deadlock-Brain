---
title: "Frostbite Charm"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_shivas_bracelet"
canonical_name: "Frostbite Charm"
snapshot_id: 40450
source_document_id: 7073
payload_hash: "09b680e61fdca190657e046baf54aa162f32873e997ac956924c38330c14aa7d"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.613090+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Frostbite Charm

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_shivas_bracelet`
- Snapshot ID: `40450`
- Source-Dokument: `7073`
- Kurzinfo: Frostbite Charm aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "Imbued ability has its <span class=\"highlight\">Cooldown reduced</span> and gains <span class=\"highlight\">Spirit Power</span>. When the ability deals damage, <span class=\"highlight\">freeze</span> the target and apply <span class=\"highlight\">bonus damage</span>. <span class=\"diminish\"><br><br>Cooldown is per target.</span>",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 10.0,
    "DescKey": "#upgrade_shivas_bracelet_desc",
    "Main": [
      {
        "Key": "ImbuedCooldownReduction",
        "Type": "cooldown",
        "Value": 50
      },
      {
        "Key": "ImbuedTechPower",
        "Type": "tech_damage",
        "Value": 70
      },
      {
        "Key": "Damage",
        "Type": "tech_damage",
        "Value": 200
      },
      {
        "Key": "FreezeDuration",
        "Value": 1
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_shivas_bracelet",
  "Name": "Frostbite Charm",
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
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityCooldown": -2,
    "Damage": 100,
    "FreezeDuration": 0.3,
    "ImbuedCooldownReduction": 15,
    "ImbuedTechPower": 40
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
      "lookup": "frostbite charm",
      "name": "Frostbite Charm",
      "type": "item"
    }
  ]
}
````
