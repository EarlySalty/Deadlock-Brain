---
title: "Umbrella Maneuver"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "operative_umbrella_maneuver"
canonical_name: "Umbrella Maneuver"
snapshot_id: 39864
source_document_id: 7071
payload_hash: "919cacd3d243924eb765e0ca40277a09f94eaede904e05fbac3842123fb5c84b"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.353234+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Umbrella Maneuver

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `operative_umbrella_maneuver`
- Snapshot ID: `39864`
- Source-Dokument: `7071`
- Kurzinfo: Umbrella Maneuver aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 32.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "operative_umbrella_maneuver_desc",
  "HeroKey": "hero_operative",
  "HeroName": "Raven",
  "Info1": {
    "Alt": [],
    "DescKey": "operative_umbrella_maneuver_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.395
          },
          "Type": "tech_damage",
          "Value": 100
        }
      ]
    }
  },
  "Key": "operative_umbrella_maneuver",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Umbrella Maneuver",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ActivateTime": {
      "Name": null,
      "Value": 0.2
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 3.81
    },
    "BackwardsVelocity": {
      "Name": null,
      "Value": 13.0
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 1.524
    },
    "TimeBeforeProjectileLaunch": {
      "Name": null,
      "Value": 1.25
    },
    "UpImpulse": {
      "Name": null,
      "Value": 15.0
    }
  },
  "Range": {
    "ExplodeRadius": {
      "Name": "Explosion Radius",
      "Type": "distance",
      "Value": 5
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -14
    },
    {
      "Damage": 50
    },
    {
      "AbilityCooldown": 0,
      "DescKey": "operative_umbrella_maneuver_t3_desc"
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
      "hero_key": "hero_operative",
      "hero_name": "Raven",
      "lookup": "umbrella maneuver",
      "name": "Umbrella Maneuver",
      "type": "ability"
    }
  ]
}
````
