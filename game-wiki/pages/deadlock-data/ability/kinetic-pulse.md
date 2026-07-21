---
title: "Kinetic Pulse"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_stomp"
canonical_name: "Kinetic Pulse"
snapshot_id: 39648
source_document_id: 7070
payload_hash: "f34035c7769f4d7bf4b8d9cc1e5adc246fc6a747ab8f233558b82723751f2af7"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.858436+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Kinetic Pulse

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_stomp`
- Snapshot ID: `39648`
- Source-Dokument: `7070`
- Kurzinfo: Kinetic Pulse aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.42,
  "AbilityCharges": 1,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": 5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BulletResistModifier": {
    "Class": "StompDebuff",
    "Subclass": "StompDebuff"
  },
  "ChannelMoveSpeed": 1.3,
  "ClimbHeight": 1.0,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.55
    },
    "Value": 115.0
  },
  "DebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "DebuffModifier"
  },
  "DistanceAboveGround": 1.0,
  "DropDownRate": 20,
  "ImpactInterval": 0.1,
  "IsDisabled": false,
  "Key": "citadel_ability_stomp",
  "Name": "Kinetic Pulse",
  "StompRange": 16,
  "StompWidth": 5.5,
  "TechCleaveExpireTime": 0.2,
  "TossDuration": 1,
  "TossSpeed": 450,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "BulletResistReduction": -15,
      "SlowDuration": 4,
      "SlowPercent": 30
    },
    {
      "Damage": 135,
      "StompRange": 20
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
    "card_name": "Kinetic Pulse",
    "hero_key": "hero_dynamo",
    "hero_name": "Dynamo",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_dynamo",
      "hero_name": "Dynamo",
      "lookup": "kinetic pulse",
      "name": "Kinetic Pulse",
      "type": "ability"
    }
  ]
}
````
