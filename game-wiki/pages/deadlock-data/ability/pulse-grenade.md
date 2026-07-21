---
title: "Pulse Grenade"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_chrono_pulse_grenade"
canonical_name: "Pulse Grenade"
snapshot_id: 39604
source_document_id: 7070
payload_hash: "2682a7d70ee5e2d533269704f4df5b1424490acc2fe305c7d016becb74a0352d"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.753393+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Pulse Grenade

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chrono_pulse_grenade`
- Snapshot ID: `39604`
- Source-Dokument: `7070`
- Kurzinfo: Pulse Grenade aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

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
