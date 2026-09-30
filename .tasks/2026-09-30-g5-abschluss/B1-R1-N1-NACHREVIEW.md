# B1-R1-N1: unabhängiges Nachreview der Cargo-Weitergabe

Datum: 2026-09-30. fertig: J. Fix: N.

**Statisches GO: Der Dreistellenbefund R1-N1 ist geschlossen.** Die Cargoauflösung erfolgt vor der HOME-Isolation, und die Kindrunner erhalten den vorhandenen ausführbaren Einstieg. Keine CI-, Compiler-, Test-, DB-, Prozess-, Last-, Dienst- oder Deployausführung.

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft

## Eingefrorene Bindung

Geprüft wurde `8c31b09deea51ae00130a56a42efee7034b08d92..0c56f85cf2c76aec963db3d567009bcd461c19c0` aus `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930`, nicht der Arbeitsbaum. Genau zwei Workflowdateien, drei Einfügungen und drei Löschungen. Der Autorbericht aus `392267b133a9a5a8a91602247a441ff7bf609c4b` ergänzt gegenüber diesem Quellhead eine Berichtsdatei. Das separate B2-Urteil steht in `B2-R1-R3-NACHREVIEW.md`.

Berichtssenke ist der bestehende Reviewworktree `/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929` auf `review/pre-g5-core-abnahme-20260929`. Keine Produktänderung durch den Reviewer.

## Geprüfte Wirkung

1. `.github/workflows/brain-serve-c1.yml:75-76` übergibt `BRAIN_TEST_CARGO="$(command -v cargo)"` an den Serve-Runner. Die aufrufende Shell expandiert `command -v` vor Ausführung von `env -i`; die gleichzeitig übergebene neue HOME-Zeichenfolge ist dabei noch keine Änderung der aufrufenden Shellumgebung.
2. `.github/workflows/rust-core-verification.yml:69-70,87-88` tut dasselbe für Core-PG und Serve. `scripts/test_brain_core_postgres.sh:15` sowie `scripts/test_brain_serve.sh:15` verwenden damit den gesetzten Cargo-Einstieg statt `$HOME/.cargo/bin/cargo` unter dem isolierten Testverzeichnis. Die vorhandenen Toolchain-Installationsschritte liefern den Cargo-Einstieg; seine tatsächliche Verfügbarkeit in einem späteren Job wurde hier nicht gemessen.
3. HOME-Isolation, Cargo-/Rustup-Homes, der explizite absolute Targetcache samt Existenzprüfung, Ein-Job-Vorgabe und Pipeline-`pipefail` bleiben erhalten. Die Kindrunner verwenden weiterhin `+1.97.1 --locked --offline --jobs 1 --target-dir`. Kein neues Cargo, kein neuer Cache und keine produktive ENV-Konfiguration. Upgrade mit explizitem Cargoargument, Wiki und lokale Wrapper sind durch diesen Dreizeilenfix unverändert.

**Unabhängige statische Verifikation:** Graphify vor der Quellprüfung; die drei aus dem Ausgangsbefund bekannten Aufrufstellen gegen die eingefrorenen Empfänger geprüft. `git diff --check 8c31b09 0c56f85`: Exit 0. Die drei geänderten Shell-`run`-Blöcke aus den Git-Blobs separat an `/bin/bash -n` übergeben: dreimal Exit 0. Keine Wrapperausführung oder behauptete grüne GitHub-Ausführung; die Shell-Syntaxprüfung ist kein YAML-/CI-Lauf.

## Laufzeitgrenze und nächster Bedarf

Für diesen abgeschlossenen Dreizeilen-Quellfix entsteht **kein eigener zusätzlicher Cargo-Bedarf**. Der engste nächste Compilerbedarf kommt aus dem separat geprüften B2-Delta und ist dort exakt benannt: `cargo +1.97.1 check` für `brain-legacy-import` und `brain-storage`, `--all-targets --locked --offline --jobs 1` mit dem vorhandenen Targetcache. Kein automatischer Folgelauf; der zentrale Slot bleibt beim Twitch-Integrator.

Ein späterer Serve-Aufruf enthält inzwischen den angeschlossenen B2-Cutover-Scratchfall, echte lokale PG-/Importer-/Serveprozesse und weiterhin 600 Lastanfragen bei jeweils 8/16/32 Workern. Das sind 1.800 Lastanfragen zusätzlich zu den funktionalen Requests. Statisches CI-GO ist keine Zuteilung dieses Ablaufs. GitHub Actions werden dadurch nicht zum Merge-Gate; B2s eigener noch offener Konkurrenztestbefund wird durch das CI-GO nicht geschlossen.

TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: B1-R1-N1-NACHREVIEW.md
