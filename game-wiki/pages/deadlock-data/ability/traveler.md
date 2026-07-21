---
title: "Traveler"
entity_type: "ability"
source: "deadlock_data"
external_id: "mirage_teleport"
canonical_name: "Traveler"
snapshot_id: 39694
source_document_id: 7070
payload_hash: "2b57d01b755bbb334f1dfa84be64bdbd09daadf90be535b70121c61483a99c63"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.969450+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Traveler

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `mirage_teleport`
- Snapshot ID: `39694`
- Source-Dokument: `7070`
- Kurzinfo: Traveler aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 140.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorCanCastOnZipline",
    "BehaviorCastableWhileBusy",
    "BehaviorRequireAbilityButtonToCancel",
    "BehaviorCanCancelDuringCastDelay"
  ],
  "BuffModifier": {
    "Class": "MirageTravelerMovementSpeed",
    "Subclass": "MirageTeleportMovementSpeed"
  },
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "ImmunityModifier": {
    "Class": "Unstoppable",
    "DisabledStateMask": [
      "Disarmed",
      "Muted",
      "Silenced",
      "SilenceMovementAbilites",
      "Slowed",
      "Glitched",
      "MeleeDisabledDebuff",
      "DashDisabledDebuff"
    ],
    "EnabledStateMask": [
      "StatusImmune",
      "SlowImmune",
      "KnockdownImmune",
      "Unstoppable"
    ],
    "StatusEffectPriority": 0,
    "Subclass": "Unstoppable"
  },
  "InterruptCooldown": 4,
  "InterruptNotificationModifier": {
    "Class": "Base",
    "Duration": 2.0,
    "Subclass": "Notification"
  },
  "IsDisabled": false,
  "Key": "mirage_teleport",
  "Name": "Traveler",
  "SearchRadius": 30,
  "TeleportCompletedTime": 2,
  "Upgrades": [
    {
      "BonusFireRate": 20,
      "BonusMoveSpeed": 3,
      "MovementSpeedBonusDuration": 12
    },
    {
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 400
      }
    },
    {
      "AbilityCooldown": -70
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-data.json",
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
  "_deadlock_data_card": {
    "card_name": "Traveler",
    "hero_key": "hero_mirage",
    "hero_name": "Mirage",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_mirage",
      "hero_name": "Mirage",
      "lookup": "traveler",
      "name": "Traveler",
      "type": "ability"
    }
  ]
}
````
