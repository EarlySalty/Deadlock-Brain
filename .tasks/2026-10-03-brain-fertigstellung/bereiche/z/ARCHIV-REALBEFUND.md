status: aktiv
Datum: 2026-10-03

# Archivstruktur: lesender Echtbefund

Aufnahme am 03.10.2026 vor 17:05:50 UTC. Ausschließlich PostgreSQL-Katalogmetadaten in `BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY`, abgeschlossen mit `ROLLBACK`. Keine Inhaltszeilen, Rohtexte, Secrets oder Produktionsänderungen.

Quelle: vorhandene OS-Rolle `postgres`, Socket `/var/run/postgresql`, Port 5432, DB `deadlock`, Schema `brain`. Zielaufnahme: vorhandene OS-Rolle `deadlock-brain-pg`, Socket `/run/deadlock-brain-postgresql`, Port 5446, DB `brain`, Schema `brain_legacy`. Feste PostgreSQL-16-`psql`-Binary, jeweils `-X -q -At -v ON_ERROR_STOP=1`; kein Credentialwert oder DSN ausgegeben.

## Tatsächliche Strukturen

Viele der nicht ausgeschlossenen Quelltabellen besitzen Defaults und Fremdschlüssel. `application_catalog` hat zusätzlich einen Check. Die fünf bestehenden Ausschlüsse bleiben `patch_changes`, `feeder_runs`, `plan_items`, `plan_items_verworfen`, `plan_runs`.

Die Defaultklassifizierung der eingeschlossenen Tabellen ergab:

| Klasse | Anzahl |
| --- | --- |
| `now()` | 24 |
| Sequenzzugriff | 30 |
| UUID-Erzeugung | 1 |
| Boolean | 1 |
| Zahl | 13 |
| Andere, noch einzeln zu prüfen | 38 |

Gemessene Spaltentypen liegen ausschließlich in `pg_catalog`: Arrays `_int2`, `_int4`, `_int8`, `_text`; außerdem `bool`, `float8`, `int2`, `int4`, `int8`, `jsonb`, `text`, `timestamptz`, `uuid`.

Im gesamten Quellschema liegen 35 Sequenzen. Alle 46 dort erfassten Fremdschlüssel zeigen nach `brain`; diese Gesamtzahl umfasst auch die getrennt ausgeschlossenen Tabellen. Sie wird nicht als Zahl allein der kopierten Tabellen ausgegeben.

Das bestehende Archiv enthält entsprechende Defaults und Fremdschlüssel. Gegenüber der Quelle fehlen darin `application_catalog` und `application_emojis`. Die aufgenommenen Spaltenzahlen der übrigen Tabellen stimmen überein; das beweist noch keine Gleichheit aller Typen, Constraints, Indexe, Sequenzstände oder Rechte.

## Konsequenz für den Bau

Der erste Rust-Stand unter `ops/brain-postgres/legacy-refresh/` blockiert Defaults, Sequenzen, Checks und Fremdschlüssel grundsätzlich. Er ist damit für diesen echten Bestand nicht nutzbar, auch wenn einfache Fixtures später grün wären. Der erhaltene Stand wird vervollständigt, nicht durch Entfernen der Guards freigegeben.

Neue Generationen müssen den tatsächlich gebundenen Quelltabellensatz einschließlich zulässiger neuer Tabellen erhalten. Neue Objekte begründen keine zusätzlichen Consumer- oder Nutzungsrechte. Unbekannte Strukturklassen, nicht gebundene Änderungen und unklare Commitausgänge bleiben Stop-Bedingungen. Ein vollständiger Struktur- und Inhaltsvergleich sowie echte isolierte Laufzeitprüfungen fehlen noch.

Der erste Worker lieferte Source und sieben isolierte PostgreSQL-Tests. Fmt und offline Lock-Erzeugung waren erfolgreich; Clippy und Tests starteten wegen einer beendeten Hostlock-Wartekette nicht. Seine Rückgabe behauptet ausdrücklich keine Bauverifikation. Der frische Fortbauworker `wf_e02c6493-1f2` übernimmt genau diese Dateien und den Echtbefund. Kein Produktionsimport ausgeführt.
