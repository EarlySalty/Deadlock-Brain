---
title: "Eternal Night"
entity_type: "ability"
source: "deadlock_data"
external_id: "drifter_darkness"
canonical_name: "Eternal Night"
snapshot_id: 39672
source_document_id: 7070
payload_hash: "d89e088937d2c9d66bbed27186496b9106c87f5d39d562385dd637196f7fe6c2"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.913242+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Eternal Night

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `drifter_darkness`
- Snapshot ID: `39672`
- Source-Dokument: `7070`
- Kurzinfo: Eternal Night aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1.0,
  "AbilityCastRange": 100,
  "AbilityCooldown": 145.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6.5,
  "AbilityUnitTargetLimit": 1,
  "AuraLingerDuration": 0.001,
  "BehaviourBits": [
    "BehaviorProjectilePassThroughWorld",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusFireRate": {
    "Scale": {
      "Type": "spirit",
      "Value": 0
    },
    "Value": 0
  },
  "BonusSprintAcceleration": 12,
  "BonusSprintSpeed": 2,
  "CasterModifier": {
    "Class": "DrifterDarknessCaster",
    "EnabledStateMask": [
      "SinclairTaxUltActive"
    ],
    "Subclass": "DrifterDarknessCaster"
  },
  "ChannelMoveSpeed": 1.3,
  "DarkFactor": 1.0,
  "DistanceForMaxProjSpeed": 200,
  "DrifterNearbyRangeCheck": 40,
  "IsDisabled": false,
  "Key": "drifter_darkness",
  "MaxProjectileSpeed": 3000,
  "MaxTargets": 2,
  "MinProjectileSpeed": 3000,
  "Name": "Eternal Night",
  "PostProcessFadeInTime": 0.2,
  "PostProcessFadeOutTime": 1.0,
  "RevealDuration": 3,
  "SmallVisionDistance": 15,
  "TargetModifier": {
    "Class": "DrifterDarknessTarget",
    "ProvidedByAura": {
      "Class": "DrifterDarknessTargetBoundaryUnit",
      "Subclass": "DrifterDarknessTargetBoundaryUnit"
    },
    "Subclass": "DrifterDarknessTarget"
  },
  "TargetRevealModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "VisibleToEnemy",
      "GlowThroughWallsToProvider"
    ],
    "Subclass": "DrifterDarknessTargetReveal"
  },
  "Upgrades": [
    {
      "BonusSprintSpeed": 10
    },
    {
      "AbilityCooldown": -40
    },
    {
      "AbilityDuration": 2.5,
      "MaxTargets": 1
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
    "card_name": "Eternal Night",
    "hero_key": "hero_drifter",
    "hero_name": "Drifter",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_drifter",
      "hero_name": "Drifter",
      "lookup": "eternal night",
      "name": "Eternal Night",
      "type": "ability"
    }
  ]
}
````
