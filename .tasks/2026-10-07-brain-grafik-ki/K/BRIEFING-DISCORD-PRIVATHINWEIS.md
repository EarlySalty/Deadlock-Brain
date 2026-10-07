# K: Privaten Hilfshinweis begrenzen

## Ziel und Vertrag

Gemeinsamer Consumer-/Logtestgate 56571e40..4b36999d, Task b11b6qt1a, regulär ALLOW. Neuer NIT an modglue.rs:748: private Hilfshinweise werden vor handle_discord_query gesendet und umgehen dessen Reservierung. K hat Aufrufkette nach Graphify gelesen: answer_discord_event prüft can_reply und is_public, private Branch reserviert nichts; DM und private !brain/Erwähnung laufen darüber. Slash hat ebenfalls private frühe Rückgabe. Die bestehenden BrainCooldowns/DiscordRateState reservieren Nutzer, Kanal und Tagesbudget. Den berechtigten Kern durch Wiederverwendung dieser bestehenden Mechanik beheben. Kein eigener zweiter Limiter, kein Modell-/Timeoutwechsel und keine private Antwortengine. Privaten Rohtext weiterhin nie an Provider weiterreichen.

## Eigentum und Stand

Worktree /home/nathanael/.worktrees/bots-k-guide-20261007, Branch feat/bots-k-guide-20261007, HEAD4b36999d regulär ALLOW und auf origin. Voriger Logtestfixer abgeschlossen, keine Bots-Writer. Exklusiver Schreibbereich bestehende private Hilfshinweispfade und notwendige Reservierungsauslagerung in rust/bin/dl-bot/src/modglue.rs sowie rust/crates/dl-brain/src/lib.rs und deren eng zugehörige Tests. brain_api.rs, andere Consumer, Konfiguration, Prompt und ursprüngliche Antworttexte unverändert. Kein pauschales Refactoring. Öffentliche Antworten genau einmal reservieren, keine Doppelreservierung. Parallel nur Brain-Testfixtureprüfer a7fe6a42abeb6ebb1 in anderem Repo. Kein Commit, Push, Merge, Deploy, Restart oder weiterer Agent. K allein sichert.

## Nachweise

Graphify vor eigener Bestandssuche. Vorhandene Limits wiederverwenden, private Hilfshinweise als echte automatische Antworten behandeln; direkte DM, private Erwähnung/!brain und Slash-Zwilling prüfen. Tests dürfen durch wirklich veränderte legitime Reservierungswirkung nachgezogen werden, nicht pauschal abgeschwächt. Belegen: Wiederholung gedrosselt, kein Provideraufruf bei privat/ungeklärt, Identitäts-/Kanalgrenze erhalten, öffentlich kein versehentlicher Null-Lauf durch Doppelreservierung. Keine neue echte Wall-Clock-Wartezeit in Tests. Bestehende dl-brain- und modglue-Auswahl mit `--include-ignored`; Compiler, scoped Format und passende strikte Clippy-Auswahl. Cargo/Rust1.99.0 für Baselinevergleich, bisher Library3 und Bot-only57 rot, echte Logs K/ZENTRALANTWORT-BAU.md. Exakte Befehle/Exits/Zahlen/Logpfade, keine pauschale Vorherbehauptung.

Absolute Cargo-CLI, SQLX_OFFLINE=true als Prüfkonfiguration, vorhandene Caches. Freien Buildslot /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/build-slot-{1,2,3}.lock tatsächlich halten, höchstens `--jobs 3`. Keine fremden Prozesse stoppen oder Test-Exits maskieren. Regulären Gate fährt K auf Commit, kein zusätzlicher Reviewer.

## Verifizierter technischer Arbeitsweg

Erster Worker a601550b5dcb28578 ist ohne Änderung/Test beendet. K hat ausschließlich seine Fehlerrecords abgeleitet, nicht das volle Transcript gelesen. Der EnterWorktree-Aufruf konnte den beauftragten fremden Repo-Worktree nicht wechseln. context-mode ctx_execute_file ist auf den kanonischen ursprünglichen Projektroot beschränkt, native Read ist für beauftragte Dateien der richtige Weg. Die Cargoablehnung nennt ausdrücklich eine nicht auflösbare plain flock-Testkommandoform und fordert Ausführung aus /home/nathanael/.worktrees/brain-k-ki-20261007. Das belegt keine fehlende fachliche Bots-Freigabe.

Frischer Fixer behält Parent-CWD Brain-K. Kein EnterWorktree, kein cd in Bots oder Kanon, keine Settings-/Permissionsänderung. Absolute native Read/Edit-Dateipfade und Cargo mit absolutem Bots-Manifest. Für die geforderte eindeutige Kommandoform darf ein Prüfaufruf mit `cd /home/nathanael/.worktrees/brain-k-ki-20261007 && flock <literaler freier Slot> /home/nathanael/.cargo/bin/cargo ...` beginnen; dies bestätigt nur den bestehenden eigenen CWD, wechselt kein Repo. Slot bleibt gehalten, Tests nicht aus dem Lock lösen. Wird auch diese korrigierte Form vor Ausführung verweigert, stoppen und den genauen Deny zurückgeben, keine weitere Befehlsvariation oder Umgehung. Keine fremden Ressourcen oder Zielrechte erweitern.

Stand nach Rückgaben: a601550b5dcb28578, ae2b217d8863173dd und aba35c6872805ca22 jeweils ohne Änderung/Test beendet. Die vorgegebene cd-Form ist in ihren sichtbaren Werkzeugaufrufen nicht belegt; keine weitere native Wiederholung. K hat den eigenen CWD durch pwd bestätigt. Drei eigene sichtbare flock-Aufrufe ohne diesen Präfix und einmal transparentes `env --chdir=/home/nathanael/.worktrees/brain-k-ki-20261007` wurden vor Prozessstart mit demselben Wrapper-/gitfrei-Deny abgewiesen. Keine Cargo-Exitcodes, Testzahlen, Settings-/Hook-/Rechteänderungen oder Umgehung. NIT bleibt offen; keine weiteren Kommandoexperimente. Brain-Core-Bestandsprüfung läuft unabhängig weiter.

## Sicherheit und Routing

NEVER read, print or write plaintext secrets.
MUST NOT send private user/community data to remote models.

Synthetische Fixtures, keine privaten Originalfragen, Secrets oder echten Kanalproben. Kein ListAgents, SendMessage, T3-Thread, zentrale Register-/TODO-Edits oder G-WIP. Rust/Postgres, keine neuen Code-Kommentare. K-Session988eeaea-28ee-424c-b362-e250610cde91, teil-k, Versuch1; Delegator481426fe-b477-42b3-91c6-901811fcba1d, Hauptorchestrator d3a1741e-82bc-4a48-865b-2845c663dca7. Nur Abschluss oder echter Blocker als native Rückgabe.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/bots-k-guide-20261007
