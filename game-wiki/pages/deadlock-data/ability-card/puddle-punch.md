---
title: "Puddle Punch"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "viscous_telepunch"
canonical_name: "Puddle Punch"
snapshot_id: 39929
source_document_id: 7071
payload_hash: "657e79658e57fc26483a22206a25023dc1c460155a4ab4a4a1014f470cdd7a39"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.479030+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Puddle Punch

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `viscous_telepunch`
- Snapshot ID: `39929`
- Source-Dokument: `7071`
- Kurzinfo: Puddle Punch aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
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
    "Value": 24.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 1.7
  },
  "DescKey": "viscous_telepunch_desc",
  "HeroKey": "hero_viscous",
  "HeroName": "Viscous",
  "Info1": {
    "Alt": [
      {
        "Key": "ImpactDuration",
        "Name": "Slow Duration",
        "Value": 4
      },
      {
        "Key": "LifeStealPercentOnHit",
        "Type": "healing",
        "Value": 0
      }
    ],
    "DescKey": "viscous_telepunch_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "melee",
            "Value": 0.6
          },
          "Type": "melee_damage",
          "Value": 20.0
        },
        {
          "Key": "DamageHeavyMelee",
          "Name": "Damage",
          "Scale": {
            "Type": "heavy_melee",
            "Value": 0
          },
          "Type": "melee_damage",
          "Value": 0
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 20
        },
        {
          "Key": "TossDuration",
          "Name": "Duration",
          "StatusEffect": "Displacement",
          "Value": 0.6
        }
      ]
    }
  },
  "Key": "viscous_telepunch",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Puddle Punch",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "FriendlyImpactDuration": {
      "Name": null,
      "Value": 2
    },
    "PunchFriendlyAirControl": {
      "Name": null,
      "Value": 30
    },
    "PunchHalfHeight": {
      "Name": null,
      "Value": 5.5
    },
    "PunchRollSlow": {
      "Name": null,
      "Value": -40
    },
    "PunchRollSlowDuration": {
      "Name": null,
      "Value": 1
    },
    "TossGroundSideRatio": {
      "Name": null,
      "Value": 0.7
    },
    "TossSpeed": {
      "Name": null,
      "Value": 625
    },
    "TossSpeedUpWall": {
      "Name": null,
      "Value": 500
    },
    "TossSpeedWall": {
      "Name": null,
      "Value": 750
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 4
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCharges": 1,
      "Damage": 20,
      "DescKey": "viscous_telepunch_t1_desc"
    },
    {
      "DescKey": "viscous_telepunch_t2_desc",
      "LifeStealPercentOnHit": 60,
      "Radius": 1.5
    },
    {
      "AbilityCooldown": -14,
      "Damage": {
        "Scale": {
          "Multiply": true,
          "Type": "melee",
          "Value": 0.0
        },
        "Value": -40
      },
      "DamageHeavyMelee": {
        "Scale": {
          "Type": "heavy_melee",
          "Value": 0.6
        },
        "Value": 40
      },
      "DescKey": "viscous_telepunch_t3_desc",
      "UseHeavyMelee": 1
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
      "hero_key": "hero_viscous",
      "hero_name": "Viscous",
      "lookup": "puddle punch",
      "name": "Puddle Punch",
      "type": "ability"
    }
  ]
}
````
