---
title: "Bounce Pad"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_bounce_pad"
canonical_name: "Bounce Pad"
snapshot_id: 39411
source_document_id: 7070
payload_hash: "c800e3ee1160be62e3e7351d3d58e90377f0ba988e468fc5aa050d9b33134221"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.257335+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Bounce Pad

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_bounce_pad`
- Snapshot ID: `39411`
- Source-Dokument: `7070`
- Kurzinfo: Bounce Pad aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.08,
  "AbilityCharges": 1,
  "AbilityCooldown": 41,
  "AbilityCooldownBetweenCharge": 3.5,
  "AbilityDuration": 22,
  "AbilityUnitTargetLimit": 1,
  "AirControlAccelPercent": 50,
  "AirControlPercent": 100,
  "AllyBounceModifier": {
    "Class": "BouncePadAlly",
    "Subclass": "BouncePadAlly"
  },
  "BarrelBounceVelocity": 800,
  "BarrelUpFactor": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorMovement",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BounceModifier": {
    "Class": "CitadelBouncePadStomp",
    "Subclass": "CitadelBouncePadStomp"
  },
  "BounceVelocity": 750,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_bounce_pad",
  "MinAirTimeForStomp": 0.2,
  "Name": "Bounce Pad",
  "PlaceDistance": 200,
  "Radius": 9,
  "Scale": 1,
  "SpeedOnLandModifier": {
    "Class": "Base",
    "Subclass": "SpeedOnLand"
  },
  "StompDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.372
    },
    "Value": 60
  },
  "TossSpeed": 12.7,
  "UpFactor": 1.2,
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "SpeedOnLand": 4,
      "SpeedOnLandDuration": 4
    },
    {
      "StompStunDuration": 0.7
    }
  ],
  "VerticalDifferenceTolerance": 60,
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
    "card_name": "Bounce Pad",
    "hero_key": "hero_astro",
    "hero_name": "Holliday",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_astro",
      "hero_name": "Holliday",
      "lookup": "bounce pad",
      "name": "Bounce Pad",
      "type": "ability"
    }
  ]
}
````
