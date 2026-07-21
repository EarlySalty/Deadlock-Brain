---
title: "Hex-Lined Snap Trap"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_priest_beartrap"
canonical_name: "Hex-Lined Snap Trap"
snapshot_id: 39873
source_document_id: 7071
payload_hash: "de48707d95305f8fe4eae3cc5d275ae8dc20ac933ce267e40147e80cd9b9ba73"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.370890+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Hex-Lined Snap Trap

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_priest_beartrap`
- Snapshot ID: `39873`
- Source-Dokument: `7071`
- Kurzinfo: Hex-Lined Snap Trap aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 28
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 8
  },
  "DescKey": "ability_priest_beartrap_desc",
  "Duration": {
    "ArmTime": {
      "Name": "Arm Time",
      "Type": "duration",
      "Value": 0.5
    }
  },
  "HeroKey": "hero_priest",
  "HeroName": "Venator",
  "Info1": {
    "Alt": [
      {
        "Key": "IncomingDamagePercentFromCaster",
        "Name": "Damage Taken",
        "Type": "damage",
        "Value": 0
      },
      {
        "Key": "RevealDuration",
        "Name": "Reveal Duration",
        "Type": "duration",
        "Value": 6
      },
      {
        "Key": "Lifetime",
        "Name": "Lifetime",
        "Type": "duration",
        "Value": 30
      }
    ],
    "DescKey": "ability_priest_beartrap_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.2
          },
          "Type": "tech_damage",
          "Value": 80
        },
        {
          "Key": "ImmobilizeDuration",
          "Name": "Immobilize Duration",
          "StatusEffect": "Immobilize",
          "Type": "duration",
          "Value": 1.25
        }
      ]
    }
  },
  "Key": "ability_priest_beartrap",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Hex-Lined Snap Trap",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TetherDuration": {
      "Name": "Tether Duration",
      "Value": 0.6
    },
    "TetherRadius": {
      "Name": "Tether Radius",
      "Value": 0.3
    },
    "TickRate": {
      "Name": null,
      "Value": 0.1
    },
    "TrapHeight": {
      "Name": null,
      "Value": 2
    },
    "TripUpSpeed": {
      "Name": null,
      "Value": 6.35
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 2
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -11
    },
    {
      "ImmobilizeDuration": 1.0
    },
    {
      "AbilityCharges": 1,
      "DescKey": "ability_priest_beartrap_t3_desc",
      "IncomingDamagePercentFromCaster": 30
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
      "hero_key": "hero_priest",
      "hero_name": "Venator",
      "lookup": "hex-lined snap trap",
      "name": "Hex-Lined Snap Trap",
      "type": "ability"
    }
  ]
}
````
