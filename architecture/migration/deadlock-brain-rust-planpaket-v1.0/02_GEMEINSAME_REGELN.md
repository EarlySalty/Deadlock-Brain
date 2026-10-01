# Gemeinsame Regeln für alle Chats

## Verbindliche Grenzen

1. Eigene produktive Backend-Logik ist Rust. Keine neue Python-Brücke, kein permanenter Python-Worker, kein Python-basierter Inferenz-Sidecar als versteckter Kern. Optionale lokale Hilfsskripte stehen in einer Ausnahmeinventarliste mit Zweck, Besitzer und Ablaufdatum.
2. Ein Answer Kernel. Adapter dürfen Auth-/Transport-/Plattformfunktionen enthalten, aber keine eigenen Modellzugänge, Retrievalstrategien oder Faktenprompts.
3. Jede Datenquelle erhält ein Inventar, eine Herkunft, eine Berechtigung und einen Migrationsentscheid. Kein „alles kopiert“ ohne Abgleich.
4. Der Code-Stand ist maßgeblich für Ist-Behauptungen. Recherche und historische Notizen sind Anhaltspunkte, kein Beweis eines heute laufenden Services. Jev- und Provider-APIs gegen aktuelle Primärdokumentation prüfen.
5. Vorhandene Funktionen nicht stillschweigend weglassen. Jede Abweichung vom Altverhalten wird als Bugfix, bewusste Produktänderung oder Blocker dokumentiert.
6. Sicherheitsprüfung schlägt Performance. Keine automatische Sichtbarkeitserweiterung durch AI, Deduplikation, Feeder, Caches oder Rollback.

## Arbeitsweise in getrennten Chats

Jeder Chat startet mit seinem Auftrag, dem zuletzt integrierten Basis-Commit, der Contract-Version, relevanten ADRs und einer Liste der benötigten Übergaben. Kein vermeintliches Gedächtnis zwischen Chats voraussetzen.

Ein Branch/Worktree pro Arbeitspaket, beispielsweise `migration/06-retrieval`. Nur der Besitzer verändert seine vorgesehenen Pfade. Änderungen außerhalb des Bereichs werden über eine kleine Schnittstellenanforderung beantragt und vom Besitzer umgesetzt oder ausdrücklich freigegeben. Gemeinsame Root-Manifeste, Lockfile-Konflikte, globale Workflows, Vertragsdateien und Migrationsnummern werden nicht parallel ungeordnet editiert.

Chat 00 koordiniert. Chat 02 besitzt anfangs Workspace, gemeinsame Verträge und zentrale CI-Grunddateien. Chat 03 besitzt SQL-Schema und Migrationsnummern. Chat 10 liefert Qualitätsregeln und Tests; globale CI-Änderungen werden mit 02 integriert. Chat 11 verändert Deployment-/Cutover-Dateien, aber keine fachlichen Contracts im Alleingang.

Eigene Crate-Manifeste können vom Modulbesitzer geändert werden. Die neu aufgelöste Dependency-/Lockfile-Version wird im Integrationsbranch serialisiert und geprüft. Keine gewaltsamen Resets fremder Arbeit.

## Pro Änderung erforderlich

Ziel und konkrete Dateiliste; kompatible Contract-/Schema-Version; eigene Tests plus betroffene Integrationstests; reproduzierbarer Testbefehl mit tatsächlichem Ergebnis; relevante Performance-/Sicherheitsfolgen; Übergabe mit Commit/PR und offenen Blockern.

Standardprüfungen nach vorhandenem Fundament:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --workspace --release --locked
```

Featurekombinationen werden zusätzlich über eine freigegebene Matrix getestet, nicht pauschal mit inkompatiblen `--all-features`. Zielplattformen, MSRV und native Abhängigkeiten werden explizit geprüft. Ein erfolgreicher Mocktest ersetzt weder einen echten Provider-Vertragstest noch einen Lasttest.

## Done und Blocked

`done` bedeutet: Änderungen integriert, relevante Checks bestanden, Übergabe vorhanden und Akzeptanzkriterien belegt. „Code vorgeschlagen“, „Test nicht ausgeführt“ und „läuft mit Mock“ sind andere Zustände.

Bei einem fehlenden Port zunächst gegen den fixierten Vertrag arbeiten; keine zweite private Schnittstelle erfinden. Kritische Unbekannte mit klarer Auswirkung als Blocker melden. Keine destruktive Operation, Löschung, Repo-Archivierung oder Produktivumschaltung nur aus einer Vermutung heraus ausführen.

## Daten und Secrets

Produktive Rohdaten und sensible Evalbeispiele nicht in offene Chats, Logs oder öffentliche CI-Artefakte schreiben. Testdaten anonymisieren oder synthetisch erzeugen, Herkunft der Referenztests trotzdem dokumentieren. Ein privater Retrieval-Scope schützt keine schon öffentlich eingecheckte Datei. Provider-Egress ist eine eigene Prüfung zusätzlich zur Benutzerberechtigung.

## Einheitliche Übergabe

Die Vorlage `vorlagen/UEBERGABE.md` verwenden. Ergebnisse nach `architecture/migration/handoffs/NN-<paket>.md` ablegen; sensible Artefakte nur über berechtigte interne Referenzen verknüpfen. Chat 00 aktualisiert STATUS und freigegebene Entscheidungen. Der nächste Chat erhält die relevanten Dateien und den integrierten Commit, nicht nur eine Zusammenfassung im Chatverlauf.


## Zusätzliche Grenzen für Wiki, Datenfeeds und Replays

Chat 04 besitzt gemeinsame Worker-/Job-/Feeder-Infrastruktur, nicht sämtliche fachlichen Parser. 12 besitzt Wiki-Discovery/Parser und die Wissenskarten-Projektion; 13 externe API-/Git-/Schema-Adapter; 14 Replaydecoder und Observation-Normalisierung. 05 besitzt Fachnormalisierung, Regeln, Formeln, Synergien, Reasoning und Population/Learning. Übergaben nutzen dieselben Verträge aus 02 und denselben Store aus 03.

Vor G1 legt 00/02 eine reale Dateieigentümerschaft fest (`vorlagen/PFAD_OWNER.csv`). Die im Plan gezeigten Modulnamen sind nicht der Beweis, dass diese Pfade schon existieren. Allein 03 integriert DB-Migrationen; keine gleichzeitig schreibenden Migrationstests auf derselben Testdatenbank. Jeder Arbeitschat nutzt eigenes Branch/Worktree und isolierte Testdaten/Queues.

Schema-, Auth-, Release- oder Pflicht-Scope-Änderung invalidiert die betroffenen Gates bis zur erneuten Prüfung. Gegen Testports bestandene Tests sind `mock_verified`, nicht `integration_verified`. Ein zweiter Chat darf nach Freigabe vorbereiten, aber fehlende echte Daten niemals durch erfundene Fixtures als produktiven Erfolg ausweisen.

Quellenprogramme, Wiki-Lua/Templates und extrahierte Ausdrücke nicht ungeprüft ausführen. Eigene reguläre Parser, Contract-Prüfer und Worker sind Rust; fremde Offline-Referenzen bleiben außerhalb des notwendigen Produktions-/Rebuildpfads. Rohreplays und private Projektdaten nicht aufgrund vermeintlicher Anonymisierung veröffentlichen.
