---
title: "Smoke Bomb"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_smoke_bomb"
canonical_name: "Smoke Bomb"
snapshot_id: 39535
source_document_id: 7070
payload_hash: "62617c5dbd53ab41dc10a3018dd97d1f5e5db3152a120fa440b046f962ab2f00"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.568229+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Smoke Bomb

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_smoke_bomb`
- Snapshot ID: `39535`
- Source-Dokument: `7070`
- Kurzinfo: Smoke Bomb aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 33.0,
  "AbilityDuration": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.1
    },
    "Value": 8
  },
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDamageDoesntWakeFromSleep",
    "BehaviorNoTarget",
    "BehaviorDontInterruptSprint",
    "BehaviorCleaveDisabled",
    "BehaviorCanCastOnZipline"
  ],
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "BuffModifier"
  },
  "ChannelMoveSpeed": -1,
  "FullInvisDistance": 50,
  "InvisAlertWhenFading": 1,
  "InvisFadeToDuration": 1.5,
  "InvisModifier": {
    "Class": "Invis",
    "Subclass": "SmokebombModifierInvis"
  },
  "IsDisabled": false,
  "Key": "ability_smoke_bomb",
  "Name": "Smoke Bomb",
  "PhaseOutModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "Invulnerable"
    ],
    "Subclass": "PhaseoutModifier"
  },
  "RevealOnDamageDuration": 1.5,
  "RevealOnSpottedDuration": 0.5,
  "SpottedRadius": 18,
  "Upgrades": [
    {
      "InvisMoveSpeedMod": 7
    },
    {
      "AbilityCharges": 2,
      "AbilityCooldownBetweenCharge": 7
    },
    {
      "BulletLifesteal": 50,
      "DispelOnUse": 1,
      "PostInvisBuffDuration": 5
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
    "card_name": "Smoke Bomb",
    "hero_key": "hero_haze",
    "hero_name": "Haze",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_haze",
      "hero_name": "Haze",
      "lookup": "smoke bomb",
      "name": "Smoke Bomb",
      "type": "ability"
    }
  ]
}
````
