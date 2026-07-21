---
title: "Barrage"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "synth_barrage"
canonical_name: "Barrage"
snapshot_id: 39899
source_document_id: 7071
payload_hash: "a0c42b0cc63850b498d14499316a3266b5c768a726a5d530035daf9253a23ef5"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.421468+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Barrage

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `synth_barrage`
- Snapshot ID: `39899`
- Source-Dokument: `7071`
- Kurzinfo: Barrage aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.3
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 32.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "synth_barrage_desc",
  "HeroKey": "hero_synth",
  "HeroName": "Pocket",
  "Info1": {
    "Alt": [
      {
        "Key": "ProjectileAmount",
        "Name": "Projectile Amount",
        "Value": 4
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 1.5
      },
      {
        "Key": "AmpDuration",
        "Name": "Amp Duration",
        "Type": "duration",
        "Value": 15
      }
    ],
    "DescKey": "synth_barrage_desc",
    "Main": {
      "Props": [
        {
          "Key": "AmpPercentPerStack",
          "Name": "Amp Per Stack",
          "Type": "damage",
          "Value": 6
        },
        {
          "Key": "DamagePerProjectile",
          "Name": "Damage Per Projectile",
          "Scale": {
            "Type": "spirit",
            "Value": 0.465
          },
          "Type": "tech_damage",
          "Value": 32
        },
        {
          "Key": "MoveSlowPercent",
          "Name": "Movement Slow",
          "Type": "slow",
          "Value": 30
        }
      ]
    }
  },
  "Key": "synth_barrage",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Barrage",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirDrag": {
      "Name": null,
      "Value": 0.3
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 2.54
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 10
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 4.5
  },
  "Slot": "1",
  "Upgrades": [
    {
      "DamagePerProjectile": 16
    },
    {
      "AbilityCooldown": -16.0
    },
    {
      "AmpPercentPerStack": 4,
      "DescKey": "synth_barrage_t3_desc",
      "Radius": 3
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
      "hero_key": "hero_synth",
      "hero_name": "Pocket",
      "lookup": "barrage",
      "name": "Barrage",
      "type": "ability"
    }
  ]
}
````
