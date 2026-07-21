---
title: "Spirit Strike"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_acolytes_glove"
canonical_name: "Spirit Strike"
snapshot_id: 39979
source_document_id: 7072
payload_hash: "347c352726986b72a0af1ca57e18261caf1e6a310b502491aebfd7b6ce33b20c"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.589937+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Spirit Strike

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_acolytes_glove`
- Snapshot ID: `39979`
- Source-Dokument: `7072`
- Kurzinfo: Spirit Strike aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 8,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": "When you perform a <span class=\"highlight\">Light or Heavy Melee</span> attack against a hero, deal extra {g:citadel_inline_attribute:'SpiritDamage'} with the attack and reduce the target's <span class=\"highlight\">Spirit Resist</span>.<span class=\"diminish\"><br><br>Cooldown is 2x longer for Light Melee hits.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_acolytes_glove",
  "LightMeleeCooldownMult": 2,
  "Name": "Spirit Strike",
  "PropertyUpgrades": {
    "SpiritDamage": 80,
    "TechArmorDamageReduction": -5
  },
  "ShopFilters": [
    "MagicDamage",
    "Melee"
  ],
  "Slot": "Tech",
  "SpiritDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.372
    },
    "Value": 40
  },
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "TechArmorDamageReduction": -6,
  "Tier": 1,
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
      "lookup": "spirit strike",
      "name": "Spirit Strike",
      "type": "item"
    }
  ]
}
````
