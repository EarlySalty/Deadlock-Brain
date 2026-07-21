---
title: "Crow Familiar"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_hornet_sting"
canonical_name: "Crow Familiar"
snapshot_id: 39618
source_document_id: 7070
payload_hash: "6df0190ea15ea3600ea6d466ca6d7233f16bb5bdfb2737c0498afc24a827648b"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.787014+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Crow Familiar

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hornet_sting`
- Snapshot ID: `39618`
- Source-Dokument: `7070`
- Kurzinfo: Crow Familiar aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 32.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorCleaveDisabled",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BulletResistReduction": -6,
  "ChannelMoveSpeed": -1,
  "DebuffDuration": 5,
  "DebuffModifier": {
    "Class": "CitadelHornetStingDebuff",
    "Subclass": "CitadelHornetStingDebuff"
  },
  "DotHealthPercent": 2.2,
  "ImpactDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.744
    },
    "Value": 40
  },
  "IsDisabled": false,
  "Key": "citadel_ability_hornet_sting",
  "Name": "Crow Familiar",
  "TechArmorDamageReduction": -6,
  "TickRate": 1.0,
  "Upgrades": [
    {
      "HealAmpReceivePenaltyPercent": -35,
      "HealAmpRegenPenaltyPercent": -35
    },
    {
      "AbilityCooldown": -16.0,
      "DotHealthPercent": 0.5
    },
    {
      "BulletResistReduction": -8,
      "DebuffDuration": 2,
      "TechArmorDamageReduction": -8
    }
  ],
  "VisualSplashRadius": 4,
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
    "card_name": "Crow Familiar",
    "hero_key": "hero_hornet",
    "hero_name": "Vindicta",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_hornet",
      "hero_name": "Vindicta",
      "lookup": "crow familiar",
      "name": "Crow Familiar",
      "type": "ability"
    }
  ]
}
````
