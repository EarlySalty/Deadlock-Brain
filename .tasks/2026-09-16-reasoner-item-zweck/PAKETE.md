# Pakete: Alle-Helden-Reasoner in Rust

Stand 16.09.2026. Grundlage AUFTRAG.md, BEFUND.md und ORCHESTRIERUNG.md. Kein Agentenstart allein durch diese Datei. Tatsächlich gestartete Threads stehen ausschließlich in REGISTER.md. Delegator ist dieser ChatGPT-Chat via codex-mcp; kein erfundener Intent-Thread. Die Phasen bauen strikt aufeinander auf. Pro Implementierung ein frischer Opus-4.8-Thread, eigener Worktree, kein Unterdelegieren; danach unabhängiges Review. Maximal ein Implementierer aktiv. Historische Vergleichswerte sind keine aktuelle Baseline.

| Paket | Konkrete Lieferung | Abhängigkeit / Abnahme | Zustand |
|---|---|---|---|
| 0 | Eingefrorene lokale Inputs, Hashes, Warden-/Population-Metriken, Multi-Hero-Ausgangsstand, unbekannte Mechaniken | Keine Produktänderung vor Baseline; fehlende Messung offen markieren | Mit A beauftragt |
| A | Waffen-Konversionsgraph in bestehender Rust-Pipeline, Einheiten/Vorzeichen, alle aus Assets belegten Konverter, 3 reale Hero-Fixtures und Roster-Scan | Nicht schlechterer gleicher Backtest, unabhängiger Reviewer | Aktiv, Thread 803d3e94 |
| B | Rohe KV-Magnituden, vollständigeres Ereignis-/Bedingungsmodell, Hero-Abilities/Trigger/Charges/DoT/Stacks/Mehrziel-Uptime, Coverage | Auf abgenommenem A, frischer Worker; konkrete KV-Abdeckung steigt | Wartet auf A |
| C | Echter HP-Pool, getrennte Schadens-/Shield-Kanäle, Risiko/Heilung/Defensive im Zeitverlauf | Auf abgenommenem B; Verlust und Deckung kontextabhängig nachweisbar | Wartet auf B |
| D | Gemeinsame marginale Inventarbewertung für Kauf/Upgrade/Verkauf, nichtlineare Synergien, deterministische Mechanikketten | Auf abgenommenem C; keine Namen-/Archetyp-Sonderregeln | Wartet auf C |
| E | Unabhängige Multi-Hero-Gesamtabnahme, Guards/Determinismus/Holdouts, Rust-Runtime-Audit, regulärer Release/Publish-Beleg | Erst wenn finale Qualitätsgates grün; keine vorgezogene Veröffentlichung | Wartet auf D |

## Review jeder Phase

Ein neuer Thread prüft exakt Basis..Commit, Vertrag, Rohdatenquellen und wirklich ausgeführte Tests. Kein eigener Patch im Reviewer. Befunde mit Pfad:Zeile, Schweregrad, Gegenbeispiel, reproduzierbarem Test und minimalem Fix; ALLOW oder BLOCK eindeutig. Teilphasen dürfen keine Regression gegenüber identischen Phase-0-Eingaben verbergen. Bestehende rote Gesamtgates werden separat offen gehalten; ein technisches ALLOW für einen Zwischenpatch ist keine finale Releasefreigabe. Fixes gehen an den einzigen zuständigen Implementierungsthread, danach erneute unabhängige Prüfung. Keine Reviews ohne fertigen prüfbaren Commit.

## B: genaue Prüfaufträge

Bestehende types/hero/item/mechanics/combat/ability_interactions/item_interactions verwenden, keine zweite Engine. Zuerst aktuellen KV-Korpus inventarisieren: Effekte/Magnitude/Einheit/Quelle, unterstützte und nicht quantifizierte Pfade, positive/negative Werte. Bedingung aus nachweisbarer Mechanik statt Itemname oder Zwecktext. Strukturiertes Vokabular mit Trigger-Art, Kanal, Hero-/PvE-Ziel, Distinct-Target-Key, Dauer/Refresh, Tickintervall, Stack-Cap/Decay, ICD, Schaden-/HP-Schwelle und aktivem Cooldown. Proc-disabled und rekursive Proc-Schleifen korrekt behandeln.

Erwartungswert aus Hero-Ereignisstrom, Zielkontakt, Cooldowns, Charges, Fähigkeitspunkten und tatsächlichem Zeitfenster, nicht bloß hohem/niedrigem weapon_share. Burst und DoT mit gleicher Gesamtschadensmenge als Gegenprobe: prozentualer Lifesteal bleibt ohne weitere Spielregel gleich, duration-/hero-stackbasierte Heilung kann unterschiedlich ausfallen. Keine ausgedachten Begründungen für einen gewünschten Item-Rang.

Vorprüf-Befund auf Basis 706b129: combat.rs::active_with_text gibt für ConditionKind::MeleeBound immer false zurück. Prüfen, wie vorhandene Melee-/Ability-Interaktionen tatsächlich ausgeführt werden; Triggerbedingungen gegen echte Nahkampfereignisse testen. Nicht behaupten, sämtlicher Nahkampfschaden sei bereits als Ganzes geprüft. Das ist ein konkreter Cross-Hero-Test, keine neue Helden-Ausnahmeregel.

Generische Fixtures: einzelner Burst vs periodisch vs wiederholte Ability, ein vs mehrere unterschiedliche Hero-Ziele, gleicher Hero darf Distinct-Stack nicht erhöhen, Refresh vs neue Stacks, Cap und ICD, überlappende Effekte, abgelaufener Effekt, Melee/Weapon/Spirit-Procs und Null-Trigger, Gegner mit niedriger vs hoher Waffenbedrohung. Holdout-Helden nach Fähigkeitenfamilien aus dem eingefrorenen Roster auswählen; Auswahl und Datenstand vor Ergebnisvergleich dokumentieren. Kein Coverage-Prozent nur aus geladenen JSON-Zeilen.

## C: genaue Prüfaufträge

Echte Basis/Levelwachstum/Seelenbelohnungen/Kaufboni/aktuelles Inventar in den HP-Pool nehmen. Prozentverlust genau einmal, korrekte additive/multiplikative Stacking-Regel aus Daten, Downside beim Kauf und Verkauf neu rechnen. 1599 auf 1391 belegt etwa 208 Verlust, kein fixer 600er-Abzug.

Vorprüf-Befund 706b129: combat.rs::apply addiert BulletShieldMaxHealth, TechShieldMaxHealth und CombatBarrier in ein einziges Stats::shield. Diese Kanäle dürfen nicht als gegenseitig beliebig austauschbare Deckung gelten. Vorhandene downstream-Pfade prüfen und über denselben Evaluator bereinigen, keine zweite Risikoheuristik daneben.

Tests: Gun-/Spirit-/gemischter Gegner, passende/unpassende Shields, Shieldablauf und Reuse-Cooldown, Incoming-Damage-Stärke, CC-Immunität ohne direkte Damage-Immunität, Schaden ohne CC, Trade/verlängerter Kampf, Overheal vs nutzbare Heilung, Anti-Heal falls modelliert, negativer HP-Wert und extreme/fehlende Daten. Ein Verteidigungsitem ist keine boolesche Glass-Cannon-Freigabe. Mechanisch mögliche Tradeoffs erlauben, aber fehlende Deckung/unsichere Mechanik sichtbar halten.

## D: genaue Prüfaufträge

plan_build/compose_build_with_sources/evaluate_inventory und bestehende Aufrufer zuerst nachweisen. Ein Evaluator und gleiche Daten/Parameter für Kandidat, Kauf, Upgrade, Ersetzen und endgültige Kaufroute. In jedem Kaufzustand Slotlimit, Komponentenverbrauch, Verkaufserlös, Imbue, gleichzeitige Aktivierungen und spätere Wiedereinkäufe korrekt. Kostenbänder dürfen keine versteckten Zweckrollen einführen.

Erklärungen aus Evaluator-Zwischengrößen: Quelle -> Statänderung -> Konversionskante -> Waffen/Ability/Defensive im Szenario -> marginaler Nettowert und Grenze. Nicht nachträglich durch LLM erfundene Mechanikketten. Zusätzlich Austauschverlust (Inventar ohne verkauftes Item vs unverändert vs Ersatz) und Opportunity Cost sichtbar. Synergie darf sowohl positiv als auch sättigend/negativ sein. Keine pauschale Feuerarten-/Heldengruppe-Korrektur.

Tests: Multiplikator im vollständigen Inventar wertvoller als isoliert; Spirit-Konverter vs Nichtkonverter, Redundanz/Sättigung, Konkurrenz von Channel und Schüssen, Reload/Ammo, Imbue-Wechsel, negative Kante/Downside, unterschiedliche Budgets/Kaufzustände. Ergebnis darf nach neutraler Umbenennung und kohärenter ID-Permutation der Eingaben nicht mechanisch anders werden.

## E: genaue Prüfaufträge

Gleiche unveränderte Inputhashes für Vorher/Nachher, Referenzkategorie/-version und Metrikdefinitionen exakt notieren. Autor-Übereinstimmung und Population getrennt, keine Summenmetrik die schlechte Holdouts verbirgt. Kendall nur mit genügend vergleichbaren Rängen, ties/fehlende Werte gemäß vorhandener Definition; null nicht zu 0 umdeuten. Jaccard@12 auf derselben Definition von Kaufpfad/Finalinventar/Upgrades, nie zwischen Messungen wechseln. Auswahl- und Kaufreihenfolge getrennt ausweisen.

Vollständiges eingefrorenes Roster: jeder Hero bekommt belegte Coverage oder präzise Lücke; entscheidungsrelevante nicht verstandene Fähigkeit muss Ergebnis-/Publish-Vertrauen begrenzen statt still als 0 Nutzen zu gelten. Drei bitidentische Rust-Replays auf fixiertem Input samt Hashes. Drei unabhängige Live-KI-Läufe sind ein anderer Nachweis; nicht mit --no-ai gleichsetzen. Keine Prompt-/Tool-Ausgaben mit Secrets.

Guard: keine Namen-/ID-/Reference-Sonderregeln im Bewertungszweig, einschließlich indirekter Tabellen und Archetyp-Hardcodings. Anzeigenamen/IDs als Daten und Testfixtures sind legal, ein grep allein ist kein Beweis. Metamorphe Umbenennung und ID-Bijektion unter Erhalt aller Relationen, unverändertes normalisiertes mechanisches Resultat.

Rust: produktiven Weg vom CLI/API-Einstieg über Parsing, Modelle, Bewertung, Planung und Erklärung bis Publish nachweisen. Legacy-Python nur als vorhandene separate Dienste transparent inventarisieren, nicht blind löschen; keine neue Python-Produkt-/Glue-Logik. Frische unabhängige Gesamtabnahme vor Merge/Release. DEPLOY-BRAIN.md ist historischer Betriebsbeleg, aktuelle Unit/Release-Wrapper/HEAD erneut prüfen. Kein Service-Restart bloß wegen Doku. Kein Publish ohne gemessene Qualitätsfreigabe; neue hero_build_id nur nach tatsächlichem Steam-Erfolg/Readback.
