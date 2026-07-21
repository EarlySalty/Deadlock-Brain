---
title: "Frost Grenade"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_ice_grenade"
canonical_name: "Frost Grenade"
snapshot_id: 39835
source_document_id: 7071
payload_hash: "6986e3e8e84d259b72285442cc00c42c330c76632ea199e6055fead8d518d179"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.294863+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Frost Grenade

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_ice_grenade`
- Snapshot ID: `39835`
- Source-Dokument: `7071`
- Kurzinfo: Frost Grenade aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 7
  },
  "DescKey": "ability_ice_grenade_desc",
  "HeroKey": "hero_kelvin",
  "HeroName": "Kelvin",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 4
      }
    ],
    "DescKey": "ability_ice_grenade_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.6
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "HealAmount",
          "Name": "Heal Amount",
          "Scale": {
            "Type": "spirit",
            "Value": 0.8
          },
          "Type": "healing",
          "Value": 60
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 40
        }
      ]
    }
  },
  "Key": "ability_ice_grenade",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Frost Grenade",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 6.5
  },
  "Slot": "1",
  "Upgrades": [
    {
      "Damage": 30,
      "DescKey": "ability_ice_grenade_t1_desc",
      "HealAmount": 30
    },
    {
      "AbilityCooldown": -10,
      "DescKey": "ability_ice_grenade_t2_desc",
      "PauseStaminaRegen": 1
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.8
        },
        "Value": 0
      },
      "DescKey": "ability_ice_grenade_t3_desc",
      "HealAmount": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.9
        },
        "Value": 0
      },
      "Radius": 2
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
      "hero_key": "hero_kelvin",
      "hero_name": "Kelvin",
      "lookup": "frost grenade",
      "name": "Frost Grenade",
      "type": "ability"
    }
  ]
}
````
