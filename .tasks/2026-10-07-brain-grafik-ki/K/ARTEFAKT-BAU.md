# K: Vergleichsartefakte

## Gebauter Teilstand

Nativer Implementierer ab219fb00896aa7ba abgeschlossen. K hat Storage, Maintenance, Site und SQL selbst gelesen. Erster Checkpoint b310e223 mit13 Produktdateien,1558 eingefügten/vier entfernten Zeilen war regulär BLOCK wegen instabiler JSONB-Fingerprints. Frischer Fixer a6b4e405e6be3d53d abgeschlossen, fünf-Dateien-Diff von K gelesen und gezielt als3c6f220b committed. Gemeinsamer regulärer Gate7cbb9fe6..3c6f220b, Task bxxpvybee, Exit0: `[gpt-6.1-sol] ALLOW: No confirmed merge-blocking defect in the supplied diff.` Wortlaut gelesen, Featurepush bestätigt. Ein zusammenhängender Pfad, kein G-Rechenkern oder zweite Darstellung. Der ausdrücklich vorrangige zentrale Provideranschluss wurde zuerst gestartet und bleibt ohne eigenen Coregate noch WIP.

`CompareArtifact` bindet Rechnung, Mechanikversion, Releasefingerprint, vollständige Dokumentpins und HTML/SVG-Bytes an eine serverseitige Hash-ID. `prepare_compare_artifact` verwendet den bestätigten H-Renderer. `render_stored_compare` rekonstruiert dieselben Bytes und prüft Quellen- und Rechnungsbindung. Veröffentlichung benötigt den vertrauenswürdigen `CompareCalculationVerifier`; `UnconfirmedCompareCalculation` verweigert sie.

Postgres speichert Kandidat und Veröffentlichungsquittung. Veröffentlichung prüft eingefrorene Revisionen sowie aktuelle kanonische Köpfe erneut, mit Kopfzeilensperre in der Transaktion. Abruf über die begrenzte SQL-Funktion prüft Quittung, Hash, Release, öffentliche Sichtbarkeit und aktuelle Rechte. Die Site-Rolle erhält keine allgemeinen Quelldatenrechte. HTML und SVG werden über `/compare/<id>` und `/compare/<id>/chart.svg` ausgeliefert. GET und HEAD prüfen dieselben Rechte; `no-store` verhindert eine Freigabe über Cacheantworten.

Neue eigene Migration `scripts/migrations/2026-10-07-brain-compare-artifacts-v1.sql`. Vor Sicherung origin frisch geholt, Main unverändert `f6f5cef65f1f946113f0b8216c6475f6d38ec928`. Keine angewandte Migration geändert und diese Migration nicht produktiv ausgeführt.

## Nachgelesene Prüfbeweise

Finale Kette des Implementierers mit `set -e`, absolutem Cargo `+1.97.1`, `--locked --offline --jobs 3`, `SQLX_OFFLINE=true` und vorhandenem zentralen Buildcache. Buildslot 3 gehalten. Logmarker von K unabhängig nachgelesen:

- `/tmp/brain-k-artifact-acceptance.log`: Maintenance zwei und Storage vier Fälle, jeweils 0 failed/ignored/filtered. Storage-I/O mit eigener echter PostgreSQL-Instanz, synthetische Rechnung und Prüfadapter ausdrücklich kein echter G-Beweis.
- `/tmp/brain-k-artifact-site-acceptance.log`: vier Fälle, 0 failed/ignored/filtered. Echte isolierte HTTP-/Postgres-Prüfung mit begrenzter Site-Rolle, einschließlich bestehender Kommentare.
- `/tmp/brain-k-artifact-render-regression.log`: 15 bestehende H-Fälle, 0 failed/ignored/filtered.
- `/tmp/brain-k-artifact-clippy-final.log` und `/tmp/brain-k-artifact-site-clippy-final.log`: beide `Finished`, keine Compilerfehler, striktes scoped Clippy mit `--no-deps` und `-D warnings`. Kein pauschales reposweites Clippygrün.
- `/tmp/brain-k-artifact-format-final.log` leer. K wiederholte `cargo +1.97.1 fmt --manifest-path /home/nathanael/.worktrees/brain-k-ki-20261007/rust/Cargo.toml -p brain-storage -p brain-maintenance -p deadlock-brain --check`, Exit 0.

Testauswahl: `-p brain-storage -p brain-maintenance --test compare_artifact`, Site `-p deadlock-brain --bin deadlock-brain-site`, Renderer `-p brain-maintenance --test hero_compare_render`; jeweils `-- --include-ignored --test-threads=1`. 25 bestandene scoped Fälle, keine vollständige Reposuite.

Zusätzlicher breiter Lauf `/tmp/brain-k-artifact-tests-final.log` zunächst56 passed, vier failed,0 ignored an fehlender fixer Postgres-Fixture. Frischer eigener Prüffixer a92067f4cff648233 hat die vorhandene isolierte Testmechanik vollständig hergestellt und denselben bestehenden Lauf wiederholt:106 passed/0 failed/0 ignored/0 filtered, Test und Cleanup Exit0, keine Produktänderung. K hat Marker selbst nachgelesen. Ein eigener59/1-Zwischenlauf brauchte zusätzlich die vorhandene Profil-Testfixture als read-only Bind. Keine unveränderte Vorherbaseline oder Altfehlerbehauptung. Fachbericht `K/ARTEFAKT-BREITPRUEFUNG.md`. Dieser breite ältere Lauf ist kein Beweis für spätere Core-/R1änderungen. Der JSONB-Blocker wurde separat in der frischen Fixrunde mit Originalbytebindung und echten PG-Zahlenroundtrips behoben.

TESTNACHWEIS[TW-1]: 25 passed, 0 ignored | Baseline: nicht erhoben rot

## Tatsächliche Restintegration

K hat seine Artefakthülle gebaut und wartet nicht auf eine G-Artefaktquittung. Der reale G-Draft liefert Vergleich, Szenario, Version, Werte und Quellen-/Regelbezüge, aber die tatsächliche kanonische Dokumentpinzuordnung und ein bestätigender `CompareCalculationVerifier` sind noch nicht angeschlossen. `ModelSource` allein ist kein vollständiger Dokumentpin. Keine G-WIP-Datei übernommen und keine synthetische Fixture als echte Vergleichsrechnung veröffentlicht.

G meldet inzwischen seinen gemeinsamen Provider-/Kernelumfang `a6568629..dbce14ae` regulär ALLOW. Nach frischem Fetch steht `origin/feat/brain-v2-g-20261007` tatsächlich auf `2b67796fb80ae3440a0c9e76671dfc8169032844`; die Ancestry von `dbce14aedadd94881a3cb21151d9840994094cd9` wurde mit Exit 0 geprüft. Damit ist dieser Provider-/Kernelstand entgegen dem älteren Dokumentstand auf origin gesichert. Dies bestätigt noch keinen vollständigen Rechenkern-, G-V-, Spiegel-/F- oder K-Liveanschluss. Vertrag und Eigentum für K stehen in `G/ANTWORTPORT-VERTRAG.md`: vollständige Delegation des bestehenden Serve-Enums und bestehender Kernel, kein Ersatzweg.

## Frische Fixrunde und nachgelesener Vertrag

3c6f220b speichert die vollständige vom Rustvertrag erzeugte JSON-Zeichenfolge als body_text. SQL-CHECK bindet sie an body_json=body_text::jsonb, ID und Wiedergabe verwenden Originalbytes. Kein Floatverbot oder Rundungsverlust. Echte PG-Prüfung belegt Normalisierung sowie verlustfreie negative Null,1e18,1e-100, präzise Zahlen und Integergrenzen. Vollständige Releaseprüfung bei Veröffentlichung mit FOR-SHARE-Lock, zusätzlich PG-JSONB-Releasefingerprint in der Quittung. SQL-Ausgabe an brain_site enthält nur CompareReleaseBinding statt unbeteiligter privater Dokumentliste; allgemeine Corpus-/Artefakttabellen bleiben unzugänglich.

Finale Logs: /tmp/brain-k-artifact-r1-tests-final.log (sechs Artefaktfälle), /tmp/brain-k-artifact-r1-site-tests-final.log (vier Sitefälle), /tmp/brain-k-artifact-r1-renderer-tests.log (15 Rendererfälle), jeweils0 failed/ignored/filtered. Compiler über echte Testbuilds, striktes scoped Clippy und Format0. K hat Marker selbst gelesen. Clippylogs /tmp/brain-k-artifact-r1-clippy.log und /tmp/brain-k-artifact-r1-site-clippy.log, Format /tmp/brain-k-artifact-r1-fmt.log. Gehaltener Buildslot2, Cargo+1.97.1, SQLX_OFFLINE=true nur Prüfkonfiguration.

Nichtblockierender neuer Gate-NIT an maintenance/compare_artifact.rs:48 nachgelesen: hero_compare_render.rs:315-316 validiert zuerst,:205-213 prüft vollständige Boonreihen und identische Positionen,:214-223 ausschließlich endliche nichtnegative Quantified-Werte. Voraussetzungen vor unreachable und Rekonstruktion erfüllt. Keine weitere Produktänderung nötig. Regulärer gemeinsamer ALLOW in /tmp/k-compare-artifact-r1-gate-20261007.log, Featurepush3c6f220b bestätigt.

Gebaut und regulär gemeinsam ALLOW: K-Hülle samt Originalbytefix. Gemergt: nein. Live: nein. Keine produktive Migration, Veröffentlichung, Kanalprobe oder Runtimewirkung. Echte G-Pins/Verifier und gemeinsamer Gesamtabschluss bleiben offen.
