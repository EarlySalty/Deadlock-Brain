---
title: "Full Auto"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_wraith_rapidfire"
canonical_name: "Full Auto"
snapshot_id: 39665
source_document_id: 7070
payload_hash: "65a4e299254f951bfee5b932ca499cd910a61fc787f547f3f781f41e8efaf257"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.897775+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Full Auto

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_wraith_rapidfire`
- Snapshot ID: `39665`
- Source-Dokument: `7070`
- Kurzinfo: Full Auto aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 45,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "BonusFireRate": 20,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "citadel_ability_wraith_rapidfire",
  "MagicDamagePerBullet": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.03
    },
    "Value": 2
  },
  "Name": "Full Auto",
  "RapidFireModifier": {
    "Class": "Rapidfire",
    "Subclass": "Rapidfire"
  },
  "Upgrades": [
    {
      "AbilityCooldown": -20
    },
    {
      "AbilityDuration": 3,
      "BonusFireRate": 10
    },
    {
      "MagicDamagePerBullet": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.06
        },
        "Value": 0
      },
      "UnlimitedAmmo": 1
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
    "card_name": "Full Auto",
    "hero_key": "hero_wraith",
    "hero_name": "Wraith",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_wraith",
      "hero_name": "Wraith",
      "lookup": "full auto",
      "name": "Full Auto",
      "type": "ability"
    }
  ]
}
````
