status: aktiv, Testquellenabgabe ohne Laufzeitbeweis
Datum: 2026-09-30

# B2-R1: synchronisierte Quellsperren-Gegenprobe

Ausgang `392267b133a9a5a8a91602247a441ff7bf609c4b`, eigener gepushter Testquellhead `89491987a52053c94f98a7d3430d7139570caa95` auf `fix/g5-replay-deferred-20260930`. Der R1-Produktpfad, R2, R3 und der getrennte B1-CI-Fix blieben unverändert. Das Nachreview auf `eceb14c` akzeptierte den Produktpfad statisch, verlangte aber einen echten Sperrüberlappungsnachweis anstelle der seriellen Negativfälle. Die frühere Beschreibung dieser seriellen Fälle wurde im B2-Fixbericht berichtigt.

## Geschriebene Synchronisation

`rust/crates/brain-legacy-import/src/bin/brain-legacy-import.rs:679-932` ergänzt ausschließlich die vorhandene ignorierte Scratchtestquelle. Der Test legt innerhalb von `brain_cutover_test` je Szenario zwei getrennte synthetische Quellen an, befüllt sie über den vorhandenen `PgStore::commit` und bereitet für beide Quellen Batches mit **leerer Recordliste** und neuen Checkpointgenerationen vor. Die Releasepins kommen aus beiden Checkpoints; der erwartete Headzustand enthält beide Quellen. Die alten seriellen Tombstone-/Scopefälle bleiben unverändert und laufen vorher.

Eine Testtransaktion hält den vorhandenen Release-Advisory-Key. Der geprüfte Zwei-Quellen-Publish nimmt seine Quellsperren, schreibt die leeren Batches, vergleicht Heads und Checkpoints und wartet erst danach in `publish_release_tx` auf diesen Release-Key. `advisory_waiter` fragt `pg_stat_activity` und `pg_blocking_pids` nach dem konkreten PostgreSQL-Backend und dem Advisory-Warteereignis. Erst nach dem belegten Warten des Publishers starten ein echter `PgStore::commit` mit vorbereitetem Patch-Tombstone und ein direkter `PgStore::apply` mit Entity-Scopewiderruf auf den beiden gesperrten Quellen. Für beide Writer muss PostgreSQL den Publisher als blockierenden Backend-PID **am Advisory-Lock** melden. Die 15-Sekunden-Grenze beendet fehlende Signale mit Fehler; sie ist keine Schlafpause und kein Ersatz für das Locksignal (`bin/brain-legacy-import.rs:679-698,811-879`). Vor Freigabe des Release-Keys dürfen die beiden Writer nicht beendet sein und die Releasezahl bleibt gleich.

**Commitfall:** Nach Freigabe des Test-Keys muss der geprüfte Publish mit zwei Receipts committen. Der bereits wartende Tombstonebatch erhält danach einen Checkpoint-CAS-Fehler, wird aus dem neuen Checkpoint frisch vorbereitet, mit neuer Lease committet und bleibt als Tombstone-Head erhalten. Der direkte Scope-Apply endet nach dem Publish erfolgreich. Die neue Releasezeile bewahrt die historischen drei Pins, während die aktuellen Heads den Tombstone und den privaten Scope zeigen (`bin/brain-legacy-import.rs:881-931`).

**Rollbackfall:** Vor dem Versuch existiert eine unveränderliche Releasezeile mit derselben ID und abweichender Epoch. Der geprüfte Publish gelangt nach erfolgreichem Head-/Checkpointvergleich erneut zum gesperrten Release-Key, scheitert nach dessen Freigabe am unveränderlichen Releasekonflikt und rollt seine leeren Batches zurück. Erst danach darf der bereits wartende Tombstonebatch mit der weiter gültigen ursprünglichen Lease erfolgreich committen; der direkte Scope-Apply bleibt ebenfalls erhalten. Die alte Releaseepoch und der Entity-Checkpoint bleiben unverändert. Das zeigt die Sperrgrenze auch bis zum Rollback (`bin/brain-legacy-import.rs:779-785,879-928`).

Ohne die effektive Quellsperre des Publishers erreicht ein Writer nicht das behauptete Advisory-Warten auf dessen Backend. Ohne `lock_source` im Batchwriter wartet dessen Aufruf allenfalls an der Job-/Transaktionszeile, nicht am abgefragten Advisory-Lock; `advisory_waiter` für `cutover-sync-batch` scheitert. Ohne `lock_source` in `apply_connection` kann der direkte Apply während des blockierten Publishes fertig werden; `advisory_waiter` für `cutover-sync-apply` scheitert. Die Assertions isolieren damit die beiden Writerpfade bei einem bereits erreichten Vergleichspunkt. Sie unterscheiden nicht die vorgelagerte Sperrnahme in `pg_release` von der zusätzlichen Sperrnahme in `commit_batch_tx`, wenn beide wirksam bleiben. Es wurde keine Mutation ausgeführt.

## Anschluss und Ressourcen

Kein neuer Testselektor oder Runneraufruf: `scripts/test_brain_serve.sh:75-79` startet weiter genau `tests::same_database_archive_to_core_requires_bound_private_snapshot -- --ignored --exact` innerhalb des bestehenden seriellen Serve-Harnesses. Der neue Abschnitt läuft am Ende dieses Fixtures, nach den bisherigen seriellen Negativfällen. `brain_schema_test` bleibt für Schema99 unangetastet. Je Szenario gibt es zwei zusätzliche Testquellen mit Checkpoints und Jobs, drei zusätzliche Heads, einen gehaltenen Release-Key, drei bis zu einer Verbindung begrenzte Testpools für Publisher und Writer sowie Lockzustandsabfragen über den vorhandenen Pool. Beide Szenarien laufen nacheinander und schließen ihre Zusatzpools. Kein neues Schema, keine neue Datenbank, Rolle, Dependency oder Konfiguration. Der volle Runner enthält weiterhin PostgreSQL, Serve-Prozesse und **1.800 Lastanfragen**; er ist kein Smoke-Test. Die synthetische Rolle `brain_core_test` ersetzt keinen produktiven Rollen- oder Secret-Exec-Nachweis.

Nur die Testquelle wurde mit Rust 1.97.1 `rustfmt --check` statisch geprüft, außerdem `bash -n scripts/test_brain_serve.sh`, `git diff --check` und `git diff --cached --check`, jeweils Exit 0. Kein Cargo, Compiler, Clippy, Test, PostgreSQL, Import, Fixture, Dienst oder Deploy lief für diese Nacharbeit. Nach gesonderter Zuteilung ist zuerst der engste Compilercheck mit Testtargets nötig; erst ein gesonderter Scratch-, Prozess- und Lastslot darf den vollständigen Runner ausführen:

```sh
/home/nathanael/.cargo/bin/cargo +1.97.1 check --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-legacy-import -p brain-storage --all-targets --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/test_brain_serve.sh /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

Der nächste Schritt ist ausschließlich ein enges unabhängiges Nachreview der Testquellen auf dem fixierten Quellhead. Weder die geschriebene Quelle noch der Bericht sind eine Laufzeit- oder Cutoverfreigabe.

BESTAND[BS-1]: ja | Fundort: rust/crates/brain-legacy-import/src/bin/brain-legacy-import.rs:679 | Anknüpfung: vorhandene Scratchfixture und bestehende Store-Sperren
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: B2-R1-SYNCHRONISATION-ERGEBNIS.md
ORCHESTRIERUNG[OR-1]: Stufe groß | Schritt bau | Artefakt: .tasks/2026-09-30-g5-abschluss/B2-R1-SYNCHRONISATION-ERGEBNIS.md
