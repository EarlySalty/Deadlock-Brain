# Brain-Schema für isolierte CI-Datenbanken

## Vertrag

Die Source of Truth der Basistabellen ist
`EarlySalty/Deadlock-Bots`, `rust/crates/dl-central-db/migrations`,
Commit `ff635f7b354cb09909c01ddd6f773d0682dd89c9`.
Die sechs SQL-Dateien unter `vendor/dl-central-db/` sind byteidentische Kopien
aller dortigen Migrationen mit `brain.*`-DDL an diesem Commit. Sie enthalten
keine Produktionsdaten. `SHA256SUMS` fixiert ihre Inhalte und die beiden bereits
versionierten Brain-Migrationen für Reasoner und Population.
Produktiv angewandte Migrationen wurden nicht verändert.

Voraussetzungen: PostgreSQL 16, `psql`, eine leere Wegwerf-Datenbank und ein
Benutzer mit DDL-Rechten in dieser Datenbank. Keine zusätzlichen Extensions:
`gen_random_uuid()` gehört zu PostgreSQL 16. Keine privaten Git-Checkouts,
Deploy-Keys, Produktions-Secrets, Daten-Dumps oder Live-Dienste sind erforderlich.

```sh
# Im Checkout des unveränderlichen Brain-Commits ausführen.
sha256sum --check schema/SHA256SUMS
# DATABASE_URL muss ausschließlich auf die isolierte Testinstanz zeigen.
psql "$DATABASE_URL" -X --set=ON_ERROR_STOP=1   --file scripts/ci/bootstrap-brain-schema.sql
```

Der Datenbankname muss `ci` oder `test` als durch Unterstriche getrennten
Bestandteil enthalten, beispielsweise `twitch_ci` oder `brain_test`.
Der Einstieg erstellt ausschließlich `brain.*`, einschließlich
`brain.hero_catalog`, `brain.item_catalog` und der weiteren Tabellen für die
Brain-Crates. Er verweigert bestehende Brain-Relationen und führt alle Schritte
in einer Transaktion aus. Ein zweiter Aufruf in derselben Datenbank ist bewusst
ein Fehler, kein teilweise erfolgreicher Replay. Für einen Replay eine neue
Testdatenbank anlegen. Die Guards ersetzen keine isolierte Infrastruktur.

## Twitch-Übergabe

Brain am im PR genannten vollständigen Commit-SHA auschecken, den obigen Aufruf
**vor** Compile-Prüfungen der eingebundenen Brain-Crates ausführen und
`DATABASE_URL` für die Online-Validierung der SQLx-Makros auf dieselbe Test-DB
setzen (`SQLX_OFFLINE=false`). Die Verantwortung für Nicht-Brain-Schemas und
Twitch-Migrationen bleibt beim Twitch-Repository. Enthält ein dortiger
Bootstrap schon Brain-Migrationen, darf dieser Einstieg nicht zusätzlich
über dieselben Tabellen laufen; die zuständige Session legt die Reihenfolge
fest. Der Dependency-Pin wird ausschließlich dort geändert.

Schemaänderungen benötigen neue Migrationen bzw. einen neuen, überprüften
Upstream-Snapshot mit neuer Herkunft und Prüfsummen. Die historischen Kopien
sind keine Einladung, Produktionsmigrationen nachträglich umzuschreiben.
