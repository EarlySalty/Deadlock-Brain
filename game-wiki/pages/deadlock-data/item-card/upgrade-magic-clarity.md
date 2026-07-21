---
title: "upgrade_magic_clarity"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_magic_clarity"
canonical_name: "upgrade_magic_clarity"
snapshot_id: 40394
source_document_id: 7073
payload_hash: "be76a2e855bd3713ad79ce0dddfc5a3d4abff0c3c0605b5f17cf5f3fd133a18c"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.517496+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# upgrade_magic_clarity

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_clarity`
- Snapshot ID: `40394`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_magic_clarity aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "On ability or item use, gain a <span class=\"highlight\">move speed</span> bonus for {s:AbilityDuration}s. Your next ability cast will remove the move speed bonus and apply a <span class=\"highlight\">Spirit</span> bonus to that ability.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 16.0,
    "DescKey": "#upgrade_magic_clarity_desc",
    "Main": [
      {
        "Key": "BonusSpirit",
        "LocTokenOverride": "ClarityBonusSpirit",
        "Type": "tech_damage",
        "Value": 28
      },
      {
        "Key": "BonusMovespeed",
        "Type": "movement_speed",
        "Value": "2m"
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_magic_clarity",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 8
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusSpiritMaxTime": {
      "Key": "BonusSpiritMaxTime",
      "Value": 12
    },
    "BonusSpiritMin": {
      "Key": "BonusSpiritMin",
      "Value": 2
    },
    "BonusSpiritWindow": {
      "Key": "BonusSpiritWindow",
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
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
