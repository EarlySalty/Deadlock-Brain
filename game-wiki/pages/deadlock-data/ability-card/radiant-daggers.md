---
title: "Radiant Daggers"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_unicorn_luminousstrike"
canonical_name: "Radiant Daggers"
snapshot_id: 39913
source_document_id: 7071
payload_hash: "e96919b0a17adadf5eb37f926bd082b7d2be2fa28e218095c41b55c4a4270e44"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.447779+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Radiant Daggers

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_unicorn_luminousstrike`
- Snapshot ID: `39913`
- Source-Dokument: `7071`
- Kurzinfo: Radiant Daggers aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 30
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 33
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 2
  },
  "DescKey": "ability_unicorn_luminousstrike_desc",
  "HeroKey": "hero_unicorn",
  "HeroName": "Celeste",
  "Info1": {
    "Alt": [
      {
        "Key": "ExplosionRadius",
        "Name": "Explosion Radius",
        "Type": "distance",
        "Value": 8
      },
      {
        "Key": "PreExplosionDuration",
        "Type": "duration",
        "Value": 1.4
      },
      {
        "Key": "BuffMaxStacks",
        "Type": "cast",
        "Value": 6
      },
      {
        "Key": "BuffDuration",
        "Name": "Buff Duration",
        "Type": "duration",
        "Value": 30
      }
    ],
    "DescKey": "ability_unicorn_luminousstrike_desc",
    "Main": {
      "Props": [
        {
          "Key": "ImpactDamage",
          "Name": "Impact Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.63
          },
          "Type": "tech_damage",
          "Value": 55
        },
        {
          "Key": "MagicIncreasePerStack",
          "Name": "Spirit Amp per Stack",
          "Type": "tech_armor_down",
          "Value": 8
        },
        {
          "Key": "FireRatePerStack",
          "Name": "Fire Rate per Stack",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_unicorn_luminousstrike",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Radiant Daggers",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BuffDelay": {
      "Name": null,
      "Value": 0.75
    },
    "ClimbHeight": {
      "Name": null,
      "Value": 50.0
    },
    "ExplosionInterval": {
      "Name": "Beam Interval",
      "Value": 0.7
    },
    "PostExplosionDuration": {
      "Name": null,
      "Value": 0.8
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCharges": 2
    },
    {
      "AbilityCooldown": -22,
      "DescKey": "ability_unicorn_luminousstrike_t2_desc",
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
      "hero_key": "hero_unicorn",
      "hero_name": "Celeste",
      "lookup": "radiant daggers",
      "name": "Radiant Daggers",
      "type": "ability"
    }
  ]
}
````
