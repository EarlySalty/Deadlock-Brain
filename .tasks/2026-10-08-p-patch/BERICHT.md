# P: Lieferblocker im vorhandenen Forumreader

ABWEICHUNG: Die ausschließlich freigegebenen P-Dateien plus CLI/Timer ergeben gegen aktuellen Main keinen baubaren Lieferkandidaten. `rust/crates/dbrain-sources/src/forum.rs` benötigt den bereits erhaltenen Export `first_post_html`. Diese Datei liegt außerhalb des ausdrücklich benannten P-Eigentums. Kein fremder Arbeitsbaum wurde geändert.

## Konkreter Beleg

- Erhaltener HEAD: af4736089cc5ce5d41ed442d445c49a30d5c6375, Start sauber.
- Am 8. Oktober 2026 frisch geholter origin/main: 400381e681a2e08283db46094d1bbbde037e2813. Die Prüfung erfolgte nach dem Fetch; origin/main wurde zwischenzeitlich weitergeschoben.
- `git diff origin/main HEAD -- rust/crates/dbrain-sources/src/forum.rs` zeigt einen fehlenden 11-zeiligen Export des vorhandenen privaten `parse_thread_html`, zusätzlich zwei fremde Kommentardeltas. Die Kommentare werden nicht übernommen.
- `git show origin/main:rust/crates/dbrain-sources/src/forum.rs` enthält keinen öffentlichen `first_post_html`. `api_sync.rs` benötigt diesen Export für den bestehenden Reader. Kein kombinierter Compilerlauf gegen Main wurde behauptet; die fehlende Symbolabhängigkeit ist am tatsächlichen Blob belegt.

## Berechtigter Fixkern im P-Arbeitsbaum

Originalbindung vor dem vorhandenen Reader: Root-Thread-ID, XenForo-Originalmetadaten und Gleichheit des Originaltextes mit dem sichtbaren ersten Beitrag. Folgeseiten, fremde Threads und fehlende Originalmetadaten werden nicht importiert. Gebundener Metal-Skin-Name wird vom Kosmetikveto ausgenommen, der eigentliche Änderungsteil bleibt geprüft. Kurze narrative und kompakte Sternchenänderungen erreichen die vorhandene Ereignisprojektion. Der bereits vorhandene Schutz gegen Feedauszüge bleibt erhalten.

Neue Wrapper und Negativmutationen in den Regressionen sind synthetisch. Die Metadatenform wurde gegen einen tatsächlichen öffentlichen XenForo-Root geprüft. Keine Community-Rohdaten gespeichert. Die Patchtests sind im erhaltenen Featurestand grün, aber nicht als Prüfung einer Mainvereinigung oder als Lieferfreigabe ausgegeben.

## Erforderliche Freigabe

FRAGE AN ORCHESTRATOR: Den vorhandenen Export in `rust/crates/dbrain-sources/src/forum.rs` als unmittelbare Readerabhängigkeit exklusiv P freigeben oder einen bereits gegen aktuellen Main gelieferten Readerexport benennen. Erforderlich ist der bestehende 11-zeilige Patchanteil, keine zweite Discoverypipeline und keine Parserneuentwicklung. Die Gegenprobe und Originalbindung verbleiben in P.

## Getrennte Nachweise

- Gebaut: Testbinary im erhaltenen Featurestand kompiliert, 30,25 Sekunden. Kein Build gegen Main.
- Getestet: 45 bestanden, 0 fehlgeschlagen, 1 ignorierter Postgres-Scratchtest, 73 herausgefiltert. Gesamtlauf Exit 0, Ereignisparser einschließlich Originalbindung, Metal Skin und kompakter Änderungen geprüft. Kein DB-/Live-Reimportbeweis. Ein eigener vorheriger Fixture-Serialisierungsfehler wurde korrigiert; kein Altfehler behauptet.
- Clippy: `env -u DEADLOCK_CENTRAL_DSN -u DATABASE_URL SQLX_OFFLINE=true cargo-slot +1.97.1 clippy --manifest-path /home/nathanael/.worktrees/brain-p-patch-20261008/rust/Cargo.toml --jobs 3 -p deadlock-brain --bin deadlock-brain --tests -- -D warnings`, Exit 0, 2 Minuten.
- Formatiert: eigener API-Modulformatlauf und rustfmt-Check des vorhandenen Patchparsers, beide Exit 0.
- Gate: nicht gestartet. Historische Urteile nicht wiederverwendet.
- Main/Merge/Deploy/Live/Cleanup: nicht ausgeführt. Archivbranch und fremde Worktrees bleiben erhalten.

## Reproduzierbarer Testlauf

```text
env -u DEADLOCK_CENTRAL_DSN -u DATABASE_URL SQLX_OFFLINE=true cargo-slot +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-p-patch-20261008/rust/Cargo.toml --jobs 3 -p deadlock-brain --bin deadlock-brain pg_patchnotes:: -- --test-threads=1
```

TESTNACHWEIS[TW-1]: 45 passed, 1 ignored | Baseline: n/a rot

Baseline n/a: kein vorbestehender Fehler behauptet, kein Baselinevergleich durchgeführt. Der ignorierte Bestandstest erfordert die eigene DB brain_fixer12_patch_lookup und DEADLOCK_BRAIN_SCRATCH_DSN. Er wurde nicht als ausgeführt ausgegeben.

Prüfbindung: API-Modul SHA256 `39a6114ee0218c9c48eb21a3a601761917cef61bfd57f2101318ad931b5168ca`. Code und Fachakte im Checkpoint c8db98a9 auf origin/feat/brain-patch-discovery gesichert. Die unveränderten Prüfprotokolle wurden ohne Überschreiben oder Löschen nach `/tmp/brain-p-patch-c8db98a9-tests.log` und `/tmp/brain-p-patch-c8db98a9-clippy.log` verschoben. Archivcheckpoint, kein Lieferbranch auf aktuellem Main.

Der Abschluss-Hook fordert einen pauschalen Merge des Archivbranches mit 34 gegenüber Main zusätzlichen Commits. Das widerspricht dem konkreten Auftrag, keine pauschale Altbranchübernahme und kein Archivcleanup durchzuführen. Der Branch bleibt deshalb geschützt erhalten. Kein Hook wurde abgeschaltet oder umgangen; der tatsächliche Scopeblocker wird zurückgegeben.

MERGEPROTOKOLL[MS-1]: 9 Git-Schritte einzeln | Anläufe: 0 | Gate: nicht gestartet, Readerabhängigkeit außerhalb P-Eigentum

Zählbereich: Startstatus, Start-HEAD, erster Fetch, Readerdelta, eigener Status, erneuter Fetch, exakt gepinnter Main-SHA, eigene Dateien staged und staged-Stat. Read-only-Aggregate in context-mode sind nicht als Mergeversuche gezählt.
