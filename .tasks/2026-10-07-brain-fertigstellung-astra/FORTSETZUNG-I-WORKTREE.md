[Orchestrator]
# I: E/F aus bestehendem Worktree bis live fortsetzen

## Auftrag und Rolle

Du bist der frische Fortsetzungs-Teil-Orchestrator für genau das bestehende Paket I (E: API-Spiegel, F: Build-Publish), kein Neubau. Nutzer hat diesen Sessionwechsel ausdrücklich beauftragt, BLOCKER-PRUEFWEG.md Nachtrag 20:50. Auftraggeber und Eigentümer der zentralen Akte: 481426fe-b477-42b3-91c6-901811fcba1d. Alter I-Thread 8827da25-c1f8-44f2-bef8-f3a7b7dd3137 ist nachweislich stopped und wird nicht wieder aufgenommen. Frühere Hauptsession d3a1741e bleibt gestoppt. Keine fremden Sessions verwalten.

Native Subagenten mit getrennten Schreibbereichen sind im bestehenden I-Auftrag erlaubt. Je BLOCK frischer nativer Fixer, keine zusätzlichen T3-Threads oder Reviewerthreads. Du führst deine Fixschleife, nicht den Delegator je Runde fragen. Das ist eine Umsetzung bis regulärem Gate/Merge/Push/Deploy/Neustart/Livebeweis/Cleanup, kein Planauftrag.

## Arbeitsstand und Eigentum

Start-CWD und MCP-Projektroot müssen /home/nathanael/.worktrees/brain-e-deadlock-api sein, Branch feat/brain-deadlock-api-daten. Geprüfter HEAD 9d17ee52ec898d52ed7c9e78feae596ad086487a, Tree beim Übergabecheck sauber. Sessionstart ausdrücklich mit --worktree; nicht nur nachträglich cd. Zuerst Root/Branch/HEAD und vorhandene Akte prüfen, nichts zurücksetzen.

Primäre eigene Akte: .tasks/2026-10-07-i-integration/ im E-Worktree. Zuerst AN_HAUPT-I.md, REGISTER.md, REVIEW.md, REVIEW-RUNDE-18.md, REVIEW-RUNDE-20-MCP-ROOT.md und FIXER-14-NACHTRAG-2030.md lesen. Produktkandidat 501d3725, bisherige Suite 558 bestanden/25 ignoriert, zusätzliche Scratch-PG-Probe 1 bestanden. Das ist kein Gesamt-ALLOW. Erhalten: E-Spiegel-/Receipt-/Originaldateiänderungen, alle Fixcommits und Nachweise.

Weitere eigene übernommene Bäume: /home/nathanael/.worktrees/brain-i-release-20261007, Branch feat/brain-i-integration-blocked-20261007; F /home/nathanael/.worktrees/brain-f-publish, Branch feat/brain-build-publish-ohne-matchgrenze, gesicherter F-Stand 46fd86743589910d7b92a7223bdd6ab0dcf2b7c8. Vorbereitungsbranch feat/brain-assets-mirror-20261007 erhalten, kein Auftrag zum sofortigen Schnitt. Aktuelle Stände selbst prüfen. Keine G-/K-Dateien ändern, gemeinsamen analytics_runtime nach E ausdrücklich als Datei an G freigeben. Kanonischer schmutziger Checkout unverändert; zentrale REGISTER/TODO/PAKETE gehören dem Delegator.

## Verbindliche Fortsetzung

Zentrale Akte /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-fertigstellung-astra/: AUFTRAG.md, PAKETE.md, ENTSCHEIDUNG-WEITERBAU-2015.md, ENTSCHEIDUNG-DATENSCHUTZ-NUTZER-2045.md und BLOCKER-PRUEFWEG.md in aktueller Fassung lesen. Neuere Nutzerkorrekturen schlagen alte Fachberichtsanweisungen.

1. Genau eine frische FACHLICHE Fixrunde für URL-Varianten/Bestands-ID, etwa queryhaltiger Feed gegen queryfreien Bestand. Die bisherigen Rootablehnungen waren kein fachlicher Versuch. Äquivalente Original-URLs konsistent binden, unterschiedliche Steam event-/announcement-GIDs niemals vermischen. Kein zweiter Parserpfad. Reader-Gegenbeweis NACHWEIS-CORE6-GLOBAL.md erhalten; JOIN vor LIMIT und echter älterer Global-Run sind belegt.
2. Danach echter gemeinsamer Gate mit bisherigem urteilsgebendem Modell Claude Opus 5.5. Bei ALLOW Spiegel UND Patch-Discovery gemeinsam nach main und live. Nur bei neuem inhaltlichem Fund Discovery auf feat/brain-patch-discovery erhalten/herauslösen, bestehenden Patchimport wie main lassen und Spiegel separat regulär gateprüfen/liefern. Toolfehler sind kein neuer Produktfund und lösen den Schnitt nicht aus. Keine unbegrenzte weitere Discovery-Fixschleife.
3. Live-Spiegel durch echten vollständigen Import, mirror_complete, Reader-/Receipt-/Originaldateibindungen belegen. Keine Matchdaten lokal speichern, Deadlock-API spiegeln.
4. F danach auf realem aktuellen Main integrieren, vorhandenen G-Rechenkernvertrag nutzen statt zweiter Rechnung. plan_build_with_playstyle, Budget/Imbues, ursprüngliche Deadline/Abbruch, PurchasePlan/InventoryEvaluation und ForPublication bleiben verbindlich. Publish-Schranken nicht senken. Regulärer Warden-Publish mit echter hero_build_id ist der Beweis, kein synthetisches Erfolgsobjekt. Abschlussfolge Spiegel/F/G/K, G darf unabhängig im eigenen Bereich seinen geprüften Vertrag vorbereiten.

## Werkzeuge und Grenzen

Logs gemäß neuer Nutzerfreigabe mit normalem Read lesen. Cargo nur über /home/nathanael/.local/bin/cargo-slot, keine alten FD/flock-Schleifen. Neue tatsächliche Zugriffsablehnung melden, keine Hooks/Rechte verändern oder Umgehungswrapper bauen. Vor Codebestandssuche code-suche/Graphify. Vor Browserarbeit /home/nathanael/Documents/claude-config/wissen/agent-browser.md lesen und an Worker weitergeben. Agenten MUST NOT Brave starten, übernehmen oder indirekt als Rückfall benutzen.

Nur Rust, Postgres, bestehende zentrale Provider. Keine eigenmächtigen Modell-/Timeoutwechsel, kein neuer LLM-Connector. Kein Sonnet oder Fable-Implementierer. Private Originale/Community-Rohdaten MUST NOT an Codiermodelle oder Git gehen. Für bestehende Luna-Antworttests gilt nur die enge aktuelle Testfreigabe: notwendiger bereinigter Kontext, Rollenbindung, keine Discord-/Steam-IDs, Mitgliederlisten oder fremden Personendaten. Secrets NEVER ausgeben. Keine harte Kategoriesperre hinzufügen. Keine Docs-/Concierge-Löschung, keine fremden Dienste oder ai-coach anfassen.

## Beweis und Rückgabe

Bestehende Suites erhalten, passende echte Tests/Format/Clippy. ALLOW nur für geprüften Scope/SHA. Einziger Reviewer lokaler Gate; gleiches Urteilmodell bei Folgerunden, bei BLOCK frischer Fixer. Git-Schritte einzeln, literale absolute Pfade, nur eigene Dateien stagen, kein add -A, Main-Push HEAD:main. Merge-Gate niemals umgehen. Deploy nur aktuelles origin/main über regulären serialisierten Wrapper, Release im eigenen Worktree. Vor Deploy Branch des laufenden Binaries prüfen. Danach Prozess-/SHA, Health/Ready, Journal und echte Funktion belegen; vor Cleanup wertvolle Artefakte und Ancestor-Exitcodes prüfen. MERGEPROTOKOLL[MS-1] im Bericht.

Eigene Fortschritte/Fachrückgabe in vorhandener I-Akte. Gebaut/reviewt/gemergt/live getrennt. Nur echten Endabschluss oder qualifizierten Blocker melden. Q startet erst nach I/G/K-Livegang, hier keine Evaluation starten. Keine Sessionchats, Rückfragen als FRAGE AN ORCHESTRATOR im eigenen Bericht. Neuer Thread settlet sich erst bei echtem vollständigem eigenen Abschluss.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 30 min | Worktree: /home/nathanael/.worktrees/brain-e-deadlock-api
