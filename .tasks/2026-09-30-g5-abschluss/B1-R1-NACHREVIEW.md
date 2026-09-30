# B1-R1: unabhängiges Nachreview der Aufrufer

Datum: 2026-09-30. fertig: J. Fix: J.

**Pflichtargumentbefund B1-R1 geschlossen; vollständige Aufruferabnahme weiter BLOCK wegen eines konkreten Cargo-Pfadrestbefunds an drei CI-Aufrufen.** Die lokalen Wrapper einschließlich `all` erfüllen den geprüften Argument-/Cache-/Compilervertrag. Keine Wrapper-, Compiler-, Test-, DB-, Rollenfixture-, Produktprozess-, Dienst- oder Deployausführung.

WIRKUNGSPRUEFUNG[WP-1]: 1 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft

## Bindung und Prüfumfang

- Eingefrorenes Delta `e2cb15486f9816ac541b2025d53833039926bd6c..a427be3099d9bb4d93dad8ce43cfe4c1820d4596` aus `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930`. Sechs Dateien, 45 Einfügungen, 23 Löschungen: zwei Wrapper, drei Workflows und das Reproduktionsbeispiel. Kein bewegliches B2-Fix-WIP gelesen.
- Autorbericht `4ee56de1ef465b4b29633a543c3e2930b3003303:.tasks/2026-09-30-g5-abschluss/B1-R1-FIX-ERGEBNIS.md` geprüft. Dieser Folgecommit ergänzt eine Berichtsdatei. Die fünf ursprünglichen B1-Runner sind zwischen `0a6e095` und `a427be3` unverändert.
- Bericht im bestehenden Reviewworktree `/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929`, Branch `review/pre-g5-core-abnahme-20260929`, sauberer Berichtsstand nach `854825b`. Kein Produktfix durch den Reviewer.
- Graphify zuerst; mangels passender neuer Graphstellen anschließend eingefrorene Dateien und die bekannten Aufruferketten gelesen. Keine neue Vollinventur. `git diff --check e2cb154 a427be3` erfolgreich. Zwei vollständige Wrapperblobs und die fünf geänderten Shell-`run`-Blöcke der Workflows per stdin mit `/bin/bash -n` geprüft: siebenmal Exit 0. Das war keine YAML-/GitHub-Ausführung und keine Argumentprobe am laufenden Skript.

Syntaxbindung der Wrapper:

| Datei | SHA256 bei `a427be3` |
| --- | --- |
| `scripts/check_brain_core.sh` | `94e9a1cc3dbd2ce84259fc147f90be34d4f9fc3dd1c1f676322d6572164700c9` |
| `scripts/run_isolated_load.sh` | `ce3574b18fc916e613f6481ffb3e249aae10e9bb5fc65a8f595708daaed6d660` |

## R1-N1, P2: Drei isolierte CI-Aufrufe verlieren weiterhin den Cargo-Einstieg

**Anker bei `a427be3`:** `.github/workflows/brain-serve-c1.yml:75-76`, `.github/workflows/rust-core-verification.yml:69-70,87-88`.

**Konkreter Ablauf:** Der Workflow installiert Rust unter dem normalen Benutzerkonto und erstellt einen leeren Test-HOME. Der geänderte Aufruf setzt per `env -i` anschließend `HOME` auf diesen Testordner, erhält `CARGO_HOME`/`RUSTUP_HOME` und `PATH`, übergibt aber kein `BRAIN_TEST_CARGO`. Sowohl `scripts/test_brain_serve.sh:15` als auch `scripts/test_brain_core_postgres.sh:15` wählen dann `CARGO="${BRAIN_TEST_CARGO:-$HOME/.cargo/bin/cargo}"`.

Damit wird beim C1-Aufruf `$GITHUB_WORKSPACE/.core-test-logs/c1/ci-home/.cargo/bin/cargo` ausgeführt; bei den beiden Matrixaufrufen ist es `$PWD/.consumer-ci-reports/test-home/.cargo/bin/cargo`. Die Workflows installieren dort kein Cargo. Ein richtiger `PATH` und ein vorhandenes `CARGO_HOME` ändern den ausdrücklich zusammengesetzten Programmpfad nicht. Nach erfolgreicher Argumentprüfung und Scratch-PG-Anlage scheitert deshalb der Compileraufruf im beschriebenen frischen CI-Aufbau mit fehlender Datei statt den Test auszuführen. `set -e` und Pipeline-`pipefail` erhalten den Fehlerstatus; es ist kein stiller grüner Test.

**Historische Einordnung:** Der HOME-/Cargo-Konflikt ist bereits im eingefrorenen Basisstand `e2cb154` zu sehen und wird nicht als neue Regression dieses Deltas ausgegeben. Der neue Cacheparameter beseitigt den früheren vorzeitigen Usage-Abbruch, aber die Abgabeaussage einer funktionierenden vollständigen Aufruferkette ist damit noch nicht erfüllt. Dies ist ein direkt an den beauftragten Aufrufstellen belegter Anschlussrest, keine zusätzliche Prüfung unbeteiligter CI-Jobs.

**Zwillingssuche und unauffällige Gegenstellen:** Drei betroffene Callsites oben. Der Upgrade-Aufruf in `.github/workflows/rust-core-verification.yml:77-78` übergibt bereits `"$(command -v cargo)"` als erstes Positionsargument und ist für diesen Konflikt korrekt. `scripts/check_brain_core.sh:32-33` gibt seinen vorhandenen Cargo-Einstieg als `BRAIN_TEST_CARGO` an den Kindrunner weiter. Der Wiki-Workflow ersetzt den normalen HOME vor seinem Runner nicht und ist von diesem konkreten Fehler nicht betroffen.

**Enge Korrektur:** In den drei vorhandenen `env -i`-Aufrufen den vor der Isolation aufgelösten Cargo-Einstieg über das bereits unterstützte interne Testfeld weitergeben, beispielsweise `BRAIN_TEST_CARGO="$(command -v cargo)"`. Keine zweite Installation im Test-HOME, kein neuer Cache, kein Abschalten der HOME-Isolation und keine neue produktive ENV-Konfiguration. Danach diese drei Aufrufketten statisch nachprüfen. Kein CI-Lauf oder pauschaler neuer Build ist zur Prüfung der Quellkorrektur erforderlich.

## Geschlossene und unauffällige Prüfpunkte

1. **Lastwrapper:** `scripts/run_isolated_load.sh:4` übergibt `"$@"` unverändert per `exec`. Das vorhandene Serve-Skript validiert Anzahl und absolutes existierendes Targetverzeichnis vor Clusteranlage. Fehlende, zusätzliche, relative und nicht existente Argumente führen statisch zum bestehenden Exit 2. `exec` erhält den Kindstatus; kein Defaultcache.
2. **Core-Wrapper einschließlich `all`:** `scripts/check_brain_core.sh:8-21` verlangt zwei Argumente und prüft Cache und Modus vor Loganlage. `postgres` und `all` reichen dasselbe gequotete Target weiter (`:52,59`). Die sechs eigenen Compileraufrufstellen in `bootstrap`, `core`, `release` und `all` tragen `+1.97.1`, `--locked --offline --jobs 1 --target-dir` (`:42,45,49,56-58`). `fmt` ist kein Compileraufruf und erhält Toolchain plus den aus dem Pflichtargument intern gesetzten Targetpfad. Kein neues Target-`mkdir` oder automatisches `all` bei fehlenden Argumenten.
3. **Status und Reihenfolge:** `run_check` speichert den echten Unterprozessstatus vor Loganzeige und setzt `FAILED=1`; der abschließende Exit verwendet diesen Sammelstatus (`:35-38,63`). `all` behält die serielle Folge fmt, Clippy, Workspace-Test, Releasebuild und Core-PG bei und bricht nach einem Fehler nicht automatisch ab. Das ist unverändertes Verhalten, keine versteckte Parallelisierung. Die Loganzeige per `tail` ersetzt den zuvor gesicherten Fehlerstatus nicht.
4. **CI-Cache:** C1 und Core-Matrix prüfen das absolute `$GITHUB_WORKSPACE/rust/target` vor dem Harness und reichen es weiter. Vorherige vorhandene Clippy-/Test-/Buildschritte verwenden den entsprechenden Workspace. Wiki verwendet den bestehenden Jobwert `CARGO_TARGET_DIR: ${{ github.workspace }}/rust/target` und prüft ihn vor dem Aufruf (`wiki-runtime-pilot.yml:24-27,41-51`). Der spätere tatsächliche Cachebestand wird damit im Job kontrolliert, nicht hier als vorhanden behauptet. Kein hostlokaler Worktreepfad wird in CI eingebaut. Die geänderten Pipelines behalten `set -euo pipefail`; der jeweilige Kindrunner bindet seine Compilerflags. Unbeteiligte frühere CI-Compiler-/Fetchschritte sind nicht in einen neuen hostweiten Ein-Job-Vertrag umgedeutet.
5. **Dokumentation und Scope:** `rust/crates/brain-storage/tests/fixtures/storage_v1/README.md:22-25` zeigt Cargo-Executable und vorhandenen absoluten Cache in richtiger Reihenfolge. Die acht aktiven Runneraufrufe aus dem Ausgangsbefund enthalten jetzt die Weitergabe. Deaktivierte historische Runner, Testfilter, Scratchgrenzen und Produktquellen sind in diesem Sechsdateidelta unverändert. Die B2-Befunde werden dadurch weder geschlossen noch neu geprüft.

## Spätere Direktaufrufe und Grenzen

Nicht ausgeführt und nicht zugeteilt. Voraussetzung für einen späteren Lauf ist ein sauberer konkret geprüfter Quellstand, nicht der derzeit bewegliche B2-Arbeitsbaum, sowie der vorhandene Targetcache, Cargo-Home `/home/nathanael/.cargo` und installierte Toolchain 1.97.1.

```bash
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/check_brain_core.sh core /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/check_brain_core.sh postgres /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/run_isolated_load.sh /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

Die Zeilen sind getrennte mögliche Aufträge, kein auszuführender Sammelblock. `core` startet einen Compiler-/Rust-Testlauf; `postgres` zusätzlich den privaten Core-PG mit Unix-Socket Port 55439, `max_connections=12`, `shared_buffers=16MB`. Der Lastwrapper führt den vollständigen bestehenden Serve-Harness mit PG-, Importer-, Serve- und lokalen Providerfixtures sowie **600 Lastanfragen bei jeweils 8/16/32 Workern** aus. Kein leichter Smoke. Scratchstop und Fehlerbelegaufbewahrung bleiben beim jeweiligen Kindrunner.

`check_brain_core.sh all <target>` ist statisch geprüft, aber keine passende Abkürzung für einen begrenzten Testslot: Es führt Workspace-Clippy, Workspace-Tests, einen Workspace-Releasebuild und Core-PG aus. `release <target>` startet ebenfalls einen Releasebuild. Dafür gelten eigene revisionsgebundene Worktree-/Ressourcengrenzen. Ein Cargojob begrenzt nicht sämtliche Test-/Linkerthreads. Der bestehende Brain-Ressourcenhalt bleibt unverändert; keine Aussage zum aktuellen Twitch-/Ops-Livezustand erhoben.

## Freigabepunkt

B1-R1-Pflichtargumentweitergabe ist statisch abgenommen. Der verbleibende Anschlussfix ist präzise **R1-N1 an drei vorhandenen Workflow-Aufrufen**. Lokale direkte Wrapper sind davon nicht betroffen. GitHub Actions bleiben kein Merge-Gate; der Befund beschreibt fehlerhafte Aufrufquellen, nicht eine geforderte grüne Actions-Freigabe. Nächste Aktion: dem bestehenden Autor diese drei Cargo-Weitergaben zur engen Korrektur zuweisen. Kein neuer Thread, keine neue Genehmigungsschleife und kein automatischer Folgelauf.

TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: B1-R1-NACHREVIEW.md
