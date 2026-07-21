---
title: "Frost Grenade"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_ice_grenade"
canonical_name: "Frost Grenade"
snapshot_id: 39462
source_document_id: 7070
payload_hash: "c77e9479b53e3e672fdc057772d8933661c561ff2625e4c207ecae2355dc8dd3"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.386324+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Frost Grenade

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_ice_grenade`
- Snapshot ID: `39462`
- Source-Dokument: `7070`
- Kurzinfo: Frost Grenade aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

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
