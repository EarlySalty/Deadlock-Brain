---
title: "Bounce Pad"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_bounce_pad"
canonical_name: "Bounce Pad"
snapshot_id: 39749
source_document_id: 7071
payload_hash: "2cf0328941b17be353ba7e64fa60797585209708760309b55578fb2d75fb4ff3"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.127450+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Bounce Pad

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_bounce_pad`
- Snapshot ID: `39749`
- Source-Dokument: `7071`
- Kurzinfo: Bounce Pad aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.08
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 41
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 3.5
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 22
  },
  "DescKey": "ability_bounce_pad_desc",
  "Duration": {
    "MinAirTimeForStomp": {
      "Name": "Min Air Time for Stomp",
      "Type": "duration",
      "Value": 0.2
    }
  },
  "HeroKey": "hero_astro",
  "HeroName": "Holliday",
  "Info1": {
    "Alt": [
      {
        "Key": "AirControlPercent",
        "Name": "Air Control",
        "Type": "move_speed",
        "Value": 100
      },
      {
        "Key": "SpeedOnLandDuration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_bounce_pad_desc",
    "Main": {
      "Props": [
        {
          "Key": "StompDamage",
          "Name": "Stomp Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.372
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "Radius",
          "Name": "Radius",
          "Type": "distance",
          "Value": 9
        }
      ]
    }
  },
  "Key": "ability_bounce_pad",
  "Move": {
    "AirControlAccelPercent": {
      "Name": "Air Acceleration",
      "Type": "move_speed",
      "Value": 50
    },
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Bounce Pad",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BarrelBounceVelocity": {
      "Name": null,
      "Value": 800
    },
    "BarrelUpFactor": {
      "Name": null,
      "Value": 1
    },
    "BounceVelocity": {
      "Name": null,
      "Value": 750
    },
    "PlaceDistance": {
      "Name": null,
      "Value": 200
    },
    "Scale": {
      "Name": null,
      "Value": 1
    },
    "TossSpeed": {
      "Name": null,
      "Value": 12.7
    },
    "UpFactor": {
      "Name": null,
      "Value": 1.2
    },
    "VerticalDifferenceTolerance": {
      "Name": null,
      "Value": 60
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 9
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -10,
      "DescKey": "ability_bounce_pad_t1_desc"
    },
    {
      "DescKey": "ability_bounce_pad_t2_desc",
      "SpeedOnLand": 4,
      "SpeedOnLandDuration": 4
    },
    {
      "DescKey": "ability_bounce_pad_t3_desc",
      "StompStunDuration": 0.7
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
      "hero_key": "hero_astro",
      "hero_name": "Holliday",
      "lookup": "bounce pad",
      "name": "Bounce Pad",
      "type": "ability"
    }
  ]
}
````
