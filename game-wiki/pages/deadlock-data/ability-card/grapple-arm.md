---
title: "Grapple Arm"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_hook"
canonical_name: "Grapple Arm"
snapshot_id: 39758
source_document_id: 7071
payload_hash: "415d6af8a07f9b04d50f58d89fb0eb240ff8e1f7424d3f54061832885caf7311"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.146553+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Grapple Arm

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hook`
- Snapshot ID: `39758`
- Source-Dokument: `7071`
- Kurzinfo: Grapple Arm aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 30
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 23.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_hook_desc",
  "Duration": {
    "RestrictionDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 0.5
    }
  },
  "HeroKey": "hero_bebop",
  "HeroName": "Bebop",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_hook_desc",
    "Main": {
      "Props": [
        {
          "Key": "AbilityCastRange",
          "Name": "Cast Range",
          "Type": "range",
          "Value": 30
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "melee",
            "Value": 0.7
          },
          "Type": "melee_damage",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_hook",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Grapple Arm",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CancelHookDuration": {
      "Name": null,
      "Value": 0.2
    },
    "FriendlyHookIgnoreRange": {
      "Name": null,
      "Value": 8
    },
    "HookImpactDelay": {
      "Name": null,
      "Value": 0.5
    },
    "HookingSlowSpeedLimit": {
      "Name": null,
      "Value": 5
    },
    "SlowPercent": {
      "Name": "Move Speed",
      "Value": 90
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BulletAmp": 20,
      "BulletAmpDuration": 6,
      "DescKey": "citadel_ability_hook_t1_desc"
    },
    {
      "AbilityCastRange": 30
    },
    {
      "AbilityCooldown": -11.5
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
      "hero_key": "hero_bebop",
      "hero_name": "Bebop",
      "lookup": "grapple arm",
      "name": "Grapple Arm",
      "type": "ability"
    }
  ]
}
````
