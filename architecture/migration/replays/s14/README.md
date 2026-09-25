# S14: Replayzugang und Observation-Abnahme vorbereiten

Status: **Vorbereitung implementiert und lokal getestet; vollständiges S14 blockiert.**
Keine Runtimeimplementierung, keine Golden-Replays, keine G1/G2/G3/G4-Freigabe.

Basis: `30326512568b7370524956839100462ba71bdb92` (`origin/main`, live neu geprüft).
Branch: `migration/s14-replay-preparation-20260924`.
Plan: Nutzerpaket v1.0 vom 24.09.2026, Chat 14 und `13_REPLAYS_UND_POPULATION.md`.
Contract-/DB-Schemaversion: auf der integrierten Basis offen. Der getrennte
Reviewbranch `review/brain-s01-s09-20260924` bei `2c47086` enthält den Entwurf
`brain.v1`; er ist kein freigegebener integrierter S14-Vertrag.
Arbeitsmodus: additive Vorbereitung zur Prüfung durch S00, keine selbsterteilte
Implementierungsfreigabe. Gemeinsame STATUS-/Owner-/Gate-Dateien bleiben unverändert.

## Geliefert

- `audit/`: ausführbarer dependencyfreier Rust-Prüfer für **Inventarmetadaten**,
  mit Bibliotheks- und CLI-Tests; nicht Mitglied des produktiven Workspaces.
- `CORPUS.tsv` / `CAPABILITIES.tsv`: absichtlich leere echte Corpusregister.
  Keine erfundenen Matches oder synthetischen Daten als echtes Golden-Corpus.
- `DECODER_CANDIDATES.json`: live gelesene vollständige Kandidatenrevisionen,
  tatsächliche Features und transitive Pins; keine ungeprüfte neue Root-Dependency.
- `CAPABILITY_REVIEW.tsv`: sämtliche Pflichtfelder ausdrücklich `unverified`.
- `CR-S14-01.md`: konkrete gemeinsame Contract-, Store-, Job-, Schema-,
  Pfadowner- und CI-Anforderungen, keine private Ersatzschnittstelle.
- `REFERENCE_AND_BUDGET_PLAN.md`: Feldsemantik, Korrupt-/Reparse-/Privacytests,
  Ressourcen- und Holdoutabnahme mit klaren noch nicht erbrachten Nachweisen.
- `TEST_REPORT.md` und `../../handoffs/14-replays-observations.md`: Ergebnisse
  und nächste Besitzer. Die Übergabe liegt relativ zum Migrationswurzelverzeichnis
  unter `architecture/migration/handoffs/14-replays-observations.md`.

## Belegter Ausgangszustand

`architecture/migration/STATUS.md` auf dem Basiscommit weist G0 bis G6 offen aus;
Contract und Schema sind nicht freigegeben. `PFAD_OWNER.csv` überlappt bei S14
mit S04/S13 (`dbrain-sources/src/deadlock_api*`) und S05 (`dbrain-population/**`).
S14 verändert deshalb diese Runtimepfade nicht. Der Hauptcheckout enthält fremde
uncommittete Arbeit, die nicht übernommen, bereinigt oder zurückgesetzt wurde.

`rust/crates/dbrain-sources/src/deadlock_api.rs` konsumiert externe Demo-SQL-Jobs
und normalisiert NDJSON-Ergebnisse. `normalize_demo_rows` setzt u.a. query- und
zeilenbasierte Evidence-IDs. Das ist **kein** eigener `.dem`-Decoder und kein
Nachweis eines stabilen Replay-/Observationvertrags. `dbrain-population` enthält
bereits aggregierte Matchverarbeitung; sie wird nicht durch einen zweiten Store
oder eine zweite Population ersetzt.

Der gezielte read-only Dateiscan (`find`, maximale Tiefe 6) der im Auftrag
bekannten Brain-/2nd-Brain-Checkouts lieferte keine `.dem`, `.dem.bz2`, `.dem.zst`
oder Replaymanifest-Treffer. Einschränkung: kein vollständiges Host-/Volume-/DB-
Inventar; es wurden weder Secret-/Produktivdaten noch fremde Benutzerverzeichnisse
geöffnet. Das beweist nicht, dass nirgends Replays liegen. Im gesichteten Umfang
ist **kein** echter Replayfall mit dokumentierter Nutzungs-/Speicherfreigabe und
verifiziertem Datei-/Hash-/Patchstatus verfügbar. Deshalb wurden keine fremden
Replays heruntergeladen und keine externen Demo-Jobs ausgelöst.

## Offline-Prüfer benutzen

```sh
cargo test --locked --offline --manifest-path architecture/migration/replays/s14/audit/Cargo.toml
cargo run --locked --offline --manifest-path architecture/migration/replays/s14/audit/Cargo.toml -- \
  architecture/migration/replays/s14/CORPUS.tsv \
  architecture/migration/replays/s14/CAPABILITIES.tsv
```

Die zweite Prüfung muss mit dem aktuellen leeren Corpus **Exit 2** liefern.
Exit 0 bedeutet ausschließlich `metadata_complete`, Exit 2 fehlende Nachweise,
Exit 1 ungültige Eingaben. Jede Ergebnisausgabe enthält unveränderlich
`implementation_authorized=false` und `integration_verified=false`.

Ein zukünftiger **vor** Auswahl/Tuning von S10 festgelegter Cutoff kann mit
`--cutoff-epoch <UTC-Unixsekunden>` angegeben werden. Ohne Cutoff bleibt die
Metadatenabnahme blockiert. Der Prüfer zählt mindestens zehn eindeutige reale
Matches mit vollständig deklarierten neun Pflichtcapabilities; alternative
Pilotumfänge brauchen einen expliziten S00/S10-Entscheid, nicht einen stillen
Override. G2 mit einem echten Match wird separat abgenommen; dieses Werkzeug
ist kein G2- oder G3-Gate.

### Format, Grenzen und Datenschutz

TSV mit exakten Headern, LF oder CRLF, keine Kommentare, Quotes oder eingebetteten
Tabs/Zeilenumbrüche. `-` bedeutet unbekannt, niemals Null oder automatisch erlaubt.
Kennungen sind opake ASCII-Tokens bis 128 Bytes (`A-Z a-z 0-9 _ - . :`). Keine URLs,
Pfadangaben, Account-IDs oder Spielernamen für öffentliche Register verwenden.
Ein opakes Format garantiert selbst keine Anonymität; Freigabe vor Veröffentlichung
bleibt erforderlich. Rohlocator/Rechtebelege verbleiben im berechtigten internen
System; das Tool liest und veröffentlicht sie nicht.

Maximal 256 KiB pro Manifest. Lokale vertrauenswürdige reguläre Dateien werden
begrenzt gelesen; erkannte Symlinks, Verzeichnisse und Spezialdateien abgewiesen.
**Keine Sandbox gegen nebenläufig manipulierte Dateisystempfade:** Für produktive
Replayworker sind sichere Dateideskriptoren und Prozess-/Cgroupgrenzen separat zu
implementieren. Das Tool öffnet keine `.dem`, dekomprimiert nichts, hasht keine
Replays, prüft keine Signaturen und bescheinigt keine Rechte oder Parsersemantik.
Es validiert lediglich selbst deklarierte Metadaten und Konsistenz.

Synthetische Testfälle zählen niemals als echte Matches. Mehrere Quellen oder
Parserrevisionen desselben Matches erhöhen das Corpus-n nicht. Matchübergreifende
Rawhash-Kollisionen, widersprüchliche Patch-/Modekennungen, doppelte Case-/Feldkeys
und nicht bekannte Capabilitynamen werden abgewiesen. Holdouts werden nach Match
und Zeit getrennt; Quellenverfügbarkeit zählt zusätzlich zum Spielzeitpunkt.
Alle Pflichtfelder müssen für dieselbe konkrete Case-/Parserrevision komplett
sein; halbe Ergebnisse verschiedener Parser werden nicht zusammengezählt.

`approved`, `verified`, `independent` sind in diesem Inventar **Behauptungen**, die
S00/S10 anhand der internen Belege prüfen müssen. Ein bereinigter Tabellenwert ist
kein Rechtebescheid. Unterstützte/abgeleitete Felder benötigen Referenz, Unit und
passende Zeitbasis; Ableitungen zusätzlich einen Algorithmusbeleg. Unbekannte,
korrelierte, fehlende oder nicht verfügbare Daten können keine grüne echte
Replayabnahme erzeugen. Berichte/Fehler enthalten nur feste Codes und Zähler,
keine eingelesenen Zellwerte, Pfade, Matchkennungen oder rohen Fehlertexte.

## Nächste Integration

S00/S01: zugelassenen internen Replayzugang und Corpusumfang ermitteln; G0 belegen.
S02/S03: gemeinsamen versionierten Replay-/Observationvertrag und G1 integrieren.
S04/S13: begrenzten Jobport und kompatible gepinnte Schemarevision bereitstellen.
Danach S14: echte Rust-Decoderintegration in zugewiesenen Runtimepfaden, zunächst
Raw → Decoder → selektierte Observations für einen erlaubten Match. S05/S10:
Populationübergabe, unabhängiger Referenzvergleich und gemischte Last abnehmen.
