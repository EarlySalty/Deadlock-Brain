---
title: "Chain Gang"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_punkgoat_tether"
canonical_name: "Chain Gang"
snapshot_id: 39522
source_document_id: 7070
payload_hash: "fddec94b89f33da6ec5c00f69b1a7422ba44f0b5d52a0c572ca96a4a29df1c94"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.537504+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Chain Gang

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_punkgoat_tether`
- Snapshot ID: `39522`
- Source-Dokument: `7070`
- Kurzinfo: Chain Gang aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 12.0,
  "AbilityCooldown": 175,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 2.8,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorSilentCastFailureFeedback",
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorCanSetQuickCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.9
    },
    "Value": 45
  },
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 120
  },
  "DamageIncreasePct": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "FireRateSlowModifier": {
    "Class": "Base",
    "Subclass": "TetherFirerateSlow"
  },
  "IsDisabled": false,
  "Key": "ability_punkgoat_tether",
  "MoveSpeedSlowMaxPct": 35,
  "MoveSpeedSlowMinPct": 25,
  "Name": "Chain Gang",
  "PullDistance": 4,
  "PullDuration": 0.8,
  "PullForceMax": 2000,
  "PullModifier": {
    "Class": "PunkgoatTetherPull",
    "Subclass": "Pull"
  },
  "PullTrackCasterDuration": 0.5,
  "RopeLength": 2.0,
  "RopeSnapDistance": 45.0,
  "RopeSnapNoLOSDuration": 0.5,
  "RopeSoftEdgeLength": 4.5,
  "TetheredModifier": {
    "Class": "PunkgoatTether",
    "Subclass": "Tether"
  },
  "TickRate": 0.25,
  "UnstoppableModifier": {
    "Class": "Unstoppable",
    "DisabledStateMask": [
      "Disarmed",
      "Muted",
      "Silenced",
      "SilenceMovementAbilites",
      "Slowed",
      "Glitched",
      "MeleeDisabledDebuff",
      "DashDisabledDebuff"
    ],
    "EnabledStateMask": [
      "StatusImmune",
      "SlowImmune",
      "KnockdownImmune",
      "Unstoppable"
    ],
    "StatusEffectPriority": 25,
    "Subclass": "Unstoppable"
  },
  "Upgrades": [
    {
      "BulletResist": 40,
      "TechResist": 40
    },
    {
      "AbilityCooldown": -40
    },
    {
      "AbilityCastRange": 5,
      "UnstoppablePerHero": 1.3
    }
  ],
  "WaitingToPullModifier": {
    "Class": "PunkgoatTetherWaiting",
    "Subclass": "WaitingForPull"
  },
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
    "card_name": "Chain Gang",
    "hero_key": "hero_punkgoat",
    "hero_name": "Billy",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_punkgoat",
      "hero_name": "Billy",
      "lookup": "chain gang",
      "name": "Chain Gang",
      "type": "ability"
    }
  ]
}
````
