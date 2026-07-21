---
title: "Paradoxical Swap"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_chrono_swap"
canonical_name: "Paradoxical Swap"
snapshot_id: 39774
source_document_id: 7071
payload_hash: "9427adcf561265e11b8773382fc265348118f59e27f03b7e7aa0fecb1ac49e8f"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.178805+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Paradoxical Swap

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chrono_swap`
- Snapshot ID: `39774`
- Source-Dokument: `7071`
- Kurzinfo: Paradoxical Swap aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 25
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 110.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_chrono_swap_desc",
  "HeroKey": "hero_chrono",
  "HeroName": "Paradox",
  "Info1": {
    "Alt": [
      {
        "Key": "CombatBarrier",
        "Name": "Barrier",
        "Scale": {
          "Type": "spirit",
          "Value": 0.0
        },
        "Type": "bullet_armor_up",
        "Value": 0
      },
      {
        "Key": "BarrierDuration",
        "Name": "Barrier Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "MultiSwap",
        "Name": "Multi Target Radius",
        "Type": "distance",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_chrono_swap_desc",
    "Main": {
      "Props": [
        {
          "Key": "SwapDamage",
          "Name": "Swap Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.1
          },
          "Type": "tech_damage",
          "Value": 150.0
        },
        {
          "Key": "MaxHealthDamage",
          "Name": "Max Health Damage",
          "Type": "tech_damage",
          "Value": 0.0
        }
      ]
    }
  },
  "Key": "citadel_ability_chrono_swap",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Paradoxical Swap",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DistanceToMaxTime": {
      "Name": null,
      "Value": 30
    },
    "InitialFreezeTime": {
      "Name": null,
      "Value": 0.25
    },
    "InitialHeight": {
      "Name": null,
      "Value": 350
    },
    "MinSwapTime": {
      "Name": null,
      "Value": 0.6
    },
    "SwapTime": {
      "Name": null,
      "Value": 1.0
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "BarrierDuration": 8,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.5
        },
        "Value": 200
      },
      "DescKey": "citadel_ability_chrono_swap_t1_desc"
    },
    {
      "AbilityCastRange": 13,
      "AbilityCooldown": -30,
      "DescKey": "citadel_ability_chrono_swap_t2_desc"
    },
    {
      "DescKey": "citadel_ability_chrono_swap_t3_desc",
      "MaxHealthDamage": 10,
      "MultiSwap": 7
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
      "lookup": "paradoxical swap",
      "name": "Paradoxical Swap",
      "type": "ability"
    }
  ]
}
````
