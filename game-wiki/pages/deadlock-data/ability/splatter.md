---
title: "Splatter"
entity_type: "ability"
source: "deadlock_data"
external_id: "viscous_goo_grenade"
canonical_name: "Splatter"
snapshot_id: 39735
source_document_id: 7070
payload_hash: "5416144abc694ba8e53f26200119c1da12404b3064a60570de4d6b8dbd4ef5a7"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.082530+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Splatter

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `viscous_goo_grenade`
- Snapshot ID: `39735`
- Source-Dokument: `7070`
- Kurzinfo: Splatter aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

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
