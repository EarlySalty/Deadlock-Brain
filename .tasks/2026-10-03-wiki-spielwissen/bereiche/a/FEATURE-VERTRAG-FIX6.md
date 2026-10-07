# Fix6: bestätigte serde_json-Anforderung für C3

Datum: 03.10.2026. Aus dem tatsächlichen engen Fix6-Prüflauf des eigenen nativen Sol-high-Fixers übernommen. Gemeinsame produktive Manifeste, Lockfile und Verbraucher ausschließlich C3; A ändert sie nicht.

## Aktuelle produktive Grenze, Punkt 51

Root bestätigt Cs vorhandenen produktiven Pin `serde_json 1.0.150` mit `arbitrary_precision`. A fordert kein Versionsupgrade: Der eingefrorene lokale Harness bleibt unverändert auf `1.0.151`. C prüft den tatsächlichen Featuregraph, PostgreSQL sowie Serialize-/Deserialize- und Leserübergänge mit seinem produktiven Pin getrennt. Die folgenden Angaben zu `1.0.151` beschreiben ausschließlich As lokalen Beleg.

## Lokale Harness-Konfiguration

Bestehende aufgelöste Version serde_json1.0.151 bleibt unverändert. Zusätzlich erforderlich:

```toml
serde_json = { version = "1.0", features = ["arbitrary_precision"] }
```

Vorhandene weitere Features bleiben erhalten. Tatsächlich gebauter lokaler Zielgraph: arbitrary_precision, default, raw_value, std. raw_value war bereits vor Fix6 aktiv. Kein Versionsupgrade oder Lockfilewechsel. Der eingefrorene lokale Harness verwendet diese Einstellung mit Version 1.0.151; C verwendet laut Punkt 51 seinen vorhandenen produktiven Pin 1.0.150 mit arbitrary_precision und prüft die Verbraucherwirkung getrennt.

## Tatsächlicher enger Beleg

Neue Regression im echten bestehenden XML-Normalisierer und WikiSpool bestätigt Dezimalzahlen durch JSONL-Schreiben, erneutes Lesen, from_value und Wiederholung unverändert als Value::Number. Betroffene Werte20.000010800000002 und55.555555555555564 sowie lange Dezimal- und große Integerwerte geprüft. Keine Fake-Parser-/Spoolkomponente.

Tatsächliche ungefilterte Suite Exit0:47passed,0failed,0ignored,0filtered. Zusätzlich echter Archivbeweis Seite2880/Revision18108 Exit0: alle3366 Fakten einschließlich1988 JSON-Zahlen exakt gegen Originalcontent, beide ursprünglichen Abweichungen korrigiert; echte Spoolwiederholung und erneute Publikation byteidentisch.

Die finale Prüffolge ist inzwischen abgeschlossen: 47 Tests bestanden, Clippy aller Targets mit `-D clippy::all` und Debugbau Exit0, selektive Formatprüfung0. Lokale dead_code-Warnungen bleiben sichtbar; Gesamtformat1 nur im unveränderten fremden util.rs. A hat vollständige finale Logs und Präzisionsbeleg gelesen sowie14 End-/Erhaltungsdateien und beide Binaryhashs selbst bestätigt. Eigene Prüffolge endete um12:05:06UTC, Writer/Kinder/Sperren frei. Die Testmodul-Reihenfolgewarnung wurde vor dieser finalen Folge eng geschlossen.

Vier unnötig entfernte vorhandene Kommentare sind inzwischen exakt wiederhergestellt. Der endgültige Source- und Binaryfreeze ist von A bestätigt, ebenso die danach erneut bestandenen 47 Tests und passenden Prüfungen. Einziger Operator führt denselben vollständigen Datenauftrag aus 47 unveränderten Originalinputs fort, derzeit Task bwrrp3l2b am ersten Hostlock. Noch kein vollständiger Wertebeweis oder Importgrün; enger Archivbeleg und Tests reichen dafür nicht.

TESTNACHWEIS[TW-1]: 47 passed, 0 ignored | Baseline: nicht gemessen rot

## Typenwirkung und Verbrauchergrenze

- Value::Number bleibt ein JSON-Zahlenwert. Number verwendet intern eine dezimale Stringdarstellung; das ist weder Value::String noch ein Vertragswechsel.
- Zahlen im Import-/Lesepfad nicht in as_f64 oder eigene Floattypen überführen. Jeder spätere Serialize-/Deserialize-Übergang braucht dieselbe erhaltende Wirkung; C3 prüft tatsächlichen Import, PostgreSQL und bestehenden Leser.
- Number-PartialEq ist darstellungsabhängig. Numerisch gleiche verschiedene Darstellungen dürfen nicht mit stiller Rundung oder Epsilon-Toleranz verglichen werden.
- Bibliothek kanonisiert Exponentenschreibweise (E003 wird e+003) und ganzzahliges -0 zu0. Ursprüngliche Quellenlexeme bleiben bytegenau im content samt unverändertem Inhaltshash. Numerische Fakten sind JSON-Zahlen, Originalrepräsentationen über content und json_pointer nachvollziehbar.
- Kein zweiter Parser, Sonderimporter, Epsilon oder Zahlentypwechsel. Keine Änderung am gemeinsamen Spoolcode bisher benötigt oder freigegeben.

## Noch offene Gesamtfreigabe

Alte Fakten mit282 Abweichungen nicht importfreigegeben. Alte Ausgaben/Binary/Freezes erhalten. Nach tatsächlichem Fix6-Freeze getrennten korrigierten Bestand aus denselben47 Originalen erzeugen, sämtliche Faktenwerte/Originaltexte/Identitäten und vollständige Wiederholung prüfen, gezielte Unterschiede zum alten Stand dokumentieren. Danach frische unabhängige Eigenabnahme und eigener Modulcommit. C3s finale Integration/Gate/Deploy bleiben getrennt.
