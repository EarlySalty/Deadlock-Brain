status: aktiv
Datum: 2026-10-03

# Gemeinsamer C9-/Adapterstand: Lockdatei und Bauprüfung

[Orchestrator] Paket Z, Versuch 2. Übernimm den vorhandenen, beendeten Adapterbau; kein Neubau. Auftraggeber ist diese native Z-Hauptsession. Gesamt-Hauptorchestrator Codex /root, T3 `e6c19079-657e-4db9-80bd-8e1313e7f785`. Fachbericht ausschließlich an Z, keine anderen Sessions oder Threads.

## Ziel und Ausgangsstand

Worktree `/home/nathanael/.worktrees/brain-fertig-z`, Branch `feat/brain-fertig-z-20261003`, letzter bestätigter HEAD `b86353acca8027431de58a2edadbebcdd01bad10`. Ein anderer eigener Fixer darf eng begrenzte Ops-Commits erstellen, darum HEAD vor Rückgabe frisch nennen. Vorhandene C9-Commits `e48c189` und `5c220a8` liegen vorbereitet im Index; vier zusätzliche Adapterdateien sind uncommitted. Der Kandidatenworker ist tatsächlich beendet. Keine fremde Arbeit löschen, zurücksetzen oder neu anfangen.

Lies `bereiche/z/BETRIEBSVERTRAG.md`, `VON_HAUPT.md` und die tatsächlichen Cargo-Prüflogs des beendeten Kandidatenauftrags. Seine Hashaufnahme `/tmp/brain-z-candidate-sources-post-v4.log` nennt die unveränderten Eingaben. Ergebnis: Fmt und Formatcheck Exit 0, Clippy/Adaptertests/bestehende Suites jeweils Exit 101 vor Compilerstart wegen nicht synchroner Workspace-Lockdatei. 16 lokale Tests sind definiert, keiner ausgeführt. Das ist kein erfolgreicher Compiler- oder Testlauf.

## Eigentum

Alleinige Schreibzuständigkeit für diese vorhandenen Dateien:

- `rust/Cargo.lock`
- `rust/crates/brain-maintenance/src/bin/brain-candidate-activate.rs`
- `rust/crates/brain-maintenance/src/integration/activation.rs`
- `rust/crates/brain-serve/src/config.rs`
- `rust/crates/brain-serve/src/health.rs`

Der zusätzliche Lockumfang wurde vor deinem Start in BETRIEBSVERTRAG.md angekündigt. Keine anderen Manifeste, Exports, C9-, Storage-, Consumer-, Ops- oder Produktivdateien ändern. Unerwartet nötige weitere Datei mit Ursache melden, damit Z sie vor dem Schreiben konkret ankündigen kann. Zwei andere eigene Worker schreiben disjunkt in `ops/brain-release/` und `ops/brain-postgres/legacy-refresh/`. Keine Git-Schreiboperationen, kein Commit/Push/Merge, keine weitere Delegation. Keine neuen Codekommentare oder globale Formatierung.

## Arbeit und Vertrag

Vor Codefragen Graphify, danach tatsächliche Fundstellen. Workspace-Lockdatei minimal an die schon vorbereiteten Manifeste anpassen, ohne breit angelegte Versionsupdates und ohne frei neue Dependencies zu erfinden. Vorhandene gelockte Versionen erhalten, soweit ihre Verträge das zulassen. Die vorbereitete Transportversionskorrektur gehört zum C9-Vorstand.

Danach den erhaltenen Adapter tatsächlich kompilieren, seine 16 Tests und passende bestehende betroffene Suites ausführen; echte Compilerfehler innerhalb deines Umfangs korrigieren. Bestehende Tests weder löschen noch abschwächen oder still überspringen.

P aktiviert ausschließlich Standard-/Patchnotesbindung. Q aktiviert ausschließlich Second-Brain-Grant.release und internal_operator.release atomar und übereinstimmend. Öffentliche Docs-/Twitch- und Standardbindung bleiben bei Q unverändert. Basisbindung/Serialisierungshash und exakte erwartete Konfigurationsbytes unter gemeinsamen Publikations-/Configsperren frisch prüfen. Fremde Pins erhalten, bestehendes Journal/Restart/Readiness/Rollback benutzen. Geladener gemeinsamer Bindungsdigest muss die tatsächliche Serverkonfiguration bestätigen, keine privaten Inhalte oder Credentials offenlegen. Ohne `--apply` kein Konfigurationstausch und kein Restart. Keine Storepublikation oder Anbieteraufrufe aus dem Aktivierungsadapter.

## Prüfmechanik

Tatsächliches Rustup-Cargo `/home/nathanael/.cargo/bin/cargo`. Jeder Compiler oder Cargo-Resolver unter beiden vorhandenen Hostlocks: zuerst `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock`, danach `/tmp/deadlock-cargo-release.lock`. Identitäten und NonZombie-Probe frisch, maximal zwei Jobs. Bei fremden Compilern beide Locks halten und nach 30 Sekunden erneut prüfen. Keine Wartezeitbegrenzung, keine fremden Prozesse stoppen. Nur exakt `cargo metadata --format-version 1 --no-deps --manifest-path /absoluter/pfad/Cargo.toml` ist als Ausnahme belegt; alle anderen Cargo-/Rustc-/Rustfmt-/Clippyprozesse gelten als Compiler. Keine zweite eigene Prüfchain neben deiner tatsächlich laufenden.

Nur eigenes privates/ignoriertes Target, kein globales CARGO_TARGET_DIR. Fmt eng auf eigene Dateien, Formatcheck der tatsächlichen betroffenen Gruppe. Clippy mit `-D warnings`, vorhandene Tests und mindestens tatsächlicher Compile-/Buildnachweis. Einzel-Exitcodes sicher erfassen, keine Pipes oder Schlussdruckbefehle als Erfolg werten. Passende bestehende Testfeatures/DSN nur über vorhandene Wege prüfen; keine ENV-Dateien, Secretwerte oder privaten Texte ausgeben. Keine Produktionsdatenbank als Testbasis und keine Provideraufrufe.

## Rückgabe

Genaue Änderungen und Lockdiff, tatsächliche Eingabehashes, Prüfbefehle, Einzel-Exitcodes, Testanzahlen, tatsächliche Fehlermeldungen und noch offene Dateien melden. Gebaut, geprüft, gemergt und live getrennt. Kein eigener Reviewthread oder eigener Bug-/Securityreviewer; zentrale Gateprüfung und unabhängige gemeinsame Intent-Abnahme folgen durch Z. Kein Schreiben in TODO.md, REGISTER.md oder zentrale Status-/Berichtsdateien. Deutsche Fachrückgabe, echte Umlaute, humanizer/no-em-dashes.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-z
