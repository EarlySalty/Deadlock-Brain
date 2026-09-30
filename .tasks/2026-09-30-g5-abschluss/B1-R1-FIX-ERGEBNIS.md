status: aktiv, Quellabgabe ohne Laufzeitfreigabe
Datum: 2026-09-30

# B1-R1: vorhandene Aufrufer geben den Targetcache weiter

Ausgangshead nach der getrennt gepushten B2-Abgabe: `e2cb15486f9816ac541b2025d53833039926bd6c`. Der B1-R1-Quellcommit auf `fix/g5-replay-deferred-20260930` ist `a427be3099d9bb4d93dad8ce43cfe4c1820d4596`. Dieser Bericht erhält einen eigenen Commit. B2-Quellcommit `c5d2b1f4f18eb8fd360641b3e66fa7453cafb4d2` und B1-Quellhead `35673e7959290431ca60641195ee1378c09194eb` bleiben unverändert. Der Fix beseitigt den in `B1-HARNESS-REVIEW.md` beschriebenen Usage-Exit vor den eigentlichen Prüfungen; eine ausgeführte Prüfung ist damit noch nicht belegt.

## Vollständige Aufruferkette im beauftragten Scope

- `scripts/run_isolated_load.sh:4` reicht alle Argumente direkt an `test_brain_serve.sh` weiter. Ohne expliziten Cache beendet weiterhin der Runner den Aufruf mit Usage/Exit 2.
- `scripts/check_brain_core.sh:8-21,40-63` verlangt `<modus> <absolutes-bestehendes-target-verzeichnis>` vor jeder Log- oder Cargo-Aktion. Die beiden Core-PG-Aufrufe erhalten diesen Pfad. Eigene Cargo-Aufrufe verwenden Rust 1.97.1, `--locked --offline --jobs 1 --target-dir` und denselben Cache; `fmt` erhält die Toolchain und `CARGO_TARGET_DIR`, benötigt aber keine Buildflags. `run_check` behält den bisherigen Fehlerstatus und die Protokollierung. Fehlendes, relatives oder nicht existentes Targetverzeichnis erzeugt keinen Defaultcache.
- `.github/workflows/brain-serve-c1.yml:69-76` prüft das durch vorherige Cargo-Schritte erzeugte `$GITHUB_WORKSPACE/rust/target` und gibt es dem Serve-Runner. `.github/workflows/rust-core-verification.yml:61-88` tut das für Core-PG, Upgrade und Serve; Upgrade erhält zuerst den Cargo-Pfad und danach den Cache. `.github/workflows/wiki-runtime-pilot.yml:24-27,45-51` verwendet das bereits konfigurierte absolute `CARGO_TARGET_DIR` aus dem vorangehenden Build-/Cachepfad. Die drei Workflows behalten `set -euo pipefail` bei den geänderten Pipeline-Aufrufen; GitHub Actions sind dadurch kein Merge-Gate.
- `rust/crates/brain-storage/tests/fixtures/storage_v1/README.md:19-25` zeigt beim Upgrade jetzt Cargo-Executable und bereits vorhandenes absolutes Targetverzeichnis in dieser Reihenfolge.

Es wurde weder ein weiterer Hostcache angelegt noch ein historischer Runner reaktiviert. Die fünf B1-Runner und der Rust-Produktcode blieben in diesem Anschlusscommit unangetastet.

## Statische Gegenbeweise und spätere Laufklassen

`bash -n scripts/check_brain_core.sh scripts/run_isolated_load.sh`, YAML-Parse der drei Workflowdateien, gezielte Sichtung aller acht genannten Runner-Aufrufstellen sowie `git diff --check` und `git diff --cached --check` waren erfolgreich. Der statische Nachweis prüft die Argumentweitergabe, nicht die tatsächliche Existenz eines Cacheordners im späteren CI-Job. Die CI-Jobs prüfen diese Existenz vor Clusterstart und verweigern einen stillen Fallback.

Nach gesonderter Compiler-/Scratch-/Prozesszuteilung dürfen jeweils getrennt die folgenden Aufrufe geprüft werden. Der benannte Cache stammt aus B1 und muss zum Ausführungszeitpunkt weiterhin vorhanden sein. Keiner dieser Befehle wurde für B1-R1 gestartet.

```sh
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/check_brain_core.sh core /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/check_brain_core.sh postgres /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/run_isolated_load.sh /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

`core` benötigt den Compiler-/Testslot, `postgres` zusätzlich einen isolierten DB-Slot. `run_isolated_load.sh` ruft den vollständigen Serve-E2E einschließlich 1800 Anfragen auf und braucht ausdrücklich einen Prozess- und Lastslot. `check_brain_core.sh release` und `all` können Release-Builds auslösen und gehören nur in einen dafür zugelassenen eigenen Worktree mit geprüftem Commitstand. Vor GitHub-Workflowläufen muss der jeweilige Job den Cache wie beschrieben selbst erzeugt haben; sie ersetzen weder unabhängiges Review noch die lokale Merge-Schleuse.

Keine Wrapper, Cargo, Tests, Datenbanken, Produktprozesse, Dienste oder Deploys ausgeführt. Compiler- und DB-Schreibhalt für Brain gelten weiter. Das separate B2-Dokument `B2-IMPORT-ERGEBNIS.md` benennt die noch fehlenden echten Snapshot-, Policy-, Widerrufs- und Tombstonebelege; diese Anschlusskorrektur gewährt keine Cutoverfreigabe.

BESTAND[BS-1]: ja | Fundort: scripts/check_brain_core.sh:52 | Anknüpfung: bestehende Wrapper und CI-Aufrufer statt neuem Harness
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 1 belegt | Senke: B1-R1-FIX-ERGEBNIS.md
ORCHESTRIERUNG[OR-1]: Stufe mittel | Schritt bau | Artefakt: .tasks/2026-09-30-g5-abschluss/B1-R1-FIX-ERGEBNIS.md
