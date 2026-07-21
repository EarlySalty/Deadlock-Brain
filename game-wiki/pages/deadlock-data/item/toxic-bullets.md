---
title: "Toxic Bullets"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_toxic_bullets"
canonical_name: "Toxic Bullets"
snapshot_id: 40209
source_document_id: 7072
payload_hash: "468457b61b170a5ecb993231398450290f6ad22b8bfe7c9a0bf8bd24a416de06"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.175593+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Toxic Bullets

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_toxic_bullets`
- Snapshot ID: `40209`
- Source-Dokument: `7072`
- Kurzinfo: Toxic Bullets aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BuildUpDuration": 5,
  "BuildUpPerShot": 1.28,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "Your bullets build up a <span class=\"highlight\">Bleed</span> on enemies, causing them to lose a <span class=\"highlight\">percentage</span> of their <span class=\"highlight\">Max Health</span> over time. Also applies <span class=\"highlight\">Healing Reduction</span> on the bleeding target.",
  "DotDuration": 4,
  "DotHealthPercent": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.005
    },
    "Value": 1.9
  },
  "DotMultiplerTroopers": 0.5,
  "HealAmpReceivePenaltyPercent": -35,
  "HealAmpRegenPenaltyPercent": -35,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_toxic_bullets",
  "Name": "Toxic Bullets",
  "PropertyUpgrades": {
    "DotHealthPercent": 0.7,
    "HealAmpReceivePenaltyPercent": -30,
    "HealAmpRegenPenaltyPercent": -30
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy",
    "MinionEnemy"
  ],
  "TickRate": 0.5,
  "Tier": 3,
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
      "lookup": "toxic bullets",
      "name": "Toxic Bullets",
      "type": "item"
    }
  ]
}
````
