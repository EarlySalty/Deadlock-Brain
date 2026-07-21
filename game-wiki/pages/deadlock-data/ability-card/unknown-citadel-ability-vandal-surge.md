---
title: "Unknown(citadel_ability_vandal_surge)"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_vandal_surge"
canonical_name: "Unknown(citadel_ability_vandal_surge)"
snapshot_id: 39919
source_document_id: 7071
payload_hash: "dc982be415d7f2caf1d9260a60923c36d44f1c9a85f56c184e05a4aca39495e2"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.459605+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Unknown(citadel_ability_vandal_surge)

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_vandal_surge`
- Snapshot ID: `39919`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(citadel_ability_vandal_surge) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.6
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 20
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 16
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 1.25
  },
  "HeroKey": "hero_vandal",
  "HeroName": "Vandal",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_psychic_lift_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.974938
          },
          "Type": "tech_damage",
          "Value": 100
        }
      ]
    }
  },
  "Key": "citadel_ability_vandal_surge",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Unknown(citadel_ability_vandal_surge)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DampingFactor": {
      "Name": null,
      "Value": 0.5
    },
    "LiftHeight": {
      "Name": null,
      "Value": 120
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -28.0
    },
    {
      "AbilityDuration": 0.5
    },
    {
      "AbilityUnitTargetLimit": 5
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
  }
}
````
