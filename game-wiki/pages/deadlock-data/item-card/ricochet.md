---
title: "Ricochet"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_ricochet"
canonical_name: "Ricochet"
snapshot_id: 40441
source_document_id: 7073
payload_hash: "e2be0653810dbd4ce1be5b9035e070aa3aa99c59f69cc1c3ab0c0332e926c8dd"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.596637+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Ricochet

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_ricochet`
- Snapshot ID: `40441`
- Source-Dokument: `7073`
- Kurzinfo: Ricochet aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Your bullets will <span class=\"highlight\">ricochet</span> on enemies near your target, <span class=\"highlight\">applying any bullet procs</span> and <span class=\"highlight\">dealing a percentage of the original damage.</span>",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusFireRate",
        "Value": 18
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "RicochetTargetsTooltipOnly",
        "Value": 2
      },
      {
        "Key": "RicochetRadius",
        "Type": "distance",
        "Value": "13m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_ricochet_desc",
    "Main": [
      {
        "Key": "RicochetDamagePercent",
        "Type": "bullet_damage",
        "Value": 65
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_ricochet",
  "Name": "Ricochet",
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
  "ShopFilters": [
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "BossEnemy",
    "TrooperEnemy",
    "PropEnemy",
    "MinionEnemy",
    "Neutral"
  ],
  "Tier": 4,
  "Upgrades": {
    "BonusFireRate": 25,
    "RicochetDamagePercent": 15
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
      "lookup": "ricochet",
      "name": "Ricochet",
      "type": "item"
    }
  ]
}
````
