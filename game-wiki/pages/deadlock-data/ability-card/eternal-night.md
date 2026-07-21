---
title: "Eternal Night"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "drifter_darkness"
canonical_name: "Eternal Night"
snapshot_id: 39782
source_document_id: 7071
payload_hash: "d2041af4ef1e0e7049b4918f1c1c1afb138effd7d0da507d154714371600d291"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.194432+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Eternal Night

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `drifter_darkness`
- Snapshot ID: `39782`
- Source-Dokument: `7071`
- Kurzinfo: Eternal Night aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 1.0
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 100
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 145.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6.5
  },
  "Damage": {
    "BonusFireRate": {
      "Name": "Fire Rate",
      "Scale": {
        "Type": "spirit",
        "Value": 0
      },
      "Type": "fire_rate",
      "Value": 0
    }
  },
  "DescKey": "drifter_darkness_desc",
  "Duration": {
    "RevealDuration": {
      "Name": "Reveal Duration",
      "Type": "duration",
      "Value": 3
    }
  },
  "HeroKey": "hero_drifter",
  "HeroName": "Drifter",
  "Info1": {
    "Alt": [
      {
        "Key": "MaxTargets",
        "Name": "Max Targets",
        "Value": 2
      }
    ],
    "DescKey": "drifter_darkness_desc",
    "Main": {
      "Props": [
        {
          "Key": "SmallVisionDistance",
          "Name": "Reduced Vision",
          "Type": "distance",
          "Value": 15
        },
        {
          "Key": "BonusSprintSpeed",
          "Name": "Sprint Speed",
          "Type": "move_speed",
          "Value": 2
        }
      ]
    }
  },
  "Key": "drifter_darkness",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Eternal Night",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AuraLingerDuration": {
      "Name": null,
      "Value": 0.001
    },
    "BonusSprintAcceleration": {
      "Name": null,
      "Value": 12
    },
    "DarkFactor": {
      "Name": null,
      "Value": 1.0
    },
    "DistanceForMaxProjSpeed": {
      "Name": null,
      "Value": 200
    },
    "MaxProjectileSpeed": {
      "Name": null,
      "Value": 3000
    },
    "MinProjectileSpeed": {
      "Name": null,
      "Value": 3000
    },
    "PostProcessFadeInTime": {
      "Name": null,
      "Value": 0.2
    },
    "PostProcessFadeOutTime": {
      "Name": null,
      "Value": 1.0
    }
  },
  "Range": {
    "DrifterNearbyRangeCheck": {
      "Name": "Drifter Nearby",
      "Type": "distance",
      "Value": 40
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "BonusSprintSpeed": 10
    },
    {
      "AbilityCooldown": -40
    },
    {
      "AbilityDuration": 2.5,
      "DescKey": "drifter_darkness_t3_desc",
      "MaxTargets": 1
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
      "hero_key": "hero_drifter",
      "hero_name": "Drifter",
      "lookup": "eternal night",
      "name": "Eternal Night",
      "type": "ability"
    }
  ]
}
````
