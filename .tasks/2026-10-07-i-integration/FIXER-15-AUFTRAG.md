# Fixer 15: eine tatsächliche URL-Identitätsfixrunde

status: aktiv, 2026-10-07

## Ziel und Vertrag

Eine frische fachliche Runde nach REVIEW-RUNDE-18.md. Fixer 13/14 waren Zugriffsblocker, keine fachlichen Versuche. Queryfreier Bestand gegen queryhaltigen Feed, Fragment- und Slashvarianten konsistent über bestehenden Lookup erhalten. Akzeptierte Steam-URL-Varianten nur bei nachgewiesen derselben konkreten Ereignisidentität verbinden. Event- und Announcement-GIDs niemals pauschal gleichsetzen. Kein neuer Parser, keine Titel-/Zeitgleichheit als Identitätsbeleg, keine Änderungen der Herkunfts-/Netz-/Publishgrenzen. Die zwei NITs bleiben offen. NACHWEIS-CORE6-GLOBAL.md und vorhandene Readerbeweise erhalten.

Maßgeblich: REVIEW-RUNDE-18.md, FIXER-13-AUFTRAG.md, aktueller Nutzerauftrag, ENTSCHEIDUNG-WEITERBAU-2015.md Punkt 2. Genau eine Runde. Nach neuem inhaltlichem BLOCK keine weitere Discovery-Fixschleife. Werkzeugfehler sind kein Produktfund.

## Eigentum und Arbeitsstand

Worker, keine zusätzliche Delegation oder T3-Threads. Tatsächlicher Arbeitsroot und MCP-Projektroot der frisch gestarteten Hauptsession /home/nathanael/.worktrees/brain-e-deadlock-api. pwd, Branch und HEAD bestätigt. ctx_execute_file hat pg_patchnotes.rs tatsächlich erfolgreich gelesen (3058 Zeilen), keine Schutzänderung.

Branch feat/brain-deadlock-api-daten, Ausgang 9d17ee52ec898d52ed7c9e78feae596ad086487a, vor diesem Briefing sauber. Schreibbereich ausschließlich rust/crates/deadlock-brain/src/pg_patchnotes.rs, rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs und eigener FIXER-15-BERICHT.md in dieser Akte. Gemeinsamer Kandidat /home/nathanael/.worktrees/brain-i-release-20261007 auf 501d3725 und F /home/nathanael/.worktrees/brain-f-publish auf 46fd8674 bleiben unverändert. Keine G/K-Dateien, analytics_runtime oder zentrale Akte ändern. Hauptsession schreibt nur ihre Akten, nicht deine Quelldateien.

## Beweis und Git

Vor Suche code-suche/Graphify. Bestehende echte Scratch-PG-Probe erweitern: queryfreier Bestand gegen Query+Fragment, Slash in beiden Richtungen, gleiches tatsächlich akzeptiertes Steam-Ereignis und negative fremde Event-/Announcementfälle. Eigene isolierte PostgreSQL-Instanz ohne TCP, vorhandener Scratch-DSN-Eingang und geforderter DB-Name, keine Produktiv-DSNs oder schreibendes psql. Bestehende Tests/Format/Compiler/striktes Clippy, Cargo nur /home/nathanael/.local/bin/cargo-slot mit --jobs 3, env -u DEADLOCK_CENTRAL_DSN -u DATABASE_URL SQLX_OFFLINE=true, Tests seriell. cargo fmt nur --check, Formatänderungen nur eigene Dateien. Logs mit normalem Read erlaubt. Tatsächliche neue Schutzablehnung melden, keine Umgehungswege oder Hookänderung.

Eigene geprüfte Dateien gezielt stagen, kein add -A. Git einzeln mit literalen absoluten Pfaden. Committrailer Co-authored-by: GPT 6.1 Sol <gpt-6.1-sol@local>. Commit und Feature-Push erlaubt, kein Main-Merge/Push oder Deploy. Selbst-Gate nach Commit: /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-e-deadlock-api --base 9d17ee52ec898d52ed7c9e78feae596ad086487a --head <eigener SHA> --model claude-opus-5-5. Kein separater Reviewer. Bei BLOCK Urteil zurückgeben, keine zweite Runde. ALLOW ist nur begrenzte Selbstfreigabe, Hauptsession fährt danach gemeinsamen Gate.

## Routing und Grenzen

Auftraggeber ist diese I-Fortsetzung b17d5729-a475-4bcb-8fc9-0aa6103d4555, Delegator 481426fe-b477-42b3-91c6-901811fcba1d. Worker meldet Fachbericht als FIXER-15-BERICHT.md und Native-Endergebnis ausschließlich hierher. Hauptsession ist einziger I-Statusproduzent und Integrationsverantwortlicher. Keine Sessionchats, keine TODO-/REGISTER-Fremdänderungen. Bericht: Ausgang/Fix-SHA, konkrete Wirkung, Tests mit Exit/Zahlen, Gate vollständig, offen, gebaut/reviewt getrennt. Kein Cleanup oder Self-Settle.

Nur Rust/Postgres/bestehende Provider. Secrets NEVER ausgeben. Private Originale und Community-Rohdaten MUST NOT an Codiermodelle oder Git. Sonnet/Fable verboten. Keine Browserarbeit benötigt; vor nötiger Browserarbeit agent-browser.md lesen, ausschließlich Moli, MUST NOT Brave oder persönliche Browser. Fremde Dienste unangetastet.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 30 min | Worktree: /home/nathanael/.worktrees/brain-e-deadlock-api
