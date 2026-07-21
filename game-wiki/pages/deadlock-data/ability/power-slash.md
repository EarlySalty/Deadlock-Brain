---
title: "Power Slash"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_power_slash"
canonical_name: "Power Slash"
snapshot_id: 39633
source_document_id: 7070
payload_hash: "3f8f2b9accb0c9bf95cef26f73c00263f2d86b2a20efcf2cc02f3d25e84a93a5"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.822977+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Power Slash

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_power_slash`
- Snapshot ID: `39633`
- Source-Dokument: `7070`
- Kurzinfo: Power Slash aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

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
