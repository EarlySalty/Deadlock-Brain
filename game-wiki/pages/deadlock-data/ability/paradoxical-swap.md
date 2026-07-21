---
title: "Paradoxical Swap"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_chrono_swap"
canonical_name: "Paradoxical Swap"
snapshot_id: 39605
source_document_id: 7070
payload_hash: "7648af4a04ea7c4d03456c3e8537397243fecd1f482e5770efb8cf55215a387d"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.756042+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Paradoxical Swap

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chrono_swap`
- Snapshot ID: `39605`
- Source-Dokument: `7070`
- Kurzinfo: Paradoxical Swap aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 25,
  "AbilityCooldown": 110.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorCleaveDisabled",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorAllowAltCast"
  ],
  "BubbleMoveModifier": {
    "Class": "ChronoSwapBubbleMove",
    "EnabledStateMask": [
      "ChronoSwapping",
      "CommandRestricted",
      "AirDuckingForced",
      "SilenceMovementAbilites"
    ],
    "MultiSwapDistFromOrigin": 80.0,
    "Subclass": "ChronoSwapBubbleMove"
  },
  "ChannelMoveSpeed": 1.3,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "DistanceToMaxTime": 30,
  "InitialFreezeTime": 0.25,
  "InitialHeight": 350,
  "IsDisabled": false,
  "Key": "citadel_ability_chrono_swap",
  "MinSwapTime": 0.6,
  "Name": "Paradoxical Swap",
  "ShieldModifier": {
    "Class": "Base",
    "Subclass": "BarrierModifier"
  },
  "SwapDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.1
    },
    "Value": 150.0
  },
  "SwapTime": 1.0,
  "TickRate": 0.25,
  "Upgrades": [
    {
      "BarrierDuration": 8,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.5
        },
        "Value": 200
      }
    },
    {
      "AbilityCastRange": 13,
      "AbilityCooldown": -30
    },
    {
      "MaxHealthDamage": 10,
      "MultiSwap": 7
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
    "card_name": "Paradoxical Swap",
    "hero_key": "hero_chrono",
    "hero_name": "Paradox",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_chrono",
      "hero_name": "Paradox",
      "lookup": "paradoxical swap",
      "name": "Paradoxical Swap",
      "type": "ability"
    }
  ]
}
````
