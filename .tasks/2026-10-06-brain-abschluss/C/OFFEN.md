# Paket C: Entscheidungen für Paket A

## Ausführung durch C, Snapshot 06.10.2026 23:59:22 Uhr CEST

83/83 SHA-Zeilen entschieden. Nach dem ergänzenden Auftrag liegen nicht laufende Erhaltungsstände als 50 geprüfte annotierte Tags auf origin, statt als offene Remote-Branches. 59 der 65 Erhaltungs-SHAs sind über diese Tags erreichbar, sechs bleiben in geschützter A-/Kanon-/G5-/Q-/Z-Arbeit. 18 Verwerfungsentscheidungen wurden für den exakten alten Remote-SHA ausgeführt. Die Entscheidungen unten bleiben unverändert als historische Akte erhalten.

Tag-Namen, Ausführung, Rückholweg und vorher/nachher-Zahlen: [ARCHIV-TAGS.md](ARCHIV-TAGS.md). Kein pauschaler Main-Merge, keine Freigabe der geretteten Produktstände.

Stand: 06.10.2026, 22:14 Uhr CEST. Entscheidung bitte in der letzten Spalte als `übernehmen` oder `verwerfen` eintragen. Gleicher SHA bedeutet dieselbe Entscheidung für die im [Backup](BACKUP-SHAS.txt) aufgeführten lokalen und Remote-Aliase. Die achtstelligen SHA-Präfixe sind im Backup eindeutig und dort vollständig ausgeschrieben.

Basis: `origin/main` = `d6131cc52711a3e8b02d299704244f8d7dbdbce6`. Jeder Eintrag unten hat `merge-base --is-ancestor` Exit 1 und mindestens eine `+`-Zeile in `git cherry`. Das ist ein konservativer Restbestand, kein Beweis, dass die jeweilige Funktion auf main fehlt. Alte Dokumente und anders integrierte Änderungen können Patch-Unterschiede verursachen.

Diffspalte: Dateien / hinzugefügte Zeilen / entfernte Zeilen, gemessen mit `git diff --shortstat origin/main <sha>`. Das ist der vollständige Standvergleich, kein Maß für den übernehmbaren Patch. Historische Branches enthalten viele inzwischen entfernte Dateien; nicht pauschal mergen.

| Branch oder führender Alias | SHA | Letzter Commit | Inhalt | Diff F / + / - | Entscheidung A |
| --- | --- | --- | --- | --- | --- |
| fix/sheet-sync-secret-exec-20260924 | dcee4761 | 24.09.2026 08:14 | Systemd-Zugangsdaten für Sheet-Sync bevorzugen. | 924 / 6662 / 264262 | übernehmen: Credentialfix integriert, Caption-/Patchhistorienbestand nicht vollständig ersetzt |
| codex/brain-deploy-completion-20260918 | 458d56d0 | 18.09.2026 19:47 | Release-Artefakte und damalige Deploy-Blocker dokumentieren; PR #6. | 928 / 6773 / 264464 | übernehmen: eigenständige Evidenzspeicherung und Regressionen erhalten |
| codex/brain-luna-abo-20261003 | 9d236af5 | 03.10.2026 04:07 | Abo-Antwortgrößen am Connector ausrichten; zusätzlich ungesicherte Dateien vorhanden. | 186 / 2086 / 45627 | übernehmen: Abo-Connector und Größenprüfung fehlen; nicht eigenmächtig aktivieren |
| codex/brain-luna-dialogue-20261003 | 35c93de0 | 03.10.2026 03:23 | Zentralen Abo-Textdialog mit Eingabebudget anbinden. | 187 / 2274 / 45627 | übernehmen: Textdialogvertrag und Eingabebudget ohne Ersatzbeleg erhalten |
| codex/brain-pr61-pr9-integration-20261001 | 97515060 | 01.10.2026 22:47 | Schema-Übergabeblocker für PR #9 dokumentieren. | 190 / 643 / 57777 | übernehmen: zusätzliche Sicherheitsregressionen fehlen auf Main |
| codex/core-completion-20260925 (lokaler Stand) | 60dec8f5 | 25.09.2026 20:02 | Typisierte Domänenobjekte an Quellenrechte binden; Remote desselben Namens hat anderen, gemergten SHA. | 718 / 2329 / 148913 | übernehmen: Main-Filter stärker, zusätzliche ACL-Gegenprobe erhalten |
| codex/brain-release-20260918 | 9ead4617 | 18.09.2026 16:59 | Revisionssichere Verlaufsschnittstelle und Sync-Betrieb; PR #5. | 922 / 6101 / 264455 | übernehmen: gemeinsame Historie, Drift und Caption-Evidenz nicht vollständig ersetzt |
| codex/fix-c6-domain-kernel-wiring | 36577ba0 | 26.09.2026 05:45 | Domänenkernel-Anbindung und damalige Release-Prüfung dokumentieren. | 416 / 3251 / 98133 | verwerfen: Mainvorfahr als Basis, eigener Anteil ausschließlich historische Handoffnotizen |
| codex/fix-pr61-publication-20261001 | 8d355fce | 01.10.2026 03:15 | Unbekannte Ursprungsstände beim Veröffentlichen zurückweisen. | 281 / 4627 / 70934 | übernehmen: zusätzliche Gegenprobe für unbekannte Ursprungsverträge fehlt |
| codex/fix-pr61-publication-enforcement-20261001 | 27671409 | 01.10.2026 21:50 | Veröffentlichungsprüfungen mit damaligem main integrieren. | 254 / 3170 / 65330 | übernehmen: URL-/ACL-Regressionen fehlen trotz integrierter Produktteile |
| docs/pre-g5-closeout-20260929 | 9d040c15 | 29.09.2026 18:54 | Abschließende Migrationsbefunde dokumentieren. | 383 / 2849 / 86086 | verwerfen: Audit-/Messbelege auf Main identisch, Rest historische Statusnotizen |
| docs/reasoner-all-heroes | 8c98c63e | 16.09.2026 17:43 | Heldenübergreifende Reasoner-Recherche und Korrekturen dokumentieren. | 909 / 3202 / 263844 | übernehmen: API-Recherche und mechanische Gegenproben für Reasonerabschluss |
| feat/brain-fertig-q-20261003 | 96bf05c4 | 03.10.2026 21:02 | Paket Q am Haltepunkt sichern; Prozess im Worktree, nicht anfassen. | 209 / 7062 / 43275 | übernehmen: eigenständige Aufbewahrung, Feed, Audit und Messartefakte; geschützt |
| feat/brain-fertig-r-20261003 | e202beac | 03.10.2026 20:57 | Replay-Import und echte Decoder-Kompatibilität sichern. | 175 / 3322 / 45013 | übernehmen: Replay/Decoder eigenständig, zurückgestellt erhalten |
| feat/brain-fertig-z-20261003 | f1026afd | 03.10.2026 21:12 | Paket Z am angeordneten Haltepunkt sichern. | 156 / 3991 / 39348 | übernehmen: zusätzlicher Refresh-/Releasebestand am geschützten Haltepunkt |
| feat/brain-final-integration-20260921 | 22c8ebe0 | 21.09.2026 02:30 | Systemd-Zugangsdaten im nativen Rust-Startpfad bevorzugen. | 929 / 7418 / 182061 | übernehmen: Credential-Descriptor-Gegenproben und Auswertungshelfer erhalten |
| feat/brain-global-toml-20260920 | e4a9fe6c | 20.09.2026 02:29 | Typisierten TOML-Vertrag mit Prüfungen vorbereiten; zusätzlich ungesicherter Modellresolver vorhanden. | 899 / 3704 / 263040 | übernehmen: typisierter TOML-Vertrag; neuer Resolver-WIP separat schützen |
| feat/brain-release-completion-20260921, feat/brain-meta-publish-emojis | e752d251 | 21.09.2026 04:58 | Aktuelle Assets und verlustfreie Reasoner-Replays absichern; PR #9. | 941 / 8027 / 182073 | übernehmen: eingefrorene Reasoner-Replay-/Scratch-Prüfhelfer erhalten |
| feat/brain-runtime-release-20260921 | 457188a9 | 21.09.2026 02:41 | Nativen Runtime-Zugang isoliert freigeben. | 893 / 2934 / 262355 | übernehmen: spezifische Descriptor-/Systemd-Gegenproben fehlen |
| feat/brain-rust-cutover-20260919 (lokal), dependency/s00-planpaket-20260924 | 2734c2da | 24.09.2026 10:12 | Rust-Migrationsplan ablegen; kanonischer Checkout geschützt und ungesichert. | 872 / 6773 / 258715 | übernehmen: Kanon geschützt, Evidenzanteil nicht vollständig ersetzt |
| feat/brain-s01-inventory-20260924 | 76205b5f | 24.09.2026 11:35 | S01-Inventar und Übergabe dokumentieren. | 870 / 6799 / 258474 | übernehmen: ursprüngliche Fachartefakte/Evidenzlinie ohne vollständigen Ersatzbeleg |
| feat/brain-s05-domain-audit-20260924 | cdc3bb59 | 24.09.2026 12:26 | S05-Domänenaudit und blockierte Übergabe dokumentieren. | 827 / 2152 / 163760 | verwerfen: 10/10 Eigenanteilsdateien blobgleich auf Main |
| feat/brain-wiki-spielwissen-a | 116f643a | 03.10.2026 18:57 | Originaltreues Wiki-Inventar mit revisionsfestem Spool bauen. | 872 / 6817 / 254214 | übernehmen: Inventar weitgehend integriert, Patch-Evidenzbestand noch ohne Ersatzbeleg |
| feat/brain-wiki-spielwissen-b | 0aa0d9ee | 03.10.2026 14:45 | Inventargebundene Dateiobjekte im Extraktor lesen. | 870 / 6885 / 255896 | übernehmen: verlustfreie Parser/Prüfartefakte nicht vollständig ersetzt |
| feat/brain-wiki-spielwissen-c | c3b9cf59 | 03.10.2026 19:43 | C3-Prüfende und Vorrangregel sichern. | 933 / 8099 / 258715 | übernehmen: C2/C3-Referenzen und geerbten Evidenzbestand erhalten |
| feat/brain-wiki-spielwissen-c-integration (lokal) | f7a03f9a | 03.10.2026 14:54 | Inventargebundene Extraktion integrieren; umfangreiche ungesicherte Rust-Änderungen vorhanden. | 156 / 1214 / 37690 | übernehmen: Extraktions-/Revisionsbestand und separaten neuen WIP erhalten |
| feat/serverguide-brain-20261003 | 849926b2 | 03.10.2026 01:37 | Private Verarbeitung an freigegebenes Anbieterziel binden. | 175 / 3411 / 44909 | übernehmen: reicheren Guide-/Privacy-Kern zurückgestellt erhalten |
| fix/brain-readonly-query-cli | a7db4e2e | 22.09.2026 01:11 | Wissensabfragen in Lesemodus zwingen. | 892 / 2813 / 262370 | verwerfen: Main enthält Lesepool, Klassifikation und beide Regressionen, erweitert |
| fix/brain-spielwissen-c-liveprofile-20261005 | bb3ebf31 | 05.10.2026 09:28 | Fehlende Operatorfreigabe am gespeicherten Kandidaten belegen. | 6 / 298 / 214 | übernehmen: lokale Profil-/Operatorproben für aktuelle Diagnose |
| fix/discord-brain-antwort | 4eac7503 | 06.10.2026 22:06 | Release-Rechte und Originalbelege im Discord-Antwortpfad prüfen; heutige Arbeit, geschützt. | 25 / 2095 / 405 | übernehmen: heutige Release-Rechte-/Originalbelegarbeit geschützt |
| fix/forum-xml-sitemap-20261003 | 6946269f | 03.10.2026 20:29 | Gzip-Sitemaps begrenzt über bestehenden HTTP-Pfad lesen. | 149 / 484 / 42192 | übernehmen: eigenständige XML-/Gzip-Quelle, Forum zurückgestellt erhalten |
| fix/paket-b-timer-20260930 | fd6e1576 | 30.09.2026 17:01 | Vorübergehende API-Fehler mit Backoff wiederholen. | 825 / 2648 / 162116 | übernehmen: Retry-Anteil im aktuellen Build-Daten-Client fehlt |
| fix/pr61-remaining-review-20261001 | 45c88e38 | 01.10.2026 04:44 | Infisical-Konfigurationspfad im MCP korrigieren. | 254 / 1112 / 66186 | verwerfen: Schutz-/Pfadkorrekturen identisch oder stärker auf Main, Rest patchgleich |
| fix/reasoner-build-without-live-schema-20260925 | 4269337a | 30.09.2026 17:07 | Lokale Reasoner-Arbeit vor früherer Bereinigung sichern. | 838 / 2515 / 164235 | verwerfen: beide Eigenanteilsdateien engine.rs/sync.rs blobgleich auf Main |
| fix/reasoner-mechanics-completion | 3e7864a5 | 30.09.2026 17:07 | Lokale Mechanik-Arbeit vor früherer Bereinigung sichern. | 848 / 2330 / 167053 | übernehmen: zusätzliche generische Spielstilplanung fehlt auf Main |
| integrate/serverguide-deploy-20261003 | 1d820bc2 | 03.10.2026 20:23 | Guide-Aktionspfad und Prüfungen integrieren. | 181 / 5291 / 44908 | übernehmen: erweiterte Guide-Aktion/Feedback/Einladungen erhalten |
| integration/final-local-20260926 | 870c7a15 | 26.09.2026 09:23 | Geschlossene Idle-Verbindungen verwerfen; Commit nennt sich ersetzt, Beleg noch nicht geprüft. | 413 / 3227 / 96932 | übernehmen: Umgang mit geschlossenen Idle-Verbindungen ohne Ersatzbeleg |
| integration/patch-analysis-evidence-20261001 | 49151073 | 01.10.2026 20:53 | Patchanalyse-Evidenz integrieren und damaligen Gate-Befund dokumentieren. | 281 / 6289 / 66853 | übernehmen: zeitgebundene Caption-/Patch-Review-Funktionen erhalten |
| integration/technical-closeout-20260929 | 20b279a6 | 30.09.2026 18:30 | Clippy-, Unit- und Postgres-Prüfungen dokumentieren. | 355 / 4741 / 80611 | übernehmen: eingefrorener G5-Nachweis, Worktree ausdrücklich gesperrt |
| fix/patch-insights-evidence-20260918 | 089be38c | 18.09.2026 10:28 | MCP-Start und gemeinsame Abfragen gegen isoliertes Postgres prüfen; PR #3. | 905 / 4331 / 264141 | übernehmen: PR3-Evidenz-/Entwurfsvertrag und Prozessfixtures erhalten |
| codex/patch-understanding-evidence-20260918 | 9efeb1e4 | 18.09.2026 13:20 | Evidenzhistorie, Zeitsegmente und Vertrauensprüfungen; PR #4. | 913 / 4759 / 264440 | übernehmen: Zeitsegmente, Wissenszeit und Review-/Storyboard-Verträge erhalten |
| luna/abschluss-brain-pr4-integrate-pr61-20261001 | aa2a051e | 01.10.2026 21:20 | PR4/PR61-Integration mit Gate-Übergabe dokumentieren. | 284 / 5118 / 69247 | übernehmen: zusätzliche ID-/Vertrauens-/Evidenzerhaltungsprüfungen |
| codex/fix-c10-real-replay-validation | 919c790f | 26.09.2026 03:34 | Begrenzte Replay-Prüfung und offene Freigabepunkte dokumentieren. | 516 / 5383 / 114766 | übernehmen: begrenzte Replay-Abnahme zurückgestellt erhalten |
| luna/abschluss-brain-pr9-20261001 | 7deebcb7 | 01.10.2026 22:27 | PR9-Patchhistorie übergeben. | 943 / 8256 / 182080 | übernehmen: Prüfwerkzeuge/eingefrorene Auswertungen ohne Main-Ersatzbeleg |
| luna/finish-brain-global-toml-20261001 | 40b43ac8 | 01.10.2026 23:04 | Wiki-TOML-Pfadpriorität klären. | 818 / 3480 / 160724 | übernehmen: gemeinsamen TOML-Vertrag und Verbraucher erhalten |
| codex/fix-pr61-build-publish-url-20261001 | 0e0f5137 | 01.10.2026 02:59 | Host und Herkunft der Build-Veröffentlichungs-URL prüfen. | 280 / 3746 / 71189 | verwerfen: Main-Publisher enthält URL-/Herkunftsschutz plus Redirect-/Proxysperre |
| feat/brain-rust-cutover-20260919 (Remote) | c8ad3ef6 | 24.09.2026 10:14 | Abweichenden Rust-Cutover-Quellstand erhalten. | 914 / 7489 / 274367 | übernehmen: historische Rust-MCP-/Evidenzverträge ohne vollständigen Ersatzbeleg |
| migration/07-provider-jev-20260924 | e660a8dc | 24.09.2026 12:25 | Provider-Verträge und Jev-Fixtures vorbereiten. | 777 / 2152 / 161612 | verwerfen: 60/60 Eigenanteilsdateien blobgleich auf Main |
| migration/s03-storage-preparation-20260924 | feda2355 | 24.09.2026 12:19 | S03-Speicher-Vorbereitung und Übergabe dokumentieren. | 834 / 2152 / 164174 | verwerfen: 3/3 Vorbereitungsartefakte blobgleich auf Main |
| migration/s04-ingestion-preparation-20260924 | 01f307c8 | 24.09.2026 12:17 | S04-Ingestion-Vorbereitung und Übergabe dokumentieren. | 832 / 2152 / 163977 | verwerfen: 5/5 Eigenanteilsdateien blobgleich auf Main |
| migration/s06-retrieval-preparation-20260924 | 176b78d3 | 24.09.2026 12:21 | S06-Retrieval-Vorbereitung und Übergabe dokumentieren. | 831 / 2152 / 163912 | verwerfen: 6/6 Eigenanteilsdateien blobgleich auf Main |
| migration/s08-answer-kernel-20260924 | c99d9994 | 24.09.2026 12:21 | S08-Antwortkernel-Vorbereitung dokumentieren. | 832 / 2152 / 163956 | verwerfen: 5/5 Vorbereitungsartefakte blobgleich auf Main |
| migration/s10-quality-performance-20260924 | 5676167e | 24.09.2026 16:22 | Qualitätsbasis und Performance-Prüfungen dokumentieren. | 815 / 2625 / 161411 | verwerfen: 25/27 Blobs gleich, Main erweitert Beobachtungsfensterprüfung/Regression |
| migration/s11-cutover-preparation-20260924 | e2377f76 | 24.09.2026 22:28 | Prozessgruppen und geladene Laufzeiten beim Cutover prüfen. | 824 / 2639 / 161836 | verwerfen: 16/18 Blobs gleich, Main erweitert Runtime-Erkennung/Regressionen |
| migration/s14-replay-preparation-20260924 | afdffba1 | 24.09.2026 23:18 | Replay-Vorbereitung und damalige Umsetzung dokumentieren. | 829 / 2624 / 162640 | verwerfen: gesamter Bestand erhalten, Main ergänzt nur historische Vorspänne |
| review/brain-s01-s09-20260924 | 2c470864 | 24.09.2026 15:40 | S05 bis S09 integrieren. | 687 / 3097 / 150180 | verwerfen: Integrationskern auf Main erhalten/erweitert, keine fehlende Eigenanteilsdatei |
| review/pre-g5-core-abnahme-20260929 | 9070ba94 | 30.09.2026 17:47 | Auto-Deref-Korrekturen statisch abnehmen. | 392 / 3779 / 85249 | übernehmen: explizit gesperrten unabhängigen G5-Prüfstand erhalten |
| sol/c9-brain/7bf0e0375ee34a00 | 357548d7 | 03.10.2026 03:00 | C9-Releasebindungen im Scratch-Serve prüfen. | 171 / 2029 / 43979 | übernehmen: TOML-Anschluss und Scratch-Serve-Regressionen fehlen |
| codex/brain-maintenance-storage-20261002 | e26f673b | 02.10.2026 02:35 | Ursprünglichen Queue-Auftrag bei Quellenfreigabe erhalten. | 185 / 439 / 56475 | verwerfen: Migrationen identisch, Main erhält Enqueue-Auftrag und erweitert Leases |
| feat/brain-wiki-spielwissen-c-integration (Remote) | b1b92418 | 03.10.2026 06:58 | Rust-Import und Faktenlesepfad von C sichern. | 159 / 1404 / 42092 | verwerfen: Import/Faktenlesepfad auf Main weiterentwickelt; lokaler neuer WIP ausgeschlossen |
| luna/brain-codex-brain-deploy-completion-20260918-458d56d | 3d9098c6 | 01.10.2026 19:23 | Deploy-Abschluss mit damaligem main-Gate-Befund dokumentieren. | 929 / 6838 / 264464 | übernehmen: konkrete Evidenzmigrationen und Regressionen ohne Ersatzbeleg |
| luna/brain-codex-brain-release-20260918-9ead461 | 79ec3857 | 01.10.2026 19:15 | PR9-Abstammung und Release-Haltepunkt dokumentieren. | 923 / 6142 / 264455 | übernehmen: Historien-/Zeitbelege im ursprünglichen Eigenanteil erhalten |

## Urteil von A, 06.10.2026

62/62 alte SHA-Zeilen bewertet: 44 `übernehmen`, 18 `verwerfen`. Native read-only Bewertung `wf_21fbe619-94c`, drei Worker mit 21/21/20 Positionen; vollständige Rückgaben und konkrete Main-/Blob-/Ancestrybelege unter der Paket-A-Session in `tasks/wusiurcuz.output`. `übernehmen` umfasst gezieltes Wiederverwenden oder bewusstes Erhalten, keinen pauschalen Merge, keine Aktivierung zurückgestellter Funktionen und keinen eigenmächtigen Modellwechsel. Die Entscheidung gilt exakt zum Tabellen-SHA.

Vor jeder Entfernung ist der aktuelle SHA samt Ancestry-Exit, WIP und ignorierten wertvollen Artefakten erneut zu prüfen. Nicht-Vorfahren nicht als gemergt darstellen. Bestehende SHA-/Remote-Sicherungen erhalten. Explizit geschützte G5-, Replay-, laufende Q/Z-, Kanon- und heutige Paket-A/B-Arbeit bleibt unangetastet. Insbesondere `.core-test-pg/`, `.core-test-logs/` und die drei Abo-Archive `FIFO-SNAPSHOT.tar`, `FIXRUNDE-SNAPSHOT.tar`, `FIXRUNDE-2-SNAPSHOT.tar` sind durch kein altes SHA-Urteil zur Löschung freigegeben. C führt die Mechanik aus, A hat nichts gelöscht.

## Zusätzliche ungesicherte Stände

Die Worktree-Tabelle in `INVENTAR.md` kennzeichnet sie. Code und Berichte werden vor einer möglichen Entfernung separat als WIP gesichert. Kanonischer Checkout, laufende Arbeit und Pakete A/B bleiben unangetastet. Unversionierte Build-Verzeichnisse werden nicht als Quellcode eingecheckt. Entscheidungen über einen alten SHA gelten nicht automatisch für später gesicherte WIP-Commits.

## Neu gesicherte WIP-Stände, 06.10.2026 22:48 Uhr CEST

Diese 21 Stände waren vorher uncommittiert. Vollständige SHAs, lokale Branches, Pfade und bestätigte Remote-Aliase stehen in `BACKUP-WIP-SHAS.txt`. Sie haben Vorrang vor einer Entscheidung über den alten Eltern-SHA. Die WIPs sind Sicherungen, keine geprüften Produktfixes. Bevor ein wiederverwendeter Branch gelöscht wird, muss die Entscheidung ausdrücklich den aktuellen WIP-SHA betreffen.

| Remote-Sicherung | SHA | Letzter Commit, CEST | Inhalt | Diff F / + / - | Entscheidung A |
| --- | --- | --- | --- | --- | --- |
| feat/brain-meta-publish-emojis | 36588add | 06.10.2026 22:35 | Bestehende Composer- und Planner-Änderungen sichern. | 942 / 8043 / 182148 | übernehmen: neue Composer-/Planneränderungen separat erhalten, noch nicht produktgeprüft |
| fix/paket-b-timer-20260930 | 8318690c | 06.10.2026 22:36 | Bestehende Batch-, Timer- und Dienstdateien sichern. | 827 / 2723 / 162116 | übernehmen: neuer Betriebsbestand für Tageslauf, keine pauschale Übernahme |
| fix/brain-rust-cutover-gate-20260930 | 3d74ae54 | 06.10.2026 22:37 | Bestehende Rust-Patchanalyse und SQL-Prüffixtures sichern. | 875 / 6903 / 258720 | übernehmen: Patchanalyse/SQL-Gegenproben erhalten, Ersatz nicht belegt |
| codex/core-completion-20260925 | e2bd0b22 | 06.10.2026 22:37 | Offenen lokalen Rechtefix mit vorhandenen Kernübergaben sichern. | 720 / 2616 / 148913 | übernehmen: neuen Rechtefix samt Kernübergaben separat erhalten |
| feat/brain-final-integration-20260921 | 7e4d5e6c | 06.10.2026 22:37 | Bereits vorgemerkte Aktenlöschungen und vorhandenen Hinweis sichern. | 933 / 7441 / 268017 | übernehmen: neuer Sicherungsstand, Löschungen nicht ungeprüft nach Main tragen |
| backup/wip-functional-closeout-20261006 | 0e91b977 | 06.10.2026 22:38 | Bestehenden Postgres-Lesepfad und Deadlinetests sichern. | 285 / 3694 / 72333 | übernehmen: Lesepfad/Deadlinetests ohne Ersatzbeleg erhalten |
| feat/brain-global-toml-20260920 | 38e4e559 | 06.10.2026 22:39 | Bestehenden TOML-Vertrag und zentralen Modellresolver sichern. | 906 / 4765 / 263270 | übernehmen: gemeinsamen TOML-/Resolverbestand erhalten, kein Modellwechsel |
| codex/brain-luna-abo-20261003 | ee85ff14 | 06.10.2026 22:40 | 80 vorhandene Abo-Prüfbelege sichern; drei Archive bleiben lokal. | 266 / 11847 / 45627 | übernehmen: 80 Abo-Belege und separat geschützte lokale Archive erhalten |
| integration/pr61-luna-20261001 | 6c6dd8ba | 06.10.2026 22:40 | Vorhandene Veröffentlichungsprüfungen sichern. | 265 / 3291 / 67756 | übernehmen: neue Publikationsprüfungen ohne vollständigen Ersatzbeleg erhalten |
| integration/brain-v1-closeout-20261001 | 4dbf3940 | 06.10.2026 22:41 | Zweiten vorhandenen Veröffentlichungsprüfstand sichern. | 265 / 3291 / 67756 | übernehmen: separaten Publikationsprüfstand erhalten, Duplikat nicht ungeprüft löschen |
| backup/wip-pr61-review-20261006 | 28ac5c6f | 06.10.2026 22:42 | Detached-Prüfstand mit Deadlinetests und Aufrufskripten sichern. | 291 / 4038 / 74915 | übernehmen: Detached-Deadlinetests/Aufrufverträge erhalten |
| backup/wip-pr61-scratch-20261006 | 7d60f287 | 06.10.2026 22:42 | Drei Detached-Scratch-Prüfdateien sichern. | 290 / 3996 / 74779 | übernehmen: neue Scratch-Belege ohne Ersatzbeleg erhalten |
| feat/brain-release-completion-20260921 | 79f5a020 | 06.10.2026 22:43 | Vorhandenen Replay-Prüfaufruf sichern; PR #9 jetzt auf diesem WIP. | 942 / 8039 / 182073 | übernehmen: eigenständigen Replay-Prüfaufruf zurückgestellt erhalten |
| luna/finish-brain-rust-cutover-20261001 | f34cd867 | 06.10.2026 22:43 | Bestehenden nativen MCP-Einstieg und Readme sichern. | 806 / 2156 / 160420 | übernehmen: neuen MCP-Einstieg samt Vertrag erhalten |
| codex/luna-native/brain-pr4-evidence-20261001 | 1f4bb874 | 06.10.2026 22:44 | Bestehenden YouTube-Testhelfer sichern. | 273 / 3406 / 68897 | übernehmen: zusätzlichen YouTube-Prüfhelfer erhalten |
| feat/brain-wiki-spielwissen-c | 209f1fbb | 06.10.2026 22:45 | Vorhandene Wiki-C-Verträge und Übergaben sichern. | 936 / 8191 / 258715 | übernehmen: neue Wiki-C-Vertrags-/Übergabebelege erhalten |
| feat/brain-wiki-spielwissen-a | 80d1ca81 | 06.10.2026 22:45 | 87 Wiki-A-Berichte und Statusmeldungen sichern; Daten bleiben lokal. | 959 / 10949 / 254214 | übernehmen: neue Wiki-A-Belege erhalten, Daten separat schützen |
| feat/brain-wiki-spielwissen-b | b9be82ce | 06.10.2026 22:46 | 71 Wiki-B-Berichte und Statusmeldungen sichern. | 941 / 8337 / 255896 | übernehmen: neue Wiki-B-Belege erhalten |
| feat/brain-wiki-spielwissen-d | 60610fce | 06.10.2026 22:46 | 57 Wiki-D-Berichte und Statusmeldungen sichern. | 216 / 1404 / 44934 | übernehmen: neue Wiki-D-Belege erhalten |
| worktree-wiki-spielwissen-status-s | 2f35c6b4 | 06.10.2026 22:47 | Drei vorhandene Statusübergaben sichern. | 162 / 696 / 44934 | übernehmen: neue Statusübergaben erhalten |
| backup/wip-wiki-c-integration-20261006 | 74f6500e | 06.10.2026 22:34 | 26 vorhandene Rust-Dateien für Wiki-Import und Wissensprojektion sichern; besonders für A relevant. | 157 / 3106 / 35263 | übernehmen: 26 Rust-Dateien selektiv für Wiki-/Profilanschluss prüfen, nicht verlieren |

Die 21 neuen WIPs sind konservative Erhaltungsentscheidungen auf Grundlage der C-Sicherungsakte, keine zusätzliche Behauptung, ihr kompletter Produktdiff sei geprüft oder müsse gemergt werden. Kein WIP wird durch die 18 alten Verwerfungsentscheidungen gelöscht.

## Entscheidungsfenster

Erste Bereitstellung: 06.10.2026 22:18 Uhr CEST. Kontrolle alle 30 Minuten bis 07.10.2026 01:18 Uhr CEST. Ohne Entscheidung bleibt der Eintrag erhalten. Paket C meldet den Rest anschließend als offen.
