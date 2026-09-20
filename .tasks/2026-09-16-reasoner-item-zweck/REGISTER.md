# Register – Item-Zweck und Helden-Skalierung

## Aktueller Integrationsstand der Fortsetzung

Maßgeblicher Code-Stand: d7c0f38248127438fe89cabc9373144ab38a3a02 auf `fix/reasoner-mechanics-completion`, Worktree `/home/nathanael/repos/wt/brain-purpose-finish`. Die folgenden alten Worker-Tabellen sind historisch und keine Aufforderung, neue Opus-Threads zu starten. Die Nutzerkorrektur gilt: Umsetzung/Prüfungen in diesem Chat, keine neuen Claude- oder anderen bezahlten Worker gestartet.

Neu direkt implementiert und gepusht: ee22628 (Spirit-Treffer/Refresh), 2cc57ac (expliziter DB-freier Replay), 53c3e40 (Schadenskanäle und einheitliche Defensive, doppelte Regen-Gutschrift entfernt), 51d91f0 (echter eingehender Schaden und endliche Schilde), d7c0f38 (Sparvergleich auch für Populations-Staples, korrekte Verkaufstexte). Details, rote Gegenproben und tatsächlich ausgeführte Tests stehen in FORTSETZUNG.md.

212 Library-Tests und 22 Example-Testausführungen bestanden, 16 DB-Tests nicht ausgeführt. Fokussierte Formatierung und Clippy grün. Roster-Kampf-Replay auf 51d91f0: drei mal 228 vollständige Auswertungen, beide Dateivergleiche Exit 0. Das ist kein Live-KI-Nachweis und keine vollständige Mechanikabdeckung.

Warden-Replay auf d7c0f38: tatsächlich Exit 0, 27,686 Sekunden; 6/9 Referenzwaffen, Populations-Kendall 0,577411, Jaccard@12 0,5. Magnum im Kern mit sell_priority=null; die alte Magnum-Verkaufsanweisung ist weg. Weiterhin 9/10 Staples (Enduring Speed fehlt), Rusted Barrel und Healing Tempo im Kern, Glass Cannon ebenfalls. Autoren-recall 0,421053 bleibt unter dem Vorherwert 0,473684. Keine Gesamtfreigabe.

Unabhängige Endabnahme der eigenen Änderungen fehlt. Kein Main-Merge, Deploy, Restart, zentraler DB-Write oder Publish; keine neue hero_build_id. Keine automatische Wache und kein neuer Worker. Unfertige Branches/Worktrees werden nicht vor vollständiger Integration und erforderlichem Live-Beleg gelöscht. Konkrete offenen Schritte in TODO.md.

## Historischer Beginn der Orchestrierung

Stand der bestätigten Werkzeugeingriffe: 16.09.2026. Hauptrepo /home/nathanael/repos/Deadlock-Brain, produktiver Quell-Ausgangspunkt 706b129; Dokumentations-Commit ee440f3. Status basiert auf echten T3-/Git-Abfragen, nicht auf geplanten Worker-Namen.

| Paket | Zustand | Reale Thread-ID / Modell | Worktree / Basis | Nachweis |
|---|---|---|---|---|
| 0 + A | BEAUFTRAGT; zuletzt AKTIV, keine Freigabe | 803d3e94-9b1d-42c5-9bb7-1905c8146acc / claude-opus-4-8 | /home/nathanael/repos/wt/brain-purpose-a / 706b129 | T3-Titel „Reasoner alle Helden (A Konversionsgraph)“; aktiver Thread vorgefunden und koordiniert, kein Doppelstart. Messwerkzeug-Commit 887e48d nachgeprüft, noch keine fertige Phase-0/A-Tabelle. Weitere T3-Lese-/Steueraufrufe zuletzt vom Sicherheitsfilter blockiert; aktueller Laufstatus deshalb nicht erneut bestätigt. |
| Review A | PLAN | noch kein Thread | erst gegen fertigen A-Commit | Keine Selbstfreigabe; eigener Reviewer erforderlich. |
| B | PLAN | noch kein Thread | erst auf freigegebenem A | B-AUFTRAG.md |
| Review B | PLAN | noch kein Thread | nach B | Frischer Reviewer. |
| C | PLAN | noch kein Thread | erst auf freigegebenem B | CDE-AUFTRAEGE.md |
| Review C | PLAN | noch kein Thread | nach C | Frischer Reviewer. |
| D | PLAN | noch kein Thread | erst auf freigegebenem C | CDE-AUFTRAEGE.md |
| Review D / E | PLAN | noch kein Thread | nach D | Unabhängige Gesamt-/Intentabnahme und technisches Merge-Gate erforderlich. |
| Release / Publish | BLOCKIERT durch ausstehende Abnahme | kein Deployer gestartet | kein neuer Release | Keine neue hero_build_id, keine Änderung an produktiven Diensten, Timern oder zentraler DB durch diese Orchestrierung. |

## Bestätigter Verlauf

- Nutzerbefund unverändert in BEFUND.md gesichert; ORCHESTRIERUNG.md und A-AUFTRAG.md auf main committet (ee440f3, Dokumentation, keine Produktänderung).
- Modellkontingent geprüft: Opus 4.8 verfügbar; Codex/Astra und Sol/Luna gesperrt bis 19.09.2026 13:23 laut T3-Status. Kein Wechsel auf kostenpflichtige neue Modelle.
- Vor Startversuch war der passende Worktree/Branch bereits vorhanden; anschließend passender aktiver T3-Thread gefunden. Keine bestehenden Änderungen überschrieben, keinen zweiten Implementierer gestartet.
- 16:21 CEST: Hinweis zur bereits vorhandenen spirit_rate-Konversion in combat.rs und zur Vermeidung von Doppelzählung gesendet; Worker bestätigt.
- 16:26–16:28 CEST: Worker meldet gebaute Baseline-Messbinary, SOURCE_CLEAN-Prüfung und laufenden Read-only-Freeze. Dies sind Prozess-/Buildmeldungen, noch kein Qualitäts- oder Baseline-Nachweis.
- 16:29 CEST: fehlendes Einfrieren der Population im bestehenden build_evaluation-Example belegt und gemeldet. 16:30 CEST: Worker bestätigt lokale Population und Datei-Loader für vollständig fixierte Vergleiche.
- PRUEFHINWEISE.md dokumentiert weitere konkrete Ausgangslücken in Bedingungen, Schild-/Gegnerkontext, Abdeckungszähler und Messpfad. Keine Produktcodeänderung durch die Sichtprüfung.

## Regeln für Fortschreibung

Kein MERGED/DEPLOYED ohne Commit, unabhängiges Urteil, Gate und echten Live-Beleg. Historische 6/9 Referenzwaffen, 9/10 Staples, tau 0,577 und Jaccard@12 0,500 bleiben historische Angaben bis Phase 0 reproduziert ist. Bei Daten-/Metrikdrift keine Verbesserung behaupten. B/C/D starten sequenziell und jeweils als neuer Opus-4.8-Thread. Eigene fertige Threads settle; alte fremde Threads unangetastet.

Keine automatische 45-Minuten-Wache eingerichtet: die hier verfügbare Aufgabenplanung unterstützt höchstens stündliche Läufe. Aktuelle T3-Zustände werden während der laufenden Bearbeitung direkt geprüft. Kein automatischer Start weiterer Phasen nach Ende eines Chat-Turns behauptet.
