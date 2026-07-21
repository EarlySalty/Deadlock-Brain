---
title: "Setting Sun"
entity_type: "ability"
source: "deadlock_data"
external_id: "yakuza_setting_sun"
canonical_name: "Setting Sun"
snapshot_id: 39743
source_document_id: 7070
payload_hash: "82e73a87e13ccae7681fb763c1de0c353e013b7d96cae05afd3388ecb72752b0"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.102213+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Setting Sun

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `yakuza_setting_sun`
- Snapshot ID: `39743`
- Source-Dokument: `7070`
- Kurzinfo: Setting Sun aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 74.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "CenterDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 250
  },
  "CenterRadius": 5,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "yakuza_setting_sun",
  "Name": "Setting Sun",
  "OuterDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 100
  },
  "Radius": 10,
  "Range": 25,
  "SettingSunThinkerModifier": {
    "Class": "SettingSunThinker",
    "Subclass": "SettingSunThinker"
  },
  "ShootDuration": 1.5,
  "TargetingDuration": 1.0,
  "Upgrades": [
    {
      "AbilityCooldown": -19.0
    },
    {
      "Range": 175
    },
    {
      "CenterDamage": 200
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_yakuza",
      "hero_name": "The Boss",
      "lookup": "setting sun",
      "name": "Setting Sun",
      "type": "ability"
    }
  ]
}
````
