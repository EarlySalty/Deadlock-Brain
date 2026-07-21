---
title: "Mauling Leap"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_werewolf_maulingleap"
canonical_name: "Mauling Leap"
snapshot_id: 39940
source_document_id: 7071
payload_hash: "d5a7cc3e5420dfb6564ed9d30897fd84c80e9185442870cfe7f7a9748f128e5d"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.499712+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Mauling Leap

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_maulingleap`
- Snapshot ID: `39940`
- Source-Dokument: `7071`
- Kurzinfo: Mauling Leap aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.35
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 18.1
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.55
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 16
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_werewolf_maulingleap_desc",
  "HeroKey": "hero_werewolf_transformed",
  "HeroName": "Silver (Transformed)",
  "Info1": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 6
      }
    ],
    "DescKey": "ability_werewolf_maulingleap_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "melee",
            "Value": 1.5
          },
          "Type": "tech_damage",
          "Value": 0
        },
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.15
          },
          "Title": "On Hit:",
          "Type": "tech_damage",
          "Value": 15
        },
        {
          "Key": "BulletArmorReduction",
          "Name": "Bullet Resist",
          "Title": "On Hit:",
          "Type": "bullet_armor_down",
          "Value": -8
        }
      ]
    }
  },
  "Key": "ability_werewolf_maulingleap",
  "Name": "Mauling Leap",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.1
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AllowRamMultiple": {
      "Name": null,
      "Value": 1
    },
    "CameraTurnRateMax": {
      "Name": null,
      "Value": 188
    },
    "LeapForwardOffset": {
      "Name": null,
      "Value": 0.3
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    },
    "WorldImpactRadius": {
      "Name": null,
      "Value": 25
    }
  },
  "Range": {
    "LeapMultiHitRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 1.5
    },
    "LeapRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 2.8
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "DPS": 10
    },
    {
      "AbilityCooldown": -9
    },
    {
      "BulletArmorReduction": -12
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
      "hero_key": "hero_werewolf_transformed",
      "hero_name": "Silver (Transformed)",
      "lookup": "mauling leap",
      "name": "Mauling Leap",
      "type": "ability"
    }
  ]
}
````
