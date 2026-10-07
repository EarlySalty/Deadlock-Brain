# Paket D: Code-Ernte

Stand: 07.10.2026, 02:10 CEST. Vergleichsbasis: frisch geholtes `origin/main` = `fde910f6a0199c00f44083e73fc8f4c5e4f80b86`.

## Ergebnis und Zählweise

Alle 44 historischen „übernehmen“-Zeilen aus `C/OFFEN.md`, ihre 21 nachträglich gesicherten WIP-Stände und die fünf tatsächlichen PR-Heads sind geprüft. **36 der 44 historischen Stände tragen noch fehlenden Rust-Code oder zusätzliche Tests.** Das sind ausdrücklich keine 36 unabhängigen Features: zahlreiche Stände tragen denselben alten Caption-/Patch-Review-Anteil als Vorfahren. Acht Stände haben keinen zusätzlich benötigten Eigenanteil mehr. Die WIP-Ergänzungen werden unten gesondert ausgewiesen und nicht als weitere historische Stände gezählt.

PR-Heads per `gh pr view --json headRefOid,state,files` verifiziert; alle fünf PRs sind geschlossen. PR #9 zeigt inzwischen auf den gesicherten WIP `79f5a020`, nicht mehr auf den alten Tabellenstand `e752d251`.

Untersucht wurden die tatsächlichen Rust-Dateien und Testkörper, die Änderungen vom jeweiligen Merge-Base sowie Gegenvergleiche mit Main. Ein fehlender Funktionsname allein gilt nicht als Funktionslücke. Beispielsweise sind Veröffentlichungsrechte, URL-Prüfung und mehrere Deadline-Gegenproben auf Main unter anderen Namen neu umgesetzt. Kein Produktcode, Dienst, Datenbestand oder fremder Worktree wurde für diese Inventur geändert.

## Wiederverwendbare Bereiche und Gegenbelege

Die Pfade in diesem Abschnitt beziehen sich auf `rust/crates/`, sofern nicht anders angegeben. `Tag:Pfad:Zeile` meint den Inhalt im genannten Git-Stand, nicht den veränderten kanonischen Checkout.

| Kennung | Tatsächlicher fehlender Anteil und Codebeleg | Main-Gegenbeleg / Nutzen / Zuständigkeit |
| --- | --- | --- |
| K1 | Caption-Rohbeleg, Zeitsegmente, Stück-Offsets und revisionssensibler Hash: `aa2a051e:deadlock-brain-yt/src/transcripts.rs:48`; ältere Parser enthalten zusätzliche Unicode-/Wiederholungs-/Grenztests. | Main `transcripts.rs:47` hält nur `source_kind` und `transcript_text`, JSON3 ab :94 nur `segs.utf8`. Mittel für belastbare Patch-Story; YouTube bleibt bewusst pausiert. A-F4 kann Legacy-Aufruf-/Konfigdateien berühren, diese nicht parallel übernehmen. |
| K2 | Offline-Patch-Review mit Belegrevision, Ereignis-/Snapshotprüfung, Bedingungen, Gegenbelegen und nicht als Fakt ausgegebenem Storyboard: `aa2a051e:deadlock-brain/src/bin/deadlock-brain-patch-review.rs`; abgesicherte Insight-Aufbereitung in `aa2a051e:deadlock-brain/src/pg_insights.rs:112`. | Die Review-Binary fehlt auf Main; Main `pg_insights.rs:112` besitzt nicht diesen kompletten Vertrag. Mittel für Patch-Story. `main.rs`/Patch-Import gehören gleichzeitig A; unabhängige Review-/Insightdateien bevorzugen, Verdrahtung nicht doppeln. |
| K3 | Getrennter Analyse-/Reviewvertrag für Patch-Insights mit Zielpatch-Belegen, maximal 24 Punkten, expliziten Bedingungen und Gegenbedingungen: `49151073:deadlock-brain/src/bin/patch_insights/contract.rs:60`. | Main hat weder diese Binary noch das Vertragsmodul. Mittel, aber Überschneidung mit K2. Nur einen Analyseweg anschließen, keine zwei Antwortwege schaffen. |
| K4 | Rust-MCP-Werkzeuge für Patchhistorie, Patchsuche, Patchliste und Entity-Summary in `f34cd867:deadlock-brain/src/bin/deadlock-brain-mcp.rs`. | Main hat `brain-mcp`, aber nicht diesen historischen Werkzeugvertrag. Niedrig für den jetzt priorisierten Discord-/Twitch-Antwortweg; historischen Vertrag dokumentieren, keinen zweiten MCP-Dienst starten. |
| K5 | Validierter typisierter TOML-Snapshot mit unbekannte-Felder-Verbot, sicheren Pfaden, Grenzen, Fingerprint und unverändertem gültigem Snapshot nach fehlerhaftem Reload: `40b43ac8:deadlock-brain-core/src/bot_config.rs:35,105,122`; ältere Gegenproben in `e4a9fe6c:.../bot_config_tests.rs`. | Main `deadlock-brain-core/src/config.rs:86` lädt noch `.env`-Settings, `model_resolver.rs:8` existiert schon. Mittel für konsistenten Builds-Betrieb. **A-F4 hat Vorrang** auf Konfig-/AI-/Startdateien; keine vollständige TOML-Umstellung parallel. Modellresolver/Modellkonfiguration niemals pauschal rückübernehmen. |
| K6 | Zusätzliche abgesicherte Config-/Credential-Dateileser gegen FIFO/Symlink/Übergröße sowie Descriptor-Gegenproben und Pool-Gegenproben aus `35c93de0:deadlock-brain-core/src/config.rs,pg_secrets.rs`, `457188a9:.../pg.rs`. | Main `pg.rs:25` setzt Read-only schon als Startupoption; Main Credentials wählen benannte Systemd-Datei bereits vor FD. Das ist kein fehlender vollständiger Rechtefix. Zusätzliche Dateischutz-/Regressionsteile fehlen. Hoch für verlässlichen Start, **A-F4 hat Vorrang**. |
| K7 | Zentraler Abo-Prozessprovider und separater Dialogue-Vertrag: `35c93de0:brain-providers/src/codex.rs`, `brain-contracts/src/lib.rs`; 80 Abo-Prüfbelege im WIP. | Dateien fehlen auf Main. Niedrig für die aktuelle Ernte: würde Provider-/Modellbetrieb ändern, ist keine Freigabe zum Modellwechsel. Nur Belege und Vertrag erhalten. |
| K8 | Gegenprobe gegen generische Auslieferung typisierter Domain-Dokumente einschließlich Dense-Zweig ohne Embedding-Aufruf: `60dec8f5:dbrain-retrieval/tests/core_retrieval.rs:54`. | Main `release_port.rs:88` filtert `domain_contract`/`domain_input` schon aus dem Prose-Index. Der kombinierte Test fehlt. Hoch; **Tests portieren, den älteren Retriever nicht zurückbauen**. Kein A-Codeeigentum betroffen. |
| K9 | Ergänzende Warm-Index-Gegenprobe nach Egresswiderruf bzw. entfernter aktueller Herkunft in `6c6dd8ba:brain-api/tests/publication.rs:412`; Gegenprobe für ungültige Dienstbudgets in `0e91b977:brain-api/tests/review_deadlines.rs`. | Nach vollständigem Testkörpervergleich: transitive Metadatenrechte sind durch `publication_checks_direct_and_transitive_metadata_dependencies_pinned_and_current`, Partialbody-Keepalive durch `unread_keepalive_bodies_close_after_auth_failure_or_timeout` bereits ersetzt. Hoch für die beiden verbleibenden Gegenproben; nur Testdateien ergänzen, keine alten API-/Storageimplementierungen und keine Duplikate. |
| K10 | Generische Spielstilplanung für Weapon/Spirit/Tank mit Meta-Filterung und Berücksichtigung von Spirit-Skalierung der Waffe: `3e7864a5:dbrain-reasoner/src/lib.rs:127,195,300`. | Main besitzt weder diese Funktionen noch einen Spielstilpfad in `core_rules.rs`/`domain_builds.rs`. Hoch für Builds-Skill. A baut Skilldispatch/Matchdaten, D liefert nur den Reasoner-Anschluss. **Review-Publish mit abgesenkter Prüfung wird nicht übernommen:** alter `publish.rs:143` ermöglicht ein bewusst ungeprüftes Review-Build. Die Qualitätsgrenze des Auftrags gilt weiterhin. |
| K11 | Wiederholung vorübergehender HTTP-/Bodyfehler des Build-Datenclients mit gezielten Gegenproben: `8318690c:dbrain-builds/src/api.rs`. | Main liest mit einem einzelnen `send()`/Bodyabruf. Mittel für ausreichend frische Builddaten. A hat den Matchdatenauftrag; deshalb vor Änderung des Clients Eigentum melden. Keine alten Systemd-Timer pauschal übernehmen, keine fest einkompilierten neuen Zeitlimits. |
| K12 | Bitgenauer eingefrorener Reasoner-Prüfbestand, reservierte Scratch-Datenbank und zusätzliche `f64`-Roundtrip-Gegenprobe: `79f5a020:dbrain-reasoner/examples/family_evaluation.rs:527`, `.../dbrain-builds/examples/scratch_asset_catalog.rs`. | Main enthält inzwischen viele Familien-/Replay-Prüfhelfer, aber nicht jede alte Prüfung. Niedrig für das Grundding; Belegbestand bewahren statt einen zweiten Build-/Messweg einzurichten. Historische Assets-Endpunkte/Versionsauswahl nicht wiederbeleben: Main `dbrain-sources/src/assets_api.rs:16` hat den aktuellen Host plus Schema-Preflight :75. |
| K13 | Echter Replay-Import/Validierung und zusätzliche Decoderzustände: `e202beac:dbrain-replay/src/import.rs,validation.rs,frame.rs`; `919c790f:.../tests/validation.rs`. | Funktion fehlt teilweise auf Main. Niedrig, von A ausdrücklich zurückgestellt; nicht portieren. |
| K14 | Begrenzt entpackte XML-/Gzip-Sitemaps: `6946269f:dbrain-sources/src/forum.rs`, `decode_sitemap_xml` mit Grenzen für komprimierte und entpackte Bytes. | Decoder fehlt auf Main. Niedrig, Forum ausdrücklich zurückgestellt; nicht portieren. |
| K15 | Q: Quellenaufbewahrung, vollständiger Sheet-/YouTube-Feed, einmaliges redigiertes Request-Audit, Provider-Shadowprüfer. | Mehrere Module fehlen auf Main, Q steht geschützt am Haltepunkt. Niedrig für den priorisierten Antwort-/Buildpfad; keine Aufbewahrungslöschung oder Q-Freigabe durch D. |
| K16 | Z: abgesicherte Legacy-Snapshotgeneration und atomarer Refresh; einzelne zusätzliche Releaseprüfungen. | `ops/brain-postgres/legacy-refresh` fehlt auf Main, normaler `brain-release` existiert. Niedrig; Z/G5-Sperren respektieren, keinen zweiten Releaseweg bauen. |
| K17 | Vollständiger Serverguide mit Personen-/Kanalbindung, Profilen, Widerruf, privatem Egress und Aktionen. | Main hat den absichtlich engeren Serverguide-MVP. Niedrig für die Ernte, A besitzt den gemeinsamen Serverwissens-/Antwortanschluss. Der alte Invite-Aktionspfad darf A-E3/E4 nicht ersetzen. |
| K18 | Zwei zusätzliche profilspezifische Live-/Scratch-Diagnosen `bb3ebf31:brain-maintenance/src/integration/entity_profiles.rs:1204,1309`. | Tests laden echte `/etc`-Konfiguration und teilweise echte Prod-Snapshots; kein portabler Standardtest. Niedrig, nur Belegdoku; A-F1b/F3b haben hier Vorrang. |

## Die 44 historischen Entscheidungen

`Lücke ja` bedeutet, dass der Stand fehlenden Code oder einen zusätzlichen Testkörper trägt. Gemeinsame Vorfahren werden dabei sichtbar gemacht und niemals mehrfach portiert. Tag-Namen sind `archiv/` plus der Name in Spalte 2, außer den ausdrücklich noch aktiven bzw. lokal gesicherten Ständen. Supersedierte Tags können auf den neueren WIP zeigen; der historische SHA bleibt deshalb immer dabei.

| Nr. | Stand / historischer SHA | Fehlender Inhalt in einem Satz | Lücke | Nutzen | Konflikt mit A | Empfehlung |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | `fix/sheet-sync-secret-exec-20260924` / `dcee4761` | Trägt K1/K2 aus der alten Basis; der Secret-Exec-Eigenanteil ist auf Main vorhanden. | ja | mittel | A-F4 Startpfad | K1/K2 einmal portieren, Secret-Exec schon ersetzt. |
| 2 | `codex/brain-deploy-completion-20260918` / `458d56d0` | Trägt K1/K2 samt fehlenden Revalidierungsgegenproben. | ja | mittel | A-F4 / Patchverdrahtung | K1/K2 einmal portieren. |
| 3 | `codex/brain-luna-abo-20261003` / `9d236af5` | K6-Dateischutz und K7-Abo-Provider fehlen. | ja | hoch für K6, niedrig für K7 | A-F4 | K6 an A anschließen; K7 nur Belegdoku. |
| 4 | `codex/brain-luna-dialogue-20261003` / `35c93de0` | Ergänzt K6 um K7-Dialogue mit Datenschutz- und Budgetprüfung. | ja | hoch für K6, niedrig für K7 | A-F4 / A-E3 | K6 einmal anschließen; Dialogue nicht als zweiten Antwortweg portieren. |
| 5 | `codex/brain-pr61-pr9-integration-20261001` / `97515060` | Der K8-Test fehlt, URL-Schutz und normaler Publish sind inzwischen neuer umgesetzt. | ja | hoch | keiner auf Testdatei | K8-Test portieren; alten Client schon ersetzt. |
| 6 | lokaler `codex/core-completion-20260925`, gesichert als `archiv/luna/finish-brain-core-completion-acl-20261001` / `60dec8f5` | Derselbe K8-Test fehlt. | ja | hoch | keiner | Schon durch Portierung aus Zeile 5 abdecken. |
| 7 | `codex/brain-release-20260918` / `9ead4617` | K1/K2 fehlen trotz vorhandenen heutigen Releasewerkzeugs. | ja | mittel | A Patchverdrahtung | K1/K2 einmal portieren. |
| 8 | `codex/fix-pr61-publication-20261001` / `8d355fce` | Kein fehlender Rechtezweck gefunden: Main prüft aktuelle und gepinnte Freigaben samt Flights und Cache. | nein | keiner | A-E3 | Schon ersetzt; alten Rechte-/Cachepfad nicht portieren. |
| 9 | `codex/fix-pr61-publication-enforcement-20261001` / `27671409` | K8 und eine zusätzliche patchagnostische Release-/Mode-Gegenprobe fehlen als Testkörper. | ja | hoch | A-E3 nur gemeinsame Crates | Nützliche Tests portieren; Rechte-/URLfunktion schon ersetzt. |
| 10 | `docs/reasoner-all-heroes` / `8c98c63e` | Kein zusätzlicher Rust-Code, nur Dokumentation. | nein | niedrig | keiner | Nur Belegdoku, nicht portieren. |
| 11 | aktiver `feat/brain-fertig-q-20261003` / `96bf05c4` | K15-Aufbewahrung, Feed und Audit fehlen teilweise. | ja | niedrig | geschütztes Q / A Wartung | Nur Belegdoku, nicht portieren. |
| 12 | `feat/brain-fertig-r-20261003` / `e202beac` | K13-Replayimport und reale Decodervalidierung fehlen. | ja | niedrig | A hat Replays zurückgestellt | Nicht portieren. |
| 13 | aktiver `feat/brain-fertig-z-20261003` / `f1026afd` | K16-Legacy-Refresh fehlt. | ja | niedrig | geschütztes Z/G5 | Nicht portieren. |
| 14 | `feat/brain-final-integration-20260921` / `22c8ebe0` | Trägt K1/K2/K4/K6, nicht nur einen Releasebericht. | ja | hoch für K6, mittel für K1/K2 | A-F4 | Fehlende Eigenanteile selektiv; MCP nur Belegdoku. |
| 15 | `feat/brain-global-toml-20260920` / `e4a9fe6c` | K5-TOML-Snapshot samt Grenzen und Reloadtests fehlt. | ja | mittel | A-F4 | Portierung nur nach A-Eigentumsübergabe, neuerer `40b43ac8` als Anschluss. |
| 16 | `feat/brain-release-completion-20260921`, alter Alias `feat/brain-meta-publish-emojis` / `e752d251` | Trägt K1/K2/K4/K6 und K12-Prüfbestand. | ja | hoch für K6, mittel für K1/K2 | A-F4 / Reasoner | K1/K2/K6 deduplizieren; K12 nur Belegdoku. |
| 17 | `feat/brain-runtime-release-20260921` / `457188a9` | K6-Pool-/Descriptor-Gegenproben fehlen, der Grundschutz existiert schon. | ja | hoch | A-F4 | Nur verbleibende Schutzteile/Gegenproben, keine alte Startupbasis. |
| 18 | lokaler `feat/brain-rust-cutover-20260919`, Tag `dependency/s00-planpaket-20260924` / `2734c2da` | Der Plan selbst ist Doku, der Stand trägt außerdem dieselben fehlenden K1/K2-Vorfahren. | ja | mittel für K1/K2 | veränderter Kanon / A | K1/K2 aus Fachstand ernten; Plan nicht als Feature übernehmen. |
| 19 | `feat/brain-s01-inventory-20260924` / `76205b5f` | Inventurartefakte ergänzen keinen Dienst, aber die Basis trägt K1/K2. | ja | mittel für K1/K2 | A Inventur | Nur denselben Fachanteil ernten, Inventur nur Belegdoku. |
| 20 | `feat/brain-wiki-spielwissen-a` / `116f643a` | Wiki-Eigenanteil ist ersetzt; aus der alten Basis bleiben K1/K2. | ja | mittel für K1/K2 | A Stufe 2 | Wiki schon ersetzt, K1/K2 einmal ernten. |
| 21 | `feat/brain-wiki-spielwissen-b` / `0aa0d9ee` | Spieldatei-Eigenanteil ist reorganisiert vorhanden; aus der Basis bleiben K1/K2. | ja | mittel für K1/K2 | A Spielwissen | Parser schon ersetzt, K1/K2 einmal ernten. |
| 22 | `feat/brain-wiki-spielwissen-c` / `c3b9cf59` | Kein zusätzlicher aktueller Profilpfad, aber gleiche K1/K2-Basis. | ja | mittel für K1/K2 | A-F1b/F3b | Profilpfad schon ersetzt, K1/K2 einmal ernten. |
| 23 | lokaler `feat/brain-wiki-spielwissen-c-integration` / `f7a03f9a` | Kein zusätzlicher notwendiger Integrationspfad nach Vergleich mit aktuellen Projektionen und Importverträgen. | nein | keiner | A-F1b/F3b | Schon ersetzt; neueren WIP separat prüfen. |
| 24 | `feat/serverguide-brain-20261003` / `849926b2` | K17-Gesamtguide geht über den vorhandenen MVP hinaus. | ja | niedrig | A gemeinsamer Antwortweg | Nicht portieren. |
| 25 | `fix/brain-spielwissen-c-liveprofile-20261005` / `bb3ebf31` | K18-Live-Diagnosetests fehlen, nicht ein zusätzlich benötigter Profilfix. | ja | niedrig | A-F1b/F3b | Nur Belegdoku, nicht portieren. |
| 26 | `fix/discord-brain-antwort` / `4eac7503` | SHA ist bereits Mainvorfahr; heutiger Bots-Antwortfix ist A-Arbeit, kein neuer Brain-Fachanteil dieses SHA. | nein | keiner | A-F2c | Schon ersetzt / A Vortritt. |
| 27 | `fix/forum-xml-sitemap-20261003` / `6946269f` | K14-Gzip-Decoder mit zwei Größenbegrenzungen fehlt. | ja | niedrig | Forum zurückgestellt | Nicht portieren. |
| 28 | `fix/paket-b-timer-20260930` / `fd6e1576` | K11-Wiederholung vorübergehender Build-Datenfehler fehlt. | ja | mittel | A Matchdatenauftrag | Neueren WIP `8318690c` selektiv portieren, Client-Eigentum zuvor melden. |
| 29 | `fix/reasoner-mechanics-completion` / `3e7864a5` | K10-Spielstilplanung fehlt trotz vorhandenem Mechanikkern. | ja | hoch | A Skilldispatch / Matchdaten | Reasoner-Fachanteil portieren; Review-Publish nicht übernehmen. |
| 30 | `integrate/serverguide-deploy-20261003` / `1d820bc2` | K17 enthält zusätzliche Guideaktionen einschließlich eines alten Invitepfads. | ja | niedrig | A-E3/E4 | Nicht portieren, kein paralleler Invite-/Antwortpfad. |
| 31 | `integration/final-local-20260926` / `870c7a15` | Isolierter Server-/Reader-Eigenanteil ist durch spätere Connection-/Deadline-/C9-Implementierung ersetzt. | nein | keiner | A gemeinsamer Reader | Schon ersetzt. |
| 32 | `integration/patch-analysis-evidence-20261001` / `49151073` | K3-Patchanalyse sowie K1/K2/K4 fehlen weiterhin. | ja | mittel | A Patchstory / K2-Deduplizierung | Einen gemeinsamen Reviewvertrag ernten, keine alten Gesamtintegration. |
| 33 | aktiver `integration/technical-closeout-20260929` / `20b279a6` | Kein eigener Produktcode gegenüber Merge-Base, nur Abschluss-/Belegstand. | nein | niedrig | G5 geschützt | Nur Belegdoku, nicht portieren. |
| 34 | `fix/patch-insights-evidence-20260918` / `089be38c` | K3 plus separater Caption-Belegadapter fehlt. | ja | mittel | K1/K2 / A Patchstory | Dedupliziert ernten, Python-History-Tools nicht produktiv übernehmen. |
| 35 | `codex/patch-understanding-evidence-20260918` / `9efeb1e4` | K1/K2 einschließlich Beleg-/Bedingungsgegenproben fehlt. | ja | mittel | A Patchstory | Neueren integrierten `aa2a051e` bevorzugen, ursprüngliche Tests erhalten. |
| 36 | `luna/abschluss-brain-pr4-integrate-pr61-20261001` / `aa2a051e` | K1/K2 hat den neueren kanonischen Patch-ID-Vertrag und Review-Migration. | ja | mittel | A Patchverdrahtung | Selektiver Erntekandidat für K1/K2, keine bestehende Migration verändern. |
| 37 | `codex/fix-c10-real-replay-validation` / `919c790f` | K13-Datenschutz-/Hash-/Pfadvalidierung fehlt als kompletter Replaypfad. | ja | niedrig | Replay zurückgestellt | Nicht portieren. |
| 38 | `luna/abschluss-brain-pr9-20261001` / `7deebcb7` | K12-Prüfbestand und die gemeinsamen K1/K2/K4/K6-Vorfahren bleiben teilweise fehlend. | ja | mittel für K1/K2, hoch für K6 | A-F4 / Reasoner | Gemeinsame Fachanteile einmal; K12 nur Belegdoku. |
| 39 | `luna/finish-brain-global-toml-20261001` / `40b43ac8` | K5 bindet den typisierten Snapshot zusätzlich an bestehende Verbraucher. | ja | mittel | A-F4 | A Vortritt; nach Eigentumsübergabe selektiv, kein Modellwechsel. |
| 40 | aktiver Remote-Stand `feat/brain-rust-cutover-20260919` / `c8ad3ef6` | Trägt K1/K2/K4 und zusätzliche Runtime-/YouTube-Altanteile. | ja | mittel | Kanon / A-F4 | K1/K2 aus Fachstand ernten, keine Altbasis mergen. |
| 41 | aktiver `review/pre-g5-core-abnahme-20260929` / `9070ba94` | Kein eigener Produktcode, Abnahmebelege. | nein | niedrig | G5 geschützt | Nur Belegdoku. |
| 42 | `sol/c9-brain/7bf0e0375ee34a00` / `357548d7` | C9-Operator-/Wartungs-/internes HTTP ist auf Main vorhanden und erweitert. | nein | keiner | A Wartung / A-E3 | Schon ersetzt. |
| 43 | `luna/brain-codex-brain-deploy-completion-20260918-458d56d` / `3d9098c6` | Zusätzlicher Bericht, aber derselbe fehlende K1/K2-Code wie Zeile 2. | ja | mittel für K1/K2 | A Patchverdrahtung | Fachanteil einmal ernten, Bericht nur Belegdoku. |
| 44 | `luna/brain-codex-brain-release-20260918-9ead461` / `79ec3857` | Zusätzlicher Bericht, aber derselbe fehlende K1/K2-Code wie Zeile 7. | ja | mittel für K1/K2 | A Patchverdrahtung | Fachanteil einmal ernten, Bericht nur Belegdoku. |

## Die 21 neueren Sicherungen

Diese Stände verhindern, dass bei der Ernte versehentlich ein älterer Parent den zuletzt gesicherten Bestand verdrängt. Ein WIP-Tag ersetzt nicht automatisch die fachliche Prüfung seines Parents.

| Stand / SHA | Ergebnis am Code und Empfehlung |
| --- | --- |
| `feat/brain-meta-publish-emojis` / `36588add` | K1/K2/K4/K6/K12; kein eigenständiges zusätzliches Emoji-Feature auf dem gemeinsamen Antwortweg. Fachanteile deduplizieren. |
| `fix/paket-b-timer-20260930` / `8318690c` | Verfeinerter K11-Transport-/Bodyretry, gegenüber `fd6e1576` bevorzugen; Systemd-Altänderungen nicht rückübernehmen. |
| `fix/brain-rust-cutover-gate-20260930` / `3d74ae54` | Zusätzlicher Scratch-/Gatebeleg neben alter Cutoverbasis, kein neuer aktueller Releaseweg. Nur Belegdoku. |
| `codex/core-completion-20260925` / `e2bd0b22` | Letzte Sicherung fügt keinen neuen Rust-Eigenanteil hinzu; K8 aus ursprünglichem Fachfix separat erhalten. |
| `feat/brain-final-integration-20260921` / `7e4d5e6c` | Letzte Sicherung fügt keinen neuen Rust-Eigenanteil hinzu; gemeinsame K1/K2/K4/K6 wie historischer Stand. |
| `backup/wip-functional-closeout-20261006` / `0e91b977` | K9: ungültige Dienstbudgets zusätzlich prüfen; Partialbody-Keepalive ist im aktuellen Main bereits ersetzt. Alten Reader nicht rückübernehmen. |
| `feat/brain-global-toml-20260920` / `38e4e559` | K5 plus umfangreicher neuer Modellkatalog-/Resolver-WIP. Katalog/Probeauswahl nicht eigenmächtig aktivieren; A-F4 Vortritt. |
| `codex/brain-luna-abo-20261003` / `ee85ff14` | Gegen Parent keine weiteren Ruständerungen, zusätzliche Host-/Abo-Belege. Nicht als 80 fehlende Features zählen. |
| `integration/pr61-luna-20261001` / `6c6dd8ba` | K9: Warm-Index nach geänderter aktueller Herkunft zusätzlich prüfen. Transitive Metadatenrechte sind durch den heutigen pinned/current-Test ersetzt; Dotted-IDs/Routegrenzen durch `brain-feeds/tests/build_publish_endpoint.rs:727,1046`. Nur die verbleibende Warm-Index-Gegenprobe portieren. |
| `integration/brain-v1-closeout-20261001` / `4dbf3940` | Gleiche zwei Testdateien und gleicher Inhalt wie `6c6dd8ba`; kein zweiter Port. |
| `backup/wip-pr61-review-20261006` / `28ac5c6f` | Startup-Deadline- und Rechteprüfstand; Main hat Connection-/Startup-/Publication-Gegenproben bereits. Nur Belegbestand, kein alter Parallelharness. |
| `backup/wip-pr61-scratch-20261006` / `7d60f287` | Drei ältere Regressionstestdateien zu Partialbody, URL-Userinfo und unzitierten Inputs; durch Main `review_deadlines.rs`, `build_publish_endpoint.rs`, `review_publication.rs` funktional ersetzt. Nur Belegdoku. |
| `feat/brain-release-completion-20260921` / `79f5a020` | Tatsächlicher PR-9-Head; gegenüber `e752d251` nur zusätzlicher dauerhafter Prüflauf in `.tasks`, nicht neuer Produktcode. K1/K2/K6 deduplizieren. |
| `luna/finish-brain-rust-cutover-20261001` / `f34cd867` | Trägt K4 und K12 samt neuerer Cutoverbelege; niedrige Teile nicht als zweiten Brainpfad aktivieren. |
| `codex/luna-native/brain-pr4-evidence-20261001` / `1f4bb874` | K1/K2 in neuerer Integrationsbasis; `aa2a051e` für eigentlichen Reviewvertrag plus ursprüngliche Gegenproben vergleichen. |
| `feat/brain-wiki-spielwissen-c` / `209f1fbb` | Gegen historischen `c3b9cf59` keine zusätzlichen Rust-/TOML-/Shelländerungen; lokale wertvolle Archive bleiben unabhängig erhalten. |
| `feat/brain-wiki-spielwissen-a` / `80d1ca81` | Gegen `116f643a` keine zusätzlichen Rust-/TOML-/Shelländerungen; Wiki-Verarbeitung heute A. |
| `feat/brain-wiki-spielwissen-b` / `b9be82ce` | Gegen `0aa0d9ee` keine zusätzlichen Rust-/TOML-/Shelländerungen; kein zweiter Spieldateiimport. |
| `feat/brain-wiki-spielwissen-d` / `60610fce` | Sicherungscommit ohne neuen Rust-Eigenanteil; vorhandene D-Verträge nicht als ungebautes Feature zählen. |
| `worktree-wiki-spielwissen-status-s` / `2f35c6b4` | Status-/Belegsicherung, kein neuer Rust-Eigenanteil. Nur Belegdoku. |
| `backup/wip-wiki-c-integration-20261006` / `74f6500e` | Projektion, lokaler Reader, idempotente Releasewiederaufnahme und Index-Preflight liegen inzwischen auf Main unter den gleichen Funktionen. A Stufe 2 hat Vortritt; kein erneuter Integrationsport. |

## Fünf PR-Heads

| PR / exakter Head | Fehlender Anteil / Nutzen / Konflikt / Empfehlung |
| --- | --- |
| #3 / `089be38c5e4d755abb59952ba74d4e1cdfc86378` | K3 und Caption-Belege fehlen; mittel; K1/K2/A Patchstory; nur einen deduplizierten Fachvertrag portieren. |
| #4 / `9efeb1e44ead5cdf5d01e05f242291fee79e803e` | K1/K2 fehlen; mittel; A Patchstory; neuere integrierte Fachdateien mit ursprünglichen Gegenproben verwenden. |
| #5 / `9ead46171f3d0f3c5d0e2fe739f1a9e693e37013` | Dieselben K1/K2 fehlen; mittel; A Patchverdrahtung; kein zusätzlicher Gesamtmerge. |
| #6 / `458d56d0845f83bb50fdb635e78c0367f3cff8a1` | K1/K2 plus Claim-Revalidierungsgegenproben fehlen; mittel; A-F4/Patchverdrahtung; dedupliziert portieren. |
| #9 / `79f5a020fdec562658a8d947bce32321e48974db` | K1/K2/K6 sowie niedrige K4/K12-Anteile fehlen; hoch für Startschutz, mittel für Patchreview; A-F4 Vortritt, keine alte Komplettbasis. |

## Freigegebene Ausführungsreihenfolge

Die inzwischen gelesene Vorabfreigabe in `VON_HAUPT.md`, 07.10. 01:45 Punkt 3, ersetzt den ursprünglichen Wartepunkt. D beginnt nach dieser Inventur ohne weitere Freigabefrage.

1. K8/K9: fehlende nützliche Sicherheits-/Rechtegegenproben auf aktuellem Main, ohne ältere Produktimplementierungen zurückzubauen.
2. K1/K2/K3: einen deduplizierten Caption-/Patch-Review-Fachanteil ernten; bestehender Provider und gemeinsame Ingest-/Antwortverträge bleiben maßgeblich.
3. K10: Spielstilplanung des Reasoners an heutige Typen anschließen, normale Publishgrenze unverändert; keinen Review-Publish-Bypass übernehmen.
4. K6/K5/K11: Überschneidung vorab an A über `AN_HAUPT-D.md` melden. A hat Vorrang; diese Dateibereiche nicht parallel verändern.

K4/K7/K12 bis K18 sind nicht für den priorisierten Port freigegeben. Bestehende angewandte Migrationen bleiben unverändert. Tests sind in dieser Inventur nicht ausgeführt; ihre Körper wurden geprüft. Laufzeitwirksamkeit wird erst je Port mit Build, Gate, Installation und Live-Beleg behauptet.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/dbrain-retrieval/src/release_port.rs:88 | Anknüpfung: aktueller Prosefilter und Veröffentlichungsrechte bleiben; fehlende kombinierte Gegenprobe aus 60dec8f5 sowie deduplizierter Caption-/Patchreview und generische Spielstilplanung
