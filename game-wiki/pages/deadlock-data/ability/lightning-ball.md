---
title: "Lightning Ball"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_lightning_ball"
canonical_name: "Lightning Ball"
snapshot_id: 39626
source_document_id: 7070
payload_hash: "dfd9fa9f688076612aebc0434cd39bf67c18f3ae5cea9e306246a61ed9c3a839"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.807065+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Lightning Ball

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_lightning_ball`
- Snapshot ID: `39626`
- Source-Dokument: `7070`
- Kurzinfo: Lightning Ball aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCharges": 1,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.5
    },
    "Value": 75
  },
  "IsDisabled": false,
  "Key": "citadel_ability_lightning_ball",
  "MaxLifetime": 5,
  "MinShockDuration": 0.5,
  "Name": "Lightning Ball",
  "ShockRadius": 4.25,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "GigagwattLightningballSlow"
  },
  "TickRate": 0.1,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "MaxLifetime": 1,
      "SlowPercent": 35
    },
    {
      "DPS": 58.5,
      "ShockRadius": 1.5
    }
  ],
  "ZapModifier": {
    "Class": "CitadelLightningball",
    "Subclass": "CitadelLightningball"
  },
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
    "card_name": "Lightning Ball",
    "hero_key": "hero_gigawatt",
    "hero_name": "Seven",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_gigawatt",
      "hero_name": "Seven",
      "lookup": "lightning ball",
      "name": "Lightning Ball",
      "type": "ability"
    }
  ]
}
````
