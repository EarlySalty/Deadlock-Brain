# Frisches Legacy-Archiv

Eigenständige Rust-Crate außerhalb des Brain-Workspaces. Der Erstimport wird nicht erneut ausgeführt. Die feste PostgreSQL-Werkzeugkette und exportierte Snapshotbindung verwenden den vorhandenen Ansatz aus `legacy-import.sh` und `backup.sh`. Der Kern wird weder zurückgespielt noch geändert.

## Schnittstellen

- `inspect_plan(plan, source, target)` prüft Quelle und Archiv ausschließlich lesend, einschließlich der ausdrücklich erlaubten zusätzlichen Tabellen.
- `capture` hält eine `REPEATABLE READ READ ONLY`-Quelltransaktion offen. Beide `pg_dump`-Aufrufe importieren deren exportierten Snapshot. Tabellenzahlen und vollständige SHA256-Fingerprints stammen aus derselben Transaktion. Zeilen werden nicht protokolliert.
- `capture_and_stage` baut eine eindeutige Generation `brain_legacy_g<32 Hexzeichen>` aus der geprüften Quellstruktur und überträgt die Tabellen per binärem COPY aus derselben Transaktion. Vor Commit werden sämtliche Tabellen, Sequenzen und Archivberechtigungen verglichen. Das bisherige Archiv bleibt erhalten.
- `validate_generation` bindet die tatsächlichen Staging-OIDs, Inhalte und Rechte an das Manifest und liefert eine nicht frei konstruierbare `ValidatedGeneration`.
- `activate_generation` prüft die Artefakte frisch, erwirbt `ARCHIVE_LOCK`, sperrt Alt-/Neutabellen exklusiv und vergleicht beide Generationen nochmals. Beide Schemaumbenennungen laufen in einer Transaktion. Der Vorgänger bleibt als `brain_legacy_p<Generation>` erhalten. Die Aktivierungsabsicht wird vor Commit dauerhaft abgelegt.

CLI mit absoluten privaten Pfaden:

```text
brain-legacy-refresh plan <Plan>
brain-legacy-refresh capture <Plan> <Artefaktverzeichnis>
brain-legacy-refresh stage <Plan> <Artefaktverzeichnis>
brain-legacy-refresh validate <Artefaktverzeichnis>
brain-legacy-refresh activate <Artefaktverzeichnis> <erwarteter Archiv-SHA256>
```

Die CLI startet weder eine Shell noch `sudo`. Sie erlaubt keine frei angegebenen Programme oder SQL-Befehle und verweigert root-Ausführung. `stage` und `activate` sind schreibende Aufrufe, die hier ausschließlich gegen eigene isolierte PostgreSQL-Instanzen geprüft werden.

## Unterstützte Struktur und verbleibende Betriebsgrenze

Der Adapter unterstützt gewöhnliche permanente Heap-Tabellen mit den tatsächlich beobachteten eingebauten Typen: `smallint`, `integer`, `bigint`, `text`, `boolean`, `double precision`, `jsonb`, `timestamptz`, `uuid` sowie Integer- und Textarrays. Er rekonstruiert Primär-/Unique-Schlüssel, Btree-Indizes mit Sortieroptionen, die beobachteten `COALESCE`-Indexausdrücke, den `IS NOT NULL`-Partialindex, lokale Fremdschlüssel, den beobachteten Text-Enum-Check sowie eng geprüfte Defaults. Erlaubt sind skalare Zahlen/Booleanwerte, typisierte Literale, `pg_catalog.now()`, `pg_catalog.gen_random_uuid()` und an übernommene Sequenzen gebundenes `nextval`.

Spaltengebundene Serial- und Identitysequenzen erhalten Parameter, Stand, Eigentümer und bisherige reine Leserechte. Neue Generationen haben eigene Sequenz-OIDs. Defaults und Fremdschlüssel verweisen auf deren Objekte; beim Schemawechsel folgen die OID-Bindungen dem neuen Archiv. Schemaähnlicher Text in Literalen wird nicht umgeschrieben. Identitywerte werden per COPY explizit übertragen.

PostgreSQL-Sequenzstände sind nicht MVCC-snapshotgebunden. Ein exportierter Snapshot friert `last_value` und `is_called` nicht ein. Zwei gleiche Beobachtungen beweisen keinen unveränderten Zustand während eines parallelen `nextval` oder `setval`. Deshalb verweigert `capture` für die produktive Quelle jede Aufnahme mit Sequenzen bereits vor Dump oder Zielschreibzugriff. Isolierte Testquellen dürfen diese Mechanik bei kontrolliertem Stillstand nachweisen. Für einen verlustfreien Produktivimport fehlt weiterhin ein nachgewiesener Stillstandsadapter. Die Strukturunterstützung hebt diese Grenze nicht auf.

Eigene Funktionen/Typen/Operatorklassen/Kollationen, andere Ausdrucks- oder Partialindexklassen, nicht unterstützte Checkklassen, externe Referenzen, Partitionen, Vererbung, RLS, Trigger, Regeln, Erweiterungen, abweichende Storageoptionen und Standardrechte bleiben gesperrt. Nicht validierte Constraints und unbekannte Identity-/Indexsemantik blockieren ebenfalls. Bestehende Tabellen und Sequenzdefinitionen müssen strukturell übereinstimmen.

Neue Tabellen werden exakt und sortiert über `Plan.added_tables` gebunden. Für den beobachteten Stand sind das `application_catalog` und `application_emojis`. Ungebundene Ergänzungen oder weggefallene Bestandsobjekte blockieren. Neue Tabellen und deren Sequenzen erhalten ausschließlich Ownerrechte, keine pauschalen Readergrants. Die Aufnahme begründet keine neuen Nutzungs- oder Veröffentlichungsrechte.

## Privater Artefakt- und Q-Vertrag

Das vorab angelegte Artefaktverzeichnis gehört dem ausführenden Benutzer und hat Modus `0700`. Dateien werden mit `0600` neu angelegt und nie überschrieben. Geöffnete Verzeichnis-Inodes, Symlink-/Owner-/Hardlinkprüfungen und eine Verzeichnissperre schützen die Zugriffe. Beide Dumpdateien enthalten private Daten und bleiben lokal.

Manifestversion 2 bindet Plan, Generation, zusätzliche Tabellen, exportierten Snapshot, Snapshotlabel, fünf Ausschlüsse, Quell- und Staging-OIDs, Tabellenzahlen, vollständige Daten-/Struktur-SHA256, Sequenzparameter und -stände, Eigentümer/Rechte sowie Dump- und Schema-Dump-SHA256. `archive_sha256` umfasst diese Metadaten. Version 1 wird abgewiesen. Zugangsdaten stehen weder im Plan noch im Manifest.

`ActivationReceipt` enthält tatsächliche Quell-/Ziel-Datenbank- und Schema-OIDs, Quell-/Archivtabellen und Sequenzen, erhaltene Vorgänger-OID, zusätzliche Tabellen sowie Snapshotlabel, Archiv-/Dump-/Schema-Dump-/Schema-SHA256. `q_binding_required=true` und `g5_complete=false` bleiben fest. Q muss seinen eigenen kanonischen Snapshot beobachten und Alt-/Neubindung, Widerrufe, Tombstones, monotone Kernrevisionen, Checkpoints und Corpuspins prüfen. Der Ops-Archivhash ist kein Kern-Snapshotdigest.

Ein Aufbaufehler rollt die Zieltransaktion zurück. Scheitert anschließend die Manifestablage, bleibt eine ungewechselte Generation erhalten und ein blinder Retry blockiert. Ein unklarer Commit wird als `CommitUncertain` gemeldet. OIDs und vorhandene Artefakte sind dann lesend abzugleichen. Eine Aktivierungsabsicht allein beweist keinen Commit. Scheitert die Belegablage nach bestätigter Aktivierung, meldet die CLI ausdrücklich den erfolgten Wechsel; sie aktiviert nicht erneut. Es gibt keinen automatischen Rückweg über ältere Kernstände.

## Nicht geheimer Zugangsvertrag

Die lokale Metadatenprobe am 3. Oktober 2026 belegt einen vorhandenen direkten Peer-Zugang für denselben Prozess:

| Bindung | Vertrag |
| --- | --- |
| OS-Prozess | `nathanael`, UID 1000, kein root und keine privilegierte Checkout-Ausführung |
| Quelle | `/var/run/postgresql`, Port 5432, DB `deadlock`, DB-Rolle `nathanael`; vorhandene Peer-Anmeldung, Rolle ist Superuser |
| Ziel | `/run/deadlock-brain-postgresql`, Port 5446, DB `brain`, DB-Rolle `brain_migrate`; vorhandene Peer-Map `brain_migrate` ordnet OS-`nathanael` dieser Rolle zu |
| Archivowner | `brain_migrate`; Zielverbindung darf kein Superuser sein |

`Endpoint::connect` verwendet dafür `postgres::Config::host_path`, Port, Datenbank und Rolle mit `NoTls`, ohne Passwort oder ENV-/Passwortdatei. `Endpoint::matches` prüft aktuelle Rolle, DB-OID, DB-Owner, Socketpfad und Serverport frisch. Tatsächliche OIDs und Owner müssen in einem privaten Plan stehen. Die CLI bildet damit einen engen Clientadapter für den vorhandenen Zugang, keine allgemeine Privilegienbrücke. Wegen der Sequenzgrenze ist sie noch kein freigegebener Produktivimportweg. Hier wurden keine produktiven Schreibaufrufe oder Dienständerungen ausgeführt.

`pg_dump` ist auf den geschützten PostgreSQL-16-Pfad festgelegt. Der Unterprozess übernimmt keine `PG*`-Umgebung und keine Passwortdatei. Prozessargumente werden einzeln übergeben, nicht als Shelltext.

## Isolierte Prüfungen

Die Tests starten eigene PostgreSQL-16-Prozesse mit privatem Socket und ohne TCP-Listener. Die sieben übernommenen Tests bleiben bestehen. Ergänzende Tests prüfen Sequenz-/Identitystände, Defaults nach dem Rename, lokale Fremdschlüssel, Enum-Checks, explizite neue Tabellen und enge Rechte. Unsichere Defaults, externe Fremdschlüssel, ungebundene Tabellen und nachträgliche Readergrants müssen blockieren.

Der zusätzliche reale Strukturtest benötigt zwei ausschließlich lokal erzeugte, private Schema-Dumps. Er importiert keine Produktivzeilen. Die Pfade stehen nur im Testprozess:

```text
BRAIN_REFRESH_SOURCE_SCHEMA=<private source.sql>
BRAIN_REFRESH_ARCHIVE_SCHEMA=<private archive.sql>
cargo test --offline --all-targets -j 2 -- --include-ignored --test-threads=1
```

Der ignoriert markierte Strukturtest wird mit `--include-ignored` tatsächlich ausgeführt und verweigert einen fehlenden Fixturepfad. Die Schema-Dumps dürfen nicht ins Git oder an externe Anbieter gelangen. Die Tests ersetzen keinen Produktions- oder G5-Nachweis.
