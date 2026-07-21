---
title: "Decay"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_rupture"
canonical_name: "Decay"
snapshot_id: 40166
source_document_id: 7072
payload_hash: "fe3eb2646428090f80eb211240c531d996531826890c482d4fdb39a1637a5540"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.057402+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Decay

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_rupture`
- Snapshot ID: `40166`
- Source-Dokument: `7072`
- Kurzinfo: Decay aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.1
    },
    "Value": "20m"
  },
  "AbilityCooldown": 30.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 10,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusHealth": 65,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "Inflict <span class=\"highlight\">damage over time</span> to a target, dealing damage based on their current health.<br>Decay's damage is non-lethal and does not apply item procs.",
  "DotHealthPercent": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.004
    },
    "Value": 2.6
  },
  "HealAmpReceivePenaltyPercent": -50,
  "HealAmpRegenPenaltyPercent": -50,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rupture",
  "Name": "Decay",
  "PropertyUpgrades": {
    "AbilityCooldown": -10,
    "BonusHealth": 90,
    "DotHealthPercent": 0.5,
    "HealAmpReceivePenaltyPercent": -20,
    "HealAmpRegenPenaltyPercent": -20,
    "TechPower": 12
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "TechPower": 8,
  "TickRate": 1.0,
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
      "lookup": "decay",
      "name": "Decay",
      "type": "item"
    }
  ]
}
````
