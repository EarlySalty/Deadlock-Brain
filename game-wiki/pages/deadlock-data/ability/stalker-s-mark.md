---
title: "Stalker's Mark"
entity_type: "ability"
source: "deadlock_data"
external_id: "drifter_shadow_mark"
canonical_name: "Stalker's Mark"
snapshot_id: 39673
source_document_id: 7070
payload_hash: "f26c21a8a580570440b27ac42774d8320fd3d6e04fa0e61ac3bd96f10fb77c02"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.915696+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Stalker's Mark

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `drifter_shadow_mark`
- Snapshot ID: `39673`
- Source-Dokument: `7070`
- Kurzinfo: Stalker's Mark aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCooldown": 20,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "AirDrag": 3,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCleaveDisabled",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "DrifterStalkersMarkBuff"
  },
  "ChannelMoveSpeed": -1,
  "DotHealthPercent": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.015
    },
    "Value": 2.0
  },
  "FallSpeedMax": 0.3,
  "IsDisabled": false,
  "Key": "drifter_shadow_mark",
  "Name": "Stalker's Mark",
  "PostTeleportModifier": {
    "Class": "DrifterStalkersMarkPostTeleport",
    "Subclass": "DrifterStalkersMarkPostTeleport"
  },
  "TargetModifier": {
    "Class": "DrifterShadowMarkTarget",
    "Subclass": "DrifterShadowMarkTarget"
  },
  "TargetTeleportModifier": {
    "Class": "Base",
    "Subclass": "DrifterShadowMarkTeleportTarget"
  },
  "TeleportBackOffsetFromTarget": 135,
  "TeleportDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.5
    },
    "Value": 0
  },
  "TickRate": 0.5,
  "Upgrades": [
    {
      "BulletResistReduction": -8
    },
    {
      "AbilityCooldown": -8,
      "AbilityDuration": 3
    },
    {
      "DotHealthPercent": 2.0,
      "HealAmpReceivePenaltyPercent": -40,
      "HealAmpRegenPenaltyPercent": -40
    }
  ],
  "VerticalDrag": 1,
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
    "card_name": "Stalker's Mark",
    "hero_key": "hero_drifter",
    "hero_name": "Drifter",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_drifter",
      "hero_name": "Drifter",
      "lookup": "stalker's mark",
      "name": "Stalker's Mark",
      "type": "ability"
    }
  ]
}
````
