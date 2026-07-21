---
title: "Shadow Strike"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_shadow_strike"
canonical_name: "Shadow Strike"
snapshot_id: 40169
source_document_id: 7072
payload_hash: "84f4c114eefe18b86301e1893d624444d46c403363bf9bcbfbe7675662485ef5"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.065624+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Shadow Strike

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_shadow_strike`
- Snapshot ID: `40169`
- Source-Dokument: `7072`
- Kurzinfo: Shadow Strike aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 3,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 350,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.4
    },
    "Value": 125
  },
  "Description": "Go <span class=\"highlight\">Invisible</span> on <span class=\"highlight\">Stamina use</span> with no detection range. Doing a <span class=\"highlight\">melee attack</span> while invisible will cause you to <span class=\"highlight\">steal bullet and spirit resistance</span> from them and deal <span class=\"highlight\">damage over time</span>.",
  "InvisAlertWhenFading": 1,
  "InvisFadeToDuration": 0.2,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_shadow_strike",
  "MaxStacks": 1,
  "Name": "Shadow Strike",
  "PropertyUpgrades": {
    "AbilityDuration": 3,
    "BonusHealth": 250,
    "DPS": 125,
    "ResistStealAmount": 20,
    "Stamina": 1
  },
  "ResistStealAmount": 40,
  "RevealOnDamageDuration": 1.5,
  "RevealOnSpottedDuration": 0.25,
  "ShopFilters": null,
  "Slot": "Armor",
  "SpottedRadius": "0m",
  "Stamina": 3,
  "StealDuration": 6,
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TickRate": 1,
  "Tier": 5,
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
      "lookup": "shadow strike",
      "name": "Shadow Strike",
      "type": "item"
    }
  ]
}
````
