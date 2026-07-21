---
title: "Fixation"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_stacking_damage"
canonical_name: "Fixation"
snapshot_id: 39536
source_document_id: 7070
payload_hash: "033b14d264faeafcda4c685f46ff164ef39ce55b472b27a5adf1869087f15f42"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.570776+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Fixation

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_stacking_damage`
- Snapshot ID: `39536`
- Source-Dokument: `7070`
- Kurzinfo: Fixation aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "ChannelMoveSpeed": -1,
  "DamageBonusFixedPerStack": 0.2,
  "IsDisabled": false,
  "Key": "ability_stacking_damage",
  "MaxStacks": 40,
  "Name": "Fixation",
  "ProcDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "StackingModifier": {
    "Class": "PassiveHazeStackingDamage",
    "SlowModifier": {
      "Class": "SlowBase",
      "Subclass": "Slow"
    },
    "Subclass": "PassiveHazeStackingDamage"
  },
  "Upgrades": [
    {
      "ProcDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.8
        },
        "Value": 40
      },
      "ProcDamageStackCount": 20,
      "SlowDuration": 2,
      "SlowPercent": 15
    },
    {
      "AbilityDuration": 5,
      "MaxStacks": 40
    },
    {
      "DamageBonusFixedPerStack": 0.14
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-data.json",
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
  "_deadlock_data_card": {
    "card_name": "Fixation",
    "hero_key": "hero_haze",
    "hero_name": "Haze",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_haze",
      "hero_name": "Haze",
      "lookup": "fixation",
      "name": "Fixation",
      "type": "ability"
    }
  ]
}
````
