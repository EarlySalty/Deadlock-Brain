---
title: "Vexing Bolt"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_magician_magicbolt"
canonical_name: "Vexing Bolt"
snapshot_id: 39847
source_document_id: 7071
payload_hash: "0117744ec23603c2dd61657ce4d14a6b0b1f7c5598fac5496d742090aaba4c18"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.320636+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Vexing Bolt

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_magician_magicbolt`
- Snapshot ID: `39847`
- Source-Dokument: `7071`
- Kurzinfo: Vexing Bolt aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Type": "range",
    "Value": 500
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 24
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 3
  },
  "DescKey": "ability_magician_magicbolt_desc",
  "HeroKey": "hero_magician",
  "HeroName": "Sinclair",
  "Info1": {
    "Alt": [
      {
        "Key": "MaxDamageTime",
        "Name": "Time for Max Damage",
        "Type": "duration",
        "Value": 2
      },
      {
        "Key": "FireRateSlow",
        "Name": "Fire Rate",
        "Type": "fire_rate",
        "Value": 0
      },
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_magician_magicbolt_desc",
    "Main": {
      "Props": [
        {
          "Key": "MinDamage",
          "Name": "Minimum Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.93
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "MaxDamage",
          "Name": "Max Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.86
          },
          "Type": "tech_damage",
          "Value": 120
        },
        {
          "Key": "CloneDamagePercentage",
          "Name": "Assistant Damage",
          "Type": "tech_damage",
          "Value": 50.0
        }
      ]
    }
  },
  "Key": "ability_magician_magicbolt",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Vexing Bolt",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.3
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CloneBoltDelay": {
      "Name": null,
      "Value": 0.1
    },
    "InitialProjectileVelocity": {
      "Name": null,
      "Value": 800
    },
    "ProjectileLifetime": {
      "Name": null,
      "Value": 4
    },
    "ProjectileRedirectCount": {
      "Name": "Max Redirects",
      "Value": 1
    },
    "RedirectVelocity": {
      "Name": null,
      "Value": 1500
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 3.25
  },
  "Slot": "1",
  "Upgrades": [
    {
      "DebuffDuration": 5,
      "DescKey": "ability_magician_magicbolt_t1_desc",
      "FireRateSlow": 25
    },
    {
      "AbilityCooldown": -13
    },
    {
      "CloneDamagePercentage": 50.0,
      "DescKey": "ability_magician_magicbolt_t3_desc",
      "MaxDamage": 126.0
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
      "hero_key": "hero_magician",
      "hero_name": "Sinclair",
      "lookup": "vexing bolt",
      "name": "Vexing Bolt",
      "type": "ability"
    }
  ]
}
````
