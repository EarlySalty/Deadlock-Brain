---
title: "Shining Wonder"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_unicorn_dazzlingorb"
canonical_name: "Shining Wonder"
snapshot_id: 39551
source_document_id: 7070
payload_hash: "3999c759caa4211ec2542d9867541df86dbefb64b60b92c0a7741e6d1cd4e16b"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.614727+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Shining Wonder

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_unicorn_dazzlingorb`
- Snapshot ID: `39551`
- Source-Dokument: `7070`
- Kurzinfo: Shining Wonder aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

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
