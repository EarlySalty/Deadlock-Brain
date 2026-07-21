---
title: "Crawling Plague"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_trapper_spiderwave"
canonical_name: "Crawling Plague"
snapshot_id: 39910
source_document_id: 7071
payload_hash: "d45de10000b86a32725eac856aa2836f6b52bb67dd2cf9575999a031f6b00c61"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.441627+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Crawling Plague

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_spiderwave`
- Snapshot ID: `39910`
- Source-Dokument: `7071`
- Kurzinfo: Crawling Plague aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 160
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_trapper_spiderwave_desc",
  "HeroKey": "hero_trapper",
  "HeroName": "Trapper",
  "Info1": {
    "Alt": [
      {
        "Key": "SpiderLifetime",
        "Name": "Spider Life Time",
        "Value": 25
      },
      {
        "Key": "SpiderSearchRadius",
        "Name": "Spider Chase Distance",
        "Type": "distance",
        "Value": 2
      },
      {
        "Key": "SpiritStealDuration",
        "Name": "Spirit Debuff Duration",
        "Type": "duration",
        "Value": 10
      },
      {
        "Key": "SpiritReducedPerStack",
        "Name": "Spirit Reduction",
        "Type": "tech_damage",
        "Value": 5
      },
      {
        "Key": "SpiritResReducedPerStack",
        "Name": "Spirit Resist Reduction",
        "Type": "tech_armor_up",
        "Value": 5
      }
    ],
    "DescKey": "ability_trapper_spiderwave_desc",
    "Main": {
      "Props": [
        {
          "Key": "SpiderDamage",
          "Name": "Spider Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.744
          },
          "Type": "tech_damage",
          "Value": 140
        },
        {
          "Key": "SpiderCount",
          "Name": "Spiders Released",
          "Value": 5
        }
      ]
    }
  },
  "Key": "ability_trapper_spiderwave",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Crawling Plague",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "SpiderArmingTime": {
      "Name": null,
      "Value": 0.5
    },
    "SpiderChaseVelocity": {
      "Name": null,
      "Value": 400
    },
    "SpiderClimbHeight": {
      "Name": null,
      "Value": 0.3
    },
    "SpiderDistAboveGround": {
      "Name": null,
      "Value": 0.1
    },
    "SpiderExplodeRadius": {
      "Name": null,
      "Value": 3
    },
    "SpiderFloatDownRate": {
      "Name": null,
      "Value": 8
    },
    "SpiderGravity": {
      "Name": null,
      "Value": 1
    },
    "SpiderRandomPositionRadius": {
      "Name": null,
      "Value": 4
    },
    "SpiderTickRate": {
      "Name": null,
      "Value": 0.3
    },
    "SpreadAngle": {
      "Name": null,
      "Value": 30
    },
    "SpreadDistance": {
      "Name": null,
      "Value": 900
    }
  },
  "Radius": {
    "Name": "Radius",
    "Value": 3.5
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -45
    },
    {
      "DescKey": "ability_trapper_spiderwave_t2_desc",
      "SpiritReducedPerStack": 3,
      "SpiritResReducedPerStack": 3
    },
    {
      "DescKey": "ability_trapper_spiderwave_t3_desc",
      "SpiderCount": 5,
      "SpreadDistance": 900
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
      "hero_key": "hero_trapper",
      "hero_name": "Trapper",
      "lookup": "crawling plague",
      "name": "Crawling Plague",
      "type": "ability"
    }
  ]
}
````
