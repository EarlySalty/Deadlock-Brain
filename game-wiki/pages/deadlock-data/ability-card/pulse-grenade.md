---
title: "Pulse Grenade"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_chrono_pulse_grenade"
canonical_name: "Pulse Grenade"
snapshot_id: 39819
source_document_id: 7071
payload_hash: "0044e221cef0b7739db5f3b47d3221498f1f6bb85088900f9f4367b73025a1cd"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.264772+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Pulse Grenade

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chrono_pulse_grenade`
- Snapshot ID: `39819`
- Source-Dokument: `7071`
- Kurzinfo: Pulse Grenade aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 32.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 3.2
  },
  "DescKey": "citadel_ability_chrono_pulse_grenade_desc",
  "HeroKey": "hero_gunslinger",
  "HeroName": "Gunslinger",
  "Info1": {
    "Alt": [
      {
        "Key": "RadiusIncreasePerPulse",
        "Name": "Radius Per Pulse",
        "Type": "distance",
        "Value": 1
      },
      {
        "Key": "PulseInterval",
        "Name": "Pulse Interval",
        "Type": "duration",
        "Value": 0.8
      },
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 8.0
      },
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 20
      },
      {
        "Key": "MovementSlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 0.2
      }
    ],
    "DescKey": "citadel_ability_chrono_pulse_grenade_desc",
    "Main": {
      "Props": [
        {
          "Key": "PulseDamage",
          "Name": "Pulse Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.3
          },
          "Type": "tech_damage",
          "Value": 35
        },
        {
          "Key": "DamageAmplificationPerStack",
          "Name": "Bonus Damage per Stack",
          "Type": "damage",
          "Value": 4
        }
      ]
    }
  },
  "Key": "citadel_ability_chrono_pulse_grenade",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Pulse Grenade",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 5.5
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -12
    },
    {
      "DescKey": "citadel_ability_chrono_pulse_grenade_t2_desc",
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
      "DamageAmplificationPerStack": 4,
      "DescKey": "citadel_ability_chrono_pulse_grenade_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_chrono",
      "hero_name": "Paradox",
      "lookup": "pulse grenade",
      "name": "Pulse Grenade",
      "type": "ability"
    }
  ]
}
````
