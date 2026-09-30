status: aktiv, enger Nachreview auf eingefrorenem Testfix
Datum: 2026-09-30

# R1-Synchronisationsnachweis auf 8949198 abnehmen

Derselbe unabhängige Reviewer52c34332, vorhandener Reviewworktree /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929, Branch review/pre-g5-core-abnahme-20260929. Intent562a877b-0939-440a-964d-1145d9e9431a. Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder Unter-Agenten spawnen.

## Bindung

Quelle /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930, Branch fix/g5-replay-deferred-20260930. Neuer tatsächlicher Quellfix89491987a52053c94f98a7d3430d7139570caa95, Commit30.09./15:10:22UTC, gepusht. Gegenbasis392267b133a9a5a8a91602247a441ff7bf609c4b. Eine Datei,273 neue Zeilen, ausschließlich Importer-Testmodul. Autor arbeitet beim Dispatch noch an den Berichten, kein Quell-WIP mehr gesehen. Unveränderlichen Commit prüfen; neuen Bericht B2-R1-SYNCHRONISATION-ERGEBNIS.md und Berichtigung B2-R1-R3-FIX-ERGEBNIS.md mitlesen, sobald vorhanden, aber kein bewegliches Quell-WIP als Bindung verwenden.

Dein Ausgangsurteil eceb14c bleibt gültig: B1-CI GO, R2/R3 geschlossen, R1-Produktkorrektur akzeptiert. Einziger Nachreviewgegenstand ist der belegte Rest: frühere Writer-Joins vor Veröffentlichung waren seriell und hätten fehlende Locks nicht erkannt. Keine erneute Vollinventur akzeptierter Teile.

## Konkrete Prüfung

- Neue Helper advisory_waiter und synchronized_cutover_writers sowie ihr Anschluss im vorhandenen ignorierten Cutovertest. Nachweis tatsächlicher PostgreSQL-Blockierbeziehung statt nur Schlafpause oder unfertigem Task.
- Veröffentlichungsstopp nach der geschützten Vergleichsgrenze unter beiden Quellsperren; konkurrierender Batch-Tombstone und direkter Scope-Apply müssen nachweislich auf genau diesen Publisher warten.
- Zwei Quellen und leerer Batch, Commit und Rollback. Danach Tombstone-/Scopezustand erhalten; erwartbarer CAS-Konflikt nach Commit korrekt getrennt von Sperrbeweis und begrenzt im bestehenden Fixture wiederholt.
- Welche Assertion würde ohne welche neue Quellsperre scheitern? Statisch bewerten, keinen Mutationstest oder Runtimebeweis erfinden. Prüfe, dass ein unbeteiligter Gate-/Recordsperreffekt den eigentlichen Sperrbeweis nicht bloß vortäuscht.
- Cleanup, Fehler-/Timeoutpfade, Verbindungsspitze bei weiterhin max_connections12, vorhandene Runnerselektion und Testattribute. Kein neuer nichtignorierter DB-/Prozessfall im regulären Unitlauf. Keine Produkt-/Manifest-/Rollenänderung.

## Grenzen und Abgabe

Nur statisches Nachreview, keine Quelldatei ändern, keine Code-Kommentare. Kein Cargo, Compiler, Testlauf, PG-/Rollenfixture, DB-Schreiben, Import, Modell-/Serve-/Dienststart oder Deploy. Twitch-Compiler ist laut Nutzer gerade frei, das ist keine konkrete Brain-Zuteilung. Produktimport/Serve weiterhin getrennte koordinierte Runtimephase.

Eigenen Bericht B2-R1-SYNCHRONISATION-NACHREVIEW.md mit genauem GO/BLOCK, verbliebenen konkreten Befunden oder geschlossener Beweisquelle, Laufzeitgrenzen und tatsächlichen SHAs im vorhandenen Reviewbranch committen/pushen. Keine weitere Quellrunde ohne Befund. Bei GO ist die erste beantragte Compilerklasse das gezielte all-targets-Clippy für brain-storage und brain-legacy-import mit Rust1.97.1, locked/offline/jobs1 und bestehendem Cache /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target. Kein zusätzlicher Check vorab nötig, reguläre Tests und PG-/Lastklasse separat. Vollständiger Vertrag in NAECHSTER-G5-LAUF.md der Koordinationsakte, nicht ausführen.

ORCHESTRIERUNG[OR-1]: Stufe groß | Schritt review | Artefakt: .tasks/2026-09-30-g5-abschluss/B2-R1-SYNCHRONISATION-REVIEW-BRIEFING.md
