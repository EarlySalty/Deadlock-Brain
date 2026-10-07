[Orchestrator]
# G: bestehende Rechenschicht und Werkzeuge bis live fortsetzen

## Auftrag und Rolle

Frischer Fortsetzungs-Teil-Orchestrator für den offenen G-Auftrag, kein Neubau. Nutzer hat Sessionwechsel ausdrücklich beauftragt, BLOCKER-PRUEFWEG.md Nachtrag 20:50. Auftraggeber ist Delegator 481426fe-b477-42b3-91c6-901811fcba1d. Alter G-Thread a867ef50-88e6-41ac-a852-724f5184c6e6 wurde regulär gestoppt (Dispatch 1861508, T3 stopped bestätigt) und wird nicht wieder aufgenommen. Früherer Haupt-Orchestrator d3a1741e bleibt gestoppt. Keine fremden Sessions verwalten.

Native Subagenten für vorhandene G-Pakete mit getrennten Schreibbereichen erlaubt, keine neuen T3-Threads. Bei BLOCK je Runde frischer nativer Fixer; gleicher Reviewer wie Runde 1, kein Modellwürfeln. Du führst die Fixschleife autonom, Rückgabe nur bei Task-Ende oder wirklichem Feststecken nach fünf erfolglosen Runden. Kein zusätzlicher Reviewthread. Auftrag reicht bis regulärem Merge/Push/Deploy/Restart/Livebeweis/Cleanup, nicht beim Plan stoppen.

## Arbeitsstand und Eigentum

Start-CWD/MCP-Root /home/nathanael/.worktrees/brain-g-v2-20261007, Branch feat/brain-v2-g-20261007, HEAD 8feb8b6ec0bf3dac7a8e180bfacc59ed001d3206. Beim Übergabecheck sechs Commits vor Trackingbranch, vorhandener WIP in combat.rs, data.rs, lib.rs und neue calculation.rs/calculation_tests.rs sowie 52 Auftragsakteneinträge. Das umfasst älteren uncommittierten G-WIP und muss vollständig erhalten werden; nicht auf die drei letzten Regressionen reduzieren, nicht resetten/stashen.

Eigene Akte .tasks/2026-10-06-brain-abschluss/ im G-Worktree. Zuerst AN_HAUPT-G.md, G/CHECKPOINTS.md, G/RECHENKERN-VERTRAG.md, G/ANTWORTPORT-VERTRAG.md, G/G-M-S2-BLOCK-NACHWEISE.md und G/G-M-S2-CARGO-SLOT-SPERRE.md lesen. Workflow wf_5503ece4-336 ist beendet, kein aktiver alter Writer laut Rückgabe. Vor neuem Fixer eigenen tatsächlichen Zustand erneut prüfen, keine Doppelworker.

G besitzt Kernel, Provider-/Werkzeug-/Rechenverträge und Reasoner, freigegebene Anschlüsse brain-contracts/src/entity_profile.rs und brain-serve/src/analytics.rs. analytics_runtime erst nach Is ausdrücklicher Eigentumsübergabe. Is Spiegel/Assets/Receipt/SQL und Fs Planer/Publish nicht parallel bearbeiten. K besitzt Consumer und zentrale Antwortverdrahtung. Kanonischer schmutziger Checkout unverändert. Zentrale REGISTER/TODO/PAKETE beim Delegator.

## Verbindliche Fortsetzung

Zentrale Akte /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-fertigstellung-astra/: AUFTRAG.md, PAKETE.md, ENTSCHEIDUNG-G-S2-RESTKERN.md, ENTSCHEIDUNG-WEITERBAU-2015.md, ENTSCHEIDUNG-DATENSCHUTZ-NUTZER-2045.md und BLOCKER-PRUEFWEG.md lesen. Neuere Nutzerentscheidungen schlagen ältere pauschale Sperrtexte.

1. Neues Root tatsächlich prüfen. Normales Read ist für Logs ausdrücklich freigegeben. Letzter cargo-slot-Lauf startete und endete mit Exit 101, also roter Prüflauf, kein Cargo-Startverbot. Ursache und Fallzahlen sind mangels Logauswertung noch unbekannt, jetzt aus G/pruefungen/g-m-s2-cargo-slot/test-vor-fix.log und laufstatus.json ermitteln, keine erfundenen Zähler.
2. Erhaltene drei Regressionen und zwei Helfer in combat.rs prüfen: stack_bonus_shred_matches_default_fast_and_binding_paths, explicit_stack_shred_is_applied_once_with_target_resistance, duplicate_items_have_one_stat_shop_and_effect_population_in_public_simulations. Die letzten 180 Testzeilen waren kein Produktfix. Deadlineprobe mit RequestDeadline::after(60s) hatte noch keine kontrollierte Zeitbasis, sauber prüfen statt Messwerte passend machen.
3. Bestätigte S2-Restkerne mit frischem nativen Fixer wirklich korrigieren: bullet_shred aus Stackbonus muss in default/fast/binding wirken; explizites Szenario darf Shred nicht doppelt anwenden. Doppelte Item-IDs an öffentlichen Simulationsgrenzen dürfen Stat-/Shop-/Effektpopulation nicht vervielfachen. Bestehende Normalisierung evaluate_core_with_deadline wiederverwenden, keine zweite Rechenstrecke. Alle bisherigen fünf Fixrunden (Kontaktzeit, ProcChance, Amplifikation, Nachfüllen, abgeschlossenes Nachladen/Druckdauer) erhalten.
4. Tatsächliche numerische Tests und vollständiger S2-Gate mit bisherigem Urteilmodell gpt-6.1-sol. Danach erhaltene S3/S4 und G-V-Produktionsvertrag abschließen. I/F brauchen deinen gesicherten geprüften Rechenkernvertrag, K braucht echte Tool-/Pin-/Verifieranbindung. Keine nur dokumentierte oder gefälschte Vertragsunabhängigkeit.

Letzter tatsächlicher Cargoaufruf als Referenz:
/home/nathanael/.local/bin/cargo-slot test --manifest-path /home/nathanael/.worktrees/brain-g-v2-20261007/rust/Cargo.toml --package dbrain-reasoner --lib --locked --offline --jobs 3 --target-dir /tmp/brain-g-m-0645-target combat::tests -- --include-ignored --test-threads=1 --nocapture

## Verträge und Reihenfolge

Bestehende answer_accounted/answer_turn_accounted, tatsächliche UsageAccounting einschließlich unbekannter und reservierter Nutzung, Originalrequest/AuthorizedContext/server pin/call_id, typisierte ToolRequest und vollständige Dependencyvalidierung für echten Zweck erhalten. InternalRead und ForPublication trennen; validate_build_plan an wirklichen deterministischen F-Output binden. Fail-closed Defaults nicht durch Erfolgsshims ersetzen. Kernel::with_tools, GameContextResolver und echte Versionsbindung gemeinsam prüfen. Keine lokale Matchablage, Publish-Schranken unverändert.

I versucht zunächst eine URL-Fixrunde und gemeinsamen Gate, liefert bei ALLOW Spiegel samt Discovery, trennt Discovery nur bei neuem Produktfund. Danach F/Warden, G, K. G darf parallel im eigenen Bereich geprüften Vertrag liefern; keine Sessionchat-/Wartefenster auf fremde Builds, keine Verletzung von Eigentum. Aktuelles main vor realer Integration neu prüfen, kein alter ALLOW als kombinierter Freibrief.

## Werkzeuge, Datenschutz und Abschluss

Cargo ausschließlich über cargo-slot. Kein Hook-/Rechteumbau, keine alten FD/flock-Schleifen oder Umgehungswrapper. Erneute tatsächliche Schutzablehnung präzise zurückmelden. Vor Codebestandssuche code-suche/Graphify. Vor Browserarbeit /home/nathanael/Documents/claude-config/wissen/agent-browser.md lesen und an Worker geben. Agenten MUST NOT Brave starten, übernehmen oder indirekt als Rückfall benutzen.

Nur Rust und Postgres, bestehende zentrale Provider. Kein Sonnet, kein Fable-Implementierer, kein neuer LLM-Connector oder eigenmächtiger Modell-/Timeoutwechsel. Secrets NEVER ausgeben. Private Originale und Community-Rohdaten MUST NOT an Codiermodelle/Git gehen. Luna-Testfreigabe gilt nur für minimalen bereinigten Antwortkontext ohne Discord-/Steam-IDs, Mitgliederlisten, fremde Personendaten, unter tatsächlicher Rollenbindung; keine harte Kategoriesperre. Q beginnt erst nach I/G/K-Livegang, hier nicht starten. ai-coach, fremde Dienste, Docs/Concierge-Löschung tabu.

Passende Tests/Format/Clippy, bestehende Suites erhalten. Einziger Reviewer lokaler Gate. Git einzeln mit literalen absoluten Pfaden, nur eigene Dateien, kein add -A, Main-Push HEAD:main. Nach echtem ALLOW regulär mergen, pushen, Deploy des aktuellen origin/main über serialisierten Wrapper, Release im eigenen Worktree. Vor Deploy laufenden Binary-Branch prüfen, danach SHA/Prozess/Health/Ready/Journal/Funktion belegen. Vor Cleanup wertvolle Artefakte und Ancestor-Exitcodes prüfen. MERGEPROTOKOLL[MS-1] im Bericht. Gebaut/reviewt/gemergt/live trennen, keine falsche Fertigmeldung. Berichte in bestehender G-Akte, Rückfragen als FRAGE AN ORCHESTRATOR. Neuer Thread settlet sich erst nach echtem eigenen Abschluss.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 30 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
