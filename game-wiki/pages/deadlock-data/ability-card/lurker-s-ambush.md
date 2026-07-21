---
title: "Lurker's Ambush"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "fathom_lurkers_ambush"
canonical_name: "Lurker's Ambush"
snapshot_id: 39894
source_document_id: 7071
payload_hash: "6f2ebce9967c1da67b64639db3d1fb99c644c67e39e1bb5061417721ffa1b8e1"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.411601+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Lurker's Ambush

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `fathom_lurkers_ambush`
- Snapshot ID: `39894`
- Source-Dokument: `7071`
- Kurzinfo: Lurker's Ambush aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 30
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 9999
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 50.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Duration": {
    "NonLatchedDurationPct": {
      "Name": null,
      "Type": "duration",
      "Value": 50
    }
  },
  "HeroKey": "hero_slork",
  "HeroName": "Fathom",
  "Info1": {
    "Alt": [],
    "DescKey": "fathom_lurkers_ambush_passive_desc",
    "Main": {
      "Props": [
        {
          "Key": "NotSeenByEnemiesRegen",
          "Name": "Max Health Regen",
          "Type": "healing",
          "Value": 3
        },
        {
          "Key": "InvisFadeToDuration",
          "Name": "Fade Time",
          "Value": 1.5
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "fathom_lurkers_ambush_active_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.651
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "EnemySlowPct",
          "Name": "Enemy Move Speed",
          "Type": "slow",
          "Value": 60
        },
        {
          "Key": "DebuffMaxDuration",
          "Name": "Max Slow Duration",
          "Type": "duration",
          "Value": 3.5
        }
      ]
    }
  },
  "Key": "fathom_lurkers_ambush",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 5
    }
  },
  "Name": "Lurker's Ambush",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ChannelTimeForMaxDebuff": {
      "Name": null,
      "Value": 1.5
    },
    "DebuffMinDuration": {
      "Name": null,
      "Value": 1.0
    },
    "InitialHeight": {
      "Name": null,
      "Value": 350
    },
    "RevealOnDamageDuration": {
      "Name": null,
      "Value": 0.5
    },
    "RevealOnSpottedDuration": {
      "Name": null,
      "Value": 3
    },
    "StandStillMinTime": {
      "Name": null,
      "Value": 0.5
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Range": {
    "SpottedRadius": {
      "Name": "Spot Radius",
      "Type": "distance",
      "Value": 999
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -15
    },
    {
      "DebuffMaxDuration": 1
    },
    {
      "NotSeenByEnemiesRegen": 2
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
      "hero_key": "hero_slork",
      "hero_name": "Fathom",
      "lookup": "lurker's ambush",
      "name": "Lurker's Ambush",
      "type": "ability"
    }
  ]
}
````
