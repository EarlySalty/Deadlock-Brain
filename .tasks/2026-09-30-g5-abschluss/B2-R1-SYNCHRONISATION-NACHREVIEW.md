# B2-R1: statisches Nachreview des Synchronisationsnachweises

Datum: 2026-09-30. fertig: J. Fix: N.

**GO für den engen Testquellenfix `89491987a52053c94f98a7d3430d7139570caa95`.** Der letzte R1-Rest aus `eceb14c` ist statisch geschlossen: Die Quelle erzwingt jetzt überlappende Publisher-/Writertransaktionen mit beobachteter PostgreSQL-Blockierbeziehung, statt Writer vor dem Publish vollständig abzuwarten. Commit, Rollback, zwei Quellen und leere Publikationsbatches sind im vorhandenen ignorierten Fixture verbunden. Kein Compiler- oder Laufzeit-GO daraus ableiten.

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 1/1 geprüft

Der Fremddienstpfad ist der statisch gelesene PostgreSQL-Synchronisationsvertrag. Keine Tests, Compiler, PG-Verbindungen, Rollenfixtures, Imports, Produktprozesse, Modelle, Dienste oder Deploys ausgeführt.

## Bindung und tatsächlich ausgeführte Prüfung

- Eingefrorene Quelle: `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930`, Delta `392267b133a9a5a8a91602247a441ff7bf609c4b..89491987a52053c94f98a7d3430d7139570caa95`. Eine Importerdatei, 273 hinzugefügte Zeilen. Der Dateiteil vor `#[cfg(test)]` ist bytegleich; Testattribute und bestehender Runner sind unverändert. Kein Produkt-, Manifest-, Rollen- oder Grantdelta.
- Eigene Berichtssenke: `/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929`, Branch `review/pre-g5-core-abnahme-20260929`, sauberer Ausgangsstand nach `eceb14c`. Akzeptierte B1-, R2-/R3- und R1-Produktteile wurden nicht neu entworfen oder erneut inventarisiert.
- Graphify zu den zwei neuen Helpernamen lieferte keine Knoten. Danach eingefrorenen Diff, Helper, deren zwei Aufrufe und die unmittelbaren Bestandsverträge geprüft. `git diff --check 392267b 8949198`: Exit 0. Keine Rustfmt-/Compiler-/Testausführung durch den Reviewer.
- Bei der abschließenden begrenzten Berichtsaufnahme war der Quell-HEAD weiterhin `8949198`; `B2-R1-SYNCHRONISATION-ERGEBNIS.md` war darin noch nicht committed. Die angekündigte Autoren-Berichtsabgabe wurde nicht abgewartet und nicht als Nachweis vorausgesetzt. Der Quellbefund ist unabhängig davon abgeschlossen. Keine laufende Autorenarbeit gestoppt.

Nachfolgende Importerzeilen beziehen sich auf `rust/crates/brain-legacy-import/src/bin/brain-legacy-import.rs` bei `8949198`.

## Nachweisführung und Urteil

1. **Publisher erreicht die geschützte Vergleichsgrenze, GO.** Der Test hält zunächst in einer eigenen Transaktion den Schlüssel `core-release:<release_id>` (`:820-829`). Der gestartete Publisher verwendet die echte Methode `commit_batches_and_publish_checked` mit zwei Batches (`:841-849`). `advisory_waiter` verlangt für dessen benannten Backendprozess `wait_event_type='Lock'`, `wait_event='advisory'` und die konkrete Gate-PID in `pg_blocking_pids(pid)` (`:679-697,850`). Im akzeptierten Storepfad folgt die Release-Schlüsselsperre auf beide Batchcommits und den geschützten Checkpoint-/Headvergleich. Ein Backend, das auf diesem Gate wartet, hat diese Grenze erreicht; es hält bis dahin beide Quellsperren. Kein bloßer Sleep oder ungeprüfter Taskstatus als Ersatz.
2. **Beide Writer warten auf genau diesen Publisher, GO.** Der Batchwriter versucht den Tombstonecommit und der direkte Apply-Writer den Scopewiderruf über getrennte Ein-Verbindungs-Pools (`:852-874`). Für beide wird ein Advisory-Wait auf die zuvor ermittelte Publisher-PID verlangt, bevor das Gate freigegeben wird (`:875-879`). Der Batchwriter erhält absichtlich die bereits beanspruchte Leasekopie (`:819,863`), damit keine vorgelagerte Claim-Kollision den Commitpfad ersetzt. Das ist eine synthetische Sperrprobe, kein zweiter produktiver Leasebesitzer. Keiner der Writer verlangt den Release-Gateschlüssel; der Publisher ist wegen leerer Records auch kein konkurrierender Recordschreiber. Die nachgewiesenen Writerwartebeziehungen sind daher nicht durch das Testgate oder eine Recordsperre vorgetäuscht.
3. **Commitfall mit begrenztem CAS-Wiederanlauf, GO.** Beide vorbereiteten Publikationsbatches enthalten keine neuen Records (`:748-762`), werden aber gemeinsam veröffentlicht. Nach Gatefreigabe verlangt der Test zwei Commitreceipts. Der schon vorbereitete Tombstonebatch muss wegen seiner alten Checkpointgeneration scheitern; anschließend wird er genau einmal aus dem neuen Checkpoint vorbereitet, neu geclaimt und committed (`:879-899`). Die davor beobachtete Advisory-Blockade ist unabhängig von diesem später erwarteten CAS-Konflikt. Direkter Apply muss `Updated` liefern, Releaseanzahl steigt um eins, Entity-Checkpointgeneration um eins. Historischer Release mit drei Revisionen und aktuelle private/tombstonierte Heads werden danach geprüft (`:901-928`).
4. **Rollbackfall mit echten nachlaufenden Writern, GO.** Separat benannte Fixturequellen und ein bereits vorhandener Release derselben ID mit abweichender Epoch erzwingen den unveränderlichen Releasekonflikt (`:711-723,779-786`). Der Publisher durchläuft trotzdem zuerst die geschützte Vergleichsgrenze und wartet am Gate. Nach Gatefreigabe muss er fehlschlagen; der wartende Tombstonebatch kann unter der erhaltenen alten Generation/Lease erfolgreich committen, der direkte Scope-Apply ebenfalls (`:881-904`). Unveränderte Releaseanzahl, zurückgerollte Entity-Checkpointgeneration, erhaltene alte Release-Epoch und danach vorhandene Tombstone-/Privatheads werden geprüft (`:905-928`). Es wird kein Rollback über die später erfolgreichen Writer hinweg behauptet.
5. **Anschluss und Reichweite, GO.** Beide Helperaufrufe sind im bestehenden `#[ignore]`-Test, erst Commit-, dann Rollbackfall (`:934-936,1253-1270`). Die bisherigen seriellen Fälle bleiben als Baseline-/CAS-Gegenproben erhalten, sind nicht mehr der Konkurrenzbeweis. Der bereits akzeptierte Serve-Runner selektiert denselben exakten Testnamen in `brain_cutover_test`; die getrennte leere `brain_schema_test`-Kontrolle bleibt danach bestehen. Kein neuer nichtignorierter DB-Fall im regulären Binärtestlauf. Die Helper verwenden separate synthetische Quell-IDs innerhalb derselben Scratch-DB und die echte geprüfte Storemethode; sie ersetzen nicht den vorherigen positiven Importerzweig im umgebenden Test.

## Welche Schutzfehler diese Testquelle erkennen würde

- Fehlt die frühe Quellsperre des **Batchwriters**, wartet sein `SELECT ... FOR UPDATE` zunächst auf den vom Publisher gehaltenen Jobdatensatz. Ein solcher Row-/Transaktionslock erfüllt den verlangten Advisory-Wait nicht. Die Beobachtung bei `:875` schlägt dann per Timeout fehl, statt die zufällige Zeilensperre als Quellenfencing zu akzeptieren.
- Fehlt die Quellsperre des **direkten Apply-Writers**, hält der Publisher bei den leeren Batches keine passende Recordsperre. Der Apply kann durchlaufen; die verlangte Advisory-Blockade bei `:876` fehlt und der Test scheitert.
- Hält der **Publisher keine der betroffenen Quellsperren** bis zur Veröffentlichung, kann insbesondere der direkte Apply nicht als Advisory-Waiter auf diese Publisher-PID beobachtet werden. Die Testquelle reagiert damit auf das ursprüngliche Schutzloch. Die zusätzliche sortierte Vorabsperrung und die per Batch wiederholte Sperrnahme sind teilweise redundant: Das isolierte Entfernen einer redundanten Sperrnahme muss nicht fehlschlagen, solange die wirksame gemeinsame Schreibgrenze bestehen bleibt. Kein ausgeführter Mutationstest wird behauptet.

Zwillingssuche umfasst beide Writerpfade, beide Helperaufrufe, ihre Join-Reihenfolge und die drei Advisory-Beobachtungen pro Fall. Kein verbleibender konkreter Quellbefund im beauftragten Synchronisationsrest.

## Timeout, Cleanup und Ressourcenwirkung

`advisory_waiter` hat je Beobachtung eine 15-Sekunden-Grenze und prüft wiederholt den tatsächlichen Backendzustand; `yield_now` ersetzt keine behauptete Mindestwartezeit. SQLfehler und ausbleibende Blockierbeziehung führen zu einem Testfehler. Die nach Gatefreigabe erfolgenden Task-Joins und Pool-Closes besitzen keine zusätzlichen expliziten Testdeadlines; der Helper belegt deshalb keine harte Gesamtzeitgrenze für beliebige DB-/Verbindungsfehler. Im gelesenen normalen Sperrgraph gibt es nach Gatefreigabe keinen vom Test weiter absichtlich gehaltenen Blocker.

Im Erfolgsweg werden die drei eigenen Pools geschlossen (`:929-931`), bevor der zweite Fall beginnt. Bei Panic vor Gatefreigabe ist der lokale Gate-Transaktionswert nicht committed und wird verworfen; die Taskbeendigung folgt der Test-Runtime, das äußere vorhandene Harness stoppt beim fehlgeschlagenen Testprozess seinen eigenen Cluster und behält Fehlerbelege. Keine neue Dienst- oder Fremdcluster-Cleanupaktion eingeführt.

Während der synchronisierten Phase werden planmäßig **fünf gleichzeitig benötigte Verbindungen** genutzt: Gate und Beobachter aus dem bestehenden Fixturepool sowie je eine für Publisher, Batchwriter und Applywriter. Die drei zusätzlichen Pools sind mit `max_connections(1)` begrenzt. Die beiden Fälle laufen nacheinander, und der bisherige Fixturepool wird nicht auf ein höheres Maximum umgestellt. Dies ist die aus dem Ablauf abgeleitete gleichzeitige Nutzung, keine gemessene serverweite Spitze oder Garantie über jede asynchrone Poolabbauphase. Der spätere Lauf muss den tatsächlichen Peak und den bestehenden Verbindungsfehlercheck bestätigen. Clustervertrag bleibt Unix-Socket Port 55439, `max_connections=12`, `shared_buffers=16MB`.

Runnerklasse unverändert sieben serielle Cargoaufrufe, nun mit den zusätzlichen zwei synchronisierten Storefällen im vorhandenen Cutover-Testprozess. Das erzeugt echte lokale Claims, Seeds, Checkpoints, direkte Applies und Releases, einschließlich eines absichtlichen Rollbacks. Der spätere vollständige Runner enthält weiterhin die separaten Serve-/Provider-/SCRAM-Prozessfixtures und **600 Lastanfragen bei jeweils 8/16/32 Workern**, also 1.800 Lastanfragen zusätzlich zu funktionalen Requests. Kein leichter Smoke, keine produktive Rollen-/Secret-Exec-Parität aus der privilegierten Scratchrolle.

## Freigabepunkt und erste beantragte Compilerklasse

Der R1-Testquellenrest ist mit diesem statischen GO geschlossen; B1-CI-GO sowie R2/R3- und R1-Produktabnahme bleiben bestehen. Compiler, reguläre Tests und Scratch-/Lastnachweise sind noch auszuführen, aber nicht durch diesen Bericht zugeteilt. Quelle ist `89491987a52053c94f98a7d3430d7139570caa95`; spätere reine Berichtscommits können diese Bindung dokumentieren, ohne sie durch einen beweglichen Arbeitsbaum zu ersetzen.

Gemäß aktueller Koordinationsakte `NAECHSTER-G5-LAUF.md` ist **gezieltes All-Targets-Clippy der zwei betroffenen Pakete** die erste beantragte Compilerklasse. Die frühere Check-Alternative wird nicht zusätzlich vorgeschaltet:

```bash
/home/nathanael/.cargo/bin/cargo +1.97.1 clippy --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-storage -p brain-legacy-import --all-targets --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target -- -D warnings
```

Voraussetzungen bleiben konkreter revisionsgebundener sauberer Stand, vorhandene Toolchain 1.97.1, Cargo-Home `/home/nathanael/.cargo`, bestehender Targetcache und tatsächlich zugeteilter Slot. Kein Fetch, neuer Cache, Releasebuild oder vorausgehender Zusatzcheck. Clippy führt diese Testfunktionen nicht aus. Nichtignorierte Library-/Binärtests und der vollständige Serve-Harness erhalten getrennte Laufzuteilungen gemäß Koordinationsakte. Keine automatische Verkettung und kein Start aus der Meldung eines freien Twitch-Compilers.

Nächste Aktion: dieses Quell-GO mit SHA `8949198` in den bestehenden begrenzten Clippy-Antrag übernehmen. Keine weitere Quellrunde ohne neuen konkreten Befund.

TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 1 belegt | Senke: B2-R1-SYNCHRONISATION-NACHREVIEW.md
