# Paket I: Register

status: aktiv, 2026-10-07

Intent-/Auftragsthread: `8827da25-c1f8-44f2-bef8-f3a7b7dd3137`. Delegator: `481426fe-b477-42b3-91c6-901811fcba1d`. Haupt-Orchestrator: `d3a1741e-82bc-4a48-865b-2845c663dca7`. Übergabe ausschließlich als `AN_HAUPT-I.md`, keine Sessionkontakte.

| Bereich | Arbeitsbaum | Branch | bestätigter Stand | Status |
| --- | --- | --- | --- | --- |
| E | `/home/nathanael/.worktrees/brain-e-deadlock-api` | `feat/brain-deadlock-api-daten` | `438b7bfac0eb70a3912e766c26427148e328c9d1` | tatsächliche Schreibziele geschützt; begrenztes ALLOW, gemeinsame aktuelle Integration ausstehend |
| F | `/home/nathanael/.worktrees/brain-f-publish` | `feat/brain-build-publish-ohne-matchgrenze` | `46fd86743589910d7b92a7223bdd6ab0dcf2b7c8` | übernommen, noch unverändert |
| Releasequelle | `/home/nathanael/.worktrees/brain-i-release-20261007` | detached | `3ceb504d6cebc8dda6e7438a40acd8e7570128be` | sauber auf frisch geholtem Main; regulärer Releaseplan geprüft; kein Build oder Install |

Zehn abgeschlossene native Fixer, keine neuen T3-Threads. Fix-SHAs und wörtliche Gateurteile stehen in `REVIEW.md`. Fixer 10 erhielt für die tatsächlichen Raw-/Cache-/Source-Schreibziele ein begrenztes ALLOW. Absolute konfigurierte Pfade und Originalspeicherung bleiben erhalten. Noch kein Gesamt-ALLOW, Main-Push oder Liveabschluss; die reale DotEnv-CLI-Probe bleibt wegen R12 ausgelassen. `eeb4116c` ist auf dem eigenen Featurebranch gesichert.

Nach erneutem Fetch bestätigt `origin/main=3ceb504d6cebc8dda6e7438a40acd8e7570128be`; seit gemeinsamem `f6f5cef6` haben Main und E keine gemeinsam geänderten Pfade (12 beziehungsweise 59). Noch keine Integration daraus behauptet. Der eigene Releasebaum wurde sauber auf `3ceb504d` weitergesetzt; der reguläre Wrapperplan bestätigt denselben Remote-main sowie Tree `21ac878933b4762e689b84579743eb6556b33131` und Fingerprint `5abc4c1337edf320de784c4242be35137813649191755a6b0111146545951ac1`. Das ist eine Quellvorprüfung, kein Build oder Deploy. Noch kein eigener Main-Push, Deploy, Import oder Cleanup. Tatsächlicher Live-Vorcheck: PID 2645590, Binary `bfda408cb988722ddceadb56bca5b72e12d12731`, `/healthz` meldet `ok`, `/readyz` meldet `ready`, beide JSON und HTTP 200. Steam/GC ebenfalls verbunden. Die zuerst angefragten `/health` und `/ready` waren falsche Routen mit 404, kein Dienstfehler.

G wird nicht bearbeitet. Neuester lesend bestätigter committed Lieferstand `54f76ba0315df6299d8bdc5e5c7c9f380d54d22c` enthält nach S1 die Kampfsimulation S2. Der Bericht meldet Wachstumsschnittstellen; eine gezielte Prüfung der committed Reasoner-/Vertragsquellen fand die fünf gemeldeten Namen an diesem Stand noch nicht. Keine vollständige Übernahme behauptet. Übernahme erst nach eigener Prüfung des tatsächlich committed Vertrags; kein fremder WIP.

E-Vertragsergänzung: derselbe SQL-Leser liefert gepaarte Payload-/Run-/Manifest-/Originalhash-Belege und explizite Run-Bindung. Der vorhandene Import umfasst 13 versionierte Endpoint-/Sprachkombinationen. Die dauerhafte Sicherung der Originalbytes ist noch nicht abschließend freigegeben; ein expliziter Liveimportpfad allein behebt nicht den Timer- und Wiederverwendungsfall.

F-Compilerbaseline auf unverändertem `46fd86743589910d7b92a7223bdd6ab0dcf2b7c8`: `SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo check --locked --manifest-path /home/nathanael/.worktrees/brain-f-publish/rust/Cargo.toml -p dbrain-reasoner -j 2`, Exit 0, 1m 09s. Beleg `/tmp/brain-i-f-baseline-check.log`. Zusätzliche reine Composerbaseline: `SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo test --locked --manifest-path /home/nathanael/.worktrees/brain-f-publish/rust/Cargo.toml -p dbrain-reasoner --lib composer::tests -j 2 -- --test-threads=1`, Exit 0, 28 passed, 0 failed, 0 ignored, 275 filtered. Beleg `/tmp/brain-i-f-composer-baseline.log`. Kein F/G-Anschlussbeweis.

Prüfungen: 536 passed, 0 failed, 24 ignored im bestehenden Paketlauf; gesonderte echte öffentliche Assets-/Scratch-PG-Probe 10 passed, 0 failed, 0 ignored; strikter Parser 3 passed. Formatcheck und Clippy bestanden. Eigene neue CLI-Testabhängigkeit und die vorhandene verkürzte Mirror-Scratch-Schemafixture wurden vor diesem grünen Schlusslauf korrigiert. Ausgelassene isolierte DB-/Browserfälle werden nicht als Produktivbeweis gezählt.

TESTNACHWEIS[TW-1]: 545 passed, 19 ignored | Baseline: keine Altfehler behauptet

Fixer-10-Schlussprüfung am `438b7bfa`: `env -u DEADLOCK_CENTRAL_DSN -u DATABASE_URL SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo test --locked --manifest-path /home/nathanael/.worktrees/brain-e-deadlock-api/rust/Cargo.toml -p deadlock-brain -p deadlock-brain-core -p dbrain-sources -p brain-storage -j 2 -- --test-threads=1`, Exit 0; 545 passed, 0 failed, 19 ignored, 30 Testtargets. Original `/tmp/brain-e-fixer10-tests-final.log`. Reale CLI-Pfadproben 10 passed, 0 ignored. Begrenztes Gate ALLOW, Format und striktes Clippy Exit 0. Keine Produktivbelege aus ausgelassenen DB-/Browserfällen ableiten.

Fixer-8-Schlussprüfung am `879e4cc3`: `SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo test --manifest-path /home/nathanael/.worktrees/brain-e-deadlock-api/rust/Cargo.toml -p deadlock-brain -p dbrain-sources -p brain-storage -p deadlock-brain-core --all-targets -j 2 -- --test-threads=1`, Exit 0; 538 passed, 0 failed, 19 ignored. Original `/tmp/brain-e-fixer8-tests-final.log`. Abweichende Auswahl gegenüber dem früheren 536er-Lauf: ohne `dbrain-builds` und ohne Doc-Tests, mit allen Targets. Keine Behauptung einer identischen Vergleichsbasis.

Weitere F-Baseline am unveränderten `46fd8674`: `SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo test --locked --manifest-path /home/nathanael/.worktrees/brain-f-publish/rust/Cargo.toml -p dbrain-reasoner --lib planner::tests -j 2 -- --test-threads=1`, Exit 0; 8 passed, 0 failed, 0 ignored, 295 filtered. Original `/tmp/brain-i-f-planner-baseline.log`.

Cleanup-Vorprüfung, noch keine Löschung: E enthält als ignoriertes Artefakt nur `rust/target`, der eigene Releasebaum keines. F enthält neben `rust/target` 20 vorhandene Prüf-/Publikationslogs im Aufgabenordner. Diese vor späterem Cleanup lokal erhalten, nicht ungeprüft mitsamt möglicherweise privaten Inhalten nach GitHub übertragen. Vor der tatsächlichen Löschung Inventar und laufende Binary-Pfade erneut prüfen.
