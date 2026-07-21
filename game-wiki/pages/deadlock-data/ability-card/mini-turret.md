---
title: "Mini Turret"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_shieldedsentry"
canonical_name: "Mini Turret"
snapshot_id: 39795
source_document_id: 7071
payload_hash: "026c4ce50fdd33fd99dd1f55c7cfe4ebf79a6ad7b96ee15004eeb77d2ebc8e11"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.219551+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Mini Turret

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shieldedsentry`
- Snapshot ID: `39795`
- Source-Dokument: `7071`
- Kurzinfo: Mini Turret aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 20
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 18.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 3
  },
  "DescKey": "citadel_ability_shieldedsentry_desc",
  "Duration": {
    "TurretDeployTime": {
      "Name": "Deploy Time",
      "Type": "duration",
      "Value": 0.25
    }
  },
  "HeroKey": "hero_forge",
  "HeroName": "McGinnis",
  "Info1": {
    "Alt": [
      {
        "Key": "TurretLifetime",
        "Name": "Lifetime",
        "Type": "duration",
        "Value": 35
      }
    ],
    "DescKey": "citadel_ability_shieldedsentry_desc",
    "Main": {
      "Props": [
        {
          "Key": "TurretDPS",
          "Name": "Turret DPS",
          "Scale": {
            "Type": "spirit",
            "Value": 0.42
          },
          "Type": "tech_damage",
          "Value": 24
        },
        {
          "Key": "TurretBaseHealth",
          "Name": "Turret Health",
          "Scale": {
            "Type": "power_increase",
            "Value": 7.8
          },
          "Type": "health",
          "Value": 90
        },
        {
          "Key": "TurretAttackRange",
          "Name": "Attack Range",
          "Type": "distance",
          "Value": 30
        }
      ]
    }
  },
  "Key": "citadel_ability_shieldedsentry",
  "Name": "Mini Turret",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AttackConeAngle": {
      "Name": null,
      "Value": 10
    },
    "AttackSpeedMult": {
      "Name": null,
      "Value": 100
    },
    "BossDamagePercentIncoming": {
      "Name": null,
      "Value": 50
    },
    "BossDamagePercentOutgoing": {
      "Name": null,
      "Value": 30
    },
    "DecayingResist": {
      "Name": null,
      "Value": 80
    },
    "DecayingResistDuration": {
      "Name": null,
      "Value": 6
    },
    "MeleeResist": {
      "Name": null,
      "Value": 35
    },
    "ModelScale": {
      "Name": null,
      "Value": 0.8
    },
    "NonHeroDamagePercentOutgoing": {
      "Name": null,
      "Value": 50
    },
    "TechResist": {
      "Name": "Spirit Resist",
      "Value": 35
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    },
    "TrackingSpeed": {
      "Name": null,
      "Value": 430
    },
    "TurretAttackDelay": {
      "Name": null,
      "Value": 0.2
    }
  },
  "Range": {
    "TurretAttackFalloffEnd": {
      "Name": null,
      "Type": "distance",
      "Value": 30
    },
    "TurretAttackFalloffStart": {
      "Name": null,
      "Type": "distance",
      "Value": 20
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "DescKey": "citadel_ability_shieldedsentry_t1_desc",
      "TurretAttackRange": 10,
      "TurretDPS": 10
    },
    {
      "AbilityCharges": 2
    },
    {
      "AttackSpeedMult": 25,
      "DescKey": "citadel_ability_shieldedsentry_t3_desc",
      "TurretLifetime": 12
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
      "hero_key": "hero_forge",
      "hero_name": "McGinnis",
      "lookup": "mini turret",
      "name": "Mini Turret",
      "type": "ability"
    }
  ]
}
````
