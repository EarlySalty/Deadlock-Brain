---
title: "Spirit Snare"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_immobilize_trap"
canonical_name: "Spirit Snare"
snapshot_id: 39465
source_document_id: 7070
payload_hash: "91f36dddec5db0b906d33ff9d80309d883833bb0d45bb4eb85a71f5386c5c0c0"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.394782+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Spirit Snare

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_immobilize_trap`
- Snapshot ID: `39465`
- Source-Dokument: `7070`
- Kurzinfo: Spirit Snare aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 34,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "ArmTime": 2.0,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventTrainingBotUsage",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "ChargedShotHitRadiusScale": 30,
  "Damage": 25,
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "ImmobilizeTrapDebuff"
  },
  "GlitchModifier": {
    "Class": "GlitchDebuff",
    "EnabledStateMask": [
      "Disarmed",
      "Silenced",
      "Muted",
      "Glitched"
    ],
    "Subclass": "GlitchDebuff"
  },
  "IsDisabled": false,
  "Key": "ability_immobilize_trap",
  "Lifetime": 22,
  "Name": "Spirit Snare",
  "Radius": 6.5,
  "SkipFrames": 6,
  "SlowPercent": 30,
  "TetherDuration": 2.25,
  "TetherRadius": 6,
  "TrapHeight": 2,
  "TripGravity": 0.4,
  "TripTime": 0.5,
  "TripUpSpeed": 6.35,
  "Upgrades": [
    {
      "AbilityCooldown": -20.0
    },
    {
      "BulletArmorReduction": -15,
      "DebuffDuration": 10
    },
    {
      "Radius": 1.5,
      "TetherDuration": 1
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
    "card_name": "Spirit Snare",
    "hero_key": "hero_orion",
    "hero_name": "Grey Talon",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_orion",
      "hero_name": "Grey Talon",
      "lookup": "spirit snare",
      "name": "Spirit Snare",
      "type": "ability"
    }
  ]
}
````
