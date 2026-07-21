---
title: "Seismic Impact"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_bull_leap"
canonical_name: "Seismic Impact"
snapshot_id: 39755
source_document_id: 7071
payload_hash: "ad6457ec54c18328a696a761566530fc99becef361e13c3994b3f384e71f60e1"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.140085+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Seismic Impact

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_bull_leap`
- Snapshot ID: `39755`
- Source-Dokument: `7071`
- Kurzinfo: Seismic Impact aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 215.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_bull_leap_desc",
  "HeroKey": "hero_atlas",
  "HeroName": "Abrams",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_bull_leap_desc",
    "Main": {
      "Props": [
        {
          "Key": "ImmunityDuration",
          "Name": "Immunity Duration",
          "StatusEffect": "Unstoppable",
          "Type": "duration",
          "Value": 0
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.325
          },
          "Type": "tech_damage",
          "Value": 100
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Title": "On Hit:",
          "Type": "duration",
          "Value": 1.6
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "BonusMaxHealthPerHero",
          "Name": "Max Health",
          "Title": "On Hero Hit:",
          "Type": "health",
          "Value": 0
        },
        {
          "Key": "BonusFireRatePerHero",
          "Name": "Fire Rate",
          "Title": "On Hero Hit:",
          "Type": "bullet_damage",
          "Value": 0
        },
        {
          "Key": "LandingBonusesDuration",
          "Name": "Buff Duration",
          "Title": "On Hero Hit:",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_bull_leap",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Seismic Impact",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ImpactHeight": {
      "Name": null,
      "Value": 6
    },
    "TossSpeed": {
      "Name": null,
      "Value": 450
    }
  },
  "Range": {
    "ImpactRadius": {
      "Name": "Impact Radius",
      "Type": "distance",
      "Value": 9
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -30.0
    },
    {
      "StunDuration": 0.8
    },
    {
      "DescKey": "citadel_ability_bull_leap_t3_desc",
      "ImmunityDuration": 6,
      "ImpactRadius": 6
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
      "hero_key": "hero_atlas",
      "hero_name": "Abrams",
      "lookup": "seismic impact",
      "name": "Seismic Impact",
      "type": "ability"
    }
  ]
}
````
