---
title: "Flying Slash"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_flying_strike"
canonical_name: "Flying Slash"
snapshot_id: 39612
source_document_id: 7070
payload_hash: "909e91583d47770738e48534af8b0bf514ef85e82a76c0eb956876d2db9cd8ef"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.773579+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Flying Slash

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_flying_strike`
- Snapshot ID: `39612`
- Source-Dokument: `7070`
- Kurzinfo: Flying Slash aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 28,
  "AbilityCooldown": 36.0,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorMovement",
    "BehaviorCanSetQuickCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "SpiritBuff"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "melee",
      "Value": 1.0
    },
    "Value": 0
  },
  "GrappleTargetModifier": {
    "Class": "CitadelFlyingStrike",
    "Subclass": "Target"
  },
  "IsDisabled": false,
  "Key": "citadel_ability_flying_strike",
  "Name": "Flying Slash",
  "SlowDuration": 2.5,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "SlowPercent": 50,
  "Upgrades": [
    {
      "AbilityCooldown": -18
    },
    {
      "BuffDuration": 6,
      "SpiritBonus": 40
    },
    {
      "AbilityCastRange": 15,
      "AbilityCharges": 3,
      "AbilityCooldownBetweenCharge": 3,
      "CanGrappleAllyHeroes": 1
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
    "card_name": "Flying Slash",
    "hero_key": "hero_yamato",
    "hero_name": "Yamato",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_yamato",
      "hero_name": "Yamato",
      "lookup": "flying slash",
      "name": "Flying Slash",
      "type": "ability"
    }
  ]
}
````
