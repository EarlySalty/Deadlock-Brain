# Q2: sichere Wiederaufnahme

Gesamt-Q ist noch nicht abgenommen. Kein neuer Thread und keine zweite Quellensammlung erforderlich.

## Erhaltenes Material

- Alte Quellbindung: `q-partial-v1-20261007`, Teilplanhash `c2332949206a0773b604391d7f540c9520598a66409053f94defb4076447b0ed`. Zwei unveränderte lokale Originalkopien gemäß Q/WIEDERAUFNAHME.md.
- Neue lokale Prüfliste: `/home/nathanael/.local/share/brain-q2-private-20261008/review/review-v1.json`, Vorbereitungshash `afc6b5436982b755450c27730a56baad2928b51f0b7b8d16b9370095ab9d5762`. 166 ungeprüfte Einträge; 37 mögliche Dubletten. Datei MUST NOT in die Codiermodellkonversation gelangen.
- Neue öffentliche Originalassets: `/home/nathanael/.local/share/brain-q2-private-20261008/public-assets/`, Snapshotcontainerhash `491e31de7281f022badaec6eb2e9de070893b5423877e76e4112fe059cdbb4a1`.
- Sichere Verträge: `FALLVERTRAEGE.json`, `LAUFVERTRAG.json`, `METHODIK.md`. Referenzierte alte Patch-/Pocket-/Haze-Belege unverändert.
- Ergänzende Originalbindung: `ORIGINALBINDUNG.md`. Neuer getrennt gesicherter Logabzug `q2-required-source-v1-20261008`, kein Ersatz des alten Sets. Geschützte Kopie unter `/home/nathanael/.local/share/brain-q2-private-20261008/required-user-sources/`; öffentliche Coachingabzüge unter `public-web-originals/`. Originale nicht an das Codiermodell geben. Die reine Stichwortprüfung bindet noch keinen Pflichtfall automatisch.

## Vollständige Sicherung vor mechanischem Branch-Abschluss

Der Stop-Hook verlangt die Bereinigung des bereits integrierten Brain-Arbeitsbranches. Das beendet nicht den Q-Gesamtauftrag. Beide vollständigen eigenen privaten Bäume liegen unter `/home/nathanael/.local/share/brain-q2-private-20261008/stop-checkpoint/`: `acceptance-private/` enthält Prüfliste und sämtliche eigenen Protokolle; `collector-private/` die neuen Asset- und Zusatzquellenversionen. 15 Dateien und fünf Verzeichnisse, Bytes und relative Pfade identisch, Eigentümer und 0600/0700 geprüft. Historische zentrale Originalkopien und bisherige Sicherungen bleiben unverändert. In einer frischen ausdrücklich zugewiesenen Ausführungsumgebung die auf main liegenden Quellen verwenden und private Daten ausschließlich aus diesen geschützten Kopien binden.

Das zuvor verwendete Collectorbinary fehlt inzwischen im eigenen Debugcache. Ein neuer Build ist erforderlich; nur cargo-slot benutzen. Kein Compiler- oder Integritätsnachweis aus dem fehlenden Cache erfinden. Die Sicherung selbst wurde mit rekursivem, inhaltsfreiem `diff --brief` und getrennten Metadatenprüfungen tatsächlich bestätigt.

## Zusätzliche Kontrollaufträge beendet, nicht bestanden

`b1d39twbo` (dl-brain-Library) und `bdca849wx` (modglue-Fixturesuite) wurden nach 1800 Sekunden vom Hintergrundlimit beendet. Beide Protokolle sind leer; kein tatsächliches Suite-Ergebnis beobachtet. Sie sind keine noch laufenden Jobs und keine roten Testergebnisse. Für die spätere Wiederholung die Befehle aus `TODO.md` verwenden, mit ausreichend langem eigenen Harness-Zeitfenster. Keine fremden Builds oder Dienste stoppen, den Slot nicht umgehen. Fixtures bleiben getrennt von Gold-, Provider- und Kanalabnahme.

## Reihenfolge

1. Eigene Akte und jeweils tatsächlich belegte G/K-Livelieferung lesen, ohne fremde Threads zu verwalten. Aktuelle Prozess-/Release-/Import-/Consumerbindung neu erfassen. Technischer Snapshot aus STATUS ist zeitgebunden, keine dauerhafte Ausführungsfreigabe.
2. Originalkopien mit `compare-private` und Teilplan mit `audit` erneut prüfen. Neue Prüfliste nicht erneut über `prepare-review` überschreiben. Vor lokalem Bearbeiten ihre vorhandene Sicherung erhalten. Lokale fachliche Originalprüfung, bereinigte Frage, Antwortart, Sollfakten und tatsächliche Belegdateien vervollständigen. Die fünf späteren Nutzerfälle einschließlich gemeinsamer Dreifachnachricht und Ortsbeleg ebenfalls an Originalquellen binden.
3. `check-review` ausführen. Ein ALLOW dieses Strukturchecks ist kein fachliches Goldurteil und kein Merge-Gate. Semantik und Datenschutz bleiben gesondert zu belegen. Den abgenommenen Frage-/Antwortartbestand erst nach mindestens 30 echten geprüften Fällen und vollständigen Pflichtfällen unveränderlich versionieren.
4. Konkreten bestehenden Consumerlauf sowie geschützte Werkzeug-/Antwort-/Verbrauchserfassung binden. Erst nach erfüllter Livevoraussetzung den ersten Luna-Abolauf durchführen. Keine kostenpflichtige Konfiguration aktivieren. Alle Teilfragen, Sichtgrenzen, positive private Antwort ohne Mitlesen, echte Zustellung und Zeit getrennt prüfen.
5. Dasselbe Set nach späteren Deploys wiederholen. Vergleichsschicht unverändert halten, Datenänderungen dokumentieren. P0 bis P11 und A-Fragen abschließend beurteilen. Cleanup und Self-Settle erst bei tatsächlichem Abschluss.

Die zu vervollständigenden Felder der privaten Prüfliste heißen `authenticity`, `privacy`, `provider_question`, `expected_response_kind`, `original_facts`, `original_fact_evidence`, `transport`, `review_evidence_sha256` und `accepted_gold`. Sie sind keine automatischen Labels. Eine Hashzeichenfolge genügt nicht als Beweis: die zugrunde liegende lokale Datei und ihre inhaltliche Zuordnung müssen geprüft sein.
