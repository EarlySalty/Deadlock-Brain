---
title: "Rend"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "drifter_blood_blast"
canonical_name: "Rend"
snapshot_id: 39779
source_document_id: 7071
payload_hash: "90c0cb64ed24190f46cfd31b84b062db02c72fdb4ee666e3ef5e10a9e0825c73"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.188583+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Rend

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `drifter_blood_blast`
- Snapshot ID: `39779`
- Source-Dokument: `7071`
- Kurzinfo: Rend aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.4
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 16
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 16.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "drifter_blood_blast_desc",
  "HeroKey": "hero_drifter",
  "HeroName": "Drifter",
  "Info1": {
    "Alt": [
      {
        "Key": "RangeForBonusDamage",
        "Name": "Close Range",
        "Type": "distance",
        "Value": 8
      },
      {
        "Key": "LifestealDuration",
        "Name": "Lifesteal Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "drifter_blood_blast_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusDamage",
          "Name": "Bonus Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.8
          },
          "Type": "tech_damage",
          "Value": 40.0
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "melee",
            "Value": 1.2
          },
          "Title": "On Close Range",
          "Type": "melee_damage",
          "Value": 0.0
        },
        {
          "Key": "DamageHeavyMelee",
          "Name": "Damage",
          "Scale": {
            "Type": "heavy_melee",
            "Value": 0
          },
          "Title": "On Close Range",
          "Type": "melee_damage",
          "Value": 0
        },
        {
          "Key": "BulletLifestealPercentHero",
          "Name": "Bullet Lifesteal",
          "Title": "On Close Range",
          "Type": "healing",
          "Value": 0
        },
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "StatusEffect": "Silence",
          "Title": "On Close Range",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "drifter_blood_blast",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Rend",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.4
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 30
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 70
    },
    "ExtraSweepConeAngle": {
      "Name": null,
      "Value": 60
    },
    "ExtraSweepOffsetBehindCaster": {
      "Name": null,
      "Value": 80
    },
    "ExtraSweepRange": {
      "Name": null,
      "Value": 3
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 1
    }
  },
  "Range": {
    "TargetingConeAngle": {
      "Name": "Attack Angle",
      "Type": "distance",
      "Value": 50
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "BonusDamage": 40
    },
    {
      "AbilityCooldown": -8.0,
      "DescKey": "drifter_blood_blast_t2_desc"
    },
    {
      "Damage": {
        "Scale": {
          "Multiply": true,
          "Type": "melee",
          "Value": 0.0
        },
        "Value": 0
      },
      "DamageHeavyMelee": {
        "Scale": {
          "Type": "heavy_melee",
          "Value": 0.55
        },
        "Value": 0
      },
      "DebuffDuration": 2.3,
      "DescKey": "drifter_blood_blast_t3_desc",
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
      "hero_key": "hero_drifter",
      "hero_name": "Drifter",
      "lookup": "rend",
      "name": "Rend",
      "type": "ability"
    }
  ]
}
````
