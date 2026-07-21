---
title: "Love Bites"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_vampirebat_lovebites"
canonical_name: "Love Bites"
snapshot_id: 39559
source_document_id: 7070
payload_hash: "d1a53350a658ada08666c9a7ba35b4fb3de266e78012d7fdfac2b77e9a21363a"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.634246+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Love Bites

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_vampirebat_lovebites`
- Snapshot ID: `39559`
- Source-Dokument: `7070`
- Kurzinfo: Love Bites aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "BonusDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.85
    },
    "Value": 45
  },
  "BuildUpDuration": 5,
  "BuildUpHeadshotBonus": 1.5,
  "BuildUpModifier": {
    "BuildUpDecayDelay": 3.0,
    "Class": "CitadelBaseBuildup",
    "Subclass": "LovebitesBuildup"
  },
  "BuildUpPerBat": 20,
  "BuildUpPerDagger": 30,
  "BuildUpPerShot": 18.4,
  "ChannelMoveSpeed": -1,
  "DamageProcModifier": {
    "BuffModifier": {
      "Class": "Base",
      "Subclass": "LovebitesBuff"
    },
    "Class": "VampirebatLovebitesproc",
    "SlowModifier": {
      "Class": "SlowBase",
      "Subclass": "Slowsf"
    },
    "Subclass": "LovebitesProc"
  },
  "EffectivenessVolumeScaleMax": 1.0,
  "EffectivenessVolumeScaleMin": 0.5,
  "IsDisabled": false,
  "Key": "ability_vampirebat_lovebites",
  "MagicDamagePerBullet": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.09
    },
    "Value": 4
  },
  "Name": "Love Bites",
  "PerTargetCooldown": 10,
  "Upgrades": [
    {
      "SlowDuration": 3,
      "SlowPercent": 30
    },
    {
      "BonusDamage": 45,
      "MagicDamagePerBullet": 3.0
    },
    {
      "BonusFireRate": 25,
      "BuffDuration": 5,
      "PerTargetCooldown": -5
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
    "card_name": "Love Bites",
    "hero_key": "hero_vampirebat",
    "hero_name": "Mina",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_vampirebat",
      "hero_name": "Mina",
      "lookup": "love bites",
      "name": "Love Bites",
      "type": "ability"
    }
  ]
}
````
