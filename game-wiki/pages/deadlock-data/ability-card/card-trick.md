---
title: "Card Trick"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_card_toss"
canonical_name: "Card Trick"
snapshot_id: 39943
source_document_id: 7071
payload_hash: "88f0f6fff0355ee83e6bb38a7468f6cdcf3c1dac289e4d0ebf7fc3576a96271d"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.505156+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Card Trick

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_card_toss`
- Snapshot ID: `39943`
- Source-Dokument: `7071`
- Kurzinfo: Card Trick aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Scale": {
      "Type": "range",
      "Value": 0.0
    },
    "Type": "range",
    "Value": 500
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Scale": {
      "Type": "cooldown",
      "Value": 0.0
    },
    "Type": "cooldown",
    "Value": 0.6
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_card_toss_desc",
  "Duration": {
    "ClubSlowDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 3
    }
  },
  "HeroKey": "hero_wraith",
  "HeroName": "Wraith",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_card_toss_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.55
          },
          "Type": "tech_damage",
          "Value": 45
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "ability_header_card_toss_suits_desc",
    "Main": {
      "Props": [
        {
          "Key": "SpadeDamageBonus",
          "Name": "Bonus Damage",
          "Title": "Spade",
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "DiamondResistShred",
          "Name": "Bullet and Spirit Resist Reduction",
          "Title": "Diamond",
          "Type": "bullet_armor_down",
          "Value": -8.0
        }
      ]
    }
  },
  "Info3": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "ClubSlowPercent",
          "Name": "Movement Slow",
          "Title": "Club",
          "Type": "slow",
          "Value": -30
        },
        {
          "Key": "HeartHeal",
          "Name": "Heal",
          "Scale": {
            "Type": "spirit",
            "Value": 1.0
          },
          "Title": "Heart",
          "Type": "healing",
          "Value": 75
        }
      ]
    }
  },
  "Key": "citadel_ability_card_toss",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Card Trick",
  "Other": {
    "AbilityChargesConditionally": {
      "Name": null,
      "Value": 1
    },
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.1
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BonusAbilityResource": {
      "Name": null,
      "Value": 100
    },
    "CardResourceGenPctScale": {
      "Name": "Card Summon Rate",
      "Scale": {
        "Type": "cooldown",
        "Value": -1.0
      },
      "Value": 85
    },
    "CardResourcePerBulletCrit": {
      "Name": null,
      "Value": 6
    },
    "CardResourcePerBulletHit": {
      "Name": null,
      "Value": 4
    },
    "CardResourcePerHeavyMelee": {
      "Name": null,
      "Value": 25
    },
    "CardResourcePerLightMelee": {
      "Name": null,
      "Value": 10
    },
    "CooldownBetweenCards": {
      "Name": null,
      "Value": 0.5
    },
    "DiamondResistShredDuration": {
      "Name": null,
      "Value": 5
    },
    "HeartHealNonHeroRatio": {
      "Name": null,
      "Value": 0.5
    },
    "NonPlayerCardResourceScale": {
      "Name": null,
      "Value": 0.35
    },
    "ProjectileOriginHeightOffset": {
      "Name": null,
      "Value": 50
    },
    "ResourcePerCard": {
      "Name": null,
      "Value": 100
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 4
  },
  "Range": {
    "JokerExtraCardSearchRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 20
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCharges": 2
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.4
        },
        "Value": 40
      },
      "DescKey": "citadel_ability_card_toss_t2_desc"
    },
    {
      "ClubSlowPercent": -20,
      "DescKey": "citadel_ability_card_toss_t3_desc",
      "DiamondResistShred": -5,
      "HeartHeal": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 75
      },
      "ImprovedJokerChance": 1,
      "SpadeDamageBonus": 40
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
      "hero_key": "hero_wraith",
      "hero_name": "Wraith",
      "lookup": "card trick",
      "name": "Card Trick",
      "type": "ability"
    }
  ]
}
````
