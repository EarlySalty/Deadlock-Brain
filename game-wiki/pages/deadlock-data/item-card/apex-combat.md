---
title: "Apex Combat"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_apex_combat"
canonical_name: "Apex Combat"
snapshot_id: 40269
source_document_id: 7073
payload_hash: "9446be826b3863dc9932bc9f8e0e749426c144c0d230faa0e7669c45cf4f13b5"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.297945+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Apex Combat

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_apex_combat`
- Snapshot ID: `40269`
- Source-Dokument: `7073`
- Kurzinfo: Apex Combat aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_ricochet"
  ],
  "Cost": 9999,
  "Description": "Your bullets will <span class=\"highlight\">ricochet</span> on enemies near your target, <span class=\"highlight\">applying any bullet procs</span> and <span class=\"highlight\">dealing a percentage of the original damage.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "RicochetTargetsTooltipOnly",
        "Value": 4
      },
      {
        "Key": "RicochetRadius",
        "Type": "distance",
        "Value": "13m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_apex_combat_desc",
    "Main": [
      {
        "Key": "RicochetDamagePercent",
        "Type": "bullet_damage",
        "Value": 65
      }
    ],
    "Type": null
  },
  "Info2": {
    "Alt": [
      {
        "Key": "ProcChance",
        "Value": 40
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_critshot_desc",
    "Main": [
      {
        "Key": "CritDamagePercent",
        "Type": "bullet_damage",
        "Value": 125
      }
    ],
    "Type": null
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_apex_combat",
  "Name": "Apex Combat",
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
  "StreetBrawl": true,
  "TargetTypes": [
    "HeroEnemy",
    "BossEnemy",
    "TrooperEnemy",
    "PropEnemy",
    "MinionEnemy",
    "Neutral"
  ],
  "Tier": 5,
  "Upgrades": {
    "CritDamagePercent": 30,
    "ProcChance": 5,
    "RicochetDamagePercent": 25
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
  }
}
````
