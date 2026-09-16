# Reasoner-Befund: Item-Zweck und Helden-Skalierung fehlen

Stand 2026-09-16. Ausloeser: veroeffentlichtes Warden-Build (hero_build_id 811202) ist "nicht ultra bad, aber definitiv nicht wie ein normales Build gebaut wird" (Nutzer). Referenz-Maszstab bleibt Lightbringer-Build 779996 ("LIGHTBRINGERxSITUATION WARDEN BUILD").

## Kernproblem
Der Reasoner bewertet Items nach roher Kampf-Grenznuetzlichkeit in isolierter Duell-Sim. Ihm fehlen:
1. Helden-Skalierungs-Identitaet: Warden skaliert Spirit -> Fire Rate. Spirit-Items sind dadurch zugleich Waffen-DPS. In-Game-Beweis: Weapon Investment zeigt Spirit x0.25 fliesst in 208% Weapon Damage und 101% Fire Rate (Mercurial Magnum 22%, Burst Fire 10%, Spiritual Overflow 30%, Healing Tempo 35%, Swift Striker 20%).
2. Item-Zweck/Rolle statt nur Stats: Dauer-Sustain vs Burst-Sustain, Anti-Gun-Carry-situational, Mobilitaet, Save/Defensive, HP-Downside.
3. Downside-Gewichtung im echten Spiel statt nur im 3-Sekunden-Duell.

## Fehlurteile im veroeffentlichten Build (Nutzer-Feedback, verbindlich)
- Mystic Regeneration: KEIN Warden-Item. Passiv gibt Regen beim Zufuegen von Spirit-Schaden an Gegner-Heroes, stackt je unterschiedlichem Hero. Gebaut auf Heroes mit LANGEM/kontinuierlichem Spirit-Schaden (z.B. Shiv Knife), die NICHT von %-Spirit-Lifesteal profitieren, weil der Schaden pro Tick zu klein ist. Warden burstet -> passt nicht. Reasoner zaehlt die +50 HP und Regen faelschlich als Kampfnutzen.
- Glass Cannon: jain, kann Sinn ergeben, aber -13% Max-HP sind auf Max-Level real ~600 HP (Screenshot 1599 -> 1391 bei Level ~26; skaliert weiter). Nur mit Survivability-Backup vertretbar. Reasoner laesst dafuer Mercurial Magnum verkaufen (staerkster Waffenhebel) -> falsch.
- Rusted Barrel: KEIN Kern. Situational gegen Gun-Carries, wenn man ein Problem damit hat, eher auf Support-Hero. Reasoner kauft es als Lane-Core.
- Healing Tempo: okay, aber Warden hat ueber Spirit schon massig Fire Rate -> redundant.

## Referenz-Build 779996 (gutes Warden-Build), Item-Layout aus Screenshot
CORE (/// CAN BUY ANY 3K ITEM INSTEAD OF VEIL):
High-Velocity Rounds, Opening Rounds, Extra Regen, Monster Rounds, Extended Magazine, Swift Striker, Titanic Magazine, Quicksilver Reload(imbue), Veil Walker, Enduring Speed, Fleetfoot, Mercurial Magnum(imbue), Spiritual Overflow, Siphon Bullets, Frenzy, Boundless Spirit(upgrade), Blood Tribute, Unstoppable, Witchmail, Transcendent Cooldown, Juggernaut
TRYHARD: Slowing Hex
CAN BUY 1 OR 2: Return Fire, Metal Skin, Dispel Magic, Spirit Resilience, Counterspell, Bullet Resilience
SHIELDS: Spirit Shielding, Reactive Barrier, Weapon Shielding
OPTIONAL: Healing Booster, Silencer, Spellslinger, Toxic Bullets, Ricochet, Armor Piercing Rounds, Crippling Headshot, Spellbreaker, Inhibitor, Plated Armor, Vampiric Burst

Was der Referenz-Kern zeigt: Sustain aus Bullet-Lifesteal (Spiritual Overflow, Siphon Bullets) + Blood Tribute statt Mystic Regeneration; Survivability aus Unstoppable/Metal Skin/Veil Walker/Shields statt HP-Verlust-Items; Spirit-Items doppeln als Fire-Rate-Skalierung.

## Nutzervorgabe: emergent, KEINE hand-getaggten Zweck-Labels
Der Builder muss den Zweck eines Items SELBST aus den Spieldaten ableiten. Das ist der Kern des Build-Bauens. Kein Item-Rollen-Tag von Hand, kein "MR schlecht fuer Warden" im Code. Steht auch in der Abnahme-Regel (keine Warden-Sondergewichte, keine Referenz-Itemnamen).

## Mechanismus (wie die Aspekte emergent reinkommen)
Gruest der bereits vorhandenen Struktur nutzen statt neu bauen: HeroModel.damage_plan (weapon_dps/spirit_dps/weapon_share/primary_axis) und ScalingStat.per_spirit existieren; ItemModel hat property_spirit_scaling, conditional_properties, ConditionKind; ItemScore hat condition_factor.

A. Helden-Konversionsgraph vollstaendig lesen (groesster Hebel, kleinster Eingriff). WeaponProfile hat KEIN per_spirit -> Wardens Spirit x0.25 -> Fire Rate/Weapon Damage ist fuer die Sim unsichtbar. Waffen-Stat-Skalierung durch Spirit aus Helden-Daten ins Modell ziehen; dann erzeugt jedes +Spirit ueber den Graphen automatisch Waffen-DPS fuer DIESEN Helden. Spirit-Items bewerten sich von selbst hoch bei Warden, anders bei Shiv.

B. Bedingte Effekte wirklich rechnen statt als "nicht quantifiziert" droppen. ConditionKind ist zu grob (ActiveCooldown/ShotBound/StateBound/...); MRs Bedingung (dauerhafter Spirit-Schaden an N unterschiedlichen Heroes, gestapelt) passt in kein Variant. Drei Schritte: (1) Magnitude aus rohem Item-KV parsen, (2) Bedingungs-Vokabular erweitern, (3) erwartete Ausloeserate gegen HeroModel.damage_plan rechnen. Warden burstet, hoher weapon_share -> MR-Erwartungswert kollabiert -> faellt emergent raus. Die "nicht quantifiziert"-Liste IST die To-do-Liste.

C. Downside am echten Spielzustand: Glass Cannon -13% Max-HP am tatsaechlichen HP-Pool (~600 auf Max-Level), gegen vorhandene Survivability im Build gegenrechnen. Zahlt nur, wenn Shields/Unstoppable den Verlust decken.

D. Build als Zusammenspiel: Item-Wert ueber den Graphen aus A, damit Compounding (Spiritual Overflow + Spirit-Skalierung + Fire Rate) sichtbar wird. Begruendung = Kette im Graphen, nicht flache Zahl.

E. Pruefen statt Vorschreiben (haelt es emergent): nie "bau wie 779996"; nur messen, ob das emergente Build gegen 779996 und die Population konvergiert (Kendall tau, Jaccard@12, Staple-Gate existieren). Zwei Wissensquellen, beide vorhanden: Mechanik = das Warum (Property + Bedingung + Konversionsgraph), Population = das Was (Pro-Kaeufe je Archetyp als Gegenprobe, nie Namenskopie).

Aufwand: A Zuendschluessel (klein), B die eigentliche Arbeit (KV-Parsing + Bedingungs-Sprache), C/D fallen dann fast von allein, E ist der Beweis.

Status: nur Mechanik-Befund, noch KEIN Worker beauftragt (Nutzer wollte erst das Wie verstehen).

## PLAN (wie das gehandhabt wird)

Prinzip: sequenziell A -> B -> C -> D, jede Phase eigener opus48-coder-Worker im eigenen Worktree, Review durch frischen Reviewer (nie Selbst-Review), Merge einer Phase nur wenn der Backtest gegen 779996 nicht schlechter wird. Entwicklung gegen lokale Wegwerf-DB, zentrale DB read-only, Migration beim Deploy durch den Delegator. KEINE Warden-Sondergewichte, KEINE Referenz-Itemnamen im Code (Guard-Test).

Phase 0 - Messbasis fixieren (kein Code)
- deadlock-brain reason backtest Warden + population stats laufen lassen, Baseline festhalten: Referenzwaffen (aktuell 6/9), Staples (9/10), Kendall tau, Jaccard@12, plus die drei Fehlurteile (MR/Rusted Barrel/Healing Tempo im Kern, Glass Cannon ohne Survivability). Zahlen in REPORT.md.
- Ergebnis ist die rote Zahl, an der A-E gemessen werden.

Phase A - Helden-Konversionsgraph (Zuendschluessel, klein)
- types.rs: WeaponProfile um Spirit-Skalierung der Waffenstats erweitern (per_spirit je Waffenstat), analog zur vorhandenen ScalingStat.
- parse/enrich (HeroModel-Befuellung aus Assets): die Spirit->Fire Rate/Weapon Damage-Modifier des Helden einlesen (dieselben, die das In-Game "Weapon Investment"-Panel zeigt).
- combat.rs/mechanics.rs: Waffen-DPS ueber den Konversionsgraphen rechnen, sodass +Spirit automatisch Waffen-DPS erzeugt.
- Test: +Spirit-Item hebt Wardens Waffen-DPS; Snapshot: abgeleitete weapon_share/Anteile stimmen mit dem In-Game-Panel (Toleranz) fuer 2-3 Referenzhelden.
- Messung: Backtest-Delta gegen Phase 0.

Phase B - Bedingte Effekte modellieren (die eigentliche Arbeit)
- B1 KV-Parsing: fehlende Magnituden aus dem rohen Item-KV in ItemModel.properties/passive_properties/conditional_properties fuellen; Ziel ist die "nicht quantifiziert"-Liste messbar verkuerzen.
- B2 types.rs: ConditionKind-Vokabular erweitern, sodass "dauerhafter Spirit-Schaden an N unterschiedlichen Heroes, gestapelt" u.ae. ausdrueckbar sind.
- B3 mechanics.rs/combat.rs: erwartete Ausloeserate je Bedingung gegen HeroModel.damage_plan (weapon_share, primary_axis, burst vs sustained) rechnen, in condition_factor einspeisen.
- Test-Fixtures: MR-Erwartungswert niedrig bei Burst/weapon_share-Held (Warden), hoch bei einem dauerhaften Spirit-Held; Rusted Barrel nur bei Anti-Gun-Carry-Kontext relevant.

Phase C - Downside am echten Spielzustand
- item.rs/combat.rs: Max-HP-Downside am tatsaechlichen HP-Pool zum Kaufzeitpunkt (Glass Cannon ~-600 Max-Level) und gegen die im Build vorhandene Survivability gegenrechnen.
- Test: Glass Cannon negativ ohne Shields/Unstoppable, tragbar mit.

Phase D - Compounding ueber den Graphen
- planner.rs/composer.rs: marginale Item-Bewertung laeuft ueber den Konversionsgraphen aus A (faellt groesstenteils von selbst); Begruendung je Item = Kette im Graphen.
- Test: Build konvergiert auf Spirit-Fire-Rate-Kern (Spiritual Overflow, Mercurial Magnum, Siphon Bullets), MR/Rusted Barrel/Healing Tempo nicht mehr im Kern.

Phase E - Beweis, Merge, Deploy
- Backtest gegen 779996 + Population; Zielkorridor: Referenzwaffen >= Phase-0-Stand, MR/Rusted Barrel/Healing Tempo raus aus Kern, Glass Cannon nur mit Survivability-Deckung, Kendall tau und Jaccard@12 hoch, drei KI-Laeufe zeichengleich.
- Guard: grep-Test gegen Referenz-Itemnamen und Warden-Sondergewichte im Produktcode.
- Merge je Phase gemergt, Deploy per Release-Weg (DEPLOY-BRAIN.md), Warden neu veroeffentlichen, neue hero_build_id an den Nutzer.

Routing/Betrieb: Worker opus48-coder (Codex-Kontingent leer bis 19.09.), 45-Minuten-Wache, tote/fertige Worker sofort weiterziehen. Artefakte (REPORT.md, Register) in dieser Akte, gepusht.
