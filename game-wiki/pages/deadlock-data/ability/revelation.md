---
title: "Revelation"
entity_type: "ability"
source: "deadlock_data"
external_id: "operative_revelation"
canonical_name: "Revelation"
snapshot_id: 39699
source_document_id: 7070
payload_hash: "7104705227904cb4e883ec000562bdb835a20cc4795c1c6d3411923067f013c1"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.983078+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Revelation

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `operative_revelation`
- Snapshot ID: `39699`
- Source-Dokument: `7070`
- Kurzinfo: Revelation aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.3,
  "AbilityCooldown": 90.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "CasterModifier": {
    "AuraModifier": {
      "Class": "OperativeRevelationAura",
      "ProvidedByAura": {
        "Class": "OperativeRevelationTarget",
        "DebuffModifier": {
          "Class": "GlitchDebuff",
          "EnabledStateMask": [
            "Disarmed",
            "Silenced",
            "Muted",
            "Glitched"
          ],
          "Subclass": "GlitchDebuff"
        },
        "Subclass": "OperativeRevelationTarget"
      },
      "Subclass": "OperativeRevelationAura"
    },
    "Class": "OperativeRevelationCaster",
    "StatusEffectPriority": 25,
    "Subclass": "OperativeRevelationCaster"
  },
  "ChannelMoveSpeed": 4.2,
  "CurseDuration": 3,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.651
    },
    "Value": 50
  },
  "GroundDashReductionPercent": -20,
  "IsDisabled": false,
  "Key": "operative_revelation",
  "MaxCameraAngleForSeeing": 180,
  "MoveSpeedReduction": 20,
  "Name": "Revelation",
  "Radius": 15,
  "SlowPercent": 25,
  "TickRate": 0.25,
  "TimeBeforeCursed": 2,
  "Upgrades": [
    {
      "Radius": 5
    },
    {
      "AbilityCooldown": -25.0
    },
    {
      "DPS": 50
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
    "card_name": "Revelation",
    "hero_key": "hero_operative",
    "hero_name": "Raven",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_operative",
      "hero_name": "Raven",
      "lookup": "revelation",
      "name": "Revelation",
      "type": "ability"
    }
  ]
}
````
