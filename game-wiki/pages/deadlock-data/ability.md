---
title: "ability"
entries: 295
---

# ability

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_swan_acrobat" title="Acrobat" -->

## Acrobat

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_swan_acrobat`
- Snapshot ID: `39537`
- Source-Dokument: `7070`
- Kurzinfo: Acrobat aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Acrobat`
- Payload Hash: `e09e8e6e96ecf48fa0dd29342e0ade9206fa90ca1470cbbdec6a2f44dfc0a998`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.573224+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 7,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "BurstBonusPerStack": 1,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_swan_acrobat",
  "MaxStacks": 4,
  "Name": "Acrobat",
  "StackingModifier": {
    "Class": "SwanAcrobat",
    "Subclass": "Stack"
  },
  "Upgrades": [
    {
      "AbilityDuration": 3
    },
    {
      "FireRatePerStack": 4
    },
    {
      "MaxStacks": 4
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
    "card_name": "Acrobat",
    "hero_key": "hero_swan",
    "hero_name": "Swan",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_swan",
      "hero_name": "Swan",
      "lookup": "acrobat",
      "name": "Acrobat",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="synth_affliction" title="Affliction" -->

## Affliction

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `synth_affliction`
- Snapshot ID: `39708`
- Source-Dokument: `7070`
- Kurzinfo: Affliction aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Affliction`
- Payload Hash: `2027406db26dda1635b9d0af4e5a731a3fc413f44c45c335980e933b50640433`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.009634+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.6,
  "AbilityCooldown": 170.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "CanBePurged": 1,
  "ChannelMoveSpeed": 1.3,
  "CurrentHealthDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.22
    },
    "Value": 34
  },
  "DamageInterval": 0.5,
  "DebuffDuration": 11,
  "DebuffModifier": {
    "Class": "SynthAfflictionDebuff",
    "Subclass": "SynthAfflictionDebuff"
  },
  "IsDisabled": false,
  "Key": "synth_affliction",
  "Name": "Affliction",
  "Radius": 9,
  "Upgrades": [
    {
      "AbilityCooldown": -35
    },
    {
      "DebuffDuration": 3,
      "Radius": 4
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.11
        },
        "Value": 14
      },
      "DisableHealing": 1
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
    "card_name": "Affliction",
    "hero_key": "hero_synth",
    "hero_name": "Pocket",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_synth",
      "hero_name": "Pocket",
      "lookup": "affliction",
      "name": "Affliction",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_afterburn" title="Afterburn" -->

## Afterburn

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_afterburn`
- Snapshot ID: `39392`
- Source-Dokument: `7070`
- Kurzinfo: Afterburn aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Afterburn`
- Payload Hash: `4f48e675969b2b2c9453e4e1daabcfd27679c41c85e865172be48dfe780931ad`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.208391+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "AfterburnDotModifier": {
        "Class": "AfterburnDot",
        "EnabledStateMask": [
          "Afterburning"
        ],
        "StatusEffectPriority": 50,
        "Subclass": "AfterburnDot"
      },
      "BuildUpModifier": {
        "Class": "CitadelBaseBuildup",
        "Subclass": "CitadelBaseBuildup"
      },
      "Class": "AfterburnWatcher",
      "HeavyMeleeBuildUp": 35,
      "HeavyMeleeRefresh": 3.0,
      "LightMeleeBuildUp": 20,
      "LightMeleeRefresh": 1.5,
      "Subclass": "AfterburnWatcher"
    }
  ],
  "BehaviourBits": [
    "BehaviorCleaveDisabled",
    "BehaviorDisplaysDamageImpact"
  ],
  "BuildUpBulletPercentPerHit": 8.1,
  "BuildUpDuration": 17,
  "BurnDuration": 3,
  "BurnDurationBase": 3,
  "ChannelMoveSpeed": -1,
  "CritBuildup": 15.4,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.66
    },
    "Value": 14.0
  },
  "IsDisabled": false,
  "Key": "ability_afterburn",
  "Name": "Afterburn",
  "RefillDuration": 0.5,
  "RefillDurationCrit": 1.0,
  "TickRate": 0.5,
  "Upgrades": [
    {
      "DPS": 16
    },
    {
      "OutgoingTechDamagePercent": -35
    },
    {
      "BurnDuration": 3
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
    "card_name": "Afterburn",
    "hero_key": "hero_inferno",
    "hero_name": "Infernus",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_inferno",
      "hero_name": "Infernus",
      "lookup": "afterburn",
      "name": "Afterburn",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_tengu_airlift" title="Air Drop" -->

## Air Drop

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tengu_airlift`
- Snapshot ID: `39652`
- Source-Dokument: `7070`
- Kurzinfo: Air Drop aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Air Drop`
- Payload Hash: `8f7829cc86d23692abde3eaae3f643fd8989dd8cddac201dbd618a8a3dc78f18`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.867495+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 22,
  "AbilityCooldown": 100.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 21.0,
  "AbilityUnitTargetLimit": 1,
  "AirDropBulletShield": {
    "Scale": {
      "Type": "spirit",
      "Value": 0
    },
    "Value": 0
  },
  "AirDropOutgoingDamagePercent": 20,
  "AllyCastDelay": 0.1,
  "AllyOutgoingDamagePercent": -20,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorAllowSelfCast",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorMovement",
    "BehaviorRequireAbilityButtonToCancel",
    "BehaviorCanSetQuickCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BuffDuration": 8,
  "BulletResistModifier": {
    "Class": "Base",
    "Subclass": "AirdropBulletresist"
  },
  "ChannelMoveSpeed": 1.3,
  "CooldownReductionPctOnOthers": 30,
  "DroppedBuffModifier": {
    "Class": "Base",
    "Subclass": "BarrierModifier"
  },
  "ExplodeDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.7
    },
    "Value": 115.0
  },
  "ExplodingAllyModifier": {
    "Class": "AirliftExplodingAlly",
    "Subclass": "AirliftExplodingAlly"
  },
  "FlyingModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "AbilityMovement",
      "Sprinting",
      "DashDisabled",
      "ShootingDisabled",
      "JumpDisabled",
      "DuckingDisabled",
      "MantleDisabled",
      "AimForwardWithPitch",
      "MeleeDisabled",
      "SlidingDisabled",
      "HideCrosshair"
    ],
    "Subclass": "Base"
  },
  "GrabModifier": {
    "AllyGrabCancelTime": 1.0,
    "AllyPossibleStuckDistance": 320,
    "Class": "AirliftGrab",
    "EnabledStateMask": [
      "AbilityMovement",
      "ShootingDisabled",
      "MeleeDisabled",
      "IgnorePortals"
    ],
    "FollowDampingFactor": 20,
    "FollowDistance": -60,
    "LiftHeight": -120,
    "LiftHorizontal": 0,
    "Subclass": "AirliftGrab"
  },
  "InterruptCooldown": 3.5,
  "IsDisabled": false,
  "Key": "citadel_ability_tengu_airlift",
  "Name": "Air Drop",
  "OnLandDamageRadius": 20,
  "OnLandDamageRadiusStart": 16,
  "SilenceBombSpeed": 12,
  "SilenceModifier": {
    "Class": "CitadelSilenced",
    "Subclass": "CitadelSilenced"
  },
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "AirliftSlow"
  },
  "Upgrades": [
    {
      "AirDropBulletShield": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.7
        },
        "Value": 300
      }
    },
    {
      "DebuffDuration": 3,
      "SlowPercent": 40
    },
    {
      "AirDropBulletShield": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 0
      },
      "ExplodeDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.5
        },
        "Value": 0
      },
      "SilenceDuration": 3
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
    "card_name": "Air Drop",
    "hero_key": "hero_tengu",
    "hero_name": "Ivy",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_tengu",
      "hero_name": "Ivy",
      "lookup": "air drop",
      "name": "Air Drop",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_warden_crowd_control" title="Alchemical Flask" -->

## Alchemical Flask

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_warden_crowd_control`
- Snapshot ID: `39569`
- Source-Dokument: `7070`
- Kurzinfo: Alchemical Flask aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Alchemical Flask`
- Payload Hash: `06a8c418a40429e6f670930a79af1c7e7a14bcaa36fccaa2fe1bd317624e1e8e`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.660418+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCooldown": 12.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.63
    },
    "Value": 60
  },
  "DebuffDuration": 7,
  "DebuffModifier": {
    "Class": "WardenCrowdControlDebuff",
    "EnabledStateMask": [
      "Slowed"
    ],
    "Subclass": "WardenCrowdControlDebuff"
  },
  "ForwardVelocity": 800,
  "IsDisabled": false,
  "Key": "ability_warden_crowd_control",
  "MoveSpeedSlowPct": 20,
  "Name": "Alchemical Flask",
  "ProjectileLifetime": 60,
  "Radius": 5.5,
  "SlowDuration": 3,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "Upgrades": [
    {
      "StaminaReduction": 1
    },
    {
      "Damage": 35,
      "WeaponPowerDebuff": -25
    },
    {
      "AbilityCooldown": -7,
      "FireRateSlow": 30,
      "Radius": 2
    }
  ],
  "WeaponPowerDebuff": -25,
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
    "card_name": "Alchemical Flask",
    "hero_key": "hero_warden",
    "hero_name": "Warden",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_warden",
      "hero_name": "Warden",
      "lookup": "alchemical flask",
      "name": "Alchemical Flask",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="drifter_shadow_mark_teleport" title="Ambush" -->

## Ambush

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `drifter_shadow_mark_teleport`
- Snapshot ID: `39674`
- Source-Dokument: `7070`
- Kurzinfo: Ambush aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ambush`
- Payload Hash: `e9e00832c51ee274ec943a0e2c57a8737cc063ec5ae32d22bd0f64cfe59a9c51`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.918018+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.3,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontInterruptSprint",
    "BehaviorNoTarget",
    "BehaviorCastableWhileHidden",
    "BehaviorIgnoreSelectionMashProtection",
    "BehaviorTrigger",
    "BehaviorMovement"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "drifter_shadow_mark_teleport",
  "Name": "Ambush",
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="cadence_ability_anthem" title="Anthem" -->

## Anthem

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `cadence_ability_anthem`
- Snapshot ID: `39591`
- Source-Dokument: `7070`
- Kurzinfo: Anthem aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Anthem`
- Payload Hash: `0158944c8a0247d783740b92adff2255ea78b2e77388295273e2448d8595d52c`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.714245+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.75,
  "AbilityCooldown": 37.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 1,
  "AnthemAOEModifier": {
    "Class": "CadenceAnthemAoe",
    "ProvidedByAura": {
      "Class": "CadenceAnthemBuff",
      "Subclass": "CadenceAnthemBuff"
    },
    "Subclass": "CadenceAnthemAoe"
  },
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": 1.3,
  "ExtraLargeClip": 25,
  "IsDisabled": false,
  "Key": "cadence_ability_anthem",
  "LingerDuration": 0.5,
  "Name": "Anthem",
  "PeakFireRateBonus": 100,
  "Radius": 12,
  "Upgrades": [
    {
      "ExtraLargeClip": 75
    },
    {
      "Radius": 4
    },
    {
      "PeakFireRateBonus": 100
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_cadence",
      "hero_name": "Cadence",
      "lookup": "anthem",
      "name": "Anthem",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_icebeam" title="Arctic Beam" -->

## Arctic Beam

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_icebeam`
- Snapshot ID: `39463`
- Source-Dokument: `7070`
- Kurzinfo: Arctic Beam aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Arctic Beam`
- Payload Hash: `e77fe031c665eca836afb044961b1e85ce9090bd1c5547911f383a0870e9edf4`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.389298+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 28.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5.0,
  "AbilityUnitTargetLimit": 1,
  "BeamSplit": {
    "Scale": {
      "Type": "range",
      "Value": 0.0
    },
    "Value": 0
  },
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDontAimFacingEnemy",
    "BehaviorRequireAbilityButtonToCancel"
  ],
  "BuildupModifier": {
    "BuildUpDecayDelay": 0.4,
    "Class": "IcebeamStackingSlow",
    "EnabledStateMask": [
      "Slowed"
    ],
    "StatusEffectPriority": 50,
    "Subclass": "IcebeamStackingSlow"
  },
  "CameraDistance": 250,
  "ChannelMoveSpeed": -1,
  "ChannelSlowPercent": 8,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.38
    },
    "Value": 45.0
  },
  "IceBeamBuildupProcDuration": 2,
  "IceBeamModifier": {
    "Class": "Base",
    "Subclass": "Icebeaming"
  },
  "IsDisabled": false,
  "Key": "ability_icebeam",
  "MaxFireRateSlowPercent": 20,
  "MaxGroundDashReductionPercent": -20,
  "MaxSlowPercent": 20,
  "MaxSlowTime": 2.0,
  "MinSlowPercent": 30,
  "Name": "Arctic Beam",
  "PathLength": 25,
  "PathWidth": 1.1,
  "SlowDuration": 0.6,
  "TickRate": 0.1,
  "Upgrades": [
    {
      "MaxFireRateSlowPercent": 25,
      "MaxSlowPercent": 25
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 20
      }
    },
    {
      "AbilityCooldown": -13,
      "BeamSplit": {
        "Scale": {
          "Type": "range",
          "Value": 0.93
        },
        "Value": 10
      },
      "BeamSplitCount": 2
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
    "card_name": "Arctic Beam",
    "hero_key": "hero_kelvin",
    "hero_name": "Kelvin",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_kelvin",
      "hero_name": "Kelvin",
      "lookup": "arctic beam",
      "name": "Arctic Beam",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_hornet_snipe" title="Assassinate" -->

## Assassinate

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hornet_snipe`
- Snapshot ID: `39617`
- Source-Dokument: `7070`
- Kurzinfo: Assassinate aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Assassinate`
- Payload Hash: `5e3c651ee71c60c7caa58cac0b52332ee05e05a989759edde56bacecfcb32db6`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.785026+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCharges": 2,
  "AbilityCooldown": 55.0,
  "AbilityCooldownBetweenCharge": 2.5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusGoldOnKill": 250,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.93
    },
    "Value": 90
  },
  "GlowEnemyModifier": {
    "Class": "LowHealthGlow",
    "EnabledStateMask": [
      "AssassinateLowhealthTarget"
    ],
    "Subclass": "LowHealthGlow"
  },
  "HeadshotBonus": 20,
  "IsDisabled": false,
  "Key": "citadel_ability_hornet_snipe",
  "KillCheckModifier": {
    "Class": "Base",
    "Subclass": "KillcheckModifier"
  },
  "LowHealthEnemyDamageBonus": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.3
    },
    "Value": 90
  },
  "LowHealthEnemyThresholdPct": 50,
  "MaxSoundDistance": 2000,
  "MinChargeDamagePercent": 50,
  "MoveSpeed": 4,
  "Name": "Assassinate",
  "Range": 1000,
  "ShotRadius": 4.0,
  "SnipeModifier": {
    "Class": "CitadelHornetSnipe",
    "Subclass": "CitadelHornetSnipe"
  },
  "TimeToFullCharge": 1.0,
  "Upgrades": [
    {
      "AbilityCooldown": -15.0
    },
    {
      "LowHealthEnemyDamageBonus": 80
    },
    {
      "WeaponDamageBonusPerKill": 4
    }
  ],
  "ViewPunch": 2.5,
  "WeaponDamageBonusPerKill": 6,
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
    "card_name": "Assassinate",
    "hero_key": "hero_hornet",
    "hero_name": "Vindicta",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_hornet",
      "hero_name": "Vindicta",
      "lookup": "assassinate",
      "name": "Assassinate",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_wrecker_teleport" title="Astral Walk" -->

## Astral Walk

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_wrecker_teleport`
- Snapshot ID: `39587`
- Source-Dokument: `7070`
- Kurzinfo: Astral Walk aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Astral Walk`
- Payload Hash: `8464e77073bdceccc401fa3082bdd1221d1771aac93468490664b23fefec425b`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.704085+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1.5,
  "AbilityChannelTime": 8,
  "AbilityCooldown": 138.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "DamagePerSecondFlown": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.487469
    },
    "Value": 16
  },
  "DebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "SlowBase"
  },
  "EnemyMoveSlowDuration": 1,
  "EnemySlowPct": 60,
  "ExplosionRadius": 8,
  "GuidingModifier": {
    "Class": "Base",
    "Subclass": "GuidingModifier"
  },
  "IsDisabled": false,
  "Key": "ability_wrecker_teleport",
  "Name": "Astral Walk",
  "Upgrades": [
    {
      "AbilityChannelTime": 8
    },
    {
      "AbilityCooldown": -47.0
    },
    {
      "DamagePerSecondFlown": 16
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_magician_copyult" title="Audience Participation" -->

## Audience Participation

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_copyult`
- Snapshot ID: `39477`
- Source-Dokument: `7070`
- Kurzinfo: Audience Participation aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Audience Participation`
- Payload Hash: `dc07a1c5487aac5d344af9781efa4d43d6bbec568b40714b3416d987d7821d52`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.433784+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 20,
  "AbilityCooldown": 85,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "CopiedUltSpawnedEntityModifier": {
    "Class": "CitadelCopiedultSpawnedentity",
    "EnabledStateMask": [
      "MaterialOverride"
    ],
    "Subclass": "CopiedultSpawnedentity"
  },
  "CopiedUltWindow": 12,
  "CopyCooldownPercentage": 40,
  "CopyInternalCooldown": 0.5,
  "InformTargetUltCopiedModifier": {
    "Class": "Base",
    "Subclass": "CopyultInformtargetultcopied"
  },
  "IsDisabled": false,
  "Key": "ability_magician_copyult",
  "Name": "Audience Participation",
  "UltActiveModifier": {
    "Class": "CitadelCopyult",
    "Subclass": "CopyultModifier"
  },
  "UltCopiedModifier": {
    "Class": "CitadelCopyultpending",
    "Subclass": "Ultcopied"
  },
  "Upgrades": [
    {},
    {},
    {}
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
    "card_name": "Audience Participation",
    "hero_key": "hero_magician",
    "hero_name": "Sinclair",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_magician",
      "hero_name": "Sinclair",
      "lookup": "audience participation",
      "name": "Audience Participation",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_frank_painaura" title="Aura of Suffering" -->

## Aura of Suffering

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_frank_painaura`
- Snapshot ID: `39443`
- Source-Dokument: `7070`
- Kurzinfo: Aura of Suffering aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Aura of Suffering`
- Payload Hash: `faf001b3eccb74e2aa34a56cea5761bd84f6130869bbf9bd1812bec7f3c7f8a2`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.336865+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 2.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 1,
  "AuraModifier": {
    "Class": "FrankPainaura",
    "DebuffModifier": {
      "Class": "FrankPainauraTarget",
      "Subclass": "Painauradebuff"
    },
    "Subclass": "Painaura"
  },
  "AuraOffModifier": {
    "Class": "Base",
    "Subclass": "Auraoff"
  },
  "BehaviourBits": [
    "BehaviorStartCooldownOnToggleOff",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDoNotAllowSpamProc",
    "BehaviorCanCastOnZipline"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.1
    },
    "Value": 0
  },
  "DebuffDuration": 0.5,
  "IsDisabled": false,
  "Key": "ability_frank_painaura",
  "MaxDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.72
    },
    "Value": 58
  },
  "MinDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.15
    },
    "Value": 13
  },
  "Name": "Aura of Suffering",
  "Radius": 8,
  "SelfDPS": 15,
  "SelfDamagePercentage": 70,
  "TickRate": 0.25,
  "ToggleOffDelay": 0.5,
  "Upgrades": [
    {
      "DebuffDuration": 0.5,
      "EnemyDashSlowPercent": -25,
      "SlowPercent": 25
    },
    {
      "MaxDPS": 34.0,
      "MinDps": 6
    },
    {
      "IncomingDamagePercent": 15,
      "Radius": 1
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
    "card_name": "Aura of Suffering",
    "hero_key": "hero_frank",
    "hero_name": "Victor",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_frank",
      "hero_name": "Victor",
      "lookup": "aura of suffering",
      "name": "Aura of Suffering",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_nano_catform" title="Ava" -->

## Ava

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_nano_catform`
- Snapshot ID: `39484`
- Source-Dokument: `7070`
- Kurzinfo: Ava aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ava`
- Payload Hash: `3f667f8397ca8803cb69290581dbcf00589f12fc31d0c1f6c8097ec3a4c2525c`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.450633+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 30,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorInhibitSoftCameraCollision",
    "BehaviorMovement",
    "BehaviorPreventBotUsage",
    "BehaviorRequireAbilityButtonToCancel"
  ],
  "BuffDuration": 15,
  "BuffModifier": {
    "Class": "NanoCatform",
    "EnabledStateMask": [
      "IsTinyCharacter"
    ],
    "ModelScale": 1.0,
    "Subclass": "Catform"
  },
  "CatFormDamageDealtReduction": -100,
  "ChannelMoveSpeed": -1,
  "DamageAmpModifier": {
    "Class": "NanoDamageamp",
    "Subclass": "Damageamp"
  },
  "EnemyDamageSpeedPenalty": 65,
  "InterruptCooldown": 6,
  "IsDisabled": false,
  "Key": "ability_nano_catform",
  "MaxBonusMoveSpeedPercent": 65,
  "MinBonusMoveSpeedPercent": 30,
  "Name": "Ava",
  "SpeedBuildDuration": 4,
  "Upgrades": [
    {
      "BuffDuration": 15
    },
    {
      "HealthRegen": 15,
      "MaxBonusMoveSpeedPercent": 45
    },
    {
      "DamageAmpBuildDuration": 10,
      "DamageAmpDuration": 6,
      "OutgoingDamagePercent": 20
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
    "card_name": "Ava",
    "hero_key": "hero_nano",
    "hero_name": "Calico",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_nano",
      "hero_name": "Calico",
      "lookup": "ava",
      "name": "Ava",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_necro_coffin" title="Back Off!" -->

## Back Off!

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_necro_coffin`
- Snapshot ID: `39494`
- Source-Dokument: `7070`
- Kurzinfo: Back Off! aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Back Off!`
- Payload Hash: `2eca31158636da8b4e6861bcb313661f1d8303e232b36f4bc5e2a77b496f86f2`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.474537+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 28,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BonusMoveSpeedPercent": 15,
  "BuffDuration": {
    "Scale": {
      "Type": "duration",
      "Value": 1.0
    },
    "Value": 4
  },
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "Buff"
  },
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 65
  },
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 65
  },
  "DebuffDuration": 3,
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "Debuff"
  },
  "ImmobilizeModifier": {
    "Class": "CitadelRoot",
    "Subclass": "Immobilize"
  },
  "IsDisabled": false,
  "Key": "ability_necro_coffin",
  "Name": "Back Off!",
  "Radius": 9,
  "SlowPercent": 40,
  "Upgrades": [
    {
      "CombatBarrier": 50
    },
    {
      "AbilityCooldown": -6
    },
    {
      "BonusMoveSpeedPercent": 15
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="thumper_ability_3" title="Badger Drone" -->

## Badger Drone

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `thumper_ability_3`
- Snapshot ID: `39720`
- Source-Dokument: `7070`
- Kurzinfo: Badger Drone aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Badger Drone`
- Payload Hash: `151f794d0705bf3ad7aa9556d582071b74fd56f6a992e38e17bf583cf7530ad0`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.039831+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 30,
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorAlwaysPreviewRadius"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 8
  },
  "DroneModifier": {
    "Class": "ThumperDrone",
    "Subclass": "ThumperDrone"
  },
  "IsDisabled": false,
  "Key": "thumper_ability_3",
  "Name": "Badger Drone",
  "TickInterval": 0.5,
  "Upgrades": [
    {
      "AbilityCooldown": -0.75
    },
    {
      "AbilityCooldown": -0.75
    },
    {
      "AbilityCooldown": -0.75
    }
  ],
  "VisibilityTime": 0.2,
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_thumper",
      "hero_name": "Thumper",
      "lookup": "badger drone",
      "name": "Badger Drone",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="synth_barrage" title="Barrage" -->

## Barrage

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `synth_barrage`
- Snapshot ID: `39709`
- Source-Dokument: `7070`
- Kurzinfo: Barrage aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Barrage`
- Payload Hash: `3a8bafadb73a7c8f727b848626eed83eef6fd08562a42f643404a53c980447d8`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.012439+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.3,
  "AbilityChannelTime": 2,
  "AbilityCooldown": 32.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AirDrag": 0.3,
  "AirSpeedMax": 2.54,
  "AmpDuration": 15,
  "AmpModifier": {
    "Class": "SynthBarrageAmp",
    "Subclass": "SynthBarrageAmp"
  },
  "AmpPercentPerStack": 6,
  "AutoChannelModifier": {
    "Class": "IntrinsicBase",
    "Subclass": "IntrinsicBase"
  },
  "BarrageCasterModifier": {
    "Class": "SynthBarrageCaster",
    "Subclass": "SynthBarrageCaster"
  },
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorChannelled",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "DamagePerProjectile": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.465
    },
    "Value": 32
  },
  "DebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "SynthBarrageDebuff"
  },
  "FallSpeedMax": 10,
  "IsDisabled": false,
  "Key": "synth_barrage",
  "MoveSlowPercent": 30,
  "Name": "Barrage",
  "ProjectileAmount": 4,
  "Radius": 4.5,
  "SlowDuration": 1.5,
  "Upgrades": [
    {
      "DamagePerProjectile": 16
    },
    {
      "AbilityCooldown": -16.0
    },
    {
      "AmpPercentPerStack": 4,
      "Radius": 3
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
    "card_name": "Barrage",
    "hero_key": "hero_synth",
    "hero_name": "Pocket",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_synth",
      "hero_name": "Pocket",
      "lookup": "barrage",
      "name": "Barrage",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_punkgoat_ult" title="Bashdown" -->

## Bashdown

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_punkgoat_ult`
- Snapshot ID: `39523`
- Source-Dokument: `7070`
- Kurzinfo: Bashdown aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bashdown`
- Payload Hash: `610cc57a330595d607cdacf59eafa5101fe034dbec19b38c044bfa7574a0f52a`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.539927+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 4,
  "AbilityChannelTime": 0.3,
  "AbilityCharges": 1,
  "AbilityCooldown": 35,
  "AbilityCooldownBetweenCharge": 8,
  "AbilityPostCastDuration": 0.3,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorSilentCastFailureFeedback",
    "BehaviorChannelled",
    "BehaviorExclusiveUse",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorHoldsAtMaxChannel"
  ],
  "CameraTurnRateMax": 2000,
  "ChannelMoveSpeed": 4.826,
  "CountsAsLightMelee": 1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.1
    },
    "Value": 35
  },
  "DiminishingSlowModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "Slow"
  },
  "ExplodeDelay": 0.5,
  "FireRateModifier": {
    "Class": "Base",
    "Subclass": "SlamFirerateSlow"
  },
  "GroundAuraModifier": {
    "Class": "PunkgoatSigilAura",
    "Height": 100.0,
    "ProvidedByAura": {
      "Class": "Base",
      "Subclass": "SigilModifier"
    },
    "Subclass": "SigilAura"
  },
  "HeavyMeleeDamage": {
    "Scale": {
      "Type": "heavy_melee",
      "Value": 0
    },
    "Value": 0
  },
  "IsDisabled": false,
  "Key": "ability_punkgoat_ult",
  "MeleeDamage": {
    "Scale": {
      "Type": "melee",
      "Value": 0.9
    },
    "Value": 0
  },
  "Name": "Bashdown",
  "PlaceDistanceInFrontOfCaster": 6.2,
  "PullDownDuration": 0.75,
  "PullDownRange": 3,
  "PullToGroundModifier": {
    "Class": "ChargeDragEnemy",
    "ForceDistScale": 11,
    "ForwardOffset": 200,
    "Subclass": "ChargeDragEnemy",
    "VerticalOffset": 0
  },
  "TossDuration": 0.4,
  "TossForce": 350,
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "AbilityCastRange": 2,
      "AbilityCharges": 1
    },
    {
      "AbilityCooldownBetweenCharge": -3,
      "CountsAsHeavyMelee": 1,
      "CountsAsLightMelee": -1,
      "HeavyMeleeDamage": {
        "Scale": {
          "Type": "heavy_melee",
          "Value": 0.5
        },
        "Value": 0.0
      },
      "MeleeDamage": {
        "Scale": {
          "Multiply": true,
          "Type": "melee",
          "Value": 0.0
        },
        "Value": 0.0
      }
    }
  ],
  "WaveEndRadius": 8.0,
  "WaveStartRadius": 0.5,
  "WaveThickness": 1,
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
    "card_name": "Bashdown",
    "hero_key": "hero_punkgoat",
    "hero_name": "Billy",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_punkgoat",
      "hero_name": "Billy",
      "lookup": "bashdown",
      "name": "Bashdown",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_warden_lock_down" title="Binding Word" -->

## Binding Word

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_warden_lock_down`
- Snapshot ID: `39571`
- Source-Dokument: `7070`
- Kurzinfo: Binding Word aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Binding Word`
- Payload Hash: `ee977588d5b2eaab916fbd63a3f7e787fcb7b339ca4fe91cc2f1c060cd5910b7`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.666948+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 15,
  "AbilityCooldown": 34,
  "AbilityUnitTargetLimit": 1,
  "AdditionalTargetRadius": 20,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.437344
    },
    "Value": 110
  },
  "DebuffModifier": {
    "BulletResistModifier": {
      "Class": "LockdownBulletResist",
      "Subclass": "LockdownBulletResist"
    },
    "Class": "WardenLockdownDebuff",
    "RootModifier": {
      "Class": "CitadelRoot",
      "Subclass": "CitadelRoot"
    },
    "SilencedModifier": {
      "Class": "CitadelSilenced",
      "Subclass": "CitadelSilenced"
    },
    "Subclass": "WardenLockdownDebuff"
  },
  "EscapeRange": 20,
  "EscapeTime": 2.8,
  "ImmobilizeDuration": 1.75,
  "IsDisabled": false,
  "Key": "ability_warden_lock_down",
  "Name": "Binding Word",
  "Upgrades": [
    {
      "BulletArmorReduction": 20,
      "BulletArmorReductionDuration": 5
    },
    {
      "ImmobilizeDuration": 0.75
    },
    {
      "AbilityCooldown": -14,
      "SilenceDebuff": 1
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
    "card_name": "Binding Word",
    "hero_key": "hero_warden",
    "hero_name": "Warden",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_warden",
      "hero_name": "Warden",
      "lookup": "binding word",
      "name": "Binding Word",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_scrap_blast" title="Bio Blast" -->

## Bio Blast

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_scrap_blast`
- Snapshot ID: `39525`
- Source-Dokument: `7070`
- Kurzinfo: Bio Blast aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bio Blast`
- Payload Hash: `19d8e0db51315110fe29d3e94fd2b0ce12fa3510042a0152a09e0b7261f8752d`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.544329+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCastRange": 15,
  "AbilityCharges": 2,
  "AbilityCooldown": 64.0,
  "AbilityCooldownBetweenCharge": 3,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BlastRadius": 10,
  "ChannelMoveSpeed": -1,
  "DebuffModifier": {
    "Class": "ScrapBlastDebuff",
    "Subclass": "ScrapBlastDebuff"
  },
  "EnemyMoveSlow": 10,
  "EnemyMoveSlowDuration": 5,
  "IsDisabled": false,
  "Key": "ability_scrap_blast",
  "Name": "Bio Blast",
  "ScrapDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.731203
    },
    "Value": 75
  },
  "Upgrades": [
    {
      "AbilityCharges": 2
    },
    {
      "ScrapDamage": 55
    },
    {
      "EnemyMoveSlow": 20
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
    "card_name": "Bio Blast",
    "hero_key": "hero_wrecker",
    "hero_name": "Wrecker",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_wrecker",
      "hero_name": "Wrecker",
      "lookup": "bio blast",
      "name": "Bio Blast",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_punkgoat_blasted" title="Blasted" -->

## Blasted

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_punkgoat_blasted`
- Snapshot ID: `39519`
- Source-Dokument: `7070`
- Kurzinfo: Blasted aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Blasted`
- Payload Hash: `e079cceef0fcf21a8539dfa7964b24ecb0ab4b9add79a55bb4656278a77a026f`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.531031+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.28,
  "AbilityCooldown": 27,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8.0,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "Class": "PunkgoatBlastedhealthwatcher",
      "Subclass": "Blastedhealthwatcher"
    }
  ],
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorAllowGunFireAfterCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BlastedModifier": {
    "Class": "PunkgoatBlastedactive",
    "Subclass": "Blastedactive"
  },
  "BlastedPassiveModifier": {
    "Class": "PunkgoatBlastedpassive",
    "Subclass": "Blasted"
  },
  "BlastedRateOnBulletPct": 50,
  "BulletDamageAmp": 10,
  "BulletDamageAmpDuration": 7.0,
  "BulletsReloadedPerHeavyMeleePct": 100,
  "BulletsReloadedPerLightMeleePct": 35,
  "ChannelMoveSpeed": -1,
  "DurationPerHeavyMelee": 4.5,
  "DurationPerLightMelee": 2.8,
  "HealthBoostDuration": 11,
  "HealthDisplayModifier": {
    "Class": "Base",
    "Subclass": "Blastedhealthdisplay"
  },
  "HealthModifier": {
    "Class": "PunkgoatBlastedhealth",
    "Subclass": "Blastedhealth"
  },
  "IsDisabled": false,
  "Key": "ability_punkgoat_blasted",
  "LightMeleeScalePct": 40,
  "MaxDuration": 35.0,
  "MaxHealthMelee": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 70
  },
  "Name": "Blasted",
  "NonPlayerResourceScalePct": 25,
  "ShredModifier": {
    "Class": "PunkgoatBlastedshred",
    "Subclass": "Blastedshred"
  },
  "Upgrades": [
    {
      "BonusMoveSpeed": 2.25
    },
    {
      "BulletDamageAmp": 7,
      "GainSlamOnUse": 1
    },
    {
      "MaxHealthMelee": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 50
      }
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
    "card_name": "Blasted",
    "hero_key": "hero_punkgoat",
    "hero_name": "Billy",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_punkgoat",
      "hero_name": "Billy",
      "lookup": "blasted",
      "name": "Blasted",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_priest_antispiritvest" title="Blessed Tac-Vest" -->

## Blessed Tac-Vest

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_antispiritvest`
- Snapshot ID: `39509`
- Source-Dokument: `7070`
- Kurzinfo: Blessed Tac-Vest aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Blessed Tac-Vest`
- Payload Hash: `75e20baf06d3d8a0ed34ba747e9a8e636fd173394d678611910eaecc32082605`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.508556+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 12,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "BlockBufferDuration": 0.5,
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "Buff"
  },
  "BulletResist": 20,
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 100
  },
  "IsDisabled": false,
  "Key": "ability_priest_antispiritvest",
  "Name": "Blessed Tac-Vest",
  "ShieldBreakModifier": {
    "Class": "Base",
    "Subclass": "Shieldbreak"
  },
  "StackingModifier": {
    "Class": "PriestStackingdefense",
    "Subclass": "Stackingdefense"
  },
  "TechResist": 20,
  "Upgrades": [
    {
      "CombatBarrier": 50
    },
    {
      "AbilityCooldown": -4
    },
    {
      "BaseAttackDamagePercent": 35,
      "BuffDuration": 6
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="tokamak_radiance" title="Blinding Radiance" -->

## Blinding Radiance

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `tokamak_radiance`
- Snapshot ID: `39728`
- Source-Dokument: `7070`
- Kurzinfo: Blinding Radiance aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Blinding Radiance`
- Payload Hash: `ab04c3df080b2cf6ca9fe7663b91e8b88b3eb34168c46a8106b6744df06e9880`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.064865+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCooldown": 48.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact"
  ],
  "BlindScale": 0.5,
  "ChannelMoveSpeed": -1,
  "EvasionChance": 20,
  "IsDisabled": false,
  "Key": "tokamak_radiance",
  "LookDotMin": 0.866,
  "LookRadiusScale": 1,
  "MaxDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.097494
    },
    "Value": 16
  },
  "Name": "Blinding Radiance",
  "RadianceModifier": {
    "Class": "TokamakRadiance",
    "Subclass": "TokamakRadiance"
  },
  "Radius": 40,
  "TickRate": 0.25,
  "Upgrades": [
    {
      "AbilityCooldown": -14.0
    },
    {
      "EvasionChance": 20
    },
    {
      "MaxDPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.146241
        },
        "Value": 24
      }
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_tokamak",
      "hero_name": "Tokamak",
      "lookup": "blinding radiance",
      "name": "Blinding Radiance",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="operative_blindside" title="Blindside" -->

## Blindside

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `operative_blindside`
- Snapshot ID: `39698`
- Source-Dokument: `7070`
- Kurzinfo: Blindside aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Blindside`
- Payload Hash: `fa31acabc74a8233f7ffb251c89c0baae333147a516c8ac62bdf47c521d5f96d`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.978444+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCooldown": 30.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BackstabBonusDamagePct": 40,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanHealPlayers",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCanSetQuickCast"
  ],
  "CameraTurnRateMax": 100,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.279
    },
    "Value": 30
  },
  "EnemyDebuffModifier": {
    "Class": "OperativeBlindsideEnemyDebuff",
    "Subclass": "OperativeBlindsideEnemyDebuff"
  },
  "IsDisabled": false,
  "Key": "operative_blindside",
  "MaxCameraAngleForSeeing": 180,
  "Name": "Blindside",
  "Radius": 6.5,
  "TurnRateSlowDuration": 2.0,
  "Upgrades": [
    {
      "TurnRateSlowDuration": 1
    },
    {
      "AbilityCooldown": -12
    },
    {
      "BackstabBonusDamagePct": 30
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
    "card_name": "Blindside",
    "hero_key": "hero_operative",
    "hero_name": "Raven",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_operative",
      "hero_name": "Raven",
      "lookup": "blindside",
      "name": "Blindside",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_shiv_defer_damage" title="Bloodletting" -->

## Bloodletting

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shiv_defer_damage`
- Snapshot ID: `39641`
- Source-Dokument: `7070`
- Kurzinfo: Bloodletting aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bloodletting`
- Payload Hash: `0b51a554c9de2523efb9ae3167a49228087510c0ce465dc93c016d681f781cce`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.841821+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCooldown": 20.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoCastDelayModifier": {
    "Class": "Base",
    "Subclass": "Cast"
  },
  "BehaviourBits": [
    "BehaviorDamageDoesntWakeFromSleep",
    "BehaviorDontConsumeAbilityResourceOnCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DamagePctDeferred": 25,
  "DamagePctDeferredMaxRage": 15,
  "DeferClearPct": 35,
  "DeferredDamageDuration": 6,
  "IsDisabled": false,
  "Key": "citadel_ability_shiv_defer_damage",
  "Name": "Bloodletting",
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "DeferClearPct": 35
    },
    {
      "DamagePctDeferred": 15
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
    "card_name": "Bloodletting",
    "hero_key": "hero_shiv",
    "hero_name": "Shiv",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_shiv",
      "hero_name": "Shiv",
      "lookup": "bloodletting",
      "name": "Bloodletting",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_drifter_hunger" title="Bloodscent" -->

## Bloodscent

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_drifter_hunger`
- Snapshot ID: `39424`
- Source-Dokument: `7070`
- Kurzinfo: Bloodscent aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bloodscent`
- Payload Hash: `c06ef8ffef5aafc9e8d2ea029cf97632bc2653bfc9c2ac734b64747562c293d1`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.285040+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 80,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AmpDamagePercent": 15,
  "BehaviourBits": null,
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "DrifterHungerBuff"
  },
  "ChannelMoveSpeed": -1,
  "DelayBeforeInvisStarts": 0.6,
  "HealOnKillPct": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "InvisDuration": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "InvisFadeToDuration": 0.3,
  "InvisModifier": {
    "Class": "Invis",
    "Subclass": "Invis"
  },
  "IsDisabled": false,
  "IsolationAssistPercentValue": 100,
  "IsolationRange": 20,
  "Key": "ability_drifter_hunger",
  "KillDuration": 300,
  "LowHealthThreshold": 30,
  "MaxTrailTargets": 2,
  "Name": "Bloodscent",
  "RevealOnDamageDuration": 0.25,
  "RevealOnSpottedDuration": 1.5,
  "SpottedRadius": 15,
  "TargetLingerDuration": 3,
  "TargetModifier": {
    "Class": "HungerTarget",
    "Subclass": "HungerTarget"
  },
  "TickRate": 1,
  "TrailDuration": 10,
  "Upgrades": [
    {
      "BonusMoveSpeed": 3
    },
    {
      "HealOnKillPct": 24,
      "StaminaToRestore": 2
    },
    {
      "AmpDamagePercent": 12.0
    }
  ],
  "WeaponDmgPerIsolationKill": 3,
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
    "card_name": "Bloodscent",
    "hero_key": "hero_drifter",
    "hero_name": "Drifter",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_drifter",
      "hero_name": "Drifter",
      "lookup": "bloodscent",
      "name": "Bloodscent",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_bookworm_dragonfire" title="Bookwyrm" -->

## Bookwyrm

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_bookworm_dragonfire`
- Snapshot ID: `39408`
- Source-Dokument: `7070`
- Kurzinfo: Bookwyrm aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bookwyrm`
- Payload Hash: `85c4845d7dfbfe3e22ac1c41cedff204628f5ad398eb1402c9ce9d9a11f1cabf`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.249651+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCharges": 1,
  "AbilityCooldown": 33,
  "AbilityCooldownBetweenCharge": 7,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "AuraLingerDuration": 0.1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.3
    },
    "Value": 30
  },
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.3
    },
    "Value": 60
  },
  "DebuffDuration": 1.5,
  "DragonConeRange": 5,
  "DragonRangePerSecond": 500,
  "DragonSearchRadius": 8.5,
  "DragonSearchTickRate": 0.1,
  "DragonTravelRange": 20,
  "DragonUpwardSpeed": 400,
  "GroundAuraModifier": {
    "Class": "DragonfireGroundAura",
    "ProvidedByAura": {
      "Class": "BookwormDragonfire",
      "Subclass": "GroundauraBurn"
    },
    "Subclass": "Groundaura"
  },
  "GroundAuraSpacing": 1,
  "GroundFlameDuration": 3.0,
  "IsDisabled": false,
  "Key": "ability_bookworm_dragonfire",
  "Name": "Bookwyrm",
  "Radius": 4,
  "StartupDelay": 0.3,
  "TickRate": 0.3,
  "Upgrades": [
    {
      "AbilityCooldown": -12
    },
    {
      "AbilityCharges": 1,
      "GroundFlameDuration": 2,
      "Radius": 1
    },
    {
      "DPS": 30.0,
      "Damage": 100,
      "DragonTravelRange": 12
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
    "card_name": "Bookwyrm",
    "hero_key": "hero_bookworm",
    "hero_name": "Paige",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_bookworm",
      "hero_name": "Paige",
      "lookup": "bookwyrm",
      "name": "Bookwyrm",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_werewolf_kickflip" title="Boot Kick" -->

## Boot Kick

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_kickflip`
- Snapshot ID: `39576`
- Source-Dokument: `7070`
- Kurzinfo: Boot Kick aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Boot Kick`
- Payload Hash: `547d4274e35a7af95464961546eff048b2f6aa59d216366b6dc3d227d8e13f01`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.680966+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCastRange": 10.6,
  "AbilityChannelTime": 0.35,
  "AbilityCooldown": 21,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.25,
  "AbilityUnitTargetLimit": 1,
  "AirDrag": 0.8,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BonusDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.0
    },
    "Value": 25
  },
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "Buff"
  },
  "CameraTurnRateMax": 188,
  "Damage": {
    "Scale": {
      "Type": "melee",
      "Value": 0.9
    },
    "Value": 0
  },
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "Debuff"
  },
  "DisarmModifier": {
    "Class": "CitadelDisarmed",
    "Subclass": "Disarm"
  },
  "EnemyPushForceAway": 300,
  "EnemyPushForceUp": 300,
  "FallSpeedMax": 20,
  "IsDisabled": false,
  "Key": "ability_werewolf_kickflip",
  "LeapForwardOffset": 2.5,
  "LeapRadius": 1.7,
  "LeapingModifier": {
    "Class": "CitadelWerewolfLeaping",
    "Subclass": "Leaping"
  },
  "MarkDuration": {
    "Scale": {
      "Type": "duration",
      "Value": 1.0
    },
    "Value": 3
  },
  "MarkModifier": {
    "Class": "WerewolfKickflipBonusdamage",
    "Subclass": "Bonusdamage"
  },
  "Name": "Boot Kick",
  "SelfPushForceCameraAway": 600,
  "SelfPushForceUp": 200,
  "SlowDuration": 0.1,
  "SuccessEnemyModifier": {
    "Class": "CitadelRoot",
    "Subclass": "Enemysuccess"
  },
  "SuccessInputWindow": 0.3,
  "SuccessSelfModifier": {
    "Class": "WerewolfKickflipSuccessSelf",
    "EnabledStateMask": [
      "Immobilized",
      "CommandRestricted"
    ],
    "Subclass": "Selfsuccess"
  },
  "TimeScaleDebuff": 95,
  "Upgrades": [
    {
      "AbilityCooldown": -6
    },
    {
      "StaminaRestore": 2
    },
    {
      "BonusDamage": 80,
      "DebuffDuration": 5,
      "OutgoingDamagePercent": -35
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
    "card_name": "Boot Kick",
    "hero_key": "hero_werewolf",
    "hero_name": "Silver",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_werewolf",
      "hero_name": "Silver",
      "lookup": "boot kick",
      "name": "Boot Kick",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_necro_killsummon" title="Borrow Life" -->

## Borrow Life

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_necro_killsummon`
- Snapshot ID: `39499`
- Source-Dokument: `7070`
- Kurzinfo: Borrow Life aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Borrow Life`
- Payload Hash: `1a1fb025d911a44eea54826110279fe80b19cdbfb55482e896a742d743e45b99`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.486534+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 30,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_necro_killsummon",
  "Name": "Borrow Life",
  "RecastWindow": 10,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_necro_gravestone" title="Borrowed Decree" -->

## Borrowed Decree

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_necro_gravestone`
- Snapshot ID: `39496`
- Source-Dokument: `7070`
- Kurzinfo: Borrowed Decree aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Borrowed Decree`
- Payload Hash: `a45b694ef64dc05a196105ac9434e479ec7d06b425f06e9b64f823f3e8218b5e`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.478907+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 20,
  "AbilityChannelTime": 0.66,
  "AbilityCooldown": 140,
  "AbilityDuration": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.04
    },
    "Value": 16
  },
  "AbilityUnitTargetLimit": 1,
  "AuraRadius": 8,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorCanSetQuickCast",
    "BehaviorRefundFullCooldownOnChannelInterrupt",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BlockerScaleFactor": 1,
  "BonusHealthRegen": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "BonusSpiritDamagePercentage": 15,
  "BuffDuration": -1,
  "BulletResist": 12.5,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.27
    },
    "Value": 115
  },
  "DamageSlowDuration": 0.5,
  "DamageSlowPercent": 20,
  "DecayDuration": 1,
  "DecayTickRate": 0.1,
  "ExplodeDelay": 0.23,
  "ExplosionRadius": 6.5,
  "GraveStoneModifier": {
    "Class": "Gravestone",
    "EnabledStateMask": [
      "TechUntargetableByEnemies",
      "IgnoredByNpcTargeting"
    ],
    "GravestoneCriticalModifier": {
      "Class": "Base",
      "Subclass": "Gravestonecritical"
    },
    "ProvidedByAura": {
      "Class": "Base",
      "EnabledStateMask": [
        "NearHeavyPunchableDestroy"
      ],
      "Subclass": "Nearby"
    },
    "Subclass": "Gravestone"
  },
  "GravestoneHealth": 100,
  "GravestoneTakesDamage": 1,
  "GrowTime": 0.1,
  "IsDisabled": false,
  "Key": "ability_necro_gravestone",
  "KnockupRadius": 4,
  "KnockupSideRatio": 1,
  "KnockupSpeed": 240,
  "MaxGravestones": 3,
  "MaxStacks": 40,
  "Name": "Borrowed Decree",
  "PushForce": 300,
  "ReplicateZombieCast": 1,
  "SlowDuration": 1.25,
  "SlowPercent": 80,
  "SlowPercentPerStack": 0.5,
  "SpawnDuration": 1.5,
  "StackDuration": 5,
  "StackingDebuffTickRate": 0.25,
  "SummonBurstCount": 2,
  "SummonBurstFrequency": 0.1,
  "SummonFrequency": 4,
  "SummonHealth": {
    "Scale": {
      "Type": "power_increase",
      "Value": 8.0
    },
    "Value": 180
  },
  "SummonInitialDelay": 0.3,
  "SummonLifetime": 20,
  "SummonMaxCount": 32,
  "SummonMeleeDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.5
    },
    "Value": 40
  },
  "SummonSearchRadius": 4,
  "TechArmorDamageReductionPerStack": -0.5,
  "TechPower": {
    "Scale": {
      "Type": "power_increase",
      "Value": 1.0
    },
    "Value": 0
  },
  "TickRate": 0.4,
  "Upgrades": [
    {
      "AbilityCooldown": -15
    },
    {
      "AbilityDuration": 10,
      "MoveSpeedPercent": 25
    },
    {
      "CurrentHealthDamagePercentage": 5
    }
  ],
  "ZombieSummonModifier": {
    "Class": "NecroSummonzombiesArea",
    "ForwardWalkDistance": 0.0,
    "SpawningInModifier": {
      "Class": "Base",
      "EnabledStateMask": [
        "Immobilized",
        "CommandRestricted",
        "UnitStatusHealthHidden",
        "UnitStatusHidden",
        "DoNotDrawModel"
      ],
      "Subclass": "Spawningin"
    },
    "SpawningInTime": 0.1,
    "Subclass": "Summonzombiesarea",
    "SummonDecayModifier": {
      "Class": "NecroSummondecay",
      "Subclass": "SummonDecay"
    },
    "SummonModifier": {
      "Class": "Base",
      "Subclass": "SummonBuffs"
    },
    "ZombieSpawnForwardOffset": 100.0,
    "ZombieSpawnNavMeshSearchDistance": 300.0
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
    "card_name": "Borrowed Decree",
    "hero_key": "hero_necro",
    "hero_name": "Graves",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_necro",
      "hero_name": "Graves",
      "lookup": "borrowed decree",
      "name": "Borrowed Decree",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_trapper_poisonjar" title="Bottled Phantasmicide" -->

## Bottled Phantasmicide

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_poisonjar`
- Snapshot ID: `39545`
- Source-Dokument: `7070`
- Kurzinfo: Bottled Phantasmicide aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bottled Phantasmicide`
- Payload Hash: `8e0217558ae5f06dcdc634b0f9556788157adbb749eb9efc27c55518d6cc5fde`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.595720+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 30,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 1,
  "AuraModifier": {
    "Class": "CitadelTrapperPoisonjarAura",
    "ProvidedByAura": {
      "Class": "CitadelSilenced",
      "Subclass": "PoisonjarDebuff"
    },
    "Subclass": "PoisonjarAura"
  },
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "Height": 2,
  "InitialRadius": 6,
  "IsDisabled": false,
  "Key": "ability_trapper_poisonjar",
  "Name": "Bottled Phantasmicide",
  "RadiusPerSecond": 0.25,
  "SlowPercent": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.279
    },
    "Value": 25
  },
  "TickRate": 0.25,
  "Upgrades": [
    {
      "SlowPercent": 20
    },
    {
      "AbilityDuration": 4
    },
    {
      "TechArmorDamageReduction": -25
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
    "card_name": "Bottled Phantasmicide",
    "hero_key": "hero_trapper",
    "hero_name": "Trapper",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_trapper",
      "hero_name": "Trapper",
      "lookup": "bottled phantasmicide",
      "name": "Bottled Phantasmicide",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_bounce_pad" title="Bounce Pad" -->

## Bounce Pad

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_bounce_pad`
- Snapshot ID: `39411`
- Source-Dokument: `7070`
- Kurzinfo: Bounce Pad aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bounce Pad`
- Payload Hash: `c800e3ee1160be62e3e7351d3d58e90377f0ba988e468fc5aa050d9b33134221`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.257335+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.08,
  "AbilityCharges": 1,
  "AbilityCooldown": 41,
  "AbilityCooldownBetweenCharge": 3.5,
  "AbilityDuration": 22,
  "AbilityUnitTargetLimit": 1,
  "AirControlAccelPercent": 50,
  "AirControlPercent": 100,
  "AllyBounceModifier": {
    "Class": "BouncePadAlly",
    "Subclass": "BouncePadAlly"
  },
  "BarrelBounceVelocity": 800,
  "BarrelUpFactor": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorMovement",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BounceModifier": {
    "Class": "CitadelBouncePadStomp",
    "Subclass": "CitadelBouncePadStomp"
  },
  "BounceVelocity": 750,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_bounce_pad",
  "MinAirTimeForStomp": 0.2,
  "Name": "Bounce Pad",
  "PlaceDistance": 200,
  "Radius": 9,
  "Scale": 1,
  "SpeedOnLandModifier": {
    "Class": "Base",
    "Subclass": "SpeedOnLand"
  },
  "StompDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.372
    },
    "Value": 60
  },
  "TossSpeed": 12.7,
  "UpFactor": 1.2,
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "SpeedOnLand": 4,
      "SpeedOnLandDuration": 4
    },
    {
      "StompStunDuration": 0.7
    }
  ],
  "VerticalDifferenceTolerance": 60,
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
    "card_name": "Bounce Pad",
    "hero_key": "hero_astro",
    "hero_name": "Holliday",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_astro",
      "hero_name": "Holliday",
      "lookup": "bounce pad",
      "name": "Bounce Pad",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="fathom_breach" title="Breach" -->

## Breach

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `fathom_breach`
- Snapshot ID: `39675`
- Source-Dokument: `7070`
- Kurzinfo: Breach aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Breach`
- Payload Hash: `1a4b640dd0c743efe98943d6ff52ae2dcca963b8e74d28ff0c35343cd42f5ac2`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.921019+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 20,
  "AbilityCooldown": 22.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorMovement"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.651
    },
    "Value": 80
  },
  "ExplosionRadius": 6,
  "GravityScale": 1.4,
  "InFlightModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "DashDisabled"
    ],
    "Subclass": "InFlight"
  },
  "IsDisabled": false,
  "Key": "fathom_breach",
  "Name": "Breach",
  "TossSpeed": 350,
  "Upgrades": [
    {
      "ExplosionRadius": 3
    },
    {
      "AbilityCooldown": -8
    },
    {
      "Damage": 120
    }
  ],
  "WallImpactLookAheadDistance": 100,
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
    "card_name": "Breach",
    "hero_key": "hero_slork",
    "hero_name": "Fathom",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_slork",
      "hero_name": "Fathom",
      "lookup": "breach",
      "name": "Breach",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="tokamak_breach" title="Breach" -->

## Breach

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `tokamak_breach`
- Snapshot ID: `39722`
- Source-Dokument: `7070`
- Kurzinfo: Breach aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Breach`
- Payload Hash: `dda9340b3381b4721d2d6797e1599b2668b9a474fed48651555753789715c27f`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.046946+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AllySmokeAOEModifier": {
    "Class": "TokamakAllySmokeAoe",
    "ProvidedByAura": {
      "Class": "Invis",
      "Subclass": "TokamakAllyInSmoke"
    },
    "Subclass": "TokamakAllySmokeAoe"
  },
  "BehaviourBits": null,
  "ChannelMoveSpeed": -1,
  "EnemySmokeAOEModifier": {
    "Class": "TokamakEnemySmokeAoe",
    "ProvidedByAura": {
      "Class": "CitadelSilenced",
      "Subclass": "CitadelSilenced"
    },
    "Subclass": "TokamakEnemySmokeAoe"
  },
  "FullInvisDistance": 5,
  "InvisAlertWhenFading": 1,
  "InvisFadeToDuration": 1.0,
  "IsDisabled": false,
  "Key": "tokamak_breach",
  "Name": "Breach",
  "Radius": 12,
  "RevealOnDamageDuration": 1.0,
  "RevealOnSpottedDuration": 1.0,
  "SmokeDuration": 8,
  "SpottedRadius": 2.5,
  "TechResist": 20,
  "Upgrades": [
    {
      "AbilityCooldown": -14.0
    },
    {
      "PurgeDebuffs": 1
    },
    {
      "EMPEnemies": 1
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_bullet_flurry" title="Bullet Dance" -->

## Bullet Dance

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_bullet_flurry`
- Snapshot ID: `39412`
- Source-Dokument: `7070`
- Kurzinfo: Bullet Dance aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bullet Dance`
- Payload Hash: `79765ede01872654ef7835f0d93c8106fc45c69eb33b96583fff0b534f9eb4c9`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.259969+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.4,
  "AbilityCooldown": 165.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.03
    },
    "Value": 3.5
  },
  "AbilityUnitTargetLimit": 1,
  "AutoCastDelayModifier": {
    "Class": "CitadelBulletFlurryWindup",
    "Subclass": "Cast"
  },
  "BehaviourBits": [
    "BehaviorExclusiveUse",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCleaveDisabled",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BonusFireRate": 25,
  "BulletFlurryModifier": {
    "Class": "CitadelBulletFlurry",
    "EnabledStateMask": [
      "SilencedHidden",
      "InfiniteClip"
    ],
    "Subclass": "CitadelBulletFlurry"
  },
  "ChannelMoveSpeed": 4,
  "EvasionPercent": 30,
  "IsDisabled": false,
  "Key": "ability_bullet_flurry",
  "Name": "Bullet Dance",
  "OverrideBulletRadius": 10,
  "ProcChance": 100,
  "Radius": 16,
  "RadiusMin": 0.75,
  "TargetsPerTick": 1,
  "Upgrades": [
    {
      "WeaponDamageBonus": 7
    },
    {
      "BonusFireRate": 10,
      "ChannelMoveSpeed": 3
    },
    {
      "AbilityCooldown": -65,
      "EvasionPercent": 40.0
    }
  ],
  "WeaponDamageBonus": 7,
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
    "card_name": "Bullet Dance",
    "hero_key": "hero_haze",
    "hero_name": "Haze",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_haze",
      "hero_name": "Haze",
      "lookup": "bullet dance",
      "name": "Bullet Dance",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_burrow" title="Burrow" -->

## Burrow

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_burrow`
- Snapshot ID: `39413`
- Source-Dokument: `7070`
- Kurzinfo: Burrow aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Burrow`
- Payload Hash: `4ce73ce37f5e3481f0830c14543909abbbabc49251ecf0a1b853aeb850e4e58f`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.262106+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1,
  "AbilityChannelTime": 5,
  "AbilityCooldown": 40.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BonusMoveSpeed": 5,
  "BulletResist": 60,
  "BurrowModifier": {
    "Class": "Burrow",
    "DesatAmount": 0.3,
    "EnabledStateMask": [
      "Sprinting",
      "DamageMovementPenaltyImmune",
      "MantleDisabled",
      "MeleeDisabled",
      "DashDisabled",
      "DuckingDisabled",
      "ZiplineDisabled",
      "AllowInTunnelsNoDuck",
      "ShrunkCharacter",
      "IsTinyCharacter"
    ],
    "Subclass": "Burrow"
  },
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.488
    },
    "Value": 75
  },
  "EnemyDamageSpeedPenalty": 0.5,
  "IsDisabled": false,
  "Key": "ability_burrow",
  "Name": "Burrow",
  "Radius": 5,
  "SpeedLostDuration": 1,
  "SpinDuration": 1.5,
  "SpinModifier": {
    "Class": "Spin",
    "SlowModifier": {
      "Class": "SlowBase",
      "Subclass": "Slow"
    },
    "Subclass": "Spin"
  },
  "SpinSlowDuration": 0.3,
  "SpinSlowPercent": 10,
  "TechResist": 30,
  "TickRate": 0.1,
  "TossDuration": 1,
  "UpForce": 250,
  "Upgrades": [
    {
      "DPS": 50
    },
    {
      "AbilityChannelTime": 4,
      "Radius": 2
    },
    {
      "AbilityCooldown": -20.0,
      "BonusMoveSpeed": 4
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
    "card_name": "Burrow",
    "hero_key": "hero_krill",
    "hero_name": "Mo & Krill",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_krill",
      "hero_name": "Mo & Krill",
      "lookup": "burrow",
      "name": "Burrow",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_doorman_bomb" title="Call Bell" -->

## Call Bell

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_doorman_bomb`
- Snapshot ID: `39419`
- Source-Dokument: `7070`
- Kurzinfo: Call Bell aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Call Bell`
- Payload Hash: `512a551ac30e525f304ff939e5ca96c7c2f817037cc1523d0160bfd5d1c6e299`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.273937+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCharges": 1,
  "AbilityCooldown": 18.0,
  "AbilityCooldownBetweenCharge": 6,
  "AbilityDuration": 4,
  "AbilityPostCastDuration": 0.5,
  "AbilityUnitTargetLimit": 1,
  "AccuracyDebuffFalloffBias": 0.3,
  "AuraModifier": {
    "Class": "DoormanBellAura",
    "ProvidedByAura": {
      "Class": "SlowBase",
      "Subclass": "SlowAuraModifier"
    },
    "Subclass": "DoormanBellAura"
  },
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DebuffAccuracy": -40,
  "EnableAura": 1,
  "ExplosionDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.2
    },
    "Value": 55
  },
  "ImpactDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.7
    },
    "Value": 40
  },
  "InaccuracyModifier": {
    "Class": "DoomanBombDebuff",
    "Subclass": "Debuff"
  },
  "IsDisabled": false,
  "Key": "ability_doorman_bomb",
  "Name": "Call Bell",
  "ProjectileDrag": 0.975,
  "ProjectileFuse": 3,
  "Radius": 5.5,
  "SlowPercent": 35,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "ExplosionDamage": 40,
      "ImpactDamage": 30
    },
    {
      "ExplosionDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.35
        },
        "Value": 0
      },
      "ImpactDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.35
        },
        "Value": 0
      },
      "ProjectileFuse": 26,
      "Radius": 4.5
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
    "card_name": "Call Bell",
    "hero_key": "hero_doorman",
    "hero_name": "The Doorman",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_doorman",
      "hero_name": "The Doorman",
      "lookup": "call bell",
      "name": "Call Bell",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_fissure_wall_cancel" title="Cancel Fissure Wall" -->

## Cancel Fissure Wall

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_fissure_wall_cancel`
- Snapshot ID: `39610`
- Source-Dokument: `7070`
- Kurzinfo: Cancel Fissure Wall aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Cancel Fissure Wall`
- Payload Hash: `6622f8a8e9321c2245f61da650d2f1b53de4aedd572d6abb8d04b7d42c324c28`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.768048+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 1,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontInterruptSprint",
    "BehaviorNoTarget",
    "BehaviorCastableWhileHidden",
    "BehaviorIgnoreSelectionMashProtection",
    "BehaviorTrigger",
    "BehaviorCastableWhileBusy",
    "BehaviorNotSilencable"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "citadel_ability_fissure_wall_cancel",
  "Name": "Cancel Fissure Wall",
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_bookworm_aoemagic" title="Captivating Read" -->

## Captivating Read

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_bookworm_aoemagic`
- Snapshot ID: `39407`
- Source-Dokument: `7070`
- Kurzinfo: Captivating Read aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Captivating Read`
- Payload Hash: `ab9a3fc954d0fb563d8e50e86d87bb6ea243b2feaa9b3ac48cdc3d6b66795df8`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.247136+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 30,
  "AbilityCooldown": 30,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AreaModifier": {
    "Class": "BookwormAoemagicAreamodifier",
    "DebuffModifier": {
      "Class": "Base",
      "Subclass": "Debuff"
    },
    "RootModifier": {
      "Class": "BookwormImmobilize",
      "Subclass": "Root"
    },
    "SlowModifier": {
      "Class": "SlowBase",
      "Subclass": "Slowmodifier"
    },
    "Subclass": "Areamodifier"
  },
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.3
    },
    "Value": 90
  },
  "DetonationDelay": 1.25,
  "Height": 8,
  "ImmobilizeDuration": 1.0,
  "IsDisabled": false,
  "Key": "ability_bookworm_aoemagic",
  "Name": "Captivating Read",
  "Radius": 7.5,
  "SlowDuration": 0.5,
  "SlowPercent": 45,
  "Upgrades": [
    {
      "AbilityCooldown": -11
    },
    {
      "ImmobilizeDuration": 1
    },
    {
      "DebuffDuration": 6,
      "Radius": 1,
      "TechArmorDamageReduction": -18
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
    "card_name": "Captivating Read",
    "hero_key": "hero_bookworm",
    "hero_name": "Paige",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_bookworm",
      "hero_name": "Paige",
      "lookup": "captivating read",
      "name": "Captivating Read",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_magician_stage" title="Captive Audience" -->

## Captive Audience

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_stage`
- Snapshot ID: `39482`
- Source-Dokument: `7070`
- Kurzinfo: Captive Audience aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Captive Audience`
- Payload Hash: `25e588109b2b71a734cb4eb143d9792e201cf049c6b07c9c8524cf418a15f640`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.446159+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 20,
  "AbilityCooldown": 127.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5.5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorAllowSelfCast",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "BlockerScaleFactor": 115,
  "BonusHealthRegen": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 120
  },
  "ChannelMoveSpeed": -1,
  "EnemyDragSpeed": 25.4,
  "GrowTime": 0.2,
  "IceDomeModifier": {
    "Class": "IceDome",
    "EnemyAuraModifier": {
      "Class": "BaseAura",
      "ProvidedByAura": {
        "Class": "IcedomeAuramodifierBase",
        "Duration": 0.5,
        "EnabledStateMask": [
          "Slowed"
        ],
        "Subclass": "Debuff"
      },
      "Subclass": "IceDomeEnemyAura"
    },
    "FriendlyAuraModifier": {
      "Class": "BaseAura",
      "ProvidedByAura": {
        "Class": "IceDomeFriendly",
        "Duration": 0.5,
        "Subclass": "IceDomeFriendly"
      },
      "Subclass": "IceDomeFriendlyAura"
    },
    "Subclass": "IceDome"
  },
  "IsDisabled": false,
  "Key": "ability_magician_stage",
  "Name": "Captive Audience",
  "Radius": 15,
  "SlowPercent": 35,
  "Upgrades": [
    {
      "FireRateSlow": 40
    },
    {
      "AbilityCooldown": -38.0
    },
    {
      "BonusHealthRegen": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.52334
        },
        "Value": 70
      }
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_card_toss" title="Card Trick" -->

## Card Trick

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_card_toss`
- Snapshot ID: `39600`
- Source-Dokument: `7070`
- Kurzinfo: Card Trick aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Card Trick`
- Payload Hash: `7484c13c05ccb32174ccbe4e462692d70bd72f027b62ff179a8eaacaf82b89f0`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.741259+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Scale": {
      "Type": "range",
      "Value": 0.0
    },
    "Value": 500
  },
  "AbilityCharges": 2,
  "AbilityChargesConditionally": 1,
  "AbilityCooldown": {
    "Scale": {
      "Type": "cooldown",
      "Value": 0.0
    },
    "Value": 0.6
  },
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.1,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorAllowAltCast",
    "BehaviorAllowAltCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusAbilityResource": 100,
  "CardResourceGenPctScale": {
    "Scale": {
      "Type": "cooldown",
      "Value": -1.0
    },
    "Value": 85
  },
  "CardResourcePerBulletCrit": 6,
  "CardResourcePerBulletHit": 4,
  "CardResourcePerHeavyMelee": 25,
  "CardResourcePerLightMelee": 10,
  "ChannelMoveSpeed": -1,
  "ClubModifier": {
    "Class": "SlowBase",
    "Subclass": "ClubDebuff"
  },
  "ClubSlowDuration": 3,
  "ClubSlowPercent": -30,
  "CooldownBetweenCards": 0.5,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.55
    },
    "Value": 45
  },
  "DiamondModifier": {
    "Class": "CardtossStackingResistShred",
    "Subclass": "DiamondCardDebuff"
  },
  "DiamondResistShred": -8.0,
  "DiamondResistShredDuration": 5,
  "HeartHeal": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 75
  },
  "HeartHealNonHeroRatio": 0.5,
  "IsDisabled": false,
  "JokerExtraCardSearchRadius": 20,
  "Key": "citadel_ability_card_toss",
  "Name": "Card Trick",
  "NonPlayerCardResourceScale": 0.35,
  "ProjectileOriginHeightOffset": 50,
  "Radius": 4,
  "ResourcePerCard": 100,
  "SpadeDamageBonus": 60,
  "Upgrades": [
    {
      "AbilityCharges": 2
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.4
        },
        "Value": 40
      }
    },
    {
      "ClubSlowPercent": -20,
      "DiamondResistShred": -5,
      "HeartHeal": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 75
      },
      "ImprovedJokerChance": 1,
      "SpadeDamageBonus": 40
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
    "card_name": "Card Trick",
    "hero_key": "hero_wraith",
    "hero_name": "Wraith",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_wraith",
      "hero_name": "Wraith",
      "lookup": "card trick",
      "name": "Card Trick",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_punkgoat_tether" title="Chain Gang" -->

## Chain Gang

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_punkgoat_tether`
- Snapshot ID: `39522`
- Source-Dokument: `7070`
- Kurzinfo: Chain Gang aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Chain Gang`
- Payload Hash: `fddec94b89f33da6ec5c00f69b1a7422ba44f0b5d52a0c572ca96a4a29df1c94`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.537504+00:00`

### Vollstaendige Payload

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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_chain_lightning" title="Chain Lightning" -->

## Chain Lightning

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chain_lightning`
- Snapshot ID: `39601`
- Source-Dokument: `7070`
- Kurzinfo: Chain Lightning aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Chain Lightning`
- Payload Hash: `48063f21215923f5d5e4e4358ccfcfffc42b1ea94185bd9142912b3189df112c`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.743725+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 0.5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "Class": "CitadelLightningBullet",
      "Subclass": "CitadelLightningBullet"
    }
  ],
  "BehaviourBits": null,
  "ChannelMoveSpeed": -1,
  "ConeAngle": 45,
  "ConeRadius": 6,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.55
    },
    "Value": 15
  },
  "IsDisabled": false,
  "Key": "citadel_ability_chain_lightning",
  "Name": "Chain Lightning",
  "Upgrades": [
    {
      "ConeRadius": 4
    },
    {
      "Damage": 19.8
    },
    {
      "MultiChain": 1
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_charged_shot" title="Charged Shot" -->

## Charged Shot

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_charged_shot`
- Snapshot ID: `39416`
- Source-Dokument: `7070`
- Kurzinfo: Charged Shot aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Charged Shot`
- Payload Hash: `7b2e3663af9355d8d943d742244dbbf5aee67fa7089d868fc02bfa89fb8ef1bb`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.267228+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.5,
  "AbilityChannelTime": 9999,
  "AbilityCharges": 1,
  "AbilityCooldown": 17.0,
  "AbilityCooldownBetweenCharge": 4,
  "AbilityUnitTargetLimit": 1,
  "AirSpeedMax": 4.1,
  "AutoChannelModifier": {
    "Class": "IntrinsicBase",
    "Subclass": "IntrinsicBase"
  },
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "CameraHeightOffset": 20,
  "CameraHorizontalOffset": 15,
  "ChannelMoveSpeed": 1.5,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 80
  },
  "FallSpeedMax": 1.524,
  "IsDisabled": false,
  "Key": "ability_charged_shot",
  "Name": "Charged Shot",
  "TechCleaveExpireTime": 0.2,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "Damage": 54.0
    },
    {
      "AbilityCooldownBetweenCharge": -3,
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 0
      }
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
    "card_name": "Charged Shot",
    "hero_key": "hero_orion",
    "hero_name": "Grey Talon",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_orion",
      "hero_name": "Grey Talon",
      "lookup": "charged shot",
      "name": "Charged Shot",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_charged_tackle" title="Charged Tackle" -->

## Charged Tackle

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_charged_tackle`
- Snapshot ID: `39602`
- Source-Dokument: `7070`
- Kurzinfo: Charged Tackle aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Charged Tackle`
- Payload Hash: `b7df72ef2c3a1d2d16f7e5d7f3dbc9aedcb28aa05f291506542ad1a6835fd9e5`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.746752+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 21.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorMovement"
  ],
  "CameraDistance": 120,
  "ChannelMoveSpeed": -1,
  "ChargeActiveModifier": {
    "Class": "CitadelChargedTackleActive",
    "Subclass": "CitadelChargedTackleActive"
  },
  "ChargeDistance": 20,
  "ChargeDragVerticalOffset": 40,
  "ChargePrepareModifier": {
    "Class": "CitadelChargedTacklePrepare",
    "Subclass": "CitadelChargedTacklePrepare"
  },
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.584963
    },
    "Value": 64
  },
  "DragModifier": {
    "Class": "ChargePullEnemy",
    "Subclass": "ChargePullEnemy"
  },
  "DragReleaseSpeed": 6,
  "IsDisabled": false,
  "Key": "citadel_ability_charged_tackle",
  "Name": "Charged Tackle",
  "PrepareTime": 0.6,
  "PullTargetSpeed": 25,
  "TackleRadius": 3,
  "TackleSpeed": 25,
  "TechCleaveExpireTime": 0.2,
  "Upgrades": [
    {
      "ChargeDistance": 10,
      "Damage": 80
    },
    {
      "TechShield": 400,
      "TechShieldDuration": 20
    },
    {
      "AbilityCooldown": -9.5,
      "PrepareTime": -0.6
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="rutger_cheat_death" title="Cheat Death" -->

## Cheat Death

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `rutger_cheat_death`
- Snapshot ID: `39701`
- Source-Dokument: `7070`
- Kurzinfo: Cheat Death aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Cheat Death`
- Payload Hash: `7bd154d551fba86871eef3ea52a7c79b0221ce3f028204c87f2b80ba0f615343`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.989684+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "BonusHealthRegen": 2,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "rutger_cheat_death",
  "ModifierCheatDeathActivated": {
    "Class": "RutgerCheatDeathActivated",
    "Duration": -1.0,
    "Subclass": "RutgerCheatDeathActivated"
  },
  "Name": "Cheat Death",
  "Upgrades": [
    {
      "AbilityDuration": 2
    },
    {
      "BonusMoveSpeed": 50
    },
    {
      "BulletLifestealPercent": 100
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_priest_silencebomb" title="Choking Incense" -->

## Choking Incense

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_silencebomb`
- Snapshot ID: `39515`
- Source-Dokument: `7070`
- Kurzinfo: Choking Incense aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Choking Incense`
- Payload Hash: `9957eb85790bdddae4fb02afcdc4a311651738499ad3fac92212634fa08371de`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.522718+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 33,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 99,
  "AuraModifier": {
    "Class": "CitadelPriestSilencebombAura",
    "ProvidedByAura": {
      "Class": "SilencebombDebuff",
      "Subclass": "Debuff"
    },
    "Subclass": "Aura"
  },
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": 5,
  "DebuffDuration": 3,
  "InitialRadius": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.01
    },
    "Value": 4
  },
  "IsDisabled": false,
  "Key": "ability_priest_silencebomb",
  "Name": "Choking Incense",
  "RadiusPerSecond": 0.2,
  "SlowDuration": 2,
  "SlowPercent": 30,
  "SmokeGrenadeModifier": {
    "Class": "Smokegrenade",
    "EnemyAuraModifier": {
      "Class": "BaseAura",
      "ProvidedByAura": {
        "Class": "Base",
        "Subclass": "Debuff"
      },
      "Subclass": "Enemyaura"
    },
    "Subclass": "PriestSmokegrenade"
  },
  "TickRate": 0.15,
  "Upgrades": [
    {
      "AbilityDuration": 3
    },
    {
      "SlowPercent": 20
    },
    {
      "SilenceStamina": 1
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_doorman_doorway_close" title="Close Doors" -->

## Close Doors

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_doorman_doorway_close`
- Snapshot ID: `39421`
- Source-Dokument: `7070`
- Kurzinfo: Close Doors aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Close Doors`
- Payload Hash: `038f87ecbf3faea9b4c0bc70b8dfd0f6c2c189e7bd459331484ce4d643be078c`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.278467+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 1,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontInterruptSprint",
    "BehaviorNoTarget",
    "BehaviorCastableWhileHidden",
    "BehaviorIgnoreSelectionMashProtection",
    "BehaviorTrigger",
    "BehaviorCastableWhileBusy"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_doorman_doorway_close",
  "Name": "Close Doors",
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_ult_combo" title="Combo" -->

## Combo

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_ult_combo`
- Snapshot ID: `39550`
- Source-Dokument: `7070`
- Kurzinfo: Combo aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Combo`
- Payload Hash: `5f92b77537074ed8c8fe1bc2ad7c699ea21582c208bef7e14f9a4bd2d7694a45`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.612158+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 4.0,
  "AbilityChannelTime": 2.4,
  "AbilityCooldown": 150.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorDisplaysDamageImpact",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BonusHealthOnKill": {
    "Scale": {
      "Type": "power_increase",
      "Value": 2
    },
    "Value": 40
  },
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 40
  },
  "IsDisabled": false,
  "Key": "ability_ult_combo",
  "KillCheckModifier": {
    "Class": "Base",
    "Subclass": "KillcheckModifier"
  },
  "Name": "Combo",
  "SelfModifier": {
    "Class": "UltComboSelf",
    "EnabledStateMask": [
      "ForceCanParry"
    ],
    "Subclass": "UltComboSelf"
  },
  "TargetModifier": {
    "Class": "UltComboTarget",
    "StatusEffectPriority": 50,
    "Subclass": "UltComboTarget",
    "TargetPosDistance": 100.0,
    "TargetPosRange": 20.0
  },
  "Upgrades": [
    {
      "LifeStealPercentOnHit": 100
    },
    {
      "AbilityCooldown": -30,
      "BulletResist": 50
    },
    {
      "AbilityChannelTime": 0.7,
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.4
        },
        "Value": 40
      }
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
    "card_name": "Combo",
    "hero_key": "hero_krill",
    "hero_name": "Mo & Krill",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_krill",
      "hero_name": "Mo & Krill",
      "lookup": "combo",
      "name": "Combo",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_fire_bomb" title="Concussive Combustion" -->

## Concussive Combustion

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_fire_bomb`
- Snapshot ID: `39437`
- Source-Dokument: `7070`
- Kurzinfo: Concussive Combustion aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Concussive Combustion`
- Payload Hash: `342d3e04c015e8ecc5f8823463a1dcdb7c3285b0341076eff8d9abc918cbfcda`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.322591+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 190.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorCastableWhileBusy",
    "BehaviorDisplaysDamageImpact"
  ],
  "BuffModifier": {
    "Class": "FirebombBuff",
    "Subclass": "Buff"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.974938
    },
    "Value": 125
  },
  "DebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "ExplodeDelay": 3.25,
  "FireBombModifier": {
    "Class": "Firebomb",
    "StatusEffectPriority": 45,
    "Subclass": "Firebomb"
  },
  "IsDisabled": false,
  "Key": "ability_fire_bomb",
  "Name": "Concussive Combustion",
  "ProgressBarModifier": {
    "Class": "Base",
    "Subclass": "Progressbar"
  },
  "Radius": 12,
  "StunDuration": 1.25,
  "Upgrades": [
    {
      "Damage": 100
    },
    {
      "AbilityCooldown": -65.0,
      "LifeStealPercentOnHit": 100
    },
    {
      "Radius": 10,
      "StunDuration": 0.9
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
    "card_name": "Concussive Combustion",
    "hero_key": "hero_inferno",
    "hero_name": "Infernus",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_inferno",
      "hero_name": "Infernus",
      "lookup": "concussive combustion",
      "name": "Concussive Combustion",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_priest_flashbang" title="Consecrating Grenade" -->

## Consecrating Grenade

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_flashbang`
- Snapshot ID: `39512`
- Source-Dokument: `7070`
- Kurzinfo: Consecrating Grenade aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Consecrating Grenade`
- Payload Hash: `f83ae2d7811f69d0a99b5aca2c92254e860f3d896b9e2d4a14fb4dcf58ba9cb6`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.515752+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.03,
  "AbilityCooldown": 25,
  "AbilityPostCastDuration": 0.15,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BounceGrenadeSpeed": 1100,
  "BounceLifetime": 0.5,
  "BurnDuration": 3.5,
  "BurnLingerDuration": 0.15,
  "BurnRadius": 4.5,
  "CameraTurnRateMax": 15,
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "power_increase",
      "Value": 1.6
    },
    "Value": 10
  },
  "Damage": {
    "Scale": {
      "Type": "weapon_damage_increase",
      "Value": 1.0
    },
    "Value": 35
  },
  "EnemyDebuffModifier": {
    "AuraRadius": 0.0,
    "Class": "PriestFlashbangburnaura",
    "FlashFadeOutTime": 0.1,
    "ProvidedByAura": {
      "Class": "PriestFlashbangburn",
      "Subclass": "Burn"
    },
    "Subclass": "Burnaura"
  },
  "HealAmpReceivePenaltyPercent": -30,
  "HealAmpRegenPenaltyPercent": -30,
  "IsDisabled": false,
  "Key": "ability_priest_flashbang",
  "Name": "Consecrating Grenade",
  "PreBounceLifetime": 15,
  "Radius": 4.5,
  "TickRate": 0.2,
  "Upgrades": [
    {
      "AbilityCooldown": -5
    },
    {
      "BurnDuration": 1.0,
      "BurnRadius": 1.5,
      "Radius": 1.5
    },
    {
      "AbilityCharges": 1,
      "AbilityCooldownBetweenCharge": 3,
      "HealAmpReceivePenaltyPercent": -20,
      "HealAmpRegenPenaltyPercent": -20
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
    "card_name": "Consecrating Grenade",
    "hero_key": "hero_priest",
    "hero_name": "Venator",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_priest",
      "hero_name": "Venator",
      "lookup": "consecrating grenade",
      "name": "Consecrating Grenade",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_wrecker_salvage" title="Consume" -->

## Consume

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_wrecker_salvage`
- Snapshot ID: `39586`
- Source-Dokument: `7070`
- Kurzinfo: Consume aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Consume`
- Payload Hash: `124fa09cd7b5b3b3b11534c89698e459303519f6e6ddc2f6f1fe909446eb0865`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.700800+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 15,
  "AbilityCooldown": 12.5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled"
  ],
  "BuffModifier": {
    "Class": "WreckerSalvageBuff",
    "Subclass": "WreckerSalvageBuff"
  },
  "ChannelMoveSpeed": 3.8,
  "ConsumeHealPercentage": 50,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.974938
    },
    "Value": 60
  },
  "IsDisabled": false,
  "Key": "ability_wrecker_salvage",
  "MaxRange": 20,
  "Name": "Consume",
  "SalvageDuration": 4,
  "SalvageEnemyModifier": {
    "Class": "WreckerSalvage",
    "Subclass": "WreckerSalvage"
  },
  "StunEnemyModifier": {
    "Class": "CitadelStunned",
    "Subclass": "WreckerSalvageStun"
  },
  "TickInterval": 0.25,
  "Upgrades": [
    {
      "ConsumeHealPercentage": 25
    },
    {
      "DPS": 40
    },
    {
      "AbilityUnitTargetLimit": 2
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
    "card_name": "Consume",
    "hero_key": "hero_wrecker",
    "hero_name": "Wrecker",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_wrecker",
      "hero_name": "Wrecker",
      "lookup": "consume",
      "name": "Consume",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_crackshot" title="Crackshot" -->

## Crackshot

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_crackshot`
- Snapshot ID: `39417`
- Source-Dokument: `7070`
- Kurzinfo: Crackshot aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Crackshot`
- Payload Hash: `01e93d8ec81c4484746e5b74a35540ee5e9df7a1c061a7af85c66d18b5e84b46`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.269519+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCooldown": 20,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "CrackshotNPCCDReduction": 50,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.116
    },
    "Value": 55
  },
  "DebuffDuration": 2,
  "DebuffModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "Slow"
  },
  "ExplosionRadius": 2,
  "FadingSlowPercent": 50,
  "IsDisabled": false,
  "Key": "ability_crackshot",
  "Name": "Crackshot",
  "Upgrades": [
    {
      "FadingSlowPercent": 25
    },
    {
      "Damage": 49.5
    },
    {
      "AbilityCooldownPerHeadshot": -6,
      "AbilityCooldownPerHeadshotNPC": -3
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
    "card_name": "Crackshot",
    "hero_key": "hero_astro",
    "hero_name": "Holliday",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_astro",
      "hero_name": "Holliday",
      "lookup": "crackshot",
      "name": "Crackshot",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_trapper_spiderwave" title="Crawling Plague" -->

## Crawling Plague

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_spiderwave`
- Snapshot ID: `39547`
- Source-Dokument: `7070`
- Kurzinfo: Crawling Plague aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Crawling Plague`
- Payload Hash: `2f781f5380fe74930053c87f1a3a2cbbb013411dc1fcf3333d6f07685cf3e2af`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.603249+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.6,
  "AbilityCooldown": 160,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "IsDisabled": false,
  "Key": "ability_trapper_spiderwave",
  "Name": "Crawling Plague",
  "Radius": 3.5,
  "SpiderArmingTime": 0.5,
  "SpiderChaseVelocity": 400,
  "SpiderClimbHeight": 0.3,
  "SpiderCount": 5,
  "SpiderDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.744
    },
    "Value": 140
  },
  "SpiderDistAboveGround": 0.1,
  "SpiderExplodeRadius": 3,
  "SpiderFloatDownRate": 8,
  "SpiderGravity": 1,
  "SpiderLifetime": 25,
  "SpiderRandomPositionRadius": 4,
  "SpiderSearchRadius": 2,
  "SpiderTickRate": 0.3,
  "SpiritReducedPerStack": 5,
  "SpiritResReducedPerStack": 5,
  "SpiritStealDebuffModifier": {
    "Class": "TrapperStealspiritDebuff",
    "Subclass": "TrapperStealspiritDebuff"
  },
  "SpiritStealDuration": 10,
  "SpreadAngle": 30,
  "SpreadDistance": 900,
  "Upgrades": [
    {
      "AbilityCooldown": -45
    },
    {
      "SpiritReducedPerStack": 3,
      "SpiritResReducedPerStack": 3
    },
    {
      "SpiderCount": 5,
      "SpreadDistance": 900
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
    "card_name": "Crawling Plague",
    "hero_key": "hero_trapper",
    "hero_name": "Trapper",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_trapper",
      "hero_name": "Trapper",
      "lookup": "crawling plague",
      "name": "Crawling Plague",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="cadence_ability_crescendo" title="Crescendo" -->

## Crescendo

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `cadence_ability_crescendo`
- Snapshot ID: `39592`
- Source-Dokument: `7070`
- Kurzinfo: Crescendo aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Crescendo`
- Payload Hash: `f40f6573d826b775564f6247b0309dbfd9debe994904c53b132b1973d346e58a`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.717948+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.5,
  "AbilityChannelTime": 3,
  "AbilityCooldown": 95.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorExclusiveUse",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BulletResist": -20,
  "CrescendoAOEModifier": {
    "Class": "CadenceCrescendoAoe",
    "ProvidedByAura": {
      "Class": "CadenceCrescendoInAoe",
      "EnabledStateMask": [
        "Disarmed",
        "Silenced",
        "Invulnerable",
        "MeleeDisabledDebuff",
        "CommandRestricted"
      ],
      "PostAOEModifier": {
        "Class": "CadenceCrescendoPostAoe",
        "Subclass": "CadenceCrescendoPostAoe"
      },
      "Subclass": "CadenceCrescendoInAoe"
    },
    "Subclass": "CadenceCrescendoAoe"
  },
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.487469
    },
    "Value": 120
  },
  "DebuffDuration": 6,
  "IsDisabled": false,
  "Key": "cadence_ability_crescendo",
  "Name": "Crescendo",
  "Radius": 12,
  "Upgrades": [
    {
      "BulletResist": -10
    },
    {
      "DebuffDuration": 4
    },
    {
      "Damage": 120
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_cadence",
      "hero_name": "Cadence",
      "lookup": "crescendo",
      "name": "Crescendo",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_healing_slash" title="Crimson Slash" -->

## Crimson Slash

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_healing_slash`
- Snapshot ID: `39613`
- Source-Dokument: `7070`
- Kurzinfo: Crimson Slash aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Crimson Slash`
- Payload Hash: `812c87fa3e850b4fd0848929534071d1a150c306a8a58a2e1cffa5c9187eb122`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.775781+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.3,
  "AbilityCooldown": 16,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BuffDuration": {
    "Scale": {
      "Type": "duration",
      "Value": 1.0
    },
    "Value": 0
  },
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "HealingSlashBuffModifier"
  },
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.37
    },
    "Value": 55
  },
  "DebuffDuration": 4,
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "Debuff"
  },
  "FireRateSlow": 30,
  "HealFixedHealth": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.035871
    },
    "Value": 55
  },
  "IsDisabled": false,
  "Key": "citadel_ability_healing_slash",
  "Name": "Crimson Slash",
  "Radius": 13,
  "Upgrades": [
    {
      "BuffDuration": 4,
      "BuffMeleeDamage": 30
    },
    {
      "HealMaxHealth": 6
    },
    {
      "AbilityCooldown": -10,
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 0
      }
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
    "card_name": "Crimson Slash",
    "hero_key": "hero_yamato",
    "hero_name": "Yamato",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_yamato",
      "hero_name": "Yamato",
      "lookup": "crimson slash",
      "name": "Crimson Slash",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_hornet_sting" title="Crow Familiar" -->

## Crow Familiar

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hornet_sting`
- Snapshot ID: `39618`
- Source-Dokument: `7070`
- Kurzinfo: Crow Familiar aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Crow Familiar`
- Payload Hash: `6df0190ea15ea3600ea6d466ca6d7233f16bb5bdfb2737c0498afc24a827648b`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.787014+00:00`

### Vollstaendige Payload

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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_werewolf_leap" title="Crushing Leap" -->

## Crushing Leap

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_leap`
- Snapshot ID: `39577`
- Source-Dokument: `7070`
- Kurzinfo: Crushing Leap aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Crushing Leap`
- Payload Hash: `1e1157dd8959f8d8f0a8491f0dd2757f6a0aa50c656bdfd011a1400570356d76`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.683314+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.35,
  "AbilityChannelTime": 3,
  "AbilityCooldown": 16,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 0.25,
  "AbilityUnitTargetLimit": 1,
  "AirDrag": 0.1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": 5.08,
  "Damage": {
    "Scale": {
      "Type": "melee",
      "Value": 1.9
    },
    "Value": 0
  },
  "DebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "Debuff"
  },
  "EnemyPushForceAway": 200,
  "EnemyPushForceUp": 300,
  "GravityScale": 1,
  "Height": 4,
  "IsDisabled": false,
  "Key": "ability_werewolf_leap",
  "LandingBonusesModifier": {
    "Class": "Base",
    "Subclass": "Buff"
  },
  "LeapCameraSpeed": 300,
  "LeapForwardSpeed": 800,
  "LeapInputSpeed": 250,
  "LeapMultiHitRadius": 1.5,
  "LeapRadius": 2.54,
  "LeapUpSpeed": 200,
  "LeapingModifier": {
    "Class": "CitadelWerewolfLeaping",
    "Subclass": "Leaping"
  },
  "Name": "Crushing Leap",
  "Radius": 7.5,
  "SlowPercent": 30,
  "Upgrades": [
    {
      "BonusMoveSpeed": 2,
      "LandingBonusesDuration": 5
    },
    {
      "AbilityCooldown": -9
    },
    {
      "StunDuration": 1.2
    }
  ],
  "WorldImpactRadius": 25,
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_dash" title="Dash" -->

## Dash

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_dash`
- Snapshot ID: `39608`
- Source-Dokument: `7070`
- Kurzinfo: Dash aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Dash`
- Payload Hash: `7a25cdfee38e41572cf2b340d984773f286b758044779d0144d097594fb837b1`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.762973+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 0.6,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "Class": "StaminaRegenJumpReduction",
      "Subclass": "StaminaRegenJumpReduction"
    }
  ],
  "BehaviourBits": [
    "BehaviorHidden",
    "BehaviorDontBreakInvisibility",
    "BehaviorDontInterruptSprint",
    "BehaviorInputDirectional2d",
    "BehaviorNotSilencable",
    "BehaviorNoTarget",
    "BehaviorNonCombat",
    "BehaviorCastableWhileHidden",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "citadel_ability_dash",
  "Name": "Dash",
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_unicorn_prismaticguard" title="Dazzling Trick" -->

## Dazzling Trick

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_unicorn_prismaticguard`
- Snapshot ID: `39553`
- Source-Dokument: `7070`
- Kurzinfo: Dazzling Trick aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Dazzling Trick`
- Payload Hash: `00a55a42719d359b16d37fc7031fa9939970a1cbbbc50ea83045e0b74388cf3a`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.619721+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 32,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BarrierDamagePercentage": 50,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld"
  ],
  "BuffDuration": 4,
  "BuffModifier": {
    "Class": "UnicornPrismaticGuard",
    "Subclass": "UnicornPrismaticGuardBuff",
    "VerticalBoost": 0.5
  },
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.8
    },
    "Value": 100
  },
  "DebuffDuration": 1.75,
  "DebuffModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "Silenced"
    ],
    "Subclass": "UnicornPrismaticGuardDebuff"
  },
  "ExplodeRadius": 14,
  "IsDisabled": false,
  "Key": "ability_unicorn_prismaticguard",
  "MaxLifetime": 4,
  "Name": "Dazzling Trick",
  "Upgrades": [
    {
      "BonusMoveSpeed": 3.5
    },
    {
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.7
        },
        "Value": 80
      }
    },
    {
      "AbilityCooldown": -18,
      "DebuffDuration": 1.5
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
    "card_name": "Dazzling Trick",
    "hero_key": "hero_unicorn",
    "hero_name": "Celeste",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_unicorn",
      "hero_name": "Celeste",
      "lookup": "dazzling trick",
      "name": "Dazzling Trick",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_lash_ultimate" title="Death Slam" -->

## Death Slam

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_lash_ultimate`
- Snapshot ID: `39625`
- Source-Dokument: `7070`
- Kurzinfo: Death Slam aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Death Slam`
- Payload Hash: `6ecfa24784fbef976d4146dace5503f8a481fbceca2f84032576a09018c71771`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.804729+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.3,
  "AbilityCastRange": 20,
  "AbilityChannelTime": 2.3,
  "AbilityCooldown": 170.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 6,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast",
    "BehaviorCastRangeIs2d"
  ],
  "BoostTime": 1.0,
  "ChannelMoveSpeed": -1,
  "GrappleEnemyModifier": {
    "Class": "LashGrappleEnemyDebuff",
    "DebuffModifier": {
      "Class": "SlowBase",
      "Subclass": "Slow"
    },
    "EnabledStateMask": [
      "AbilityMovementDebuff"
    ],
    "Subclass": "LashGrappleEnemyDebuff"
  },
  "GrappleTargetModifier": {
    "Class": "LashGrappleTarget",
    "Subclass": "Target"
  },
  "HangTime": 0.6,
  "ImpactDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.974938
    },
    "Value": 105
  },
  "ImpactRadius": 6,
  "IsDisabled": false,
  "Key": "citadel_ability_lash_ultimate",
  "LiftHeight": 6,
  "LockonConeAngle": 40,
  "LosingLockGraceTime": 0.4,
  "MaxLockonStacks": 1,
  "Name": "Death Slam",
  "NotInConeLosesLock": 1,
  "SlamSpeed": 1600,
  "SlowDuration": 4,
  "SlowPercent": 50,
  "TargetModifier": {
    "Class": "LashGrappleTarget",
    "Subclass": "Lockon"
  },
  "ThrowDistance": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.14
    },
    "Value": 14
  },
  "ThrowStraightDuration": 1.5,
  "TimeToGainLockonStack": 0.7,
  "TimeToLoseLockonStack": 2,
  "UpBoostSpeed": 400,
  "Upgrades": [
    {
      "ThrowDistance": 12
    },
    {
      "AbilityCooldown": -35
    },
    {
      "AbilityCastRange": 6,
      "StunDuration": 1.2
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
    "card_name": "Death Slam",
    "hero_key": "hero_lash",
    "hero_name": "Lash",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_lash",
      "hero_name": "Lash",
      "lookup": "death slam",
      "name": "Death Slam",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_death_tax" title="Death Tax" -->

## Death Tax

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_death_tax`
- Snapshot ID: `39418`
- Source-Dokument: `7070`
- Kurzinfo: Death Tax aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Death Tax`
- Payload Hash: `a13078d675b57e25ab2ff1e80a7a536b8814156cf90d6afde077c9118b424497`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.271659+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "ChannelMoveSpeed": -1,
  "DeathTaxHeal": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 30
  },
  "IsDisabled": false,
  "Key": "ability_death_tax",
  "Name": "Death Tax",
  "Upgrades": [
    {
      "CooldownReductionOnKill": 1
    },
    {
      "DeathTaxHeal": 30
    },
    {
      "TechPowerAmpBonus": 10,
      "TechPowerAmpBonusDuration": 10,
      "TechPowerAmpBonusMaxStacks": 10
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_gunslinger_salvo" title="Demontrigger Blitz" -->

## Demontrigger Blitz

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_gunslinger_salvo`
- Snapshot ID: `39455`
- Source-Dokument: `7070`
- Kurzinfo: Demontrigger Blitz aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Demontrigger Blitz`
- Payload Hash: `92aef4dacdc8ec70cd164559354371e5055452eaf9fb7676c7f4e274adbea2eb`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.367545+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1,
  "AbilityCastRange": 60,
  "AbilityChannelTime": 1,
  "AbilityCooldown": 90,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisarmable",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCastableWhileDodging",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": 3.8,
  "Damage": null,
  "IsDisabled": false,
  "Key": "ability_gunslinger_salvo",
  "Name": "Demontrigger Blitz",
  "OverrideBulletRadius": 0.3,
  "ProcChance": 100,
  "ProcDamagePercentage": 220,
  "ProcWatcherModifier": {
    "Class": "SalvoBullet",
    "MaxBulletsToProcInShot": 1.0,
    "Subclass": "SalvoBulletWatcher"
  },
  "TickRate": 0.5,
  "TotalShots": 4,
  "Upgrades": [
    {
      "AbilityCooldown": -20
    },
    {
      "DebuffDuration": 6
    },
    {
      "TotalShots": 2
    }
  ],
  "VictimWarningModifier": {
    "Class": "Base",
    "Subclass": "GunslingerSalvoWarningModifier"
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
    "card_name": "Demontrigger Blitz",
    "hero_key": "hero_skyrunner",
    "hero_name": "Skyrunner",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_gunslinger",
      "hero_name": "Gunslinger",
      "lookup": "demontrigger blitz",
      "name": "Demontrigger Blitz",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_fencer_throwblade" title="Disengaging Sigil" -->

## Disengaging Sigil

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_fencer_throwblade`
- Snapshot ID: `39435`
- Source-Dokument: `7070`
- Kurzinfo: Disengaging Sigil aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Disengaging Sigil`
- Payload Hash: `80e8bb9c64ec0e589cabcddd69c4fa16e7b1713c4b43818d6f27cc86c7f6d0d0`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.317702+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.5,
  "AbilityCooldown": 12,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AirDrag": 2.0,
  "AirSpeedMax": 70,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorMovement"
  ],
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "FencerSigilBuff"
  },
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.3
    },
    "Value": 85
  },
  "DebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "FencerSigilSlow"
  },
  "DisarmModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "Disarmed"
    ],
    "Subclass": "UpgradeGreaterWitheringWhipDebuff"
  },
  "FallSpeedMax": 1,
  "IsDisabled": false,
  "JumpVelocityHidden": 16,
  "Key": "ability_fencer_throwblade",
  "Name": "Disengaging Sigil",
  "SigilRadius": 6.5,
  "SlowDuration": 4,
  "SlowPercent": 30,
  "TraceToGroundDistance": 1000,
  "Upgrades": [
    {
      "BonusBulletSpeedPercent": 25,
      "BonusFireRate": 25,
      "BuffDuration": 8
    },
    {
      "ResetsAirLimit": 1,
      "StaminaToRestore": 1
    },
    {
      "RecastTime": 4
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
    "card_name": "Disengaging Sigil",
    "hero_key": "hero_fencer",
    "hero_name": "Apollo",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_fencer",
      "hero_name": "Apollo",
      "lookup": "disengaging sigil",
      "name": "Disengaging Sigil",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="mirage_sand_phantom" title="Djinn's Mark" -->

## Djinn's Mark

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `mirage_sand_phantom`
- Snapshot ID: `39693`
- Source-Dokument: `7070`
- Kurzinfo: Djinn's Mark aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Djinn's Mark`
- Payload Hash: `0135cbce667291da63627ddfa3acd99b166f31d8462f271cb2185d028e1e9e7f`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.966292+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 3,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "Class": "MirageSandPhantomProc",
      "MaxBulletsToProcInShot": 1,
      "PassiveVictimModifier": {
        "Class": "MirageSandPhantomPassiveVictim",
        "SlowModifier": {
          "Class": "DiminishingSlow",
          "Subclass": "MirageDjinnsMarkSlow"
        },
        "Subclass": "MirageSandPhantomPassiveVictim"
      },
      "ProcReadyModifier": {
        "Class": "MirageSandPhantomProcReady",
        "Subclass": "MirageSandPhantomProcReady"
      },
      "Subclass": "MirageSandPhantomProc"
    }
  ],
  "BehaviourBits": [
    "BehaviorDontInterruptSprint",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanCastOnZipline",
    "BehaviorDoNotAllowSpamProc"
  ],
  "ChannelMoveSpeed": -1,
  "DMarkMultiplierPerStack": 2,
  "IsDisabled": false,
  "Key": "mirage_sand_phantom",
  "MaxStacks": 4,
  "Name": "Djinn's Mark",
  "ProcChance": 100,
  "ProcCooldown": 3,
  "ProcDamageBase": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.35
    },
    "Value": 11
  },
  "ProcMaxRange": 40,
  "RevealDuration": 6,
  "Upgrades": [
    {
      "MovementSpeedSlow": 60,
      "SlowDuration": 0.8
    },
    {
      "ProcDamageBase": 20,
      "VictimStackDuration": 3
    },
    {
      "AbilityCooldown": -1,
      "MaxStacks": 1,
      "ProcCooldown": -1,
      "StunDuration": 0.5
    }
  ],
  "VictimStackDuration": 5,
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
    "card_name": "Djinn's Mark",
    "hero_key": "hero_mirage",
    "hero_name": "Mirage",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_mirage",
      "hero_name": "Mirage",
      "lookup": "djinn's mark",
      "name": "Djinn's Mark",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_doorman_doorway" title="Doorway" -->

## Doorway

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_doorman_doorway`
- Snapshot ID: `39420`
- Source-Dokument: `7070`
- Kurzinfo: Doorway aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Doorway`
- Payload Hash: `b07bdb0ec632b5b8b014ee22950f950cf45e0569e502b987cf38207ae3400f29`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.276272+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 50,
  "AbilityCooldown": 45,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 20,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorAllowAltCast",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "DoorwayCloseCooldown": 8,
  "DoorwayDistance": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 70
  },
  "DoorwayTimerModifier": {
    "Class": "Base",
    "Subclass": "DoorwayTimerModifier"
  },
  "IsDisabled": false,
  "Key": "ability_doorman_doorway",
  "Name": "Doorway",
  "PortalBarrierModifier": {
    "Class": "Base",
    "Subclass": "DoorwayBarrier"
  },
  "Upgrades": [
    {
      "AbilityDuration": 15
    },
    {
      "BarrierDuration": 12,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.5
        },
        "Value": 250
      }
    },
    {
      "AbilityCastRange": 30,
      "DoorwayDistance": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.15
        },
        "Value": 45
      }
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
    "card_name": "Doorway",
    "hero_key": "hero_doorman",
    "hero_name": "The Doorman",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_doorman",
      "hero_name": "The Doorman",
      "lookup": "doorway",
      "name": "Doorway",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="mirage_tornado" title="Dust Devil" -->

## Dust Devil

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `mirage_tornado`
- Snapshot ID: `39695`
- Source-Dokument: `7070`
- Kurzinfo: Dust Devil aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Dust Devil`
- Payload Hash: `b277dea249250d142c1eeab0674032c741eaef2d22260c63dd7d21600e4e80d0`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.972262+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 20,
  "AbilityCooldown": 36.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontInterruptSprint",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "ClimbHeight": 1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.3
    },
    "Value": 65
  },
  "DampingFactor": 0.1,
  "DistanceAboveGround": 0.5,
  "DropDownRate": 10,
  "EnemyLiftDuration": 0.2,
  "HoldInPlaceDuration": 0.3,
  "IsDisabled": false,
  "Key": "mirage_tornado",
  "LiftHeight": 3,
  "MaxDeltaMovementControl": 2,
  "Name": "Dust Devil",
  "OpenHeight": 8,
  "ProjectileThinkInterval": 0.01,
  "Radius": 4,
  "SlowDuration": 3,
  "SlowPercent": 30,
  "TickRate": 0.25,
  "TornadoSpeed": 24,
  "Upgrades": [
    {
      "Damage": 60
    },
    {
      "AbilityCooldown": -12,
      "WhirlwindEvasionChance": 30
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 0
      },
      "HoldInPlaceDuration": 0.3,
      "RecastWindow": 6
    }
  ],
  "WhirlwindDuration": 4,
  "WhirlwindEvasionChance": 30,
  "WhirlwindEvasionModifier": {
    "Class": "MirageTornadoEvasion",
    "Subclass": "MirageTornadoEvasion"
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
    "card_name": "Dust Devil",
    "hero_key": "hero_mirage",
    "hero_name": "Mirage",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_mirage",
      "hero_name": "Mirage",
      "lookup": "dust devil",
      "name": "Dust Devil",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_kali_dust_storm" title="Dust Storm" -->

## Dust Storm

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_kali_dust_storm`
- Snapshot ID: `39468`
- Source-Dokument: `7070`
- Kurzinfo: Dust Storm aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Dust Storm`
- Payload Hash: `1fc9542105e4112e7289cc4869fa674bf687bfdbbde8dcfec3e7e4ec07dcf3d4`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.403323+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 25.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontInterruptSprint",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDontTriggerSpellBlock",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "ClimbHeight": 1,
  "CloseRangeSpeed": 80,
  "DamagePerSecond": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 50
  },
  "DebuffDuration": 3.0,
  "DistanceAboveGround": 2,
  "DropDownRate": 2,
  "FireRateSlow": 20,
  "GrenadeTrailModifier": {
    "Class": "CitadelDustStormThrown",
    "Subclass": "CitadelDustStormThrown"
  },
  "GroundDashReductionPercent": -30,
  "IsDisabled": false,
  "Key": "ability_kali_dust_storm",
  "Name": "Dust Storm",
  "OpenHeight": 2,
  "Radius": 3.5,
  "SlowPercent": 20,
  "ThrownObjectRadius": 20,
  "TickRate": 0.25,
  "TornadoSpeed": 600,
  "TrackingDistance": 15,
  "Upgrades": [
    {
      "SlowPercent": 30
    },
    {
      "FireRateSlow": 40
    },
    {
      "AbilityCooldown": -9.5,
      "AbilityDuration": 3
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_kali",
      "hero_name": "Kali",
      "lookup": "dust storm",
      "name": "Dust Storm",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="tokamak_dying_star" title="Dying Star" -->

## Dying Star

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `tokamak_dying_star`
- Snapshot ID: `39724`
- Source-Dokument: `7070`
- Kurzinfo: Dying Star aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Dying Star`
- Payload Hash: `3adbaab781dd8ca075415ebfadedffe54ee44d25bf9641c0c57de4f46b00105c`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.053516+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 20,
  "AbilityCooldown": 25.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.487469
    },
    "Value": 80
  },
  "ExplosionRadius": 6,
  "GravityScale": 1.4,
  "InFlightModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "CommandRestricted"
    ],
    "Subclass": "InFlight"
  },
  "IsDisabled": false,
  "Key": "tokamak_dying_star",
  "Name": "Dying Star",
  "TossSpeed": 350,
  "Upgrades": [
    {
      "ExplosionRadius": 3
    },
    {
      "AbilityCooldown": -7.5
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.487469
        },
        "Value": 80
      }
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_tokamak",
      "hero_name": "Tokamak",
      "lookup": "dying star",
      "name": "Dying Star",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="synth_pulse" title="Enchanter's Satchel" -->

## Enchanter's Satchel

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `synth_pulse`
- Snapshot ID: `39712`
- Source-Dokument: `7070`
- Kurzinfo: Enchanter's Satchel aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Enchanter's Satchel`
- Payload Hash: `eac3ca8f573aeee7ee5db3d29b612ae441b95f235de5ada48727f5c2e967502c`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.019466+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityChannelTime": 1.5,
  "AbilityCooldown": 17.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.1
    },
    "Value": 65
  },
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "Debuff"
  },
  "EscapeModifier": {
    "Class": "SynthPulseEscape",
    "EnabledStateMask": [
      "DashDisabled",
      "MantleDisabled",
      "DuckingDisabled",
      "MeleeDisabled"
    ],
    "Subclass": "SynthPulseEscape"
  },
  "FallSpeedMax": 0.0254,
  "IsDisabled": false,
  "Key": "synth_pulse",
  "Name": "Enchanter's Satchel",
  "Radius": 12,
  "Upgrades": [
    {
      "AbilityCooldown": -5
    },
    {
      "Damage": 90
    },
    {
      "AbilityChannelTime": 1.5,
      "DebuffDuration": 4.0,
      "FireRateSlow": 40,
      "MoveSlowPercent": 40,
      "Radius": 4
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
    "card_name": "Enchanter's Satchel",
    "hero_key": "hero_synth",
    "hero_name": "Pocket",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_synth",
      "hero_name": "Pocket",
      "lookup": "enchanter's satchel",
      "name": "Enchanter's Satchel",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_werewolf_transformation_trigger" title="End the Curse" -->

## End the Curse

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_transformation_trigger`
- Snapshot ID: `39584`
- Source-Dokument: `7070`
- Kurzinfo: End the Curse aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `End the Curse`
- Payload Hash: `52ebbe3361d50d6cddde841b12e98a9b3d53d3f8b9c4100e0b9aab9813fb1907`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.696725+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_werewolf_transformation_trigger",
  "Name": "End the Curse",
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_werewolf_netshot" title="Entangling Bola" -->

## Entangling Bola

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_netshot`
- Snapshot ID: `39579`
- Source-Dokument: `7070`
- Kurzinfo: Entangling Bola aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Entangling Bola`
- Payload Hash: `5676d8cc98c3eaec8acf3a48e2d684a10ca623f91221fd599645e131ee21c2b1`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.687534+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.24,
  "AbilityCooldown": 23,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusDebuffModifier": {
    "Class": "Base",
    "Subclass": "Bonusdebuff"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.6
    },
    "Value": 40
  },
  "DebuffDuration": 1.5,
  "DebuffModifier": {
    "Class": "SlowBase",
    "EnabledStateMask": [
      "SilenceMovementAbilites",
      "SprintDisabled",
      "DashDisabledDebuff",
      "Slowed",
      "StaminaRegenPaused"
    ],
    "Subclass": "Debuff"
  },
  "IsDisabled": false,
  "Key": "ability_werewolf_netshot",
  "Name": "Entangling Bola",
  "RootModifier": {
    "Class": "CitadelRoot",
    "Subclass": "Root"
  },
  "SlowPercent": 20,
  "Upgrades": [
    {
      "SlowPercent": 25
    },
    {
      "AbilityCooldown": -8
    },
    {
      "DebuffDuration": 0.75,
      "RicochetCount": 2,
      "RicochetRange": 15
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
    "card_name": "Entangling Bola",
    "hero_key": "hero_werewolf",
    "hero_name": "Silver",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_werewolf",
      "hero_name": "Silver",
      "lookup": "entangling bola",
      "name": "Entangling Bola",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_tengu_urn" title="Entangling Thorns" -->

## Entangling Thorns

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tengu_urn`
- Snapshot ID: `39654`
- Source-Dokument: `7070`
- Kurzinfo: Entangling Thorns aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Entangling Thorns`
- Payload Hash: `01bbed8505fbd6b13625a81a2e6e9568e3de1b7a15de93237b0bdf8f7b79cc5a`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.872992+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCharges": 1,
  "AbilityCooldown": 32.0,
  "AbilityCooldownBetweenCharge": 5,
  "AbilityDuration": 4,
  "AbilityUnitTargetLimit": 1,
  "AuraModifier": {
    "Class": "CitadelTenguUrnAura",
    "ProvidedByAura": {
      "Class": "UrnDebuff",
      "EntangleModifier": {
        "Class": "CitadelRoot",
        "Subclass": "Immobilize"
      },
      "Subclass": "UrnDebuff"
    },
    "Subclass": "UrnAura"
  },
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.55
    },
    "Value": 40
  },
  "Height": 2,
  "IsDisabled": false,
  "Key": "citadel_ability_tengu_urn",
  "Name": "Entangling Thorns",
  "Radius": 6,
  "SlowPercent": 35,
  "TickRate": 0.25,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 0
      },
      "Radius": 2
    },
    {
      "EntangleDuration": 1.6,
      "TimeToEntangle": 2
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
    "card_name": "Entangling Thorns",
    "hero_key": "hero_tengu",
    "hero_name": "Ivy",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_tengu",
      "hero_name": "Ivy",
      "lookup": "entangling thorns",
      "name": "Entangling Thorns",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_magician_escape" title="Escape Artist" -->

## Escape Artist

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_escape`
- Snapshot ID: `39478`
- Source-Dokument: `7070`
- Kurzinfo: Escape Artist aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Escape Artist`
- Payload Hash: `c4e04d6323632ad248b95369eec05ecd0f116e6230662423e3445cedbf290cce`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.436764+00:00`

### Vollstaendige Payload

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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_blood_bomb" title="Essence Bomb" -->

## Essence Bomb

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_blood_bomb`
- Snapshot ID: `39396`
- Source-Dokument: `7070`
- Kurzinfo: Essence Bomb aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Essence Bomb`
- Payload Hash: `90a35685c55fa52f9aff1a12970ce3716fe0c007296a065f2d93951f08fedcf9`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.222529+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 14.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "ArmingDuration": 0.65,
  "BeepSoundBuildupCount": 4,
  "BeepSoundIntervalBias": 0.55,
  "BeepSoundMaxFrequency": 0.1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.22
    },
    "Value": 90
  },
  "IsDisabled": false,
  "Key": "ability_blood_bomb",
  "Name": "Essence Bomb",
  "Radius": 7,
  "SelfDamagePct": 30,
  "SpilledBloodModifier": {
    "Class": "SpilledBloodThinker",
    "Height": 80.0,
    "Subclass": "SpilledBloodThinker",
    "TickRate": 0.5
  },
  "Upgrades": [
    {
      "AbilityCooldown": -5
    },
    {
      "Damage": 50,
      "Radius": 2
    },
    {
      "BloodSpillDPSPercent": 26,
      "BloodSpillDuration": 6
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
    "card_name": "Essence Bomb",
    "hero_key": "hero_ghost",
    "hero_name": "Lady Geist",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_ghost",
      "hero_name": "Lady Geist",
      "lookup": "essence bomb",
      "name": "Essence Bomb",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_necro_fear" title="Essence Theft" -->

## Essence Theft

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_necro_fear`
- Snapshot ID: `39495`
- Source-Dokument: `7070`
- Kurzinfo: Essence Theft aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Essence Theft`
- Payload Hash: `11000da5071c50c853ca665764458780daf731b1489289894e233d9686781daa`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.476292+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "ChannelMoveSpeed": -1,
  "DebuffModifier": {
    "BuildUpDecayDelay": 2.0,
    "Class": "NecroRampup",
    "CycleTimeDelayAdd": 0.1,
    "Subclass": "Debuff"
  },
  "DelayBeforeLoss": 0.5,
  "IsDisabled": false,
  "Key": "ability_necro_fear",
  "MaxStolenAttackDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.25
    },
    "Value": 25
  },
  "MaxStolenSpiritResist": 10,
  "MaxStolenTargets": 3,
  "Name": "Essence Theft",
  "ProgressLossMultiplier": 2.3,
  "ProgressLossPerSecond": 1,
  "ShootDurationForMax": 4,
  "TickInterval": 0.15,
  "Upgrades": [
    {
      "MaxStolenSpiritResist": 5
    },
    {
      "MaxStolenAttackDamage": 20
    },
    {
      "SkullBuildUp": 0.15,
      "ZombieExplosionBuildUp": 1.0,
      "ZombieMeleeBuildUp": 0.15
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
    "card_name": "Essence Theft",
    "hero_key": "hero_necro",
    "hero_name": "Graves",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_necro",
      "hero_name": "Graves",
      "lookup": "essence theft",
      "name": "Essence Theft",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="drifter_darkness" title="Eternal Night" -->

## Eternal Night

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `drifter_darkness`
- Snapshot ID: `39672`
- Source-Dokument: `7070`
- Kurzinfo: Eternal Night aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Eternal Night`
- Payload Hash: `d89e088937d2c9d66bbed27186496b9106c87f5d39d562385dd637196f7fe6c2`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.913242+00:00`

### Vollstaendige Payload

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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_uppercut" title="Exploding Uppercut" -->

## Exploding Uppercut

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_uppercut`
- Snapshot ID: `39661`
- Source-Dokument: `7070`
- Kurzinfo: Exploding Uppercut aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Exploding Uppercut`
- Payload Hash: `e6dbfd8035e8683d9259d173d00e49c5372dfa40d673197453903c5b4d519db0`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.889639+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 22.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BonusFireRate": {
    "Scale": {
      "Type": "spirit",
      "Value": -0.186
    },
    "Value": -14
  },
  "BuffGunRangePercent": 100,
  "BuffModifier": {
    "Class": "UppercutBuff",
    "EnabledStateMask": [
      "NoWindup"
    ],
    "Subclass": "UppercutBuff"
  },
  "ChannelMoveSpeed": -1,
  "ClipModifier": {
    "Class": "UppercutClipsize",
    "Subclass": "UppercutClipsize"
  },
  "EnemyHeroTossVelocity": 20,
  "ExplodeDebuffDuration": 5,
  "ForceReductionOnAngleDown": 0.75,
  "IsDisabled": false,
  "Key": "citadel_ability_uppercut",
  "LandingDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 75
  },
  "MeleeAttackLength": 6,
  "MeleeHalfAngle": 60,
  "MeleeRadius": 2.5,
  "Name": "Exploding Uppercut",
  "OnLandDamageRadius": 14,
  "TossDuration": 0.5,
  "TossDurationFriendly": 0.3,
  "TossVelocity": 25,
  "Upgrades": [
    {
      "AbilityCooldown": -11
    },
    {
      "BuffBaseWeaponPct": 30,
      "UppercutBuffOnHit": 9
    },
    {
      "MissingHPHeal": 18,
      "RestoreHookCooldown": 1
    }
  ],
  "UppercutDamage": {
    "Scale": {
      "Type": "melee",
      "Value": 1.0
    },
    "Value": 0.01
  },
  "UppercutModifier": {
    "Class": "CitadelUppercutted",
    "ExplodeDebuffModifier": {
      "Class": "Base",
      "Subclass": "UppercutDebuff"
    },
    "NoExplodeModifier": {
      "Class": "Base",
      "Subclass": "NoUppercutExplosionDamage"
    },
    "StatusEffectPriority": 20,
    "Subclass": "CitadelUppercutted"
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
    "card_name": "Exploding Uppercut",
    "hero_key": "hero_bebop",
    "hero_name": "Bebop",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_bebop",
      "hero_name": "Bebop",
      "lookup": "exploding uppercut",
      "name": "Exploding Uppercut",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="mirage_fire_beetles" title="Fire Scarabs" -->

## Fire Scarabs

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `mirage_fire_beetles`
- Snapshot ID: `39692`
- Source-Dokument: `7070`
- Kurzinfo: Fire Scarabs aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Fire Scarabs`
- Payload Hash: `2215efcae706493170be261e3ad839574d0aa154890babd40a5e66f933ac1f7c`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.963704+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.05,
  "AbilityCharges": 2,
  "AbilityCooldown": 35,
  "AbilityCooldownBetweenCharge": 1,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.1
    },
    "Value": 8
  },
  "IsDisabled": false,
  "Key": "mirage_fire_beetles",
  "MaxStacks": 100,
  "Name": "Fire Scarabs",
  "OutgoingDamagePenaltyPercent": -20,
  "StatStolenDebuffModifier": {
    "Class": "MirageFireScarabsHealthLoss",
    "EnabledStateMask": [
      "HasFirebeetlesDebuff"
    ],
    "Subclass": "MirageFireScarabsHealthLoss"
  },
  "StealDuration": 7,
  "Upgrades": [
    {
      "DPS": 7
    },
    {
      "AbilityCharges": 2
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.17
        },
        "Value": 0
      },
      "OutgoingDamagePenaltyPercent": -15
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
    "card_name": "Fire Scarabs",
    "hero_key": "hero_mirage",
    "hero_name": "Mirage",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_mirage",
      "hero_name": "Mirage",
      "lookup": "fire scarabs",
      "name": "Fire Scarabs",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_stacking_damage" title="Fixation" -->

## Fixation

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_stacking_damage`
- Snapshot ID: `39536`
- Source-Dokument: `7070`
- Kurzinfo: Fixation aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Fixation`
- Payload Hash: `033b14d264faeafcda4c685f46ff164ef39ce55b472b27a5adf1869087f15f42`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.570776+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "ChannelMoveSpeed": -1,
  "DamageBonusFixedPerStack": 0.2,
  "IsDisabled": false,
  "Key": "ability_stacking_damage",
  "MaxStacks": 40,
  "Name": "Fixation",
  "ProcDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "StackingModifier": {
    "Class": "PassiveHazeStackingDamage",
    "SlowModifier": {
      "Class": "SlowBase",
      "Subclass": "Slow"
    },
    "Subclass": "PassiveHazeStackingDamage"
  },
  "Upgrades": [
    {
      "ProcDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.8
        },
        "Value": 40
      },
      "ProcDamageStackCount": 20,
      "SlowDuration": 2,
      "SlowPercent": 15
    },
    {
      "AbilityDuration": 5,
      "MaxStacks": 40
    },
    {
      "DamageBonusFixedPerStack": 0.14
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
    "card_name": "Fixation",
    "hero_key": "hero_haze",
    "hero_name": "Haze",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_haze",
      "hero_name": "Haze",
      "lookup": "fixation",
      "name": "Fixation",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_skyrunner_flakshot" title="Flakshot" -->

## Flakshot

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_skyrunner_flakshot`
- Snapshot ID: `39530`
- Source-Dokument: `7070`
- Kurzinfo: Flakshot aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Flakshot`
- Payload Hash: `82f7b1585f48f3e263e6eff2e5152dc82f4bffea1bdbafc6dae88525304c7f37`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.556576+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "Class": "Base",
      "Subclass": "Flakshotbase"
    }
  ],
  "BehaviourBits": [
    "BehaviorProjectileFiredAsBullet"
  ],
  "BonusDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.093
    },
    "Value": 3
  },
  "ChannelMoveSpeed": -1,
  "DirectionVariance": 0.02,
  "IsDisabled": false,
  "Key": "ability_skyrunner_flakshot",
  "MinEffectiveness": -1,
  "Name": "Flakshot",
  "Radius": 5,
  "RicochetAssistRatio": 0.5,
  "RicochetChance": 50,
  "RicochetDamagePercent": 100,
  "RicochetRadius": 20,
  "Upgrades": [
    {
      "RicochetChance": 50
    },
    {
      "BonusDamage": 5
    },
    {
      "RicochetDamagePercent": 50
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
    "card_name": "Flakshot",
    "hero_key": "hero_skyrunner",
    "hero_name": "Skyrunner",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_skyrunner",
      "hero_name": "Skyrunner",
      "lookup": "flakshot",
      "name": "Flakshot",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_flame_dash" title="Flame Dash" -->

## Flame Dash

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_flame_dash`
- Snapshot ID: `39438`
- Source-Dokument: `7070`
- Kurzinfo: Flame Dash aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Flame Dash`
- Payload Hash: `7830481a9528a914e4d4f94af295cae847c6f2a99233b17439a87df2b67a0a1e`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.324939+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 38.0,
  "AbilityDuration": 3.0,
  "AbilityUnitTargetLimit": 1,
  "AuraLingerDuration": 1.0,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventBotUsage",
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": 18,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.7
    },
    "Value": 30
  },
  "DashAirSpeed": 8,
  "DashSpeed": 12,
  "FlameAuraRadius": 4.5,
  "FlameDashJumpBonus": 50,
  "FlameDashModifier": {
    "Class": "Flamedash",
    "EnabledStateMask": [
      "AbilityMovement",
      "MantleDisabled",
      "MeleeDisabled",
      "SlidingDisabled",
      "DuckingDisabled"
    ],
    "GroundAuraModifier": {
      "Class": "FlamedashGroundAura",
      "Height": 80.0,
      "ProvidedByAura": {
        "Class": "FlamedashBurn",
        "DebuffModifier": {
          "Class": "Base",
          "Subclass": "FlamedashDebuff"
        },
        "Subclass": "FlamedashBurn"
      },
      "Subclass": "FlamedashGroundAura"
    },
    "ProgressModifier": {
      "Class": "Base",
      "Subclass": "ProgressWatcher"
    },
    "Subclass": "Flamedash"
  },
  "GroundAuraSpacing": 1,
  "GroundFlameDuration": 4,
  "IsDisabled": false,
  "Key": "ability_flame_dash",
  "Name": "Flame Dash",
  "SideMoveSpeedReduction": -65,
  "SlowResistance": 50,
  "SpeedBurstSpeed": 20,
  "TickRate": 0.5,
  "Upgrades": [
    {
      "AbilityCooldown": -12.0
    },
    {
      "DPS": 20.0,
      "GroundFlameDuration": 1
    },
    {
      "AbilityCharges": 2,
      "AbilityCooldownBetweenCharge": 14
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
    "card_name": "Flame Dash",
    "hero_key": "hero_inferno",
    "hero_name": "Infernus",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_inferno",
      "hero_name": "Infernus",
      "lookup": "flame dash",
      "name": "Flame Dash",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_gunslinger_spreadingfire" title="Flame Imp" -->

## Flame Imp

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_gunslinger_spreadingfire`
- Snapshot ID: `39456`
- Source-Dokument: `7070`
- Kurzinfo: Flame Imp aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Flame Imp`
- Payload Hash: `2824c7c7f82f9f5d5ed6595661ffd2e746798cdb4127cf8e4e934b2eac627bf9`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.371681+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 10,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontSwitchAwayOnCast",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BurnDuration": 4,
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6975
    },
    "Value": 20
  },
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.744
    },
    "Value": 50
  },
  "FireDebuffModifier": {
    "Class": "SpreadingfireDot",
    "StatusEffectPriority": 50,
    "Subclass": "SpreadingfireDot"
  },
  "IsDisabled": false,
  "Key": "ability_gunslinger_spreadingfire",
  "Name": "Flame Imp",
  "SpreadRadius": 7,
  "SpreadTargets": 1,
  "TickRate": 0.5,
  "Upgrades": [
    {
      "SpreadRadius": 3
    },
    {
      "BurnDuration": "2s"
    },
    {
      "DPS": 25,
      "SpreadTargets": 1
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_fencer_lunge" title="Flawless Advance" -->

## Flawless Advance

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_fencer_lunge`
- Snapshot ID: `39432`
- Source-Dokument: `7070`
- Kurzinfo: Flawless Advance aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Flawless Advance`
- Payload Hash: `d193ee5ed6896bf359e591c7c5c881b6e662e2a2ddcb8b3a61281e2eee2585d2`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.310628+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCooldown": 26,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "AttackDashRange": 3.0,
  "AttackingDashSpeed": 55.88,
  "BaseDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.55
    },
    "Value": 25
  },
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DashAngleThreshold": 89,
  "DashBuffModifier": {
    "Class": "Base",
    "Subclass": "Dashmodifier"
  },
  "DashRadius": 1.85,
  "DashRange": 5.0,
  "DashSpeed": 27.94,
  "HealFixedHealth": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "HoldDurationMax": 1.1,
  "HoldDurationMin": 0.25,
  "IsDisabled": false,
  "Key": "ability_fencer_lunge",
  "MaxDamageBeforePerfect": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.9
    },
    "Value": 40
  },
  "MaxProcBleedDamagePercent": 50,
  "MaxStabs": 3,
  "MaxStacks": 2,
  "Name": "Flawless Advance",
  "ParryCooldownReduction": 5,
  "PctTravelDistanceToDamageIn": 80,
  "PerfectDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.55
    },
    "Value": 65
  },
  "PerfectHoldTimeStart": 0.525,
  "PerfectWindowDuration": 0.25,
  "RecastTime": 5,
  "SlashCollisionRadius": 4.05,
  "SlashLength": 13,
  "SlashRadius": 1.6,
  "Upgrades": [
    {
      "HealFixedHealth": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.3
        },
        "Value": 35
      }
    },
    {
      "AbilityCooldown": -12,
      "BulletResist": 60,
      "DashBuffDuration": 1.5
    },
    {
      "AttackDashRange": 3.0,
      "BaseDamage": {
        "Scale": {
          "Multiply": true,
          "Type": "spirit",
          "Value": 1.15
        },
        "Value": 30
      },
      "DashSpeed": 13.97,
      "MaxDamageBeforePerfect": {
        "Scale": {
          "Multiply": true,
          "Type": "spirit",
          "Value": 1.15
        },
        "Value": 45
      },
      "PerfectDamage": {
        "Scale": {
          "Multiply": true,
          "Type": "spirit",
          "Value": 1.15
        },
        "Value": 65
      }
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
    "card_name": "Flawless Advance",
    "hero_key": "hero_fencer",
    "hero_name": "Apollo",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_fencer",
      "hero_name": "Apollo",
      "lookup": "flawless advance",
      "name": "Flawless Advance",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_hornet_leap" title="Flight" -->

## Flight

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hornet_leap`
- Snapshot ID: `39616`
- Source-Dokument: `7070`
- Kurzinfo: Flight aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Flight`
- Payload Hash: `c027458461c3d4a17a02383fb8f3c17c66d2df51b89377a2a99728297348e6a9`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.782932+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": 0.2,
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 13,
  "AbilityUnitTargetLimit": 1,
  "AirSideMoveSpeedPercentage": -35,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorMovement",
    "BehaviorRequireAbilityButtonToCancel",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "FlyingItemCastRange": 50,
  "IsDisabled": false,
  "JumpVelocity": 1000,
  "Key": "citadel_ability_hornet_leap",
  "KillCheckModifier": {
    "Class": "Base",
    "Duration": 1.0,
    "Subclass": "KillcheckModifier"
  },
  "LeapModifier": {
    "Class": "HornetLeap",
    "EnabledStateMask": [
      "AbilityMovement",
      "JumpDisabled",
      "DisableAirSpreadPenalty",
      "UnlimitedAirDashes",
      "ZiplineDisabled"
    ],
    "Subclass": "HornetLeap"
  },
  "MagicDamagePerBullet": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.18
    },
    "Value": 10
  },
  "MaxFlyHeight": 1720,
  "MinVelocityZ": -20.0,
  "Name": "Flight",
  "Upgrades": [
    {
      "BonusClipSizePercent": 50
    },
    {
      "AbilityDuration": 10
    },
    {
      "MagicDamagePerBullet": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.1
        },
        "Value": 10
      },
      "RefreshOnKill": 1
    }
  ],
  "WeaponRecoilReduction": 40,
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
    "card_name": "Flight",
    "hero_key": "hero_hornet",
    "hero_name": "Vindicta",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_hornet",
      "hero_name": "Vindicta",
      "lookup": "flight",
      "name": "Flight",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_lash_flog" title="Flog" -->

## Flog

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_lash_flog`
- Snapshot ID: `39470`
- Source-Dokument: `7070`
- Kurzinfo: Flog aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Flog`
- Payload Hash: `fecd391eb8400da0d680cbbf47839b7b83595f5b5451652bfea0a2d37866cc21`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.413444+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 20,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 30,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorNoTarget",
    "BehaviorShowCastRangeAsSatSphereWhileCasting"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.85
    },
    "Value": 65
  },
  "FlogDebuffModifier": {
    "Class": "LashFlogDebuff",
    "Subclass": "LashFlogDebuff"
  },
  "HealPctVsHeroes": 50,
  "HealPctVsNonHeroes": 16,
  "IsDisabled": false,
  "Key": "ability_lash_flog",
  "Name": "Flog",
  "TargetingConeAngle": 38,
  "Upgrades": [
    {
      "EnemySlowDuration": 3,
      "EnemySlowPct": 35
    },
    {
      "AbilityCooldown": -16.0,
      "FireRateSlow": 30
    },
    {
      "Damage": 80,
      "HealPctVsHeroes": 20,
      "HealPctVsNonHeroes": 6,
      "TargetingConeAngle": 40
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
    "card_name": "Flog",
    "hero_key": "hero_lash",
    "hero_name": "Lash",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_lash",
      "hero_name": "Lash",
      "lookup": "flog",
      "name": "Flog",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="synth_plasma_flux" title="Flying Cloak" -->

## Flying Cloak

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `synth_plasma_flux`
- Snapshot ID: `39710`
- Source-Dokument: `7070`
- Kurzinfo: Flying Cloak aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Flying Cloak`
- Payload Hash: `00c7e45b120cb4aa0b2cfc81c55f3b27c390d1d4ba28783e586ac578f50ff7eb`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.014973+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 25.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.3
    },
    "Value": 60
  },
  "IsDisabled": false,
  "Key": "synth_plasma_flux",
  "MaxLifetime": 3.8,
  "Name": "Flying Cloak",
  "Radius": 5,
  "TickRate": 0.1,
  "Upgrades": [
    {
      "Damage": 70
    },
    {
      "WeaponDamageBonus": 5,
      "WeaponDamageBonusDuration": 6
    },
    {
      "AbilityCooldown": -11,
      "MaxLifetime": 1.6
    }
  ],
  "WeaponDamageBonusModifier": {
    "Class": "SynthPlasmaFluxWeaponDamage",
    "Subclass": "SynthPlasmaFluxWeaponDamage"
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
    "card_name": "Flying Cloak",
    "hero_key": "hero_synth",
    "hero_name": "Pocket",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_synth",
      "hero_name": "Pocket",
      "lookup": "flying cloak",
      "name": "Flying Cloak",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_flying_strike" title="Flying Slash" -->

## Flying Slash

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_flying_strike`
- Snapshot ID: `39612`
- Source-Dokument: `7070`
- Kurzinfo: Flying Slash aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Flying Slash`
- Payload Hash: `909e91583d47770738e48534af8b0bf514ef85e82a76c0eb956876d2db9cd8ef`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.773579+00:00`

### Vollstaendige Payload

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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="rutger_force_field" title="Force Field" -->

## Force Field

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `rutger_force_field`
- Snapshot ID: `39702`
- Source-Dokument: `7070`
- Kurzinfo: Force Field aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Force Field`
- Payload Hash: `bf9b6c7f867618ff1b72cf39e3485b2371ce64263e9bd2e3b66225a065623610`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.992336+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 10,
  "AbilityUnitTargetLimit": 1,
  "AuraModifier": {
    "Class": "RutgerForceFieldAura",
    "ProvidedByAura": {
      "Class": "Base",
      "Subclass": "RutgerForceField"
    },
    "Subclass": "ForceFieldAura"
  },
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "ChargeUpTime": 0.5,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 70
  },
  "EdgePushDuration": 0.15,
  "ForceFieldThinkRate": 0.05,
  "Height": 150,
  "IsDisabled": false,
  "Key": "rutger_force_field",
  "Name": "Force Field",
  "SlowDuration": 0.3,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "SlowPercent": 60,
  "SpherePushExtraDistance": 1.5,
  "SphereRadius": 5,
  "Upgrades": [
    {
      "AbilityCooldown": -14.0
    },
    {
      "Damage": 70
    },
    {
      "AbilityDuration": 3
    }
  ],
  "VictimPushModifier": {
    "Class": "RutgerForceFieldPushOut",
    "Duration": -1.0,
    "EnabledStateMask": [
      "Silenced",
      "CommandRestricted",
      "AirDuckingForced"
    ],
    "Subclass": "RutgerForceFieldPushOut"
  },
  "VictimPushTime": 0.4,
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_rutger",
      "hero_name": "Rutger",
      "lookup": "force field",
      "name": "Force Field",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_astro_shotgun_toggle" title="Force of Nature" -->

## Force of Nature

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_astro_shotgun_toggle`
- Snapshot ID: `39394`
- Source-Dokument: `7070`
- Kurzinfo: Force of Nature aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Force of Nature`
- Payload Hash: `dee753d6002eb5048a04116a44f95d602d95aef874b60f4afc674b7d0bf43b6d`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.216468+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.4,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BackwardsShotDelayTime": 0.15,
  "BehaviourBits": [
    "BehaviorNoTarget"
  ],
  "BuffModifier": {
    "Class": "AstroShotgunBuff",
    "Subclass": "AstroShotgunBuff"
  },
  "BulletScaleFactor": -0.75,
  "ChannelMoveSpeed": -1,
  "ClipSizeOverride": 2,
  "IsDisabled": false,
  "Key": "ability_astro_shotgun_toggle",
  "Name": "Force of Nature",
  "Upgrades": [
    {
      "CloseRangeBonusDamageRange": 7,
      "CloseRangeBonusWeaponPower": 20
    },
    {
      "ClipSizeOverride": 2
    },
    {
      "BulletDamageIncrease": 50
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_ice_grenade" title="Frost Grenade" -->

## Frost Grenade

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_ice_grenade`
- Snapshot ID: `39462`
- Source-Dokument: `7070`
- Kurzinfo: Frost Grenade aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Frost Grenade`
- Payload Hash: `c77e9479b53e3e672fdc057772d8933661c561ff2625e4c207ecae2355dc8dd3`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.386324+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCharges": 2,
  "AbilityCooldown": 30.0,
  "AbilityCooldownBetweenCharge": 7,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanHealPlayers",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 60
  },
  "HealAmount": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.8
    },
    "Value": 60
  },
  "IceGrenadeSlowModifier": {
    "Class": "IcegrenadeDebuff",
    "Subclass": "KelvinIceGrenadeSlow"
  },
  "IsDisabled": false,
  "Key": "ability_ice_grenade",
  "Name": "Frost Grenade",
  "Radius": 6.5,
  "SlowDuration": 4,
  "SlowPercent": 40,
  "Upgrades": [
    {
      "Damage": 30,
      "HealAmount": 30
    },
    {
      "AbilityCooldown": -10,
      "PauseStaminaRegen": 1
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.8
        },
        "Value": 0
      },
      "HealAmount": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.9
        },
        "Value": 0
      },
      "Radius": 2
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
    "card_name": "Frost Grenade",
    "hero_key": "hero_kelvin",
    "hero_name": "Kelvin",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_kelvin",
      "hero_name": "Kelvin",
      "lookup": "frost grenade",
      "name": "Frost Grenade",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_ice_dome" title="Frozen Shelter" -->

## Frozen Shelter

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_ice_dome`
- Snapshot ID: `39460`
- Source-Dokument: `7070`
- Kurzinfo: Frozen Shelter aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Frozen Shelter`
- Payload Hash: `d998c0d8a76d8ee6d34a2ddfe1402c7e08e37c95f4d292881cf652fee462cdee`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.381578+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 8,
  "AbilityCooldown": 195,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorCastableWhileBusy",
    "BehaviorInterruptMeleeOnCast",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorDisplaysDamageImpact",
    "BehaviorRequireAbilityButtonToCancel",
    "BehaviorCanSetQuickCast",
    "BehaviorAllowSelfCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BlockerScaleFactor": 115,
  "BonusHealthRegen": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 90
  },
  "ChannelMoveSpeed": -1,
  "EnemyDragSpeed": 25.4,
  "GrowTime": 0.2,
  "IceDomeModifier": {
    "Class": "IceDome",
    "EnabledStateMask": [
      "SinclairTaxUltActive"
    ],
    "EnemyAuraModifier": {
      "Class": "BaseAura",
      "ProvidedByAura": {
        "Class": "IcedomeAuramodifierBase",
        "Duration": 0.5,
        "EnabledStateMask": [
          "Slowed"
        ],
        "Subclass": "Debuff"
      },
      "Subclass": "IceDomeEnemyAura"
    },
    "FriendlyAuraModifier": {
      "Class": "BaseAura",
      "ProvidedByAura": {
        "Class": "IceDomeFriendly",
        "Duration": 0.5,
        "Subclass": "IceDomeFriendly"
      },
      "Subclass": "IceDomeFriendlyAura"
    },
    "Subclass": "IceDome"
  },
  "IsDisabled": false,
  "Key": "ability_ice_dome",
  "Name": "Frozen Shelter",
  "Radius": 10,
  "SlowPercent": 35,
  "Upgrades": [
    {
      "AbilityCooldown": -20
    },
    {
      "AbilityDuration": 1.5
    },
    {
      "BonusHealthRegen": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 65
      },
      "PurgeOnCast": 1
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
    "card_name": "Frozen Shelter",
    "hero_key": "hero_kelvin",
    "hero_name": "Kelvin",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_kelvin",
      "hero_name": "Kelvin",
      "lookup": "frozen shelter",
      "name": "Frozen Shelter",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_ice_dome_trigger" title="Frozen Shelter" -->

## Frozen Shelter

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_ice_dome_trigger`
- Snapshot ID: `39461`
- Source-Dokument: `7070`
- Kurzinfo: Frozen Shelter aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Frozen Shelter`
- Payload Hash: `6f0d48ca8db708856b2c5dc7d0d5f29cfeeb1eedcd7172be3cd6205185dc408b`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.383934+00:00`

### Vollstaendige Payload

````json
{
  "BehaviourBits": null,
  "IsDisabled": false,
  "Key": "ability_ice_dome_trigger",
  "Name": "Frozen Shelter",
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_wraith_rapidfire" title="Full Auto" -->

## Full Auto

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_wraith_rapidfire`
- Snapshot ID: `39665`
- Source-Dokument: `7070`
- Kurzinfo: Full Auto aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Full Auto`
- Payload Hash: `65a4e299254f951bfee5b932ca499cd910a61fc787f547f3f781f41e8efaf257`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.897775+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 45,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "BonusFireRate": 20,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "citadel_ability_wraith_rapidfire",
  "MagicDamagePerBullet": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.03
    },
    "Value": 2
  },
  "Name": "Full Auto",
  "RapidFireModifier": {
    "Class": "Rapidfire",
    "Subclass": "Rapidfire"
  },
  "Upgrades": [
    {
      "AbilityCooldown": -20
    },
    {
      "AbilityDuration": 3,
      "BonusFireRate": 10
    },
    {
      "MagicDamagePerBullet": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.06
        },
        "Value": 0
      },
      "UnlimitedAmmo": 1
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
    "card_name": "Full Auto",
    "hero_key": "hero_wraith",
    "hero_name": "Wraith",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_wraith",
      "hero_name": "Wraith",
      "lookup": "full auto",
      "name": "Full Auto",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_nano_clustergrenade" title="Gloom Bombs" -->

## Gloom Bombs

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_nano_clustergrenade`
- Snapshot ID: `39487`
- Source-Dokument: `7070`
- Kurzinfo: Gloom Bombs aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Gloom Bombs`
- Payload Hash: `21704ff150ada49b2e9b675eb39090b61d538b8af52d410495f347ed3ca0ceaa`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.457672+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCooldown": 14,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.644
    },
    "Value": 45
  },
  "DebuffModifier": {
    "Class": "NanoClusterGrenadeDebuff",
    "Subclass": "NanoClusterGrenadeDebuff"
  },
  "GrenadeAngleVariance": 0.08,
  "GrenadeCount": 4,
  "IsDisabled": false,
  "Key": "ability_nano_clustergrenade",
  "Lifetime": 0.75,
  "ModifierDragEnemy": {
    "Class": "PerchedPredatorDrag",
    "Subclass": "PerchedPredatorDrag"
  },
  "MultiHitPenaltyPercentage": 65,
  "Name": "Gloom Bombs",
  "Radius": 3.0,
  "TimeBetweenGrenades": 0.05,
  "TossSpeed": 400,
  "Upgrades": [
    {
      "AbilityCooldown": -3
    },
    {
      "MeleeResistReduction": -6,
      "MeleeResistReductionDuration": 6
    },
    {
      "GrenadeCount": 3
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
    "card_name": "Gloom Bombs",
    "hero_key": "hero_nano",
    "hero_name": "Calico",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_nano",
      "hero_name": "Calico",
      "lookup": "gloom bombs",
      "name": "Gloom Bombs",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_werewolf_frenzy" title="Go For The Throat" -->

## Go For The Throat

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_frenzy`
- Snapshot ID: `39574`
- Source-Dokument: `7070`
- Kurzinfo: Go For The Throat aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Go For The Throat`
- Payload Hash: `0b567247941e007b6d955441f6b95c11863eeff0d59c655373413b0dbe26045a`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.677247+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 7.5,
  "AbilityCooldown": 6.5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.5,
  "AbilityUnitTargetLimit": 16,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorUseLagCompensationForUnitTargeting"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "melee",
      "Value": 1.5
    },
    "Value": 0.0
  },
  "IsDisabled": false,
  "Key": "ability_werewolf_frenzy",
  "LifeStealPercentOnHit": 40,
  "MissingHealthDamagePercentage": 6,
  "Name": "Go For The Throat",
  "TargetModifier": {
    "Class": "DrifterRendBulletLifesteal",
    "Subclass": "DrifterRendBulletLifesteal"
  },
  "TargetingConeAngle": 40,
  "Upgrades": [
    {
      "Damage": 30
    },
    {
      "LifeStealPercentOnHit": 25
    },
    {
      "MissingHealthDamagePercentage": 4
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
    "card_name": "Go For The Throat",
    "hero_key": "hero_werewolf_transformed",
    "hero_name": "Silver (Transformed)",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_werewolf_transformed",
      "hero_name": "Silver (Transformed)",
      "lookup": "go for the throat",
      "name": "Go For The Throat",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="viscous_goo_bowling_ball" title="Goo Ball" -->

## Goo Ball

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `viscous_goo_bowling_ball`
- Snapshot ID: `39734`
- Source-Dokument: `7070`
- Kurzinfo: Goo Ball aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Goo Ball`
- Payload Hash: `a53b7d4bbb07e6433debcf6916165acc081c2b317cefaf9123f4383f1a8777d7`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.079863+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.55,
  "AbilityCooldown": 150,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 11,
  "AbilityUnitTargetLimit": 1,
  "AccelerationPercentage": -60,
  "AirJumpForce": 500,
  "BallHitRadius": 1.8,
  "BallOffset": 50,
  "BallRadius": 1.4,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast",
    "BehaviorRequireAbilityButtonToCancel",
    "BehaviorInhibitSoftCameraCollision"
  ],
  "BreakablePropDamageRadius": 75,
  "BulletResist": 35,
  "CastWhileRolling": 1,
  "ChannelMoveSpeed": 7,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1
    },
    "Value": 110
  },
  "DamagePreventionModifier": {
    "Class": "Base",
    "Duration": 1.2,
    "Subclass": "ViscousBallDamagePrevention"
  },
  "FrictionPercentage": -85,
  "IsDisabled": false,
  "JumpForce": 500,
  "Key": "viscous_goo_bowling_ball",
  "KnockForce": 400,
  "MoveSpeedMax": 7,
  "Name": "Goo Ball",
  "ParticleRadiusMultiplier": 1.2,
  "RollingModifier": {
    "Class": "ViscousBall",
    "Subclass": "ViscousRollingResist"
  },
  "StunDuration": 0.5,
  "TechResist": 35,
  "TickRate": 0.25,
  "Upgrades": [
    {
      "AbilityCooldown": -25
    },
    {
      "BulletResist": 10,
      "Damage": 70,
      "TechResist": 10
    },
    {
      "AbilityDuration": 7,
      "StunDuration": 0.3
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
    "card_name": "Goo Ball",
    "hero_key": "hero_viscous",
    "hero_name": "Viscous",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_viscous",
      "hero_name": "Viscous",
      "lookup": "goo ball",
      "name": "Goo Ball",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="cadence_ability_grandfinale" title="Grand Finale" -->

## Grand Finale

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `cadence_ability_grandfinale`
- Snapshot ID: `39593`
- Source-Dokument: `7070`
- Kurzinfo: Grand Finale aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Grand Finale`
- Payload Hash: `c94818ab62ddfe585cd7e5c3a3f693efe54fd8d55c16e1ea9ba713ab66fefaac`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.721351+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCooldown": 95.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact"
  ],
  "BuildUpDuration": 10,
  "BuildUpMaxDamage": 150,
  "BuildUpMaxDamageNonHero": 300,
  "ChannelMoveSpeed": -1,
  "ExplosiveDamage": 120,
  "FireRateBonus": 30,
  "GrandFinaleAOEModifier": {
    "Class": "CadenceGrandfinaleAoe",
    "ProvidedByAura": {
      "BuildUpModifier": {
        "Class": "CitadelBaseBuildup",
        "Subclass": "CitadelBaseBuildup"
      },
      "Class": "CadenceGrandfinaleBuff",
      "Subclass": "CadenceGrandfinaleBuff"
    },
    "Subclass": "CadenceGrandfinaleAoe"
  },
  "IsDisabled": false,
  "Key": "cadence_ability_grandfinale",
  "Name": "Grand Finale",
  "Radius": 12,
  "StageDuration": 12,
  "StageRadius": 15,
  "Upgrades": [
    {
      "AbilityCooldown": -19.0
    },
    {
      "ExplosiveDamage": 120
    },
    {
      "FireRateBonus": 20
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_magician_bigbolt" title="Grand Finale!" -->

## Grand Finale!

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_bigbolt`
- Snapshot ID: `39474`
- Source-Dokument: `7070`
- Kurzinfo: Grand Finale! aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Grand Finale!`
- Payload Hash: `e8c74b45c318dca990ce9b722b291d7d3f22db21ce257dd6584311e114261199`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.426611+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.5,
  "AbilityCastRange": 500,
  "AbilityChannelTime": 8,
  "AbilityCooldown": 100,
  "AbilityCooldownBetweenCharge": 3,
  "AbilityPostCastDuration": 0.3,
  "AbilityUnitTargetLimit": 1,
  "AirSpeedMax": 70,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCooldownOnChannelEnd"
  ],
  "BoltHitModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "MagicianBigboltModifier"
  },
  "BoltRefundPerKill": 1,
  "CasterModifier": {
    "Class": "Base",
    "Subclass": "MagicianCasterBigboltModifier"
  },
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.395
    },
    "Value": 120
  },
  "DamagePerShot": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.465
    },
    "Value": 50
  },
  "DebuffDuration": 2,
  "FallSpeedMax": 1,
  "InitialProjectileVelocity": 1000,
  "IsDisabled": false,
  "Key": "ability_magician_bigbolt",
  "Name": "Grand Finale!",
  "ProjectileLifetime": 3,
  "ProjectileRedirectCount": 1,
  "Radius": 3,
  "RedirectVelocity": 1500,
  "ShootDelay": 0.7,
  "SlowPercent": 25,
  "TotalBolts": 3,
  "Upgrades": [
    {
      "SlowPercent": 25
    },
    {
      "AbilityCooldown": -40
    },
    {
      "BoltRefundPerKill": 1,
      "DamagePerShot": 50
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_lash" title="Grapple" -->

## Grapple

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_lash`
- Snapshot ID: `39623`
- Source-Dokument: `7070`
- Kurzinfo: Grapple aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Grapple`
- Payload Hash: `3cff016b23ea73b8f1c9dfcbb8760ec0b03bdcad5be43191295f6c1532a93d5a`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.799904+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 30,
  "AbilityCharges": 1,
  "AbilityCooldown": 35.0,
  "AbilityCooldownBetweenCharge": 2,
  "AbilityUnitTargetLimit": 1,
  "AirControlModifier": {
    "Class": "GrappleAirControl",
    "Subclass": "AirControl"
  },
  "BehaviourBits": [
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "GrappleBuff"
  },
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "JumpSlowResistance": 0.667,
  "JumpVelocity": 20,
  "Key": "citadel_ability_lash",
  "LashFriendlies": 1,
  "Name": "Grapple",
  "Upgrades": [
    {
      "AbilityCooldown": -17.0
    },
    {
      "AbilityCastRange": 20,
      "WeaponDamageBonus": 7.0,
      "WeaponDamageBonusDuration": 10
    },
    {
      "AbilityCharges": 1,
      "AirControlPercent": 60,
      "RestoreStaminaOnUse": 1
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
    "card_name": "Grapple",
    "hero_key": "hero_lash",
    "hero_name": "Lash",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_lash",
      "hero_name": "Lash",
      "lookup": "grapple",
      "name": "Grapple",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_hook" title="Grapple Arm" -->

## Grapple Arm

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hook`
- Snapshot ID: `39614`
- Source-Dokument: `7070`
- Kurzinfo: Grapple Arm aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Grapple Arm`
- Payload Hash: `9a5355d581f9ce2abf28d027407778a11bfa14503fdccf9fdb3bb5b25c83536b`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.778180+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 30,
  "AbilityCooldown": 23.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorPreventBotUsage",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BulletAmpModifier": {
    "Class": "BebopHookBulletAmp",
    "Subclass": "BebopHookBulletAmp"
  },
  "CancelHookDuration": 0.2,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "melee",
      "Value": 0.7
    },
    "Value": 0
  },
  "FriendlyHookIgnoreRange": 8,
  "HookImpactDelay": 0.5,
  "HookingSlowSpeedLimit": 5,
  "IsDisabled": false,
  "Key": "citadel_ability_hook",
  "Name": "Grapple Arm",
  "RestrictionDuration": 0.5,
  "SelfModifier": {
    "Class": "CitadelHookself",
    "EnabledStateMask": [
      "CastingHook"
    ],
    "Subclass": "CitadelHookself"
  },
  "SlowPercent": 90,
  "TargetModifier": {
    "Class": "CitadelHooktarget",
    "CloseEnoughDistance": 30.0,
    "FailSafeDurationMult": 2.0,
    "FailSafeMinTime": 1.0,
    "RestrictionModifier": {
      "Class": "SlowBase",
      "EnabledStateMask": [
        "SilenceMovementAbilites",
        "MovementAbilityRestricted",
        "DashDisabled",
        "Slowed"
      ],
      "Subclass": "SlowBases"
    },
    "ReturnPositionForwardOffset": 100.0,
    "ReturnSpeed": 2000.0,
    "ReturnSpeedFail": 100.0,
    "ReturnStuckTime": 0.2,
    "Subclass": "CitadelHooktarget",
    "TossUpSpeed": 0.0
  },
  "Upgrades": [
    {
      "BulletAmp": 20,
      "BulletAmpDuration": 6
    },
    {
      "AbilityCastRange": 30
    },
    {
      "AbilityCooldown": -11.5
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
    "card_name": "Grapple Arm",
    "hero_key": "hero_bebop",
    "hero_name": "Bebop",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_bebop",
      "hero_name": "Bebop",
      "lookup": "grapple arm",
      "name": "Grapple Arm",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_necro_zombiewall" title="Grasping Hands" -->

## Grasping Hands

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_necro_zombiewall`
- Snapshot ID: `39503`
- Source-Dokument: `7070`
- Kurzinfo: Grasping Hands aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Grasping Hands`
- Payload Hash: `269ba003b65f8c9413b31f58d5b37c6e710543dd45e32b5ce765dc2120228360`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.494352+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 24,
  "AbilityCooldown": 34,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "AuraRadius": 0.75,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorAllowAltCast",
    "BehaviorCanSetQuickCast",
    "BehaviorSuppressAltCastOnceSelected"
  ],
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "Buff"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.6
    },
    "Value": 90
  },
  "DebuffDuration": 0.5,
  "GroundAuraModifier": {
    "Class": "ZombiewallGroundAura",
    "ProvidedByAura": {
      "Class": "NecroZombiewallDebuff",
      "Subclass": "Debuff"
    },
    "Subclass": "Zombiewall"
  },
  "GroundAuraPopDelay": 1.1,
  "GroundAuraPriorityModifier": {
    "Class": "ZombiewallGroundAura",
    "ProvidedByAura": {
      "Class": "NecroZombiewallDebuff",
      "Subclass": "Debuff"
    },
    "Subclass": "Zombiewall"
  },
  "GroundAuraSpacing": 1,
  "ImmobilizeDuration": 1.25,
  "ImmobilizeModifier": {
    "Class": "CitadelRoot",
    "Subclass": "Immobilize"
  },
  "IsDisabled": false,
  "Key": "ability_necro_zombiewall",
  "Name": "Grasping Hands",
  "SlowPercent": 40,
  "SummonCount": 1,
  "TetherDuration": 1,
  "TetherModifier": {
    "Class": "NecroZombiewallTether",
    "Subclass": "Tether"
  },
  "TetherRadius": 0.1,
  "TickRate": 0.1,
  "Upgrades": [
    {
      "AbilityDuration": 2
    },
    {
      "Damage": 90,
      "ZombieWallLength": 10
    },
    {
      "AbilityCooldown": -10,
      "ImmobilizeDuration": 0.75,
      "SummonCount": 1
    }
  ],
  "ZombieWallDeployTime": 0.6,
  "ZombieWallHeight": 2.5,
  "ZombieWallLength": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.05
    },
    "Value": 14
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
    "card_name": "Grasping Hands",
    "hero_key": "hero_necro",
    "hero_name": "Graves",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_necro",
      "hero_name": "Graves",
      "lookup": "grasping hands",
      "name": "Grasping Hands",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_priest_stackingdefense" title="Grit" -->

## Grit

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_stackingdefense`
- Snapshot ID: `39517`
- Source-Dokument: `7070`
- Kurzinfo: Grit aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Grit`
- Payload Hash: `4da0ef908a1ddc04d2b3714390029e6893537b393b269c7ae89b51be9acb58c1`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.526346+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "ChannelMoveSpeed": -1,
  "CooldownPerStack": 0.5,
  "IsDisabled": false,
  "Key": "ability_priest_stackingdefense",
  "MaxStacks": 20,
  "Name": "Grit",
  "ResistancePerStack": 1,
  "StackDuration": 8,
  "StackingModifier": {
    "Class": "PriestStackingdefense",
    "Subclass": "Stackingdefense"
  },
  "Upgrades": [
    {
      "StackDuration": 2
    },
    {
      "MaxStacks": 10
    },
    {
      "WeaponDamagePerStack": 1.5
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_lash_down_strike" title="Ground Strike" -->

## Ground Strike

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_lash_down_strike`
- Snapshot ID: `39624`
- Source-Dokument: `7070`
- Kurzinfo: Ground Strike aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ground Strike`
- Payload Hash: `cacc7bf37fd3c6a1e52f1d5bc28538853f837c0587f5d89bcfbd0e88ba4f4b62`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.802181+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCooldown": 18.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanCastOnZipline"
  ],
  "ChannelMoveSpeed": -1,
  "DownStrikeModifier": {
    "Class": "Base",
    "Subclass": "LashDownStrikeEmbedded"
  },
  "DragModifier": {
    "Class": "ChargeDragEnemy",
    "ForceDistScale": 10,
    "ForwardOffset": 200,
    "Subclass": "ChargeDragEnemy",
    "VerticalOffset": -200
  },
  "ImpactModifier": {
    "Class": "SlowBase",
    "Subclass": "SlowBase"
  },
  "IsDisabled": false,
  "Key": "citadel_ability_lash_down_strike",
  "MinAimAngle": 60,
  "Name": "Ground Strike",
  "Radius": 10,
  "StompDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.7905
    },
    "Value": 60.0
  },
  "StompDamagePerMeterPrimary": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.04
    },
    "Value": 5.5
  },
  "StompDamagePerMeterSecondary": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.008137
    },
    "Value": 4.2
  },
  "StompDamagePrimaryRange": 25,
  "StompVerticalThreshold": 118,
  "StrikeVelocity": 50,
  "Upgrades": [
    {
      "AbilityCooldown": -10.0
    },
    {
      "EnemySlowPct": 50,
      "SlowDuration": 3,
      "StompBounceHeight": 400,
      "TossDuration": 1
    },
    {
      "StompDamagePerMeterPrimary": {
        "Multiply": true,
        "Scale": {
          "Type": "spirit",
          "Value": 0.03255
        },
        "Value": 2.13
      },
      "StompDamagePerMeterSecondary": {
        "Multiply": true,
        "Scale": {
          "Type": "spirit",
          "Value": 0.008137
        },
        "Value": 2.13
      }
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
    "card_name": "Ground Strike",
    "hero_key": "hero_lash",
    "hero_name": "Lash",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_lash",
      "hero_name": "Lash",
      "lookup": "ground strike",
      "name": "Ground Strike",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_guided_arrow" title="Guided Owl" -->

## Guided Owl

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_guided_arrow`
- Snapshot ID: `39453`
- Source-Dokument: `7070`
- Kurzinfo: Guided Owl aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Guided Owl`
- Payload Hash: `4cca66047aec9e9bae150486f9683af5ede35756fa58aedf2d391f9b66b3c043`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.361684+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1.5,
  "AbilityChannelTime": 20,
  "AbilityCooldown": 125.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BonusTechPowerPerKill": 8,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.93744
    },
    "Value": 230.0
  },
  "ExplosionRadius": 12,
  "GuidingModifier": {
    "Class": "GuidingArrow",
    "EnabledStateMask": [
      "JumpDisabled",
      "ZiplineDisabled",
      "DashDisabled"
    ],
    "GlowEnemeyModifier": {
      "Class": "LowHealthGlow",
      "Subclass": "LowHealthGlow"
    },
    "Subclass": "GuidingArrow"
  },
  "IsDisabled": false,
  "Key": "ability_guided_arrow",
  "KillCheckModifier": {
    "Class": "GuidingArrowKillcheck",
    "Subclass": "KillcheckModifier"
  },
  "Name": "Guided Owl",
  "StunDuration": 0.75,
  "Upgrades": [
    {
      "Damage": 85.0
    },
    {
      "AbilityCooldown": -40.0
    },
    {
      "LowHealthEnemyThresholdPct": 22
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
    "card_name": "Guided Owl",
    "hero_key": "hero_orion",
    "hero_name": "Grey Talon",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_orion",
      "hero_name": "Grey Talon",
      "lookup": "guided owl",
      "name": "Guided Owl",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_priest_knockback" title="Gutshot" -->

## Gutshot

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_knockback`
- Snapshot ID: `39513`
- Source-Dokument: `7070`
- Kurzinfo: Gutshot aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Gutshot`
- Payload Hash: `dc1f69092dec5b8af20d1bf95bb065bede4e4c4c7e41677bdfe7d804ca0d5a35`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.518293+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCastRange": 10,
  "AbilityCooldown": 23,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 99,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusDamage": {
    "Scale": {
      "Type": "weapon_damage_increase",
      "Value": 0.8
    },
    "Value": 30
  },
  "BonusKnockbackDistance": 3.5,
  "BuffModifier": {
    "Class": "PriestKnockbackBuff",
    "Subclass": "Buff"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "weapon_damage_increase",
      "Value": 0.7
    },
    "Value": 60
  },
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "Debuff"
  },
  "IsDisabled": false,
  "Key": "ability_priest_knockback",
  "KnockbackModifier": {
    "Class": "Priestknockback",
    "MomentumMaintained": 0.3,
    "Subclass": "PriestKnockback"
  },
  "KnockbackSpeed": 1200,
  "KnockbackToWallModifier": {
    "Class": "Priestknockback",
    "Subclass": "PriestKnockbacktowall"
  },
  "MaxPushForceHorizontal": 1400,
  "MaxPushForceVertical": 500,
  "Name": "Gutshot",
  "PushForce": 6,
  "SelfPushForce": 500,
  "SlowModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "Slow"
  },
  "StunDuration": 0.6,
  "TargetingConeAngle": 60,
  "Upgrades": [
    {
      "Damage": 25
    },
    {
      "AbilityCooldown": -10,
      "StunDuration": 0.4
    },
    {
      "BuffDuration": 5
    }
  ],
  "WallStunDistance": 7,
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
    "card_name": "Gutshot",
    "hero_key": "hero_priest",
    "hero_name": "Venator",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_priest",
      "hero_name": "Venator",
      "lookup": "gutshot",
      "name": "Gutshot",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_hat_trick" title="Hat Trick" -->

## Hat Trick

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_hat_trick`
- Snapshot ID: `39457`
- Source-Dokument: `7070`
- Kurzinfo: Hat Trick aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hat Trick`
- Payload Hash: `98aba3225017cde8ecfb8fdebe315f68595f9d8ad905041f47afd39a43b957d3`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.374305+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCooldown": 21.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.93744
    },
    "Value": 100
  },
  "DebuffDuration": 5,
  "DebuffModifier": {
    "Class": "SlowBase",
    "EnabledStateMask": [
      "GlowThroughWallsToProvider",
      "SilenceMovementAbilites"
    ],
    "Subclass": "Slow"
  },
  "ExplosionRadius": 2,
  "IsDisabled": false,
  "Key": "ability_hat_trick",
  "Name": "Hat Trick",
  "SlowPercent": 20,
  "Upgrades": [
    {
      "Damage": 50
    },
    {
      "AbilityCooldown": -9.5
    },
    {
      "SlowPercent": 30
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_necro_haunt" title="Haunting Spectres" -->

## Haunting Spectres

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_necro_haunt`
- Snapshot ID: `39497`
- Source-Dokument: `7070`
- Kurzinfo: Haunting Spectres aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Haunting Spectres`
- Payload Hash: `813da49207faf2bc89fe92c0480567be1c647762e6ee1575b3172a79d8ee20e3`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.481788+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 35,
  "AbilityCooldown": 22,
  "AbilityDuration": 5,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorAllowSelfCast",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorAllowAltCast",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorCanSetQuickCast"
  ],
  "BonusMoveSpeed": 3,
  "BuffModifier": {
    "Class": "NecroHauntingspirits",
    "Subclass": "Buff"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.3
    },
    "Value": 25
  },
  "DebuffDuration": 3,
  "DebuffModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "Slow"
  },
  "DetonationRange": 1,
  "HauntCount": 3,
  "HoverRadius": 0.75,
  "IsDisabled": false,
  "Key": "ability_necro_haunt",
  "Name": "Haunting Spectres",
  "PhysicsCurlNoiseFrequency": 0.005,
  "PhysicsCurlNoiseStrength": 0.6,
  "PhysicsDamperStrength": 5,
  "PhysicsSpinPerSecond": 180,
  "PhysicsSpringStrength": 150,
  "SlowPercent": 30,
  "SpawnRadius": 0.2,
  "TargetSearchDelayOnSuccess": 0.3,
  "TargetSearchInitialDelay": 0.7,
  "TargetSearchRadius": 6,
  "TargetSearchRadiusVsHeroes": 15,
  "TargetSearchTick": 0.1,
  "TickRate": 0.1,
  "Upgrades": [
    {
      "Damage": 16
    },
    {
      "FireRateSlow": 30
    },
    {
      "HauntCount": 3
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_rocket_barrage" title="Heavy Barrage" -->

## Heavy Barrage

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_rocket_barrage`
- Snapshot ID: `39636`
- Source-Dokument: `7070`
- Kurzinfo: Heavy Barrage aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Heavy Barrage`
- Payload Hash: `dd5019e6f31103d6bfd188628ea9c420a79eb91524c3917246cd231cce45cd16`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.830399+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 36,
  "AbilityCooldown": 200.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 100,
  "BarrageModifier": {
    "Class": "CitadelRocketBarrageVolley",
    "EnabledStateMask": [
      "AllowDashWhenChanneling",
      "SinclairTaxUltActive",
      "Disarmed"
    ],
    "Subclass": "CitadelRocketBarrageVolley"
  },
  "BehaviourBits": [
    "BehaviorExclusiveUse",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCleaveDisabled",
    "BehaviorDeactivateCrouchToggleOnCast",
    "BehaviorCanSetQuickCast",
    "BehaviorRequireAbilityButtonToCancel",
    "BehaviorDontSwitchAwayOnCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DamagePerRocket": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.2
    },
    "Value": 21
  },
  "DetonateTimer": 5,
  "ExplosionFalloffDisabled": 1,
  "ExplosionRadius": 4.5,
  "GrenadesPerSecond": 6,
  "GroundDashReductionPercent": -35,
  "IntervalRampUpStart": 0.35,
  "IntervalRampUpTime": 0.3,
  "IsDisabled": false,
  "Key": "citadel_ability_rocket_barrage",
  "MaxSpread": 5,
  "MinDistance": 8.5,
  "MoveSlowModifier": {
    "Class": "SlowBase",
    "Subclass": "ForgeRocketBarrageSlow"
  },
  "Name": "Heavy Barrage",
  "ProjectileIgnoreCollisionTime": 0.2,
  "TrackSpeedFar": 100,
  "TrackSpeedNear": 150,
  "TrackingTime": 0.4,
  "Upgrades": [
    {
      "EnemyDashSlowPercent": -18,
      "MoveSlowDuration": 1,
      "MoveSlowPercent": 30
    },
    {
      "AbilityCooldown": -45.0,
      "AbilityDuration": 6
    },
    {
      "DamagePerRocket": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.16
        },
        "Value": 15
      },
      "ExplosionRadius": 2
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
    "card_name": "Heavy Barrage",
    "hero_key": "hero_forge",
    "hero_name": "McGinnis",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_forge",
      "hero_name": "McGinnis",
      "lookup": "heavy barrage",
      "name": "Heavy Barrage",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_gunslinger_demon_carbine" title="Hellfire Salvo" -->

## Hellfire Salvo

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_gunslinger_demon_carbine`
- Snapshot ID: `39454`
- Source-Dokument: `7070`
- Kurzinfo: Hellfire Salvo aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hellfire Salvo`
- Payload Hash: `d9fe10ca4f65c127a5972b40b0eef412e73a2f0213ba44c2191795ea21f31f15`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.364513+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 30.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BaseBulletDamage": {
    "Scale": {
      "Type": "damage",
      "Value": 1.0
    },
    "Value": 5
  },
  "BehaviourBits": [
    "BehaviorDontBreakInvisibility",
    "BehaviorDontInterruptSprint",
    "BehaviorCastableWhileBusy",
    "BehaviorDontInterruptMeleeOnCast",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusBulletSpeed": 100,
  "BulletRadiusOverride": 13.7,
  "BulletTimeScale": 0.01,
  "ChannelMoveSpeed": -1,
  "ChargingModifier": {
    "Class": "GunslingerDemonCarbine",
    "Subclass": "GunslingerDemonCarbine"
  },
  "DebuffModifier": {
    "Class": "ChronoKineticCarbineSlow",
    "StatusEffectPriority": 50,
    "Subclass": "ChronoKineticCarbineSlow"
  },
  "DemonShotCount": 3,
  "HeadshotBonus": 15,
  "IsDisabled": false,
  "Key": "ability_gunslinger_demon_carbine",
  "MaxChargeDuration": 2.5,
  "MoveSpeedWhileShootingPenaltyReduction": 100,
  "Name": "Hellfire Salvo",
  "ProcDamagePercentage": 400,
  "SpeedChange": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0914
    },
    "Value": 25
  },
  "Upgrades": [
    {
      "AbilityCooldown": -15
    }
  ],
  "WeaponReadyDuration": 5,
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_priest_beartrap" title="Hex-Lined Snap Trap" -->

## Hex-Lined Snap Trap

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_beartrap`
- Snapshot ID: `39511`
- Source-Dokument: `7070`
- Kurzinfo: Hex-Lined Snap Trap aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hex-Lined Snap Trap`
- Payload Hash: `b7b088b0bd7d3b9888eb1a493d13c5b71d0d66742bab0d74338786cf115a13c4`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.512987+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCharges": 2,
  "AbilityCooldown": 28,
  "AbilityCooldownBetweenCharge": 8,
  "AbilityUnitTargetLimit": 1,
  "ArmTime": 0.5,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventTrainingBotUsage",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.2
    },
    "Value": 80
  },
  "DebuffModifier": {
    "Class": "PriestBeartrapDebuff",
    "EnabledStateMask": [
      "VisibleToEnemy",
      "GlowThroughWallsToEnemy"
    ],
    "Subclass": "Debuff"
  },
  "ImmobilizeDuration": 1.25,
  "ImmobilizeModifier": {
    "Class": "PriestImmobilize",
    "Subclass": "Immobilize"
  },
  "IsDisabled": false,
  "Key": "ability_priest_beartrap",
  "Lifetime": 30,
  "Name": "Hex-Lined Snap Trap",
  "Radius": 2,
  "RevealDuration": 6,
  "TetherDuration": 0.6,
  "TetherModifier": {
    "Class": "PriestTether",
    "Subclass": "Tether"
  },
  "TetherRadius": 0.3,
  "TickRate": 0.1,
  "TrapHeight": 2,
  "TripUpSpeed": 6.35,
  "Upgrades": [
    {
      "AbilityCooldown": -11
    },
    {
      "ImmobilizeDuration": 1.0
    },
    {
      "AbilityCharges": 1,
      "IncomingDamagePercentFromCaster": 30
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
    "card_name": "Hex-Lined Snap Trap",
    "hero_key": "hero_priest",
    "hero_name": "Venator",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_priest",
      "hero_name": "Venator",
      "lookup": "hex-lined snap trap",
      "name": "Hex-Lined Snap Trap",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="yakuza_kobun" title="Hired Muscle" -->

## Hired Muscle

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `yakuza_kobun`
- Snapshot ID: `39741`
- Source-Dokument: `7070`
- Kurzinfo: Hired Muscle aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hired Muscle`
- Payload Hash: `db69478d97cc10b6aa0877ed6f6fb149f558157fb47c050e7fe28c6fd4d138e9`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.095342+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 30,
  "AbilityCooldown": 32.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAllowSelfCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "CloneModifier": {
    "Class": "HeroClone",
    "Subclass": "HeroClone"
  },
  "IsDisabled": false,
  "Key": "yakuza_kobun",
  "Name": "Hired Muscle",
  "SummonCount": 1,
  "SummonDPS": 60,
  "SummonHealth": 450,
  "SummonLifetime": 45,
  "SummonMoveSpeed": 200,
  "Upgrades": [
    {
      "SummonCasterHealthPct": 20
    },
    {
      "SummonDPS": 45
    },
    {
      "SummonCount": 1
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_yakuza",
      "hero_name": "The Boss",
      "lookup": "hired muscle",
      "name": "Hired Muscle",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_viper_hookdagger" title="Hook Blade" -->

## Hook Blade

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_viper_hookdagger`
- Snapshot ID: `39563`
- Source-Dokument: `7070`
- Kurzinfo: Hook Blade aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hook Blade`
- Payload Hash: `95b703e874742e652f31a37b47780da5813c6e7a7d2396dc32f14208c88ee317`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.644112+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCooldown": 9,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCleaveDisabled"
  ],
  "CatchRadius": 3,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_viper_hookdagger",
  "Name": "Hook Blade",
  "OutgoingDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.279
    },
    "Value": 45.0
  },
  "OutgoingProjectileLifetime": {
    "Scale": {
      "Type": "range",
      "Value": 1.0
    },
    "Value": 0.4
  },
  "ReturnDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.651
    },
    "Value": 90
  },
  "ReturnRadius": 1,
  "ReturnVelocity": 2300,
  "SlowDebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "ViperSlow"
  },
  "SlowDuration": 1,
  "SlowPercent": 35,
  "SpreadAngle": 90,
  "TickRate": 0.01,
  "Upgrades": [
    {
      "AbilityCooldown": -4
    },
    {
      "SlowPercent": 35
    },
    {}
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_familiar_attach_trigger" title="Hop To Ally" -->

## Hop To Ally

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_familiar_attach_trigger`
- Snapshot ID: `39430`
- Source-Dokument: `7070`
- Kurzinfo: Hop To Ally aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hop To Ally`
- Payload Hash: `ff4c10fc549e20988efae640ce8956719635ede8fa88ad4157961af3d573fb65`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.305261+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 30,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontInterruptSprint",
    "BehaviorCastableWhileHidden",
    "BehaviorIgnoreSelectionMashProtection",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorMovement"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_familiar_attach_trigger",
  "Name": "Hop To Ally",
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="tokamak_hot_shot" title="Hot Shot" -->

## Hot Shot

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `tokamak_hot_shot`
- Snapshot ID: `39727`
- Source-Dokument: `7070`
- Kurzinfo: Hot Shot aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hot Shot`
- Payload Hash: `39623f5939354fcef20454c96136fe892e3bef5ccae058aa1bd559cbdb2f41c9`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.061618+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityChannelTime": 1.0,
  "AbilityCharges": 2,
  "AbilityCooldown": 32.0,
  "AbilityCooldownBetweenCharge": 1,
  "AbilityUnitTargetLimit": 1,
  "BeamLength": 30,
  "BeamWidth": 4.0,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanCancelDuringCastDelay",
    "BehaviorShowCastRangeAsSatSphereWhileCasting"
  ],
  "ChannelMoveSpeed": 1.8,
  "HotDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.548402
    },
    "Value": 90
  },
  "IsDisabled": false,
  "Key": "tokamak_hot_shot",
  "Name": "Hot Shot",
  "NormalDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.335135
    },
    "Value": 55
  },
  "TickRate": 0.1,
  "TrackingSpeed": 180,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "AbilityChannelTime": 0.5
    },
    {
      "HotDPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.292481
        },
        "Value": 48
      },
      "NormalDPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.097494
        },
        "Value": 16
      }
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_tokamak",
      "hero_name": "Tokamak",
      "lookup": "hot shot",
      "name": "Hot Shot",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_doorman_hotel" title="Hotel Guest" -->

## Hotel Guest

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_doorman_hotel`
- Snapshot ID: `39422`
- Source-Dokument: `7070`
- Kurzinfo: Hotel Guest aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hotel Guest`
- Payload Hash: `8a559577238853e9bbffab8ca11866211d578bf6ad9b6146b5871ea2328f6cfd`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.280402+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 7,
  "AbilityChannelTime": 1,
  "AbilityCooldown": 140,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6.5,
  "AbilityPostCastDuration": 0.7,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 75
  },
  "DamageModifier": {
    "Class": "Base",
    "Subclass": "DamageTimer"
  },
  "FreezeModifier": {
    "Class": "DoormanHotelTransitionFreeze",
    "Subclass": "Freeze"
  },
  "HotelModifier": {
    "Class": "DoormanHotelVictim",
    "EnabledStateMask": [
      "InvalidTeleportTarget",
      "IgnoreOutOfPlayAreaCheck"
    ],
    "Subclass": "Hotel"
  },
  "HotelTimeScale": null,
  "ImposterModifier": {
    "Class": "DoormanHotelImposter",
    "ImposterModifierFX": {
      "Class": "DoormanHotelImposterFx",
      "Subclass": "Fx"
    },
    "Subclass": "Imposter"
  },
  "IsDisabled": false,
  "Key": "ability_doorman_hotel",
  "LateCheckoutDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.5
    },
    "Value": 125
  },
  "Name": "Hotel Guest",
  "NoDrawModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "OutOfGame",
      "IgnoredByNpcTargeting",
      "UnitStatusHidden",
      "DoNotDrawModel"
    ],
    "Subclass": "Nodraw"
  },
  "PreTeleportModifier": {
    "Class": "Base",
    "Subclass": "PreTeleport"
  },
  "TeleportFXModifier": {
    "Class": "DoormanHotelTeleportFx",
    "Subclass": "TeleportFx"
  },
  "TimeSlowDuration": 1.0,
  "TimeSlowPercentage": 100,
  "TimeslowModifier": {
    "Class": "DoormanDiminishingTimestop",
    "Subclass": "DoormanExitTimeslow"
  },
  "UnstoppableWhileChannelingModifier": {
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
    "Subclass": "DoormanHotelUnstoppable"
  },
  "Upgrades": [
    {
      "AbilityCooldown": -20,
      "StaminaDrain": 1
    },
    {
      "Damage": 150,
      "LateCheckoutDamage": 150,
      "LateCheckoutStun": 1.5
    },
    {
      "LateCheckoutCooldown": 13,
      "UnstoppableWhileHotelOccupied": 1
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
    "card_name": "Hotel Guest",
    "hero_key": "hero_doorman",
    "hero_name": "The Doorman",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_doorman",
      "hero_name": "The Doorman",
      "lookup": "hotel guest",
      "name": "Hotel Guest",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_bebop_laser_beam" title="Hyper Beam" -->

## Hyper Beam

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_bebop_laser_beam`
- Snapshot ID: `39596`
- Source-Dokument: `7070`
- Kurzinfo: Hyper Beam aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hyper Beam`
- Payload Hash: `d6437d0431e9db8b0cd5a72679d2349f1f76cef1fdd67ddd7bda2fe51816a1d9`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.730864+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1.0,
  "AbilityChannelTime": 11,
  "AbilityCooldown": 120.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AirSpeedMax": 70,
  "BeamCloseDamagePercent": 75,
  "BeamCloseRadius": 5.0,
  "BeamEndRadius": 4.0,
  "BeamLength": 70,
  "BeamWidth": 2.9,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": 1.8,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.511
    },
    "Value": 160
  },
  "FallSpeedMax": 1,
  "GroundDashReductionPercent": -40,
  "Interval": 0.1,
  "IsDisabled": false,
  "Key": "citadel_ability_bebop_laser_beam",
  "Name": "Hyper Beam",
  "RestrictionModifier": {
    "Class": "SlowBase",
    "Subclass": "BebopLaserSlow"
  },
  "SlowPercent": 25,
  "SlowTargetDuration": 0.5,
  "TrackingSpeed": 55,
  "Upgrades": [
    {
      "AbilityCooldown": -20.0
    },
    {
      "DPS": 108.0
    },
    {
      "BeamLifesteal": 65,
      "BeamLifestealNonHeroPercent": 20
    }
  ],
  "ZoomBias": 0.5,
  "ZoomTime": 0.1,
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
    "card_name": "Hyper Beam",
    "hero_key": "hero_bebop",
    "hero_name": "Bebop",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_bebop",
      "hero_name": "Bebop",
      "lookup": "hyper beam",
      "name": "Hyper Beam",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_zip_line" title="Hyperline" -->

## Hyperline

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_zip_line`
- Snapshot ID: `39668`
- Source-Dokument: `7070`
- Kurzinfo: Hyperline aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hyperline`
- Payload Hash: `ce2e46d31af866992afe161ce155d00bab59df8984e21efe079f0a7b8a5103d8`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.905506+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontBreakInvisibility",
    "BehaviorDontInterruptSprint",
    "BehaviorNotSilencable",
    "BehaviorNoTarget",
    "BehaviorNonCombat",
    "BehaviorCastableWhileHidden"
  ],
  "ChannelMoveSpeed": -1,
  "DamageCooldown": 3,
  "DismountHorizontalMaxSpeedPercent": 85,
  "DismountHorizontalMinSpeedPercent": 40,
  "DismountVerticalSpeed": 300,
  "IsDisabled": false,
  "Key": "citadel_ability_zip_line",
  "KnockedOffDamagePct": 15,
  "KnockedOffSlowModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "KnockedOffZiplineSlow"
  },
  "LatchEndSpeed": 750,
  "LatchInitialSpeed": 600,
  "LatchMaxTime": 0.5,
  "LatchSpeed": 1500,
  "LatchVisualSnapProgress": 0.85,
  "MaxMountDistance2D": 15,
  "Name": "Hyperline",
  "PlayerSpeedCheckScale": 0.55,
  "RegenZoneDismountHorizontalMaxSpeed": 300,
  "RidingZipLineModifier": {
    "Class": "Base",
    "Subclass": "RidingZipline"
  },
  "SlowDuration": 8,
  "StunDuration": 2.5,
  "Upgrades": [],
  "ZipAcc": 1000,
  "ZipLineIntroModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "ZiplineIntro",
      "ZiplineLocked"
    ],
    "Subclass": "ZiplineIntro"
  },
  "ZipLineKnockdownImmuneModifier": {
    "Class": "ZiplineKnockdownImmune",
    "Subclass": "ZiplineKnockdownImmune"
  },
  "ZipLineSlowModifier": {
    "Class": "ZiplineSpeed",
    "PercentageMultiplierEnd": 0,
    "PercentageMultiplierStart": -70,
    "RampUpTime": 2.0,
    "Subclass": "ZiplineProtectionSlow"
  },
  "ZipSpeedInner": 693,
  "ZipSpeedOuter": 810,
  "ZiplineProtectionDamageAmp": 35,
  "ZiplineProtectionSlowDurationOnHit": 2.0,
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_icepath" title="Ice Path" -->

## Ice Path

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_icepath`
- Snapshot ID: `39464`
- Source-Dokument: `7070`
- Kurzinfo: Ice Path aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ice Path`
- Payload Hash: `f3cb6c5646a729dd2cefee7650d1de31f717405741e0e64336c9b581cbcd092c`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.392096+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 50.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontInterruptSprint",
    "BehaviorCastableWhileBusy",
    "BehaviorInterruptMeleeOnCast",
    "BehaviorNoTarget",
    "BehaviorPreventBotUsage",
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast",
    "BehaviorRequireAbilityButtonToCancel"
  ],
  "CameraDistance": 250,
  "ChannelMoveSpeed": -1,
  "IcePathAuraDuration": 18,
  "IcePathEdgeWidth": 0.7,
  "IcePathInterval": 0.5,
  "IcePathModifier": {
    "BonusSpiritLingerModifier": {
      "Class": "IcepathTechPowerLinger",
      "Subclass": "IcepathTechPowerLinger"
    },
    "Class": "Icepath",
    "EnabledStateMask": [
      "AbilityMovement"
    ],
    "FriendlyAuraModifier": {
      "Class": "BaseAura",
      "ProvidedByAura": {
        "Class": "IcepathFriendlyModifier",
        "Subclass": "IcepathBuff"
      },
      "Subclass": "IcepathFriendlyAura"
    },
    "Subclass": "Icepath"
  },
  "IcePathPullInStrength": 20,
  "IcePathShardRadius": 1.2,
  "IsDisabled": false,
  "Key": "ability_icepath",
  "MinHeight": 20,
  "ModifierRadius": 5,
  "MoveSpeedBonus": 2,
  "MoveWhileShootingSpeedPenaltyReductionPercent": 100,
  "MoveWhileZoomedSpeedPenaltyReductionPercent": 100,
  "Name": "Ice Path",
  "PopupForce": 30,
  "SlideScale": 50,
  "SlowResistancePercent": 60,
  "SprintSpeedBonus": 2,
  "Upgrades": [
    {
      "BulletResist": 35,
      "MoveSpeedBonus": 2
    },
    {
      "AbilityCooldown": -25.0
    },
    {
      "BonusSpirit": 20,
      "BonusSpiritPct": 35
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
    "card_name": "Ice Path",
    "hero_key": "hero_kelvin",
    "hero_name": "Kelvin",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_kelvin",
      "hero_name": "Kelvin",
      "lookup": "ice path",
      "name": "Ice Path",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="gunslinger_demonMark" title="Infernal Brand" -->

## Infernal Brand

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `gunslinger_demonMark`
- Snapshot ID: `39684`
- Source-Dokument: `7070`
- Kurzinfo: Infernal Brand aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Infernal Brand`
- Payload Hash: `e0f36c22614bdadd806bf11080110cf465de3a69c5de281c93668c543f665c89`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.943994+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 40,
  "AbilityCooldown": 14,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "BonusFireRate": 25,
  "BonusMoveSpeed": 2,
  "BuffDuration": 3,
  "Damage": {
    "Scale": {
      "Type": "weapon_damage",
      "Value": 0.93
    },
    "Value": 100
  },
  "IsDisabled": false,
  "Key": "gunslinger_demonMark",
  "MarkDuration": 5,
  "MarkModifier": {
    "BuffModifier": {
      "Class": "Base",
      "Subclass": "DemonmarkBuff"
    },
    "Class": "GunslingerDemonmark",
    "Subclass": "Demonmark"
  },
  "Name": "Infernal Brand",
  "ProcDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.744
    },
    "Value": 100
  },
  "SearchAngle": 20,
  "SearchRadius": 20,
  "SearchRate": 1,
  "Upgrades": [
    {
      "AbilityCooldown": -3
    },
    {
      "ProcDamage": 50
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
    "card_name": "Infernal Brand",
    "hero_key": "hero_gunslinger",
    "hero_name": "Gunslinger",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_gunslinger",
      "hero_name": "Gunslinger",
      "lookup": "infernal brand",
      "name": "Infernal Brand",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_passive_beefy" title="Infernal Resilience" -->

## Infernal Resilience

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_passive_beefy`
- Snapshot ID: `39632`
- Source-Dokument: `7070`
- Kurzinfo: Infernal Resilience aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Infernal Resilience`
- Payload Hash: `cfb85ef4fc0f2a6db3da1d844873fd62b9cea69746899ae48e976acc754c50be`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.820597+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "Class": "InfernalResilienceMelee",
      "Subclass": "InfernalResilienceMelee"
    },
    {
      "Class": "IntrinsicBase",
      "Subclass": "IntrinsicBase"
    }
  ],
  "BehaviourBits": null,
  "BonusHealthRegen": 1,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "citadel_ability_passive_beefy",
  "Name": "Infernal Resilience",
  "NonHeroHealPct": 40,
  "RegenDamageInterval": 1.0,
  "RegenIncomingDamageDuration": 20,
  "RegenIncomingDamagePercent": 13,
  "Upgrades": [
    {
      "BonusMaxHealth": 200
    },
    {
      "MeleeLifesteal": 18
    },
    {
      "RegenIncomingDamagePercent": 8,
      "StatusResistancePercent": 20
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
    "card_name": "Infernal Resilience",
    "hero_key": "hero_atlas",
    "hero_name": "Abrams",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_atlas",
      "hero_name": "Abrams",
      "lookup": "infernal resilience",
      "name": "Infernal Resilience",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_boho_damageshare" title="Intertwine" -->

## Intertwine

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_boho_damageshare`
- Snapshot ID: `39402`
- Source-Dokument: `7070`
- Kurzinfo: Intertwine aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Intertwine`
- Payload Hash: `62240ed2e68fab80f461a53b10540dd4c939dba4233b069500eafe1aab2f6414`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.235035+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "DamageShareModifier": {
    "Class": "BohoDamageshare",
    "Subclass": "Damageshare"
  },
  "DamageSharePercentage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.186
    },
    "Value": 30
  },
  "DamageShareRadius": 8,
  "DebuffDuration": 6,
  "IsDisabled": false,
  "Key": "ability_boho_damageshare",
  "LinkDuration": 0.5,
  "MaxLinks": 6,
  "Name": "Intertwine",
  "TickRate": 0.25,
  "Upgrades": [
    {
      "DamageShareRadius": 3
    },
    {
      "AbilityDuration": 3
    },
    {
      "DamageSharePercentage": 22.5
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
    "card_name": "Intertwine",
    "hero_key": "hero_boho",
    "hero_name": "Boho",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_boho",
      "hero_name": "Boho",
      "lookup": "intertwine",
      "name": "Intertwine",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_priest_weaponswap" title="Ira Domini" -->

## Ira Domini

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_weaponswap`
- Snapshot ID: `39518`
- Source-Dokument: `7070`
- Kurzinfo: Ira Domini aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ira Domini`
- Payload Hash: `049b98b731cd7813d1424706929cf60cf9ab2f53622eec95dbd4891214443381`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.528340+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.3,
  "AbilityCooldown": 160,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 15,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorDontInterruptSlideOnCast",
    "BehaviorRefundHalfCooldownOnChannelInterrupt"
  ],
  "BonusAmpToVampire": 5,
  "BonusDamage": {
    "Scale": {
      "Type": "power_increase",
      "Value": 3.0
    },
    "Value": 100
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "weapon_damage_increase",
      "Value": 1.5
    },
    "Value": 120
  },
  "ExecuteThreshold": 8,
  "ExplodeRadius": 0.2,
  "IsDisabled": false,
  "Key": "ability_priest_weaponswap",
  "Name": "Ira Domini",
  "PushForce": 500,
  "SelfModifier": {
    "Class": "PriestCrossbowequipped",
    "EnabledStateMask": [
      "AmmoChangesDisabled",
      "SinclairTaxUltActive"
    ],
    "Subclass": "Self"
  },
  "SlowModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "BarrageSlowModifier"
  },
  "StakeCount": 3,
  "SwapEndDelay": 0.6,
  "Upgrades": [
    {
      "BonusMoveSpeed": 1.2
    },
    {
      "AbilityCooldown": -15,
      "BonusDamage": 65
    },
    {
      "AllStakesBlessed": 1
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
    "card_name": "Ira Domini",
    "hero_key": "hero_priest",
    "hero_name": "Venator",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_priest",
      "hero_name": "Venator",
      "lookup": "ira domini",
      "name": "Ira Domini",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_fencer_ultimate" title="Itani Lo Sahn" -->

## Itani Lo Sahn

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_fencer_ultimate`
- Snapshot ID: `39436`
- Source-Dokument: `7070`
- Kurzinfo: Itani Lo Sahn aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Itani Lo Sahn`
- Payload Hash: `a99120a97a6a1cc8de8cc853c53a36bdb3ec8f0f93b5065ad8e3ab5d7b719b3a`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.319945+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1.5,
  "AbilityChannelTime": 9999,
  "AbilityCooldown": 145,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "AirSpeedMax": 70,
  "AutoCastDelayModifier": {
    "Class": "Base",
    "Subclass": "FencerUltCastDelay"
  },
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorNoTarget",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusDamagePercent": 60,
  "CameraDistance": 250,
  "CasterArrivalModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "SinclairTaxUltActive",
      "HideCrosshair",
      "HideStamina",
      "HideAmmo",
      "CameraTransitionAlways"
    ],
    "Subclass": "FencerUltimateCasterArrival"
  },
  "CasterLockDuration": 1.8,
  "CasterModifier": {
    "Class": "FencerUltimateCaster",
    "EnabledStateMask": [
      "SinclairTaxUltActive",
      "HideCrosshair",
      "HideStamina",
      "HideAmmo",
      "CameraTransitionAlways"
    ],
    "Subclass": "FencerUltimateCaster"
  },
  "DashAngleThreshold": 89,
  "DashRadius": 7,
  "DashRange": 27,
  "DashSpeed": 254.0,
  "DebuffDuration": 1.8,
  "DelayedDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.6
    },
    "Value": 200
  },
  "FallSpeedMax": 1,
  "GapDistanceToWall": 180,
  "GroundDashReductionPercent": -30,
  "ImpactDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.77
    },
    "Value": 70
  },
  "IncomingDamageReductionPercent": 70,
  "IsDisabled": false,
  "Key": "ability_fencer_ultimate",
  "LowHealthEnemyThresholdPct": 50,
  "MoveSpeedPenaltyMaxSpeed": 200,
  "Name": "Itani Lo Sahn",
  "SideMoveSpeedReduction": -100,
  "TargetModifier": {
    "Class": "FencerUltimateTarget",
    "DamageTimeOffset": 0.3,
    "EnabledStateMask": [
      "ModNoCleanse",
      "HealingDisabled",
      "Disarmed",
      "Silenced",
      "Muted",
      "DashDisabledDebuff",
      "SilenceMovementAbilites"
    ],
    "EndTimeScaleForFlinch": 0.9,
    "StatusEffectPriority": 50,
    "Subclass": "Cursed"
  },
  "TargetNonHeroModifier": {
    "Class": "FencerUltimateTarget",
    "DamageTimeOffset": 0.3,
    "EnabledStateMask": [
      "HealingDisabled",
      "Disarmed",
      "Silenced",
      "Muted",
      "DashDisabledDebuff",
      "SilenceMovementAbilites"
    ],
    "EndTimeScaleForFlinch": 0.9,
    "StatusEffectPriority": 50,
    "Subclass": "CursedNonHero"
  },
  "TechCleaveExpireTime": 0.35,
  "TimeScaleDebuff": 70,
  "TimerSoundDuration": 1,
  "TravelDistPctBeforeWallGapCheck": 70,
  "TurnRateMaxDuringCast": 999,
  "Upgrades": [
    {
      "DashRange": 8
    },
    {
      "AbilityCooldown": -35
    },
    {
      "BonusDamagePercent": 50
    }
  ],
  "VacuumSpeed": 10.16,
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
    "card_name": "Itani Lo Sahn",
    "hero_key": "hero_fencer",
    "hero_name": "Apollo",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_fencer",
      "hero_name": "Apollo",
      "lookup": "itani lo sahn",
      "name": "Itani Lo Sahn",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_necro_hauntingskull" title="Jar of Dead" -->

## Jar of Dead

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_necro_hauntingskull`
- Snapshot ID: `39498`
- Source-Dokument: `7070`
- Kurzinfo: Jar of Dead aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Jar of Dead`
- Payload Hash: `adeadeba6368553eacaedde525d6e8f1336d23fd0cd6459d8b38c84780d391d3`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.483938+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCharges": 4,
  "AbilityChargesConditionally": 1,
  "AbilityCooldownBetweenCharge": 13,
  "AbilityUnitTargetLimit": 1,
  "AreaModifier": {
    "Class": "NecroHauntingskullArea",
    "InitialRandomVariance": 30.0,
    "SpawnPositionNavMeshSearchRange": 60.0,
    "Subclass": "Area"
  },
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.25
    },
    "Value": 16
  },
  "DelayBeforeRespawning": 1,
  "HealPerPickup": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "IsDisabled": false,
  "Key": "ability_necro_hauntingskull",
  "KillTime": 0.2,
  "MaxHits": -1,
  "Name": "Jar of Dead",
  "PickupsPerBossDeath": 5,
  "PickupsPerDeath": 1,
  "PickupsPerHeroDeath": 5,
  "PickupsPerNeutralTrooperDeath": 2,
  "ResourceCost": {
    "Scale": {
      "Type": "cooldown",
      "Value": 1.0
    },
    "Value": 120
  },
  "ResourceGenerationPercent": 100,
  "ResourcePerPickup": 10,
  "ResourceRadius": 40,
  "SkullCount": 4,
  "SkullImmuneDuration": 0.15,
  "SkullKillGold": {
    "Scale": {
      "Type": "power_increase",
      "Value": 0.5
    },
    "Value": 7
  },
  "SkullLifetime": 10,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "SpawnRadius": 2,
  "StackingDebuffModifier": {
    "Class": "NecroHauntingskullStackingdebuff",
    "Subclass": "Debuff"
  },
  "SummonBuffModifier": {
    "Class": "Base",
    "Subclass": "Summonbuff"
  },
  "SummonHealth": {
    "Scale": {
      "Type": "power_increase",
      "Value": 1.3
    },
    "Value": 20
  },
  "SummonModifier": {
    "Class": "BarrierTracker",
    "EnabledStateMask": [
      "IgnoredByNpcTargeting",
      "IsSmallDeployable"
    ],
    "Subclass": "Barriertracker"
  },
  "SummonTakesDamage": 1,
  "TargetDashRadius": {
    "Scale": {
      "Type": "range",
      "Value": 1.0
    },
    "Value": 15
  },
  "TargetSearchDelayMax": 1.25,
  "TargetSearchDelayMin": 1.5,
  "TargetSearchInitialDelayMax": 0.2,
  "TargetSearchInitialDelayMin": 0.15,
  "TargetSearchInitialStagger": 0.125,
  "TargetSearchRadius": 7,
  "TickRate": 0.2,
  "Upgrades": [
    {
      "HealPerPickup": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.1
        },
        "Value": 5
      }
    },
    {
      "SlowDuration": 1,
      "SlowPercent": 30
    },
    {
      "SkullCount": 2,
      "SkullLifetime": 4
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
    "card_name": "Jar of Dead",
    "hero_key": "hero_necro",
    "hero_name": "Graves",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_necro",
      "hero_name": "Graves",
      "lookup": "jar of dead",
      "name": "Jar of Dead",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_swan_leap" title="Jeté" -->

## Jeté

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_swan_leap`
- Snapshot ID: `39539`
- Source-Dokument: `7070`
- Kurzinfo: Jeté aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Jeté`
- Payload Hash: `2f54c1af541f25295d37dcbcb40e4c9b5635c8154e09e7ea8054f957ffc42f43`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.580298+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": 0.15,
  "AbilityCooldown": 14,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel"
  ],
  "BuffDuration": 7,
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "Leapmodifier"
  },
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "JumpPitch": -30,
  "JumpSpeed": 20,
  "Key": "ability_swan_leap",
  "Name": "Jeté",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Jeté",
    "hero_key": "hero_swan",
    "hero_name": "Swan",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_swan",
      "hero_name": "Swan",
      "lookup": "jeté",
      "name": "Jeté",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_jump" title="Jump" -->

## Jump

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_jump`
- Snapshot ID: `39620`
- Source-Dokument: `7070`
- Kurzinfo: Jump aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Jump`
- Payload Hash: `6e15633789659dd567b62292635e67e455546154a0fbc829c5e9e9cd05c3face`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.791935+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 0.15,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AirJumpVerticalSpeedPercent": 75,
  "BehaviourBits": [
    "BehaviorHidden",
    "BehaviorDontBreakInvisibility",
    "BehaviorSilentCastFailureFeedback",
    "BehaviorDontInterruptSprint",
    "BehaviorInputDirectional2d",
    "BehaviorNotSilencable",
    "BehaviorNoTarget",
    "BehaviorNonCombat",
    "BehaviorCastableWhileHidden"
  ],
  "ChannelMoveSpeed": -1,
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "WallJumpStaminaRegenReduction"
  },
  "IsDisabled": false,
  "Key": "citadel_ability_jump",
  "Name": "Jump",
  "SlideLeapSpeedPenaltyMax": 100,
  "SlideLeapSpeedPenaltyTime": 0.2,
  "StaminaRegenReduction": -25,
  "StaminaRegenReductionDuration": 5,
  "Upgrades": [],
  "VerticalSpeed": 300,
  "WeaponSpreadPenalty": 3,
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_frank_selfzap" title="Jumpstart" -->

## Jumpstart

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_frank_selfzap`
- Snapshot ID: `39445`
- Source-Dokument: `7070`
- Kurzinfo: Jumpstart aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Jumpstart`
- Payload Hash: `5e0f45495698de3424dbb0fa387ce006957dc6edab4516a756ac299643dcaae4`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.342689+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": 0.35,
  "AbilityCharges": 1,
  "AbilityCooldown": 30,
  "AbilityCooldownBetweenCharge": 8,
  "AbilityDuration": 4.5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusMoveSpeed": 3,
  "BuffModifier": {
    "Class": "FrankSelfzap",
    "Subclass": "Selfzap"
  },
  "ChannelMoveSpeed": -1,
  "CurrentHealthPercentDamage": 15,
  "IsDisabled": false,
  "Key": "ability_frank_selfzap",
  "Name": "Jumpstart",
  "TotalHealthRegen": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.2
    },
    "Value": 100
  },
  "Upgrades": [
    {
      "BonusMoveSpeed": 3
    },
    {
      "AbilityCooldown": -8,
      "TotalHealthRegen": 70
    },
    {
      "AbilityCharges": 1,
      "StatusResistancePercent": 50,
      "TotalHealthRegen": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.9
        },
        "Value": 0
      }
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
    "card_name": "Jumpstart",
    "hero_key": "hero_frank",
    "hero_name": "Victor",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_frank",
      "hero_name": "Victor",
      "lookup": "jumpstart",
      "name": "Jumpstart",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_shiv_killing_blow" title="Killing Blow" -->

## Killing Blow

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shiv_killing_blow`
- Snapshot ID: `39642`
- Source-Dokument: `7070`
- Kurzinfo: Killing Blow aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Killing Blow`
- Payload Hash: `9a6c0b7d4ce00c167735c520275475e6cbbdba801d9676b90379674ce9a67eaf`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.845017+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.05,
  "AbilityCastRange": 12,
  "AbilityCooldown": 145.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.15,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorMovement",
    "BehaviorCanSetQuickCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BonusAbilityResource": 10,
  "BuffDamage": 8,
  "CameraDistance": 400,
  "ChannelMoveSpeed": -1,
  "Damage": 200,
  "EnemyHealthPercent": 20,
  "EnemyHealthPercentBuffer": 3,
  "FailedExecuteCooldownPenalty": 30,
  "IsDisabled": false,
  "Key": "citadel_ability_shiv_killing_blow",
  "KillableModifier": {
    "Class": "KillingBlowGlow",
    "Subclass": "KillingBlowGlow"
  },
  "LeapModifier": {
    "Class": "CitadelShivKillingblowLeap",
    "Subclass": "CitadelShivKillingblowLeap"
  },
  "MinTimeToTarget": 0.5,
  "MoveSpeedToTarget": 30,
  "Name": "Killing Blow",
  "RageDrainDelayDuration": 12,
  "RageDrainRate": 0.25,
  "RagePerHeavyMelee": 2.85384,
  "RagePerLightMelee": 1.55664,
  "RagePerSpiritDamage": 0.01452864,
  "RagePerWeaponDamage": 0.0158766,
  "RecastWindow": 20,
  "SlashRange": 90,
  "Upgrades": [
    {
      "AbilityCastRange": 6,
      "BonusMoveSpeed": 2
    },
    {
      "AbilityCooldown": -25,
      "BuffDamage": 16
    },
    {
      "EnemyHealthPercent": 8
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
    "card_name": "Killing Blow",
    "hero_key": "hero_shiv",
    "hero_name": "Shiv",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_shiv",
      "hero_name": "Shiv",
      "lookup": "killing blow",
      "name": "Killing Blow",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_chrono_kinetic_carbine" title="Kinetic Carbine" -->

## Kinetic Carbine

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chrono_kinetic_carbine`
- Snapshot ID: `39603`
- Source-Dokument: `7070`
- Kurzinfo: Kinetic Carbine aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Kinetic Carbine`
- Payload Hash: `214bcc106b43c8f989afb0c67c84e70eb3933af5977d8252d62fece2725d4f54`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.750693+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 28.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AirMoveIncreasePercent": 20,
  "BehaviourBits": [
    "BehaviorDontBreakInvisibility",
    "BehaviorDontInterruptSprint",
    "BehaviorCastableWhileBusy",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusBulletSpeed": 100,
  "BulletRadiusOverride": 16.0,
  "BulletTimeScale": 0.01,
  "ChannelMoveSpeed": -1,
  "ChargingModifier": {
    "Class": "ChronoKineticCarbine",
    "Subclass": "ChronoKineticCarbine"
  },
  "DebuffModifier": {
    "Class": "ChronoKineticCarbineSlow",
    "StatusEffectPriority": 50,
    "Subclass": "ChronoKineticCarbineSlow"
  },
  "HeadshotBonus": 14,
  "IsDisabled": false,
  "Key": "citadel_ability_chrono_kinetic_carbine",
  "MaxBonusBulletDamage": {
    "Scale": {
      "Type": "weapon_power",
      "Value": 125
    },
    "Value": 5
  },
  "MaxChargeDuration": 2.5,
  "MaxSlowDuration": 0.4,
  "MinBonusBulletDamage": {
    "Scale": {
      "Type": "weapon_power",
      "Value": 25
    },
    "Value": 5
  },
  "MinSlowDuration": 0.25,
  "MoveSpeedWhileShootingPenaltyReduction": 100,
  "Name": "Kinetic Carbine",
  "ProjectileTimeScale": 0.01,
  "ShotCount": 1,
  "SpeedBoostDuration": 3.5,
  "SpeedChange": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.13
    },
    "Value": 25
  },
  "TimeScaleDebuff": 90,
  "TimeWarpRadius": 5,
  "Upgrades": [
    {
      "MaxSlowDuration": 0.4
    },
    {
      "AbilityCooldown": -12,
      "SpeedChange": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.06
        },
        "Value": 0
      }
    },
    {
      "MaxBonusBulletDamage": {
        "Scale": {
          "Type": "weapon_power",
          "Value": 55
        },
        "Value": 0
      },
      "MinBonusBulletDamage": {
        "Scale": {
          "Type": "weapon_power",
          "Value": 55
        },
        "Value": 0
      },
      "SpeedBoostDuration": 2
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
    "card_name": "Kinetic Carbine",
    "hero_key": "hero_chrono",
    "hero_name": "Paradox",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_chrono",
      "hero_name": "Paradox",
      "lookup": "kinetic carbine",
      "name": "Kinetic Carbine",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_stomp" title="Kinetic Pulse" -->

## Kinetic Pulse

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_stomp`
- Snapshot ID: `39648`
- Source-Dokument: `7070`
- Kurzinfo: Kinetic Pulse aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Kinetic Pulse`
- Payload Hash: `f34035c7769f4d7bf4b8d9cc1e5adc246fc6a747ab8f233558b82723751f2af7`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.858436+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.42,
  "AbilityCharges": 1,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": 5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BulletResistModifier": {
    "Class": "StompDebuff",
    "Subclass": "StompDebuff"
  },
  "ChannelMoveSpeed": 1.3,
  "ClimbHeight": 1.0,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.55
    },
    "Value": 115.0
  },
  "DebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "DebuffModifier"
  },
  "DistanceAboveGround": 1.0,
  "DropDownRate": 20,
  "ImpactInterval": 0.1,
  "IsDisabled": false,
  "Key": "citadel_ability_stomp",
  "Name": "Kinetic Pulse",
  "StompRange": 16,
  "StompWidth": 5.5,
  "TechCleaveExpireTime": 0.2,
  "TossDuration": 1,
  "TossSpeed": 450,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "BulletResistReduction": -15,
      "SlowDuration": 4,
      "SlowPercent": 30
    },
    {
      "Damage": 135,
      "StompRange": 20
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
    "card_name": "Kinetic Pulse",
    "hero_key": "hero_dynamo",
    "hero_name": "Dynamo",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_dynamo",
      "hero_name": "Dynamo",
      "lookup": "kinetic pulse",
      "name": "Kinetic Pulse",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_tangotether" title="Kudzu Connection" -->

## Kudzu Connection

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tangotether`
- Snapshot ID: `39650`
- Source-Dokument: `7070`
- Kurzinfo: Kudzu Connection aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Kudzu Connection`
- Payload Hash: `148aa34ebd9d7970d081a1993e07b18db9a123c607452bafdfb14f86d62d75b9`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.863606+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 16,
  "AbilityCooldown": 37.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 12,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusFireRate": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.18
    },
    "Value": 10
  },
  "BulletLifestealPercent": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.15
    },
    "Value": 15
  },
  "ChannelMoveSpeed": -1,
  "GrappleTargetModifier": {
    "Class": "CitadelModifierTangotetherTarget",
    "Subclass": "Target"
  },
  "HealingPerGlub": 20,
  "IsDisabled": false,
  "Key": "citadel_ability_tangotether",
  "MoveWhileShootingSpeedPenaltyReductionPercent": 100,
  "MoveWhileZoomedSpeedPenaltyReductionPercent": 100,
  "Name": "Kudzu Connection",
  "TetherModifier": {
    "BuffModifier": {
      "Class": "CitadelModifierTangotetherTetherReceiver",
      "EnabledStateMask": [
        "CoopTetherActive"
      ],
      "Subclass": "CitadelModifierTangotetherTetherReceiver"
    },
    "CandidateCloserDistance": 400.0,
    "Class": "CitadelModifierTangotetherTether",
    "DisconnectDistanceBuffer": 100.0,
    "LockedTargetModifier": {
      "Class": "Base",
      "EnabledStateMask": [
        "CoopTetherLockedTarget"
      ],
      "Subclass": "TetherLockedModifier"
    },
    "MinConnectTime": 2.0,
    "NoConnectionModifier": {
      "Class": "TetherNoConnection",
      "Subclass": "TetherNoConnection"
    },
    "StatusEffectPriority": 50,
    "Subclass": "CitadelModifierTangotetherTether",
    "TargetAwayDistance": 250.0
  },
  "TetherSharedHealPct": {
    "Scale": {
      "Type": "power_increase",
      "Value": 0.85
    },
    "Value": 35
  },
  "TickRate": 0.1,
  "TotalTetherTargets": 1,
  "Upgrades": [
    {
      "MoveSpeedBonus": 2
    },
    {
      "BonusFireRate": 8,
      "BulletLifestealPercent": 8
    },
    {
      "AbilityCooldown": -37,
      "AbilityDuration": -13
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
    "card_name": "Kudzu Connection",
    "hero_key": "hero_tengu",
    "hero_name": "Ivy",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_tengu",
      "hero_name": "Ivy",
      "lookup": "kudzu connection",
      "name": "Kudzu Connection",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_tier2boss_laser_beam" title="Laser" -->

## Laser

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tier2boss_laser_beam`
- Snapshot ID: `39656`
- Source-Dokument: `7070`
- Kurzinfo: Laser aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Laser`
- Payload Hash: `4247ff9a32d11056a39f3ac49942db6b73bc78f4bec6822165a2a59cfb6b70fc`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.876811+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1.0,
  "AbilityCastRange": 34.92,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget"
  ],
  "ChannelMoveSpeed": -1,
  "CreepDPS": 190,
  "DPS": 125,
  "IsDisabled": false,
  "Key": "citadel_ability_tier2boss_laser_beam",
  "MaxHPDPS": 2,
  "Name": "Laser",
  "PathLength": 1250,
  "PathWidth": 30,
  "SweepSpeed": 1,
  "TrackingSpeed": 30,
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_warden_riot_protocol" title="Last Stand" -->

## Last Stand

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_warden_riot_protocol`
- Snapshot ID: `39572`
- Source-Dokument: `7070`
- Kurzinfo: Last Stand aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Last Stand`
- Payload Hash: `31780cc8096af01913633f0ae99c4ea1648b8b801cd537beb33f480fc012f47f`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.670227+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 2,
  "AbilityCooldown": 180.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "AutoCastDelayModifier": {
    "Class": "WardenRiotProtocolCastdelay",
    "Subclass": "WardenRiotProtocolCastDelay",
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
    }
  },
  "BehaviourBits": [
    "BehaviorExclusiveUse",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BulletResist": 50,
  "ConeAngle": 115,
  "HealthStealPct": 10,
  "HealthStealPctHero": 75,
  "IsDisabled": false,
  "Key": "ability_warden_riot_protocol",
  "Name": "Last Stand",
  "PulseDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.3
    },
    "Value": 70
  },
  "PulseInterval": 0.5,
  "Radius": 12,
  "TechResist": 50,
  "Upgrades": [
    {
      "Radius": 4
    },
    {
      "AbilityCooldown": -30.0,
      "PulseDPS": 40.5
    },
    {
      "AbilityDuration": 4,
      "BulletResist": 30,
      "TechResist": 30,
      "UnstoppableCastDelay": 1
    }
  ],
  "WardenBuffModifier": {
    "Class": "WardenRiotProtocol",
    "EnabledStateMask": [
      "SinclairTaxUltActive",
      "SinclairTaxKeepModelSwap"
    ],
    "EnemyDebuffModifier": {
      "Class": "WardenRiotProtocolEnemyDebuff",
      "Subclass": "WardenRiotProtocolEnemyDebuff"
    },
    "Subclass": "WardenRiotProtocol"
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
    "card_name": "Last Stand",
    "hero_key": "hero_warden",
    "hero_name": "Warden",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_warden",
      "hero_name": "Warden",
      "lookup": "last stand",
      "name": "Last Stand",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_nano_dash" title="Leaping Slash" -->

## Leaping Slash

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_nano_dash`
- Snapshot ID: `39488`
- Source-Dokument: `7070`
- Kurzinfo: Leaping Slash aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Leaping Slash`
- Payload Hash: `764e9f918e71526d5c777a45177d99902c4538a3e7b23702161e89617700042b`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.460616+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 9,
  "AbilityCooldown": 13,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BountyModifier": {
    "Class": "NanoBounty",
    "Subclass": "Bounty"
  },
  "CameraDistance": 550,
  "ChannelMoveSpeed": -1,
  "DashAngleThreshold": 89,
  "DashModifier": {
    "Class": "CitadelShivDash",
    "Subclass": "CitadelShivDash"
  },
  "DashRadius": 2.0,
  "DashSpeed": 60.96,
  "HealAmount": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.4
    },
    "Value": 40
  },
  "ImpactDamage": {
    "Scale": {
      "Type": "melee",
      "Value": 0.8
    },
    "Value": 10.0
  },
  "IsDisabled": false,
  "Key": "ability_nano_dash",
  "MoveSpeedPenaltyMaxSpeed": 200,
  "Name": "Leaping Slash",
  "PostDashMaintainedVelocityRatio": 0.15,
  "SideMoveSpeedReduction": -90,
  "SlashForwardOffset": 1.5,
  "SlashHeight": 2.5,
  "SlashRadius": 4,
  "Upgrades": [
    {
      "HealAmount": 25
    },
    {
      "BonusGoldOnKill": 200,
      "BountyDuration": 3
    },
    {
      "CooldownRefundPercent": 50,
      "ImpactDamage": 60
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
    "card_name": "Leaping Slash",
    "hero_key": "hero_nano",
    "hero_name": "Calico",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_nano",
      "hero_name": "Calico",
      "lookup": "leaping slash",
      "name": "Leaping Slash",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_viper_venom" title="Lethal Venom" -->

## Lethal Venom

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_viper_venom`
- Snapshot ID: `39568`
- Source-Dokument: `7070`
- Kurzinfo: Lethal Venom aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lethal Venom`
- Payload Hash: `ad8e1a45c522be6ba99f6bd02d08a768c3260e7b82a48417605dbe38fa1a7221`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.656742+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Scale": {
      "Type": "power_increase",
      "Value": 0.2
    },
    "Value": 10
  },
  "AbilityCooldown": 28.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "Class": "CitadelViperVenomProcWatcher",
      "Subclass": "VenomProcWatcher"
    }
  ],
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "BuildUpDuration": 5,
  "BuildUpModifier": {
    "BuildUpDecayDelay": 2.0,
    "Class": "CitadelBaseBuildup",
    "Subclass": "CitadelBaseBuildup"
  },
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_viper_venom",
  "Name": "Lethal Venom",
  "Upgrades": [
    {
      "VenomMaxDamage": 31.5
    },
    {
      "AbilityCooldown": -12,
      "HealAmpReceivePenaltyPercent": -40,
      "HealAmpRegenPenaltyPercent": -40
    },
    {
      "BuildUpPerShot": 4.5
    }
  ],
  "VenomBuildupPerShot": 1,
  "VenomDuration": 3,
  "VenomMaxDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.79
    },
    "Value": 140
  },
  "VenomMaxDamageHealthPercentage": 30,
  "VenomMinDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.651
    },
    "Value": 20
  },
  "VenomMinDamageHealthPercentage": 100,
  "VenomModifier": {
    "Class": "ViperVenom",
    "Subclass": "ViperVenomModifierSubclass"
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
    "card_name": "Lethal Venom",
    "hero_key": "hero_viper",
    "hero_name": "Vyper",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_viper",
      "hero_name": "Vyper",
      "lookup": "lethal venom",
      "name": "Lethal Venom",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_life_drain" title="Life Drain" -->

## Life Drain

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_life_drain`
- Snapshot ID: `39471`
- Source-Dokument: `7070`
- Kurzinfo: Life Drain aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Life Drain`
- Payload Hash: `27ef44b219df92606fe8ef7e24dbf2bdca15e1fdd15a0e5e9713041b9f7f48e8`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.416881+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 18,
  "AbilityCooldown": 34.0,
  "AbilityDuration": 2.5,
  "AbilityUnitTargetLimit": 10,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorAllowAltCast",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_life_drain",
  "LifeDrainCasterModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "FamiliarAbilityKeepalive"
    ],
    "Subclass": "LifeDrainCaster"
  },
  "LifeDrainHealthMult": 75,
  "LifeDrainPerSecond": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.43
    },
    "Value": 32
  },
  "LifeDrainTargetModifier": {
    "Class": "LifeDrain",
    "SilenceModifier": {
      "Class": "CitadelSilenced",
      "Subclass": "LifeDrainSilence"
    },
    "Subclass": "LifeDrain"
  },
  "MaxRange": 28,
  "MoveSpeedReduction": 40,
  "Name": "Life Drain",
  "TickRate": 0.1,
  "Upgrades": [
    {
      "LifeDrainPerSecond": 18
    },
    {
      "AbilityDuration": 2.5
    },
    {
      "AbilityCharges": 3,
      "AbilityCooldownBetweenCharge": 0.1,
      "LifeDrainPerSecond": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.4
        },
        "Value": 0
      }
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
    "card_name": "Life Drain",
    "hero_key": "hero_ghost",
    "hero_name": "Lady Geist",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_ghost",
      "hero_name": "Lady Geist",
      "lookup": "life drain",
      "name": "Life Drain",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_skyrunner_swingline" title="Lifethread" -->

## Lifethread

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_skyrunner_swingline`
- Snapshot ID: `39532`
- Source-Dokument: `7070`
- Kurzinfo: Lifethread aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lifethread`
- Payload Hash: `d5dfefcac429a12f77358a5eb6d724f1d122f8ce0feefaaf58332fe3afd8d4e5`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.560909+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCharges": 2,
  "AbilityCooldown": 12,
  "AbilityCooldownBetweenCharge": 0.1,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_skyrunner_swingline",
  "MaxMoveSpeed": 25,
  "Name": "Lifethread",
  "SwingLineBulletDistance": 40,
  "SwingModifier": {
    "Class": "Base",
    "Subclass": "Swinging"
  },
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Lifethread",
    "hero_key": "hero_skyrunner",
    "hero_name": "Skyrunner",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_skyrunner",
      "hero_name": "Skyrunner",
      "lookup": "lifethread",
      "name": "Lifethread",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_unicorn_radiantblast" title="Light Eater" -->

## Light Eater

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_unicorn_radiantblast`
- Snapshot ID: `39554`
- Source-Dokument: `7070`
- Kurzinfo: Light Eater aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Light Eater`
- Payload Hash: `1958d6b77deba1102e7c5e667dc67f066e5905aff4223e893e2f6d58cd65ea52`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.622560+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.3,
  "AbilityCastRange": 10,
  "AbilityCooldown": 20,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.05
    },
    "Value": 8
  },
  "AbilityLifestealPercentHero": 20,
  "AbilityUnitTargetLimit": 100,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorUseLagCompensationForUnitTargeting",
    "BehaviorDontInterruptSlideOnCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.34
    },
    "Value": 15
  },
  "DebuffModifier": {
    "Class": "UnicornRadiantFlareDamage",
    "Subclass": "UnicornRadiantblastDebuff"
  },
  "ExtraSweepRadius": 2,
  "FlareDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.47
    },
    "Value": 40
  },
  "IsDisabled": false,
  "Key": "ability_unicorn_radiantblast",
  "Name": "Light Eater",
  "TargetingConeAngle": 70,
  "TickRate": 0.5,
  "Upgrades": [
    {
      "AbilityLifestealPercentHero": 15
    },
    {
      "AbilityCastRange": 3,
      "AbilityCooldown": -10
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.2
        },
        "Value": 25
      }
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
    "card_name": "Light Eater",
    "hero_key": "hero_unicorn",
    "hero_name": "Celeste",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_unicorn",
      "hero_name": "Celeste",
      "lookup": "light eater",
      "name": "Light Eater",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_lightning_ball" title="Lightning Ball" -->

## Lightning Ball

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_lightning_ball`
- Snapshot ID: `39626`
- Source-Dokument: `7070`
- Kurzinfo: Lightning Ball aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lightning Ball`
- Payload Hash: `dfd9fa9f688076612aebc0434cd39bf67c18f3ae5cea9e306246a61ed9c3a839`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.807065+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCharges": 1,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.5
    },
    "Value": 75
  },
  "IsDisabled": false,
  "Key": "citadel_ability_lightning_ball",
  "MaxLifetime": 5,
  "MinShockDuration": 0.5,
  "Name": "Lightning Ball",
  "ShockRadius": 4.25,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "GigagwattLightningballSlow"
  },
  "TickRate": 0.1,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "MaxLifetime": 1,
      "SlowPercent": 35
    },
    {
      "DPS": 58.5,
      "ShockRadius": 1.5
    }
  ],
  "ZapModifier": {
    "Class": "CitadelLightningball",
    "Subclass": "CitadelLightningball"
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
    "card_name": "Lightning Ball",
    "hero_key": "hero_gigawatt",
    "hero_name": "Seven",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_gigawatt",
      "hero_name": "Seven",
      "lookup": "lightning ball",
      "name": "Lightning Ball",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_familiar_helpinghands" title="Lil Helpers" -->

## Lil Helpers

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_familiar_helpinghands`
- Snapshot ID: `39431`
- Source-Dokument: `7070`
- Kurzinfo: Lil Helpers aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lil Helpers`
- Payload Hash: `a02b23c2285da7540b33c7d6a017c9b824a86adc341e4e5f908f7c3e899d8ced`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.307450+00:00`

### Vollstaendige Payload

````json
{
  "AIAggroModifier": {
    "Class": "NeutralAggro",
    "Subclass": "Aiaggro"
  },
  "AIPhysicsModifier": {
    "Class": "FamiliarAiPhysics",
    "Subclass": "Aiphysics"
  },
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": {
    "Scale": {
      "Type": "range",
      "Value": 0.0
    },
    "Value": 45
  },
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "ArmTime": 0.1,
  "AuraAttackHeight": 10,
  "AuraRadius": 10,
  "AuraSoftRadius": 10,
  "BehaviourBits": [
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSprint",
    "BehaviorAllowSelfCast",
    "BehaviorDamageDoesntWakeFromSleep",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDoNotAllowSpamProc",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusMoveSpeed": 3.0,
  "ChannelMoveSpeed": -1,
  "DPSPerSprite": 1,
  "Damage": 20,
  "HelperChoreCooldownDuration": 5,
  "HelperCount": 1,
  "HelperDowntimeDuration": 15.1,
  "HelpersPerPatrol": 4,
  "InfestBurstHealthPercent": 75,
  "InfestDamageTakenPercent": 30,
  "InfestHeal": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.14
    },
    "Value": 8
  },
  "InfestHealInterval": 2.0,
  "InfestModifier": {
    "Class": "CitadelModifierFamiliarInfested",
    "Subclass": "Infest"
  },
  "InfestWaitingModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "FamiliarInfestWaiting"
    ],
    "Subclass": "Infestwaiting"
  },
  "InvisWatcherModifier": {
    "Class": "FamiliarHelperInvisWatcher",
    "Subclass": "FamiliarHelperInvisWatcher"
  },
  "IsDisabled": false,
  "Key": "ability_familiar_helpinghands",
  "NPCInfestDuration": 50,
  "Name": "Lil Helpers",
  "PatrolDamageCooldown": 10,
  "PlayerInfestDuration": 8,
  "TechArmorGain": 12,
  "TickRate": 0.2,
  "Upgrades": [
    {
      "BonusMoveSpeed": 1.5,
      "HelperCount": 1
    },
    {
      "HelperCount": 1,
      "InfestDamageTakenPercent": 15
    },
    {
      "HelperCount": 1,
      "TechArmorGain": 15
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
    "card_name": "Lil Helpers",
    "hero_key": "hero_familiar",
    "hero_name": "Rem",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_familiar",
      "hero_name": "Rem",
      "lookup": "lil helpers",
      "name": "Lil Helpers",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_vampirebat_lovebites" title="Love Bites" -->

## Love Bites

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_vampirebat_lovebites`
- Snapshot ID: `39559`
- Source-Dokument: `7070`
- Kurzinfo: Love Bites aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Love Bites`
- Payload Hash: `d1a53350a658ada08666c9a7ba35b4fb3de266e78012d7fdfac2b77e9a21363a`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.634246+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "BonusDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.85
    },
    "Value": 45
  },
  "BuildUpDuration": 5,
  "BuildUpHeadshotBonus": 1.5,
  "BuildUpModifier": {
    "BuildUpDecayDelay": 3.0,
    "Class": "CitadelBaseBuildup",
    "Subclass": "LovebitesBuildup"
  },
  "BuildUpPerBat": 20,
  "BuildUpPerDagger": 30,
  "BuildUpPerShot": 18.4,
  "ChannelMoveSpeed": -1,
  "DamageProcModifier": {
    "BuffModifier": {
      "Class": "Base",
      "Subclass": "LovebitesBuff"
    },
    "Class": "VampirebatLovebitesproc",
    "SlowModifier": {
      "Class": "SlowBase",
      "Subclass": "Slowsf"
    },
    "Subclass": "LovebitesProc"
  },
  "EffectivenessVolumeScaleMax": 1.0,
  "EffectivenessVolumeScaleMin": 0.5,
  "IsDisabled": false,
  "Key": "ability_vampirebat_lovebites",
  "MagicDamagePerBullet": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.09
    },
    "Value": 4
  },
  "Name": "Love Bites",
  "PerTargetCooldown": 10,
  "Upgrades": [
    {
      "SlowDuration": 3,
      "SlowPercent": 30
    },
    {
      "BonusDamage": 45,
      "MagicDamagePerBullet": 3.0
    },
    {
      "BonusFireRate": 25,
      "BuffDuration": 5,
      "PerTargetCooldown": -5
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
    "card_name": "Love Bites",
    "hero_key": "hero_vampirebat",
    "hero_name": "Mina",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_vampirebat",
      "hero_name": "Mina",
      "lookup": "love bites",
      "name": "Love Bites",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_doorman_luggage_cart" title="Luggage Cart" -->

## Luggage Cart

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_doorman_luggage_cart`
- Snapshot ID: `39423`
- Source-Dokument: `7070`
- Kurzinfo: Luggage Cart aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Luggage Cart`
- Payload Hash: `afdafe6e9c6d7599e2bc250a21a0dbd5b5072dfd7e49b68096a7028deb74a170`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.282859+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCastRange": 25,
  "AbilityCooldown": 30,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorAllowSelfCast",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "CartDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.75
    },
    "Value": 80
  },
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_doorman_luggage_cart",
  "ModifierDrag": {
    "Class": "LuggageDrag",
    "EnabledStateMask": [
      "IgnorePortals"
    ],
    "Subclass": "Drag"
  },
  "Name": "Luggage Cart",
  "Upgrades": [
    {
      "CartDamage": 60
    },
    {
      "AbilityCastRange": 20,
      "WallImpactDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 75
      }
    },
    {
      "AbilityCooldown": -15,
      "StunDuration": 1.25
    }
  ],
  "WallImpactDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0
    },
    "Value": 0
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
    "card_name": "Luggage Cart",
    "hero_key": "hero_doorman",
    "hero_name": "The Doorman",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_doorman",
      "hero_name": "The Doorman",
      "lookup": "luggage cart",
      "name": "Luggage Cart",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="cadence_ability_lullaby" title="Lullaby" -->

## Lullaby

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `cadence_ability_lullaby`
- Snapshot ID: `39594`
- Source-Dokument: `7070`
- Kurzinfo: Lullaby aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lullaby`
- Payload Hash: `9848f6cfd174725224779a809092c2cd534306745ff53a9ae4029495d527691e`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.724217+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCooldown": 48.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "ExternalBonusHealthRegen": 15,
  "IsDisabled": false,
  "Key": "cadence_ability_lullaby",
  "LingerDuration": 0.25,
  "MinimumSleepTime": 2,
  "Name": "Lullaby",
  "Radius": 12,
  "SleepAOEModifier": {
    "Class": "CadenceSleepAoe",
    "ProvidedByAura": {
      "Class": "CadenceSleeping",
      "Subclass": "CadenceSleeping"
    },
    "Subclass": "CadenceSleepAoe"
  },
  "SleepWakeUpDelay": 0.25,
  "Upgrades": [
    {
      "ExternalBonusHealthRegen": 15
    },
    {
      "Radius": 2
    },
    {
      "LingerDuration": 0.75
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_cadence",
      "hero_name": "Cadence",
      "lookup": "lullaby",
      "name": "Lullaby",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="fathom_lurkers_ambush" title="Lurker's Ambush" -->

## Lurker's Ambush

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `fathom_lurkers_ambush`
- Snapshot ID: `39677`
- Source-Dokument: `7070`
- Kurzinfo: Lurker's Ambush aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lurker's Ambush`
- Payload Hash: `b1b7c94c41b77937c1e939bb41c33404b21f609b3db6edae0b0fbec8bb876dad`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.926298+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 30,
  "AbilityChannelTime": 9999,
  "AbilityCooldown": 50.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorCannotCancelDuringChannel"
  ],
  "ChannelMoveSpeed": 5,
  "ChannelTimeForMaxDebuff": 1.5,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.651
    },
    "Value": 60
  },
  "DebuffMaxDuration": 3.5,
  "DebuffMinDuration": 1.0,
  "DebuffModifier": {
    "Class": "FathomLurkersAmbushDebuff",
    "EnabledStateMask": [
      "Slowed"
    ],
    "Subclass": "FathomLurkersAmbushDebuff"
  },
  "EnemySlowPct": 60,
  "InitialHeight": 350,
  "InvisFadeToDuration": 1.5,
  "InvisModifier": {
    "Class": "LurkersAmbushInvis",
    "InvisBias": 0.8,
    "Subclass": "Invis"
  },
  "IsDisabled": false,
  "Key": "fathom_lurkers_ambush",
  "Name": "Lurker's Ambush",
  "NonLatchedDurationPct": 50,
  "NotSeenByEnemiesRegen": 3,
  "RegenModifier": {
    "Class": "Base",
    "Subclass": "Regen"
  },
  "RevealOnDamageDuration": 0.5,
  "RevealOnSpottedDuration": 3,
  "SpottedRadius": 999,
  "StandStillMinTime": 0.5,
  "TickRate": 0.25,
  "Upgrades": [
    {
      "AbilityCooldown": -15
    },
    {
      "DebuffMaxDuration": 1
    },
    {
      "NotSeenByEnemiesRegen": 2
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
    "card_name": "Lurker's Ambush",
    "hero_key": "hero_slork",
    "hero_name": "Fathom",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_slork",
      "hero_name": "Fathom",
      "lookup": "lurker's ambush",
      "name": "Lurker's Ambush",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_werewolf_transformation" title="Lycan Curse" -->

## Lycan Curse

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_transformation`
- Snapshot ID: `39583`
- Source-Dokument: `7070`
- Kurzinfo: Lycan Curse aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lycan Curse`
- Payload Hash: `7687a1cf6cf774125fad24564e0fe6b168a3e8d8e64a8489e0b5185e7c6e60e4`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.694571+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityChargesConditionally": 1,
  "AbilityCooldown": 80,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 15,
  "AbilityPostCastDuration": 0.5,
  "AbilityUnitTargetLimit": 1,
  "AutoActivateHealthThreshold": 20,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusDurationOnBullet": 0.15,
  "BonusDurationOnHeavyMelee": 1.5,
  "BonusDurationOnLightMelee": 0.5,
  "BonusDurationPerHealthPercentLost": 0.1,
  "BonusFireRate": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.45
    },
    "Value": 60
  },
  "BonusHealth": {
    "Scale": {
      "Type": "power_increase",
      "Value": 15.0
    },
    "Value": 125
  },
  "BonusMoveSpeed": 1.5,
  "CameraTurnRateMax": 188,
  "ChannelMoveSpeed": 7.62,
  "EndingWarningSoundDuration": 3.0,
  "HeadshotResist": -20,
  "HealAmount": {
    "Scale": {
      "Type": "power_increase",
      "Value": 1.0
    },
    "Value": 0
  },
  "IsDisabled": false,
  "Key": "ability_werewolf_transformation",
  "KillCreditModifier": {
    "Class": "Base",
    "Subclass": "Killcredit"
  },
  "LowHealthFraction": 30,
  "LowHealthRageBonus": {
    "Scale": {
      "Type": "power_increase",
      "Value": 1.8
    },
    "Value": 40
  },
  "MaxRage": {
    "Scale": {
      "Type": "power_increase",
      "Value": 9.4
    },
    "Value": 100
  },
  "MaxStacks": 15,
  "MissingHealthPercentHeal": 30,
  "Name": "Lycan Curse",
  "RagePerDamage": 0.255,
  "RagePercentagePerSecondInCombat": 1,
  "RagePercentagePerSecondOutOfCombat": -3,
  "ReadyDuration": 3,
  "ReadyModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "WerewolfReady"
    ],
    "Subclass": "Ready"
  },
  "StackDuration": 5,
  "Upgrades": [
    {
      "BulletResist": 20,
      "TechResist": 20
    },
    {
      "BonusHealth": 200,
      "BonusMoveSpeed": 4
    },
    {
      "KillCreditWindow": 1.5,
      "KillDurationBonus": 15
    }
  ],
  "WerewolfModifier": {
    "Class": "Werewolf",
    "ModelScale": 1.0,
    "StackingBuffModifier": {
      "Class": "WerewolfStackingbuff",
      "Subclass": "Stackingbuff"
    },
    "StatusEffectPriority": 10,
    "Subclass": "Werewolf"
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
    "card_name": "Lycan Curse",
    "hero_key": "hero_werewolf_transformed",
    "hero_name": "Silver (Transformed)",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_werewolf_transformed",
      "hero_name": "Silver (Transformed)",
      "lookup": "lycan curse",
      "name": "Lycan Curse",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_wrecker_ultimate" title="Magnetic Flux" -->

## Magnetic Flux

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_wrecker_ultimate`
- Snapshot ID: `39588`
- Source-Dokument: `7070`
- Kurzinfo: Magnetic Flux aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Magnetic Flux`
- Payload Hash: `31a8f8ded09e328d6d73534342bc3349a130c8a3dc78e3fffed02667b1951d37`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.706800+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1.0,
  "AbilityChannelTime": 5,
  "AbilityCooldown": 170.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AccelSpeed": 2400,
  "ActiveModifier": {
    "Class": "WreckerUltimate",
    "EnemyDamageModifier": {
      "Class": "DamageOnHitGround",
      "Subclass": "WreckerUltimateDamageEnemy"
    },
    "EnemyGrabModifier": {
      "Class": "WreckerUltimateGrabEnemy",
      "Subclass": "WreckerUltimateGrabEnemy"
    },
    "EnemyThrowModifier": {
      "Class": "WreckerUltimateThrowEnemy",
      "Subclass": "WreckerUltimateThrowEnemy"
    },
    "InvincibleModifier": {
      "Class": "WreckerUltimateInvincible",
      "EnabledStateMask": [
        "Invulnerable",
        "TechUntargetable",
        "UnitStatusHealthHidden",
        "IgnoreBullets",
        "IgnoreMelee"
      ],
      "Subclass": "WreckerInvincible"
    },
    "Subclass": "WreckerUltimateSubclass"
  },
  "BeamLength": 20,
  "BeamWidth": 40,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorChannelled",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": 5,
  "GrabDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 0
  },
  "GrabRange": 5,
  "HoldDistance": 2,
  "HoldHeight": 120,
  "IsDisabled": false,
  "Key": "ability_wrecker_ultimate",
  "Name": "Magnetic Flux",
  "PullSpeed": 800,
  "StunDuration": 1.5,
  "ThrowDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.487469
    },
    "Value": 160
  },
  "ThrowSpeed": 500,
  "TimeUntilStasis": 0.5,
  "TrackingSpeed": 70,
  "Upgrades": [
    {
      "BeamLength": 10
    },
    {
      "AbilityCooldown": -38.0
    },
    {
      "Invulnerable": 1
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_blood_shards" title="Malice" -->

## Malice

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_blood_shards`
- Snapshot ID: `39397`
- Source-Dokument: `7070`
- Kurzinfo: Malice aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Malice`
- Payload Hash: `3a84248ceb1079cd470e131f3f3c79bdd4076412c0f9bf910d5b0b0e1e1fa3f5`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.224679+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.12,
  "AbilityCooldown": 6,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.3,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DebuffDuration": 9,
  "DebuffModifier": {
    "Class": "BloodShardDebuff",
    "Subclass": "BloodShardDebuff"
  },
  "HealthToDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.558
    },
    "Value": 23.0
  },
  "IsDisabled": false,
  "Key": "ability_blood_shards",
  "MaxStacks": 5,
  "MoveSpeedPenaltyPerStack": 15,
  "Name": "Malice",
  "NumBloodShards": 3,
  "SelfDamagePct": 9,
  "SlowDuration": 4,
  "SpreadAngleDegrees": 6,
  "Upgrades": [
    {
      "AbilityCooldown": -3
    },
    {
      "HealthToDamage": 25.2,
      "NumBloodShards": 4,
      "SpreadAngleDegrees": 22
    },
    {
      "VulnerabilityPerStack": 7
    }
  ],
  "VulnerabilityPerStack": 8,
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
    "card_name": "Malice",
    "hero_key": "hero_ghost",
    "hero_name": "Lady Geist",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_ghost",
      "hero_name": "Lady Geist",
      "lookup": "malice",
      "name": "Malice",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_astro_rifle" title="Marksman" -->

## Marksman

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_astro_rifle`
- Snapshot ID: `39393`
- Source-Dokument: `7070`
- Kurzinfo: Marksman aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Marksman`
- Payload Hash: `7ad85e2ae53a30de543d2151e26b02fad814d18f15c4e5186eb506bae93e5479`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.212613+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCharges": 4,
  "AbilityCooldown": 10.5,
  "AbilityCooldownBetweenCharge": 2.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorAllowSelfCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.438722
    },
    "Value": 48
  },
  "DamageAmplificationPerStack": 10,
  "DebuffDuration": 15,
  "DebuffModifier": {
    "Class": "AstroRifleDebuff",
    "SlowModifier": {
      "Class": "SlowBase",
      "Subclass": "Slow"
    },
    "Subclass": "AstroRifleDebuff"
  },
  "IsDisabled": false,
  "Key": "ability_astro_rifle",
  "MaxStacks": 10,
  "Name": "Marksman",
  "SelfModifier": {
    "Class": "AstroRifleSelf",
    "Subclass": "AstroRifleSelf"
  },
  "Upgrades": [
    {
      "SlowDurationPerStack": 1,
      "SlowPercent": 15
    },
    {
      "DamageAmplificationPerStack": 5
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.584963
        },
        "Value": 48
      }
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_werewolf_maulingleap" title="Mauling Leap" -->

## Mauling Leap

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_maulingleap`
- Snapshot ID: `39578`
- Source-Dokument: `7070`
- Kurzinfo: Mauling Leap aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mauling Leap`
- Payload Hash: `e980c13472c1c5fd1fe2e03c3aeb699a1fb48ad21117bfefd03192d61e491acc`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.685184+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.35,
  "AbilityCastRange": 18.1,
  "AbilityChannelTime": 0.55,
  "AbilityCooldown": 16,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.1,
  "AbilityUnitTargetLimit": 1,
  "AllowRamMultiple": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanCancelDuringCastDelay",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BulletArmorReduction": -8,
  "CameraTurnRateMax": 188,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.15
    },
    "Value": 15
  },
  "Damage": {
    "Scale": {
      "Type": "melee",
      "Value": 1.5
    },
    "Value": 0
  },
  "DebuffDuration": 6,
  "DebuffModifier": {
    "Class": "CitadelWerewolfMaulingleapdebuff",
    "Subclass": "Shred"
  },
  "IsDisabled": false,
  "Key": "ability_werewolf_maulingleap",
  "LeapForwardOffset": 0.3,
  "LeapMultiHitRadius": 1.5,
  "LeapRadius": 2.8,
  "LeapingModifier": {
    "Class": "CitadelWerewolfLeaping",
    "Subclass": "Leaping"
  },
  "Name": "Mauling Leap",
  "TickRate": 0.5,
  "Upgrades": [
    {
      "DPS": 10
    },
    {
      "AbilityCooldown": -9
    },
    {
      "BulletArmorReduction": -12
    }
  ],
  "WorldImpactRadius": 25,
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
    "card_name": "Mauling Leap",
    "hero_key": "hero_werewolf_transformed",
    "hero_name": "Silver (Transformed)",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_werewolf_transformed",
      "hero_name": "Silver (Transformed)",
      "lookup": "mauling leap",
      "name": "Mauling Leap",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_mobile_resupply" title="Medicinal Specter" -->

## Medicinal Specter

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_mobile_resupply`
- Snapshot ID: `39630`
- Source-Dokument: `7070`
- Kurzinfo: Medicinal Specter aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Medicinal Specter`
- Payload Hash: `7c9713d6d4a85d8de86c8675c0ffe448431abc7aba569e567e9f5c333d4841ca`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.815209+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 15,
  "AbilityCooldown": 50.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6.5,
  "AbilityUnitTargetLimit": 1,
  "AuraModifier": {
    "AmbientParticleRadiusControlPoint": 1,
    "Class": "MobileResupplyAura",
    "ModifierProvidedByAuraDuration": 1.0,
    "ProvidedByAura": {
      "Class": "MobileResupply",
      "Subclass": "MobileResupply"
    },
    "Subclass": "MobileResupplyAura"
  },
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanHealPlayers",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "ExternalBonusHealthRegen": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.3
    },
    "Value": 25
  },
  "HealInterval": 0.1,
  "HealRadius": 6,
  "IsDisabled": false,
  "Key": "citadel_ability_mobile_resupply",
  "Name": "Medicinal Specter",
  "TurretHealMult": 1.0,
  "Upgrades": [
    {
      "SpiritResist": 40
    },
    {
      "AbilityCooldown": -20.0,
      "StaminaCooldownReduction": 100.0
    },
    {
      "AbilityDuration": 1.5,
      "HealRadius": 3,
      "MaxHealthRegenPct": 2.0
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
    "card_name": "Medicinal Specter",
    "hero_key": "hero_forge",
    "hero_name": "McGinnis",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_forge",
      "hero_name": "McGinnis",
      "lookup": "medicinal specter",
      "name": "Medicinal Specter",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="super_neutral_charge" title="Mid Boss Charge" -->

## Mid Boss Charge

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `super_neutral_charge`
- Snapshot ID: `39706`
- Source-Dokument: `7070`
- Kurzinfo: Mid Boss Charge aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mid Boss Charge`
- Payload Hash: `e4223b48d32c0ca48e8f842b473543443ecfaf55a38435266063ced8c1987196`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.004608+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 10,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "CameraDistance": 120,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 120
  },
  "IsDisabled": false,
  "Key": "super_neutral_charge",
  "Name": "Mid Boss Charge",
  "PrepareTime": 0.8,
  "StunDuration": 0.5,
  "TackleDurationMax": 1.0,
  "TackleDurationMin": 0.5,
  "TackleRadius": 2.5,
  "TackleSpeed": 1000,
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_shieldedsentry" title="Mini Turret" -->

## Mini Turret

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shieldedsentry`
- Snapshot ID: `39638`
- Source-Dokument: `7070`
- Kurzinfo: Mini Turret aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mini Turret`
- Payload Hash: `4973553537b3d18a3164c0488a6dd777adedb14237d34ccf065bffc0948ac761`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.834880+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 20,
  "AbilityCharges": 1,
  "AbilityCooldown": 18.0,
  "AbilityCooldownBetweenCharge": 3,
  "AbilityUnitTargetLimit": 1,
  "AttackConeAngle": 10,
  "AttackSpeedMult": 100,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventTrainingBotUsage",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BossDamagePercentIncoming": 50,
  "BossDamagePercentOutgoing": 30,
  "DebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "ForgeMiniTurretDebuff"
  },
  "DecayingResist": 80,
  "DecayingResistDuration": 6,
  "InnateModifier": {
    "Class": "ForgeMiniTurretInnateModifier",
    "Subclass": "ForgeMiniTurretInnateModifier"
  },
  "IsDisabled": false,
  "Key": "citadel_ability_shieldedsentry",
  "MeleeResist": 35,
  "ModelScale": 0.8,
  "Name": "Mini Turret",
  "NonHeroDamagePercentOutgoing": 50,
  "TechResist": 35,
  "TickRate": 0.5,
  "TrackingSpeed": 430,
  "TurretAttackDelay": 0.2,
  "TurretAttackFalloffEnd": 30,
  "TurretAttackFalloffStart": 20,
  "TurretAttackRange": 30,
  "TurretBaseHealth": {
    "Scale": {
      "Type": "power_increase",
      "Value": 7.8
    },
    "Value": 90
  },
  "TurretDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.42
    },
    "Value": 24
  },
  "TurretDeployTime": 0.25,
  "TurretLifetime": 35,
  "Upgrades": [
    {
      "TurretAttackRange": 10,
      "TurretDPS": 10
    },
    {
      "AbilityCharges": 2
    },
    {
      "AttackSpeedMult": 25,
      "TurretLifetime": 12
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
    "card_name": "Mini Turret",
    "hero_key": "hero_forge",
    "hero_name": "McGinnis",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_forge",
      "hero_name": "McGinnis",
      "lookup": "mini turret",
      "name": "Mini Turret",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_magewalk" title="Misdirection" -->

## Misdirection

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_magewalk`
- Snapshot ID: `39627`
- Source-Dokument: `7070`
- Kurzinfo: Misdirection aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Misdirection`
- Payload Hash: `4c07648e0122cbebfc8e3302d8481efa305f93592f685265be82dcd9e074bcae`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.809460+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 10,
  "AbilityCooldown": 15,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventBotUsage",
    "BehaviorAllowAltCast",
    "BehaviorMovement",
    "BehaviorCanSetQuickCast"
  ],
  "BubbleModifier": {
    "Class": "Magewalk",
    "StatusEffectPriority": 100,
    "Subclass": "Magewalk"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.465
    },
    "Value": 30
  },
  "EmpoweredAttackCount": 2,
  "FireRateBonus": 25,
  "FireRateBonusDurationMax": 8,
  "ImpulseStrength": -3000,
  "IsDisabled": false,
  "Key": "citadel_ability_magewalk",
  "MageTime": 1.5,
  "Name": "Misdirection",
  "ProcChance": 100,
  "TrailInterval": 0.01,
  "TurretAttackDelay": 1,
  "TurretAttackFalloffEnd": null,
  "TurretAttackFalloffStart": null,
  "TurretBaseHealth": 400,
  "TurretHealthScaling": null,
  "TurretLifetime": 8,
  "TurretModifier": {
    "Class": "Base",
    "Subclass": "TurretModifier"
  },
  "Upgrades": [
    {
      "AbilityCastRange": 4
    },
    {
      "FireRateBonus": 25
    },
    {
      "AbilityCooldown": -4,
      "BonusClipSizePercent": 120
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_incendiary_projectile" title="Napalm" -->

## Napalm

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_incendiary_projectile`
- Snapshot ID: `39466`
- Source-Dokument: `7070`
- Kurzinfo: Napalm aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Napalm`
- Payload Hash: `c8af275f5a4de911de42a4e18eb9a0fb9431e2be7a487fe0a5ca82be450e5680`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.397584+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 20,
  "AbilityCharges": 1,
  "AbilityCooldown": 25.0,
  "AbilityCooldownBetweenCharge": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": 18,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 40.0
  },
  "DebuffDuration": 8,
  "DebuffModifier": {
    "Class": "IncendiaryDebuff",
    "StatusEffectPriority": 50,
    "Subclass": "IncendiaryDebuff"
  },
  "GrowthPerMeter": 0.5,
  "HeightOffGround": 50,
  "IncomingDamagePercentFromCaster": 16,
  "InitialWidth": 1,
  "IsDisabled": false,
  "Key": "ability_incendiary_projectile",
  "Name": "Napalm",
  "ParticleRadiusMultiplier": 1.15,
  "SlowDuration": 4,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "SlowPercent": 35,
  "TickRate": 0.5,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "LifestealPercentHero": 15
    },
    {
      "HealAmpReceivePenaltyPercent": -33,
      "HealAmpRegenPenaltyPercent": -33,
      "IncomingDamagePercentFromCaster": 17
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
    "card_name": "Napalm",
    "hero_key": "hero_inferno",
    "hero_name": "Infernus",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_inferno",
      "hero_name": "Infernus",
      "lookup": "napalm",
      "name": "Napalm",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_familiar_ability01" title="Naptime" -->

## Naptime

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_familiar_ability01`
- Snapshot ID: `39427`
- Source-Dokument: `7070`
- Kurzinfo: Naptime aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Naptime`
- Payload Hash: `6bd7cf5ef5bdf8f1dd92678531bceabbe541b93d1e9af0228d6f1e68ceb56023`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.295634+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.18,
  "AbilityCastRange": 24,
  "AbilityChannelTime": {
    "Scale": {
      "Type": "duration",
      "Value": 0.0
    },
    "Value": 1.9
  },
  "AbilityCooldown": 200.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoChannelModifier": {
    "Class": "Base",
    "Subclass": "Channeling"
  },
  "AwakeDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.6
    },
    "Value": 120
  },
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorRefundHalfCooldownOnChannelInterrupt"
  ],
  "ChannelMoveSpeed": -1,
  "DamageResistPctWhileChanneling": 30,
  "EffectModifier": {
    "Class": "CitadelModifierFamiliarAsleep",
    "Subclass": "Effect"
  },
  "Height": 20,
  "IsDisabled": false,
  "Key": "ability_familiar_ability01",
  "MinSleepTime": 0.5,
  "MoveSpeedAndDashSlowPct": 25,
  "Name": "Naptime",
  "Radius": 19.0,
  "SleepDamageThreshold": {
    "Scale": {
      "Type": "power_increase",
      "Value": 3.1
    },
    "Value": 100
  },
  "SleepDuration": 4.0,
  "SleepMoveSpeed": 1.5,
  "StaringModifier": {
    "Class": "CitadelModifierFamiliarStaring",
    "Subclass": "Staring"
  },
  "UnstoppableWhileChannelingModifier": {
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
    "Subclass": "FamiliarUltUnstoppable"
  },
  "Upgrades": [
    {
      "ConsumeStaminaOnWake": 1,
      "NoStaminaRegenDuringSleep": 1
    },
    {
      "Radius": 3,
      "SleepDuration": 0.75
    },
    {
      "AbilityCooldown": -55,
      "DamageResistPctWhileChanneling": 50,
      "UnstoppableWhileChanneling": 1
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
    "card_name": "Naptime",
    "hero_key": "hero_familiar",
    "hero_name": "Rem",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_familiar",
      "hero_name": "Rem",
      "lookup": "naptime",
      "name": "Naptime",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_nano_proximity_ritual" title="Nekomata Ward" -->

## Nekomata Ward

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_nano_proximity_ritual`
- Snapshot ID: `39491`
- Source-Dokument: `7070`
- Kurzinfo: Nekomata Ward aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Nekomata Ward`
- Payload Hash: `d7afee31ebbe8d965ae614867d4650e23b4e101636073017dffaf01ea9b1acb9`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.468114+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 30.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 60,
  "AbilityUnitTargetLimit": 1,
  "ActiveRadius": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.186
    },
    "Value": 40
  },
  "AttackRadius": 30,
  "BehaviourBits": [
    "BehaviorProjectileFiredAsBullet",
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorAllowSelfCast",
    "BehaviorCanSetQuickCast",
    "BehaviorCanCancelDuringCastDelay"
  ],
  "CatActivateDuration": 2.0,
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.279
    },
    "Value": 40
  },
  "DamageTick": 1.0,
  "HealAmpReceivePenaltyPercent": -20,
  "HealAmpRegenPenaltyPercent": -20,
  "InvisFadeToDuration": 1.0,
  "IsDisabled": false,
  "Key": "ability_nano_proximity_ritual",
  "Name": "Nekomata Ward",
  "PredatoryStatueModifier": {
    "Class": "NanoPredatoryStatue",
    "MinRevealTime": 2.0,
    "NewTargetAttackTime": 0.5,
    "RevealModifier": {
      "Class": "Base",
      "EnabledStateMask": [
        "VisibleToEnemy"
      ],
      "Subclass": "RevealModifier"
    },
    "SelfHealScale": 0.2,
    "Subclass": "NanoPredatoryStatue",
    "TargetModifier": {
      "Class": "NanoPredatoryStatueTarget",
      "EnabledStateMask": [
        "PredatoryStatueTarget"
      ],
      "Subclass": "NanoPredatoryStatueTarget"
    }
  },
  "RecentDamageMarkDuration": 1.5,
  "RecentDamageModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "NanoRecentDamage"
    ],
    "Subclass": "NanoRecentDamage"
  },
  "RevealOnDamageDuration": 1.5,
  "RevealOnSpottedDuration": 1.5,
  "SpottedRadius": 20,
  "StatueArmTime": 0.5,
  "StatueHealth": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.86
    },
    "Value": 300
  },
  "TargetLifesteal": 30,
  "TargetLifestealNonHero": 10,
  "TickInterval": 0.1,
  "Upgrades": [
    {
      "MakeInvisible": 1
    },
    {
      "AbilityDuration": 30,
      "DPS": 20
    },
    {
      "HealAmpReceivePenaltyPercent": -30,
      "HealAmpRegenPenaltyPercent": -30,
      "VictimDamageReduction": -30
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_vampirebat_batswarm" title="Nox Nostra" -->

## Nox Nostra

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_vampirebat_batswarm`
- Snapshot ID: `39558`
- Source-Dokument: `7070`
- Kurzinfo: Nox Nostra aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Nox Nostra`
- Payload Hash: `4a667041015c5c33bc9742595e1bada632ec4dc35f4f80d4def38dbcf69b1406`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.631509+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.5,
  "AbilityCastRange": 40,
  "AbilityCooldown": 150,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 12,
  "AirDrag": 12,
  "BatCount": 75,
  "BatCountPerWave": 1,
  "BatEffectiveness": 0.2,
  "BatPerSecond": 30,
  "BatSpawnRadius": 1.5,
  "BatSpawnRandomAngle": 0.15,
  "BatSpawnRandomVelocity": 300,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusBatsMax": 50,
  "BonusBatsPerProc": 2,
  "ChannelMoveSpeed": -1,
  "CurrentHealthDamageCapToBosses": 20,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.094
    },
    "Value": 4.6
  },
  "DebuffDuration": 1.25,
  "DebuffModifier": {
    "Class": "CitadelSilenced",
    "Subclass": "Debuff"
  },
  "FallSpeedMax": 1,
  "GroundAccelerationPercentage": -80,
  "GroundFrictioNpercentage": -80,
  "IsDisabled": false,
  "JumpCeilingCheckDistance": 11,
  "JumpPitch": -60,
  "JumpSpeed": 17,
  "Key": "ability_vampirebat_batswarm",
  "LockonConeAngle": 40,
  "MaxBatTargets": 2,
  "MaxLockonStacks": 1,
  "Name": "Nox Nostra",
  "NotInConeLosesLock": 1,
  "StacksCanDecay": 1,
  "TargetingConeAngle": 20,
  "TimeToGainLockonStack": 0.01,
  "TimeToLoseLockonStack": 0.3,
  "Upgrades": [
    {
      "Damage": 1.9
    },
    {
      "AbilityCooldown": -45
    },
    {
      "CurrentHealthPercent": 0.5
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
    "card_name": "Nox Nostra",
    "hero_key": "hero_vampirebat",
    "hero_name": "Mina",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_vampirebat",
      "hero_name": "Mina",
      "lookup": "nox nostra",
      "name": "Nox Nostra",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_kali_spinning_blade" title="Occilioblade" -->

## Occilioblade

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_kali_spinning_blade`
- Snapshot ID: `39622`
- Source-Dokument: `7070`
- Kurzinfo: Occilioblade aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Occilioblade`
- Payload Hash: `240f6061c6b21122ceb5fecc4b353741a9a8086563d76fbf647b03d2b60cf6de`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.796754+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCastRange": 50,
  "AbilityCharges": 2,
  "AbilityCooldown": 30.0,
  "AbilityCooldownBetweenCharge": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDontTriggerSpellBlock",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSprint"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 120
  },
  "DebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "IsDisabled": false,
  "Key": "citadel_ability_kali_spinning_blade",
  "MinReflectionDOTResult": -0.95,
  "MinReflectionZ": 0.3,
  "Name": "Occilioblade",
  "NoClipDuration": 1,
  "ProjectileFlyOutTime": 0.6,
  "ProjectileFlyReturnTime": 1.0,
  "ReflectionSpeedFactor": 0.5,
  "ReturnOffSetTargetDistance": 150,
  "ReturnUpVelocity": 200,
  "TechCleaveExpireTime": 0.2,
  "Upgrades": [
    {
      "SlowDuration": 4,
      "SlowPercent": 30
    },
    {
      "Damage": {
        "Scale": {
          "Multiply": true,
          "Type": "spirit",
          "Value": 1.218672
        },
        "Value": 40
      }
    },
    {
      "CooldownReductionOnHit": -7
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_kali",
      "hero_name": "Kali",
      "lookup": "occilioblade",
      "name": "Occilioblade",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_werewolf_onthehunt" title="On The Hunt" -->

## On The Hunt

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_onthehunt`
- Snapshot ID: `39580`
- Source-Dokument: `7070`
- Kurzinfo: On The Hunt aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `On The Hunt`
- Payload Hash: `ee2fcab502fbc1dea811acaec9652c79364b37e27b103739510f178f33297582`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.689870+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 48.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.060934
    },
    "Value": 4
  },
  "AbilityUnitTargetLimit": 1,
  "AlliedFireRatePercentage": 50,
  "BehaviourBits": null,
  "BonusFireRate": 30,
  "BonusMoveSpeed": 2,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_werewolf_onthehunt",
  "Name": "On The Hunt",
  "Radius": 25,
  "RapidFireModifier": {
    "Class": "WerewolfOnthehunt",
    "Subclass": "Rapidfire"
  },
  "Upgrades": [
    {
      "AbilityDuration": 3
    },
    {
      "BonusMoveSpeed": 1
    },
    {
      "BulletLifestealPercent": 18
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_wrecker_garbage_suck" title="Overload" -->

## Overload

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_wrecker_garbage_suck`
- Snapshot ID: `39667`
- Source-Dokument: `7070`
- Kurzinfo: Overload aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Overload`
- Payload Hash: `09bf67107e88d526b8c9369d1fc2d03c8abe1f53d309643940ad4e2aa183698a`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.902441+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityChannelTime": 3,
  "AbilityCooldown": 130,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BaseDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.93
    },
    "Value": 75
  },
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorExclusiveUse",
    "BehaviorCastableWhileBusy",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "CameraDistance": 800,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.194988
    },
    "Value": 0
  },
  "DamagePerSecond": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.604
    },
    "Value": 100
  },
  "GarbageAuraModifier": {
    "Class": "Garbageaura",
    "ProvidedByAura": {
      "Class": "Base",
      "OuterSpeedScale": 10.0,
      "StatusEffectPriority": 60,
      "Subclass": "GarbageauraTarget"
    },
    "StatusEffectPriority": 0,
    "Subclass": "Garbageaura"
  },
  "GarbageRadius": 12,
  "IsDisabled": false,
  "Key": "citadel_ability_wrecker_garbage_suck",
  "Name": "Overload",
  "SlowPercent": 50,
  "Speed": 5.08,
  "TickRate": 1,
  "TossAngle": 45,
  "TossSpeed": 8.89,
  "Upgrades": [
    {
      "GarbageRadius": 2
    },
    {
      "AbilityCooldown": -35
    },
    {
      "BaseDamage": 100,
      "DamagePerSecond": 50
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
    "card_name": "Overload",
    "hero_key": "hero_wrecker",
    "hero_name": "Wrecker",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_wrecker",
      "hero_name": "Wrecker",
      "lookup": "overload",
      "name": "Overload",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_kali_disruptive_charge" title="Pack Hunter" -->

## Pack Hunter

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_kali_disruptive_charge`
- Snapshot ID: `39621`
- Source-Dokument: `7070`
- Kurzinfo: Pack Hunter aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Pack Hunter`
- Payload Hash: `9bd8237d1cf88dd2f3164f47fb84dc73ae718c14dbeea0cfa92b306fb4153704`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.793787+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 21.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontInterruptSprint",
    "BehaviorNoTarget"
  ],
  "BuffModifier": {
    "Class": "CitadelDisruptiveCharge",
    "Subclass": "CitadelDisruptiveCharge"
  },
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "citadel_ability_kali_disruptive_charge",
  "MoveSpeedBonusPercentStart": 120,
  "Name": "Pack Hunter",
  "Radius": 10,
  "Upgrades": [
    {
      "AbilityDuration": 2
    },
    {
      "LifestealPercent": 30
    },
    {
      "BonusFireRate": 40
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_kali",
      "hero_name": "Kali",
      "lookup": "pack hunter",
      "name": "Pack Hunter",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_frank_shocktarget2" title="Pain Battery" -->

## Pain Battery

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_frank_shocktarget2`
- Snapshot ID: `39446`
- Source-Dokument: `7070`
- Kurzinfo: Pain Battery aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Pain Battery`
- Payload Hash: `d72af38ffa6f5dd698bfda8c23af3115d211ee3289ee0e4e2dfd1318e2ec8d68`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.345214+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.35,
  "AbilityCastRange": 28,
  "AbilityChargesConditionally": 1,
  "AbilityCooldown": 2,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "BatteryGenerationPercent": {
    "Scale": {
      "Type": "cooldown",
      "Value": -1.0
    },
    "Value": 100
  },
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BoltCount": 7,
  "BonusShocksDelay": 0.2,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.6
    },
    "Value": 100
  },
  "FullyChargedFXModifier": {
    "Class": "FrankShockfullycharged",
    "Subclass": "ChargedFx"
  },
  "HealOnHit": {
    "Scale": {
      "Type": "healing",
      "Value": 1.0
    },
    "Value": 0
  },
  "IsDisabled": false,
  "Key": "ability_frank_shocktarget2",
  "MissingHealthPercentHeal": {
    "Scale": {
      "Type": "healing",
      "Value": 1.0
    },
    "Value": 0
  },
  "Name": "Pain Battery",
  "ShockModifier": {
    "Class": "FrankShocktarget",
    "Subclass": "Shock"
  },
  "SlowModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "ZapSlow"
  },
  "SpreadAngle": 40,
  "SpreadRandomness": 0.005,
  "StoredDamageHealthPercentRequired": 40,
  "Upgrades": [
    {
      "SlowDuration": "2s",
      "SlowPercent": 40
    },
    {
      "Damage": 50.0
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 0
      },
      "MissingHealthPercentHeal": 15
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
    "card_name": "Pain Battery",
    "hero_key": "hero_frank",
    "hero_name": "Victor",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_frank",
      "hero_name": "Victor",
      "lookup": "pain battery",
      "name": "Pain Battery",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_chrono_swap" title="Paradoxical Swap" -->

## Paradoxical Swap

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chrono_swap`
- Snapshot ID: `39605`
- Source-Dokument: `7070`
- Kurzinfo: Paradoxical Swap aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Paradoxical Swap`
- Payload Hash: `7648af4a04ea7c4d03456c3e8537397243fecd1f482e5770efb8cf55215a387d`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.756042+00:00`

### Vollstaendige Payload

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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_melee_parry" title="Parry" -->

## Parry

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_melee_parry`
- Snapshot ID: `39629`
- Source-Dokument: `7070`
- Kurzinfo: Parry aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Parry`
- Payload Hash: `8afafad70cb93c709605ca150ea293a584e10bdfc37f66164b3119a2c68f8c98`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.813447+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Scale": {
      "Type": "parry_cd",
      "Value": "-1"
    },
    "Value": 4.5
  },
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNotSilencable",
    "BehaviorNoTarget"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "citadel_ability_melee_parry",
  "Name": "Parry",
  "ParriedStunTime": 2.75,
  "ParryActiveModifier": {
    "Class": "CitadelParry",
    "Subclass": "CitadelParry"
  },
  "ParryBossVictimCalmModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "AiForceCalm"
    ],
    "Subclass": "BossVictimCalm"
  },
  "ParryBossVictimNoMeleeModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "MeleeDisabled"
    ],
    "Subclass": "BossVictimNoMelee"
  },
  "ParryCooldownModifier": {
    "Class": "Base",
    "Subclass": "ParryCooldownDisplay"
  },
  "ParryEndVisualModifier": {
    "Class": "Base",
    "StatusEffectPriority": 0,
    "Subclass": "ParryEndVisuals"
  },
  "ParryMoveSpeed": 50,
  "ParryVictimModifier": {
    "Class": "CitadelParriedStun",
    "Subclass": "CitadelParriedStun"
  },
  "Upgrades": [],
  "VictimDamageTakenScale": 25,
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_tier3boss_laser_beam" title="Patron Laser" -->

## Patron Laser

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tier3boss_laser_beam`
- Snapshot ID: `39659`
- Source-Dokument: `7070`
- Kurzinfo: Patron Laser aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Patron Laser`
- Payload Hash: `c3cabd5c6967a0ca852a60fa43d1e89d313002fc943f735910121d1e1cc899a9`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.885249+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 30.48,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BeamModifier": {
    "AuraDropTickRate": 0.5,
    "Class": "Tier3bossElectricBeam",
    "GroundAuraModifier": {
      "AuraRadius": 160.0,
      "Class": "Tier3bossLaserAura",
      "ModifierProvidedByAuraDuration": 4.0,
      "ProvidedByAura": {
        "Class": "Tier3bossLaserDebuff",
        "Duration": 4,
        "MaxHealthDPS": 4,
        "NPCDPS": 80,
        "PlayerDPS": 100,
        "Subclass": "Tier3bossLaserDebuff",
        "TickRate": 0.5
      },
      "Subclass": "Tier3bossLaserAura"
    },
    "LaserDPSMaxHealth": 5,
    "LaserDPSToNPCs": 80.0,
    "LaserDPSToPlayers": 440.0,
    "Subclass": "Tier3bossElectricBeam"
  },
  "BehaviourBits": [
    "BehaviorNoTarget"
  ],
  "BulletArmorReduction": -10,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "citadel_ability_tier3boss_laser_beam",
  "Name": "Patron Laser",
  "PathLength": 2000,
  "PathWidth": 40,
  "TechArmorReduction": -10,
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_trapper_spidershield" title="Pest Barrier" -->

## Pest Barrier

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_spidershield`
- Snapshot ID: `39546`
- Source-Dokument: `7070`
- Kurzinfo: Pest Barrier aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Pest Barrier`
- Payload Hash: `390b02746536386d4732f2df70fcdcbd2e69c563e982649a616bc77311d78886`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.599933+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 45,
  "AbilityCooldown": 30,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorEqualUnitTargetPriority",
    "BehaviorAllowSelfCast",
    "BehaviorCanHealPlayers",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "BuffModifier": {
    "Class": "TrapperSpidershield",
    "PulseDebuffModifier": {
      "Class": "Base",
      "Subclass": "TrapperSpidershieldPulsedebuff"
    },
    "StatusEffectPriority": 100,
    "Subclass": "TrapperSpidershield"
  },
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.046
    },
    "Value": 200
  },
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.651
    },
    "Value": 30
  },
  "DebuffDuration": 0.5,
  "IsDisabled": false,
  "Key": "ability_trapper_spidershield",
  "Name": "Pest Barrier",
  "Radius": 5,
  "SlowPercent": 30,
  "TickRate": 1,
  "Upgrades": [
    {
      "AbilityCooldown": -8
    },
    {
      "CombatBarrier": 200
    },
    {
      "Radius": 5
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
    "card_name": "Pest Barrier",
    "hero_key": "hero_trapper",
    "hero_name": "Trapper",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_trapper",
      "hero_name": "Trapper",
      "lookup": "pest barrier",
      "name": "Pest Barrier",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_viper_ult" title="Petrify" -->

## Petrify

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_viper_ult`
- Snapshot ID: `39567`
- Source-Dokument: `7070`
- Kurzinfo: Petrify aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Petrify`
- Payload Hash: `43bdc221f4945243bca51d6730578e6c73681caad25c0c6e5bf54a1f0ab59c60`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.654058+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 20,
  "AbilityCooldown": 70,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.15,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0695
    },
    "Value": 200
  },
  "HalfHeight": 6,
  "IsDisabled": false,
  "Key": "ability_viper_ult",
  "Name": "Petrify",
  "PetrifyDamageBreakThreshold": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.116
    },
    "Value": 200
  },
  "PetrifyDuration": 3,
  "PetrifyModifier": {
    "Class": "CitadelPetrify",
    "Subclass": "ViperUltPetrify"
  },
  "PreDetonateDuration": 1,
  "Radius": 4,
  "Upgrades": [
    {
      "AbilityCooldown": -15
    },
    {
      "PetrifyDuration": 1.5
    },
    {
      "Radius": 3
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_viper_petrifybola" title="Petrifying Bola" -->

## Petrifying Bola

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_viper_petrifybola`
- Snapshot ID: `39564`
- Source-Dokument: `7070`
- Kurzinfo: Petrifying Bola aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Petrifying Bola`
- Payload Hash: `580d6e7cc989bf4fa0adb2d357937a75de91307e3ac193de4306cbe3bf2ac682`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.646312+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCooldown": 105,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.744
    },
    "Value": 50.0
  },
  "IsDisabled": false,
  "Key": "ability_viper_petrifybola",
  "Name": "Petrifying Bola",
  "PetrifyDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.046
    },
    "Value": 180
  },
  "PetrifyDamageBreakThreshold": 200,
  "PetrifyDuration": 2.0,
  "PetrifyModifier": {
    "Class": "CitadelPetrify",
    "StatusEffectPriority": 0,
    "Subclass": "PetrifybolaPetrify"
  },
  "Radius": 8,
  "SlowDuration": 1.5,
  "SlowModifier": {
    "Class": "Base",
    "Subclass": "PetrifybolaSlow"
  },
  "SlowPercent": 50,
  "Upgrades": [
    {
      "PetrifyDamage": 49.5
    },
    {
      "AbilityCooldown": -20
    },
    {
      "PetrifyDuration": 1.0
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
    "card_name": "Petrifying Bola",
    "hero_key": "hero_viper",
    "hero_name": "Vyper",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_viper",
      "hero_name": "Vyper",
      "lookup": "petrifying bola",
      "name": "Petrifying Bola",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_familiar_ability02" title="Pillow Toss" -->

## Pillow Toss

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_familiar_ability02`
- Snapshot ID: `39428`
- Source-Dokument: `7070`
- Kurzinfo: Pillow Toss aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Pillow Toss`
- Payload Hash: `581320c2550311c2483e01c6172a752791f4e0a1214b32ddad2fe816f098e762`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.299364+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.3,
  "AbilityCastRange": 40,
  "AbilityCharges": 1,
  "AbilityCooldown": 25.0,
  "AbilityCooldownBetweenCharge": 8,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "AutoChannelModifier": {
    "Class": "Base",
    "Subclass": "Channeling"
  },
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectile",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "CDReduceOnPillowHit": 5,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.6
    },
    "Value": 75
  },
  "EffectDuration": 3.0,
  "EffectModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "Slow"
  },
  "FadingSlowPercent": 45,
  "IsDisabled": false,
  "Key": "ability_familiar_ability02",
  "Name": "Pillow Toss",
  "OrbsToFire": 1,
  "Radius": 5,
  "TossDuration": 0.4,
  "TossForce": 300,
  "Upgrades": [
    {
      "AbilityCooldown": -7
    },
    {
      "FireRateSlow": 35,
      "Radius": 2
    },
    {
      "AbilityCharges": 1,
      "Damage": 100
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
    "card_name": "Pillow Toss",
    "hero_key": "hero_familiar",
    "hero_name": "Rem",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_familiar",
      "hero_name": "Rem",
      "lookup": "pillow toss",
      "name": "Pillow Toss",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_bookworm_knightbarrier" title="Plot Armor" -->

## Plot Armor

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_bookworm_knightbarrier`
- Snapshot ID: `39409`
- Source-Dokument: `7070`
- Kurzinfo: Plot Armor aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Plot Armor`
- Payload Hash: `f078a5b9974788738fb0248ce3a3f8f8ac9d6cbbea8e76728c13b67d9ca30c1a`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.252033+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 35,
  "AbilityCooldown": 28.0,
  "AbilityDuration": 5,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "AttackModifier": {
    "Class": "Base",
    "Subclass": "Attackbuff"
  },
  "BarrierModifier": {
    "Class": "BookwormKnightbarrier",
    "DebuffModifier": {
      "Class": "CitadelDisarmProc",
      "EnabledStateMask": [
        "MeleeDisabled"
      ],
      "Subclass": "Disarm"
    },
    "Subclass": "Knightbarrier"
  },
  "BaseAttackDamagePercent": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.2
    },
    "Value": 25
  },
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorAllowSelfCast",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorAllowAltCast",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusFireRate": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "BonusSpiritDamagePercent": 15,
  "BonusTargetRadius": 30,
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.5
    },
    "Value": 125
  },
  "IsDisabled": false,
  "Key": "ability_bookworm_knightbarrier",
  "Name": "Plot Armor",
  "PushForce": 900,
  "ShoveRadius": 6,
  "Upgrades": [
    {
      "BonusFireRate": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.16
        },
        "Value": 14
      }
    },
    {
      "AbilityDuration": 2,
      "CombatBarrier": 100
    },
    {
      "BonusTargets": 2,
      "BonusTargetsBarrierPercentage": 100,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 0
      }
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
    "card_name": "Plot Armor",
    "hero_key": "hero_bookworm",
    "hero_name": "Paige",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_bookworm",
      "hero_name": "Paige",
      "lookup": "plot armor",
      "name": "Plot Armor",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_priest_selfheal" title="Potent Vapors" -->

## Potent Vapors

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_selfheal`
- Snapshot ID: `39514`
- Source-Dokument: `7070`
- Kurzinfo: Potent Vapors aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Potent Vapors`
- Payload Hash: `27afca2d4bec2404bd3d295d45b9c5a92a6848ac7c5c63ccd2648f7d7ffb2d6e`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.520708+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.125,
  "AbilityChannelTime": 4,
  "AbilityCooldown": 45,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.125,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorCastableWhileCmdRestricted",
    "BehaviorDisplaysDamageImpact",
    "BehaviorNonCombat",
    "BehaviorCannotCancelDuringChannel"
  ],
  "ChannelMoveSpeed": -1,
  "FlatHealthHealing": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.79
    },
    "Value": 120
  },
  "IncomingDamagePercent": -35,
  "IsDisabled": false,
  "Key": "ability_priest_selfheal",
  "Name": "Potent Vapors",
  "SelfModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "Disarmed"
    ],
    "Subclass": "Selfheal"
  },
  "TickRate": 0.25,
  "Upgrades": [
    {
      "FlatHealthHealing": 80
    },
    {
      "AbilityCooldown": -15
    },
    {
      "SlowResistance": 100
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_nano_pounce" title="Pounce" -->

## Pounce

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_nano_pounce`
- Snapshot ID: `39489`
- Source-Dokument: `7070`
- Kurzinfo: Pounce aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Pounce`
- Payload Hash: `a714e18caaa34334cd2b424328ee1f6e54531334c8fe5cda118a3f525b0ea669`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.463329+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0744
    },
    "Value": 14
  },
  "AbilityCharges": 2,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": 8,
  "AbilityPostCastDuration": 0.4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorMovement"
  ],
  "CameraDistance": 250,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.651
    },
    "Value": 60
  },
  "DoublePounceModifier": {
    "Class": "Base",
    "Subclass": "DoublePounceNotify"
  },
  "DoublePounceTime": 3,
  "ExplodeRadius": 6,
  "IsDisabled": false,
  "JumpHeight": 3,
  "Key": "ability_nano_pounce",
  "LeapModifier": {
    "Class": "CitadelNanoPounceSelf",
    "Subclass": "CitadelNanoPounceSelf"
  },
  "MinTimeToTarget": 0.5,
  "MoveSpeedToTarget": 25,
  "Name": "Pounce",
  "SlashRange": 3,
  "SlowDuration": 2.0,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "SlowBase"
  },
  "SlowPercent": 30,
  "Upgrades": [
    {
      "SlowDuration": 1
    },
    {
      "ActiveReloadPercent": 20,
      "FireRateSlow": 30
    },
    {
      "AbilityCharges": 1,
      "Damage": 60
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_explosive_barrel" title="Powder Keg" -->

## Powder Keg

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_explosive_barrel`
- Snapshot ID: `39426`
- Source-Dokument: `7070`
- Kurzinfo: Powder Keg aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Powder Keg`
- Payload Hash: `16fcd582436d759ef60b2df7e3b17130585df69ef75950cefc3c45144d7f6c61`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.291324+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.125,
  "AbilityCharges": 2,
  "AbilityCooldown": 28.0,
  "AbilityCooldownBetweenCharge": 7.5,
  "AbilityUnitTargetLimit": 1,
  "ArmTime": 0.1,
  "BarrelDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.2
    },
    "Value": 80
  },
  "BarrelHeavyMeleeForceForward": 1800,
  "BarrelHeavyMeleeForceUp": 300,
  "BarrelLifetime": 8,
  "BarrelLightMeleeForceForward": 1400,
  "BarrelLightMeleeForceUp": 300,
  "BarrelPitchMax": 90,
  "BarrelPitchMin": 2,
  "BarrelRollSpeedMoveAir": 10,
  "BarrelRollSpeedMoveMin": 20,
  "BarrelScale": 1.3,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorIgnoreSelectionMashProtection",
    "BehaviorDontInterruptSlideOnCast",
    "BehaviorAllowAltCast",
    "BehaviorCastImmediateOnOtherAbility"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_explosive_barrel",
  "MinTimeBeforeDestroy": 0.1,
  "Name": "Powder Keg",
  "Radius": 6,
  "TossDuration": 0.4,
  "TossSpeed": 3.556,
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "AbilityCharges": 1
    },
    {
      "AbilityCooldownBetweenCharge": -5,
      "BarrelDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 80
      }
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
    "card_name": "Powder Keg",
    "hero_key": "hero_astro",
    "hero_name": "Holliday",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_astro",
      "hero_name": "Holliday",
      "lookup": "powder keg",
      "name": "Powder Keg",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_power_slash" title="Power Slash" -->

## Power Slash

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_power_slash`
- Snapshot ID: `39633`
- Source-Dokument: `7070`
- Kurzinfo: Power Slash aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Power Slash`
- Payload Hash: `3f8f2b9accb0c9bf95cef26f73c00263f2d86b2a20efcf2cc02f3d25e84a93a5`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.822977+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1.4,
  "AbilityCooldown": 12,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting"
  ],
  "BulletResist": 60,
  "ChannelMoveSpeed": 1.3,
  "FallSpeedMax": 5,
  "FullChargeDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.85
    },
    "Value": 145
  },
  "IsDisabled": false,
  "Key": "citadel_ability_power_slash",
  "MediumChargeDamagePct": 50,
  "Name": "Power Slash",
  "PowerUpStages": 3,
  "ShortChargeDamagePct": 30,
  "SlashCollisionRadius": 4,
  "SlashLength": 22,
  "SlashRadius": 41,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "UnstoppableWhileCastingModifier": {
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
    "Subclass": "YamatoPowerslashUnstoppable"
  },
  "Upgrades": [
    {
      "SlowDuration": 3,
      "SlowPercent": 40
    },
    {
      "AbilityCooldown": -4
    },
    {
      "FullChargeDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 150
      },
      "SlashLength": 8
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
    "card_name": "Power Slash",
    "hero_key": "hero_yamato",
    "hero_name": "Yamato",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_yamato",
      "hero_name": "Yamato",
      "lookup": "power slash",
      "name": "Power Slash",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_power_surge" title="Power Surge" -->

## Power Surge

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_power_surge`
- Snapshot ID: `39508`
- Source-Dokument: `7070`
- Kurzinfo: Power Surge aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Power Surge`
- Payload Hash: `589d9d23f64db5df487b3909da54672ce29df705bb712dc8608ff9c814c90f0d`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.505719+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 50.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 10,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusPerChain": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.32
    },
    "Value": 10
  },
  "BuffModifier": {
    "Class": "PowerSurge",
    "DebuffModifier": {
      "Class": "Base",
      "Subclass": "PowerSurgeDebuff"
    },
    "Subclass": "PowerSurge"
  },
  "ChainCount": 4,
  "ChainModifier": {
    "Class": "PowerSurgeChainLightning",
    "Subclass": "PowerSurgeChainLightning"
  },
  "ChainRadius": 10,
  "ChainTickRate": 0.2,
  "ChannelMoveSpeed": -1,
  "DamagePerChain": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.14
    },
    "Value": 10
  },
  "IsDisabled": false,
  "Key": "ability_power_surge",
  "Name": "Power Surge",
  "Upgrades": [
    {
      "AbilityCooldown": -18.0
    },
    {
      "BonusMoveSpeed": 3,
      "BonusPerChain": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.23
        },
        "Value": 8
      },
      "DamagePerChain": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.23
        },
        "Value": 8
      }
    },
    {
      "AbilityDuration": 10,
      "DebuffDuration": 10,
      "TechResistDebuff": -15
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
    "card_name": "Power Surge",
    "hero_key": "hero_gigawatt",
    "hero_name": "Seven",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_gigawatt",
      "hero_name": "Seven",
      "lookup": "power surge",
      "name": "Power Surge",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_projectmind" title="Project Mind" -->

## Project Mind

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_projectmind`
- Snapshot ID: `39634`
- Source-Dokument: `7070`
- Kurzinfo: Project Mind aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Project Mind`
- Payload Hash: `bfe4a8e201ce01f84c6d361806aff170483f2c1d50c4068d49f05340db45b4e8`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.825225+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.75,
  "AbilityCastRange": 25,
  "AbilityCooldown": 46.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorPreventBotUsage",
    "BehaviorMovement",
    "BehaviorCanSetQuickCast"
  ],
  "CameraDistance": 250,
  "ChannelMoveSpeed": 5.1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "IsDisabled": false,
  "Key": "citadel_ability_projectmind",
  "Name": "Project Mind",
  "ProjectMindModifier": {
    "Class": "CitadelProjectmind",
    "ShieldModifier": {
      "Class": "WraithProjectMindShield",
      "Subclass": "WraithProjectMindShield"
    },
    "Subclass": "CitadelProjectmind"
  },
  "TrailInterval": 0.1,
  "Upgrades": [
    {
      "AbilityCastRange": 15
    },
    {
      "BarrierDuration": 5,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.7
        },
        "Value": 300
      }
    },
    {
      "AbilityCooldown": -32.0
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
    "card_name": "Project Mind",
    "hero_key": "hero_wraith",
    "hero_name": "Wraith",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_wraith",
      "hero_name": "Wraith",
      "lookup": "project mind",
      "name": "Project Mind",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="yakuza_protection_racket" title="Protection Racket" -->

## Protection Racket

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `yakuza_protection_racket`
- Snapshot ID: `39742`
- Source-Dokument: `7070`
- Kurzinfo: Protection Racket aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Protection Racket`
- Payload Hash: `99ba5511f6033fcca4e87b48e01e990265c33c04aff46db73cb9560490ed244f`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.098851+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCastRange": 30,
  "AbilityCooldown": 32.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 12,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorEqualUnitTargetPriority",
    "BehaviorAllowSelfCast"
  ],
  "BonusCritDamagePercent": 20,
  "BulletShieldHealth": 180,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "yakuza_protection_racket",
  "Name": "Protection Racket",
  "Upgrades": [
    {
      "MoveWhileShootingSpeedPenaltyReductionPercent": 60,
      "WeaponRecoilReduction": 60
    },
    {
      "BonusCritDamagePercent": 40
    },
    {
      "BulletShieldHealth": 360
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_yakuza",
      "hero_name": "The Boss",
      "lookup": "protection racket",
      "name": "Protection Racket",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="viscous_telepunch" title="Puddle Punch" -->

## Puddle Punch

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `viscous_telepunch`
- Snapshot ID: `39737`
- Source-Dokument: `7070`
- Kurzinfo: Puddle Punch aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Puddle Punch`
- Payload Hash: `0e07cbcc3514e70352752a2faaedb66fb16b78ff3ae72aec694d31bb5f9cdd57`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.087944+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 40,
  "AbilityCharges": 1,
  "AbilityCooldown": 24.0,
  "AbilityCooldownBetweenCharge": 1.7,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "melee",
      "Value": 0.6
    },
    "Value": 20.0
  },
  "DamageHeavyMelee": {
    "Scale": {
      "Type": "heavy_melee",
      "Value": 0
    },
    "Value": 0
  },
  "FriendlyImpactDuration": 2,
  "FriendlyImpactModifier": {
    "Class": "Base",
    "Subclass": "FriendlyPunchAirControl"
  },
  "ImpactDuration": 4,
  "ImpactModifier": {
    "Class": "SlowBase",
    "Subclass": "ViscousTelepunchMovementSlow"
  },
  "IsDisabled": false,
  "Key": "viscous_telepunch",
  "Name": "Puddle Punch",
  "PunchFriendlyAirControl": 30,
  "PunchHalfHeight": 5.5,
  "PunchRollSlow": -40,
  "PunchRollSlowDuration": 1,
  "PunchRollSlowModifier": {
    "Class": "Base",
    "Subclass": "ViscousPunchRollSlow"
  },
  "Radius": 4,
  "SlowPercent": 20,
  "TossDuration": 0.6,
  "TossGroundSideRatio": 0.7,
  "TossSpeed": 625,
  "TossSpeedUpWall": 500,
  "TossSpeedWall": 750,
  "Upgrades": [
    {
      "AbilityCharges": 1,
      "Damage": 20
    },
    {
      "LifeStealPercentOnHit": 60,
      "Radius": 1.5
    },
    {
      "AbilityCooldown": -14,
      "Damage": {
        "Scale": {
          "Multiply": true,
          "Type": "melee",
          "Value": 0.0
        },
        "Value": -40
      },
      "DamageHeavyMelee": {
        "Scale": {
          "Type": "heavy_melee",
          "Value": 0.6
        },
        "Value": 40
      },
      "UseHeavyMelee": 1
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
    "card_name": "Puddle Punch",
    "hero_key": "hero_viscous",
    "hero_name": "Viscous",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_viscous",
      "hero_name": "Viscous",
      "lookup": "puddle punch",
      "name": "Puddle Punch",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="rutger_pulse" title="Pulse" -->

## Pulse

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `rutger_pulse`
- Snapshot ID: `39703`
- Source-Dokument: `7070`
- Kurzinfo: Pulse aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Pulse`
- Payload Hash: `5a58648d52d35c1288957998219dbf957a64180c7aeb9bfb831b5c3a23169b30`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.996078+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 127.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "AuraModifier": {
    "Class": "RutgerPulseAura",
    "ProvidedByAura": {
      "Class": "RutgerPulseTarget",
      "Subclass": "RutgerPulseTargetSubclass"
    },
    "Subclass": "RutgerPulseAuraSubclass"
  },
  "BehaviourBits": [
    "BehaviorNoTarget"
  ],
  "ChannelMoveSpeed": -1,
  "DamageMax": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 350
  },
  "DamageMax_DistanceFuzz": 2,
  "DamageMin": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 50
  },
  "EndRadius": 30,
  "IsDisabled": false,
  "Key": "rutger_pulse",
  "MovementSlow": 25,
  "Name": "Pulse",
  "SpreadDuration": 0.6,
  "StartRadius": 1,
  "Upgrades": [
    {
      "MovementSlow": 25
    },
    {
      "AbilityCooldown": -47.0
    },
    {
      "DamageMax": 200
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_rutger",
      "hero_name": "Rutger",
      "lookup": "pulse",
      "name": "Pulse",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="tokamak_crimson_cannon" title="Pulse Cannon" -->

## Pulse Cannon

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `tokamak_crimson_cannon`
- Snapshot ID: `39723`
- Source-Dokument: `7070`
- Kurzinfo: Pulse Cannon aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Pulse Cannon`
- Payload Hash: `380c21fe37817fabed90df775d36e947446c242d00703e7b1e1cbc5e87bbea95`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.049374+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 2.0,
  "AbilityCastRange": 100,
  "AbilityChannelTime": 3.6,
  "AbilityCooldown": 127.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AimFOV": 60,
  "AimZoomDuration": 0.15,
  "AirSpeedMax": 70,
  "AutoCastDelayModifier": {
    "Class": "Base",
    "Subclass": "CastDelay"
  },
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.462406
    },
    "Value": 190
  },
  "DelayBetweenShots": 0.6,
  "FallSpeedMax": 1,
  "IsDisabled": false,
  "Key": "tokamak_crimson_cannon",
  "Name": "Pulse Cannon",
  "TargetingWidth": 0.8,
  "Upgrades": [
    {
      "DelayBetweenShots": -0.2
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.462406
        },
        "Value": 50
      }
    },
    {
      "AbilityCooldown": -47.0
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_tokamak",
      "hero_name": "Tokamak",
      "lookup": "pulse cannon",
      "name": "Pulse Cannon",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_chrono_pulse_grenade" title="Pulse Grenade" -->

## Pulse Grenade

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chrono_pulse_grenade`
- Snapshot ID: `39604`
- Source-Dokument: `7070`
- Kurzinfo: Pulse Grenade aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Pulse Grenade`
- Payload Hash: `2682a7d70ee5e2d533269704f4df5b1424490acc2fe305c7d016becb74a0352d`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.753393+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 32.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 3.2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DamageAmplificationPerStack": 4,
  "DebuffDuration": 8.0,
  "IsDisabled": false,
  "Key": "citadel_ability_chrono_pulse_grenade",
  "MovementSlowDuration": 0.2,
  "Name": "Pulse Grenade",
  "PulseAreaModifier": {
    "Class": "ChronoPulseGrenadePulseArea",
    "DebuffModifier": {
      "Class": "ChronoPulseGrenadeDebuff",
      "Subclass": "ChronoPulseGrenadeDebuff"
    },
    "SlowModifier": {
      "Class": "PulsegrenadeTimeslow",
      "Subclass": "Slow"
    },
    "Subclass": "ChronoPulseGrenadePulseArea"
  },
  "PulseDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.3
    },
    "Value": 35
  },
  "PulseInterval": 0.8,
  "Radius": 5.5,
  "RadiusIncreasePerPulse": 1,
  "SlowPercent": 20,
  "Upgrades": [
    {
      "AbilityCooldown": -12
    },
    {
      "PulseDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 20
      }
    },
    {
      "AbilityDuration": 1.6,
      "DamageAmplificationPerStack": 4
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
    "card_name": "Pulse Grenade",
    "hero_key": "hero_gunslinger",
    "hero_name": "Gunslinger",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_chrono",
      "hero_name": "Paradox",
      "lookup": "pulse grenade",
      "name": "Pulse Grenade",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_void_sphere" title="Quantum Entanglement" -->

## Quantum Entanglement

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_void_sphere`
- Snapshot ID: `39664`
- Source-Dokument: `7070`
- Kurzinfo: Quantum Entanglement aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Quantum Entanglement`
- Payload Hash: `00828e47d728a47184088500fb3977e8f84db6696bb95cbb821bacd1cfd52854`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.895561+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 10,
  "AbilityCooldown": 20,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 1.4,
  "AbilityUnitTargetLimit": 1,
  "AllyDistance": 13,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventBotUsage",
    "BehaviorAllowAltCast",
    "BehaviorMovement",
    "BehaviorCanSetQuickCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BubbleModifier": {
    "BuffModifier": {
      "Class": "VoidsphereBuff",
      "Subclass": "VoidsphereBuff"
    },
    "Class": "VoidSphere",
    "StatusEffectPriority": 100,
    "Subclass": "VoidSphere"
  },
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "citadel_ability_void_sphere",
  "Name": "Quantum Entanglement",
  "StaminaRestore": 1,
  "TrailInterval": 0.01,
  "Upgrades": [
    {
      "AbilityCastRange": 6
    },
    {
      "AbilityCooldown": -6
    },
    {
      "ChargeReplenish": 1,
      "ReduceDebuffs": 50
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
    "card_name": "Quantum Entanglement",
    "hero_key": "hero_dynamo",
    "hero_name": "Dynamo",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_dynamo",
      "hero_name": "Dynamo",
      "lookup": "quantum entanglement",
      "name": "Quantum Entanglement",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_nano_shadow_step" title="Queen of Shadows" -->

## Queen of Shadows

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_nano_shadow_step`
- Snapshot ID: `39493`
- Source-Dokument: `7070`
- Kurzinfo: Queen of Shadows aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Queen of Shadows`
- Payload Hash: `7a2a7018fd218c65f0b074f9d42ab8cf10483ccc39f82b5452f9288671e893d8`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.472784+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 110.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0465
    },
    "Value": 12
  },
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget"
  ],
  "BulletArmorReductionDuration": 6,
  "BulletArmorReductionHeavy": 15,
  "BulletArmorReductionLight": 5,
  "ChannelMoveSpeed": -1,
  "DamageAmplification": 20,
  "InvisAlertWhenFading": 1,
  "InvisFadeToDuration": 0.25,
  "InvisMoveSpeedMod": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0372
    },
    "Value": 2.0
  },
  "IsDisabled": false,
  "Key": "ability_nano_shadow_step",
  "MeleeAttackSpeedBonus": 20,
  "Name": "Queen of Shadows",
  "PurgeModifier": {
    "Class": "Base",
    "Subclass": "Purge"
  },
  "RevealOnDamageDuration": 0.7,
  "RevealOnSpottedDuration": 0.7,
  "ShadowModifier": {
    "Class": "NanoShadowStep",
    "DesatFactor": 0.6,
    "MaxCloak": 0.99,
    "MinCloak": 0.7,
    "SilenceModifier": {
      "Class": "CitadelSilenced",
      "Subclass": "CitadelSilenced"
    },
    "SlowModifier": {
      "Class": "SlowBase",
      "Subclass": "SlowBase"
    },
    "Subclass": "NanoShadowStep"
  },
  "SlowPercent": 30,
  "SpottedRadius": 15,
  "Upgrades": [
    {
      "StaminaCooldownReduction": 30
    },
    {
      "PurgeOnActivate": 1,
      "SlowResistancePercent": 40
    },
    {
      "DamageAmplification": 20,
      "SilenceOnHeavyDuration": 3
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_magician_animalcurse" title="Rabbit Hex" -->

## Rabbit Hex

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_animalcurse`
- Snapshot ID: `39472`
- Source-Dokument: `7070`
- Kurzinfo: Rabbit Hex aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rabbit Hex`
- Payload Hash: `0ad91c04ac5471de4b2347c5f6112f44ccf95041fe43d65b7c475378c5314adb`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.420794+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.35,
  "AbilityCastRange": 20,
  "AbilityCooldown": 45,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 2,
  "AbilityPostCastDuration": 0.1,
  "AbilityUnitTargetLimit": 1,
  "AirDampingDuration": 1,
  "AirDampingModifier": {
    "Class": "Airdamp",
    "Subclass": "AirdampAnimalcurse"
  },
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorAllowSelfCast",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "CurseModifier": {
    "Class": "CitadelAnimalcurse",
    "ModelScale": 0.75,
    "Subclass": "AnimalhexCurseModifier"
  },
  "DamageAmpPercentage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.093
    },
    "Value": 25
  },
  "HexMoveSpeedLimit": 6,
  "IsDisabled": false,
  "Key": "ability_magician_animalcurse",
  "MoveSpeedBonusPct": 35,
  "Name": "Rabbit Hex",
  "SelfBumpImpulse": 500,
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "AbilityDuration": 1
    },
    {
      "Radius": 6
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_magician_animalhexarea" title="Rabbit Hex" -->

## Rabbit Hex

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_animalhexarea`
- Snapshot ID: `39473`
- Source-Dokument: `7070`
- Kurzinfo: Rabbit Hex aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rabbit Hex`
- Payload Hash: `d57ce14fd2a88affc8f1246ad6a72ad7f36f0de09c90c189875d0fc3f551d856`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.423736+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 24,
  "AbilityCooldown": 26,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AirDampingDuration": 1,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DamageAmpPercentage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0558
    },
    "Value": 15
  },
  "DetonationDelay": 0.9,
  "HexAreaModifier": {
    "Class": "MagicianAnimalcurseHexarea",
    "HexModifier": {
      "Class": "CitadelAnimalcurse",
      "ModelScale": 0.75,
      "Subclass": "AnimalhexCurseModifier"
    },
    "Subclass": "HexareaModifier"
  },
  "HexDuration": 2,
  "HexMoveSpeedLimit": 6,
  "IsDisabled": false,
  "Key": "ability_magician_animalhexarea",
  "MoveSpeedBonusPct": 36,
  "Name": "Rabbit Hex",
  "Radius": 6.5,
  "SelfBumpImpulse": 500,
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "HexDuration": 1
    },
    {
      "DamageAmpPercentage": 7,
      "Radius": 3
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
    "card_name": "Rabbit Hex",
    "hero_key": "hero_magician",
    "hero_name": "Sinclair",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_magician",
      "hero_name": "Sinclair",
      "lookup": "rabbit hex",
      "name": "Rabbit Hex",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_unicorn_luminousstrike" title="Radiant Daggers" -->

## Radiant Daggers

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_unicorn_luminousstrike`
- Snapshot ID: `39552`
- Source-Dokument: `7070`
- Kurzinfo: Radiant Daggers aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Radiant Daggers`
- Payload Hash: `49471c3b8c32711dd5785297c782cb504a7b7eca9d1963f6ae821e25515dddbf`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.617331+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 30,
  "AbilityCharges": 1,
  "AbilityCooldown": 33,
  "AbilityCooldownBetweenCharge": 2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorCanSetQuickCast"
  ],
  "BuffDelay": 0.75,
  "BuffDuration": 30,
  "BuffMaxStacks": 6,
  "BuffModifier": {
    "Class": "UnicornLuminousstrikeBuff",
    "Subclass": "LuminousStrikeBuff"
  },
  "ChannelMoveSpeed": -1,
  "ClimbHeight": 50.0,
  "ExplosionInterval": 0.7,
  "ExplosionRadius": 8,
  "ImpactDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.63
    },
    "Value": 55
  },
  "IsDisabled": false,
  "Key": "ability_unicorn_luminousstrike",
  "MagicIncreasePerStack": 8,
  "Name": "Radiant Daggers",
  "PostExplosionDuration": 0.8,
  "PreExplosionDuration": 1.4,
  "TickRate": 0.5,
  "Upgrades": [
    {
      "AbilityCharges": 2
    },
    {
      "AbilityCooldown": -22,
      "ImpactDamage": 80
    },
    {
      "FireRatePerStack": 9,
      "MagicIncreasePerStack": 3
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
    "card_name": "Radiant Daggers",
    "hero_key": "hero_unicorn",
    "hero_name": "Celeste",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_unicorn",
      "hero_name": "Celeste",
      "lookup": "radiant daggers",
      "name": "Radiant Daggers",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_power_jump" title="Rain of Arrows" -->

## Rain of Arrows

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_power_jump`
- Snapshot ID: `39507`
- Source-Dokument: `7070`
- Kurzinfo: Rain of Arrows aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rain of Arrows`
- Payload Hash: `0a52b375ffdc66c27c69c672fd6d07e83384ccbdc6ebac08580a5c9b3aad4a6d`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.502815+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 25.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4,
  "AbilityUnitTargetLimit": 1,
  "AirMoveIncreasePercent": 25,
  "AirSpeedMax": 6.64,
  "AltJumpSpeed": 12,
  "BehaviourBits": [
    "BehaviorInputDirectional2d",
    "BehaviorNoTarget",
    "BehaviorAllowAltCast",
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BulletSplitShot": 5,
  "FallSpeedMax": 0.635,
  "FxRadius": 4,
  "InAirModifier": {
    "Class": "AirRaid",
    "EnabledStateMask": [
      "DisableAirSpreadPenalty",
      "UnlimitedAirDashes"
    ],
    "SlowModifier": {
      "Class": "SlowBase",
      "Subclass": "SlowBase"
    },
    "Subclass": "AirRaid"
  },
  "IsDisabled": false,
  "JumpPitch": -60,
  "JumpSpeed": 27.5,
  "Key": "ability_power_jump",
  "Name": "Rain of Arrows",
  "PowerJumpModifier": {
    "AirDrag": 2.0,
    "Class": "PowerJump",
    "EnabledStateMask": [
      "AbilityMovement",
      "ZiplineDisabled",
      "JumpDisabled"
    ],
    "Subclass": "PowerJump",
    "VerticalCameraOffset": 20.0,
    "VerticalCameraOffsetLerpTime": 0.4
  },
  "Upgrades": [
    {
      "SlowDuration": 1.5,
      "SlowPercent": 30,
      "WeaponDamageBonus": 3
    },
    {
      "AbilityCooldown": -12.0
    },
    {
      "BulletLifestealPercent": 30,
      "EvasionPercent": 30,
      "TechLifestealPercent": 30
    }
  ],
  "WeaponDamageBonus": 3,
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
    "card_name": "Rain of Arrows",
    "hero_key": "hero_orion",
    "hero_name": "Grey Talon",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_orion",
      "hero_name": "Grey Talon",
      "lookup": "rain of arrows",
      "name": "Rain of Arrows",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_vampirebat_steallife" title="Rake" -->

## Rake

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_vampirebat_steallife`
- Snapshot ID: `39560`
- Source-Dokument: `7070`
- Kurzinfo: Rake aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rake`
- Payload Hash: `392dbeee49723c35446282f1ce25593789c982fc4fd3acebbe18ee3daf107403`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.636630+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 10,
  "AbilityCooldown": 16,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.4,
  "AbilityUnitTargetLimit": 6,
  "AirDrag": 0.2,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast",
    "BehaviorUseLagCompensationForUnitTargeting"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "stats_count",
      "Value": 1.0
    },
    "Value": 60
  },
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "Debuff"
  },
  "FallSpeedMax": 3,
  "FallingDrag": 20,
  "FloatingModifier": {
    "Class": "Base",
    "Subclass": "Floatingmodifier"
  },
  "IsDisabled": false,
  "Key": "ability_vampirebat_steallife",
  "MaxFloatTime": 4.0,
  "MiniJumpVelocity": 200,
  "MissingHealthDamagePercentage": 6,
  "Name": "Rake",
  "RakeHealPerKill": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.3
    },
    "Value": 25
  },
  "TargetingConeAngle": 60,
  "TimeBetweenAttacks": 0.04,
  "TrooperExecuteThreshold": 60,
  "Upgrades": [
    {
      "Damage": 60
    },
    {
      "AbilityCooldown": -8,
      "RakeHealPerKill": 30
    },
    {
      "MissingHealthDamagePercentage": 7.0,
      "RakeHealPerKill": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.2
        },
        "Value": 0
      }
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
    "card_name": "Rake",
    "hero_key": "hero_vampirebat",
    "hero_name": "Mina",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_vampirebat",
      "hero_name": "Mina",
      "lookup": "rake",
      "name": "Rake",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_bookworm_knightcharge" title="Rallying Charge" -->

## Rallying Charge

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_bookworm_knightcharge`
- Snapshot ID: `39410`
- Source-Dokument: `7070`
- Kurzinfo: Rallying Charge aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rallying Charge`
- Payload Hash: `0460479a5ac93eda3803055f368f5ba3856b39b22bf0c2d3b8f0c62c10b4b5da`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.254923+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 600,
  "AbilityChannelTime": 0.7,
  "AbilityCooldown": 220,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 13,
  "AbilityUnitTargetLimit": 1,
  "AirDrag": 0.8,
  "AllyHeight": 20,
  "AllyRadius": 4,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorRefundHalfCooldownOnChannelInterrupt"
  ],
  "BonusMoveSpeed": 5,
  "BuffDuration": 9,
  "BuffModifier": {
    "Class": "BookwormKnightchargeBuff",
    "Subclass": "Buff"
  },
  "CancelCooldownRefundPercentage": 50,
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 125
  },
  "FallSpeedMax": 20,
  "GravityAcceleration": -1900,
  "GroundStickHeight": 0.05,
  "HealAmount": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.6
    },
    "Value": 125
  },
  "IsDisabled": false,
  "Key": "ability_bookworm_knightcharge",
  "KnightBonusPerWave": -99,
  "KnightChargeHeight": 3.5,
  "KnightChargeWidth": 1.7,
  "KnightCount": 5,
  "KnightCountInFirstWave": 5,
  "KnightJumpSpeed": 900,
  "KnightMaxFallHeight": -35,
  "KnightMaxJumpHeight": 30,
  "KnightNavForwardDistance": 8,
  "KnightNavSearchDistance": 10,
  "KnightPositionSpread": 1.8,
  "KnightPositionStagger": -4,
  "KnightWhiskerLength": 300,
  "KnightWhiskerSide": 50,
  "KnightWhiskerStrength": 0.2,
  "MaxAmp": 100,
  "MaxAmpDistance": 250,
  "Name": "Rallying Charge",
  "StunDuration": 1.0,
  "TargetFindingDelay": 0.04,
  "TossBackSpeed": 100,
  "TossUpSpeed": 600,
  "Upgrades": [
    {
      "HealAmount": 150
    },
    {
      "AbilityCooldown": -45,
      "KnightCount": 4,
      "KnightCountInFirstWave": 4
    },
    {
      "Damage": 160.0,
      "MaxAmp": 70,
      "StunDuration": 0.5
    }
  ],
  "WaveCount": 2,
  "WavePositionStagger": -15,
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
    "card_name": "Rallying Charge",
    "hero_key": "hero_bookworm",
    "hero_name": "Paige",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_bookworm",
      "hero_name": "Paige",
      "lookup": "rallying charge",
      "name": "Rallying Charge",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="gunslinger_rapid_fire" title="Rapid Fire" -->

## Rapid Fire

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `gunslinger_rapid_fire`
- Snapshot ID: `39686`
- Source-Dokument: `7070`
- Kurzinfo: Rapid Fire aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rapid Fire`
- Payload Hash: `f756ef07813590f3b19a76b91aafd72054edc097882b042c0fbfd115cb85729c`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.949213+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 15,
  "AbilityChannelTime": 1.5,
  "AbilityCooldown": 10.5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoChannelModifier": {
    "AirDrag": 2.0,
    "Class": "CitadelRapidFire",
    "Subclass": "CitadelRapidFire"
  },
  "BehaviourBits": [
    "BehaviorChannelled"
  ],
  "BulletAccuracy": 200,
  "BulletSpeedPercent": 100,
  "CastRangeWhenVertical": 30,
  "CastRangeWhenVerticalBias": 0.3,
  "ChannelMoveSpeed": 1.3,
  "FireRateMultiplier": 100,
  "IsDisabled": false,
  "JuggleAirSpeedMax": 50,
  "JuggleFallSpeedMax": 20,
  "Key": "gunslinger_rapid_fire",
  "Name": "Rapid Fire",
  "Upgrades": [],
  "WeaponDamageScale": -75,
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="fathom_reefdweller_harpoon" title="Reefdweller Harpoon" -->

## Reefdweller Harpoon

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `fathom_reefdweller_harpoon`
- Snapshot ID: `39678`
- Source-Dokument: `7070`
- Kurzinfo: Reefdweller Harpoon aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Reefdweller Harpoon`
- Payload Hash: `7f9b0e5b702dc8bb31a9982d72e71f15119c8e1f51742f27021b75a0dff36936`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.929637+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 30,
  "AbilityCharges": 2,
  "AbilityChargesConditionally": 1,
  "AbilityCooldown": 30,
  "AbilityCooldownBetweenCharge": 2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontBreakInvisibility",
    "BehaviorMovement"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "fathom_reefdweller_harpoon",
  "Name": "Reefdweller Harpoon",
  "ReelSpeed": 1500,
  "Upgrades": [
    {
      "AbilityCooldown": -5
    },
    {
      "AbilityCastRange": 5
    },
    {
      "BonusFireRate": 30,
      "DetachBuffDuration": 7
    }
  ],
  "WallLatchIdealDist": 5,
  "WallLatchSettleDist": 40,
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
    "card_name": "Reefdweller Harpoon",
    "hero_key": "hero_slork",
    "hero_name": "Fathom",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_slork",
      "hero_name": "Fathom",
      "lookup": "reefdweller harpoon",
      "name": "Reefdweller Harpoon",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_nikuman" title="Rejuvenating Aurora" -->

## Rejuvenating Aurora

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_nikuman`
- Snapshot ID: `39631`
- Source-Dokument: `7070`
- Kurzinfo: Rejuvenating Aurora aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rejuvenating Aurora`
- Payload Hash: `ef96e504fd501bc2f9bc4667dd578f2091486274072f740e8631194d5ff93560`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.818120+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": 5,
  "AbilityCooldown": 48.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AuraLingerDuration": 1.0,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorCanHealPlayers",
    "BehaviorDisplaysDamageImpact",
    "BehaviorRequireAbilityButtonToCancel",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "HealingPerSecond": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.4
    },
    "Value": 30
  },
  "IsDisabled": false,
  "Key": "citadel_ability_nikuman",
  "Name": "Rejuvenating Aurora",
  "NikumanModifier": {
    "Class": "Nikuman",
    "ProvidedByAura": {
      "Class": "Base",
      "Subclass": "HealTarget"
    },
    "Subclass": "Nikuman"
  },
  "ShareWithFriendsRadius": 8,
  "Upgrades": [
    {
      "MovementSpeedBonus": 4,
      "MovementSpeedBonusDuration": 8
    },
    {
      "AbilityChannelTime": 1.0,
      "AbilityCooldown": -20.0
    },
    {
      "HealMaxHealthPercent": 2.5,
      "NoChannel": 1
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
    "card_name": "Rejuvenating Aurora",
    "hero_key": "hero_dynamo",
    "hero_name": "Dynamo",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_dynamo",
      "hero_name": "Dynamo",
      "lookup": "rejuvenating aurora",
      "name": "Rejuvenating Aurora",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="drifter_blood_blast" title="Rend" -->

## Rend

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `drifter_blood_blast`
- Snapshot ID: `39671`
- Source-Dokument: `7070`
- Kurzinfo: Rend aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rend`
- Payload Hash: `00c1c6ec6e9302dba43a439366ec12a78b433846b9e76858319da05a9f4ea953`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.910565+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.4,
  "AbilityCastRange": 16,
  "AbilityCooldown": 16.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.4,
  "AbilityUnitTargetLimit": 30,
  "AirSpeedMax": 70,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorNoTarget",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.8
    },
    "Value": 40.0
  },
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "melee",
      "Value": 1.2
    },
    "Value": 0.0
  },
  "DamageHeavyMelee": {
    "Scale": {
      "Type": "heavy_melee",
      "Value": 0
    },
    "Value": 0
  },
  "DebuffModifier": {
    "Class": "CitadelSilenced",
    "Subclass": "Debuff"
  },
  "ExtraSweepConeAngle": 60,
  "ExtraSweepOffsetBehindCaster": 80,
  "ExtraSweepRange": 3,
  "FallSpeedMax": 1,
  "IsDisabled": false,
  "Key": "drifter_blood_blast",
  "Name": "Rend",
  "RangeForBonusDamage": 8,
  "TargetModifier": {
    "Class": "DrifterRendBulletLifesteal",
    "Subclass": "DrifterRendBulletLifesteal"
  },
  "TargetingConeAngle": 50,
  "Upgrades": [
    {
      "BonusDamage": 40
    },
    {
      "AbilityCooldown": -8.0
    },
    {
      "Damage": {
        "Scale": {
          "Multiply": true,
          "Type": "melee",
          "Value": 0.0
        },
        "Value": 0
      },
      "DamageHeavyMelee": {
        "Scale": {
          "Type": "heavy_melee",
          "Value": 0.55
        },
        "Value": 0
      },
      "DebuffDuration": 2.3,
      "UseHeavyMelee": 1
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
    "card_name": "Rend",
    "hero_key": "hero_drifter",
    "hero_name": "Drifter",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_drifter",
      "hero_name": "Drifter",
      "lookup": "rend",
      "name": "Rend",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_nano_shadow_pulse" title="Return to Shadows" -->

## Return to Shadows

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_nano_shadow_pulse`
- Snapshot ID: `39492`
- Source-Dokument: `7070`
- Kurzinfo: Return to Shadows aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Return to Shadows`
- Payload Hash: `3cb1a53f3626b108b6c3bd22a0a86c8c63048286bac2b5bc181f376abcfcb8ee`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.470409+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": 3,
  "AbilityCooldown": 115,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AirDrag": 4,
  "AirSpeedMax": 100,
  "AutoChannelModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "DoNotDrawModel"
    ],
    "Subclass": "Channeling"
  },
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorCastableWhileDodging",
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast",
    "BehaviorInhibitSoftCameraCollision"
  ],
  "BonusMoveSpeedPercent": 20,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 150.0
  },
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "Buff"
  },
  "FallSpeedMax": 0.254,
  "IsDisabled": false,
  "Key": "ability_nano_shadow_pulse",
  "Name": "Return to Shadows",
  "Radius": 7.5,
  "Upgrades": [
    {
      "AbilityCooldown": -20
    },
    {
      "BonusMoveSpeedPercent": 20,
      "Damage": 75
    },
    {
      "HealAmount": 450,
      "RefundCooldowns": 1
    }
  ],
  "ZAcceleration": 800,
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
    "card_name": "Return to Shadows",
    "hero_key": "hero_nano",
    "hero_name": "Calico",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_nano",
      "hero_name": "Calico",
      "lookup": "return to shadows",
      "name": "Return to Shadows",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="operative_revelation" title="Revelation" -->

## Revelation

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `operative_revelation`
- Snapshot ID: `39699`
- Source-Dokument: `7070`
- Kurzinfo: Revelation aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Revelation`
- Payload Hash: `7104705227904cb4e883ec000562bdb835a20cc4795c1c6d3411923067f013c1`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.983078+00:00`

### Vollstaendige Payload

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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_fencer_riposte" title="Riposte" -->

## Riposte

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_fencer_riposte`
- Snapshot ID: `39433`
- Source-Dokument: `7070`
- Kurzinfo: Riposte aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Riposte`
- Payload Hash: `68dffef73ee5b8960c2a2a019d373ac500b713b432d8c72a73ce061346d962d4`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.313715+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": 0.8,
  "AbilityCooldown": 22,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityLifestealPercentHero": 50,
  "AbilityUnitTargetLimit": 1,
  "AutoChannelModifier": {
    "Class": "Base",
    "Subclass": "RiposteCastModifier"
  },
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "CounterattackAntiMashDelay": 0.2,
  "DamageThreshold": {
    "Scale": {
      "Type": "power_increase",
      "Value": 4
    },
    "Value": 60
  },
  "DampingFactor": 0.5,
  "DashGraceWindow": 1.3,
  "DashRadius": 2.2,
  "DashRange": 35,
  "DashSpeed": 76.2,
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "FencerRiposteAttackDebuff"
  },
  "IsDisabled": false,
  "Key": "ability_fencer_riposte",
  "LiftHeight": 240,
  "MeleeResistReduction": -22,
  "MeleeResistReductionDuration": 3.0,
  "MoveSpeedMax": 4,
  "Name": "Riposte",
  "ParryWindow": 0.3,
  "SideMoveSpeed": -100,
  "SlashConeAngle": 90,
  "SlashHalfWidth": 1,
  "SlashRadius": 6,
  "SlowDuration": 4,
  "SlowPercent": 40,
  "StunDuration": 0.6,
  "TurnRateMax": 10,
  "Upgrades": [
    {
      "AbilityCooldown": -8
    },
    {
      "MeleeResistReduction": -30
    },
    {
      "StunDuration": 1.6
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
    "card_name": "Riposte",
    "hero_key": "hero_fencer",
    "hero_name": "Apollo",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_fencer",
      "hero_name": "Apollo",
      "lookup": "riposte",
      "name": "Riposte",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_punkgoat_goatflip" title="Rising Ram" -->

## Rising Ram

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_punkgoat_goatflip`
- Snapshot ID: `39520`
- Source-Dokument: `7070`
- Kurzinfo: Rising Ram aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rising Ram`
- Payload Hash: `c7e6781570a2295d2c05eb87cf663b9029cd4caa13d98e8a7ece1bc11cf6972f`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.533669+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.35,
  "AbilityCooldown": 32,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 0.3,
  "AbilityUnitTargetLimit": 1,
  "AirControlAccelPercent": 50.0,
  "AirControlDashReductionPct": -70.0,
  "AirControlDebuffDuration": 1.5,
  "AirControlPercent": 50.0,
  "AllowRamMultiple": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "CameraTurnRateMax": 188,
  "ChargeMultiHitRadius": 1.5,
  "ChargeRadius": 2.54,
  "ChargeSpeed": 1200,
  "ChargeStrikeDistance": 165,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.9
    },
    "Value": 40
  },
  "DealMaxHealthDamagePct": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "GoingBackAwaySpeed": -100,
  "GoingUpDistance": 3.1,
  "GoingUpEnemyDistancePercent": 95,
  "GoingUpSpeed": 430,
  "HoverGravityScale": 0.75,
  "IsDisabled": false,
  "Key": "ability_punkgoat_goatflip",
  "KnockAwaySpeed": 170,
  "Name": "Rising Ram",
  "NearbyHeroKillDistance": 10,
  "ReduceCooldownOnHitPct": 50,
  "TimeBeforeGoUpForLagComp": 0.1,
  "TimeGoingUpEnemy": 0.2,
  "Upgrades": [
    {
      "WeaponDamageBurst": 25,
      "WeaponDamageBurstDuration": 5
    },
    {
      "AbilityDuration": 0.4
    },
    {
      "AbilityCooldown": -13,
      "DealMaxHealthDamagePct": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.035
        },
        "Value": 8
      }
    }
  ],
  "WorldImpactRadius": 25,
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
    "card_name": "Rising Ram",
    "hero_key": "hero_punkgoat",
    "hero_name": "Billy",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_punkgoat",
      "hero_name": "Billy",
      "lookup": "rising ram",
      "name": "Rising Ram",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_tier2boss_rocket_barrage" title="Rocket Barrage" -->

## Rocket Barrage

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tier2boss_rocket_barrage`
- Snapshot ID: `39657`
- Source-Dokument: `7070`
- Kurzinfo: Rocket Barrage aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rocket Barrage`
- Payload Hash: `542a623342fd21db4b312ea1556a75f78998ab4fb2bed49f572ccc689c81ebb8`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.879713+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 35.56,
  "AbilityCooldown": 10,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 100,
  "AuraModifier": {
    "Class": "Tier2bossRocketDamageAura",
    "ModifierProvidedByAuraDuration": 4.0,
    "ProvidedByAura": {
      "Class": "Tier2bossRocketDamageAuraDebuff",
      "Subclass": "Tier2bossRocketDamageAuraDebuff"
    },
    "Subclass": "Tier2bossRocketDamageAura"
  },
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorTargetThroughWalls"
  ],
  "BulletArmorReduction": -10,
  "ChannelMoveSpeed": -1,
  "Damage": 200,
  "DetonateTimer": 10,
  "ExplosionFalloffDisabled": 1,
  "FireDPS": 25,
  "FireDPSTrooperFactor": 0.3,
  "FireDuration": 5,
  "FireRadius": 6,
  "FireTickInterval": 0.5,
  "GrenadesInVolley": 6,
  "InitialVolleyInaccuracy": 100,
  "IsDisabled": false,
  "Key": "citadel_ability_tier2boss_rocket_barrage",
  "MaxSimultaneousVolley": 1,
  "Name": "Rocket Barrage",
  "PerVolleyInaccuracy": 25,
  "Radius": 5,
  "TechArmorReduction": -10,
  "Upgrades": [],
  "VolleyInterval": 0.5,
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="gunslinger_rocket_launcher" title="Rocket Launcher" -->

## Rocket Launcher

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `gunslinger_rocket_launcher`
- Snapshot ID: `39687`
- Source-Dokument: `7070`
- Kurzinfo: Rocket Launcher aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rocket Launcher`
- Payload Hash: `1408467103164c5be41ede2842288022698a17df8212bad4d543289e4a7bd195`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.952259+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCharges": 1,
  "AbilityCooldown": 10.5,
  "AbilityCooldownBetweenCharge": 0.8,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 100
  },
  "ExplosionRadius": 3,
  "IsDisabled": false,
  "Key": "gunslinger_rocket_launcher",
  "LaunchMaxSpeed": 750,
  "LaunchMinSpeed": 700,
  "LaunchMinVerticalAmount": 0.7,
  "LaunchVerticalBias": 0.5,
  "Name": "Rocket Launcher",
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="rutger_rocket" title="Rocket Launcher" -->

## Rocket Launcher

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `rutger_rocket`
- Snapshot ID: `39704`
- Source-Dokument: `7070`
- Kurzinfo: Rocket Launcher aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rocket Launcher`
- Payload Hash: `42d1ecb84c863f768b3f9d68c2a06b9e6e2a731e3d514d53762a3459a0d5bcb1`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.999753+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCharges": 1,
  "AbilityCooldown": 17.0,
  "AbilityCooldownBetweenCharge": 1,
  "AbilityUnitTargetLimit": 1,
  "AirSpeedMax": 3.81,
  "AutoChannelModifier": {
    "Class": "IntrinsicBase",
    "Subclass": "IntrinsicBase"
  },
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDontTriggerSpellBlock",
    "BehaviorDisplaysDamageImpact"
  ],
  "CameraHeightOffset": 20,
  "CameraHorizontalOffset": 15,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 120
  },
  "FallSpeedMax": 1.524,
  "ImpactRadius": 5,
  "IsDisabled": false,
  "Key": "rutger_rocket",
  "LaunchMaxSpeed": 600,
  "LaunchMinSpeed": 525,
  "LaunchMinVerticalAmount": 0.2,
  "LaunchVerticalBias": 0.75,
  "Name": "Rocket Launcher",
  "SelfDamagePercent": 50,
  "SelfLaunchPercent": 175,
  "TechCleaveExpireTime": 0.2,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "Damage": 80
    },
    {
      "AbilityCooldown": -7.5
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_rutger",
      "hero_name": "Rutger",
      "lookup": "rocket launcher",
      "name": "Rocket Launcher",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_throw_sand" title="Sand Blast" -->

## Sand Blast

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_throw_sand`
- Snapshot ID: `39542`
- Source-Dokument: `7070`
- Kurzinfo: Sand Blast aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Sand Blast`
- Payload Hash: `80b87a84cba6007de86d735b324355a7a84edbb6585e2bf567da546dad7176cb`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.588589+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 25,
  "AbilityCooldown": 40.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 2.5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": 40,
  "DebuffModifier": {
    "Class": "CitadelThrowSandDebuff",
    "Subclass": "CitadelThrowSandDebuff"
  },
  "GrowthPerMeter": 0.5,
  "HeightOffGround": 20,
  "InitialWidth": 5,
  "IsDisabled": false,
  "Key": "ability_throw_sand",
  "Name": "Sand Blast",
  "Upgrades": [
    {
      "AbilityCastRange": 5,
      "Damage": 50
    },
    {
      "GroundDashReductionPercent": -30,
      "SlowPercent": 30
    },
    {
      "AbilityCooldown": -25.0,
      "AbilityDuration": 1.5
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
    "card_name": "Sand Blast",
    "hero_key": "hero_krill",
    "hero_name": "Mo & Krill",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_krill",
      "hero_name": "Mo & Krill",
      "lookup": "sand blast",
      "name": "Sand Blast",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_vampirebat_batblink" title="Sanguine Retreat" -->

## Sanguine Retreat

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_vampirebat_batblink`
- Snapshot ID: `39555`
- Source-Dokument: `7070`
- Kurzinfo: Sanguine Retreat aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Sanguine Retreat`
- Payload Hash: `85ff2bcbe229c0aec74a4406d3490cc606e2d5d178fbfbb7a47963ad480676fc`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.625485+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.02
    },
    "Value": 9
  },
  "AbilityCooldown": 32,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 0.65,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventBotUsage",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorAllowAltCast",
    "BehaviorMovement",
    "BehaviorCanSetQuickCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "Buff"
  },
  "ChannelMoveSpeed": -1,
  "EndJumpVelocity": 200,
  "ExitVelocity": 5,
  "IsDisabled": false,
  "Key": "ability_vampirebat_batblink",
  "MaxRecasts": 1,
  "Name": "Sanguine Retreat",
  "RecastWindow": 3.5,
  "SelfBuffModifier": {
    "Class": "Base",
    "StatusEffectPriority": 101,
    "Subclass": "Selfbuff"
  },
  "Upgrades": [
    {
      "BonusBullets": 8,
      "BonusFireRate": 25,
      "BuffDuration": 8
    },
    {
      "AbilityCooldown": -10
    },
    {
      "AbilityCastRange": 3,
      "MaxRecasts": 1
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
    "card_name": "Sanguine Retreat",
    "hero_key": "hero_vampirebat",
    "hero_name": "Mina",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_vampirebat",
      "hero_name": "Mina",
      "lookup": "sanguine retreat",
      "name": "Sanguine Retreat",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="fathom_scalding_spray" title="Scalding Spray" -->

## Scalding Spray

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `fathom_scalding_spray`
- Snapshot ID: `39679`
- Source-Dokument: `7070`
- Kurzinfo: Scalding Spray aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Scalding Spray`
- Payload Hash: `1a27c21b505bd14ab34eb918467487dcaf33919c9fac2e6c1247dc1e6866a743`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.933247+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCharges": 1,
  "AbilityCooldown": 40.0,
  "AbilityCooldownBetweenCharge": 8,
  "AbilityDuration": 3,
  "AbilityUnitTargetLimit": 1,
  "AuraModifier": {
    "AuraRadius": 0.0,
    "AuraTargetingConeAngle": 60.0,
    "AuraTargetingConeHalfWidth": 50.0,
    "BuffModifier": {
      "Class": "FathomScaldingSprayWeaponDamage",
      "Subclass": "FathomScaldingSprayWeaponDamage"
    },
    "Class": "FathomScaldingSprayAura",
    "ProvidedByAura": {
      "Class": "FathomScaldingSprayTarget",
      "Subclass": "FathomScaldingSprayTarget"
    },
    "Subclass": "FathomScaldingSprayAura"
  },
  "BehaviourBits": [
    "BehaviorCastableWhileBusy",
    "BehaviorNoTarget"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.372
    },
    "Value": 40
  },
  "IsDisabled": false,
  "Key": "fathom_scalding_spray",
  "Name": "Scalding Spray",
  "Radius": 12,
  "TickRate": 0.25,
  "Upgrades": [
    {
      "AbilityCooldown": -15.0
    },
    {
      "AbilityDuration": 2
    },
    {
      "DPS": 55
    }
  ],
  "WeaponDamageBonusDuration": 12,
  "WeaponDamageBonusPerSec": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0372
    },
    "Value": 5
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
    "card_name": "Scalding Spray",
    "hero_key": "hero_slork",
    "hero_name": "Fathom",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_slork",
      "hero_name": "Fathom",
      "lookup": "scalding spray",
      "name": "Scalding Spray",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_intimidate" title="Scorn" -->

## Scorn

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_intimidate`
- Snapshot ID: `39467`
- Source-Dokument: `7070`
- Kurzinfo: Scorn aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Scorn`
- Payload Hash: `05f682adeb03c013b277ce86416bb5041e5148e60b4ebda74319cfd0c23216b0`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.400759+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 13,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorCastableWhileBusy",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.75
    },
    "Value": 50
  },
  "DamageHealMult": 1.2,
  "DamageHealMultNonHero": 0.35,
  "DebuffModifier": {
    "Class": "IntimidateDebuff",
    "Subclass": "IntimidateDebuff"
  },
  "EnemyModifier": {
    "Class": "Intimidated",
    "Subclass": "Intimidated"
  },
  "IsDisabled": false,
  "Key": "ability_intimidate",
  "Name": "Scorn",
  "Radius": 9,
  "TickRate": 0.1,
  "Upgrades": [
    {
      "Damage": 35
    },
    {
      "AbilityCooldown": -5,
      "Radius": 1
    },
    {
      "DamageBonus": 15,
      "DebuffDuration": 16
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
    "card_name": "Scorn",
    "hero_key": "hero_krill",
    "hero_name": "Mo & Krill",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_krill",
      "hero_name": "Mo & Krill",
      "lookup": "scorn",
      "name": "Scorn",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_viper_debuffdagger" title="Screwjab Dagger" -->

## Screwjab Dagger

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_viper_debuffdagger`
- Snapshot ID: `39562`
- Source-Dokument: `7070`
- Kurzinfo: Screwjab Dagger aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Screwjab Dagger`
- Payload Hash: `6646223d61e4acdb10f49b70f87a7678eb1ed89cf2fdaa527394addc7c4131b9`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.641065+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCharges": 2,
  "AbilityCooldown": 10,
  "AbilityCooldownBetweenCharge": 4.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontSwitchAwayOnCast",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.8
    },
    "Value": 50
  },
  "DamagePerStack": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.4
    },
    "Value": 25
  },
  "DebuffModifier": {
    "Class": "CitadelViperStackingdebuff",
    "Subclass": "Debuff"
  },
  "IsDisabled": false,
  "Key": "ability_viper_debuffdagger",
  "MaxStacks": 3,
  "Name": "Screwjab Dagger",
  "SlowDuration": 2,
  "SlowModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "DebuffdaggerDebuff"
  },
  "SlowPercent": 35,
  "SlowPercentPerStack": 15,
  "StackDuration": 10,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "BulletResistReduction": -8,
      "BulletResistReductionPerStack": -6
    },
    {
      "AbilityCooldownBetweenCharge": -2,
      "CooldownRefundPercent": 55,
      "MaxStacks": 2
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
    "card_name": "Screwjab Dagger",
    "hero_key": "hero_viper",
    "hero_name": "Vyper",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_viper",
      "hero_name": "Vyper",
      "lookup": "screwjab dagger",
      "name": "Screwjab Dagger",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_bull_leap" title="Seismic Impact" -->

## Seismic Impact

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_bull_leap`
- Snapshot ID: `39599`
- Source-Dokument: `7070`
- Kurzinfo: Seismic Impact aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Seismic Impact`
- Payload Hash: `a1259f051a0d2a3e3a6189e9be3d66cfc339fd93a42acf9f2b65339d61aa9313`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.738669+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 215.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "ActiveModifier": {
    "Class": "Base",
    "Subclass": "LeapActive"
  },
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BoostModifier": {
    "Class": "CitadelBullLeapBoosting",
    "Subclass": "CitadelBullLeapBoosting"
  },
  "ChannelMoveSpeed": -1,
  "CrashModifier": {
    "Class": "CitadelBullLeapBoostingCrash",
    "Subclass": "CitadelBullLeapBoostingCrash"
  },
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.325
    },
    "Value": 100
  },
  "DragModifier": {
    "Class": "ChargeDragEnemy",
    "ForwardOffset": 200,
    "Subclass": "ChargeDragEnemy",
    "VerticalOffset": 0
  },
  "ImmunityModifier": {
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
  "ImpactHeight": 6,
  "ImpactRadius": 9,
  "IsDisabled": false,
  "Key": "citadel_ability_bull_leap",
  "LandingBonusesModifier": {
    "Class": "CitadelBullLeapLandingBonuses",
    "Subclass": "CitadelBullLeapLandingBonuses"
  },
  "Name": "Seismic Impact",
  "StunDuration": 1.6,
  "TossSpeed": 450,
  "Upgrades": [
    {
      "AbilityCooldown": -30.0
    },
    {
      "StunDuration": 0.8
    },
    {
      "ImmunityDuration": 6,
      "ImpactRadius": 6
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
    "card_name": "Seismic Impact",
    "hero_key": "hero_atlas",
    "hero_name": "Abrams",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_atlas",
      "hero_name": "Abrams",
      "lookup": "seismic impact",
      "name": "Seismic Impact",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_perched_predator" title="Sekhmet's Spirit" -->

## Sekhmet's Spirit

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_perched_predator`
- Snapshot ID: `39505`
- Source-Dokument: `7070`
- Kurzinfo: Sekhmet's Spirit aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Sekhmet's Spirit`
- Payload Hash: `97b1da7a8c99cfe2b1871297509d4c18fe9524694211474a277cf8c4189e94d9`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.498483+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorInhibitSoftCameraCollision"
  ],
  "CatAboveGround": 0.1,
  "CatAccel": 15,
  "CatClimbHeight": 3,
  "CatDropDownRate": 5,
  "CatLifetime": 2.5,
  "CatMaxSpeed": 25,
  "CatStartSpeed": 5,
  "ChannelMoveSpeed": -1,
  "ChargeDragVerticalOffset": 30,
  "ChargeRadius": 75,
  "ExplosionDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.209
    },
    "Value": 100
  },
  "ExplosionRadius": 8,
  "IsDisabled": false,
  "Key": "ability_perched_predator",
  "ModifierDragEnemy": {
    "Class": "PerchedPredatorDrag",
    "Subclass": "PerchedPredatorDrag"
  },
  "Name": "Sekhmet's Spirit",
  "TossSpeed": 400,
  "Upgrades": [
    {
      "ExplosionRadius": 4
    },
    {
      "AbilityCooldown": -11.5
    },
    {
      "ExplosionDamage": 120
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_shiv_dagger" title="Serrated Knives" -->

## Serrated Knives

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shiv_dagger`
- Snapshot ID: `39639`
- Source-Dokument: `7070`
- Kurzinfo: Serrated Knives aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Serrated Knives`
- Payload Hash: `dc6c0d87c1212b9f1ee275c49e3797fa4adb35e422cb4e77bd339ecf79567dab`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.836910+00:00`

### Vollstaendige Payload

````json
{
  "AOERadius": 10,
  "AbilityChannelTime": 0.2,
  "AbilityCharges": 2,
  "AbilityCooldown": 16,
  "AbilityCooldownBetweenCharge": 2,
  "AbilityPostCastDuration": 0.3,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCleaveDisabled",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BleedDPSPerStack": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.13
    },
    "Value": 10.0
  },
  "BleedDuration": 5,
  "BleedTickRate": 1,
  "ChannelMoveSpeed": -1,
  "DamageDebuffModifier": {
    "Class": "ShivThrownShivDamageDebuff",
    "Subclass": "ShivThrownShivDamageDebuff"
  },
  "ImpactDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "IsDisabled": false,
  "Key": "citadel_ability_shiv_dagger",
  "MovementSlow": 35,
  "Name": "Serrated Knives",
  "RicochetCount": 1,
  "SlowDebuffModifier": {
    "Class": "ShivThrownShivSlowDebuff",
    "Subclass": "ShivThrownShivSlowDebuff"
  },
  "Upgrades": [
    {
      "BleedDuration": 2
    },
    {
      "AbilityCharges": 2
    },
    {
      "BleedDPSPerStack": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.09
        },
        "Value": 12
      }
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
    "card_name": "Serrated Knives",
    "hero_key": "hero_shiv",
    "hero_name": "Shiv",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_shiv",
      "hero_name": "Shiv",
      "lookup": "serrated knives",
      "name": "Serrated Knives",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="yakuza_setting_sun" title="Setting Sun" -->

## Setting Sun

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `yakuza_setting_sun`
- Snapshot ID: `39743`
- Source-Dokument: `7070`
- Kurzinfo: Setting Sun aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Setting Sun`
- Payload Hash: `82e73a87e13ccae7681fb763c1de0c353e013b7d96cae05afd3388ecb72752b0`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.102213+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 74.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "CenterDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 250
  },
  "CenterRadius": 5,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "yakuza_setting_sun",
  "Name": "Setting Sun",
  "OuterDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 100
  },
  "Radius": 10,
  "Range": 25,
  "SettingSunThinkerModifier": {
    "Class": "SettingSunThinker",
    "Subclass": "SettingSunThinker"
  },
  "ShootDuration": 1.5,
  "TargetingDuration": 1.0,
  "Upgrades": [
    {
      "AbilityCooldown": -19.0
    },
    {
      "Range": 175
    },
    {
      "CenterDamage": 200
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_yakuza",
      "hero_name": "The Boss",
      "lookup": "setting sun",
      "name": "Setting Sun",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_infinity_slash" title="Shadow Transformation" -->

## Shadow Transformation

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_infinity_slash`
- Snapshot ID: `39619`
- Source-Dokument: `7070`
- Kurzinfo: Shadow Transformation aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shadow Transformation`
- Payload Hash: `0995c11e3658eaf887f1fde51f1dfba78765ac37eaa272daee6c9c2019ef12e2`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.789780+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": 1.5,
  "AbilityCooldown": 150.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilitySpeedPct": 60,
  "AbilityUnitTargetLimit": 1,
  "AutoChannelModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "Invulnerable",
      "TechUntargetable",
      "UnitStatusHealthHidden",
      "IgnoreBullets",
      "IgnoreMelee",
      "StatusImmune",
      "SlowImmune",
      "KnockdownImmune",
      "Unstoppable"
    ],
    "Subclass": "Channel"
  },
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCleaveDisabled",
    "BehaviorCanCancelDuringCastDelay",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BuffModifier": {
    "Class": "Base",
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
      "YamatoShadowForm",
      "StatusImmune",
      "SlowImmune",
      "KnockdownImmune",
      "Unstoppable"
    ],
    "StatusEffectPriority": 0,
    "Subclass": "Buff"
  },
  "BuffTimerModifier": {
    "Class": "YamatoInfinitySlashBuffTimer",
    "Subclass": "Timer"
  },
  "BulletResist": 30,
  "IsDisabled": false,
  "Key": "citadel_ability_infinity_slash",
  "MaxHealthRegen": 15,
  "Name": "Shadow Transformation",
  "ShadowFormDurationOnKill": 2.0,
  "TechResist": 30,
  "Upgrades": [
    {
      "WeaponDamageBonus": 7
    },
    {
      "AbilityCooldown": -20,
      "BonusMoveSpeed": 4
    },
    {
      "AbilityDuration": 3.0,
      "BulletResist": 30,
      "TechResist": 30
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
    "card_name": "Shadow Transformation",
    "hero_key": "hero_yamato",
    "hero_name": "Yamato",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_yamato",
      "hero_name": "Yamato",
      "lookup": "shadow transformation",
      "name": "Shadow Transformation",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="yakuza_shakedown" title="Shakedown" -->

## Shakedown

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `yakuza_shakedown`
- Snapshot ID: `39744`
- Source-Dokument: `7070`
- Kurzinfo: Shakedown aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shakedown`
- Payload Hash: `8990a79df765df84bb73a65da6a78d6eb2c7880b6ef5079ec21afe65f6fc0922`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.106047+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": 5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": 1.3,
  "IsDisabled": false,
  "Key": "yakuza_shakedown",
  "Name": "Shakedown",
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="yakuza_shakedown_target" title="Shakedown" -->

## Shakedown

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `yakuza_shakedown_target`
- Snapshot ID: `39745`
- Source-Dokument: `7070`
- Kurzinfo: Shakedown aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shakedown`
- Payload Hash: `06ae0e838e9c9e4fa8c9ecac9d1f72fde53f40057b7eb7ea0d61a9ca493e3f28`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.109260+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTimeDisplay": 5,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 45
  },
  "IsDisabled": false,
  "Key": "yakuza_shakedown_target",
  "Name": "Shakedown",
  "PulseModifier": {
    "Class": "CitadelShakedownPulse",
    "Subclass": "CitadelShakedownPulse"
  },
  "Radius": 6,
  "RootModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "Immobilized"
    ],
    "Subclass": "CitadelShakedownTarget"
  },
  "ShareDamagePercent": 33,
  "ShareDamageThreshold": 20,
  "TickTime": 0.5,
  "Upgrades": [
    {
      "WeaponPowerDebuff": -30
    },
    {
      "IgnoreChannelSlow": 1
    },
    {
      "ShareDamagePercent": 67
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_yakuza",
      "hero_name": "The Boss",
      "lookup": "shakedown",
      "name": "Shakedown",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="thumper_ability_1" title="Shatter Cannon" -->

## Shatter Cannon

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `thumper_ability_1`
- Snapshot ID: `39718`
- Source-Dokument: `7070`
- Kurzinfo: Shatter Cannon aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shatter Cannon`
- Payload Hash: `b0ca016d9720bc8c5328ab89f1f95a022a7109afc8231cbd11ecaee07f352e0d`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.031890+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 40,
  "AbilityCharges": 1,
  "AbilityCooldown": 17.0,
  "AbilityCooldownBetweenCharge": 4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDontTriggerSpellBlock",
    "BehaviorDisplaysDamageImpact",
    "BehaviorAlwaysPreviewRadius"
  ],
  "BounceRadians": 0.5,
  "BounceRange": 20,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 120
  },
  "IsDisabled": false,
  "Key": "thumper_ability_1",
  "MaxPlaneDistance": 1,
  "Name": "Shatter Cannon",
  "PlaneSpread": 30,
  "PushSpeedMax": 1000,
  "PushSpeedMid": 600,
  "PushSpeedMin": 100,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "Damage": 60
    },
    {
      "AbilityCooldown": -3.75,
      "AbilityCooldownBetweenCharge": -3
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_thumper",
      "hero_name": "Thumper",
      "lookup": "shatter cannon",
      "name": "Shatter Cannon",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_unicorn_dazzlingorb" title="Shining Wonder" -->

## Shining Wonder

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_unicorn_dazzlingorb`
- Snapshot ID: `39551`
- Source-Dokument: `7070`
- Kurzinfo: Shining Wonder aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shining Wonder`
- Payload Hash: `3999c759caa4211ec2542d9867541df86dbefb64b60b92c0a7741e6d1cd4e16b`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.614727+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.75,
  "AbilityChannelTime": 9999,
  "AbilityCooldown": 160,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorCooldownOnChannelEnd"
  ],
  "BounceGrace": 3,
  "BounceRadius": 16.5,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.8
    },
    "Value": 150
  },
  "GroundDashReductionPercent": -25,
  "IsDisabled": false,
  "Key": "ability_unicorn_dazzlingorb",
  "MaxBounces": 8,
  "Name": "Shining Wonder",
  "NextTargetDuration": 4,
  "OrbWatcherModifier": {
    "Class": "DazzlingOrbWatcher",
    "MinProjectileTravelTime": 0.2,
    "NextTargetModifier": {
      "Class": "UnicornDazzlingOrbNextTarget",
      "Subclass": "DazzlingOrbNextTargetModifier"
    },
    "OrbFriendlyBounceWatcherModifier": {
      "Class": "Base",
      "EnabledStateMask": [
        "PrismaticGuarded"
      ],
      "Subclass": "DazzlingOrbFriendlyWatcher"
    },
    "SlowModifier": {
      "Class": "SlowBase",
      "EnabledStateMask": [
        "Slowed"
      ],
      "StatusEffectPriority": 150,
      "Subclass": "DazzlingOrbSlowModifier"
    },
    "Subclass": "DazOrb"
  },
  "PriorityBounceRadius": 12.5,
  "SlowDuration": 1.5,
  "SlowPercent": 40,
  "Upgrades": [
    {
      "GroundDashReductionPercent": -15,
      "SlowPercent": 20
    },
    {
      "Damage": 80
    },
    {
      "AbilityCooldown": -30,
      "MaxBounces": 8
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
    "card_name": "Shining Wonder",
    "hero_key": "hero_unicorn",
    "hero_name": "Celeste",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_unicorn",
      "hero_name": "Celeste",
      "lookup": "shining wonder",
      "name": "Shining Wonder",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_frank_revive" title="Shocking Reanimation" -->

## Shocking Reanimation

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_frank_revive`
- Snapshot ID: `39444`
- Source-Dokument: `7070`
- Kurzinfo: Shocking Reanimation aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shocking Reanimation`
- Payload Hash: `aefdcb545a5b7a34677978cf278d9dece9a53250fdf917783efc953a2312003e`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.339535+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": 3,
  "AbilityCooldown": 275,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.66,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorCastEvenIfBusyAndExclusive",
    "BehaviorChannelled",
    "BehaviorCastableWhileBusy",
    "BehaviorNotSilencable",
    "BehaviorNoTarget",
    "BehaviorCastableWhileCmdRestricted",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanCastWhileDead"
  ],
  "BonusDamagePerBullet": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "Selfbuff"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.0
    },
    "Value": 200
  },
  "DashSlowModifier": {
    "Class": "Base",
    "Subclass": "Dashslow"
  },
  "EnemyDashSlowPercent": -30,
  "HalfHeight": 15,
  "InitialDelay": 0.5,
  "IsDisabled": false,
  "Key": "ability_frank_revive",
  "Name": "Shocking Reanimation",
  "Radius": 18,
  "RespawnDelay": 3,
  "RespawnHealthPercent": 50,
  "RevivingModifier": {
    "Class": "FrankReviving",
    "EnabledStateMask": [
      "Invulnerable",
      "HideCrosshair",
      "HideStamina",
      "HideAmmo"
    ],
    "Subclass": "Reviving"
  },
  "SlowDuration": 3,
  "SlowModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "Slow"
  },
  "SlowPercent": 120,
  "StunDuration": 1.5,
  "Upgrades": [
    {
      "BonusDamagePerBullet": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.06
        },
        "Value": 6.0
      },
      "BonusFireRate": 15
    },
    {
      "RespawnHealthPercent": 50
    },
    {
      "AbilityCooldown": -95,
      "Damage": 175,
      "StunDuration": 1.5
    }
  ],
  "ZombieModifier": {
    "Class": "FrankZombie",
    "Subclass": "Zombie"
  },
  "ZombieTickRate": 0.02,
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
    "card_name": "Shocking Reanimation",
    "hero_key": "hero_frank",
    "hero_name": "Victor",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_frank",
      "hero_name": "Victor",
      "lookup": "shocking reanimation",
      "name": "Shocking Reanimation",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_bull_charge" title="Shoulder Charge" -->

## Shoulder Charge

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_bull_charge`
- Snapshot ID: `39597`
- Source-Dokument: `7070`
- Kurzinfo: Shoulder Charge aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shoulder Charge`
- Payload Hash: `a1d2fcd8244e8ff1788b9ac1d488aba658362ea73312e6c1e043d34add1549b5`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.733395+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 33.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 1.4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "CameraTurnRateMax": 200,
  "ChannelMoveSpeed": -1,
  "ChargeDragVerticalOffset": 30,
  "ChargeRadius": 2.2,
  "ChargeSpeedMax": 30,
  "CollidePlayersStopTime": 0.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.4
    },
    "Value": 30
  },
  "IsDisabled": false,
  "Key": "citadel_ability_bull_charge",
  "ModifierBullCharging": {
    "Class": "CitadelBullCharging",
    "EnabledStateMask": [
      "AbilityMovement",
      "MantleDisabled",
      "MeleeDisabled",
      "SlidingDisabled",
      "DuckingDisabled",
      "ForceCanParry"
    ],
    "Subclass": "CitadelBullCharging"
  },
  "ModifierChargeDragEnemy": {
    "Class": "ChargeDragEnemy",
    "EnabledStateMask": [
      "AbilityMovementDebuff",
      "IgnorePortals"
    ],
    "ForwardOffset": 120,
    "Subclass": "ChargeDragEnemy",
    "VerticalOffset": 30
  },
  "ModifierTossAirControlLockout": {
    "Class": "Base",
    "Duration": 1.0,
    "EnabledStateMask": [
      "MovementAbilityRestricted"
    ],
    "Subclass": "AirControlLockout"
  },
  "ModifierWeaponPowerIncrease": {
    "Class": "Base",
    "Subclass": "ShoulderChargeBuff"
  },
  "Name": "Shoulder Charge",
  "SideMoveSpeedReduction": -65,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "WallSlamSlow"
  },
  "SpeedInitial": 18.75,
  "StunDuration": 0.3,
  "TossUpMagnitude": 0.5,
  "TurnRateMax": 140,
  "Upgrades": [
    {
      "SlowDuration": 3,
      "SlowPercent": 40
    },
    {
      "StunDuration": 0.8
    },
    {
      "AbilityCooldown": -18,
      "WeaponDamageBonus": 1.5,
      "WeaponPowerIncreaseDuration": 6
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
    "card_name": "Shoulder Charge",
    "hero_key": "hero_atlas",
    "hero_name": "Abrams",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_atlas",
      "hero_name": "Abrams",
      "lookup": "shoulder charge",
      "name": "Shoulder Charge",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="cadence_ability_silencecontraptions" title="Silence Contraptions" -->

## Silence Contraptions

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `cadence_ability_silencecontraptions`
- Snapshot ID: `39595`
- Source-Dokument: `7070`
- Kurzinfo: Silence Contraptions aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Silence Contraptions`
- Payload Hash: `a6618cf2427c9b02ff2153b4f50d159f242540016991b753b966e754caf84834`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.727626+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "DashDistance": 8,
  "DebuffDuration": 2,
  "IsDisabled": false,
  "Key": "cadence_ability_silencecontraptions",
  "MeleeEMP": 1,
  "Name": "Silence Contraptions",
  "SilenceContraptionsModifier": {
    "Class": "CadenceSilenceContraptions",
    "DebuffModifier": {
      "Class": "CadenceSilenceContraptionsDebuff",
      "Subclass": "CadenceSilenceContraptionsDebuff"
    },
    "Subclass": "CadenceSilenceContraptions"
  },
  "Upgrades": [
    {
      "SlowPercent": 40
    },
    {
      "DashDistance": 4
    },
    {
      "DebuffDuration": 1.5
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_cadence",
      "hero_name": "Cadence",
      "lookup": "silence contraptions",
      "name": "Silence Contraptions",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_trapper_webwall" title="Silktrap" -->

## Silktrap

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_webwall`
- Snapshot ID: `39548`
- Source-Dokument: `7070`
- Kurzinfo: Silktrap aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Silktrap`
- Payload Hash: `78ee7e42a32e0ea24f503d85b5649bcbdcd122c230cdc0e068d8df9aaa4a204c`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.606632+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.22,
  "AbilityCastRange": 40,
  "AbilityCharges": 1,
  "AbilityCooldown": 40,
  "AbilityCooldownBetweenCharge": 1,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.279
    },
    "Value": 40
  },
  "DebuffDuration": 3,
  "DebuffModifier": {
    "Class": "WebwallDebuff",
    "Subclass": "WebwallDebuff"
  },
  "IsDisabled": false,
  "Key": "ability_trapper_webwall",
  "MaxWallToWallDistance": 100,
  "MinWallToWallDistance": 3,
  "Name": "Silktrap",
  "Radius": 0.6,
  "SilenceModifier": {
    "Class": "CitadelSilenced",
    "Subclass": "WebwallSilence"
  },
  "SlowPercent": 99,
  "TickRate": 0.5,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "WebDuration": 120
    },
    {
      "DebuffDuration": 2,
      "SilenceDuration": 2.5
    }
  ],
  "WebArmTime": 0.5,
  "WebDuration": 120,
  "WebWallTickRate": 0.15,
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
    "card_name": "Silktrap",
    "hero_key": "hero_trapper",
    "hero_name": "Trapper",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_trapper",
      "hero_name": "Trapper",
      "lookup": "silktrap",
      "name": "Silktrap",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_self_vacuum" title="Singularity" -->

## Singularity

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_self_vacuum`
- Snapshot ID: `39637`
- Source-Dokument: `7070`
- Kurzinfo: Singularity aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Singularity`
- Payload Hash: `3257b06055d46663a585fc09f77cc7f7841be457896d5626096f1e96a9868e12`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.832710+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityChannelTime": 2.75,
  "AbilityCooldown": 265.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorExclusiveUse",
    "BehaviorCastableWhileBusy",
    "BehaviorInterruptMeleeOnCast",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "CameraDistance": 400,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.28
    },
    "Value": 75
  },
  "IsDisabled": false,
  "Key": "citadel_ability_self_vacuum",
  "Name": "Singularity",
  "Speed": 5.08,
  "TickRate": 0.25,
  "TossAngle": 45,
  "TossSpeed": 8.89,
  "Upgrades": [
    {
      "VacuumRadius": 2
    },
    {
      "AbilityChannelTime": 0.75
    },
    {
      "DPSPercentHealth": 6
    }
  ],
  "VacuumAuraModifier": {
    "Class": "Vacuumaura",
    "ProvidedByAura": {
      "Class": "VacuumauraTarget",
      "EnabledStateMask": [
        "ModNoCleanse"
      ],
      "OuterSpeedScale": 10.0,
      "StatusEffectPriority": 60,
      "Subclass": "VacuumauraTarget"
    },
    "StatusEffectPriority": 0,
    "Subclass": "Vacuumaura"
  },
  "VacuumRadius": 7,
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
    "card_name": "Singularity",
    "hero_key": "hero_dynamo",
    "hero_name": "Dynamo",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_dynamo",
      "hero_name": "Dynamo",
      "lookup": "singularity",
      "name": "Singularity",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_bull_heal" title="Siphon Life" -->

## Siphon Life

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_bull_heal`
- Snapshot ID: `39598`
- Source-Dokument: `7070`
- Kurzinfo: Siphon Life aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Siphon Life`
- Payload Hash: `9fe7e3594f724ed4670891165897462273a6fedaf2adf898e3236ddd06a3221c`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.736+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4,
  "AbilityUnitTargetLimit": 1,
  "AuraModifier": {
    "AuraRadius": 0.0,
    "Class": "BullHealAura",
    "ProvidedByAura": {
      "Class": "BullHealTarget",
      "Subclass": "BullHealTarget"
    },
    "Subclass": "BullHealAura"
  },
  "BehaviourBits": [
    "BehaviorCastableWhileBusy",
    "BehaviorNoTarget"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 22
  },
  "HealingFactor": 70,
  "IsDisabled": false,
  "Key": "citadel_ability_bull_heal",
  "Name": "Siphon Life",
  "NonHeroHealingFactor": 35,
  "Radius": 8,
  "TickRate": 0.25,
  "Upgrades": [
    {
      "AbilityCooldown": -20.0
    },
    {
      "AbilityDuration": 2
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.12
        },
        "Value": 18
      },
      "Radius": 2
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
    "card_name": "Siphon Life",
    "hero_key": "hero_atlas",
    "hero_name": "Abrams",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_atlas",
      "hero_name": "Abrams",
      "lookup": "siphon life",
      "name": "Siphon Life",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_boho_bouncyprojectile" title="Skipshot" -->

## Skipshot

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_boho_bouncyprojectile`
- Snapshot ID: `39400`
- Source-Dokument: `7070`
- Kurzinfo: Skipshot aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Skipshot`
- Payload Hash: `7fff562432e093d771b9bd51b9d6b37888dca6bf8524a9b51888580f58ef1263`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.230095+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 14,
  "AbilityCharges": 1,
  "AbilityCooldown": 15,
  "AbilityCooldownBetweenCharge": 7,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorProjectileFiredAsBullet"
  ],
  "BounceCount": 3,
  "BounceRadius": 18,
  "ChannelMoveSpeed": -1,
  "CooldownReductionPercentagePerHit": 15,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.4
    },
    "Value": 60
  },
  "IsDisabled": false,
  "Key": "ability_boho_bouncyprojectile",
  "Name": "Skipshot",
  "Upgrades": [
    {
      "AbilityCooldown": -2
    },
    {
      "Damage": 18.0
    },
    {
      "BounceCount": 2
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
    "card_name": "Skipshot",
    "hero_key": "hero_boho",
    "hero_name": "Boho",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_boho",
      "hero_name": "Boho",
      "lookup": "skipshot",
      "name": "Skipshot",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_werewolf_unloadgun" title="Slam Fire" -->

## Slam Fire

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_unloadgun`
- Snapshot ID: `39585`
- Source-Dokument: `7070`
- Kurzinfo: Slam Fire aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Slam Fire`
- Payload Hash: `44dd678259df4e87e5609899f3b6ebf6d3a1f29bafb5553c866d3cb70a61f7a3`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.698474+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.45,
  "AbilityCastRange": {
    "Scale": {
      "Type": "range",
      "Value": 0.0
    },
    "Value": 0.0254
  },
  "AbilityCooldown": 25,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "AccuracyPercentage": -30,
  "AutoChannelModifier": {
    "Class": "Base",
    "Subclass": "Channeling"
  },
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusFireRate": 300,
  "BuffModifier": {
    "Class": "WerewolfUnloadgun2",
    "MaxBulletsToProcInShot": 1.0,
    "StackingModifier": {
      "Class": "Base",
      "Subclass": "Stacking"
    },
    "Subclass": "Buff"
  },
  "BulletEffectiveness": 0.1,
  "BulletRadiusOverride": 7,
  "BulletSpread": -1,
  "ChannelMoveSpeed": 5.08,
  "CurrentHealthDamagePercentage": 2.5,
  "Damage": 40,
  "DebuffDuration": 3,
  "IsDisabled": false,
  "Key": "ability_werewolf_unloadgun",
  "LingerDuration": 0.1,
  "MaxShots": 3,
  "Name": "Slam Fire",
  "ProcChance": 100,
  "RecoilDelayFactor": 0.05,
  "RecoilRecoverySpeed": 0.1,
  "RecoilSpeed": 12,
  "RecoilStrength": 12,
  "SpreadPenaltyPerShot": 0.5,
  "Upgrades": [
    {
      "BaseAttackDamagePercent": 15
    },
    {
      "AbilityCooldown": -10
    },
    {
      "BonusCurrentHealthDamagePercentage": 7,
      "MaxStacks": 3,
      "StackDuration": 3
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
    "card_name": "Slam Fire",
    "hero_key": "hero_werewolf",
    "hero_name": "Silver",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_werewolf",
      "hero_name": "Silver",
      "lookup": "slam fire",
      "name": "Slam Fire",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="gunslinger_sleep_bomb" title="Sleep Bomb" -->

## Sleep Bomb

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `gunslinger_sleep_bomb`
- Snapshot ID: `39688`
- Source-Dokument: `7070`
- Kurzinfo: Sleep Bomb aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Sleep Bomb`
- Payload Hash: `9f059a566cb0d04e58e11eed5045cf9854999eb9dd2ebec2ed80e0bc50ea5df2`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.954810+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 95.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": "",
  "AbilityUnitTargetLimit": 1,
  "AuraModifier": {
    "Class": "SleepBombAura",
    "ProvidedByAura": {
      "Class": "SleepBombAsleep",
      "Subclass": "SleepBombAsleep"
    },
    "Subclass": "SleepBombAura"
  },
  "BehaviourBits": null,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 50
  },
  "EndRadius": 30,
  "IsDisabled": false,
  "Key": "gunslinger_sleep_bomb",
  "Name": "Sleep Bomb",
  "SleepDuration": 6,
  "SpreadDuration": 2,
  "StartRadius": 5,
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_sleep_dagger" title="Sleep Dagger" -->

## Sleep Dagger

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_sleep_dagger`
- Snapshot ID: `39534`
- Source-Dokument: `7070`
- Kurzinfo: Sleep Dagger aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Sleep Dagger`
- Payload Hash: `06e672b151f12658066328c1267e962f6d71863a3bf2786b0e5da391c3065876`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.565266+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 30.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDamageDoesntWakeFromSleep",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.8
    },
    "Value": 65
  },
  "DoesNotBreakInvis": 1,
  "IsDisabled": false,
  "Key": "ability_sleep_dagger",
  "MinimumSleepTime": 0.2,
  "Name": "Sleep Dagger",
  "SleepDuration": 2.75,
  "SleepModifier": {
    "Class": "CitadelSleepDaggerAsleep",
    "PostSleepBulletShredModifier": {
      "Class": "Base",
      "Subclass": "BulletShredPostsleep"
    },
    "PostSleepModifier": {
      "Class": "SlowBase",
      "Subclass": "Postsleep"
    },
    "PostSleepStaminaModifier": {
      "Class": "Base",
      "Subclass": "NoStamRecovery"
    },
    "Subclass": "CitadelSleepDaggerAsleep"
  },
  "SleepMoveSpeed": 1.5,
  "SleepWakeUpDelay": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.003
    },
    "Value": 0.1
  },
  "Upgrades": [
    {
      "BulletResistReduction": -10.0,
      "BulletResistReductionDuration": 6
    },
    {
      "AbilityCooldown": -18.0
    },
    {
      "DebuffDuration": 3,
      "GroundDashReductionPercent": -50,
      "SleepDuration": 1,
      "SlowPercent": 50
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
    "card_name": "Sleep Dagger",
    "hero_key": "hero_haze",
    "hero_name": "Haze",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_haze",
      "hero_name": "Haze",
      "lookup": "sleep dagger",
      "name": "Sleep Dagger",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_empowerbullet" title="Sleight of Hand" -->

## Sleight of Hand

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_empowerbullet`
- Snapshot ID: `39425`
- Source-Dokument: `7070`
- Kurzinfo: Sleight of Hand aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Sleight of Hand`
- Payload Hash: `58450805497d25e8aba270ed3c0debbecff73125b7e8185594cbc1cde87e9474`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.287186+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCooldown": 6,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusClip": 1,
  "BuffDuration": 3,
  "Damage": {
    "Scale": {
      "Type": "weapon_damage",
      "Value": 0.93
    },
    "Value": 100
  },
  "DebuffModifier": {
    "Class": "DiminishingSlow",
    "EnabledStateMask": [
      "GlowThroughWallsToProvider",
      "SilenceMovementAbilites"
    ],
    "Subclass": "Slow"
  },
  "EmpowerBulletModifier": {
    "Class": "EmpowerBullet",
    "DebuffModifier": {
      "Class": "StompDebuff",
      "Subclass": "EmpowerbulletDebuff"
    },
    "MaxBulletsToProcInShot": 1.0,
    "StatusEffectPriority": 0,
    "Subclass": "EmpowerbulletWatcher"
  },
  "IsDisabled": false,
  "Key": "ability_empowerbullet",
  "Name": "Sleight of Hand",
  "ProcChance": 100,
  "ProcDamagePercentage": 100,
  "Upgrades": [
    {
      "BonusMoveSpeed": 2
    },
    {
      "BulletArmorReduction": 25,
      "DebuffDuration": 5
    },
    {
      "ProcDamagePercentage": 70
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_shiv_dash" title="Slice and Dice" -->

## Slice and Dice

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shiv_dash`
- Snapshot ID: `39640`
- Source-Dokument: `7070`
- Kurzinfo: Slice and Dice aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Slice and Dice`
- Payload Hash: `eee179a5662c0eae0eee710faeaf18fca5bc2ff94e10d4c4104907feaadd8a54`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.839367+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCooldown": 16.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "CameraDistance": 250,
  "ChannelMoveSpeed": -1,
  "DashAngleThreshold": 89,
  "DashModifier": {
    "Class": "CitadelShivDash",
    "Subclass": "CitadelShivDash"
  },
  "DashRadius": 2.5,
  "DashRange": 12,
  "DashSpeed": 60.96,
  "DebuffDuration": 14,
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "ShivDashDebuff"
  },
  "ImpactDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.4415
    },
    "Value": 75
  },
  "IsDisabled": false,
  "Key": "citadel_ability_shiv_dash",
  "MoveSpeedPenaltyMaxSpeed": 200,
  "Name": "Slice and Dice",
  "SideMoveSpeedReduction": -100,
  "TechArmorDamageReduction": -6,
  "TechCleaveExpireTime": 0.35,
  "Upgrades": [
    {
      "AbilityCooldown": -6
    },
    {
      "DashRange": 2,
      "TechArmorDamageReduction": -6
    },
    {
      "CooldownReductionOnHit": 2,
      "CooldownReductionOnHitNonHero": 1,
      "ImpactDamage": 50,
      "MaxCooldownReductionsFromHits": 8
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
    "card_name": "Slice and Dice",
    "hero_key": "hero_shiv",
    "hero_name": "Shiv",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_shiv",
      "hero_name": "Shiv",
      "lookup": "slice and dice",
      "name": "Slice and Dice",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_viper_snakedash" title="Slither" -->

## Slither

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_viper_snakedash`
- Snapshot ID: `39566`
- Source-Dokument: `7070`
- Kurzinfo: Slither aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Slither`
- Payload Hash: `1946a645b39e26972ea3ba6290b6e788417bf7a5c3c5c4c6041cca9c80c941a5`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.650886+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "Class": "Base",
      "Subclass": "ViperslideIntrinsic"
    },
    {
      "BuffModifier": {
        "Class": "Base",
        "Subclass": "Slidebarrier"
      },
      "Class": "ViperSlidebuff",
      "Subclass": "ViperSlidebuffModifier"
    }
  ],
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventBotUsage",
    "BehaviorMovement"
  ],
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "IsDisabled": false,
  "Key": "ability_viper_snakedash",
  "Name": "Slither",
  "SlideScale": 15,
  "Upgrades": [
    {
      "SlideScale": 20
    },
    {
      "Stamina": 2
    },
    {
      "AbilityCooldown": 8,
      "BuffDuration": 5,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.8
        },
        "Value": 180
      }
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
    "card_name": "Slither",
    "hero_key": "hero_viper",
    "hero_name": "Vyper",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_viper",
      "hero_name": "Vyper",
      "lookup": "slither",
      "name": "Slither",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_smoke_bomb" title="Smoke Bomb" -->

## Smoke Bomb

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_smoke_bomb`
- Snapshot ID: `39535`
- Source-Dokument: `7070`
- Kurzinfo: Smoke Bomb aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Smoke Bomb`
- Payload Hash: `62617c5dbd53ab41dc10a3018dd97d1f5e5db3152a120fa440b046f962ab2f00`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.568229+00:00`

### Vollstaendige Payload

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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_health_swap" title="Soul Exchange" -->

## Soul Exchange

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_health_swap`
- Snapshot ID: `39459`
- Source-Dokument: `7070`
- Kurzinfo: Soul Exchange aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Soul Exchange`
- Payload Hash: `60f3feb0c9e0cb5c591e8013d5ac7822fbceb8e3a3a665eab311d3c7c5e22653`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.379297+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 5.5,
  "AbilityCooldown": 220.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 0.25,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorCleaveDisabled",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "Base"
  },
  "ChannelMoveSpeed": 2,
  "EnemyMinHealthPct": 30,
  "EnemySlowPct": 70,
  "InitialUpSpeed": 150,
  "IsDisabled": false,
  "Key": "ability_health_swap",
  "MinDiffToCast": 0.1,
  "MinHealthTakenPct": 30,
  "Name": "Soul Exchange",
  "PostCastHoldTime": 0.2,
  "PreCastModifier": {
    "Class": "HealthswapPrecast",
    "Subclass": "HealthswapPrecast"
  },
  "SilenceModifier": {
    "Class": "CitadelSilenced",
    "Subclass": "CitadelSilenced"
  },
  "SwapModifier": {
    "Class": "HealthSwapDebuff",
    "Subclass": "HealthSwapDebuff"
  },
  "Upgrades": [
    {
      "AbilityCooldown": -50.0
    },
    {
      "SilenceDuration": 3,
      "SilenceRadius": 25
    },
    {
      "BonusFireRate": 40,
      "BonusSpirit": 60,
      "SelfBuffDuration": 8,
      "TechResist": 50
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
    "card_name": "Soul Exchange",
    "hero_key": "hero_ghost",
    "hero_name": "Lady Geist",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_ghost",
      "hero_name": "Lady Geist",
      "lookup": "soul exchange",
      "name": "Soul Exchange",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_golden_idol" title="Soul Urn" -->

## Soul Urn

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_golden_idol`
- Snapshot ID: `39447`
- Source-Dokument: `7070`
- Kurzinfo: Soul Urn aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Soul Urn`
- Payload Hash: `884ebb8e45b1461f93d9060d1cc164a5a44fd5bcbfe1c2c2ba6387db40890d32`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.347798+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AirMoveIncreasePercent": 10,
  "AutoIntrinsicModifiers": [
    {
      "Class": "IntrinsicBase",
      "Subclass": "IntrinsicBase"
    }
  ],
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNotSilencable",
    "BehaviorNoTarget"
  ],
  "BonusMoveSpeed": 3.5,
  "BonusSprintAcceleration": 14,
  "BonusSprintSpeed": 2,
  "ChannelMoveSpeed": -1,
  "DropOffTimer": 0.1,
  "DropoffTimerModifier": {
    "Class": "IdolReturnTimer",
    "EnabledStateMask": [
      "ReturningIdol"
    ],
    "Subclass": "Timer"
  },
  "FixedMoveSpeed": 15,
  "GroundMoveIncreasePercent": 10,
  "HoldingIdolModifier": {
    "Class": "CitadelHoldingGoldenIdol",
    "EnabledStateMask": [
      "HoldingIdol",
      "TeleporterDisabled",
      "ZiplineDisabled",
      "AttributeCannotBePurged",
      "ParryDisabled"
    ],
    "Subclass": "CitadelHoldingGoldenIdol"
  },
  "IsDisabled": false,
  "Key": "ability_golden_idol",
  "Lifetime": 3,
  "Name": "Soul Urn",
  "Radius": 20,
  "SlowResistancePercent": 100,
  "Stamina": 1,
  "StaminaCooldownReduction": 15,
  "TrailingTeamBonusArmorDamageResist": 35,
  "TrailingTeamBonusDebuffResist": 35,
  "TrailingTeamBonusSprintSpeed": 7,
  "TrailingTeamBonusTechResist": 35,
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_magician_cloneturret" title="Spectral Assistant" -->

## Spectral Assistant

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_cloneturret`
- Snapshot ID: `39475`
- Source-Dokument: `7070`
- Kurzinfo: Spectral Assistant aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spectral Assistant`
- Payload Hash: `768383d3442ebb4aeb772c03ec7dc0c7255743383106ce7a8f4820afbe9a306d`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.429102+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 15,
  "AbilityCooldown": 40,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorPreventBotUsage",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BuffModifier": {
    "Class": "Base",
    "Duration": -1.0,
    "Subclass": "CloneturretBuffModifier"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.36
    },
    "Value": 15
  },
  "IsDisabled": false,
  "Key": "ability_magician_cloneturret",
  "LeashRadius": 20,
  "Name": "Spectral Assistant",
  "TotalSwaps": 2,
  "TurretBulletTargetAngle": 20,
  "TurretBulletTargetRadius": 500,
  "TurretBulletVerticalOffset": 2,
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "AbilityCastRange": 5,
      "AbilityDuration": 7,
      "LeashRadius": 5
    },
    {
      "BonusFireRate": 60,
      "Damage": 12.6
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
    "card_name": "Spectral Assistant",
    "hero_key": "hero_magician",
    "hero_name": "Sinclair",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_magician",
      "hero_name": "Sinclair",
      "lookup": "spectral assistant",
      "name": "Spectral Assistant",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_magician_cloneturret_trigger" title="Spectral Assistant" -->

## Spectral Assistant

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_cloneturret_trigger`
- Snapshot ID: `39476`
- Source-Dokument: `7070`
- Kurzinfo: Spectral Assistant aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spectral Assistant`
- Payload Hash: `6b3b0df1a9e136fb17c269dd1e800ac0cc1e7fdc563874db78ed99ac65f49cec`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.432137+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 2,
  "BehaviourBits": [
    "BehaviorDontBreakInvisibility",
    "BehaviorDontInterruptSprint",
    "BehaviorCastableWhileBusy",
    "BehaviorNoTarget",
    "BehaviorCastableWhileHidden",
    "BehaviorIgnoreSelectionMashProtection",
    "BehaviorTrigger",
    "BehaviorMovement"
  ],
  "IsDisabled": false,
  "Key": "ability_magician_cloneturret_trigger",
  "Name": "Spectral Assistant",
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_trapper_fear" title="Spectral Silk" -->

## Spectral Silk

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_fear`
- Snapshot ID: `39543`
- Source-Dokument: `7070`
- Kurzinfo: Spectral Silk aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spectral Silk`
- Payload Hash: `7f8ff2eda3ff9a324a33787bc9f4d1b8cd44024e51c8fe9e5206290950d1fbf5`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.591707+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.3,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "BuildUpModifier": {
        "BuildUpDecayDelay": 3.0,
        "Class": "CitadelBaseBuildup",
        "Subclass": "FearBuildup"
      },
      "BuildupProcModifier": {
        "Class": "TrapperImmobilize",
        "EnabledStateMask": [
          "Immobilized",
          "GlowThroughWallsToEnemy",
          "GlowThroughWallsToProvider",
          "GlowToProvider"
        ],
        "StatusEffectPriority": 0,
        "Subclass": "TrapperImmobilize"
      },
      "Class": "FearWatcher",
      "Subclass": "FearWatcher"
    }
  ],
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet"
  ],
  "BuildUpDuration": 15,
  "BuildupProcDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.488
    },
    "Value": 120
  },
  "BuildupProcDuration": 2,
  "BuildupSpiritDamageThreshold": 200,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_trapper_fear",
  "Name": "Spectral Silk",
  "SlowPercent": 50,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_fissure_wall" title="Spectral Wall" -->

## Spectral Wall

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_fissure_wall`
- Snapshot ID: `39609`
- Source-Dokument: `7070`
- Kurzinfo: Spectral Wall aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spectral Wall`
- Payload Hash: `3628dfd87b2e0baf5ebbc791a6e165a7e230b4718c6ba2a2e69ad47b448832bd`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.765254+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 50,
  "AbilityCooldown": 50.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventBotUsage"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.731203
    },
    "Value": 60
  },
  "IsDisabled": false,
  "Key": "citadel_ability_fissure_wall",
  "MinRange": 5,
  "Name": "Spectral Wall",
  "NumWallSegments": 8,
  "PushForce": 175,
  "SegmentEmitTime": 0.1,
  "SlowDuration": 2.5,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "SlowPercent": 20,
  "TimeBetweenSegments": 0.035,
  "TimeToMaxDistance": 1.8,
  "Upgrades": [
    {
      "BonusDamagePercent": 20,
      "DebuffDuration": 7
    },
    {
      "AbilityCooldown": -20.0,
      "AbilityDuration": 2
    },
    {
      "CreateTurrets": 2,
      "SlowPercent": 30,
      "TurretLifeTime": 8
    }
  ],
  "WallImpactRange": 5,
  "WallModifier": {
    "Class": "FissureWall",
    "DebuffModifier": {
      "Class": "BonusDamagePercent",
      "Subclass": "BonusDamagePercent"
    },
    "EnemyVisionModifier": {
      "Class": "Base",
      "EnabledStateMask": [
        "GlowThroughWallsToEnemy"
      ],
      "Subclass": "EnemyVision"
    },
    "SentryDistanceFromWall": 80,
    "SlowModifier": {
      "Class": "SlowBase",
      "Subclass": "Slow"
    },
    "Subclass": "FissureWall"
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
    "card_name": "Spectral Wall",
    "hero_key": "hero_forge",
    "hero_name": "McGinnis",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_forge",
      "hero_name": "McGinnis",
      "lookup": "spectral wall",
      "name": "Spectral Wall",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="thumper_ability_2" title="Spike Strip" -->

## Spike Strip

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `thumper_ability_2`
- Snapshot ID: `39719`
- Source-Dokument: `7070`
- Kurzinfo: Spike Strip aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spike Strip`
- Payload Hash: `ba10ef3954fa65015b5864b3a8747589febdce0bc625d77a24e84ffe432e7ea7`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.036199+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.42,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "BarbedWireAuraModifier": {
    "Class": "ThumperAbility2Aura",
    "ProvidedByAura": {
      "Class": "ThumperAbility2",
      "EnabledStateMask": [
        "Slowed",
        "GlowThroughWallsToEnemy"
      ],
      "StatusEffectPriority": 100,
      "Subclass": "ThumperAbility2"
    },
    "Subclass": "ThumperAbility2Aura"
  },
  "BarbedWireDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 10
  },
  "BarbedWireDamagePerMeter": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 30
  },
  "BarbedWireHeightOffGround": 1,
  "BarbedWireRadius": 4,
  "BarbedWireSlow": 50,
  "BarbedWireTickRate": 0.5,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 125
  },
  "ImpactInterval": 0.1,
  "IsDisabled": false,
  "Key": "thumper_ability_2",
  "Name": "Spike Strip",
  "StompRange": 25,
  "TechCleaveExpireTime": 0.2,
  "Upgrades": [
    {
      "AbilityDuration": 2
    },
    {
      "BarbedWireRadius": 3
    },
    {
      "BarbedWireDamagePerMeter": 30
    }
  ],
  "VerticalDifferenceTolerance": 2.5,
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_thumper",
      "hero_name": "Thumper",
      "lookup": "spike strip",
      "name": "Spike Strip",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_gravity_lasso" title="Spirit Lasso" -->

## Spirit Lasso

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_gravity_lasso`
- Snapshot ID: `39452`
- Source-Dokument: `7070`
- Kurzinfo: Spirit Lasso aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Lasso`
- Payload Hash: `019bce376b390070492d3f73bcc834821f216503c842a28ffcdde3fb06c65fd1`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.358957+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.5,
  "AbilityCastRange": 20,
  "AbilityCooldown": 130,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 2.25,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorCleaveDisabled",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorDontInterruptSlideOnCast",
    "BehaviorInterruptMeleeOnCast"
  ],
  "BouncePadExtendDuration": 1.0,
  "CameraPreviewDistance": 200,
  "CameraPreviewOffset": 25,
  "CameraPreviewSpeed": 0.6,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.93
    },
    "Value": 80
  },
  "ExtraTargetConeAngle": 60,
  "ExtraTargetHorizontalOffset": 30,
  "FollowDampingFactor": 8,
  "FollowDistance": 60,
  "GrabExtraTargetsRadiusMult": 2,
  "IsDisabled": false,
  "Key": "ability_gravity_lasso",
  "LassoTargetMaxSpeed": 55,
  "LiftHeight": 7,
  "LiftHorizontal": -30,
  "LiftInitialDelay": 0.5,
  "LiftInitialRisingSpeed": 100,
  "LiftInitialVelocityStart": 500,
  "Name": "Spirit Lasso",
  "Upgrades": [
    {
      "Damage": 80
    },
    {
      "AbilityDuration": 0.75
    },
    {
      "AbilityCooldown": -40
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
    "card_name": "Spirit Lasso",
    "hero_key": "hero_astro",
    "hero_name": "Holliday",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_astro",
      "hero_name": "Holliday",
      "lookup": "spirit lasso",
      "name": "Spirit Lasso",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_immobilize_trap" title="Spirit Snare" -->

## Spirit Snare

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_immobilize_trap`
- Snapshot ID: `39465`
- Source-Dokument: `7070`
- Kurzinfo: Spirit Snare aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Snare`
- Payload Hash: `91f36dddec5db0b906d33ff9d80309d883833bb0d45bb4eb85a71f5386c5c0c0`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.394782+00:00`

### Vollstaendige Payload

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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="viscous_goo_grenade" title="Splatter" -->

## Splatter

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `viscous_goo_grenade`
- Snapshot ID: `39735`
- Source-Dokument: `7070`
- Kurzinfo: Splatter aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Splatter`
- Payload Hash: `5416144abc694ba8e53f26200119c1da12404b3064a60570de4d6b8dbd4ef5a7`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.082530+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.001,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.8
    },
    "Value": 70
  },
  "DetonateCooldown": 0.15,
  "FourthHitDamagePercentage": 0.26,
  "GooGrenadeImpactModifier": {
    "Class": "ViscousGooGrenadeDebuff",
    "Subclass": "ViscousGooGrenadeDebuff"
  },
  "GooGrenadePuddleAuraFriendlyModifier": {
    "AuraRadius": -1.0,
    "Class": "BaseAura",
    "ProvidedByAura": {
      "Class": "Base",
      "Subclass": "GooPuddleSlideModifier"
    },
    "Subclass": "GooGrenadeFriendlyAura"
  },
  "GooGrenadePuddleAuraModifier": {
    "AuraRadius": -1.0,
    "Class": "ViscousGooAura",
    "ProvidedByAura": {
      "Class": "SlowBase",
      "Subclass": "PuddleSlow"
    },
    "Subclass": "GooGrenadePuddleAura"
  },
  "IsDisabled": false,
  "Key": "viscous_goo_grenade",
  "MaxBounces": 1,
  "Name": "Splatter",
  "PuddleDuration": {
    "Scale": {
      "Type": "duration",
      "Value": 1.1
    },
    "Value": 10
  },
  "PuddleSlideBuff": 60,
  "Radius": 5,
  "SecondHitDamagePercentage": 0.5,
  "SlowPercent": 35,
  "ThirdHitDamagePercentage": 0.38,
  "Upgrades": [
    {
      "Damage": 36,
      "Radius": 2
    },
    {
      "AbilityCooldown": -14
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 0
      },
      "MaxBounces": 2
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
    "card_name": "Splatter",
    "hero_key": "hero_viscous",
    "hero_name": "Viscous",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_viscous",
      "hero_name": "Viscous",
      "lookup": "splatter",
      "name": "Splatter",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_hornet_chain" title="Stake" -->

## Stake

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hornet_chain`
- Snapshot ID: `39615`
- Source-Dokument: `7070`
- Kurzinfo: Stake aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Stake`
- Payload Hash: `f415a9275aa2561701411a810bb46fe6da68294386804db2ce54ba9c66e83ee9`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.780854+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 40.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "CaptureRadius": 9,
  "ChainDuration": 1.75,
  "ChainLength": 9,
  "ChainModifier": {
    "Class": "CitadelHornetChainConnection",
    "Subclass": "CitadelHornetChainConnection"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.5
    },
    "Value": 40
  },
  "DisarmModifier": {
    "Class": "CitadelDisarmed",
    "Subclass": "CitadelDisarmed"
  },
  "EnemyDragSpeed": 25.4,
  "IsDisabled": false,
  "Key": "citadel_ability_hornet_chain",
  "Name": "Stake",
  "SlowPercent": 40,
  "Upgrades": [
    {
      "Damage": 65
    },
    {
      "AbilityCooldown": -22.0
    },
    {
      "CaptureRadius": 2,
      "ChainDuration": 0.75
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
    "card_name": "Stake",
    "hero_key": "hero_hornet",
    "hero_name": "Vindicta",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_hornet",
      "hero_name": "Vindicta",
      "lookup": "stake",
      "name": "Stake",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="drifter_shadow_mark" title="Stalker's Mark" -->

## Stalker's Mark

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `drifter_shadow_mark`
- Snapshot ID: `39673`
- Source-Dokument: `7070`
- Kurzinfo: Stalker's Mark aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Stalker's Mark`
- Payload Hash: `f26c21a8a580570440b27ac42774d8320fd3d6e04fa0e61ac3bd96f10fb77c02`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.915696+00:00`

### Vollstaendige Payload

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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_static_charge" title="Static Charge" -->

## Static Charge

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_static_charge`
- Snapshot ID: `39645`
- Source-Dokument: `7070`
- Kurzinfo: Static Charge aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Static Charge`
- Payload Hash: `ba5ea265916add13bce8df5df72b69597950fa887c59ccd4e843110c00915966`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.851826+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 15,
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.792137
    },
    "Value": 35
  },
  "IsDisabled": false,
  "Key": "citadel_ability_static_charge",
  "Name": "Static Charge",
  "ShockDelay": 3.5,
  "ShockRadius": 5,
  "StaticChargeModifier": {
    "Class": "CitadelStaticcharge",
    "Subclass": "CitadelStaticcharge"
  },
  "StunDuration": 0.9,
  "Upgrades": [
    {
      "AbilityCooldown": -20.0
    },
    {
      "AbilityCastRange": 5,
      "ShockRadius": 7
    },
    {
      "Damage": 160,
      "StunDuration": 0.9
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
    "card_name": "Static Charge",
    "hero_key": "hero_gigawatt",
    "hero_name": "Seven",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_gigawatt",
      "hero_name": "Seven",
      "lookup": "static charge",
      "name": "Static Charge",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_static_charge_v2" title="Static Charge" -->

## Static Charge

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_static_charge_v2`
- Snapshot ID: `39646`
- Source-Dokument: `7070`
- Kurzinfo: Static Charge aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Static Charge`
- Payload Hash: `e12526d9b7074bb877bd72b65a267b14357be93d4ace8f5ecd6779e5bedbf554`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.854185+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0651
    },
    "Value": 16
  },
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorUseInstantCastUnitTargetUi"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.792137
    },
    "Value": 35
  },
  "IsDisabled": false,
  "Key": "citadel_ability_static_charge_v2",
  "Name": "Static Charge",
  "ShockDelay": 3.5,
  "ShockRadius": 5,
  "StaticChargeModifier": {
    "Class": "CitadelStaticcharge",
    "Subclass": "CitadelStaticcharge"
  },
  "StaticChargeWorldModifier": {
    "Class": "CitadelStaticcharge",
    "Subclass": "CitadelStaticchargeWorld"
  },
  "StunDuration": 0.9,
  "Upgrades": [
    {
      "AbilityCooldown": -19.0
    },
    {
      "ShockRadius": 7
    },
    {
      "StunDuration": 0.9
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_bebop_stickybomb2" title="Sticky Bomb" -->

## Sticky Bomb

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_bebop_stickybomb2`
- Snapshot ID: `39395`
- Source-Dokument: `7070`
- Kurzinfo: Sticky Bomb aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Sticky Bomb`
- Payload Hash: `b4f25752313f874e4f39564c266d562da5067c26f8beb086b42835558a580f85`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.220399+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.01,
  "AbilityCooldown": 18,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCastableWhileDodging",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BombFriction": 3,
  "BombRestitution": 1.3,
  "BonusDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.86
    },
    "Value": 180
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.744
    },
    "Value": 65
  },
  "DebuffDuration": 7,
  "DebuffModifier": {
    "Class": "WardenCrowdControlDebuff",
    "EnabledStateMask": [
      "Slowed"
    ],
    "Subclass": "WardenCrowdControlDebuff"
  },
  "ForwardVelocity": 800,
  "IsDisabled": false,
  "Key": "ability_bebop_stickybomb2",
  "MoveSpeedSlowPct": 20,
  "Name": "Sticky Bomb",
  "ProjectileLifetime": 2,
  "Radius": 6.5,
  "SlowDuration": 3,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "Upgrades": [
    {
      "AbilityCooldown": -8
    },
    {
      "BonusDamage": 50,
      "Damage": 50
    },
    {
      "AbilityCooldown": -6.5
    }
  ],
  "WeaponPowerDebuff": -30,
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_sticky_bomb" title="Sticky Bomb" -->

## Sticky Bomb

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_sticky_bomb`
- Snapshot ID: `39647`
- Source-Dokument: `7070`
- Kurzinfo: Sticky Bomb aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Sticky Bomb`
- Payload Hash: `3d395adb21b545a41f83a0ecdc0dd82e1045fe22ea3fc44bf6562abd8c90f712`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.855987+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 6,
  "AbilityCooldown": 18.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 3.5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAllowSelfCast",
    "BehaviorDisplaysDamageImpact",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "BombAttachedModifier": {
    "Class": "CitadelStickyBombAttached",
    "OnGroundModifier": {
      "Class": "CitadelStickyBombOnGround",
      "OnGroundModifier": {
        "Class": "Base",
        "Subclass": "Empty"
      },
      "Subclass": "CitadelStickyBombOnGround"
    },
    "Subclass": "CitadelStickyBombAttached"
  },
  "BonusDamagePctPerPlayerHit": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0015
    },
    "Value": 1.0
  },
  "BonusDamagePctPerPlayerKilled": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.01
    },
    "Value": 2.5
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.5
    },
    "Value": 85
  },
  "FuseTime": 3.5,
  "IsDisabled": false,
  "Key": "citadel_ability_sticky_bomb",
  "KillCheckModifier": {
    "Class": "Base",
    "Subclass": "KillcheckModifier"
  },
  "KillCheckWindow": 10.0,
  "Name": "Sticky Bomb",
  "OnHitDiminish": 60,
  "OnKillDiminish": 7,
  "Radius": 8,
  "SelfBuffModifier": {
    "Class": "Base",
    "Subclass": "CitadelStickyBombSelfBuff"
  },
  "SelfDamagePercent": 20,
  "Upgrades": [
    {
      "AbilityCooldown": -8
    },
    {
      "Damage": 85
    },
    {
      "MovementSpeedBonus": 5,
      "MovementSpeedBonusDuration": 6,
      "StatusResistancePercent": 25
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
    "card_name": "Sticky Bomb",
    "hero_key": "hero_bebop",
    "hero_name": "Bebop",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_bebop",
      "hero_name": "Bebop",
      "lookup": "sticky bomb",
      "name": "Sticky Bomb",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_tier2boss_stomp" title="Stomp" -->

## Stomp

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tier2boss_stomp`
- Snapshot ID: `39658`
- Source-Dokument: `7070`
- Kurzinfo: Stomp aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Stomp`
- Payload Hash: `982109d82fb6348289d73eca6e659e8a65125ac0172dc356af450801656aaa3e`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.882220+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1.5,
  "AbilityCastRange": 12.0,
  "AbilityCooldown": 6,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 100,
  "ActivationDistance": 472.441,
  "BehaviourBits": [
    "BehaviorNoTarget"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "citadel_ability_tier2boss_stomp",
  "Name": "Stomp",
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_tengu_stone_form" title="Stone Form" -->

## Stone Form

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tengu_stone_form`
- Snapshot ID: `39653`
- Source-Dokument: `7070`
- Kurzinfo: Stone Form aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Stone Form`
- Payload Hash: `f54e8b253dfc6b91a68bd405f3905a91eb8653719c9311e579217b7848ae6473`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.870472+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCooldown": 40.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 3,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorCastableWhileBusy",
    "BehaviorInterruptMeleeOnCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 75
  },
  "DampingFactor": 0.25,
  "DragModifier": {
    "Class": "ChargeDragEnemy",
    "ForceDistScale": 10,
    "ForwardOffset": 0,
    "Subclass": "ChargeDragEnemy",
    "VerticalOffset": -200
  },
  "IsDisabled": false,
  "Key": "citadel_ability_tengu_stone_form",
  "LiftHeight": 180,
  "LiftTime": 1.0,
  "MaxHealthRegen": 6,
  "MoveSpeedMax": 8,
  "Name": "Stone Form",
  "Radius": 6,
  "StunDuration": 0.75,
  "Upgrades": [
    {
      "MaxHealthRegen": 7.0
    },
    {
      "AbilityCooldown": -25
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.7
        },
        "Value": 0
      },
      "StunDuration": 1
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
    "card_name": "Stone Form",
    "hero_key": "hero_tengu",
    "hero_name": "Ivy",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_tengu",
      "hero_name": "Ivy",
      "lookup": "stone form",
      "name": "Stone Form",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_storm_cloud" title="Storm Cloud" -->

## Storm Cloud

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_storm_cloud`
- Snapshot ID: `39649`
- Source-Dokument: `7070`
- Kurzinfo: Storm Cloud aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Storm Cloud`
- Payload Hash: `67a76769c2c75772872101b3c60e00badb18259eda229408a25cd85d2fa5cb50`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.861179+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": 7,
  "AbilityCooldown": 205.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorExclusiveUse",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "CameraDistance": 600,
  "CloudHeight": 120,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 95
  },
  "DamageInterval": 0.3,
  "EndingSoonTime": 2,
  "ExpandTime": 3.5,
  "FlightControlEnabled": 1.5,
  "InitialRadius": 10,
  "IsDisabled": false,
  "Key": "citadel_ability_storm_cloud",
  "LightningStrikeAOEModifier": {
    "Class": "LightningStrikeArea",
    "Subclass": "LightningStrike"
  },
  "LightningStrikeDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.5
    },
    "Value": 75.0
  },
  "LightningStrikeDelay": 0.25,
  "LightningStrikeKnockBackForce": 500,
  "LightningStrikeRadius": 7,
  "LightningStrikes": 1,
  "Name": "Storm Cloud",
  "Radius": 30,
  "StormCloudModifier": {
    "Class": "CitadelStormcloud",
    "Subclass": "CitadelStormcloud"
  },
  "Upgrades": [
    {
      "BulletResistOnActive": 55
    },
    {
      "AbilityChannelTime": 7,
      "InitialRadius": 5,
      "Radius": 10
    },
    {
      "DPS": 65.0,
      "FlightControlEnabled": 4
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
    "card_name": "Storm Cloud",
    "hero_key": "hero_gigawatt",
    "hero_name": "Seven",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_gigawatt",
      "hero_name": "Seven",
      "lookup": "storm cloud",
      "name": "Storm Cloud",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="trooper_boss_grenade" title="Stun Grenade" -->

## Stun Grenade

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `trooper_boss_grenade`
- Snapshot ID: `39729`
- Source-Dokument: `7070`
- Kurzinfo: Stun Grenade aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Stun Grenade`
- Payload Hash: `fa9ed4bdfdd9df95b8021d4f78f09993e8a90fab5031e9fe10a6058626ac1f19`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.068939+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.5,
  "AbilityCastRange": "38.1 50.8",
  "AbilityCooldown": "40 30 20",
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile"
  ],
  "BlastRadius": 300,
  "ChannelMoveSpeed": -1,
  "Damage": 100,
  "FuseDuration": 3,
  "Grenades": "1 1 3",
  "ImpulseVelocity": 1200,
  "IsDisabled": false,
  "Key": "trooper_boss_grenade",
  "Name": "Stun Grenade",
  "PlayerDamage": 100,
  "StunDuration": "1 2 3",
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="trooper_neutral_grenade" title="Stun Grenade" -->

## Stun Grenade

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `trooper_neutral_grenade`
- Snapshot ID: `39731`
- Source-Dokument: `7070`
- Kurzinfo: Stun Grenade aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Stun Grenade`
- Payload Hash: `5ab0a4735b17846679b2b75d0d688539fe17ba61a9651ac105de32df4894cee2`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.074393+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.5,
  "AbilityCastRange": 50.8,
  "AbilityCooldown": 40,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile"
  ],
  "BlastRadius": 300,
  "ChannelMoveSpeed": -1,
  "Damage": 50,
  "FuseDuration": 3,
  "Grenades": 1,
  "ImpulseVelocity": 1200,
  "IsDisabled": false,
  "Key": "trooper_neutral_grenade",
  "Name": "Stun Grenade",
  "PlayerDamage": 100,
  "StunDuration": 1,
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_boho_doublehit" title="Swish" -->

## Swish

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_boho_doublehit`
- Snapshot ID: `39403`
- Source-Dokument: `7070`
- Kurzinfo: Swish aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Swish`
- Payload Hash: `facfad02418ab30345355bb56d464e95f691cab135249f7237ea0a2c41ff4955`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.238054+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 11,
  "AbilityCooldown": 14,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.7,
  "AbilityUnitTargetLimit": 16,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusMoveSpeed": 2,
  "BuffDuration": 4,
  "BuffModifier": {
    "Class": "BohoDoublehitBuff",
    "Subclass": "Buff"
  },
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.8
    },
    "Value": 80
  },
  "CombatBarrierPerStack": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.2
    },
    "Value": 20
  },
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 50
  },
  "IsDisabled": false,
  "Key": "ability_boho_doublehit",
  "MaxStacks": 6,
  "Name": "Swish",
  "TargetingConeAngle": 100,
  "TimeBetweenAttacks": 0.35,
  "Upgrades": [
    {
      "Damage": 18.0
    },
    {
      "BonusMoveSpeed": 2
    },
    {
      "CombatBarrier": 80
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
    "card_name": "Swish",
    "hero_key": "hero_boho",
    "hero_name": "Boho",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_boho",
      "hero_name": "Boho",
      "lookup": "swish",
      "name": "Swish",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_familiar_attach" title="Tag Along" -->

## Tag Along

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_familiar_attach`
- Snapshot ID: `39429`
- Source-Dokument: `7070`
- Kurzinfo: Tag Along aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Tag Along`
- Payload Hash: `37bcc630cad3d37293ae2b79c425f015c477b9ff0c6128aa0918e31321bc6f4b`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.302468+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.5,
  "AbilityCastRange": 23,
  "AbilityCooldown": 40,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5.5,
  "AbilityUnitTargetLimit": 1,
  "AllyLockoutModifier": {
    "Class": "Base",
    "Subclass": "Tagalonglockout"
  },
  "AttachHealModifier": {
    "Class": "FamiliarAttachHeal",
    "Subclass": "Attachheal"
  },
  "AttachedModifier": {
    "AttachEndingModifier": {
      "Class": "Base",
      "Subclass": "Attachending"
    },
    "Class": "FamiliarAttached",
    "EndingWarningDuration": 5.0,
    "HostModifier": {
      "Class": "FamiliarAttachhost",
      "Subclass": "Host"
    },
    "ReplicatedBarrierModifier": {
      "Class": "CitadelFamiliarReplicatedbarrier",
      "Subclass": "Familiarbarrier"
    },
    "Subclass": "Attached"
  },
  "AutoCastDelayModifier": {
    "Class": "Base",
    "Subclass": "Castdelay"
  },
  "BehaviourBits": [
    "BehaviorDontBreakInvisibility",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorMovement",
    "BehaviorCanSetQuickCast",
    "BehaviorDoNotAllowSpamProc"
  ],
  "BonusMoveSpeed": 4,
  "CameraDummyModifier": {
    "Class": "FamiliarCamdummy",
    "Subclass": "Cameradummy"
  },
  "ChannelMoveSpeed": 5,
  "DeathBarrierModifier": {
    "Class": "CitadelFamiliarReplicatedbarrier",
    "Subclass": "Deathbarrier"
  },
  "HealingPerSecond": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.4
    },
    "Value": 42
  },
  "HealthRegenDuration": 2,
  "HopOffBuffModifier": {
    "Class": "Base",
    "Subclass": "Hopoutbuff"
  },
  "HopOutLockoutDuration": 0.3,
  "HopOutLockoutModifier": {
    "Class": "FamiliarHopoutlockout",
    "Subclass": "Lockout"
  },
  "IsDisabled": false,
  "Key": "ability_familiar_attach",
  "LaunchTossModifier": {
    "Class": "FamiliarAttachLaunchoff",
    "Subclass": "Launchtoss"
  },
  "LaunchedSelfModifier": {
    "Class": "CitadelModifierFamiliarSpeedlines",
    "Subclass": "Launch"
  },
  "MissingHealthBurstPct": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.03
    },
    "Value": 15
  },
  "MovingToAttachModifier": {
    "Class": "FamiliarMovingtoattach",
    "Subclass": "Movingtoattach"
  },
  "Name": "Tag Along",
  "SpeedModifier": {
    "Class": "Base",
    "Subclass": "Speed"
  },
  "Upgrades": [
    {
      "AbilityCooldown": -8
    },
    {
      "BonusBarrierAmpPercent": 35,
      "BonusItemDurationPercent": 35,
      "BonusItemRangePercent": 35
    },
    {
      "BonusSpiritPower": 35,
      "HealingPerSecond": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.34
        },
        "Value": 0
      },
      "HopOffEffecDuration": 10,
      "MissingHealthBurstPct": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.016
        },
        "Value": 0
      },
      "TechPowerPercent": 15
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
    "card_name": "Tag Along",
    "hero_key": "hero_familiar",
    "hero_name": "Rem",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_familiar",
      "hero_name": "Rem",
      "lookup": "tag along",
      "name": "Tag Along",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_werewolf_cripplingslash" title="Tail Whack" -->

## Tail Whack

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_cripplingslash`
- Snapshot ID: `39573`
- Source-Dokument: `7070`
- Kurzinfo: Tail Whack aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Tail Whack`
- Payload Hash: `9b76d7a5bab18efef63e13fc06bb85e22f6ae529950f39b76dc12a93e8f78539`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.673811+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCooldown": 18,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.5
    },
    "Value": 45
  },
  "DebuffModifier": {
    "Class": "SlowBase",
    "EnabledStateMask": [
      "Slowed"
    ],
    "Subclass": "Debuff"
  },
  "DisarmDuration": 2.0,
  "DisarmModifier": {
    "Class": "CitadelDisarmed",
    "Subclass": "Disarm"
  },
  "IsDisabled": false,
  "Key": "ability_werewolf_cripplingslash",
  "LeftForce": 400,
  "Name": "Tail Whack",
  "PushForce": 300,
  "SlashHeight": 3,
  "SlashRadius": 10,
  "SlowDuration": 2.0,
  "SlowModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "Slow"
  },
  "SlowPercent": 30,
  "Upgrades": [
    {
      "Damage": 25
    },
    {
      "SlowPercent": 40
    },
    {
      "DisarmDuration": 1.5,
      "SlowDuration": 1.5
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
    "card_name": "Tail Whack",
    "hero_key": "hero_werewolf_transformed",
    "hero_name": "Silver (Transformed)",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_werewolf_transformed",
      "hero_name": "Silver (Transformed)",
      "lookup": "tail whack",
      "name": "Tail Whack",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_target_practice" title="Target Practice" -->

## Target Practice

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_target_practice`
- Snapshot ID: `39541`
- Source-Dokument: `7070`
- Kurzinfo: Target Practice aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Target Practice`
- Payload Hash: `a744c903f4169fe0f6c0c8da1d4dd9c2c6215f12905637e35dacb53fa55b6c2f`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.585344+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 48.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSprint"
  ],
  "BonusPerHeadshot": 25,
  "ChannelMoveSpeed": -1,
  "DamageOnBuildup": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.066338
    },
    "Value": 125
  },
  "IsDisabled": false,
  "Key": "ability_target_practice",
  "Name": "Target Practice",
  "ShotsToProc": 4,
  "TargetOffSetScale": 2.0,
  "TargetPracticeDuration": 10,
  "TargetPracticeEnemyModifier": {
    "BuildupCompleteModifier": {
      "Class": "Base",
      "Subclass": "Base"
    },
    "BuildupModifier": {
      "BuildUpDecayDelay": 20.0,
      "Class": "CitadelBaseBuildup",
      "Subclass": "CitadelBaseBuildup"
    },
    "Class": "TargetPracticeEnemy",
    "DebuffModifier": {
      "Class": "Base",
      "Subclass": "TargetPracticeBulletResist"
    },
    "Subclass": "TargetPracticeEnemy"
  },
  "TargetPracticeSelfModifier": {
    "Class": "TargetPracticeSelf",
    "Subclass": "TargetPracticeSelf"
  },
  "Upgrades": [
    {
      "AbilityCooldown": -19.0
    },
    {
      "DamageOnBuildup": 50
    },
    {
      "BulletArmorReduction": -30,
      "DebuffDuration": 6
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_priest_smokegrenade" title="Teargas Grenade" -->

## Teargas Grenade

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_smokegrenade`
- Snapshot ID: `39516`
- Source-Dokument: `7070`
- Kurzinfo: Teargas Grenade aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Teargas Grenade`
- Payload Hash: `a4b84d55da11ab4aaf0ea9df741ddc1a8ee8284a2886fd61cf6f8159c8fdeaf0`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.524509+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 10,
  "AbilityCooldown": 24,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 99,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BlockerScaleFactor": 115,
  "BombFriction": 12,
  "BombRestitution": 2.0,
  "BonusDamge": 12,
  "BonusFireRate": -30,
  "BonusHealthRegen": 1,
  "ChannelMoveSpeed": -1,
  "DebuffDuration": 3,
  "GrowTime": 0.2,
  "IsDisabled": false,
  "Key": "ability_priest_smokegrenade",
  "Name": "Teargas Grenade",
  "ProjectileLifetime": 2,
  "Radius": 6,
  "SlowDuration": 2,
  "SmokeGrenadeModifier": {
    "Class": "Smokegrenade",
    "EnemyAuraModifier": {
      "Class": "BaseAura",
      "ProvidedByAura": {
        "Class": "Base",
        "Subclass": "Debuff"
      },
      "Subclass": "Enemyaura"
    },
    "Subclass": "PriestSmokegrenade"
  },
  "Upgrades": [
    {
      "AbilityDuration": 3
    },
    {
      "StaminaDrain": -1
    },
    {
      "Radius": 2
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_psychic_lift" title="Telekinesis" -->

## Telekinesis

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_psychic_lift`
- Snapshot ID: `39635`
- Source-Dokument: `7070`
- Kurzinfo: Telekinesis aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Telekinesis`
- Payload Hash: `fb8aba40700ccd9b6e3b4524fd55ab6ef720a0b0c1b9a4edd5a757028eca7a34`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.827921+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.45,
  "AbilityCastRange": 10,
  "AbilityChannelTime": 0.65,
  "AbilityCooldown": 150,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 2.25,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 100
  },
  "DampingFactor": 0.3,
  "IsDisabled": false,
  "Key": "citadel_ability_psychic_lift",
  "LiftChainRadius": 20,
  "LiftDuration": 2,
  "LiftHeight": 80,
  "LiftModifier": {
    "Class": "CitadelPsychiclift",
    "DisarmModifier": {
      "Class": "CitadelDisarmed",
      "EnabledStateMask": [
        "Disarmed"
      ],
      "Subclass": "Disarm"
    },
    "EnabledStateMask": [
      "Stunned"
    ],
    "OccilateDegreesPerSecond": 300.0,
    "OccilateMaxDistance": 400.0,
    "RiseAcc": 3000.0,
    "RiseDecayFracEnd": 0.95,
    "RiseDecayFracStart": 0.3,
    "RiseMaxSpeed": 750.0,
    "RiseTime": 0.75,
    "SilenceModifier": {
      "Class": "CitadelSilenced",
      "EnabledStateMask": [
        "Silenced"
      ],
      "Subclass": "Silence"
    },
    "SlamAcc": 6000.0,
    "SlamImpactRadius": 100.0,
    "SlamTime": 0.5,
    "SlowModifier": {
      "Class": "SlowBase",
      "EnabledStateMask": [
        "Slowed",
        "DashDisabledDebuff",
        "SilenceMovementAbilites"
      ],
      "Subclass": "LiftSlowModifier"
    },
    "Subclass": "Lift"
  },
  "Name": "Telekinesis",
  "SlowPercent": 40,
  "TossDistance": 13,
  "TossUpStrength": 220,
  "Upgrades": [
    {
      "Damage": 100
    },
    {
      "AbilityCooldown": -45
    },
    {
      "AbilityCastRange": 6,
      "AbilityDuration": 1.5,
      "TossDistance": 6
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
    "card_name": "Telekinesis",
    "hero_key": "hero_wraith",
    "hero_name": "Wraith",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_wraith",
      "hero_name": "Wraith",
      "lookup": "telekinesis",
      "name": "Telekinesis",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="gunslinger_tenacity" title="Tenacity" -->

## Tenacity

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `gunslinger_tenacity`
- Snapshot ID: `39689`
- Source-Dokument: `7070`
- Kurzinfo: Tenacity aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Tenacity`
- Payload Hash: `7a907ee4082946b1a0bd120d8455c330aa8f38a870f551b7669a1934fbfdf5f5`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.957254+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "BulletLifestealPercent": 15,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "gunslinger_tenacity",
  "LowHealthHealingScalePercent": 100,
  "Name": "Tenacity",
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="viscous_restorative_goo" title="The Cube" -->

## The Cube

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `viscous_restorative_goo`
- Snapshot ID: `39736`
- Source-Dokument: `7070`
- Kurzinfo: The Cube aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `The Cube`
- Payload Hash: `71cf6613c4615a4a79fcf94a593565775a6755a22218c576768f50e0c1063c0f`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.085247+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 26,
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 3,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorEqualUnitTargetPriority",
    "BehaviorAllowSelfCast",
    "BehaviorCanHealPlayers",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusHealthRegen": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.3
    },
    "Value": 40
  },
  "BreakoutTime": 1,
  "BulletForce": 600,
  "ChannelMoveSpeed": -1,
  "CubeScale": 1.5,
  "Friction": -80,
  "HeavyMeleeForce": 700,
  "IsDisabled": false,
  "Key": "viscous_restorative_goo",
  "LightMeleeForce": 300,
  "Name": "The Cube",
  "PostCubeBuffDuration": 8,
  "PushBackForce": 250,
  "PushBackRadius": 50,
  "RestorativeGooModifier": {
    "BreakoutProgressBarModifier": {
      "Class": "Base",
      "Subclass": "Breakoutprogress"
    },
    "Class": "RestorativeGoo",
    "DistanceCameraOffset": 450.0,
    "DistanceCameraOffsetBias": 0.75,
    "DistanceCameraOffsetLerpTime": 0.5,
    "PostCubeBuffModifier": {
      "Class": "Base",
      "Subclass": "CubePostSpeedBuff"
    },
    "Subclass": "RestorativeGoo"
  },
  "SelfCubeModelSwapModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "DoNotDrawModel"
    ],
    "Subclass": "ViscousSelfCubeModelSwap"
  },
  "SlideForce": 70,
  "Upgrades": [
    {
      "BonusMoveSpeed": 2.5,
      "PostCubeBuff": 1,
      "StaminaCooldownReduction": 30
    },
    {
      "AbilityDuration": 1,
      "BonusHealthRegen": 25
    },
    {
      "AbilityCooldown": -25.0,
      "PurgeDebuffs": 1
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
    "card_name": "The Cube",
    "hero_key": "hero_viscous",
    "hero_name": "Viscous",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_viscous",
      "hero_name": "Viscous",
      "lookup": "the cube",
      "name": "The Cube",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_magician_shadowclone" title="The Great Homonculus!" -->

## The Great Homonculus!

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_shadowclone`
- Snapshot ID: `39481`
- Source-Dokument: `7070`
- Kurzinfo: The Great Homonculus! aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `The Great Homonculus!`
- Payload Hash: `333e25925cf0087829c5cd070c530b2a4b9242be6aa364efb680332b2b940ce5`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.444068+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 20,
  "AbilityChannelTime": 1,
  "AbilityCooldown": 145.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorDisplaysDamageImpact",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "CloneDamagePercentage": 30,
  "CloneGoldToGive": 10000,
  "CloneHealthPercentage": 40,
  "CloneLifetime": 60,
  "CloneModifier": {
    "Class": "Shadowclone",
    "Subclass": "ShadowcloneModifierMagician"
  },
  "CloneSpawnDistance": 2,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.046
    },
    "Value": 220
  },
  "IsDisabled": false,
  "Key": "ability_magician_shadowclone",
  "Name": "The Great Homonculus!",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="tokamak_heat_sinks" title="Thermal Vault" -->

## Thermal Vault

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `tokamak_heat_sinks`
- Snapshot ID: `39725`
- Source-Dokument: `7070`
- Kurzinfo: Thermal Vault aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Thermal Vault`
- Payload Hash: `3097ec3b5712df0a364f44b6b64ca95a9996102ac67c02d76d52cfaaa9441532`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.057008+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BaseHeatPower": 20.0,
  "BehaviourBits": [
    "BehaviorNoTarget"
  ],
  "ChannelMoveSpeed": -1,
  "HeatDotModifier": {
    "Class": "TokamakHeatSinksDot",
    "Subclass": "TokamakHeatSinksDot"
  },
  "IsDisabled": false,
  "Key": "tokamak_heat_sinks",
  "MeleeBurnDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.487469
    },
    "Value": 80
  },
  "MeleeHitCount": 3,
  "MeleeIgniteTime": 0.5,
  "Name": "Thermal Vault",
  "TickRate": 0.1,
  "TossSpeed": 400,
  "Upgrades": [
    {
      "MeleeSpeedBonusPercentage": 30
    },
    {
      "WeaponDamagePerHeat": 1.0
    },
    {
      "BaseHeatPower": 20.0,
      "MaxHeatPower": 40.0
    }
  ],
  "WeaponDamagePerHeat": 1.0,
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_chrono_time_wall" title="Time Wall" -->

## Time Wall

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chrono_time_wall`
- Snapshot ID: `39606`
- Source-Dokument: `7070`
- Kurzinfo: Time Wall aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Time Wall`
- Payload Hash: `d1ff39485770a7d910670ad723a4b388d270d91f845e9695521536b5f4091956`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.758716+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 5.08,
  "AbilityCooldown": 25.0,
  "AbilityDuration": 5.5,
  "AbilityUnitTargetLimit": 1,
  "AuraEffectDuration": 2,
  "AuraModifier": {
    "AuraRadius": 0.001,
    "Class": "CitadelChronoTimeWallAura",
    "DebuffModifier": {
      "Class": "CitadelSilenced",
      "Subclass": "CitadelSilenced"
    },
    "ProvidedByAura": {
      "Class": "CitadelChronoTimeWallEffect",
      "Subclass": "CitadelChronoTimeWallEffect"
    },
    "Subclass": "CitadelChronoTimeWallAura"
  },
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanCancelDuringCastDelay",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "FriendlyBulletDamageBonus": 30,
  "IsDisabled": false,
  "Key": "citadel_ability_chrono_time_wall",
  "MovementSlowPct": 80,
  "Name": "Time Wall",
  "TimeScaleDuration": 0.5,
  "TimeWallDepth": 0.5,
  "TimeWallDepthVisualScale": 0.16,
  "TimeWallFormationTime": 0.5,
  "TimeWallHeight": 4,
  "TimeWallTimeScale": 0.0001,
  "TimeWallTimeScaleFriendly": 2,
  "TimeWallWidth": 8,
  "Upgrades": [
    {
      "AbilityDuration": 3.5,
      "TimeWallHeight": 1,
      "TimeWallWidth": 3
    },
    {
      "DebuffDuration": 2.3,
      "FriendlyBulletDamageBonus": 35
    },
    {
      "AbilityCharges": 3,
      "AbilityCooldownBetweenCharge": 2
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
    "card_name": "Time Wall",
    "hero_key": "hero_chrono",
    "hero_name": "Paradox",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_chrono",
      "hero_name": "Paradox",
      "lookup": "time wall",
      "name": "Time Wall",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_kali_trappers_bolo" title="Trapper's Delight" -->

## Trapper's Delight

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_kali_trappers_bolo`
- Snapshot ID: `39469`
- Source-Dokument: `7070`
- Kurzinfo: Trapper's Delight aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Trapper's Delight`
- Payload Hash: `b1795a64876a71a8160193103363e59797b9fa1bf2b8412b6c8bf8ea86ea84a1`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.408883+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 25,
  "AbilityCooldown": 127.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact"
  ],
  "BoloBounceCount": 6,
  "BoloBounceSpeed": 800,
  "BoloContractRadius": 5,
  "BoloHitDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 25
  },
  "BoloProcDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 75
  },
  "BoloRadius": 0.8,
  "ChannelMoveSpeed": -1,
  "DebuffDelay": 2,
  "DebuffModifier": {
    "Class": "CitadelBolo",
    "ReverseLeechModifier": {
      "Class": "CitadelBoloLeech",
      "Subclass": "CitadelBoloLeech"
    },
    "Subclass": "CitadelBolo",
    "TrapModifier": {
      "Class": "CitadelRoot",
      "EnabledStateMask": [
        "GlowThroughWallsToProvider",
        "GlowToProvider"
      ],
      "Subclass": "CitadelRoot"
    }
  },
  "ImmobilizeDuration": 2.0,
  "IsDisabled": false,
  "Key": "ability_kali_trappers_bolo",
  "MaxGroundDashReduction": -50,
  "MaxSlow": 100,
  "Name": "Trapper's Delight",
  "Upgrades": [
    {
      "ReverseLifeLeech": 30,
      "ReverseLifeLeechDuration": 8
    },
    {
      "AbilityCooldown": -47.0
    },
    {
      "StunsTargets": 1
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_kali",
      "hero_name": "Kali",
      "lookup": "trapper's delight",
      "name": "Trapper's Delight",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="mirage_teleport" title="Traveler" -->

## Traveler

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `mirage_teleport`
- Snapshot ID: `39694`
- Source-Dokument: `7070`
- Kurzinfo: Traveler aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Traveler`
- Payload Hash: `2b57d01b755bbb334f1dfa84be64bdbd09daadf90be535b70121c61483a99c63`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.969450+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 140.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorCanCastOnZipline",
    "BehaviorCastableWhileBusy",
    "BehaviorRequireAbilityButtonToCancel",
    "BehaviorCanCancelDuringCastDelay"
  ],
  "BuffModifier": {
    "Class": "MirageTravelerMovementSpeed",
    "Subclass": "MirageTeleportMovementSpeed"
  },
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "ImmunityModifier": {
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
    "StatusEffectPriority": 0,
    "Subclass": "Unstoppable"
  },
  "InterruptCooldown": 4,
  "InterruptNotificationModifier": {
    "Class": "Base",
    "Duration": 2.0,
    "Subclass": "Notification"
  },
  "IsDisabled": false,
  "Key": "mirage_teleport",
  "Name": "Traveler",
  "SearchRadius": 30,
  "TeleportCompletedTime": 2,
  "Upgrades": [
    {
      "BonusFireRate": 20,
      "BonusMoveSpeed": 3,
      "MovementSpeedBonusDuration": 12
    },
    {
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 400
      }
    },
    {
      "AbilityCooldown": -70
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
    "card_name": "Traveler",
    "hero_key": "hero_mirage",
    "hero_name": "Mirage",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_mirage",
      "hero_name": "Mirage",
      "lookup": "traveler",
      "name": "Traveler",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_fissure_wall_trigger" title="Trigger Fissure Wall" -->

## Trigger Fissure Wall

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_fissure_wall_trigger`
- Snapshot ID: `39611`
- Source-Dokument: `7070`
- Kurzinfo: Trigger Fissure Wall aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Trigger Fissure Wall`
- Payload Hash: `5849332241c8263d6876b4921b494025e91dd32f354b7e34a72cf73917e5bfd6`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.771248+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontInterruptSprint",
    "BehaviorNoTarget",
    "BehaviorCastableWhileHidden",
    "BehaviorIgnoreSelectionMashProtection",
    "BehaviorTrigger"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "citadel_ability_fissure_wall_trigger",
  "Name": "Trigger Fissure Wall",
  "Upgrades": [],
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="operative_umbrella_maneuver" title="Umbrella Maneuver" -->

## Umbrella Maneuver

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `operative_umbrella_maneuver`
- Snapshot ID: `39700`
- Source-Dokument: `7070`
- Kurzinfo: Umbrella Maneuver aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Umbrella Maneuver`
- Payload Hash: `32dd93d1ae0911eef97f2787715d6e747b662ac10cf55d2ecf9503bc4a99171a`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.986317+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 32.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "ActivateTime": 0.2,
  "AirHangModifier": {
    "AirDrag": -1,
    "AirSpeed": -1,
    "Class": "OperativeUmbrellaManeuverAirHang",
    "FallSpeed": 10,
    "Subclass": "OperativeUmbrellaManeuverAirHang"
  },
  "AirSpeedMax": 3.81,
  "BackwardsVelocity": 13.0,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.395
    },
    "Value": 100
  },
  "ExplodeRadius": 5,
  "FallSpeedMax": 1.524,
  "IsDisabled": false,
  "Key": "operative_umbrella_maneuver",
  "Name": "Umbrella Maneuver",
  "TimeBeforeProjectileLaunch": 1.25,
  "UpImpulse": 15.0,
  "Upgrades": [
    {
      "AbilityCooldown": -14
    },
    {
      "Damage": 50
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Umbrella Maneuver",
    "hero_key": "hero_operative",
    "hero_name": "Raven",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_operative",
      "hero_name": "Raven",
      "lookup": "umbrella maneuver",
      "name": "Umbrella Maneuver",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_magician_magicbolt" title="Vexing Bolt" -->

## Vexing Bolt

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_magicbolt`
- Snapshot ID: `39479`
- Source-Dokument: `7070`
- Kurzinfo: Vexing Bolt aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Vexing Bolt`
- Payload Hash: `a979a8df3e10d8d9ee56eaf22a7aed3c98d4ba8fc297eb99653846a60f75257e`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.439711+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 500,
  "AbilityCharges": 1,
  "AbilityCooldown": 24,
  "AbilityCooldownBetweenCharge": 3,
  "AbilityPostCastDuration": 0.3,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorTargetThroughWalls",
    "BehaviorCleaveDisabled",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "CloneBoltDelay": 0.1,
  "CloneDamagePercentage": 50.0,
  "InitialProjectileVelocity": 800,
  "IsDisabled": false,
  "Key": "ability_magician_magicbolt",
  "MaxDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.86
    },
    "Value": 120
  },
  "MaxDamageTime": 2,
  "MinDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.93
    },
    "Value": 60
  },
  "Name": "Vexing Bolt",
  "ProjectileLifetime": 4,
  "ProjectileRedirectCount": 1,
  "Radius": 3.25,
  "RedirectVelocity": 1500,
  "TargetDebuffModifier": {
    "Class": "Base",
    "Subclass": "MagicianMagicboltDebuff"
  },
  "Upgrades": [
    {
      "DebuffDuration": 5,
      "FireRateSlow": 25
    },
    {
      "AbilityCooldown": -13
    },
    {
      "CloneDamagePercentage": 50.0,
      "MaxDamage": 126.0
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
    "card_name": "Vexing Bolt",
    "hero_key": "hero_magician",
    "hero_name": "Sinclair",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_magician",
      "hero_name": "Sinclair",
      "lookup": "vexing bolt",
      "name": "Vexing Bolt",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="thumper_ability_4" title="Vortex" -->

## Vortex

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `thumper_ability_4`
- Snapshot ID: `39721`
- Source-Dokument: `7070`
- Kurzinfo: Vortex aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Vortex`
- Payload Hash: `cd1bbaf0d186ecdfcb6fa5510818d8fe0c2a0fb0c2781b6886b0be21dfd1fba8`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:24.043407+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 10.5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "ClimbHeight": 1,
  "DistanceAboveGround": 2,
  "DropDownRate": 2,
  "Duration": 4,
  "InitialForce": 300,
  "IsDisabled": false,
  "Key": "thumper_ability_4",
  "Name": "Vortex",
  "PullAOEModifier": {
    "Class": "ThumperPullAoe",
    "ProvidedByAura": {
      "Class": "ThumperEnemyPulled",
      "Subclass": "ThumperEnemyPulled"
    },
    "Subclass": "ThumperPullAoe"
  },
  "PushAccel": 1000,
  "PushNPCSpeed": 800,
  "Radius": 15,
  "TornadoSpeed": 350,
  "Upgrades": [
    {
      "AbilityCooldown": -0.75
    },
    {
      "AbilityCooldown": -0.75
    },
    {
      "AbilityCooldown": -0.75
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_thumper",
      "hero_name": "Thumper",
      "lookup": "vortex",
      "name": "Vortex",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_warden_high_alert" title="Willpower" -->

## Willpower

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_warden_high_alert`
- Snapshot ID: `39570`
- Source-Dokument: `7070`
- Kurzinfo: Willpower aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Willpower`
- Payload Hash: `459868ba390f817e0690a05ea7488084ba2644710d97714bd5b6a72e43b1b203`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.663809+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 40,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "BuffModifier": {
    "Class": "WardenHighAlert",
    "StatusEffectPriority": 100,
    "Subclass": "WardenHighAlert"
  },
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.8
    },
    "Value": 125
  },
  "IsDisabled": false,
  "Key": "ability_warden_high_alert",
  "MoveSpeedBonusPct": 15,
  "Name": "Willpower",
  "Upgrades": [
    {
      "MoveSpeedBonusPct": 20
    },
    {
      "AbilityCooldown": -24,
      "AbilityDuration": 2
    },
    {
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 2.7
        },
        "Value": 0
      },
      "StatusResistancePercent": 40
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
    "card_name": "Willpower",
    "hero_key": "hero_warden",
    "hero_name": "Warden",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_warden",
      "hero_name": "Warden",
      "lookup": "willpower",
      "name": "Willpower",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_priest_barrage" title="Witching Hour" -->

## Witching Hour

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_barrage`
- Snapshot ID: `39510`
- Source-Dokument: `7070`
- Kurzinfo: Witching Hour aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Witching Hour`
- Payload Hash: `119c53d2490888d6a87224a7610692d1246ea1b983b8f6eebeb625e1a52b0756`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.510769+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.3,
  "AbilityChannelTime": 3,
  "AbilityCooldown": 60,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "weapon_damage_increase",
      "Value": 1.0
    },
    "Value": 80
  },
  "ExplodeRadius": 0.2,
  "IsDisabled": false,
  "Key": "ability_priest_barrage",
  "Name": "Witching Hour",
  "PushForce": 300000,
  "SelfModifier": {
    "Class": "Base",
    "Subclass": "Self"
  },
  "SlowDuration": 1,
  "SlowModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "BarrageSlowModifier"
  },
  "SlowPercent": 60,
  "TimeBetweenShots": 0.3,
  "TotalShotCount": 4,
  "Upgrades": [
    {
      "AbilityCooldown": -25
    },
    {
      "Damage": 40
    },
    {
      "ExecuteThreshold": 30
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_necro_wither" title="Wither & Die" -->

## Wither & Die

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_necro_wither`
- Snapshot ID: `39502`
- Source-Dokument: `7070`
- Kurzinfo: Wither & Die aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Wither & Die`
- Payload Hash: `f75a506edd1437a45ea2ee1195b4fd5e7bd003fcd4a9834b87a1d00bf125c899`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.492154+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AreaModifier": {
    "Class": "NecroHauntingskullArea",
    "InitialRandomVariance": 30.0,
    "SpawnPositionNavMeshSearchRange": 60.0,
    "Subclass": "Area"
  },
  "BehaviourBits": null,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.3
    },
    "Value": 20
  },
  "DelayBeforeRespawning": 1,
  "IsDisabled": false,
  "Key": "ability_necro_wither",
  "MaxHits": -1,
  "MaxStacks": 40,
  "Name": "Wither & Die",
  "SkullImmuneDuration": 0.15,
  "SkullLifetime": 10,
  "SkullsOnKill": 1,
  "SkullsOnPlayerKill": 3,
  "SlowPercentPerStack": 0.5,
  "SpawnRadius": 2,
  "StackDuration": 4,
  "StackingDebuffModifier": {
    "Class": "NecroHauntingskullStackingdebuff",
    "Subclass": "Stackingdebuff"
  },
  "SummonBuffModifier": {
    "Class": "Base",
    "Subclass": "Summonbuff"
  },
  "SummonHealth": {
    "Scale": {
      "Type": "power_increase",
      "Value": 2.0
    },
    "Value": 20
  },
  "SummonModifier": {
    "Class": "BarrierTracker",
    "Subclass": "Barriertracker"
  },
  "TargetDashRadius": 13,
  "TargetSearchDelayMax": 0.88,
  "TargetSearchDelayMin": 0.65,
  "TargetSearchRadius": 5,
  "TechArmorDamageReductionPerStack": -0.5,
  "TickRate": 0.3,
  "Upgrades": [
    {
      "SlowPercentPerStack": 0.5
    },
    {
      "MaxStacks": 20
    },
    {
      "StacksToProcSkull": 10
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_wrecking_ball" title="Wrecking Ball" -->

## Wrecking Ball

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_wrecking_ball`
- Snapshot ID: `39589`
- Source-Dokument: `7070`
- Kurzinfo: Wrecking Ball aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Wrecking Ball`
- Payload Hash: `65cfb5a1b40272ff71b0de702c43f640759d7dc43411f98e05547c5cf53fb1bd`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.710221+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1.4,
  "AbilityCastRange": 50,
  "AbilityCooldown": 31,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "AutoThrowModifier": {
    "Class": "WreckingBallAutoThrow",
    "Subclass": "WreckingBallAutoThrow"
  },
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDontTriggerSpellBlock",
    "BehaviorProjectileFiredAsBullet"
  ],
  "ChannelMoveSpeed": 1.3,
  "IsDisabled": false,
  "Key": "ability_wrecking_ball",
  "MinSpeed": 80,
  "MoveSpeedLimit": 6,
  "Name": "Wrecking Ball",
  "StunDuration": 1,
  "TechCleaveExpireTime": 0.4,
  "Upgrades": [
    {
      "AbilityCooldown": -7.5
    },
    {
      "WreckingBallDamage": 80
    },
    {
      "MoveSpeedLimit": 4
    }
  ],
  "WreckingBallDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.302
    },
    "Value": 150
  },
  "WreckingBallPushForce": 1500,
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_wrecker_bouldergrenade" title="Wrecking Ball" -->

## Wrecking Ball

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_wrecker_bouldergrenade`
- Snapshot ID: `39666`
- Source-Dokument: `7070`
- Kurzinfo: Wrecking Ball aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Wrecking Ball`
- Payload Hash: `ba673d82f0b8ec267a2b1f1c61a89b41b6919a311b3d1c9d7806a6ab635c6964`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.900247+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": 50,
  "AbilityChannelTime": 1.2,
  "AbilityCooldown": 31,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": 2.5,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.302
    },
    "Value": 150
  },
  "ExplosionDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.744
    },
    "Value": 80
  },
  "ExplosionPushForce": 1200,
  "IsDisabled": false,
  "Key": "citadel_ability_wrecker_bouldergrenade",
  "Name": "Wrecking Ball",
  "Radius": 7,
  "StunDuration": 1,
  "Upgrades": [
    {
      "AbilityCooldown": -7.5
    },
    {
      "Damage": 50
    },
    {
      "StunDuration": 0.5
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
    "card_name": "Wrecking Ball",
    "hero_key": "hero_wrecker",
    "hero_name": "Wrecker",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_wrecker",
      "hero_name": "Wrecker",
      "lookup": "wrecking ball",
      "name": "Wrecking Ball",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_bomber_ability02" title="ability_bomber_ability02" -->

## ability_bomber_ability02

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_bomber_ability02`
- Snapshot ID: `39404`
- Source-Dokument: `7070`
- Kurzinfo: ability_bomber_ability02 aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_bomber_ability02`
- Payload Hash: `5341bddfb36131673011b363ad231f8931b030e7110bd1e9adc5efa23ba17289`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.241116+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_bomber_ability02",
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_bomber_ability02)",
    "hero_key": "hero_bomber",
    "hero_name": "Bomber",
    "slot": "2"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_bomber_ability03" title="ability_bomber_ability03" -->

## ability_bomber_ability03

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_bomber_ability03`
- Snapshot ID: `39405`
- Source-Dokument: `7070`
- Kurzinfo: ability_bomber_ability03 aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_bomber_ability03`
- Payload Hash: `a234d26e01a6632675cd8868d33d275846e026309fc24eee4aef61df43fd5254`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.242834+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_bomber_ability03",
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_bomber_ability03)",
    "hero_key": "hero_bomber",
    "hero_name": "Bomber",
    "slot": "3"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_bomber_ult" title="ability_bomber_ult" -->

## ability_bomber_ult

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_bomber_ult`
- Snapshot ID: `39406`
- Source-Dokument: `7070`
- Kurzinfo: ability_bomber_ult aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_bomber_ult`
- Payload Hash: `3d3034e3282e02ba93de019e61283d78b0c64a71020dac257fe3903f769ae2ea`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.244985+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 127.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_bomber_ult",
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_bomber_ult)",
    "hero_key": "hero_bomber",
    "hero_name": "Bomber",
    "slot": "4"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_charged_bomb" title="ability_charged_bomb" -->

## ability_charged_bomb

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_charged_bomb`
- Snapshot ID: `39414`
- Source-Dokument: `7070`
- Kurzinfo: ability_charged_bomb aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_charged_bomb`
- Payload Hash: `23e82c6ec33a7c987cd09b4fca962f5dd14b1fa99a2de661698debfbd32284ef`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.264221+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 10.5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BlastJumpVelocity": 25,
  "BlastJumpVelocityCrouch": 30,
  "BlastJumpVelocityGround": 20,
  "ChannelMoveSpeed": -1,
  "ChargeBombModifier": {
    "Class": "ChargedBomb",
    "Subclass": "ChargedBomb"
  },
  "IsDisabled": false,
  "Key": "ability_charged_bomb",
  "MaxChargeTime": 2.0,
  "MaxDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.16064
    },
    "Value": 100
  },
  "Name": null,
  "Radius": 7,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_charged_bomb)",
    "hero_key": "hero_bomber",
    "hero_name": "Bomber",
    "slot": "1"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_fortuna_ability01" title="ability_fortuna_ability01" -->

## ability_fortuna_ability01

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_fortuna_ability01`
- Snapshot ID: `39439`
- Source-Dokument: `7070`
- Kurzinfo: ability_fortuna_ability01 aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_fortuna_ability01`
- Payload Hash: `f397b500520cc865fcde37913a24af6c71daaaa2dea8e5f76003e009c9b46d80`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.327876+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_fortuna_ability01",
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_fortuna_ability01)",
    "hero_key": "hero_fortuna",
    "hero_name": "Fortuna",
    "slot": "1"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_fortuna_ability02" title="ability_fortuna_ability02" -->

## ability_fortuna_ability02

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_fortuna_ability02`
- Snapshot ID: `39440`
- Source-Dokument: `7070`
- Kurzinfo: ability_fortuna_ability02 aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_fortuna_ability02`
- Payload Hash: `7da455dfec2845c17f6e6b453279a23338b8898b2a4f870c189bb784f59ff40d`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.330068+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_fortuna_ability02",
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_fortuna_ability02)",
    "hero_key": "hero_fortuna",
    "hero_name": "Fortuna",
    "slot": "2"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_fortuna_ability03" title="ability_fortuna_ability03" -->

## ability_fortuna_ability03

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_fortuna_ability03`
- Snapshot ID: `39441`
- Source-Dokument: `7070`
- Kurzinfo: ability_fortuna_ability03 aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_fortuna_ability03`
- Payload Hash: `ae5396da4087c331270a7bc1a2de60334cbf2600cd73bc3a5d627594a2b3b1f0`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.332455+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_fortuna_ability03",
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_fortuna_ability03)",
    "hero_key": "hero_fortuna",
    "hero_name": "Fortuna",
    "slot": "3"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_fortuna_ult" title="ability_fortuna_ult" -->

## ability_fortuna_ult

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_fortuna_ult`
- Snapshot ID: `39442`
- Source-Dokument: `7070`
- Kurzinfo: ability_fortuna_ult aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_fortuna_ult`
- Payload Hash: `929d7ba3e7fa27cd444599bbb659699ffc640a8c7ff8f3a845980a06e22d2c80`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.334936+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 127.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_fortuna_ult",
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_fortuna_ult)",
    "hero_key": "hero_fortuna",
    "hero_name": "Fortuna",
    "slot": "4"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_graf_ability01" title="ability_graf_ability01" -->

## ability_graf_ability01

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_graf_ability01`
- Snapshot ID: `39448`
- Source-Dokument: `7070`
- Kurzinfo: ability_graf_ability01 aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_graf_ability01`
- Payload Hash: `b09d668ca30339c82ea2ea4e61fef1fc3bb995c842a40b3cb81a0602a53d592b`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.350758+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_graf_ability01",
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_graf_ability01)",
    "hero_key": "hero_graf",
    "hero_name": "Graf",
    "slot": "1"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_graf_ability02" title="ability_graf_ability02" -->

## ability_graf_ability02

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_graf_ability02`
- Snapshot ID: `39449`
- Source-Dokument: `7070`
- Kurzinfo: ability_graf_ability02 aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_graf_ability02`
- Payload Hash: `2f0ab5618e7f85d490417b409ffe3fe6eb21fd1b11895f9e73a722f39acc5228`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.353019+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_graf_ability02",
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_graf_ability02)",
    "hero_key": "hero_graf",
    "hero_name": "Graf",
    "slot": "2"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_graf_ability03" title="ability_graf_ability03" -->

## ability_graf_ability03

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_graf_ability03`
- Snapshot ID: `39450`
- Source-Dokument: `7070`
- Kurzinfo: ability_graf_ability03 aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_graf_ability03`
- Payload Hash: `0b6127c2157fd985e8000410007f8ece23695c153c19bc8f4bc22f290d845b3a`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.354861+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_graf_ability03",
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_graf_ability03)",
    "hero_key": "hero_graf",
    "hero_name": "Graf",
    "slot": "3"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_graf_ult" title="ability_graf_ult" -->

## ability_graf_ult

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_graf_ult`
- Snapshot ID: `39451`
- Source-Dokument: `7070`
- Kurzinfo: ability_graf_ult aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_graf_ult`
- Payload Hash: `f94aa9d8f37a09cd331dc250b55179a8e6fd4c558de7236bc0ce94977e8f0e30`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.356811+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 127.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_graf_ult",
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_graf_ult)",
    "hero_key": "hero_graf",
    "hero_name": "Graf",
    "slot": "4"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_haunt" title="ability_haunt" -->

## ability_haunt

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_haunt`
- Snapshot ID: `39458`
- Source-Dokument: `7070`
- Kurzinfo: ability_haunt aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_haunt`
- Payload Hash: `fbb46915898f2f2e286032838bbdcd3cb376daf210db5fda3206e56e86332584`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.377291+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "AfterburnDotModifier": {
        "Class": "AfterburnDot",
        "StatusEffectPriority": 50,
        "Subclass": "AfterburnDot"
      },
      "BuildUpModifier": {
        "Class": "CitadelBaseBuildup",
        "Subclass": "CitadelBaseBuildup"
      },
      "Class": "AfterburnWatcher",
      "Subclass": "AfterburnWatcher"
    }
  ],
  "BehaviourBits": [
    "BehaviorCleaveDisabled",
    "BehaviorDisplaysDamageImpact"
  ],
  "BuildUpBulletPercentPerHit": 8.33,
  "BuildUpDuration": 0.1,
  "BurnDuration": 0.3,
  "ChannelMoveSpeed": -1,
  "CritBuildup": 16,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.465
    },
    "Value": 15
  },
  "IsDisabled": false,
  "Key": "ability_haunt",
  "Name": null,
  "TickRate": 0.5,
  "Upgrades": [
    {
      "AfterburnSpiritDamageReduction": -30
    },
    {
      "BurnDuration": 1
    },
    {
      "DPS": 30
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
    "card_name": "Unknown(ability_haunt)",
    "hero_key": "hero_vandal",
    "hero_name": "Vandal",
    "slot": "3"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_shieldguy_ability01" title="ability_shieldguy_ability01" -->

## ability_shieldguy_ability01

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_shieldguy_ability01`
- Snapshot ID: `39526`
- Source-Dokument: `7070`
- Kurzinfo: ability_shieldguy_ability01 aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_shieldguy_ability01`
- Payload Hash: `d4fdb3dece1b2c1cdf78b3c2f6677141c8172f9720c559f4765cc8cb27c9689b`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.548659+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_shieldguy_ability01",
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_shieldguy_ability01)",
    "hero_key": "hero_shieldguy",
    "hero_name": "Shield Guy",
    "slot": "1"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_shieldguy_ability02" title="ability_shieldguy_ability02" -->

## ability_shieldguy_ability02

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_shieldguy_ability02`
- Snapshot ID: `39527`
- Source-Dokument: `7070`
- Kurzinfo: ability_shieldguy_ability02 aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_shieldguy_ability02`
- Payload Hash: `45544d2deeb9f7fd0f102806cc7ee37a5ac8da0866e658ae8da9197364f34b01`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.550640+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_shieldguy_ability02",
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_shieldguy_ability02)",
    "hero_key": "hero_shieldguy",
    "hero_name": "Shield Guy",
    "slot": "2"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_shieldguy_ability03" title="ability_shieldguy_ability03" -->

## ability_shieldguy_ability03

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_shieldguy_ability03`
- Snapshot ID: `39528`
- Source-Dokument: `7070`
- Kurzinfo: ability_shieldguy_ability03 aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_shieldguy_ability03`
- Payload Hash: `c1b3e24597b375dd2bc797ed5b829671050e0daee5b45eec18b390794e22cbe8`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.552901+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_shieldguy_ability03",
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_shieldguy_ability03)",
    "hero_key": "hero_shieldguy",
    "hero_name": "Shield Guy",
    "slot": "3"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_shieldguy_ult" title="ability_shieldguy_ult" -->

## ability_shieldguy_ult

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_shieldguy_ult`
- Snapshot ID: `39529`
- Source-Dokument: `7070`
- Kurzinfo: ability_shieldguy_ult aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_shieldguy_ult`
- Payload Hash: `8eea0d4ca20469a6dbed42c2f5f6b04004265fe339790262c4a01a097263ac67`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.554887+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 127.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_shieldguy_ult",
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_shieldguy_ult)",
    "hero_key": "hero_shieldguy",
    "hero_name": "Shield Guy",
    "slot": "4"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_skyrunner_magic_beam" title="ability_skyrunner_magic_beam" -->

## ability_skyrunner_magic_beam

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_skyrunner_magic_beam`
- Snapshot ID: `39531`
- Source-Dokument: `7070`
- Kurzinfo: ability_skyrunner_magic_beam aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_skyrunner_magic_beam`
- Payload Hash: `71fbba8840ccdac6612e5ae3ebbdfdcd99e384590dce9df7c98c1d6ce813186d`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.559505+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 30,
  "AbilityCharges": 1,
  "AbilityCooldown": 12,
  "AbilityCooldownBetweenCharge": 2,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanSetQuickCast"
  ],
  "BlockerScaleFactor": 4,
  "ChannelMoveSpeed": -1,
  "Damage": 120,
  "GrowTime": 0.2,
  "IsDisabled": false,
  "Key": "ability_skyrunner_magic_beam",
  "MagicBeamModifier": {
    "Class": "MagicBeam",
    "Subclass": "Beammodifier"
  },
  "Name": null,
  "Radius": 4,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_skyrunner_magic_beam)",
    "hero_key": "hero_skyrunner",
    "hero_name": "Skyrunner",
    "slot": "2"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_swan_featherboomerang" title="ability_swan_featherboomerang" -->

## ability_swan_featherboomerang

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_swan_featherboomerang`
- Snapshot ID: `39538`
- Source-Dokument: `7070`
- Kurzinfo: ability_swan_featherboomerang aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_swan_featherboomerang`
- Payload Hash: `e2831cb05f599eb45449af52d0d52b3e8b8a72a7ce64639bbfee7f10ca83955f`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.577986+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.05,
  "AbilityCooldown": 8,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusDamage": 170,
  "ChannelMoveSpeed": -1,
  "Damage": 30,
  "IsDisabled": false,
  "Key": "ability_swan_featherboomerang",
  "Name": null,
  "ProjectileArrivalTime": 1,
  "ProjectileForwardSpeed": 800,
  "ProjectileSideFrequency": 2,
  "ProjectileSideSpeed": 300,
  "Radius": 4,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_swan_featherboomerang)",
    "hero_key": "hero_swan",
    "hero_name": "Swan",
    "slot": "1"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_swan_ult" title="ability_swan_ult" -->

## ability_swan_ult

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_swan_ult`
- Snapshot ID: `39540`
- Source-Dokument: `7070`
- Kurzinfo: ability_swan_ult aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_swan_ult`
- Payload Hash: `b0e1ab400092daf1f81a002f42463396d2bf902214310a9628e0e79d65836ebe`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.583623+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 127.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_swan_ult",
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_swan_ult)",
    "hero_key": "hero_swan",
    "hero_name": "Swan",
    "slot": "4"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="ability_vandal_pillar" title="ability_vandal_pillar" -->

## ability_vandal_pillar

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_vandal_pillar`
- Snapshot ID: `39561`
- Source-Dokument: `7070`
- Kurzinfo: ability_vandal_pillar aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_vandal_pillar`
- Payload Hash: `dd8e147c706198f458183e5ee8cdc6352d1d761180b364117babae353e9ef2c5`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.639115+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 20,
  "AbilityCooldown": 60,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.15,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0695
    },
    "Value": 200
  },
  "HalfHeight": 6,
  "IsDisabled": false,
  "Key": "ability_vandal_pillar",
  "Name": null,
  "PetrifyDamageBreakThreshold": 200,
  "PetrifyDuration": 3,
  "PetrifyModifier": {
    "Class": "CitadelPetrify",
    "Subclass": "ViperUltPetrify"
  },
  "PreDetonateDuration": 0.6,
  "Radius": 4,
  "Upgrades": [
    {
      "AbilityCooldown": -15
    },
    {
      "PetrifyDuration": 1.5
    },
    {
      "Radius": 3
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
    "card_name": "Unknown(ability_vandal_pillar)",
    "hero_key": "hero_vandal",
    "hero_name": "Vandal",
    "slot": "2"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_vandal_overflow" title="citadel_ability_vandal_overflow" -->

## citadel_ability_vandal_overflow

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_vandal_overflow`
- Snapshot ID: `39662`
- Source-Dokument: `7070`
- Kurzinfo: citadel_ability_vandal_overflow aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `citadel_ability_vandal_overflow`
- Payload Hash: `0275e1e816662a9171d1d223d1d39d553eae256019775f7d0328510f8ff89f53`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.892129+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.6,
  "AbilityCastRange": 20,
  "AbilityCooldown": 16,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 1.25,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectilePassThroughWorld",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.974938
    },
    "Value": 100
  },
  "DampingFactor": 0.5,
  "IsDisabled": false,
  "Key": "citadel_ability_vandal_overflow",
  "LiftHeight": 120,
  "LiftModifier": {
    "Class": "CitadelVandaloverflow",
    "EnabledStateMask": [
      "Stunned"
    ],
    "Subclass": "Lift"
  },
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": -28.0
    },
    {
      "AbilityDuration": 0.5
    },
    {
      "AbilityUnitTargetLimit": 5
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
    "card_name": "Unknown(citadel_ability_vandal_overflow)",
    "hero_key": "hero_vandal",
    "hero_name": "Vandal",
    "slot": "4"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="citadel_ability_vandal_surge" title="citadel_ability_vandal_surge" -->

## citadel_ability_vandal_surge

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_vandal_surge`
- Snapshot ID: `39663`
- Source-Dokument: `7070`
- Kurzinfo: citadel_ability_vandal_surge aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `citadel_ability_vandal_surge`
- Payload Hash: `15dbed6ff28bb378ffc83d176988be7fb2de282c4561237f753a73fd98166fd3`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.893949+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.6,
  "AbilityCastRange": 20,
  "AbilityCooldown": 16,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 1.25,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectilePassThroughWorld",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.974938
    },
    "Value": 100
  },
  "DampingFactor": 0.5,
  "IsDisabled": false,
  "Key": "citadel_ability_vandal_surge",
  "LiftHeight": 120,
  "LiftModifier": {
    "Class": "CitadelVandalsurge",
    "EnabledStateMask": [
      "Stunned"
    ],
    "Subclass": "Lift"
  },
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": -28.0
    },
    {
      "AbilityDuration": 0.5
    },
    {
      "AbilityUnitTargetLimit": 5
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
    "card_name": "Unknown(citadel_ability_vandal_surge)",
    "hero_key": "hero_vandal",
    "hero_name": "Vandal",
    "slot": "1"
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability" external_id="gunslinger_knockbackblast" title="gunslinger_knockbackblast" -->

## gunslinger_knockbackblast

### Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `gunslinger_knockbackblast`
- Snapshot ID: `39685`
- Source-Dokument: `7070`
- Kurzinfo: gunslinger_knockbackblast aus `deadlock_data` / `ability` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `gunslinger_knockbackblast`
- Payload Hash: `48b39467ecd0e3c44c41049f5c075af7716ba617f197ff2c31a50743ade1614e`
- Source Content Hash: `2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json`
- Fetched At: `2026-07-09T19:36:23.947173+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 8,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "ChannelMoveSpeed": -1,
  "ConeAngle": 45,
  "Damage": 120,
  "DebuffModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "KnockbackSlow"
  },
  "EnemyShoveForce": 800,
  "IsDisabled": false,
  "Key": "gunslinger_knockbackblast",
  "Name": null,
  "SelfShoveForce": 200,
  "SlowDuration": 3,
  "SlowPercent": 30,
  "StunDuration": 1,
  "Upgrades": [],
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
    "card_name": "Unknown(gunslinger_knockbackblast)",
    "hero_key": "hero_gunslinger",
    "hero_name": "Gunslinger",
    "slot": "2"
  }
}
````

