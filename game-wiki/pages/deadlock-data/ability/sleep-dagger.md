---
title: "Sleep Dagger"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_sleep_dagger"
canonical_name: "Sleep Dagger"
snapshot_id: 39534
source_document_id: 7070
payload_hash: "06e672b151f12658066328c1267e962f6d71863a3bf2786b0e5da391c3065876"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.565266+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Sleep Dagger

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_sleep_dagger`
- Snapshot ID: `39534`
- Source-Dokument: `7070`
- Kurzinfo: Sleep Dagger aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

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
