---
title: "Pain Battery"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_frank_shocktarget2"
canonical_name: "Pain Battery"
snapshot_id: 39803
source_document_id: 7071
payload_hash: "b316de43cc4b7efc2beedfe064034f4710ccf403b25b7c0c65c0da6611fbe790"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.234923+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Pain Battery

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_frank_shocktarget2`
- Snapshot ID: `39803`
- Source-Dokument: `7071`
- Kurzinfo: Pain Battery aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 28
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 2
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_frank_shocktarget2_desc",
  "Health": {
    "HealOnHit": {
      "Name": null,
      "Scale": {
        "Type": "healing",
        "Value": 1.0
      },
      "Type": "healing",
      "Value": 0
    }
  },
  "HeroKey": "hero_frank",
  "HeroName": "Victor",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_frank_shocktarget2_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.6
          },
          "Type": "tech_damage",
          "Value": 100
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hero Hit:",
          "Type": "slow",
          "Value": 0
        },
        {
          "Key": "MissingHealthPercentHeal",
          "Name": "Missing Health as Healing",
          "Scale": {
            "Type": "healing",
            "Value": 1.0
          },
          "Title": "On Hero Hit:",
          "Type": "healing",
          "Value": 0
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_frank_shocktarget2",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Pain Battery",
  "Other": {
    "AbilityChargesConditionally": {
      "Name": null,
      "Value": 1
    },
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BatteryGenerationPercent": {
      "Name": "Battery Generation",
      "Scale": {
        "Type": "cooldown",
        "Value": -1.0
      },
      "Value": 100
    },
    "BoltCount": {
      "Name": null,
      "Value": 7
    },
    "BonusShocksDelay": {
      "Name": null,
      "Value": 0.2
    },
    "SpreadAngle": {
      "Name": null,
      "Value": 40
    },
    "SpreadRandomness": {
      "Name": null,
      "Value": 0.005
    },
    "StoredDamageHealthPercentRequired": {
      "Name": null,
      "Value": 40
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "DescKey": "ability_frank_shocktarget2_t1_desc",
      "SlowDuration": "2s",
      "SlowPercent": 40
    },
    {
      "Damage": 50.0
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 0
      },
      "DescKey": "ability_frank_shocktarget2_t3_desc",
      "MissingHealthPercentHeal": 15
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
      "lookup": "pain battery",
      "name": "Pain Battery",
      "type": "ability"
    }
  ]
}
````
