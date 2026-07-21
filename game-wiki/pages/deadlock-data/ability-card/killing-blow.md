---
title: "Killing Blow"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_shiv_killing_blow"
canonical_name: "Killing Blow"
snapshot_id: 39886
source_document_id: 7071
payload_hash: "2a9f99933ac381c094426557f5bd455f8e19bd9a1b78d1d42902b25ce587b90e"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.395289+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Killing Blow

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shiv_killing_blow`
- Snapshot ID: `39886`
- Source-Dokument: `7071`
- Kurzinfo: Killing Blow aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.05
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 12
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 145.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Cooldown": {
    "RageDrainDelayDuration": {
      "Name": null,
      "Type": "cooldown",
      "Value": 12
    },
    "RecastWindow": {
      "Name": "Recast Window",
      "Type": "cooldown",
      "Value": 20
    }
  },
  "Duration": {
    "RageDrainRate": {
      "Name": null,
      "Type": "duration",
      "Value": 0.25
    }
  },
  "HeroKey": "hero_shiv",
  "HeroName": "Shiv",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_shiv_killing_blow_active_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Type": "tech_damage",
          "Value": 200
        },
        {
          "Key": "EnemyHealthPercent",
          "Name": "Enemy health threshold",
          "Type": "health",
          "Value": 20
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "citadel_ability_shiv_killing_blow_passive_desc",
    "Main": {
      "Props": [
        {
          "Key": "BuffDamage",
          "Name": "Full Rage Damage Bonus",
          "Type": "damage",
          "Value": 8
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 0
        }
      ]
    }
  },
  "Info3": {
    "Alt": [],
    "DescKey": "citadel_ability_shiv_killing_blow_passive_desc_footer",
    "Main": {}
  },
  "Key": "citadel_ability_shiv_killing_blow",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Killing Blow",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.15
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BonusAbilityResource": {
      "Name": null,
      "Value": 10
    },
    "CameraDistance": {
      "Name": null,
      "Value": 400
    },
    "EnemyHealthPercentBuffer": {
      "Name": null,
      "Value": 3
    },
    "FailedExecuteCooldownPenalty": {
      "Name": null,
      "Value": 30
    },
    "MinTimeToTarget": {
      "Name": null,
      "Value": 0.5
    },
    "MoveSpeedToTarget": {
      "Name": null,
      "Value": 30
    },
    "RagePerHeavyMelee": {
      "Name": null,
      "Value": 2.85384
    },
    "RagePerLightMelee": {
      "Name": null,
      "Value": 1.55664
    },
    "RagePerSpiritDamage": {
      "Name": null,
      "Value": 0.01452864
    },
    "RagePerWeaponDamage": {
      "Name": null,
      "Value": 0.0158766
    },
    "SlashRange": {
      "Name": null,
      "Value": 90
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCastRange": 6,
      "BonusMoveSpeed": 2,
      "DescKey": "citadel_ability_shiv_killing_blow_t1_desc"
    },
    {
      "AbilityCooldown": -25,
      "BuffDamage": 16,
      "DescKey": "citadel_ability_shiv_killing_blow_t2_desc"
    },
    {
      "DescKey": "citadel_ability_shiv_killing_blow_t3_desc",
      "EnemyHealthPercent": 8
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
      "hero_key": "hero_shiv",
      "hero_name": "Shiv",
      "lookup": "killing blow",
      "name": "Killing Blow",
      "type": "ability"
    }
  ]
}
````
