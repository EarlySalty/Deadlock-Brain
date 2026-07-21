---
title: "Bloodscent"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_drifter_hunger"
canonical_name: "Bloodscent"
snapshot_id: 39781
source_document_id: 7071
payload_hash: "fcbce6ff00335e4a3889ae088bb7bce3a433c0e23fb08e20f9d18d017abaa142"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.192545+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Bloodscent

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_drifter_hunger`
- Snapshot ID: `39781`
- Source-Dokument: `7071`
- Kurzinfo: Bloodscent aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 80
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_drifter_hunger_desc",
  "Duration": {
    "InvisDuration": {
      "Name": "Invisibility Duration",
      "Scale": {
        "Type": "spirit",
        "Value": 0.0
      },
      "Type": "duration",
      "Value": 0
    },
    "KillDuration": {
      "Name": "Kill Duration",
      "Type": "duration",
      "Value": 300
    },
    "TrailDuration": {
      "Name": "Trail Duration",
      "Type": "duration",
      "Value": 10
    }
  },
  "Health": {
    "LowHealthThreshold": {
      "Name": "Health Threshold",
      "Type": "health",
      "Value": 30
    }
  },
  "HeroKey": "hero_drifter",
  "HeroName": "Drifter",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusMoveSpeed",
        "Name": "Move Speed",
        "Type": "move_speed",
        "Value": 0
      }
    ],
    "DescKey": "ability_drifter_hunger_passive_desc",
    "Main": {
      "Props": [
        {
          "Key": "AmpDamagePercent",
          "Name": "Amplified Damage",
          "Type": "damage",
          "Value": 15
        },
        {
          "Key": "IsolationRange",
          "Name": "Isolation Range",
          "Type": "distance",
          "Value": 20
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "WeaponDmgPerIsolationKill",
          "Name": "Permanent Weapon Damage",
          "Title": "On Isolated Hero Death:",
          "Type": "bullet_damage",
          "Value": 3
        },
        {
          "Key": "HealOnKillPct",
          "Name": "Missing Health as Healing",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Title": "On Isolated Hero Death:",
          "Type": "healing",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_drifter_hunger",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Bloodscent",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DelayBeforeInvisStarts": {
      "Name": null,
      "Value": 0.6
    },
    "InvisFadeToDuration": {
      "Name": "Fade Time",
      "Value": 0.3
    },
    "IsolationAssistPercentValue": {
      "Name": null,
      "Value": 100
    },
    "MaxTrailTargets": {
      "Name": "Max Trail Targets",
      "Value": 2
    },
    "RevealOnDamageDuration": {
      "Name": null,
      "Value": 0.25
    },
    "RevealOnSpottedDuration": {
      "Name": null,
      "Value": 1.5
    },
    "SpottedRadius": {
      "Name": "Spot Radius",
      "Value": 15
    },
    "TargetLingerDuration": {
      "Name": null,
      "Value": 3
    },
    "TickRate": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BonusMoveSpeed": 3,
      "DescKey": "ability_drifter_hunger_t1_desc"
    },
    {
      "DescKey": "ability_drifter_hunger_t2_desc",
      "HealOnKillPct": 24,
      "StaminaToRestore": 2
    },
    {
      "AmpDamagePercent": 12.0
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
      "lookup": "bloodscent",
      "name": "Bloodscent",
      "type": "ability"
    }
  ]
}
````
