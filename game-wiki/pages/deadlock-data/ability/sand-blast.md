---
title: "Sand Blast"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_throw_sand"
canonical_name: "Sand Blast"
snapshot_id: 39542
source_document_id: 7070
payload_hash: "80b87a84cba6007de86d735b324355a7a84edbb6585e2bf567da546dad7176cb"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.588589+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Sand Blast

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_throw_sand`
- Snapshot ID: `39542`
- Source-Dokument: `7070`
- Kurzinfo: Sand Blast aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 25,
  "AbilityCooldown": 40.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 2.5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": 40,
  "DebuffModifier": {
    "Class": "CitadelThrowSandDebuff",
    "Subclass": "CitadelThrowSandDebuff"
  },
  "GrowthPerMeter": 0.5,
  "HeightOffGround": 20,
  "InitialWidth": 5,
  "IsDisabled": false,
  "Key": "ability_throw_sand",
  "Name": "Sand Blast",
  "Upgrades": [
    {
      "AbilityCastRange": 5,
      "Damage": 50
    },
    {
      "GroundDashReductionPercent": -30,
      "SlowPercent": 30
    },
    {
      "AbilityCooldown": -25.0,
      "AbilityDuration": 1.5
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
    "card_name": "Sand Blast",
    "hero_key": "hero_krill",
    "hero_name": "Mo & Krill",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_krill",
      "hero_name": "Mo & Krill",
      "lookup": "sand blast",
      "name": "Sand Blast",
      "type": "ability"
    }
  ]
}
````
