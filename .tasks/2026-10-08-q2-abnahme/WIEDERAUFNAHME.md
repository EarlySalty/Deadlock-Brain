# Q2: sichere Wiederaufnahme

Gesamt-Q ist noch nicht abgenommen. Kein neuer Thread und keine zweite Quellensammlung erforderlich.

## Erhaltenes Material

- Alte Quellbindung: `q-partial-v1-20261007`, Teilplanhash `c2332949206a0773b604391d7f540c9520598a66409053f94defb4076447b0ed`. Zwei unveränderte lokale Originalkopien gemäß Q/WIEDERAUFNAHME.md.
- Neue lokale Prüfliste: `/home/nathanael/.local/share/brain-q2-private-20261008/review/review-v1.json`, Vorbereitungshash `afc6b5436982b755450c27730a56baad2928b51f0b7b8d16b9370095ab9d5762`. 166 ungeprüfte Einträge; 37 mögliche Dubletten. Datei MUST NOT in die Codiermodellkonversation gelangen.
- Neue öffentliche Originalassets: `/home/nathanael/.local/share/brain-q2-private-20261008/public-assets/`, Snapshotcontainerhash `491e31de7281f022badaec6eb2e9de070893b5423877e76e4112fe059cdbb4a1`.
- Sichere Verträge: `FALLVERTRAEGE.json`, `LAUFVERTRAG.json`, `METHODIK.md`. Referenzierte alte Patch-/Pocket-/Haze-Belege unverändert.
- Ergänzende Originalbindung: `ORIGINALBINDUNG.md`. Neuer getrennt gesicherter Logabzug `q2-required-source-v1-20261008`, kein Ersatz des alten Sets. Geschützte Kopie unter `/home/nathanael/.local/share/brain-q2-private-20261008/required-user-sources/`; öffentliche Coachingabzüge unter `public-web-originals/`. Originale nicht an das Codiermodell geben. Die reine Stichwortprüfung bindet noch keinen Pflichtfall automatisch.

## Bereits gestartete eigene Kontrollaufträge

`b1d39twbo` wiederholt die vorhandene dl-brain-Library; `bdca849wx` die gesamte modglue-Fixturesuite. Beide nutzen den eigenen unveränderten Botscheckout und cargo-slot. Beim Aktenstand noch kein Ergebnis. Nicht parallel duplizieren, keine fremden Builds oder Dienste stoppen. Nach eigener Jobmeldung die geschützten Protokolle `private/k-daily-controls.log` und `private/k-modglue-controls.log` auf tatsächliche Exit-/Passed-Zahlen prüfen; erst danach Erfolg dokumentieren und unabhängig sichern. Fixtures bleiben getrennt von Gold-, Provider- und Kanalabnahme.

## Reihenfolge

1. Eigene Akte und jeweils tatsächlich belegte G/K-Livelieferung lesen, ohne fremde Threads zu verwalten. Aktuelle Prozess-/Release-/Import-/Consumerbindung neu erfassen. Technischer Snapshot aus STATUS ist zeitgebunden, keine dauerhafte Ausführungsfreigabe.
2. Originalkopien mit `compare-private` und Teilplan mit `audit` erneut prüfen. Neue Prüfliste nicht erneut über `prepare-review` überschreiben. Vor lokalem Bearbeiten ihre vorhandene Sicherung erhalten. Lokale fachliche Originalprüfung, bereinigte Frage, Antwortart, Sollfakten und tatsächliche Belegdateien vervollständigen. Die fünf späteren Nutzerfälle einschließlich gemeinsamer Dreifachnachricht und Ortsbeleg ebenfalls an Originalquellen binden.
3. `check-review` ausführen. Ein ALLOW dieses Strukturchecks ist kein fachliches Goldurteil und kein Merge-Gate. Semantik und Datenschutz bleiben gesondert zu belegen. Den abgenommenen Frage-/Antwortartbestand erst nach mindestens 30 echten geprüften Fällen und vollständigen Pflichtfällen unveränderlich versionieren.
4. Konkreten bestehenden Consumerlauf sowie geschützte Werkzeug-/Antwort-/Verbrauchserfassung binden. Erst nach erfüllter Livevoraussetzung den ersten Luna-Abolauf durchführen. Keine kostenpflichtige Konfiguration aktivieren. Alle Teilfragen, Sichtgrenzen, positive private Antwort ohne Mitlesen, echte Zustellung und Zeit getrennt prüfen.
5. Dasselbe Set nach späteren Deploys wiederholen. Vergleichsschicht unverändert halten, Datenänderungen dokumentieren. P0 bis P11 und A-Fragen abschließend beurteilen. Cleanup und Self-Settle erst bei tatsächlichem Abschluss.

Die zu vervollständigenden Felder der privaten Prüfliste heißen `authenticity`, `privacy`, `provider_question`, `expected_response_kind`, `original_facts`, `original_fact_evidence`, `transport`, `review_evidence_sha256` und `accepted_gold`. Sie sind keine automatischen Labels. Eine Hashzeichenfolge genügt nicht als Beweis: die zugrunde liegende lokale Datei und ihre inhaltliche Zuordnung müssen geprüft sein.
