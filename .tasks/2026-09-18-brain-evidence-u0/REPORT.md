# REPORT: Brain-Evidenz U0

Stand 18. September 2026. Branch `codex/patch-understanding-evidence-20260918`, Worktree `/home/nathanael/.worktrees/brain-evidence-u0-20260918`. Einziger Worker, keine Unter-Threads. Kein Merge, kein Deploy, keine Produktionsmigration.

## Umgesetzt

Der Originalpatch `follow-up-NOT-PUSHED-NOT-CI-TESTED.patch` war nicht verfügbar. Die zwei Korrekturen wurden am vorhandenen Code als neue Folgemigration umgesetzt; die bestehende Migration blieb unverändert.

1. **Erstbeobachtung (Korrektur 1).** Neue View `brain.patch_history_v1` mit Spalte `first_observed_at` = früheste `present`-Beobachtung desselben `content_hash`. Bleibt erhalten bei unverändertem Reimport, Löschen/Reimport und Rückkehr zu einem früheren Inhalt. Inhaltsänderung erhält eine eigene, spätere Erstbeobachtung. `earlier_observation_unknown` folgt der ersten Beobachtung des Inhalts, `observation_kind=baseline` bleibt nach Reimport erhalten. `first_observed_at` liegt nie nach der annotierten Revision, damit `known-at` keine spätere Revision benutzt. Der `knowledge_events`-Trigger `brain.correct_patch_knowledge_times` übernimmt dieselbe Semantik (`observed_at` = früheste Beobachtung des aktuellen Inhalts, `occurred_at` = Quelldatum, Metadaten mit `earlier_observation_unknown`). Die Revisionszeit `observed_at` bleibt als separate Spalte erhalten.
2. **Caption-Text-Abgleich (Korrektur 2).** Neue View `brain.youtube_caption_segments_v1` prüft zusätzlich `encode(sha256(convert_to(t.transcript_text,'UTF8')),'hex') = e.text_sha256`. Spaltenname `transcript_text` am Schema verifiziert (`youtube_transcripts`, auch von `transcripts.rs::save_transcript` beschrieben). Geänderter Text liefert keine alten Zeitsegmente; gleicher Text mit neuer Zeit bleibt eine eigene, per Zeiger gewählte Evidenzfassung, ältere Fassungen bleiben gespeichert.
3. **Folgemigration.** `scripts/migrations/2026-09-18-patch-evidence-followup.sql`, nur `CREATE OR REPLACE` von Views und Funktion plus einmaliges Neuberechnen bestehender `knowledge_events`. Wiederholbar, `lock_timeout=5s`, `statement_timeout=60s`, `REVOKE ALL ... FROM PUBLIC`. Keine Datenneuschreibung, keine PUBLIC-Rechte.
4. **Tests und CI auf beide Migrationen.** Neue Assertions `followup-history-assertions.sql` und `followup-caption-assertions.sql`. Fixture um `youtube_transcripts.transcript_text/content_hash` erweitert, bestehende Caption-Assertions auf echte SHA-256 umgestellt. CI: alte Migration allein lässt beide neuen Suiten nachweislich scheitern (`expect_fail`); alte plus Folgemigration bestehen jede Suite auf einer eigenen frischen DB; beide Migrationen idempotenzgeprüft. Der sonst ignorierte Caption-Integrationstest läuft im eigenen CI-Job echt gegen ein Scratch-Schema (`caption-scratch-schema.sql`) plus beide Migrationen (`cargo test ... -- --ignored --exact`).
5. **Doku.** `docs/AUTONOMOUS_PATCH_REVIEW.md` auf den tatsächlichen Stand und die Grenzen aktualisiert (Folgemigration, first_observed_at, Caption-Text-Abgleich, echter Scratch-Integrationstest, Scratch-Schema als reduzierte Obermenge). Keine zweite Plattform, keine Creator-Urteile, kein Videoimport.

## Geänderte Dateien

- `scripts/migrations/2026-09-18-patch-evidence-followup.sql` (neu)
- `tests/patch-understanding/followup-history-assertions.sql` (neu)
- `tests/patch-understanding/followup-caption-assertions.sql` (neu)
- `tests/patch-understanding/caption-scratch-schema.sql` (neu)
- `tests/patch-understanding/schema-fixture.sql` (erweitert)
- `tests/patch-understanding/schema-assertions.sql` (Caption-Assertions auf echte Hashes)
- `.github/workflows/patch-understanding.yml` (Postgres-Job auf beide Migrationen, Caption-Integrationsjob)
- `docs/AUTONOMOUS_PATCH_REVIEW.md`
- `.tasks/2026-09-18-brain-evidence-u0/{AUFTRAG,REGISTER,REPORT}.md`

Kein Produktionscode geändert. `transcripts.rs` und `deadlock-brain-patch-review.rs` blieben unverändert; die alte View-API ist rückwärtskompatibel (nur eine Spalte angehängt, Spaltenreihenfolge erhalten).

## Tests (wirklich ausgeführt)

Toolchain lokal: stable `cargo 1.97.1` (System-`cargo 1.75.0` kann Lockfile v4 nicht lesen). Eigenes Target-Verzeichnis `target-u0`, `--jobs 2`. Rust-Tests ohne DB-Bedarf laufen als Unit-Tests; der DB-Integrationstest lief gegen einen isolierten Wegwerf-Cluster (`initdb`, eigener Socket, eigener Port), nie gegen den laufenden lokalen Server.

Wörtliche Befehle (Env: `CARGO_TARGET_DIR=target-u0`, stable-Toolchain im PATH):

```
cargo test --locked --jobs 2 --package deadlock-brain-yt --bin deadlock-brain-yt
  -> test result: ok. 17 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out
cargo test --locked --jobs 2 --package deadlock-brain --bin deadlock-brain-patch-review
  -> test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
cargo test --locked --jobs 2 --package deadlock-brain --bin deadlock-brain pg_insights::tests
  -> test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 42 filtered out
cargo clippy --locked --jobs 2 --package deadlock-brain --package deadlock-brain-yt
  -> exit 0, 0 warnings, 0 errors
# Caption-Integration echt gegen Scratch-Cluster (Scratch-Schema + beide Migrationen):
DEADLOCK_CENTRAL_DSN=postgres://test@localhost:<port>/caption_scratch \
cargo test --locked --jobs 2 --package deadlock-brain-yt --bin deadlock-brain-yt \
  -- --ignored --exact transcripts::tests::save_transcript_is_idempotent_and_sets_ready_pg
  -> test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 21 filtered out; finished in 0.44s
```

Der Integrationstest lief in 0,44s echt (kein 30s-Connect-Timeout, kein stiller Skip); der erste Versuch mit falschem Socket-Verzeichnis wurde als stiller Skip erkannt und mit korrektem `unix_socket_directories`/TCP wiederholt.

SQL-Szenarien im isolierten `initdb`-Cluster (separater Port/Socket), alle grün:
- Baseline: `schema-fixture` + alte Migration + `schema-assertions` bestehen; alte Migration idempotent.
- Regressionsbeweis: alte Migration allein lässt `followup-history-assertions` und `followup-caption-assertions` scheitern (erwarteter Fehlschlag).
- Vollfix: `fixture` + alte + Folgemigration bestehen `schema-assertions`, `followup-history-assertions`, `followup-caption-assertions` (je eigene DB).
- Folgemigration idempotent (Revisionszahl unverändert).

TESTNACHWEIS[TW-1]: 33 passed, 4 ignored | Baseline: 0 rot

(33 = 17 yt + 9 patch-review + 6 pg_insights + 1 Caption-Integration; 4 = verbleibende ignorierte yt-Tests nach explizitem Lauf des Integrationstests. Kein bestehender Test geändert, daher keine Vorfehler.)

## Ignorierte/gefilterte Tests

- 4 weiterhin ignorierte yt-Tests: weitere DB-Integrationstests, die eine vollständige Brain-Scratch-DB brauchen und hier nicht bereitgestellt wurde. Nicht als bestanden gezählt.
- 42 gefilterte Tests beim `pg_insights::tests`-Lauf sind andere Tests des `deadlock-brain`-Bins, per Filter ausgeschlossen (Auftragsvorgabe).

## Offene Blocker und Grenzen

- Graphify: kein lokaler Graph im Worktree; die globale Graph-Datei (`~/.graphify/global-graph.json`) war lesbar und wurde für die Bestandssuche genutzt, keine Reindexierung. Der vermutete absolute Pfad wurde nicht umgangen.
- Das Caption-Scratch-Schema ist eine reduzierte, aber getreue Obermenge der vom realen Pfad berührten Spalten, nicht das vollständige Produktions-DDL (das liegt nicht im Repo). Der Integrationstest deckt damit den realen `save_transcript`-Pfad ab, nicht das komplette Brain-Schema.
- Betrieb und der unabhängige Primärdatenlauf U1 werden extern geprüft, nicht hier.
- Kein Selbst-Review; der Orchestrator prüft den Diff. Merge/Deploy/Produktionsmigration bleiben beim Orchestrator.
