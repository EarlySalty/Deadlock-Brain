status: aktiv
Datum: 2026-10-03

# Noch nicht zur Bereinigung freigegeben

Stand der Inventur: 2026-10-03T14:39:43Z, Vergleichs-SHA `511a347b653beba13c2bf130f4bead7a7196cc2a`. Vollständige Namen, SHAs, Pfade und geprüfte Ancestry-Exitcodes stehen in `BACKUP-20261003T143943Z.txt`. Keine Löschung ausgeführt.

## Gemergte Branches und Worktrees

Die Inventur weist 30 lokale und 24 entfernte Branches einschließlich main als Vorfahren von `origin/main` aus. Das allein erlaubt keine Löschung: Sessionzuordnung, Laufzeitnutzung und ungesicherte Artefakte müssen zusätzlich geklärt sein. Die nachfolgende Auswahl bleibt erhalten, bis der jeweils offene Nachweis vorliegt.

| Gruppe | Konkrete Kandidaten | Offener Nachweis und Vorschlag |
| --- | --- | --- |
| Gemergte frühere Bauzweige | `audit/independent-full-system-20260930`, `codex/consumer-ci-completion-20260925`, `codex/fix-brain-db-pooling`, `codex/fix-c1-brain-serve`, `codex/fix-c11-storage-upgrade`, `codex/fix-c2-c3-retrieval`, `codex/fix-c4-contract-unification`, `codex/fix-c7-c8-runtime-tooling`, `codex/fix-c9-consumer-wiring`, `codex/fix-p2-generic-patch-20261001`, `codex/pr61-publication-independent-regressions-20261001`, `codex/wiki-completion-20260925`, `fix/pre-g5-match-contract-20260929`, `integration/brain-pg-isolation-20260926`, `integration/pre-g5-finalize-20260929`, `integration/pre-g5-review-20260926`, `migration/rust-integration` | Freiheit von laufenden Sessions und tatsächlichen Laufzeitpfaden, Inhalt ignorierter Artefakte. Gezielt klären, unmittelbar davor SHA, Bundle-Deckung und Sauberkeit erneut prüfen, erst danach einzeln bereinigen. |
| Gemergt, aber ungesicherte Arbeit positiv gefunden | `codex/brain-functional-closeout-20260930`, `codex/luna-native/brain-pr4-evidence-20261001`, `luna/finish-brain-rust-cutover-20261001` | Uncommittete oder untracked Dateien. Nicht löschen oder fremde Änderungen sichern, solange Eigentum und Aktivität unklar sind. |
| Entfernte gemergte Zweige ohne eigenen Worktree laut Inventur | `codex/brain-functional-closeout-20260930`, `codex/core-completion-20260925`, `codex/external-sources-completion-20260925`, `codex/fix-c5-wiki-runtime-integration`, `codex/replay-decoder-completion-20260925`, `feat/steam-ledger-brain-20260929`, `migration/rust-integration`; außerdem lokal und entfernt vorhandenes `fix/pre-g5-harness-20260929` ohne Worktree | Genaue Remote-SHAs sind gesichert. Noch aktive Sessions könnten diese Referenzen verwenden. Erst die eindeutige Zuordnung prüfen; lokale gleichnamige Zweige können abweichende, nicht gemergte SHAs haben und bleiben dann erhalten. |
| Detached-Prüfbäume | `/home/nathanael/.worktrees/brain-independent-cleanroom-20260930-continued`, `/home/nathanael/.worktrees/brain-pr61-review-20260930`, `/tmp/brain-pr61-review-9635592` | Aktivität und Artefakte. Die beiden PR61-Bäume enthalten untracked Dateien, darunter Rust-Tests. Kein `--force` und keine Löschung dieser Dateien. |

Diese Tabelle ist keine Löschliste. Der zweite native Sicht-Worker `wf_56dbf111-730` ist abgeschlossen. Seine präzisere Aufnahme vom 03.10. um 15:29 UTC ersetzt die pauschale Annahme einer vollständig unbrauchbaren Prozesssicht:

- 982 Prozesse, davon zwei Kernelthreads und 198 Zombies. 537 gesperrte cwd-/exe-Zugriffe betreffen 333 Containerprozesse und 204 Systemdienste/Scopes. Kein gesperrter Claude-, Codex-, Cargo- oder Rustc-Prozess; cmdline war lesbar. 28 laufende Container hatten keine Brain-Worktree-Mounts. Brain-PostgreSQL und Agent-Connectoren begrenzen die Sicht mit `ProtectHome=yes`.
- Offen bleibt die konkrete Pfadzuordnung der laufenden T3-Threads `a02207b7-fd5d-485e-882f-941c1f985f94` (Wiki), `64cab234-aaf8-48e0-ab7c-440c2468a199` (Pfad-/Mergeprüfung) und `3bca9b62-5a93-4ef9-9096-dc49ae9f9e66` (Forumsimport) mit cwd Documents. Auch Codex PID 2035117 und Appserver PID 2035374 mit cwd `/home/nathanael` waren keinem konkreten bearbeiteten Baum sicher zuzuordnen. Dies sind historische Aufnahme-PIDs, keine aktuelle Aktivitätsbehauptung.
- Von 19 gemergten angehängten, nicht ausdrücklich geschützten Bäumen hatten drei ungesicherte Änderungen und 16 ignorierte Artefakte. Zwei detached Bäume enthielten ungesicherte Dateien, ein weiterer ignorierte Artefakte. Kein Baum ist dadurch zur Entfernung freigegeben.
- Sieben gemergte Remote-Refs ohne eigenen Baum waren bundlegedeckt und mit Ancestry-Exit 0 geprüft. Ihre Aktivitätsfreiheit blieb offen. Eine Threadansicht `ready` konnte von Runtime-Metadaten `running` abweichen und genügt nicht als Freigabe.

Urteil: weiter null freigegebene Löschkandidaten. Nachweise im zweiten Workerresultat erhalten; keine fremden Sessions kontaktiert oder gestoppt. Vor einer späteren Entfernung aktuelle Aktivitätsfreiheit, Artefaktsicherung, Referenz-SHA und Ancestry erneut feststellen.

## Strikt geschützte Gruppen

- Hauptcheckout `/home/nathanael/repos/Deadlock-Brain`, lokale und entfernte main sowie möglicher Laufzeitbaum `/home/nathanael/.worktrees/brain-live-main`.
- Wiki-Auftrag einschließlich aller `feat/brain-wiki-spielwissen-*` und `worktree-wiki-spielwissen-status-s`, auch wenn einzelne Heads bereits in main stehen.
- Pakete P, S, Q, R, K und Z samt eigenen Consumer-Zweigen und Worktrees.
- Locked-Prüfbäume: `brain-g5-readonly-snapshot-20260930`, `brain-g5-replay-deferred-20260930`, `brain-pre-g5-core-review-20260929`, `brain-pre-g5-harness-20260929`, `brain-technical-closeout-20260929`.
- Alle nicht gemergten Branches, laufenden oder unklar zugeordneten Sessions und unbekannten Laufzeitpfade.

## Alte Drafts

#3, #4, #5, #6 und #9 enthalten nach der Bestandsprüfung verwertbare Restinhalte. Details und Vorschlag zur Wiederverwendung stehen in `PR-BEWERTUNG.md`. Sie bleiben offen, ihre Branches erhalten. #46 bleibt bis zur Rückmeldung von R unangetastet.

## Grenzen der Sicherung

Das verifizierte Bundle sichert Commit-Historie und Referenz-SHAs, keine uncommitteten, untracked oder ignorierten Dateien. 27 Arbeitsbäume waren unsauber, 62 enthielten ignorierte Artefakte. Daten-, Snapshot-, Bericht- und Binary-Artefakte werden nicht pauschal als Cache behandelt. Die 547 verweigerten Prozesszugriffe aus der ersten Aufnahme sind ein zu klärender Sichtbefund, kein Grund für erhöhte Rechte oder eine Umgehung.
