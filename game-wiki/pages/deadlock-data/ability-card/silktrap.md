---
title: "Silktrap"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_trapper_webwall"
canonical_name: "Silktrap"
snapshot_id: 39908
source_document_id: 7071
payload_hash: "50fa1e4518f6ad067b98b8f72aee29ebce637348eff55f19bc12fffdf2197f2f"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.436890+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Silktrap

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_webwall`
- Snapshot ID: `39908`
- Source-Dokument: `7071`
- Kurzinfo: Silktrap aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.22
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 40
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 40
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 1
  },
  "DescKey": "ability_trapper_webwall_desc",
  "HeroKey": "hero_trapper",
  "HeroName": "Trapper",
  "Info1": {
    "Alt": [
      {
        "Key": "WebDuration",
        "Name": "Web Duration",
        "Type": "duration",
        "Value": 120
      },
      {
        "Key": "MinWallToWallDistance",
        "Name": "Minimum Web Distance",
        "Type": "distance",
        "Value": 3
      },
      {
        "Key": "MaxWallToWallDistance",
        "Name": "Maximum Web Distance",
        "Type": "distance",
        "Value": 100
      }
    ],
    "DescKey": "ability_trapper_webwall_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.279
          },
          "Type": "tech_damage",
          "Value": 40
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 99
        },
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "Value": 3
        },
        {
          "Key": "SilenceDuration",
          "Name": "Silence Duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_trapper_webwall",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Silktrap",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    },
    "WebArmTime": {
      "Name": null,
      "Value": 0.5
    },
    "WebWallTickRate": {
      "Name": null,
      "Value": 0.15
    }
  },
  "Radius": {
    "Name": "Radius",
    "Value": 0.6
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "WebDuration": 120
    },
    {
      "DebuffDuration": 2,
      "DescKey": "ability_trapper_webwall_t3_desc",
      "SilenceDuration": 2.5
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
      "lookup": "silktrap",
      "name": "Silktrap",
      "type": "ability"
    }
  ]
}
````
