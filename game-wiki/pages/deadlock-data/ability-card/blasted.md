---
title: "Blasted"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_punkgoat_blasted"
canonical_name: "Blasted"
snapshot_id: 39877
source_document_id: 7071
payload_hash: "a46a3081d352adb759a64f61945c1af10e4c8bf0cb8b373e071993ed2c1142da"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.378936+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Blasted

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_punkgoat_blasted`
- Snapshot ID: `39877`
- Source-Dokument: `7071`
- Kurzinfo: Blasted aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.28
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 27
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 8.0
  },
  "DescKey": "ability_punkgoat_blasted_desc",
  "Duration": {
    "BulletDamageAmpDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 7.0
    },
    "DurationPerHeavyMelee": {
      "Name": null,
      "Type": "duration",
      "Value": 4.5
    },
    "DurationPerLightMelee": {
      "Name": null,
      "Type": "duration",
      "Value": 2.8
    }
  },
  "HeroKey": "hero_punkgoat",
  "HeroName": "Billy",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_punkgoat_blasted_desc",
    "Main": {}
  },
  "Info2": {
    "Alt": [
      {
        "Key": "HealthBoostDuration",
        "Type": "duration",
        "Value": 11
      }
    ],
    "DescKey": "ability_punkgoat_blastedactive_desc",
    "Main": {
      "Props": [
        {
          "Key": "MaxHealthMelee",
          "Name": "Melee Bonus Health",
          "Scale": {
            "Type": "spirit",
            "Value": 0.6
          },
          "Title": "While Blasted:",
          "Type": "healing",
          "Value": 70
        },
        {
          "Key": "BulletDamageAmp",
          "Name": "Wrecked Bullet Amp",
          "Title": "While Blasted:",
          "Type": "bullet_damage",
          "Value": 10
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Title": "While Blasted:",
          "Type": "move_speed",
          "Value": 0.0
        }
      ]
    }
  },
  "Key": "ability_punkgoat_blasted",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Blasted",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BlastedRateOnBulletPct": {
      "Name": null,
      "Value": 50
    },
    "BulletsReloadedPerHeavyMeleePct": {
      "Name": null,
      "Value": 100
    },
    "BulletsReloadedPerLightMeleePct": {
      "Name": null,
      "Value": 35
    },
    "LightMeleeScalePct": {
      "Name": null,
      "Value": 40
    },
    "MaxDuration": {
      "Name": null,
      "Value": 35.0
    },
    "NonPlayerResourceScalePct": {
      "Name": null,
      "Value": 25
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BonusMoveSpeed": 2.25,
      "DescKey": "ability_punkgoat_blasted_t1_desc"
    },
    {
      "BulletDamageAmp": 7,
      "DescKey": "ability_punkgoat_blasted_t2_desc",
      "GainSlamOnUse": 1
    },
    {
      "DescKey": "ability_punkgoat_blasted_t3_desc",
      "MaxHealthMelee": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 50
      }
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
      "hero_key": "hero_punkgoat",
      "hero_name": "Billy",
      "lookup": "blasted",
      "name": "Blasted",
      "type": "ability"
    }
  ]
}
````
