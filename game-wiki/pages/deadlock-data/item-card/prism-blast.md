---
title: "Prism Blast"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_prism_blast"
canonical_name: "Prism Blast"
snapshot_id: 40420
source_document_id: 7073
payload_hash: "e7c70cafb7ed65ec8202f095412f46124e218cb6528f762b4b335f6ae396d9c2"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.561291+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Prism Blast

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_prism_blast`
- Snapshot ID: `40420`
- Source-Dokument: `7073`
- Kurzinfo: Prism Blast aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 9999,
  "Description": "You enter a void state and become <span class=\"highlight\">untargetable and invincible</span> for a short duration, during which lasers blast out and rotate around you.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 40.0,
    "DescKey": "#upgrade_prism_blast_desc",
    "Main": [
      {
        "Key": "DPS",
        "Scale": {
          "Type": "spirit",
          "Value": 1.75
        },
        "Type": "tech_damage",
        "Value": 270.0
      },
      {
        "Key": "AbilityDuration",
        "LocTokenOverride": "ShiftingVeilDuration",
        "Type": "duration",
        "Value": 6.0
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_prism_blast",
  "Name": "Prism Blast",
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
    "BeamLength": {
      "Key": "BeamLength",
      "Type": "distance",
      "Value": "30m"
    },
    "BeamWidth": {
      "Key": "BeamWidth",
      "Value": "2.9m"
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DampingFactor": {
      "Key": "DampingFactor",
      "Value": 3
    },
    "FloatMoveSpeed": {
      "Key": "FloatMoveSpeed",
      "Type": "slow",
      "UsageFlags": "ConditionallyApplied",
      "Value": "2.5m"
    },
    "LiftHeight": {
      "Key": "LiftHeight",
      "Value": 100
    }
  },
  "ShopFilters": [
    "FireRate"
  ],
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityCooldown": -10,
    "AbilityDuration": 2
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
      "lookup": "prism blast",
      "name": "Prism Blast",
      "type": "item"
    }
  ]
}
````
