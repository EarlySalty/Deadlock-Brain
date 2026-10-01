# S06 — Hybrid Retrieval: Startprüfung und Codebefund

Stand: 2026-09-24. Planpaket: 1.0.
Geprüfter integrierter Basis-Commit: `c00fc8935048bf490c1e4790f7c6195864ad49e2`.
Branch: `migration/s06-retrieval-preparation-20260924`.

**Paketstatus: blockiert; ausschließlich Lese-/Entwurfsvorbereitung geliefert.**
Kein implementierter Hybrid-Retriever, keine gemessene Suchentscheidung und keine G2-Abnahme.

## Startbedingung und Arbeitsgrenze

`architecture/migration/STATUS.md` und `GATES.csv` sind auf dem geprüften Commit eindeutig: G0 und G1 sind offen. Contract-Version, DB-Schema-Version und aktives Knowledge-Release sind nicht festgelegt. S000 gibt nur S01 und S10 `prepare_only`; S06 erhält dadurch keine Implementierungsfreigabe. Der direkte S06-Auftrag wird deshalb als Lese-/Entwurfsauftrag bearbeitet. Selbst eine Pilotmessung mit Quellen/Labels beginnt erst nach der im Plan vorgesehenen Freigabe.

`PFAD_OWNER.csv` reserviert `rust/crates/dbrain-retrieval/**` für S06 nach G1, ordnet `src/game_wiki*` aber zusätzlich vorläufig S12 zu. Diese Überschneidung muss S00 vor gemeinsamen Änderungen auflösen. Die vorliegende Änderung enthält nur dieses S06-Dokumentationsverzeichnis und die vorgeschriebene S06-Übergabe. Sie ändert weder `src/` noch Manifeste, Lockfile, globale CI, gemeinsame Verträge, Status-/Ownerregister, Datenbanken oder Dienste.

## Belegte Befunde, nicht Livebehauptungen

Alle folgenden Stellen beziehen sich auf den oben genannten Commit. Zeilenangaben sind repo-relativ. Codeexistenz beweist weder aktuelle Nutzung noch einen erfolgreichen Livepfad.

| ID | Beleg | Befund und Konsequenz |
|---|---|---|
| S06-F01 | `rust/crates/dbrain-retrieval/src/lib.rs:1714–1721` | `search_mechanic_notes` liefert bei fehlenden Tabellen eine leere Liste; bei vorhandenen Tabellen folgt `todo!`. Der Rust-Vektorpfad ist damit nicht implementiert und würde diesen Zweig mit Panic verlassen. Nach G1 über den gemeinsamen Retrieval-/Embeddingport implementieren; fehlender Index und zulässige leere Suche müssen unterscheidbar sein. |
| S06-F02 | `src/deadlock_brain/storage.py:155–179`; `src/deadlock_brain/retrieval.py:17–65` | Der Altcode enthält SQLite `sqlite-vec/vec0`, `float[384]`, `all-MiniLM-L6-v2` und L2-Distanz. Bei Suchfehlern fällt er auf `LIKE` zurück und setzt `distance: 0.0`. Modellrevision, aktive DB, vorhandene Vektoren und erfolgreiche Erweiterungsladung sind nicht belegt. Kein Nachweis eines heute laufenden Vektorstores. S01 muss die Vergleichbarkeit klären; ein Textfallback darf nicht als erfolgreiche Dense-Messung zählen. |
| S06-F03 | `rust/crates/dbrain-retrieval/src/lib.rs:4046–4192` | Claims werden über Entitäten und `lower(...) LIKE` geladen, per ID zusammengeführt und mit eigener Relevanz-/Intentlogik bewertet. Das ist eine vorhandene SQL-/Heuristikbaseline, kein belegter BM25- oder PostgreSQL-FTS-Index. Die SQL-Kandidatenabfragen begrenzen dort nicht selbst die Zeilenzahl; reale Lastfolgen sind ungemessen. |
| S06-F04 | `rust/crates/dbrain-retrieval/src/game_wiki.rs:195–354` | Die Wiki-Suche sammelt Markdown-Dateien, liest deren Inhalt und bewertet Einträge mit `score_page`; Tie-Breaks erfolgen über Titel/Pfad. Diese Dateiscan-Baseline und die vorhandenen Hero-/Ability-Zuordnungen erhalten. Sie ersetzt keinen Vergleich mit einem Suchindex. |
| S06-F05 | `rust/crates/dbrain-retrieval/src/lib.rs:321–327,997–1002`; `src/game_wiki.rs:114–121` | Die betrachteten Einstiegssignaturen führen keinen gemeinsamen autorisierten Kontext und kein gepinntes Knowledge-Release. Das beweist keinen ausnutzbaren öffentlichen Leak: Vorgelagerte Authentifizierung und Deployment wurden hier nicht geprüft. Es belegt aber nicht den neuen ACL-/Egress-/Releasevertrag. Keine private S06-Ersatzschnittstelle ergänzen. |
| S06-F06 | `rust/crates/dbrain-retrieval/src/lib.rs:997–1090`; `src/game_wiki.rs:195–218` | `ask_context` kombiniert mehrere DB-Lesewege mit lokalem Wiki-Inhalt. Die Wiki-Suche löst ihren Root auf, aber ein gemeinsamer DB-/Fact-/Wiki-/Dense-Releasepin ist in diesen Einstiegspfaden nicht vorhanden. Tests müssen den Wissensmix ausdrücklich abdecken; ein aufgelöster Dateipfad allein ist kein Knowledge-Release-Nachweis. |
| S06-F07 | `rust/crates/dbrain-retrieval/src/lib.rs:1632–1687` | `analysis_run_ai` erzeugt Modellrequests, ruft `AiClient::chat` auf und speichert Analysen. Diese vorhandene Funktion nicht still löschen. Herauslösen/Weiterleitung gehört als abgestimmte Paritätsarbeit zu S07/S08, nicht als unkoordinierter S06-Umbau. |
| S06-F08 | `rust/crates/dbrain-retrieval/examples/ask_latency.rs:95–167` | Vorhandene Messhilfe: ein read-only Build-Ask einschließlich Prozess-/Zugangs-/DB-/Reasoneraufwand. Sie ist kein getrenntes lexical/dense-Benchmark und keine unabhängige Recall-Evaluation. Ihr Ergebnisobjekt enthält vollständigen Ask-Inhalt; ohne Rechte-/Redaktionsprüfung keine Veröffentlichung dieses Rohartefakts. Die Live-Messhilfe wurde nicht ausgeführt. |

## Vorhandene Tests tatsächlich ausgeführt

Auf dem unveränderten Basis-Commit, ohne `--ignored`, ohne Liveabfrage und ohne Provideraufruf:

```sh
$HOME/.cargo/bin/cargo +1.88.0 test \
  --manifest-path rust/Cargo.toml --locked --offline \
  -p dbrain-retrieval --lib -j 2
```

Ergebnis: **37 bestanden, 0 fehlgeschlagen, 14 ignoriert, 0 gemessen**. Die 14 ignorierten Fälle benötigen Scratch-Postgres (12) oder das explizit versionierte echte Game-Wiki (2). Sie sind nicht als bestandene Integration zu zählen. Die gemeldete Testlaufzeit von 0,12 Sekunden ist keine Retrieval-Latenzmessung.

Der erste Versuch mit dem unqualifizierten Systembefehl `cargo test ...` scheiterte vor dem Build: Lockfile-Version 4 benötigt dort `-Znext-lockfile-bump`. Anschließend wurde die bereits installierte Toolchain 1.88.0 explizit genutzt. Weder Toolchaininstallation noch Lockfileänderung wurden vorgenommen; daraus folgt keine neu freigegebene Workspace-MSRV.

## Artefakte und nächster Schritt

- `PILOT.md`: Vergleichsablauf und Such-/Embedding-ADR im Status Entwurf, ohne vorweggenommene Auswahl.
- `CASES.csv`: vorgeschlagene Sicherheits-, Versions-, Such- und Fehlerfälle; keine ausgeführten Tests und keine Relevanzlabels.
- `pilot-manifest.json`: explizit unvollständiger Freeze-Entwurf; fehlende Corpus-/Label-/Modellfreigaben bleiben `null`.
- `CONTRACT_REQUEST.md`: konkrete Übergaben an bestehende Besitzer, keine neuen öffentlichen Typen.
- `architecture/migration/handoffs/06-retrieval-suchindex.md`: Testergebnis, Grenzen und Übergabe an S00.

S00 prüft S01/S10 für G0 und S02/S03 für G1. Erst auf dem danach integrierten Commit und mit expliziter S06-Freigabe beginnt die Implementierung. Weder ein grüner Dokumentations-PR noch die vorhandenen Unit-Tests schließen diese Gates.
