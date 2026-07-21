---
title: "Powder Keg"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_explosive_barrel"
canonical_name: "Powder Keg"
snapshot_id: 39748
source_document_id: 7071
payload_hash: "55815cbf7e96b8ff8f8839f061ddbd02fc47f7c3592377432e790d00bd0a105e"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.124644+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Powder Keg

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_explosive_barrel`
- Snapshot ID: `39748`
- Source-Dokument: `7071`
- Kurzinfo: Powder Keg aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.125
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 28.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 7.5
  },
  "DescKey": "ability_explosive_barrel_desc",
  "Duration": {
    "BarrelLifetime": {
      "Name": "Barrel Life Time",
      "Type": "duration",
      "Value": 8
    }
  },
  "HeroKey": "hero_astro",
  "HeroName": "Holliday",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_explosive_barrel_desc",
    "Main": {
      "Props": [
        {
          "Key": "ArmTime",
          "Name": "Arm Time",
          "Type": "duration",
          "Value": 0.1
        },
        {
          "Key": "BarrelDamage",
          "Name": "Explosion Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.2
          },
          "Type": "tech_damage",
          "Value": 80
        },
        {
          "Key": "TossDuration",
          "Name": "Duration",
          "StatusEffect": "Displacement",
          "Value": 0.4
        }
      ]
    }
  },
  "Key": "ability_explosive_barrel",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Powder Keg",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BarrelHeavyMeleeForceForward": {
      "Name": null,
      "Value": 1800
    },
    "BarrelHeavyMeleeForceUp": {
      "Name": null,
      "Value": 300
    },
    "BarrelLightMeleeForceForward": {
      "Name": null,
      "Value": 1400
    },
    "BarrelLightMeleeForceUp": {
      "Name": null,
      "Value": 300
    },
    "BarrelPitchMax": {
      "Name": null,
      "Value": 90
    },
    "BarrelPitchMin": {
      "Name": null,
      "Value": 2
    },
    "BarrelRollSpeedMoveAir": {
      "Name": null,
      "Value": 10
    },
    "BarrelRollSpeedMoveMin": {
      "Name": null,
      "Value": 20
    },
    "BarrelScale": {
      "Name": null,
      "Value": 1.3
    },
    "MinTimeBeforeDestroy": {
      "Name": null,
      "Value": 0.1
    },
    "TossSpeed": {
      "Name": null,
      "Value": 3.556
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 6
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "AbilityCharges": 1
    },
    {
      "AbilityCooldownBetweenCharge": -5,
      "BarrelDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 80
      },
      "DescKey": "ability_explosive_barrel_t3_desc"
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
      "lookup": "powder keg",
      "name": "Powder Keg",
      "type": "ability"
    }
  ]
}
````
