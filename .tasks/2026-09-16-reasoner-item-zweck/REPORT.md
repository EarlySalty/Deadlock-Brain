# REPORT: Alle-Helden-Reasoner

Stand 16.09.2026. Dies ist der nachprüfbare Orchestrierungs-/Vorprüfstand, noch keine behauptete abgeschlossene A-E-Implementierung.

## Aktualisierung nach Nutzerkorrektur: ChatGPT übernimmt selbst

ARBEITSTEILUNG.md hebt die automatische Vergabe späterer Phasen an neue Opus-Threads auf. Keine neuen Claude-Worker oder Claude-Subagenten wurden gestartet. Der vorhandene Worker darf ausschließlich 0+A beenden; T3 hat die Zustellung bestätigt (sequence 588210) und der Worker die Arbeitsteilung ausdrücklich angenommen.

ChatGPT hat selbst die aktuelle Deadlock-API-Dokumentation geprüft und vorhandenen Rust-Code gelesen. Ergebnisse und präzise Ereignis-/Zeit-/Inventar-Gegenproben stehen in API-RECHERCHE-CHATGPT.md. Live-Hero-/Item-Payloads sind durch diese Recherche nicht vollständig belegt; keine neuen Gameplay-Zahlen behauptet und keine Inputs in die laufende Messung eingemischt.

Die unabhängige Entwurfsprüfung von A-DESIGN.md auf Commit 806d4d5 hat BLOCK ergeben: damage_plan absichtlich auszunehmen und endliche negative Kanten als fehlend zu behandeln verfehlt den Auftrag. A-DESIGN-REVIEW-CHATGPT.md enthält konkrete Fix- und Testanforderungen. Der bestehende Worker hat diese Korrekturen in T3 bestätigt. Kein finaler Produktpatch wurde dadurch bereits freigegeben.

Der Gesamt-Freeze ist inzwischen abgebrochen, nicht erfolgreich abgeschlossen: laut Worker hing sein Messprozess im Planner. Die Ausweichmessung benutzt FROZEN-V2 und getrennt eingefrorene Population. Neue Warden-Werte und Modell-Coverage stehen mit Quellenkennzeichnung im REGISTER.md. Besonders wichtig: mehrere ursprünglich beanstandete Items fehlen laut Worker schon im neuen Vorher-Kern; deren späteres Fehlen darf deshalb nicht als A-Erfolg gelten. Staple-Gate weiterhin rot.

Die folgenden Abschnitte dokumentieren den früheren Verlauf. Der aktuelle Status steht in REGISTER.md; Angaben über einen noch laufenden Gesamt-Freeze oder ursprüngliches automatisches Opus-Routing sind historisch überholt. Diese Aktualisierung verändert ausschließlich Dokumentation, keine Produktquellen und keine Dienste.

## Gesicherter Ausgangsstand

Produktbasis 706b129. Main hat inzwischen ee440f3, einen reinen Doku-Commit für den Nutzerbefund und bisherigen Phasenvertrag. Phase 0+A läuft auf eigenem Feature-Worktree mit unveränderter festgehaltener Basis. Zentrale DB, produktive Dienste, Timer und veröffentlichte Builds wurden vom Delegator nicht verändert.

`python3 /home/nathanael/Documents/tools/t3-thread.py kontingent status` meldete claude-opus-4-8 frei und Codex-Modelle bis 19.09.13:23 gesperrt. `new --project Deadlock-Brain --model opus48 --effort high --branch feat/reasoner-purpose-a --worktree /home/nathanael/repos/wt/brain-purpose-a` lieferte HTTP 200 für create/turn und reale Thread-ID 803d3e94-9b1d-42c5-9bb7-1905c8146acc. `read` bestätigte running und tatsächliche Bestandssichtung. Keine anderen Implementierer gestartet. Ein gestarteter Thread ist kein Fertigbeleg.

## Konkret geprüfte zusätzliche Mechaniklücken

Alle Befunde beziehen sich auf gelesenen Rust-Bestand der Produktbasis, nicht auf ausgedachte In-Game-Messungen.

1. `rust/crates/dbrain-reasoner/src/types.rs`: WeaponProfile enthält tatsächlich noch keine per_spirit-Felder. ABER `combat.rs` ca. 637-654 hat schon eine hero.scaling-Auswertung von ERoundsPerSecond bzw. EFireRate. Damit ist die ursprüngliche These einer überall völlig fehlenden Spirit->Fire-Rate-Konversion zu pauschal. Phase A muss Loader, Hero-/DamagePlan, Stat-Konversion, Combat und marginale Bewertung vereinheitlichen und dieselbe Konversion genau einmal anwenden.
2. `combat.rs::active_with_text`: ConditionKind::MeleeBound ergibt aktuell immer false. Das ist eine überprüfbare Lücke bei bedingten Nahkampf-Triggern. Noch kein globaler Nachweis, dass sämtlicher Nahkampf an anderer Stelle fehlt. B muss bestehende Interaktionspfade nachprüfen und gegen echte Trigger verbinden.
3. `combat.rs::apply`: BulletShieldMaxHealth, TechShieldMaxHealth und CombatBarrier werden in Stats::shield zusammenaddiert. C muss die downstream-Bewertung auf Schadenskanäle und wirkliche zeitliche Deckung prüfen. Ein pauschaler beliebiger Shield-Pool ist kein Beleg für Abdeckung jeder HP-Downside.
4. `ReasonerConfig::default` enthält bereits combat_window_seconds=40.0. Die Formulierung eines reinen 3-Sekunden-Duells beschreibt deshalb nicht vollständig den aktuellen Konfigurationsstand. Kürzere tatsächliche Kämpfe können durch Tod/TargetDefeated entstehen; tatsächliche Szenario-Dauer und Endgrund getrennt messen, nicht lediglich das Fenster hochsetzen.
5. Rechenpräzisierung des Nutzerbeispiels: 1599-1391=208. Rund 600 HP Verlust bei 13 Prozent setzt rund 4615 HP Ausgangspool voraus. C erhält keinen festen 600er-Abzug. Prozentualer Lifesteal auf gleiche Gesamtschadensmenge ändert sich durch bloß kleinere Ticks nicht automatisch; echte Proc-/Cap-/Timing-Regeln sind zu belegen.

6. `examples/build_evaluation.rs`: Das bisherige Frozen-Objekt speichert PopulationPrior nicht. evaluate/holdout/plan/sensitivity laden über load_populations weiterhin live aus der DB. Deshalb ist FROZEN-V2 allein kein vollständig fixiertes Bewertungseingabeset. Der Worker hat hierzu einen echten Rust-WIP-Patch begonnen: FrozenPopulationPrior, freeze-populations, Datei-Loader via FROZEN_POPULATIONS und technisch erzwungener Read-only-DB-Fallback. Der Messcode ist inzwischen als 887e48d eingecheckt (196 Einfügungen, 3 Löschungen in build_evaluation.rs und hero_conversion_coverage.rs), noch kein abgenommener Produktpatch. Vorprüfung durch Delegator: ungeschützter Ausgabeschreibpfad im neuen Coverage-Example; negative Werte weichen vom behauptet gleichen Combat-Pfad ab; normalisierte Modellfelder sind kein Nachweis vollständiger Rohasset-Abdeckung. Korrekturen samt fokussierten Rust-Gegenproben wurden im bestehenden Worker-Thread erfolgreich angefordert (T3 sequence 587854), kein zweiter Worker. Dies ist keine frische unabhängige Reviewfreigabe. Der laufende Gesamt-Roster-Freeze ist noch nicht abgeschlossen. Nach dem Export müssen beide Codefassungen mit denselben Asset- UND Populations-Dateien erneut gemessen werden; live_baseline aus dem alten Freeze allein genügt dafür nicht. Getrennte Exportzeitpunkte nicht als atomaren gemeinsamen DB-Snapshot ausgeben. Roundtrip, fehlende/ungültige Datei und belegter datenbankfreier Replay gehören ins Review.

Diese Lücken stehen mit generischen Tests in PAKETE.md. Kein Name wurde als Produktregel eingeführt. AUFTRAG.md verlangt Gesamt-Roster-Coverage und entscheidungsrelevante Unsicherheit, nicht nur einen Warden-Snapshot. Ein Coverage-Rust-Example existiert bereits unter rust/crates/dbrain-reasoner/examples/ability_coverage.rs und soll erweitert statt neu erfunden werden.

## Messstand

| Messung | Status |
|---|---|
| Aktuelle eingefrorene Phase-0-Baseline | Vom A-Worker beauftragt, hier noch nicht als Ergebnis verifiziert |
| Historischer Vergleich Warden | 6/9 Referenzwaffen, 9/10 Staples, Kendall 0.577, Jaccard@12 0.500 laut N-MESSUNG.md; ausdrücklich nicht neu gemessen |
| Aktueller Backtest nach A | Noch kein abgenommener Nachherlauf |
| Rust-Tests / Clippy / Formatter des neuen Codes | Worker meldete am 16.09. um 14:35 UTC eine Unit-Test-Ausgangsbasis von 173 passed / 16 ignored / 0 failed sowie kompilierende Examples. Noch kein unabhängiger Nachher-/Clippy-/Formatter-Abnahmebeleg für A; ignorierte Tests nicht als bestanden gezählt |
| Unabhängiges Review | Wartet auf tatsächlichen A-Commit |
| Drei identische Rust-/KI-Läufe | Noch nicht nachgewiesen, Rust-Replay und Live-KI getrennt zu messen |
| Merge/Deploy/Neuveröffentlichung | Nicht erfolgt |
| Neue hero_build_id | Keine |

## Artefakte / Gültigkeit

AUFTRAG.md erweitert Scope, PAKETE.md enthält konkrete abhängige B/C/D/E-Aufträge, REGISTER.md führt echte Threads und Zustände. Reiner Doku-Branch docs/reasoner-all-heroes hält diese Ergänzungen unabhängig vom Implementierungsworktree. Originaler BEFUND.md bleibt unverändert. Phase-0-/A-Zahlen werden erst nach Vorlage der tatsächlichen Messartefakte übernommen. Ein rotes Qualitätsgate wird nicht zu einem informellen Hinweis herabgestuft.

Keine 45-Minuten-Automation eingerichtet, keine automatische Kette nach Ende dieses Chats behauptet. Direkte Worker-Prüfung während der Bearbeitung, kein stiller Modellwechsel und keine vier parallelen bezahlten Threads.
