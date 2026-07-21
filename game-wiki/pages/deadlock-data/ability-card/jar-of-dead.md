---
title: "Jar of Dead"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_necro_hauntingskull"
canonical_name: "Jar of Dead"
snapshot_id: 39859
source_document_id: 7071
payload_hash: "585ce929f63beccbad7ee438ab211a9daa0d19216fba7e9a6fa5ec388a7bd559"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.344003+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Jar of Dead

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_necro_hauntingskull`
- Snapshot ID: `39859`
- Source-Dokument: `7071`
- Kurzinfo: Jar of Dead aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 4
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 13
  },
  "Cast": {
    "ResourceCost": {
      "Name": null,
      "Scale": {
        "Type": "cooldown",
        "Value": 1.0
      },
      "Type": "cast",
      "Value": 120
    }
  },
  "DescKey": "ability_necro_hauntingskull_desc",
  "Duration": {
    "SkullLifetime": {
      "Name": "Deadhead Lifetime",
      "Type": "duration",
      "Value": 10
    }
  },
  "Health": {
    "SummonHealth": {
      "Name": "Summon Health",
      "Scale": {
        "Type": "power_increase",
        "Value": 1.3
      },
      "Type": "health",
      "Value": 20
    }
  },
  "HeroKey": "hero_necro",
  "HeroName": "Graves",
  "Info1": {
    "Alt": [
      {
        "Key": "HealOnHit",
        "Value": 0
      },
      {
        "Key": "TargetSearchRadius",
        "Name": "Search Range",
        "Type": "distance",
        "Value": 7
      },
      {
        "Key": "TargetDashRadius",
        "Name": "Dash Range",
        "Scale": {
          "Type": "range",
          "Value": 1.0
        },
        "Type": "distance",
        "Value": 15
      },
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 0
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Value": 0
      },
      {
        "Key": "HealPerPickup",
        "Name": "Heal On Pickup",
        "Scale": {
          "Type": "spirit",
          "Value": 0.0
        },
        "Type": "healing",
        "Value": 0
      }
    ],
    "DescKey": "ability_necro_hauntingskull_desc",
    "Main": {
      "Props": [
        {
          "Key": "SkullCount",
          "Name": "Deadheads",
          "Type": "cast",
          "Value": 4
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.25
          },
          "Type": "tech_damage",
          "Value": 16
        }
      ]
    }
  },
  "Key": "ability_necro_hauntingskull",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Jar of Dead",
  "Other": {
    "AbilityChargesConditionally": {
      "Name": null,
      "Value": 1
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DelayBeforeRespawning": {
      "Name": null,
      "Value": 1
    },
    "KillTime": {
      "Name": null,
      "Value": 0.2
    },
    "MaxHits": {
      "Name": "Max Hits",
      "Value": -1
    },
    "PickupsPerBossDeath": {
      "Name": null,
      "Value": 5
    },
    "PickupsPerDeath": {
      "Name": null,
      "Value": 1
    },
    "PickupsPerHeroDeath": {
      "Name": null,
      "Value": 5
    },
    "PickupsPerNeutralTrooperDeath": {
      "Name": null,
      "Value": 2
    },
    "ResourceGenerationPercent": {
      "Name": null,
      "Value": 100
    },
    "ResourcePerPickup": {
      "Name": null,
      "Value": 10
    },
    "ResourceRadius": {
      "Name": null,
      "Value": 40
    },
    "SkullImmuneDuration": {
      "Name": null,
      "Value": 0.15
    },
    "SkullKillGold": {
      "Name": null,
      "Scale": {
        "Type": "power_increase",
        "Value": 0.5
      },
      "Value": 7
    },
    "SpawnRadius": {
      "Name": null,
      "Value": 2
    },
    "SummonTakesDamage": {
      "Name": null,
      "Value": 1
    },
    "TargetSearchDelayMax": {
      "Name": null,
      "Value": 1.25
    },
    "TargetSearchDelayMin": {
      "Name": null,
      "Value": 1.5
    },
    "TargetSearchInitialDelayMax": {
      "Name": null,
      "Value": 0.2
    },
    "TargetSearchInitialDelayMin": {
      "Name": null,
      "Value": 0.15
    },
    "TargetSearchInitialStagger": {
      "Name": null,
      "Value": 0.125
    },
    "TickRate": {
      "Name": null,
      "Value": 0.2
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "DescKey": "ability_necro_hauntingskull_t1_desc",
      "HealPerPickup": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.1
        },
        "Value": 5
      }
    },
    {
      "DescKey": "ability_necro_hauntingskull_t2_desc",
      "SlowDuration": 1,
      "SlowPercent": 30
    },
    {
      "DescKey": "ability_necro_hauntingskull_t3_desc",
      "SkullCount": 2,
      "SkullLifetime": 4
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
      "hero_key": "hero_necro",
      "hero_name": "Graves",
      "lookup": "jar of dead",
      "name": "Jar of Dead",
      "type": "ability"
    }
  ]
}
````
