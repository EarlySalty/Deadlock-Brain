# Deadlock Brain: direkter Fehlerabschluss und geprüfte Betriebsgrenzen

Stand: 30. September 2026. Die hier beschriebenen Änderungen und Prüfungen wurden direkt ausgeführt. Es wurde kein lokaler Agent beauftragt.

## Ergebnis

Zwei konkrete Betriebsfehler sind korrigiert: falsche Zugangsdaten-Referenzen im Importer und unfaire Vergabe der Verbindungen im PostgreSQL-Leserpool. Der bereits vorbereitete lesende Snapshot-Modus wurde integriert und erstmals in diesem Lauf gegen das echte Archiv über den bestehenden Infisical-Weg erfolgreich ausgeführt.

Der neue Kern ist trotzdem **noch nicht produktiv aktiviert**. Die produktive Core-Datenbank enthält weiterhin keine Source-Heads und keinen CorpusRelease. Ein erfolgreicher echter öffentlicher Consumer-Aufruf ist nicht nachgewiesen. Testantworten eines Provider-Stubs sind kein Nachweis einer tatsächlichen Modellantwort.

Produktcommit: `1ef08a4f03304db72647c3d1a00cebfd2688dec7`.
Branch: `codex/brain-functional-closeout-20260930`.
Basis: `c5951b610aa2545d2c0b43b33b5fe1906198b292`.
Arbeitsbaum: `/home/nathanael/.worktrees/brain-functional-closeout-20260930`.

## Korrigierte Fehler

### Zugangsdaten entsprachen nicht dem vorhandenen Runtime-Bestand

Die produktive Importerbindung und beide JSON-Vorlagen erwarteten `BRAIN_LEGACY_READ_AUTH` und `BRAIN_TARGET_INGEST_AUTH`. Diese Referenzen fehlen im über den vorhandenen Secret-Exec geladenen Bestand. Dagegen sind `BRAIN_PG_READONLY_PASSWORD`, `BRAIN_PG_INGEST_PASSWORD` und `BRAIN_PG_SERVICE_PASSWORD` vorhanden. Geprüft wurden ausschließlich Vorhandensein beziehungsweise Fehlen, keine Werte ausgegeben.

Importer und Vorlagen verwenden nun die bereits vorhandenen getrennten Lese- und Ingestreferenzen. Keine neuen Secrets, keine Passwortänderung, kein Alias-Fallback und keine Erweiterung von Datenbankrechten. Ein Regressionstest bindet die Produktionskonstanten an beide tatsächlich versionierten Vorlagen.

Der erste Einmallauf scheiterte zusätzlich am nicht regulären geerbten Credential-Dateideskriptor. Der anschließende Aufruf verwendet den bereits vorhandenen systemd-Mechanismus `LoadCredential` und den installierten Rust-Secret-Exec mit ausdrücklich übergebener Konfiguration. Danach ließ sich die falsche Referenz reproduzieren und mit der Korrektur der echte lesende Archivlauf erfolgreich abschließen.

### Einzelne Anfragen verhungerten im Verbindungspool

Ein erster Prozesslauf bestand alle 1.800 Lastanfragen. Die Wiederholung vor der Pool-Korrektur meldete bei 16 parallelen Anfragen jedoch 592 Antworten und acht `unavailable` bei 600 Anfragen. Die Poolstatistik zeigte acht abgelaufene Wartezeiten bei weiterhin höchstens vier Verbindungen. Der Fehler war damit intermittierend, nicht in jedem Lauf reproduzierbar.

Im bestehenden Pool konnten neue Anfragen eine gerade zurückgegebene Verbindung vor bereits wartenden Threads übernehmen. Außerdem wurde nach dem Aufwachen der Mutex vor dem nächsten Vergabeversuch erneut freigegeben. Die Korrektur ergänzt eine FIFO-Warteschlange innerhalb desselben Pools. Jeder wartende Thread besitzt ein eigenes Signal; Rückgabe, erfolgreiche Vergabe, Verbindungsfehler und Wartezeitüberschreitung wecken den vordersten verbleibenden Thread. Verbindungsaufbauten reservieren weiterhin unter demselben Mutex ihre Kapazität. Die ursprüngliche Wartefrist wird nicht bei jedem Aufwachen erneuert.

Verbindungslimit vier, Pool-Wartezeit 150 Millisekunden im Prozessprüflauf, Providerbudgets und Abnahmeschwellen blieben unverändert. Drei zusätzliche Tests prüfen die Vergabereihenfolge, abgelaufene Wartende und das Entfernen unbekannter Wartender. Zwei anschließende vollständige Prozessläufe bestanden jeweils 1.800 von 1.800 Lastanfragen. Das ist ein Nachweis dieser beiden Läufe, keine Garantie für jede beliebige Hostlast.

### Lesender Snapshot-Modus integriert

Der vorhandene, noch uncommittete Entwurf aus dem separaten Arbeitsbaum `brain-g5-readonly-snapshot-20260930` wurde gelesen und in den eigenen Arbeitsbaum übernommen. Der fremde Arbeitsbaum wurde nicht verändert. Die Erweiterung verwendet den vorhandenen Reader, dieselben Projektionen und denselben kanonischen Snapshot-Hash wie der Import.

`--observe-snapshot --config <Datei>` öffnet ausschließlich die Archiv-Leseverbindung, prüft die tatsächliche Rollen-, Datenbank- und Schemaidentität und liest in `REPEATABLE READ READ ONLY` mit anschließendem `ROLLBACK`. Es gibt keine Zielverbindung und keinen Schreibzugriff auf die in der Importkonfiguration genannte Reportdatei. Ein neuer Beobachtungszeitpunkt wird erzeugt; historische Labels werden nicht als frische Beobachtung ausgegeben.

Der bestehende echte PostgreSQL-Cutover-Test prüft zusätzlich unveränderte Zieltabellen, fehlenden Bedarf am Zielcredential, Übereinstimmung der Projektion und Fingerabdrücke sowie gesäuberte Ausgaben. Die Beobachtung erzeugt keine erfundene Nutzungsfreigabe und keine bestätigten Aktiv-, Widerrufs- oder Löschlisten.

Die weiteren Änderungen in `brain-legacy-import/src/cutover.rs`, `src/lib.rs` und `src/tests.rs` beseitigen vorgefundenen Rustfmt-Drift. Keine fachliche Vertragsänderung in diesen drei Dateien.

## Durchgeführte Verifikation

Toolchain: Rust/Cargo 1.97.1. Bestehender Targetcache: `/home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target`. Cargo-Aufrufe mit `--locked --offline --jobs 1`. Kein neuer Buildcache, kein Release-Build und kein abgeschalteter Produktivprozess.

| Prüfung | Tatsächliches Ergebnis |
| --- | --- |
| Workspace `cargo fmt --all -- --check` | Exit 0 |
| Clippy, `brain-legacy-import`, `brain-storage`, `brain-serve`, alle Targets, `-D warnings` | Exit 0 |
| Normale Tests `brain-legacy-import` und `brain-storage` | 39 bestanden, 0 Fehler, zunächst 5 PostgreSQL-Fälle ignoriert |
| Bestehender Serve-/Import-/SCRAM-Harness, zweimal vollständig | Je 7 Fälle bestanden; insgesamt 14 erfolgreiche Testausführungen |
| Lastteil dieser beiden Durchläufe | 3.600 Antworten, 0 Clientfehler, keine fehlgeschlagenen Lastantworten |
| Bestehender Upgrade-/Restore-/Minimalrollen-Harness | 1 bestanden, Exit 0 |
| Bestehender PostgreSQL-Kernvertrag | 1 bestanden, Exit 0 |
| `git diff --check` | Exit 0 |

Damit 55 erfolgreiche Testausführungen am finalen Produktstand. Die sieben Harnessfälle sind darin zweimal enthalten; dies sind nicht 55 unterschiedliche Tests. Die fünf zunächst ignorierten Fälle der beiden betroffenen Pakete wurden anschließend in ihren isolierten PostgreSQL-Runnern tatsächlich ausgeführt. Die Gesamt-Workspace-Suite wurde in dieser Session nicht erneut vollständig ausgeführt.

Der Upgradefall prüfte den echten Weg v1 nach v2, stabile IDs/Hashes/Provenienz/ACLs/Tombstones/Releasepins, minimale Dienstrechte, Writer-Fencing sowie `pg_dump`/`pg_restore` in eine frische Testdatenbank. Diese Scratch-Restoreprüfung ist kein aktuelles Produktionsbackup.

### Wiederholte Lastprüfung

| Durchlauf | Parallele Anfragen | Ergebnis | Gesamtdauer für 600 Anfragen |
| --- | ---: | ---: | ---: |
| 1 | 8 | 600 beantwortet | 3.371 ms |
| 1 | 16 | 600 beantwortet | 3.396 ms |
| 1 | 32 | 600 beantwortet | 2.728 ms |
| 2 | 8 | 600 beantwortet | 4.384 ms |
| 2 | 16 | 600 beantwortet | 3.668 ms |
| 2 | 32 | 600 beantwortet | 1.733 ms |

Das beobachtete Poolmaximum war in allen sechs Lastabschnitten vier. Die abschließende Poolstatistik enthält je Durchlauf eine absichtlich ausgelöste Wartezeitüberschreitung aus dem gesonderten Überlast-Negativtest. Diese ist nicht mit den zuvor fehlerfreien Lastabschnitten gleichzusetzen. Alle Prozess- und Compilerläufe sind beendet; erfolgreiche Runner haben ihren jeweiligen Wegwerfcluster gestoppt und entfernt.

## Echter Archivlauf

Der lesende Einmallauf über den installierten Secret-Exec und systemd Runtime Credential endete mit Exit 0. Er projizierte 905 Entitätsdokumente und 348 Patchdokumente. Gelesene Tabellen: 905 Entitäten, 3.778 Aliase, 32.821 Patchereignisse und 32.821 Anreicherungen.

Snapshotlabel: `observed-readonly-1790791596-304636766`.
Snapshot-SHA-256: `01cb83d02380342ca56d1258d74619428f94c1516287c534b711c96bc7802d75`.
Schema-SHA-256: `5fe2c40427d3dd57f3d07b714936a633e372ae0a22b6cee4a237a58366aa3a4f`.

Eine nachfolgende unabhängige READ-ONLY-Transaktion bestätigte weiterhin **0 Source-Heads und 0 CorpusReleases** in der produktiven Core-Datenbank. Kein Produktionsimport wurde durchgeführt. Die vollständige Beobachtung mit logischen IDs bleibt mit Modus 0600 außerhalb von Git; Rohtexte und Zugangsdaten wurden nicht veröffentlicht.

## Nachweisdateien

Alle folgenden Dateien liegen unter `/home/nathanael/.local/state/`. Die erfolgreichen Abschlusslogs enthalten jeweils den tatsächlichen Marker `VERIFICATION_EXIT=0`.

| Datei | SHA-256 |
| --- | --- |
| `brain-direct-final-v2-20260930.log` (Fehler vor Pool-Korrektur) | `2224f7b8cc86b651a6c984f8b266b89a3923048499875aa3a1445f4b0f69ad74` |
| `brain-direct-final-v3-20260930.log` | `3a770e6a6ec7f32916078de3dec38c2ba38899d589f61dc15127ba46379e62f6` |
| `brain-direct-upgrade-20260930.log` | `fe1342f82d1c53f48c64acb1507142d0736a9ffa36a933e3e4305d3ab6f1af30` |
| `brain-direct-core-pg-20260930.log` | `1472a3ea39eeac3bb3dc1d2e575ef84dbe07b5515bf424a4f9a8c6e81f79c1bb` |
| `brain-direct-snapshot-runtime-v2-20260930.json` (interne Metadaten, nicht in Git) | `694eb5682bd9584c2d31ba06225cda1abde579ded27ee887cda627e5002a3ac3` |

## Nicht erledigt und nicht als erledigt ausgegeben

Die tatsächlich abgefragte typed Serve-Unit ist nicht installiert. Die vorhandene Brain-Webseite ist ein anderer Dienst und kein Beleg für `/v1/answer`. Die Core-Datenbank enthält noch keinen freigegebenen Wissensrelease. Main wurde nicht geändert, kein Produktivdienst wurde von dieser Session neu gestartet und kein Consumer umgeschaltet. Es gab keine tatsächliche Modellantwort und keine gesendete Twitch- oder Discord-Nachricht.

Die vorhandenen Quellenpolicies erlauben die interne Übernahme, aber keine Providerweitergabe beziehungsweise Veröffentlichung der importierten Inhalte. Patchdokumente bleiben privat unter `brain.legacy.review`. Der geprüfte Twitch-Adapter verwendet `Explain`; ein providerfreier `Fact`-Test wäre kein Ersatz für diesen Consumer. Weder private Scopes noch Egressflags wurden zum Erzeugen einer grünen Antwort ausgeweitet.

Für den tatsächlichen Abschluss fehlen deshalb konkret: eine aus vorhandenen Entscheidungsbelegen und tatsächlichen Zustandsinventaren gebundene Produktionsübernahme; der daraus erzeugte CorpusRelease; die revisionsgebundene Serve-Installation mit passender Dienstidentität und Rückweg; und ein erfolgreicher Aufruf des bestehenden Consumers mit dafür erlaubter Quelle über den vorgesehenen Provider. Ein Snapshotfingerabdruck allein ersetzt weder diese Nutzungsbindung noch aktive/Widerrufs-/Tombstone-Listen. Es wurden keine solchen Belege erfunden.

Diese verbleibenden Punkte stehen bereits in den bestehenden G5-Verträgen. Die interne Übernahme wird nicht erneut pauschal zur Genehmigung gestellt. Der vorliegende technische Fehlerabschluss und der echte Lesezugriff sind jedoch kein vollständiger G5-Cutover. Eine unabhängige Merge-Gate-Abnahme wurde in dieser Session nicht ausgeführt; der getestete Produktcommit ist als Feature-Branch gepusht, nicht nach Main gemergt.
