---
title: "Luggage Cart"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_doorman_luggage_cart"
canonical_name: "Luggage Cart"
snapshot_id: 39777
source_document_id: 7071
payload_hash: "a9196095b2546352c02c926dc1491a7f86c99faa3d90fda1f54f5984505e0477"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.184274+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Luggage Cart

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_doorman_luggage_cart`
- Snapshot ID: `39777`
- Source-Dokument: `7071`
- Kurzinfo: Luggage Cart aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 25
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6
  },
  "DescKey": "ability_doorman_luggage_cart_desc",
  "HeroKey": "hero_doorman",
  "HeroName": "The Doorman",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_doorman_luggage_cart_desc",
    "Main": {
      "Props": [
        {
          "Key": "CartDamage",
          "Name": "Cart Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.75
          },
          "Type": "tech_damage",
          "Value": 80
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "ability_doorman_luggage_cart_t3_note",
    "Main": {
      "Props": [
        {
          "Key": "WallImpactDamage",
          "Name": "Impact Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0
          },
          "Title": "On Wall Hit",
          "Type": "tech_damage",
          "Value": 0
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Title": "On Wall Hit",
          "Type": "duration",
          "Value": 0
        }
      ]
    },
    "RequiresUpgradeIndex": 1
  },
  "Key": "ability_doorman_luggage_cart",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Luggage Cart",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "CartDamage": 60
    },
    {
      "AbilityCastRange": 20,
      "DescKey": "ability_doorman_luggage_cart_t2_desc",
      "WallImpactDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 75
      }
    },
    {
      "AbilityCooldown": -15,
      "DescKey": "ability_doorman_luggage_cart_t3_desc",
      "StunDuration": 1.25
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
      "hero_key": "hero_doorman",
      "hero_name": "The Doorman",
      "lookup": "luggage cart",
      "name": "Luggage Cart",
      "type": "ability"
    }
  ]
}
````
