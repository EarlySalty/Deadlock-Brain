---
title: "Lil Helpers"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_familiar_helpinghands"
canonical_name: "Lil Helpers"
snapshot_id: 39789
source_document_id: 7071
payload_hash: "18727015c4d26c26fd90908373ee5c5e60e73d6234630100672f4180430e5f64"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.207077+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Lil Helpers

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_familiar_helpinghands`
- Snapshot ID: `39789`
- Source-Dokument: `7071`
- Kurzinfo: Lil Helpers aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Scale": {
      "Type": "range",
      "Value": 0.0
    },
    "Type": "range",
    "Value": 45
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Damage": {
    "DPSPerSprite": {
      "Name": null,
      "Type": "tech_damage",
      "Value": 1
    },
    "Damage": {
      "Name": "Damage",
      "Type": "tech_damage",
      "Value": 20
    }
  },
  "DescKey": "ability_familiar_helpinghands_desc",
  "Duration": {
    "ArmTime": {
      "Name": "Arm Time",
      "Type": "duration",
      "Value": 0.1
    },
    "HelperChoreCooldownDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 5
    },
    "HelperDowntimeDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 15.1
    },
    "InfestHealInterval": {
      "Name": null,
      "Type": "duration",
      "Value": 2.0
    },
    "PatrolDamageCooldown": {
      "Name": null,
      "Type": "duration",
      "Value": 10
    }
  },
  "HeroKey": "hero_familiar",
  "HeroName": "Rem",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_familiar_helpinghands_desc",
    "Main": {
      "Props": []
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "TechArmorGain",
          "Name": "Spirit Resist",
          "Title": "Following Hero:",
          "Type": "tech_armor_up",
          "Value": 12
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Title": "Following Hero:",
          "Type": "move_speed",
          "Value": 3.0
        },
        {
          "Key": "PlayerInfestDuration",
          "Name": "Duration",
          "Title": "Following Hero:",
          "Value": 8
        }
      ]
    }
  },
  "Info3": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "InfestDamageTakenPercent",
          "Name": "Damage / Resists",
          "Title": "Following Trooper:",
          "Value": 30
        },
        {
          "Key": "InfestHeal",
          "Name": "Trooper Heal",
          "Scale": {
            "Type": "spirit",
            "Value": 0.14
          },
          "Title": "Following Trooper:",
          "Type": "healing",
          "Value": 8
        },
        {
          "Key": "NPCInfestDuration",
          "Name": "Duration",
          "Title": "Following Trooper:",
          "Value": 50
        }
      ]
    }
  },
  "Key": "ability_familiar_helpinghands",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Lil Helpers",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "HelperCount": {
      "Name": null,
      "Value": 1
    },
    "HelpersPerPatrol": {
      "Name": null,
      "Value": 4
    },
    "InfestBurstHealthPercent": {
      "Name": null,
      "Value": 75
    },
    "TickRate": {
      "Name": null,
      "Value": 0.2
    }
  },
  "Range": {
    "AuraAttackHeight": {
      "Name": null,
      "Type": "distance",
      "Value": 10
    },
    "AuraRadius": {
      "Name": "Aura Radius",
      "Type": "distance",
      "Value": 10
    },
    "AuraSoftRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 10
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BonusMoveSpeed": 1.5,
      "DescKey": "ability_familiar_helpinghands_t1_desc",
      "HelperCount": 1
    },
    {
      "DescKey": "ability_familiar_helpinghands_t2_desc",
      "HelperCount": 1,
      "InfestDamageTakenPercent": 15
    },
    {
      "DescKey": "ability_familiar_helpinghands_t3_desc",
      "HelperCount": 1,
      "TechArmorGain": 15
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
      "hero_key": "hero_familiar",
      "hero_name": "Rem",
      "lookup": "lil helpers",
      "name": "Lil Helpers",
      "type": "ability"
    }
  ]
}
````
