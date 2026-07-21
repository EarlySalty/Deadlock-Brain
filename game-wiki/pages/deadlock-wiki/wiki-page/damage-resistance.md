---
title: "Damage Resistance"
entity_type: "wiki_page"
source: "deadlock_wiki"
external_id: "Damage Resistance"
canonical_name: "Damage Resistance"
snapshot_id: 1641
source_document_id: 124
payload_hash: "d7e459a192591c76566d4df21689683b8c4732278eca1f9de2a9a0ff3e220cf5"
source_content_hash: "d7e459a192591c76566d4df21689683b8c4732278eca1f9de2a9a0ff3e220cf5"
source_url: "https://deadlock.wiki/api.php?action=query&format=json&prop=extracts%7Crevisions&explaintext=1&rvprop=ids%7Ctimestamp%7Ccontent&rvslots=main&titles=Damage+Resistance&redirects=1"
source_raw_path: "data/raw/deadlock_wiki/Damage_Resistance.d7e459a192591c76.json"
fetched_at: "2026-05-02T14:57:36+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "wiki_page"]
---

# Damage Resistance

## Kurzueberblick

- Typ: `wiki_page`
- Quelle: `deadlock_wiki`
- External ID: `Damage Resistance`
- Snapshot ID: `1641`
- Source-Dokument: `124`
- Kurzinfo: Damage Resistance aus `deadlock_wiki` / `wiki_page` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "batchcomplete": "",
  "query": {
    "pages": {
      "922": {
        "extract": "Damage Resistance refers to the Bullet, Melee, and Spirit Resist statistics which reduce incoming damage taken. Bullet Resist reduces all weapon and melee damage, Melee Resist reduces only melee damage, while Spirit Resist reduces all spirit damage. Units have a unique \"Damage Resistance\" stat that is applied to all sources of damage, including Troopers.\nMost resistances stack multiplicatively; they are not added together. Sources of Resistance Reduction are multiplied together separately first. The total reduction is then subtracted from the total resistance. This has the consequence of resistance reduction being more effective on targets with higher resistance.\n\n\n== Resistance types ==\nIn addition to the basic types of Bullet and Spirit Resistance, there is also Melee Resist and Crit Reduction.\n\n\n=== Melee Resist ===\nMelee Resist is a stat that reduces all incoming melee damage, including from melee abilities such as  Viscous'  Puddle Punch. It increases equally to Bullet Resist (e.g. an item that gives 10% Bullet Resist also gives 10% Melee Resist). Some items also give bonus Melee Resist. Like other types of resist, it stacks multiplicatively.\n\n\n=== Crit Reduction ===\nCrit Reduction is a stat that reduces the critical damage (headshots) received by the hero. Not to be confused with Crit Bonus Scale, a stat that reduces the critical damage dealt by the hero to other targets.\n\n\n== Starting Damage Resistance ==\nThe following Heroes either have starting Damage Resistance values or gain resistance with Boons or Spirit Power. Boon and Spirit scaling resistances stack additively.\nNote: Negative values indicate that the hero receives increased damage.\n\n\n== Calculation ==\nWhile items often display resistance as a flat percentage (e.g., +20% Resist), multiple sources of resistance do not simply add together. Instead, they stack multiplicatively.\n\n\n=== Calculating Resistance ===\nTo find your total resistance, use the following formula:\nTotal Resist=1−(1−R1)×(1−R2)…\nExample:\nIf you have two sources of resistance, one providing 40% and another providing 20%:\n\nConvert percentages to decimals: 0.40 and 0.20.\nSubtract from 1: (1 - 0.40) = 0.60 and (1 - 0.20) = 0.80.\nMultiply the remainders: 0.60 × 0.80 = 0.48.\nSubtract from 1: 1 - 0.48 = 0.52.\nYour Total Resist is 52%.\n\n\n=== Calculating Resistance Reduction ===\nResistance Reduction (often called \"shred\") acts as a counter to resistance. It is calculated separately using the same multiplicative formula, and then subtracted from the target's resistance.\nFinal Resist=Total Resistance−Total Reduction\nExample:\nA target has 52% Total Resistance (as calculated above). You attack them with two sources of Reduction: one providing 25% and another providing 20%.\nStep 1: Calculate Total Reduction\n1−(1−0.25)×(1−0.20)=0.40→𝟒𝟎%\nStep 2: Subtract Reduction from Resistance\n52%−40%=𝟏𝟐%\nThe target has a final effective Resist of only 12%.\n\n\n=== Negative Resistance ===\nIf the Total Reduction is higher than the Total Resistance, the final value will be negative. This acts as a damage amplifier.\n\n30% Resist: You take 70% of incoming damage.\n0% Resist: You take 100% of incoming damage (True Damage).\n-30% Resist: You take 130% of incoming damage.\n\n\n== Effective Health ==\n\nInclude graph of effective health as a function of Damage Reduction. In the mean time, feel free to refer to this Desmos graph.\nInclude graph of effective damage increase from using 15%, 30%, and 45% Resistance Reduction as a function of the target's original Damage Reduction. In the mean time, feel free to refer to this Desmos graph.\n\n\n== Sources of Damage Resistance ==\n\n\n=== Bullet Resist ===\n\n Monster Rounds increases Bullet Resist vs. NPCs by +25%.\n Escalating Resilience increases Bullet Resist by 2% per stack per shot when hitting enemy heroes, maxing at 30% Bullet Resist.\n\n\n=== Bullet Resist Reduction ===\n\n\n=== Melee Resist ===\n\n\n=== Spirit Resist ===\n\n\n=== Spirit Resist Reduction ===\n\n Spirit Rend reduces -7% spirit resist on bullet hit from its component  Spirit Shredder Bullets, as well as an additional -8% spirit resist reduction per stack on headshots, up to a max of 4 stacks (-33% spirit resist total).",
        "ns": 0,
        "pageid": 922,
        "revisions": [
          {
            "parentid": 65372,
            "revid": 65375,
            "slots": {
              "main": {
                "*": "'''Damage Resistance''' refers to the [[Weapon Damage|Bullet]], [[Melee Attack|Melee]], and [[Spirit Power|Spirit]] Resist statistics which reduce incoming damage taken. Bullet Resist reduces all weapon and melee damage, Melee Resist reduces only melee damage, while Spirit Resist reduces all spirit damage. [[Unit]]s have a unique \"Damage Resistance\" stat that is applied to all sources of damage, including [[Trooper]]s.\n\nMost resistances stack multiplicatively; they are not added together. Sources of Resistance Reduction are multiplied together separately first. The total reduction is then subtracted from the total resistance. This has the consequence of resistance reduction being more effective on targets with higher resistance.\n\n== Resistance types ==\nIn addition to the basic types of Bullet and Spirit Resistance, there is also Melee Resist and Crit Reduction.\n\n=== Melee Resist ===\n\nMelee Resist is a stat that reduces all incoming [[melee damage]], including from melee abilities such as {{HeroIcon|Viscous}}' {{AbilityIcon|Puddle Punch}}. It increases equally to Bullet Resist (e.g. an item that gives 10% Bullet Resist also gives 10% Melee Resist). Some items also give bonus Melee Resist. Like other types of resist, it stacks multiplicatively.\n\n=== Crit Reduction ===\nCrit Reduction is a stat that reduces the critical damage (headshots) received by the hero. Not to be confused with [[Weapon Damage#Crit Multiplier|Crit Bonus Scale]], a stat that reduces the critical damage dealt by the hero to other targets.\n\n== Starting Damage Resistance ==\nThe following [[Hero|Heroes]] either have starting Damage Resistance values or gain resistance with [[Boon|Boons]] or [[Spirit Power]]. Boon and Spirit scaling resistances stack additively.\n\nNote: Negative values indicate that the hero receives increased damage.\n\n{{#invoke:HeroDataArrays|heroDamageResistTable}}\n\n== Calculation ==\nWhile items often display resistance as a flat percentage (e.g., +20% Resist), multiple sources of resistance do not simply add together. Instead, they stack '''multiplicatively'''.\n\n=== Calculating Resistance ===\nTo find your total resistance, use the following formula:\n\n<math>\\text{Total Resist} = 1 - (1 - R_1) \\times (1 - R_2) \\dots</math>\n\n'''Example:'''\nIf you have two sources of resistance, one providing '''40%''' and another providing '''20%''':\n# Convert percentages to decimals: 0.40 and 0.20.\n# Subtract from 1: (1 - 0.40) = '''0.60''' and (1 - 0.20) = '''0.80'''.\n# Multiply the remainders: 0.60 × 0.80 = '''0.48'''.\n# Subtract from 1: 1 - 0.48 = '''0.52'''.\n# Your Total Resist is '''52%'''.\n\n=== Calculating Resistance Reduction ===\n'''Resistance Reduction''' (often called \"shred\") acts as a counter to resistance. It is calculated separately using the same multiplicative formula, and then subtracted from the target's resistance.\n\n<math>\\text{Final Resist} = \\text{Total Resistance} - \\text{Total Reduction}</math>\n\n'''Example:'''\nA target has '''52%''' Total Resistance (as calculated above). You attack them with two sources of Reduction: one providing '''25%''' and another providing '''20%'''.\n\n'''Step 1: Calculate Total Reduction'''\n<math>1 - (1 - 0.25) \\times (1 - 0.20) = 0.40 \\rightarrow \\mathbf{40\\%}</math>\n\n'''Step 2: Subtract Reduction from Resistance'''\n<math>52\\% - 40\\% = \\mathbf{12\\%}</math>\n\nThe target has a final effective Resist of only '''12%'''.\n\n=== Negative Resistance ===\nIf the Total Reduction is higher than the Total Resistance, the final value will be negative. This acts as a damage amplifier.\n\n* '''30% Resist:''' You take '''70%''' of incoming damage.\n* '''0% Resist:''' You take '''100%''' of incoming damage (True Damage).\n* '''-30% Resist:''' You take '''130%''' of incoming damage.\n\n== Effective Health ==\n{{Construction}}\n[[File:Resistance-graph.png|thumb|A graph showing how despite the resistance percentage not growing additively, it scales the player's effective health additively.]]\n''Include graph of effective health as a function of Damage Reduction.'' In the mean time, feel free to refer to this [https://www.desmos.com/calculator/97vhqmutyz Desmos graph].\n\n''Include graph of effective damage increase from using 15%, 30%, and 45% Resistance Reduction as a function of the target's original Damage Reduction.'' In the mean time, feel free to refer to this [https://www.desmos.com/calculator/u7dkpebmm7 Desmos graph].\n\n== Sources of Damage Resistance ==\n=== Bullet Resist ===\n{{Item stat table|Bullet Resist}}\n* {{ItemIcon|Monster Rounds}} increases Bullet Resist vs. NPCs by +{{#invoke:ItemData|get_prop|Monster Rounds|NonPlayerBulletResist}}. \n* {{ItemIcon|Escalating Resilience}} increases Bullet Resist by {{#invoke:ItemData|get_prop|Escalating Resilience|BulletResistPerStack}} per [[stack]] per shot when hitting enemy heroes, maxing at {{#invoke:ItemData|get_prop|Escalating Resilience|MaxArmorStacks}} Bullet Resist.\n\n=== Bullet Resist Reduction ===\n{{Item stat table|Bullet Resist Reduction}}\n\n=== Melee Resist ===\n{{Item stat table|Melee Resist}}\n\n=== Spirit Resist ===\n{{Item stat table|Spirit Resist}}\n\n=== Spirit Resist Reduction ===\n{{Item stat table|Spirit Resist Reduction}}\n\n* {{ItemIcon|Spirit Rend}} reduces {{#invoke:ItemData|get_prop|Spirit Rend|MagicResistReduction}} spirit resist on bullet hit from its component {{ItemIcon|Spirit Shredder Bullets}}, as well as an additional {{#invoke:ItemData|get_prop|Spirit Rend|TechArmorDamageReduction}} spirit resist reduction per stack on headshots, up to a max of 4 stacks (-33% spirit resist total).\n{{Stat navbox}}\n[[Category:Damage Calculation]]",
                "contentformat": "text/x-wiki",
                "contentmodel": "wikitext"
              }
            },
            "timestamp": "2026-04-23T21:06:22Z"
          }
        ],
        "title": "Damage Resistance"
      }
    }
  }
}
````
