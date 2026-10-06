# Unabhängige Eigenabnahme7: Übergabe blockiert

Datum: 03.10.2026. Prüfer `a0cfedf24bce1d03e`, frisch durch A nach Root-Wache10:45UTC gestartet, GPT6.1Sol höchstens high/medium. Native Rückgabe abgeschlossen und durch A übernommen. Keine Änderungen oder neuen Compiler-, Test-, Gate-, Netz-, Git- oder DB-Läufe durch den Prüfer.

## Urteil

Fertig: N, bezogen ausschließlich auf As Daten-/Modulübergabe. Fix nötig: J. Kein Modulcommit und keine Importfreigabe vor Auflösung des folgenden bestätigten Befunds. Kein Fix6 automatisch gestartet; Eskalation vor Runde6 gemäß Rootauftrag.

## Bestätigter Restbefund: numerische Präzision

Der Prüfer verglich sämtliche 1.109.153 strukturierten Fakten mit ihren Original-JSON-Werten: 282 numerische Abweichungen, keine sonstigen Wertabweichungen, keine fehlenden Blätter und keine doppelten Faktenzeiger. Die Gesamtzahl282 ist Ergebnis der unabhängigen vollständigen Prüfung. A hat zwei konkrete Abweichungen zusätzlich selbst rein lesend mit vorhandenem jq bestätigt, keine zweite Vollprüfung behauptet.

Dokument `wiki:deadlock-wiki:page:2880`, Revision `18108`, tatsächliche Ausgabe:
`/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized/deadlock.wiki-20250415-history/documents.jsonl`

| Original-JSON-Zeiger | Originalwert | Strukturierter Faktenwert | A-Zahlenvergleich |
| --- | ---: | ---: | --- |
| `/hero_atlas/FalloffStartRange` | 20.000010800000002 | 20.0000108 | ungleich |
| `/hero_ghost/DPS` | 55.555555555555564 | 55.55555555555557 | ungleich |

Die Differenzen sind klein, aber numerisch real. Der Originaltext und dessen Inhaltshash sind unverändert erhalten. Betroffen ist die abgeleitete strukturierte Zahl, nicht der Originaltext.

A las nach Graphify-Abfrage die benannten tatsächlichen Stellen: `rust/crates/dbrain-sources/src/wiki_inventory/normalize.rs:586` liest JSON in `serde_json::Value`, `:860-919` durchläuft die Struktur, `:902-916` übernimmt den bereits eingelesenen Wert in den Fakt. Der vorhandene Rust-Harness prüft Inhaltshash und Faktenzahl, aber bisher keine vollständige Originalwertgleichheit jedes Fakts. Die frühere Pflichtfeld-/Typenprüfung beweist deshalb nicht die numerische Werttreue.

## Begrenzter Lösungsvorschlag an Root

Eine eng begrenzte Präzisionskorrektur über den vorhandenen serde_json-Pfad prüfen. Die Zahlen müssen vom Original über Normalisierung und Spool bis zum C3-Verbraucher werttreu bleiben. Erhaltende serde_json-Features beziehungsweise vorhandene Zahlendarstellung prüfen; keine pauschale Epsilon-Toleranz, Rundung oder Unterdrückung der282 Differenzen. Keine zweite Parserimplementierung, kein Sonderimporter.

Manifest-/Core-/Verbraucheränderungen gehören C3. Vor Fix6 braucht A Roots Entscheidung zur eng begrenzten Zuständigkeit und Umsetzung. Nach ausdrücklicher Freigabe frischen Fixer einsetzen, bestehende Originale und alte geprüfte Ausgaben erhalten, Freeze bilden, unabhängige vollständige Faktenwertnachprüfung und nötige Wiederholungsbeweise erneut führen. Kein Inputwechsel und kein Überschreiben der alten Herkunft.

## Unabhängig bestätigte Beweise

- Echter GesamtloopExit0, Originalprüfung0, Wrapperende10:35:46UTC und geschlossene eigene Locks anhand des tatsächlichen Shell-Endbelegs. Kein Erfolgsschluss allein aus fehlenden PIDs.
- 38.273 Dokumentversionen und1.109.153 Fakten, alle sechs vollständigen Wiederholungen byteidentisch, alle sechs aktuellen JSONL-SHAs passend. Reproduzierbarkeit beseitigt den Präzisionsbefund nicht.
- Alle38.261 Archiv-Revisionshashmatches, keine fehlenden Versionen; zwölf API-Originaltexte zusätzlich selbst exakt verglichen. Alle47 Originalhashs erneutOK.
- Vier Module, Harnessquelle und Binary vor/nach exakt im Briefing-Freeze. Fünf Quellenidentitäten getrennt, keine globalen ID-/Revisionsdubletten. Autoren-/Contributor-Erhaltung im tatsächlichen Code nachvollzogen; unbekannte API-Autoren nicht erfunden.
- Rechte eingeschränkt: überall redistribution_allowed=false. Revisionszeiten, historische Abrufe und heutige Lizenzbeobachtung getrennt. Inventare unvollständig, historische Namespace-Nenner korrekt, Einseitendump/Redirectlücken/Challengegrenzen erhalten.
- 851.125.808 JSONL-Bytes selbst bestätigt; vorhandener Größenbeleg maximal4.763.003Bytes ohneLF pro Zeile. C3 muss tatsächliche Lesergrenzen prüfen.
- Bestehende tatsächliche Tests43passed/0failed/0ignored/0filtered, Clippy/Debugbau0; Gesamtformat1 im unveränderten fremden util.rs, eigene selektive Formatprüfung0. Keine neuen Prüfprogramme durch Eigenabnahme.

TESTNACHWEIS[TW-1]: 43 passed, 0 ignored | Baseline: nicht gemessen rot

## Freigabegrenze

Gebaut: ja. Eigenabnahme: nicht bestanden. Gemergt/live: nein. Kein A-Modulcommit, Fix6, DB-Import oder Produktivbeweis. C3s Integration, Gate und Deploy bleiben getrennt. Vor einer sechsten Fixrunde zuerst diese konkrete Ursache und diesen begrenzten Vorschlag an Root melden.
