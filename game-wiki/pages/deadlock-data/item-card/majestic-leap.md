---
title: "Majestic Leap"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_rocket_booster"
canonical_name: "Majestic Leap"
snapshot_id: 40442
source_document_id: 7073
payload_hash: "e5a24be2e1bb0d606c36ee0abc6b7269361426355f638f16bbc5b1cce4f1ce18"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.598684+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Majestic Leap

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_rocket_booster`
- Snapshot ID: `40442`
- Source-Dokument: `7073`
- Kurzinfo: Majestic Leap aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Launch yourself</span> high into the air and grant yourself a <span class=\"highlight\">Barrier</span>. While in the air, you can use the active again to drop down faster.<br><br><span class=\"diminish\">Cannot be used for 5s if attacked by enemy Hero.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BarrierDuration",
        "Type": "duration",
        "Value": 8
      },
      {
        "Key": "AirControlPercentBarrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 50
      }
    ],
    "ChargeUp": null,
    "Cooldown": 45,
    "DescKey": "#upgrade_rocket_booster_desc",
    "Main": [
      {
        "Key": "CombatBarrier",
        "Scale": {
          "Type": "power_increase",
          "Value": 12.0
        },
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 200.0
      },
      {
        "Key": "InterruptCooldown",
        "Type": "cooldown",
        "Value": 5
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rocket_booster",
  "Name": "Majestic Leap",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "AirControlPercent": {
      "Key": "AirControlPercent",
      "UsageFlags": "ConditionallyApplied",
      "Value": 100
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DropDownSpeed": {
      "Key": "DropDownSpeed",
      "Value": "35m"
    },
    "ImpactHeight": {
      "Key": "ImpactHeight",
      "Value": "2m"
    },
    "JumpVelocityHidden": {
      "Key": "JumpVelocityHidden",
      "Value": "27m"
    },
    "MaxLandingSpeed": {
      "Key": "MaxLandingSpeed",
      "Value": "20m"
    },
    "MinAimAngle": {
      "Key": "MinAimAngle",
      "Value": 30
    },
    "SlamDownRadius": {
      "Key": "SlamDownRadius",
      "Type": "distance",
      "Value": "10m"
    },
    "SlowDuration": {
      "Key": "SlowDuration",
      "Type": "duration",
      "Value": 2.5
    },
    "SlowPercent": {
      "Key": "SlowPercent",
      "Type": "slow",
      "UsageFlags": "ConditionallyApplied",
      "Value": 40
    },
    "TossSpeed": {
      "Key": "TossSpeed",
      "Value": 500
    },
    "VerticalDifferenceTolerance": {
      "Key": "VerticalDifferenceTolerance",
      "Value": "2m"
    }
  },
  "ShopFilters": [
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -35,
    "CombatBarrier": 275,
    "InterruptCooldown": -3
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
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
      "hero_key": null,
      "hero_name": null,
      "lookup": "majestic leap",
      "name": "Majestic Leap",
      "type": "item"
    }
  ]
}
````
