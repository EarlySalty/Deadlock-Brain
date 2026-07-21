---
title: "Shocking Reanimation"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_frank_revive"
canonical_name: "Shocking Reanimation"
snapshot_id: 39806
source_document_id: 7071
payload_hash: "c03c8337a59de7c0b903fa37f0436ee7e81c5244eef397f05c7da51488fdd979"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.240494+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Shocking Reanimation

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_frank_revive`
- Snapshot ID: `39806`
- Source-Dokument: `7071`
- Kurzinfo: Shocking Reanimation aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 3
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 275
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Debuff": {
    "EnemyDashSlowPercent": {
      "Name": null,
      "Type": "slow",
      "Value": -30
    }
  },
  "DescKey": "ability_frank_revive_desc",
  "HeroKey": "hero_frank",
  "HeroName": "Victor",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_up",
        "Value": 0
      },
      {
        "Key": "TechResist",
        "Name": "Spirit Resist",
        "Type": "tech_armor_up",
        "Value": 0
      },
      {
        "Key": "BonusDamagePerBullet",
        "Name": "Damage per Bullet",
        "Scale": {
          "Type": "spirit",
          "Value": 0.0
        },
        "Type": "tech_damage",
        "Value": 0
      },
      {
        "Key": "BonusFireRate",
        "Name": "Fire Rate",
        "Type": "fire_rate",
        "Value": 0
      },
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 120
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Value": 3
      }
    ],
    "DescKey": "ability_frank_revive_desc",
    "Main": {
      "Props": [
        {
          "Key": "RespawnHealthPercent",
          "Name": "Rebirth Health",
          "Type": "healing",
          "Value": 50
        },
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Title": "On Revive:",
          "Type": "tech_armor_up",
          "Value": 0
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.0
          },
          "Title": "On Revive:",
          "Type": "tech_damage",
          "Value": 200
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Title": "On Revive:",
          "Value": 1.5
        }
      ]
    }
  },
  "Key": "ability_frank_revive",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Shocking Reanimation",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.66
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "HalfHeight": {
      "Name": null,
      "Value": 15
    },
    "InitialDelay": {
      "Name": null,
      "Value": 0.5
    },
    "RespawnDelay": {
      "Name": null,
      "Value": 3
    },
    "ZombieTickRate": {
      "Name": null,
      "Value": 0.02
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 18
  },
  "Slot": "4",
  "Upgrades": [
    {
      "BonusDamagePerBullet": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.06
        },
        "Value": 6.0
      },
      "BonusFireRate": 15,
      "DescKey": "ability_frank_revive_t1_desc"
    },
    {
      "RespawnHealthPercent": 50
    },
    {
      "AbilityCooldown": -95,
      "Damage": 175,
      "DescKey": "ability_frank_revive_t3_desc",
      "StunDuration": 1.5
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
      "hero_key": "hero_frank",
      "hero_name": "Victor",
      "lookup": "shocking reanimation",
      "name": "Shocking Reanimation",
      "type": "ability"
    }
  ]
}
````
