status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-q

# Q10: kleinsten Consumer-/Auditstand fertig prüfen

## Ziel und Vertrag

Native Workerrolle im gewählten Modell, keine weitere Delegation. Priorität des Hauptorchestrators: vollständig geprüften begrenzten Consumer-/Auditcommit an Z liefern, bevor Writer, Retention und Shadow den Abschluss verzögern. Lies `Q6-CONSUMER-PRUEFUNG.md`, `AN_HAUPT.md` und `VON_HAUPT.md` dieser Akte. Vorhandene Arbeit fortführen, nicht neu bauen.

Der tatsächliche jüngste Nachlauf ist beendet: HTTP 7 bestanden, Librarys API 10, Client 2, Serve 15 bestanden und 11 fehlgeschlagen. Sieben Panics mit `ConfigInvalid("docs_client_grant")`, vier Erwartungen gültiger Konfigurationen scheitern. Build und striktes Clippy sind grün. Formatabweichung ausschließlich Importlayout in `brain-api/src/internal.rs`. Logs sind Append-Logs, nur letzte Nachprüfung zählen. Keine Altfehlerbehauptung ohne Zahlenbaseline.

Ermittle die wirkliche Ursache der Serve-Konfigurationsfehler und behebe sie eng. C9-Grenzen unverändert: docs-client/docs ausschließlich docs.public mit public-Egress und explizitem Release; second-brain/internal ausschließlich second_brain.internal ohne Egress, festes Release identisch mit internal_operator.release; interner Grant nie in öffentlicher Registry. Produktionsvalidierung nicht lockern, Tests nicht abschwächen. Eine veraltete Beispielkonfiguration muss denselben wirklichen Vertrag erfüllen. Keine Modell-/Timeoutänderung und keine öffentlichen Operatorroute.

## Eigentum

Vorhandene Q-Consumerdateien unter `brain-api` und `brain-serve/src/audit.rs`, `src/main.rs`, `src/lib.rs`. Für die konkrete docs_client_grant-Ursache zusätzlich engste bestehende Serve-Konfigurations-, Fixture-/Secret-Test- und Beispielkonfigurationspfade, nachdem Graphify die Stelle beziehungsweise seine Abdeckungslücke gezeigt hat. Keine Feed-/Ingestion-/Storage-/Shadow-/Relevanz-/Provideränderungen, keine Manifeste oder Locks. Providerprüfungen dürfen laufen, Quellstand ist bereits integriert. Keine neuen Code-Kommentare, keine globale Formatierung. Hauptsession schreibt Status und gemeinsame Berichte; du schreibst dort nichts.

## Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-fertig-q`, Branch `feat/brain-fertig-q-20261003`, HEAD `5c220a8f047eb980d953d9f9f285b34739b5ed88`. Eigenes WIP erhalten. Z integriert gemeinsam, nimmt unabhängig ab, führt den einzigen Gate aus und installiert. Kein eigener Commit, Push, Merge, Deploy, DB-Schreiben, Secret-/Anbieteraufruf oder T3-Thread. Gib nach Grün die exakte begrenzte Dateiliste für den Consumer-/Providercommit zurück; C9 ist bereits im HEAD.

Frische Hauptsessionprobe nach 18:30 UTC: bekannte Q6/Q7/Q9-Prüfer beendet, keine zum Q-Worktree gehörenden Compiler-/Sperrwarteprozesse. Vor deinem Start trotzdem aktuell prüfen. Keine Nachrichten in laufende Workflow-/T3-Kontexte. Keine fremden Prozesse stoppen.

## Beweisziel

code-suche, rolle-test-waechter, humanizer und no-em-dashes anwenden. Lies den aktuellen HOSTPROBE.md-Vertrag unter `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/`. Beide Hostlocks tatsächlich blockierend halten, frische NonZombie-Probe, reine Sperrwartezeit ohne Timeout. Maximal zwei Cargo-Jobs, serieller Prüfwrapper, volle Logs im Q-Worktree und echte Exitcodes. Rust 1.97.1, --locked --offline, Tests mit --include-ignored.

Nach Fix sämtliche bestehenden lokalen Consumer-/C9-/Audit-/Serve-Suites in brain-api, brain-client und brain-serve prüfen. Kein all-workspace-Lauf auf unfertiger Retention. Check beziehungsweise Binarybuild, striktes Clippy und gezieltes Rustfmt-check der tatsächlichen Commitdateien. Vorhandenen Providerstand unverändert mit dessen lokalen Tests und Clippy bestätigen, soweit für denselben begrenzten Commit erforderlich. Fehler selbst eng beheben; eigentumsfremde Fehler mit Ort melden. Keine neuen Bug-/Securityreview-Agenten oder Gate außerhalb Zs gemeinsamer Integration.

Bei Toolwartezeit existierenden eigenen PID erhalten, nie doppelten Lauf starten. Vollständige Befehle, Sourcebindung, passed/failed/ignored/filtered, Format- und Build-/Clippy-Exitcodes berichten. Grüne lokale Prüfung ersetzt keinen Live-, Anbieter- oder gemeinsamen Gatenachweis.

## Routing

Auftraggeber Teil Q, Paket q, Versuch 1, Produzent teil-q. Hauptorchestrator Codex /root, T3 e6c19079-657e-4db9-80bd-8e1313e7f785. Rückgabe ausschließlich an diese native Hauptsession mit Ursache, engstem Fix, exakten Pfaden und vollständigen Prüfnachweisen. Keine Statusereignisse, TODO.md oder REGISTER.md. Keine weitere Delegation. Kein Modellwechsel, kein Sessionneustart.
