---
title: "Napalm"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_incendiary_projectile"
canonical_name: "Napalm"
snapshot_id: 39831
source_document_id: 7071
payload_hash: "8504105b4a29829b0d3030ef27a9b9b484f902e98c8e7830a702de014233ba44"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.287255+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Napalm

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_incendiary_projectile`
- Snapshot ID: `39831`
- Source-Dokument: `7071`
- Kurzinfo: Napalm aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 20
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 25.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 6
  },
  "DescKey": "ability_incendiary_projectile_desc",
  "HeroKey": "hero_inferno",
  "HeroName": "Infernus",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_incendiary_projectile_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.6
          },
          "Type": "tech_damage",
          "Value": 40.0
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hit:",
          "Type": "slow",
          "Value": 35
        },
        {
          "Key": "SlowDuration",
          "Name": "Slow Duration",
          "Title": "On Hit:",
          "Type": "duration",
          "Value": 4
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "IncomingDamagePercentFromCaster",
          "Name": "Damage Taken",
          "Title": "Napalm Effects:",
          "Type": "damage",
          "Value": 16
        },
        {
          "Key": "LifestealPercentHero",
          "Name": "Lifesteal",
          "Title": "Napalm Effects:",
          "Type": "healing",
          "Value": 0
        },
        {
          "Key": "HealAmpReceivePenaltyPercent",
          "Name": "Healing Reduction",
          "Title": "Napalm Effects:",
          "Type": "healing",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_incendiary_projectile",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 18
    }
  },
  "Name": "Napalm",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "GrowthPerMeter": {
      "Name": null,
      "Value": 0.5
    },
    "HeightOffGround": {
      "Name": null,
      "Value": 50
    },
    "InitialWidth": {
      "Name": null,
      "Value": 1
    },
    "ParticleRadiusMultiplier": {
      "Name": null,
      "Value": 1.15
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "DescKey": "ability_incendiary_projectile_t2_desc",
      "LifestealPercentHero": 15
    },
    {
      "DescKey": "ability_incendiary_projectile_t3_desc",
      "HealAmpReceivePenaltyPercent": -33,
      "HealAmpRegenPenaltyPercent": -33,
      "IncomingDamagePercentFromCaster": 17
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
      "hero_key": "hero_inferno",
      "hero_name": "Infernus",
      "lookup": "napalm",
      "name": "Napalm",
      "type": "ability"
    }
  ]
}
````
