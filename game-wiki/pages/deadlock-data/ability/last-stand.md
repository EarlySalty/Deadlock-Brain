---
title: "Last Stand"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_warden_riot_protocol"
canonical_name: "Last Stand"
snapshot_id: 39572
source_document_id: 7070
payload_hash: "31780cc8096af01913633f0ae99c4ea1648b8b801cd537beb33f480fc012f47f"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.670227+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Last Stand

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_warden_riot_protocol`
- Snapshot ID: `39572`
- Source-Dokument: `7070`
- Kurzinfo: Last Stand aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

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
