status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-q

# Q9: geliefertes Shadowwerkzeug lokal verifizieren

## Ziel und Vertrag

Native Workerrolle, keine weitere Delegation. Q4 hat das Rust-Werkzeug bereits geliefert, nicht neu bauen. Lies `BRIEFING-Q4.md` und `architecture/migration/evals/q-shadow/profile.json`. Mindestens 100 erlaubte Fälle, vorgesehen 103: sieben veröffentlichte FAQ und 96 klar synthetische Varianten aus fünf öffentlichen Seiten. Keine Community- oder Nutzerfragen, keine neuen Modelle, kein neuer Providerconnector, keine echte Anbieteranfrage durch dich.

Prüfe den bestehenden Runner, kompiliere ihn und führe ausschließlich sein Offline-prepare aus. Bestand und Kandidat müssen später tatsächliche HTTP-Antwortpfade mit gleichem gepinntem Corpus, wirksamer Modellwahl, Budget und Fristen sein. Vor Versand muss jede echte gepinnte Quellrevision samt aktuellem Recht und Herkunft geprüft sein. Offline-Profil und API-Audit allein beweisen keinen Provideraufruf oder Antwortqualität. Erfinde keine Goldlabels oder Nutzungs-/Kostenwerte.

Klärung vor echtem Lauf: Ein Vergleich darf nicht durch den gemeinsamen Produktivwechsel seine Baseline verlieren. Prüfe, ob das Werkzeug den tatsächlich laufenden Basispfad mit dem lokalen Kandidaten auf demselben Release verwenden kann. Wenn der neue C9-Import dafür nötig ist oder der Herkunftsvertrag nicht zu den vorhandenen freigegebenen Kerndaten passt, melde genau diese Voraussetzung. Keine Provenienz passend machen, keine DB-/Releaseänderung und keinen zweiten Servicepfad bauen. Einen erlaubten lokalen Prüfaufbau konkret empfehlen, aber kein laufendes Produktivsystem verändern.

## Eigentum

Nur vorhandene Q4-Pfade: `rust/crates/brain-serve/src/bin/brain-provider-shadow.rs`, `src/bin/provider_shadow/` und eigene Artefakte unter `architecture/migration/evals/q-shadow/`. Engste Compiler-/Formatfixes dort erlaubt. Keine bestehenden API-/Provider-/Serve-/Config-/Storage-/Feeddateien, keine Manifeste, Locks oder Migrationen ändern. Keine neuen Code-Kommentare, keine globale Formatierung. Direkte Nachrichten an laufende Workflow-Agenten erzeugten parallele Kontexte; für dich keine Nachrichten-/Resumierungskoordination und keine Kontakte zu fremden Sessions. Hauptsession verarbeitet allein dein Ergebnis.

## Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-fertig-q`, Branch `feat/brain-fertig-q-20261003`, HEAD `5c220a8f047eb980d953d9f9f285b34739b5ed88`; frischer origin/main `358ed4ee07d315d4d71dd0438f9446f5b6a22d49`. Alle gelieferten Änderungen erhalten. Q7-Writerfortsetzung läuft in getrennten Pfaden. Q6-Workflow ist geliefert, seine korrigierte Consumer-Nachprüfung bleibt als Bash PID 2923299 erhalten und wartet auf Hostlocks. Diese Prüfung hat Priorität, keinen zweiten Consumerlauf starten oder fremde Prozesse stoppen. Z integriert, nimmt gemeinsam ab, prüft Gate und installiert. Kein Einzelmerge oder Releasewechsel durch Q.

## Beweisziel

code-suche zuerst, bestehende Graphen und APIs nutzen. rolle-test-waechter, humanizer und no-em-dashes laden. Rust 1.97.1, höchstens zwei Jobs, beide Hostlocks blockierend in richtiger Reihenfolge halten, frische NonZombie-Probe nach HOSTPROBE.md. Keine Zeitlimits für reines Sperrwarten. Cargo gezielt für `-p brain-serve --bin brain-provider-shadow --locked --offline --jobs 2`: check, striktes clippy und test mit --include-ignored. Kein all-targets-Lauf auf unfertigen fremden Sourcepfaden. Danach Offline-prepare mit dem vorhandenen Profil und mindestens 100 eindeutigen Fällen ausführen, Ursprung und synthetische Kennzeichnung zählen und hashen.

Volle Logs im Q-Worktree, tatsächliche Exitcodes und Testzahlen. Bei Toolwartezeit den bestehenden eigenen PID erhalten und nennen, niemals erfolgreich melden oder erneut starten. Gib genaue run-Argumente, benötigte bestehende Konfig-/Credential-/Releasefelder, tatsächliche Herkunfts-/Rechte-Voraussetzungen und die Aussagegrenzen zurück. Kein PROVIDER_SHADOW_PASSED ohne echten Lauf. Eigentumsfremden Compilerfehler mit konkreter Fundstelle melden statt dort selbst zu ändern. Kein Bug-/Securityreview außerhalb des Gates.

## Routing

Auftraggeber Teil Q, Paket q, Versuch 1, Produzent teil-q. Hauptorchestrator Codex /root, T3 e6c19079-657e-4db9-80bd-8e1313e7f785. Rückgabe direkt an diese native Hauptsession; Rohbericht bleibt hier. Hauptsession legt nötigen Bericht in der gemeinsamen Akte ab. Kein TODO.md, REGISTER.md oder Statusereignis. Keine weitere Delegation oder neue T3-Threads. Gebaut, gemergt und live getrennt melden.
