---
title: "Affliction"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "synth_affliction"
canonical_name: "Affliction"
snapshot_id: 39902
source_document_id: 7071
payload_hash: "d196d65e09c654ce779488fdcc853792aae12319913030339832ad16565be1f8"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.426318+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Affliction

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `synth_affliction`
- Snapshot ID: `39902`
- Source-Dokument: `7071`
- Kurzinfo: Affliction aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.6
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 170.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "synth_affliction_desc",
  "HeroKey": "hero_synth",
  "HeroName": "Pocket",
  "Info1": {
    "Alt": [
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "Name": "Healing Reduction",
        "Value": 0
      }
    ],
    "DescKey": "synth_affliction_desc",
    "Main": {
      "Props": [
        {
          "Key": "CurrentHealthDamage",
          "Name": "Current Health Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Type": "tech_damage",
          "Value": 0
        },
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.22
          },
          "Type": "tech_damage",
          "Value": 34
        },
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "Type": "duration",
          "Value": 11
        }
      ]
    }
  },
  "Key": "synth_affliction",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Affliction",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CanBePurged": {
      "Name": null,
      "Value": 1
    },
    "DamageInterval": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 9
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -35
    },
    {
      "DebuffDuration": 3,
      "DescKey": "synth_affliction_t2_desc",
      "Radius": 4
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.11
        },
        "Value": 14
      },
      "DescKey": "synth_affliction_t3_desc",
      "DisableHealing": 1
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
      "hero_key": "hero_synth",
      "hero_name": "Pocket",
      "lookup": "affliction",
      "name": "Affliction",
      "type": "ability"
    }
  ]
}
````
