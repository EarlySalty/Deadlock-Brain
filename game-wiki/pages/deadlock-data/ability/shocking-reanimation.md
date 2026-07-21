---
title: "Shocking Reanimation"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_frank_revive"
canonical_name: "Shocking Reanimation"
snapshot_id: 39444
source_document_id: 7070
payload_hash: "aefdcb545a5b7a34677978cf278d9dece9a53250fdf917783efc953a2312003e"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.339535+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Shocking Reanimation

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_frank_revive`
- Snapshot ID: `39444`
- Source-Dokument: `7070`
- Kurzinfo: Shocking Reanimation aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

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
