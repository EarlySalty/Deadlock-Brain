---
title: "Chain Gang"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_punkgoat_tether"
canonical_name: "Chain Gang"
snapshot_id: 39878
source_document_id: 7071
payload_hash: "07e2895f705d870ba0b5a27a83470b0c30ecaa00b6eab7739e1dc1935ebb7ac6"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.380763+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Chain Gang

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_punkgoat_tether`
- Snapshot ID: `39878`
- Source-Dokument: `7071`
- Kurzinfo: Chain Gang aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 12.0
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 175
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 2.8
  },
  "Damage": {
    "DPS": {
      "Name": "Damage Per Second",
      "Scale": {
        "Type": "spirit",
        "Value": 0.9
      },
      "Type": "tech_damage",
      "Value": 45
    },
    "DamageIncreasePct": {
      "Name": null,
      "Scale": {
        "Type": "spirit",
        "Value": 0.0
      },
      "Type": "damage",
      "Value": 0
    }
  },
  "DescKey": "ability_punkgoat_tether_desc",
  "HeroKey": "hero_punkgoat",
  "HeroName": "Billy",
  "Info1": {
    "Alt": [
      {
        "Key": "MoveSpeedSlowMaxPct",
        "Type": "move_speed",
        "Value": 35
      },
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
        "Key": "FireRateSlow",
        "Name": "Fire Rate",
        "Type": "bullet_damage",
        "Value": 0
      },
      {
        "Key": "FireRateSlowDuration",
        "Name": "Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_punkgoat_tether_desc",
    "Main": {
      "Props": [
        {
          "Key": "UnstoppablePerHero",
          "Name": "Per Hero",
          "StatusEffect": "Unstoppable",
          "Type": "duration",
          "Value": 0
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.0
          },
          "Type": "tech_damage",
          "Value": 120
        },
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 2.8
        }
      ]
    }
  },
  "Key": "ability_punkgoat_tether",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    },
    "MoveSpeedSlowMinPct": {
      "Name": null,
      "Type": "move_speed",
      "Value": 25
    }
  },
  "Name": "Chain Gang",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "PullDistance": {
      "Name": null,
      "Value": 4
    },
    "PullDuration": {
      "Name": null,
      "Value": 0.8
    },
    "PullForceMax": {
      "Name": null,
      "Value": 2000
    },
    "PullTrackCasterDuration": {
      "Name": null,
      "Value": 0.5
    },
    "RopeSnapNoLOSDuration": {
      "Name": null,
      "Value": 0.5
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Range": {
    "RopeLength": {
      "Name": null,
      "Type": "distance",
      "Value": 2.0
    },
    "RopeSnapDistance": {
      "Name": null,
      "Type": "distance",
      "Value": 45.0
    },
    "RopeSoftEdgeLength": {
      "Name": null,
      "Type": "distance",
      "Value": 4.5
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "BulletResist": 40,
      "DescKey": "ability_punkgoat_tether_t1_desc",
      "TechResist": 40
    },
    {
      "AbilityCooldown": -40
    },
    {
      "AbilityCastRange": 5,
      "DescKey": "ability_punkgoat_tether_t3_desc",
      "UnstoppablePerHero": 1.3
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
      "lookup": "chain gang",
      "name": "Chain Gang",
      "type": "ability"
    }
  ]
}
````
