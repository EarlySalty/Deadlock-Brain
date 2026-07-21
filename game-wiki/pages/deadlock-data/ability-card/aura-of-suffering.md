---
title: "Aura of Suffering"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_frank_painaura"
canonical_name: "Aura of Suffering"
snapshot_id: 39805
source_document_id: 7071
payload_hash: "66eb744406fa898e7f5a0074bcc736185b8c0f256c12471dba988722ccbf61ee"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.238617+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Aura of Suffering

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_frank_painaura`
- Snapshot ID: `39805`
- Source-Dokument: `7071`
- Kurzinfo: Aura of Suffering aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 2.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 8
  },
  "Damage": {
    "Damage": {
      "Name": "Damage",
      "Scale": {
        "Type": "spirit",
        "Value": 1.1
      },
      "Type": "tech_damage",
      "Value": 0
    }
  },
  "DescKey": "ability_frank_painaura_desc",
  "HeroKey": "hero_frank",
  "HeroName": "Victor",
  "Info1": {
    "Alt": [
      {
        "Key": "SelfDamagePercentage",
        "Name": "Self Damage",
        "Type": "damage",
        "Value": 70
      },
      {
        "Key": "StatusResistancePercent",
        "Name": "Debuff Resist",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_frank_painaura_desc",
    "Main": {
      "Props": [
        {
          "Key": "MinDPS",
          "Name": "Minimum DPS",
          "Scale": {
            "Type": "spirit",
            "Value": 0.15
          },
          "Type": "tech_damage",
          "Value": 13
        },
        {
          "Key": "MaxDPS",
          "Name": "Max DPS",
          "Scale": {
            "Type": "spirit",
            "Value": 0.72
          },
          "Type": "tech_damage",
          "Value": 58
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Enemy Hit:",
          "Type": "slow",
          "Value": 0
        },
        {
          "Key": "OutgoingDamagePercent",
          "Name": "Damage",
          "Title": "On Enemy Hit:",
          "Type": "damage",
          "Value": 0
        },
        {
          "Key": "IncomingDamagePercent",
          "Name": "Damage Taken",
          "Title": "On Enemy Hit:",
          "Type": "health",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_frank_painaura",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Aura of Suffering",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DebuffDuration": {
      "Name": "Debuff Duration",
      "Value": 0.5
    },
    "SelfDPS": {
      "Name": null,
      "Value": 15
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    },
    "ToggleOffDelay": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 8
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DebuffDuration": 0.5,
      "DescKey": "ability_frank_painaura_t1_desc",
      "EnemyDashSlowPercent": -25,
      "SlowPercent": 25
    },
    {
      "DescKey": "ability_frank_painaura_t2_desc",
      "MaxDPS": 34.0,
      "MinDps": 6
    },
    {
      "DescKey": "ability_frank_painaura_t3_desc",
      "IncomingDamagePercent": 15,
      "Radius": 1
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
      "hero_key": "hero_frank",
      "hero_name": "Victor",
      "lookup": "aura of suffering",
      "name": "Aura of Suffering",
      "type": "ability"
    }
  ]
}
````
