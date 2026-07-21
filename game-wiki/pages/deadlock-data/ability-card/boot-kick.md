---
title: "Boot Kick"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_werewolf_kickflip"
canonical_name: "Boot Kick"
snapshot_id: 39936
source_document_id: 7071
payload_hash: "a758ea2258f477835d8d5ee67d87f4b4a28f6ba6c03817aa4c02b80dabd9c2fb"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.492725+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Boot Kick

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_kickflip`
- Snapshot ID: `39936`
- Source-Dokument: `7071`
- Kurzinfo: Boot Kick aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.25
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 10.6
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.35
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 21
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_werewolf",
  "HeroName": "Silver",
  "Info1": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_werewolf_kickflip_desc_1",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "melee",
            "Value": 0.9
          },
          "Type": "tech_damage",
          "Value": 0
        },
        {
          "Key": "DisarmDuration",
          "Name": "Disarm Duration",
          "StatusEffect": "Disarmed",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "MarkDuration",
        "Name": "Mark Duration",
        "Scale": {
          "Type": "duration",
          "Value": 1.0
        },
        "Type": "duration",
        "Value": 3
      }
    ],
    "DescKey": "ability_werewolf_kickflip_desc_2",
    "Main": {
      "Props": [
        {
          "Key": "BonusDamage",
          "Name": "Bonus Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.0
          },
          "Type": "tech_damage",
          "Value": 25
        }
      ]
    }
  },
  "Key": "ability_werewolf_kickflip",
  "Name": "Boot Kick",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.25
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirDrag": {
      "Name": null,
      "Value": 0.8
    },
    "CameraTurnRateMax": {
      "Name": null,
      "Value": 188
    },
    "EnemyPushForceAway": {
      "Name": null,
      "Value": 300
    },
    "EnemyPushForceUp": {
      "Name": null,
      "Value": 300
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 20
    },
    "LeapForwardOffset": {
      "Name": null,
      "Value": 2.5
    },
    "SelfPushForceCameraAway": {
      "Name": null,
      "Value": 600
    },
    "SelfPushForceUp": {
      "Name": null,
      "Value": 200
    },
    "SlowDuration": {
      "Name": "Slow Duration",
      "Value": 0.1
    },
    "SuccessInputWindow": {
      "Name": null,
      "Value": 0.3
    },
    "TimeScaleDebuff": {
      "Name": null,
      "Value": 95
    }
  },
  "Range": {
    "LeapRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 1.7
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -6
    },
    {
      "DescKey": "ability_werewolf_kickflip_t2_desc",
      "StaminaRestore": 2
    },
    {
      "BonusDamage": 80,
      "DebuffDuration": 5,
      "DescKey": "ability_werewolf_kickflip_t3_desc",
      "OutgoingDamagePercent": -35
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
      "hero_key": "hero_werewolf",
      "hero_name": "Silver",
      "lookup": "boot kick",
      "name": "Boot Kick",
      "type": "ability"
    }
  ]
}
````
