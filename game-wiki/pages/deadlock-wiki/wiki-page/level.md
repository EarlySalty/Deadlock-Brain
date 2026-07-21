---
title: "Level"
entity_type: "wiki_page"
source: "deadlock_wiki"
external_id: "Level"
canonical_name: "Level"
snapshot_id: 1637
source_document_id: 120
payload_hash: "b7b37cb768f3260a9a56100cc7e603f3de2c210e9bd7cf7ceaab6be86b2cb486"
source_content_hash: "b7b37cb768f3260a9a56100cc7e603f3de2c210e9bd7cf7ceaab6be86b2cb486"
source_url: "https://deadlock.wiki/api.php?action=query&format=json&prop=extracts%7Crevisions&explaintext=1&rvprop=ids%7Ctimestamp%7Ccontent&rvslots=main&titles=Level&redirects=1"
source_raw_path: "data/raw/deadlock_wiki/Level.b7b37cb768f3260a.json"
fetched_at: "2026-05-02T14:38:48+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "wiki_page"]
---

# Level

## Kurzueberblick

- Typ: `wiki_page`
- Quelle: `deadlock_wiki`
- External ID: `Level`
- Snapshot ID: `1637`
- Source-Dokument: `120`
- Kurzinfo: Level aus `deadlock_wiki` / `wiki_page` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "batchcomplete": "",
  "query": {
    "pages": {
      "799": {
        "extract": "Boons in Deadlock indicate the player's power level over the course of the match. Players gain boons by gathering   Souls, which unlocks Ability Points and increases their Weapon Damage, Melee Damage, Health and Spirit Power. The soul count is always visible to all players under their portrait, and the boon level can be seen in the soul container icon next to the player's soul count, and over the player's portrait by pressing Tab.\nThe maximum Boon level is 35.\n\n\n== Leveling Up ==\nLeveling up in Deadlock is based on a hero's total gathered souls and grants  Boons,  Ability Unlocks and  Ability Points. The first three ability unlocks can only be used on non-ultimate abilities. The ultimate can be unlocked at  3.6k.\n\n\n=== Reward breakdown ===\nAll heroes level up at the same souls-gathered thresholds, in the following pattern:\n\nEvery level-up grants a boon.\nLevels 0, 2, 4 and 6 grant an ability unlock. Every other level grants an ability point (up to 32).\nThe following table details the rewards given at each threshold.\n\n\n== Ability Points ==\nAbility points are obtained as level up rewards. Each ability has 3 tiers, requiring 1, 2, and then 5 ability points to unlock. It is possible to spend ability points in any order, including saving them to spend on an ultimate ability or for the final level of an ability.\n\n\n== Boon Rewards ==\n\nBoon Rewards serve to increase the base attributes of the hero. Level ups always grant a Boon Reward.\nThe base attributes that are improved by Boons are: base bullet damage, base melee damage, spirit power, and base health. Each hero gains unique amounts of base stats from their Boons. The differences in these boon profiles can dictate which heroes are more powerful in the early stages of a game and which heroes become powerful in the late game. Some heroes may have additional stats improved by Boons, such as  Dynamo's bullet resist.\n\n\n== Boon Tables ==\nSee also Hero Attributes Table for dynamic boon stats.",
        "ns": 0,
        "pageid": 799,
        "revisions": [
          {
            "parentid": 64844,
            "revid": 67377,
            "slots": {
              "main": {
                "*": "[[File:Boon counter.png|thumb|Boon counter HUD icon.|159x159px]]\n[[File:Boon portrait.png|thumb|Infernus at boon level 1.]]\n[[File:Level up.png|thumb|Level up notification, rewarding an '''Ultimate Unlock''' and a '''Power Increase'''.]]\n'''Boons''' in ''[[Deadlock]]'' indicate the player's power level over the course of the match. Players gain boons by gathering {{Souls}} [[Souls]], which unlocks [[Ability Points]] and increases their [[Weapon Damage]], [[Melee Damage]], [[Health]] and [[Spirit Power]]. The soul count is always visible to all players under their portrait, and the boon level can be seen in the soul container icon next to the player's soul count, and over the player's portrait by pressing Tab.\n\nThe maximum Boon level is {{#invoke:SoulUnlock|get_max|PowerIncrease}}.\n\n== Leveling Up ==\n'''Leveling up''' in ''Deadlock'' is based on a hero's total gathered souls and grants [[File:Boon_icon.png|24px]] '''Boons''', [[File:Ability_unlock_icon.png|24px]] '''[[Ability]] Unlocks''' and [[File:Ability point unlock icon.png|24px]] '''[[Ability Points]]'''. The first three ability unlocks can only be used on non-ultimate abilities. The ultimate can be unlocked at {{Souls|3.6k}}.\n\n=== Reward breakdown ===\nAll heroes level up at the same souls-gathered thresholds, in the following pattern:\n* Every level-up grants a boon.\n* Levels 0, 2, 4 and 6 grant an ability unlock. Every other level grants an ability point (up to {{#invoke:SoulUnlock|get_max|AbilityPoints}}).\n\nThe following table details the rewards given at each threshold.\n\n<!--See [[Module:SoulUnlock]]'s write_table function-->\n{{#invoke:SoulUnlock|write_table}}\n\n== Ability Points ==\n[[Ability Point|Ability points]] are obtained as level up rewards. Each ability has 3 tiers, requiring 1, 2, and then 5 ability points to unlock. It is possible to spend ability points in any order, including saving them to spend on an ultimate ability or for the final level of an ability.\n\n== Boon Rewards ==\n<span id=\"Boon\"></span>\n[[File:Power_increase.png|300px|thumb|Haze's boon profile. (Image for reference only, numbers may be outdated)]]\n'''Boon Rewards''' serve to increase the base attributes of the hero. Level ups always grant a Boon Reward.\n\nThe base attributes that are improved by Boons are: base [[Bullet Damage|bullet damage]], base [[Melee Damage|melee damage]], [[Spirit Power|spirit power]], and base [[health]]. Each hero gains unique amounts of base stats from their Boons. The differences in these boon profiles can dictate which heroes are more powerful in the early stages of a game and which heroes become powerful in the late game. Some heroes may have additional stats improved by Boons, such as {{HeroIcon|Dynamo}}'s [[bullet resist]].\n{{clear}}\n\n== Boon Tables ==\n''See also [[Hero Attributes Table]] for dynamic boon stats.''{{#invoke:LevelTables|write_power_increase_table|BulletDamage|title=Base Bullet Damage based on Boons}}\n\n{{#invoke:LevelTables|write_power_increase_table|MaxHealth|title=Base Health based on Boons}}\n\n{{#invoke:LevelTables|write_power_increase_table|LightMeleeDamage|title=Base Light Melee Damage based on Boons}}\n\n{{#invoke:LevelTables|write_power_increase_table|HeavyMeleeDamage|title=Base Heavy Melee Damage based on Boons}}\n\n{{#invoke:LevelTables|write_power_increase_table|TechPower|title=Base Spirit Power based on Boons}}\n\n{{Navbox gameplay}}",
                "contentformat": "text/x-wiki",
                "contentmodel": "wikitext"
              }
            },
            "timestamp": "2026-05-01T19:01:37Z"
          }
        ],
        "title": "Boon"
      }
    },
    "redirects": [
      {
        "from": "Level",
        "to": "Boon"
      }
    ]
  }
}
````
