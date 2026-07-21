---
title: "Light Eater"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_unicorn_radiantblast"
canonical_name: "Light Eater"
snapshot_id: 39911
source_document_id: 7071
payload_hash: "6f5290a646fb0ee7f9af1a3ac57291b0b10e3be7722b4c6fdedbeac00eff19bd"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.444003+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Light Eater

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_unicorn_radiantblast`
- Snapshot ID: `39911`
- Source-Dokument: `7071`
- Kurzinfo: Light Eater aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.3
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 10
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 20
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Scale": {
      "Type": "spirit",
      "Value": 0.05
    },
    "Type": "duration",
    "Value": 8
  },
  "DescKey": "ability_unicorn_radiantblast_desc",
  "HeroKey": "hero_unicorn",
  "HeroName": "Celeste",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Name": "Duration",
        "Scale": {
          "Type": "spirit",
          "Value": 0.05
        },
        "Type": "duration",
        "Value": 8
      }
    ],
    "DescKey": "ability_unicorn_radiantblast_desc",
    "Main": {
      "Props": [
        {
          "Key": "AbilityLifestealPercentHero",
          "Name": "Spirit Lifesteal",
          "Type": "healing",
          "Value": 20
        },
        {
          "Key": "FlareDamage",
          "Name": "Flare Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.47
          },
          "Type": "tech_damage",
          "Value": 40
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.34
          },
          "Title": "On Bullet Hit",
          "Type": "tech_damage",
          "Value": 15
        }
      ]
    }
  },
  "Key": "ability_unicorn_radiantblast",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Light Eater",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 100
    },
    "ExtraSweepRadius": {
      "Name": null,
      "Value": 2
    },
    "TargetingConeAngle": {
      "Name": "Attack Angle",
      "Value": 70
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityLifestealPercentHero": 15
    },
    {
      "AbilityCastRange": 3,
      "AbilityCooldown": -10,
      "DescKey": "ability_unicorn_radiantblast_t2_desc"
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.2
        },
        "Value": 25
      },
      "DescKey": "ability_unicorn_radiantblast_t3_desc"
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
      "lookup": "light eater",
      "name": "Light Eater",
      "type": "ability"
    }
  ]
}
````
