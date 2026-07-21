---
title: "Escape Artist"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_magician_escape"
canonical_name: "Escape Artist"
snapshot_id: 39478
source_document_id: 7070
payload_hash: "c4e04d6323632ad248b95369eec05ecd0f116e6230662423e3445cedbf290cce"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.436764+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Escape Artist

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_escape`
- Snapshot ID: `39478`
- Source-Dokument: `7070`
- Kurzinfo: Escape Artist aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 15,
  "AbilityCooldown": 30,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorPreventBotUsage",
    "BehaviorAllowAltCast",
    "BehaviorMovement",
    "BehaviorCanSetQuickCast"
  ],
  "BuffDuration": 2,
  "ChannelMoveSpeed": -1,
  "EscapedModifier": {
    "Class": "Invis",
    "EnabledStateMask": [
      "Sprinting"
    ],
    "Subclass": "EscapeEscapedModifier"
  },
  "FullInvisDistance": 50,
  "InvisAlertWhenFading": 1,
  "InvisFadeToDuration": 0.5,
  "InvisMoveSpeedMod": 2.0,
  "InvisibilityDuration": 2,
  "IsDisabled": false,
  "Key": "ability_magician_escape",
  "Name": "Escape Artist",
  "RevealOnSpottedDuration": 0.5,
  "SpottedRadius": 2,
  "Upgrades": [
    {
      "StaminaHeal": 2
    },
    {
      "AbilityCooldown": -15
    },
    {
      "InvisibilityDuration": 2
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
  }
}
````
