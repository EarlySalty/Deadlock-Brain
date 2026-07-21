---
title: "Bookwyrm"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_bookworm_dragonfire"
canonical_name: "Bookwyrm"
snapshot_id: 39767
source_document_id: 7071
payload_hash: "bc15016afacddd07ea60420a6a23487d7da9fc2c847739a26fa20397a0f26f42"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.164985+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Bookwyrm

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_bookworm_dragonfire`
- Snapshot ID: `39767`
- Source-Dokument: `7071`
- Kurzinfo: Bookwyrm aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 33
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 7
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5
  },
  "DescKey": "ability_bookworm_dragonfire_desc",
  "Duration": {
    "DebuffDuration": {
      "Name": "Debuff Duration",
      "Type": "duration",
      "Value": 1.5
    }
  },
  "HeroKey": "hero_bookworm",
  "HeroName": "Paige",
  "Info1": {
    "Alt": [
      {
        "Key": "DragonTravelRange",
        "Name": "Dragon Travel Range",
        "Type": "distance",
        "Value": 20
      }
    ],
    "DescKey": "ability_bookworm_dragonfire_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.3
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.3
          },
          "Type": "tech_damage",
          "Value": 30
        },
        {
          "Key": "GroundFlameDuration",
          "Name": "Trail Duration",
          "Type": "duration",
          "Value": 3.0
        }
      ]
    }
  },
  "Key": "ability_bookworm_dragonfire",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Bookwyrm",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AuraLingerDuration": {
      "Name": null,
      "Value": 0.1
    },
    "DragonConeRange": {
      "Name": null,
      "Value": 5
    },
    "DragonRangePerSecond": {
      "Name": null,
      "Value": 500
    },
    "DragonSearchRadius": {
      "Name": null,
      "Value": 8.5
    },
    "DragonSearchTickRate": {
      "Name": null,
      "Value": 0.1
    },
    "DragonUpwardSpeed": {
      "Name": null,
      "Value": 400
    },
    "GroundAuraSpacing": {
      "Name": null,
      "Value": 1
    },
    "StartupDelay": {
      "Name": null,
      "Value": 0.3
    },
    "TickRate": {
      "Name": null,
      "Value": 0.3
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 4
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -12,
      "DescKey": "ability_bookworm_dragonfire_t1_desc"
    },
    {
      "AbilityCharges": 1,
      "DescKey": "ability_bookworm_dragonfire_t2_desc",
      "GroundFlameDuration": 2,
      "Radius": 1
    },
    {
      "DPS": 30.0,
      "Damage": 100,
      "DescKey": "ability_bookworm_dragonfire_t3_desc",
      "DragonTravelRange": 12
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
      "hero_key": "hero_bookworm",
      "hero_name": "Paige",
      "lookup": "bookwyrm",
      "name": "Bookwyrm",
      "type": "ability"
    }
  ]
}
````
