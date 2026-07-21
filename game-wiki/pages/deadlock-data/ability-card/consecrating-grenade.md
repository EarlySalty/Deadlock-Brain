---
title: "Consecrating Grenade"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_priest_flashbang"
canonical_name: "Consecrating Grenade"
snapshot_id: 39871
source_document_id: 7071
payload_hash: "b566d0642d8867294682a5d73e406ebe27853274e75a028d4b86e4b5acd98c48"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.366730+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Consecrating Grenade

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_priest_flashbang`
- Snapshot ID: `39871`
- Source-Dokument: `7071`
- Kurzinfo: Consecrating Grenade aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.03
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 25
  },
  "Damage": {
    "HealAmpRegenPenaltyPercent": {
      "Name": "Healing Reduction",
      "Type": "damage",
      "Value": -30
    }
  },
  "DescKey": "ability_priest_flashbang_desc",
  "HeroKey": "hero_priest",
  "HeroName": "Venator",
  "Info1": {
    "Alt": [
      {
        "Key": "BurnDuration",
        "Name": "Burn Duration",
        "Type": "duration",
        "Value": 3.5
      },
      {
        "Key": "BurnRadius",
        "Name": "Burn Radius",
        "Type": "distance",
        "Value": 4.5
      }
    ],
    "DescKey": "ability_priest_flashbang_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "weapon_damage_increase",
            "Value": 1.0
          },
          "Type": "bullet_damage",
          "Value": 35
        },
        {
          "Key": "HealAmpReceivePenaltyPercent",
          "Name": "Healing Reduction",
          "Type": "damage",
          "Value": -30
        },
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "power_increase",
            "Value": 1.6
          },
          "Title": "On Hit:",
          "Type": "damage",
          "Value": 10
        },
        {
          "Key": "DPSPercentHealth",
          "Name": "Max Health as Damage",
          "Title": "On Hit:",
          "Value": 0
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hit:",
          "Value": 0
        },
        {
          "Key": "FireRateSlow",
          "Name": "Fire Rate",
          "Title": "On Hit:",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_priest_flashbang",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Consecrating Grenade",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.15
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BounceGrenadeSpeed": {
      "Name": null,
      "Value": 1100
    },
    "BounceLifetime": {
      "Name": null,
      "Value": 0.5
    },
    "BurnLingerDuration": {
      "Name": null,
      "Value": 0.15
    },
    "CameraTurnRateMax": {
      "Name": null,
      "Value": 15
    },
    "PreBounceLifetime": {
      "Name": null,
      "Value": 15
    },
    "TickRate": {
      "Name": null,
      "Value": 0.2
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 4.5
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -5
    },
    {
      "BurnDuration": 1.0,
      "BurnRadius": 1.5,
      "DescKey": "ability_priest_flashbang_t2_desc",
      "Radius": 1.5
    },
    {
      "AbilityCharges": 1,
      "AbilityCooldownBetweenCharge": 3,
      "DescKey": "ability_priest_flashbang_t3_desc",
      "HealAmpReceivePenaltyPercent": -20,
      "HealAmpRegenPenaltyPercent": -20
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
      "hero_key": "hero_priest",
      "hero_name": "Venator",
      "lookup": "consecrating grenade",
      "name": "Consecrating Grenade",
      "type": "ability"
    }
  ]
}
````
