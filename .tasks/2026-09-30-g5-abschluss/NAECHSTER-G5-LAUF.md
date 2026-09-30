status: aktiv, Clippy/U1/U2/PG1 Exit0 abgeschlossen; als Nächstes separat PG2, noch keine Zuteilung

PG1 wurde nach separater ausdrücklicher Zuteilung ausgeführt und tatsächlich bestanden:1 passed/0 failed, Exit0, sauberer Cleanup. VollbelegSLOT-I-PG1-NACHWEIS.md. Nächster Bedarf ausschließlich der vorhandene PG2-Runner test_brain_serve.sh mit bestehendem Target; sieben serielle Cargo-Aufrufe einschließlich Cutover-Race, Tombstone/Rechte, lokalem Serve/Providerstub und1.800 Lastanfragen. Keine automatische Folgeaktion, keine produktiven Imports/Dienste. Ältere PG1-Anfragen darunter sind durch den Nachweis erledigt.

Aktueller Stand: SLOT-F/G/H belegen Clippy grün, U1 mit3 bestandenen/1 ignorierten und U2 mit20 bestandenen/1 ignorierten Fällen. Alle aufc5951b6, keine Fehler; ignoriertePG-Fälle nicht bestanden. Nächster kleinster vorhandener eigenständigerPG-Runner istPG1 aus NACHWEISFOLGE-UND-CUTOVER.md: test_brain_storage_upgrade.sh mit explizitem Cargo und vorhandenem Target, ein isolierter Upgrade-/Restore-/Minimalrollenfall. PG2 separat mit sieben Cargo-Aufrufen, Cutover-Race und Serve/Last. Kein PG-/Prozesslauf gestartet, keine automatische Kette. Ältere Start-/Anfrageabschnitte darunter sind Verlauf.

U1-Zuteilung tatsächlich erhalten und ausgeführt auf sauberemc5951b6, Start16:19:54UTC, Harnessb6sdg0hdh/PID1716785. Genau zuvor gemeldetes Importer-Binärtestkommando, locked/offline/jobs1/test-threads1. Ergebnis noch offen. U2, PG1, PG2 und Livebeweise bleiben getrennt zuzuteilen. Ältere Anfrageformulierungen darunter sind Verlauf.

Aktueller Compilerbeleg SLOT-F-CLIPPY-NACHWEIS.md: bnqo5hhln, Start16:17:01UTC, tatsächlicher Exit0, Slot sofort zurück, Nutzer bestätigt16:17:31. Der unten ursprünglich beantragte Clippy ist erledigt und nicht erneut zu starten. Nächster Bedarf U1 aus NACHWEISFOLGE-UND-CUTOVER.md, exakt ein Importer-Binärtestlauf ohne --ignored, locked/offline/jobs1 und derselbe vorhandene Targetcache. U2/PG/Serve/Last/Live bleiben getrennt zuzuteilen. Kein Folgelauf gestartet.

Aktueller Folgeplan nach grünem Clippy: NACHWEISFOLGE-UND-CUTOVER.md. Kleinste direkten Importerunits zuerst (U1:3 normale Fälle), danach betroffene Librarys (U2:20 normale Fälle,1 ignorierterPG-Fall). PG1 ist der bestehende eigenständige Upgrade-/Restore-/Minimalrollenrunner mit einem Test; PG2 der ungeteilte bestehende Serve-/Cutoverrunner mit sieben Cargo-Aufrufen und1.800 Lastanfragen. Jede Klasse einzeln zuzuteilen, kein automatischer Folgelauf. Diese Konkretisierung ersetzt die ältere Reihenfolge und behebt den veralteten Runnerblocker in SERVE-PG-VORAUSSETZUNGEN.md. Die fehlende reale Policy-/Snapshot-/ID-Bindung bleibt unabhängig davon ausdrücklich offen.

Quellabschluss16:04:24UTC übernommen: efb56023deda07ae2c273883d617f2f26b263fc8 entfernt ausschließlich PgConnection aus der bestehenden Importliste. Danach enthältc5951b6 ausschließlich den separaten Bericht. Vollständiges Delta, sauberer Status und eigener Diffcheck geprüft. Bestehende Produktabnahmen gelten weiter, keine neue Gesamtquellenrunde. Genau ein unveränderter Paket-Clippy aus Abschnitt1 ist der nächste Bedarf; alle folgenden Test-/DB-/Runtimeklassen bleiben gesondert zuzuteilen. Vor Start tatsächlichen Head erneut binden und danach echten Exit samt sofortiger Slotrückgabe sichern. Kein Lauf gestartet. Die folgenden älteren Abschnitte sind Verlauf, keine neue Ausführungserlaubnis.

Der unten zuvor vorbereitete Folgelauf wurde ausdrücklich zugeteilt und ist als ba0n8pidy am2026-09-30T15:52:18Z gestartet. Tatsächlicher Exit101, Slotrückgabe vom Nutzer für15:52:37UTC bestätigt. Neuer Fehler ausschließlich unused import PgConnection, brain-legacy-import.rs:12. Quellursprungc5d2b1f4, keine Verwendung außer dem Import, historischer grüner Headca4a8f2 ohne diesen Namen. Bestehender Autor1180821 entfernt genau den Import. BelegSLOT-E-CLIPPY-NACHWEIS.md, AuftragCLIPPY-IMPORT-FIX.md.

Präziser nächster Bedarf nach tatsächlicher Abgabe und enger statischer Deltaprüfung: unverändert genau der einzelne Paket-Clippy aus Abschnitt1, kein vorausgehender Check, keine Tests/DB/Medien/Runtime. Finalen Fixhead vor Start neu binden. Bisher KEINE dritte Slotzuteilung; die älteren Planabschnitte darunter erlauben keine Ausführung.
Datum: 2026-09-30

# Nächster begrenzter G5-Lauf nach B2-Fixabnahme

Aktueller vorbereiteter Laufhead c809b629d34e01087601db31c23fea2acd3fc088, sauber und gepusht. Er enthält den mechanischen Zweistellenfix d35a11ccfa790d2835e45ed8409910154ac13375 und danach ausschließlich CLIPPY-ZWEISTELLEN-ERGEBNIS.md. Vollständiges Produktdelta gelesen: bestehende mutable Transaktion wird direkt beziehungsweise als &mut tx an denselben lock_source-Aufruf übergeben; die Auto-Deref-Coercion erreicht dieselbe PgConnection. Quell-ID, Advisory-Key, Reihenfolge, await/map_err und Transaktionsgrenze bleiben unverändert. Enges unabhängiges Nachreview1179269 abgeschlossen mit GO9070ba94d0730c94ed1ce63c6e0b3be6fb597aac,0 Befunde, gelesen und sauber/gepusht. Keine Gesamtquellrunde; derselbe Reviewer nach Abschluss gesettelt1179862.

Der vorige einzelne Clippy b1o05m5vm aufb3523fa endete mit101, Slot unmittelbar zurückgegeben. Beleg SLOT-D-CLIPPY-NACHWEIS.md einschließlich Baselinezuordnung e878530 und vollständigem Loghash. Für den vorbereiteten Folgehead wird exakt derselbe unten genannte Clippy benötigt, kein Zusatzcheck und keine weitere Befehlsfolge. Der aktuelle Clipprüfslot gehört dem Integrator; bisher keine neue Brain-Startfreigabe. Keine Tests, DB oder Runtime mit hineinpacken. Vor Start erneut tatsächlichen Head und reines Berichtsdelta binden; danach echten Exit sichern und Slot sofort zurückgeben.

Adressat: Integrator/Root über die bestehende zentrale BRAIN-G5-BUILD-REQUEST.txt, keine Session-zu-Session-Nachricht. Twitch ist laut Nutzer13:44:20UTC mit7/7 Quellen/Engine gesund, alle vier ELFs a82, Opsa685/main und Migration155. Reihenfolge des Integrators bleibt STT, Chat/Titel, Clip-Social/Context; keine eigenmächtige Konkurrenz durch Brain.

## Quellbindung und tatsächlicher Stand

Letzter vollständiger sauberer und gepushter Quellstand b3523fa246c39249f42cd58343f1785445da0323. Darin B2-Produktfixe878530, CI-Fix0c56f85 und enger Testquellenfix89491987a52053c94f98a7d3430d7139570caa95. Der letztgenannte Fix ergänzt ausschließlich273 Zeilen im bestehenden ignorierten Importertest. Keine Änderung am Produktcode oder Runner. Autorbericht gelesen und separat gepusht.

Derselbe unabhängige Reviewer1175579 hat15:15:40 statisch GO gemeldet; der vollständige Bericht B2-R1-SYNCHRONISATION-NACHREVIEW.md ist gelesen,0 Befunde. Er schließt den letzten R1-Testquellenrest mit beobachteten Advisory-Blockierbeziehungen für beide Writer, Commit/Rollback, zwei Quellen und leere Batches. Abschlusscommit dcfee1d81abb5d3e85d1cdd0fe3e167d89d3a462 um15:24:45UTC, sauber und gepusht bestätigt. Keine Runtimefreigabe daraus ableiten. B1-CI und R2/R3 bleiben abgenommen.

Beantragt wird genau der folgende erste Paket-Clippy-Lauf, kein vorgeschalteter Check und keine automatische Testkette. Der gemeldete freie Twitch-Compiler ist noch keine konkrete Zuteilung. User-Timer PID1280937 ist separat als vorhandener Datenrefresh eingeordnet und nicht unser G5-Lauf; Detailbeleg BESTANDSLAUF-1511.md. Die vier eigenen Worktrees samt Cache wurden gegen reguläres Git-Cleanup gesperrt und im genannten Cleanup-Register als aktiv vermerkt.

## Angefragte erste Klasse: Compiler, danach getrennte Unitläufe

Arbeitsverzeichnis für alle Aufrufe: /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust. Ausschließlich vorhandene Toolchain1.97.1, Cargo-Home /home/nathanael/.cargo und Targetcache /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target. Kein Fetch, kein Releasebuild, kein neuer Cache, kein Betreiber-ENV-Setup. Kapazität vor tatsächlicher Ausführung erneut prüfen, keine feste RAMzusage. Ein Cargojob, mögliche interne Compiler-/Linkerthreads. Kein Start/Restart von Produktdiensten.

1. Zunächst gezieltes Clippy der zwei tatsächlich angefassten Produktcrates; ersetzt einen doppelten vorausgehenden Check für diesen Slot:

```sh
/home/nathanael/.cargo/bin/cargo +1.97.1 clippy --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-storage -p brain-legacy-import --all-targets --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target -- -D warnings
```

2. Nach grünem Compilerbeleg getrennt nichtignorierte Librarytests:

```sh
/home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-storage -p brain-legacy-import --lib --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target -- --test-threads=1
```

3. Getrennt nichtignorierte Importer-Binärtests:

```sh
/home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-legacy-import --bin brain-legacy-import --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target -- --test-threads=1
```

Keine automatische Verkettung, kein --ignored. Vollständige lokale Logs, tatsächliche Exits und SHA pro Lauf erhalten, Slot nach zugeteiltem Umfang sofort zurückgeben. Die endgültigen Testwirkungen müssen am Fixhead statisch geprüft sein; neue nichtignorierte DB-/Prozessfälle dürfen nicht still in diese Klasse geraten. Historische954/74 gelten nicht für den Fix.

## Zweite Klasse erst nach fertigem Fixtureanschluss

Vorhandener Serve-Harness, kein neuer Runner:

```sh
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/test_brain_serve.sh /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

R3 ist aufe878530 statisch abgenommen: separate brain_cutover_test-Datenbank, ignorierter Binärtest tests::same_database_archive_to_core_requires_bound_private_snapshot exakt vor Serve-E2E, abschließende Prüfung der unverändert leeren brain_schema_test-Fixture, bestehendes Clustercleanup. Sieben serielle Cargoaufrufe. Der ganze Runner enthält echte private PostgreSQL-Schreibvorgänge, lokale Importer-/Serve-/Provider-/SCRAM-Fixtures und 1.800 Lastanfragen, je600 bei8/16/32 Workern. Port55439, max_connections12 und shared_buffers16MB unverändert. Der Cutoverfall verwendet eine synthetische privilegierte Testrolle mit require_auth=false, ersetzt also keine getrennten produktiven Minimalrollen-/Secret-Exec-Beweise. Vor Zuteilung zusätzlich den neuen R1-Synchronisationsfix samt tatsächlicher Testwirkung abnehmen; keine Budgeterhöhung, kein Produktzugriff und kein freistehender Zusatzaufruf gegen eine zufällig vorhandene Fixture.

## Sachliche Datenbelege, getrennt von Laufzuteilung

ARCHIV-METADATEN-1347.md belegt vier erfolgreiche Read-only-Prüfungen mit Counts/Hashes und ROLLBACK: Zielbrain leer, OIDs und Schemahash,905 Entities/3778 Aliase/32821 Events und Enrichments/348 Patch-IDs, bestehende Leserrechte; alle348 aktuellen Pilotpatchheads privat brain.legacy.review. Keine Rohinhalte ausgegeben. Fehlend bleiben echter Importer-Snapshotfingerprint samt Label/Epoch und gültige vollständige Rechte-/Widerrufs-/Tombstoneinventare. Keine Platzhalterbefüllung oder Importfreigabe aus Auditwerten. Read-only-Fortsetzung ist bereits autorisiert und benötigt keine pauschale Einzelrückfrage.
