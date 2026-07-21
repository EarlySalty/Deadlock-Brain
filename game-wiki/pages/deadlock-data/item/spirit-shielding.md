---
title: "Spirit Shielding"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_spirit_bubble"
canonical_name: "Spirit Shielding"
snapshot_id: 40183
source_document_id: 7072
payload_hash: "659548580220c290ba528264527a5910741f0cb853840cd586f0eebb82ff6b91"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.107407+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Spirit Shielding

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_spirit_bubble`
- Snapshot ID: `40183`
- Source-Dokument: `7072`
- Kurzinfo: Spirit Shielding aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 45,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BarrierDuration": 8,
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "power_increase",
      "Value": 5
    },
    "Value": 300
  },
  "Components": [
    "upgrade_grit"
  ],
  "Cost": 1600,
  "DamageThreshold": 225,
  "DamageWindow": 3.5,
  "Description": "Gain a <span class=\"highlight\">Barrier</span> whenever you take significant {g:citadel_inline_attribute:'SpiritDamage'} from enemy Heroes in a small time frame.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spirit_bubble",
  "Name": "Spirit Shielding",
  "OutOfCombatHealthRegen": 2.5,
  "PropertyUpgrades": {
    "AbilityCooldown": -20,
    "CombatBarrier": 175,
    "OutOfCombatHealthRegen": 3,
    "TechResist": 20
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechResist": 18,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-data.json",
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
      "lookup": "spirit shielding",
      "name": "Spirit Shielding",
      "type": "item"
    }
  ]
}
````
