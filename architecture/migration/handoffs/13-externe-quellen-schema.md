# Übergabe · S13: Externe Quellen, APIverträge und Schemaüberwachung

Status: **blockiert für Implementierung; Startprüfung vorbereitet**.
Arbeitsmodus: `prepare_only`. Keine Freigabe von G0, G1 oder einer nächsten Arbeitswelle.
Basis-Commit: `30326512568b7370524956839100462ba71bdb92` (`origin/main`).
Ergebnis-Branch: `migration/s13-external-sources-preparation-20260924`.
Ergebnis-Commit / PR: vollständiger Head-SHA, PR-Nummer und aktuelle Actions-Nachweise werden im zugehörigen Draft-PR festgehalten; diese Datei erteilt keine Mergefreigabe.
Geprüfter Codezustand: oben genannter Basiscommit; die eigenen Änderungen sind ausschließlich die sieben S13-Übergabedateien. Prüfungen nach Commit sind dem exakten SHA im PR zugeordnet.
Contract-/DB-Schema-Version: offen, keine private Ersatzversion definiert.
Plan: v1.0; ZIP-SHA-256 `945be6983ff0235eb0ec36b451f9613ea567e0962ca5658364e8665980177716`.
Source-/Corpus-/Modellversion: drei Git-Metadatenpins und ein OpenAPI-Abrufhash in `s13/UPSTREAM_PINS.json`; kein neues Corpus, Modell, Knowledge-Release oder geprüfter Parserstand.
Betroffene Anforderungen: R32, R38, R39, R42–R48, R52, R54, R55, R58, R60. **Keine dieser Implementierungsanforderungen wird durch diese Vorbereitung als vollständig bestanden markiert.**

## Ergebnis und konkrete Änderungen

Die Startbedingung aus `chats/13_EXTERNE_QUELLEN_SCHEMA.md` ist nicht erfüllt: Der integrierte Status führt G0/G1 und Contract-/Schemaversion weiterhin offen. Deshalb erfolgt nur die nicht schreibende Startprüfung vorhandener Adapter und öffentlicher Quellenmetadaten. Eine produktive S13-Implementierung oder eine vollständige freigegebene Quellenprüfung nach G0 wird nicht behauptet.

| Datei unter `architecture/migration/handoffs/` | Inhalt und Reichweite |
|---|---|
| `13-externe-quellen-schema.md` | Diese Übergabe mit Grenzen und zuständigen Folgepaketen |
| `s13/BEFUNDE.md` | Sechs belegte Befunde am integrierten Code; die Mängel bleiben in diesem PR unbehandelt |
| `s13/QUELLENREGISTER.csv` | Alle 26 Quellen/Kandidaten des Planpakets mit unveränderten Rollen, Scopeklassen und Besitzern; ungeprüfte Verfügbarkeit, Rechte und Revisionen bleiben ausdrücklich offen |
| `s13/UPSTREAM_PINS.json` | Drei beobachtete vollständige Gitrevisionen, Elternrevision von deadlock-data, Abrufhash/Größe der API-Spezifikation und getrennte Ausfallbeobachtungen |
| `s13/API_CONTRACT_OBSERVATION.json` | Aus dem echten OpenAPI-Abruf abgeleitete Metadaten: neun bisherige Assets-Routen, neun aktuelle Operationen und ausgewählte Feldtypen; kein Runtimevertrag oder freigegebenes Payloadfixture |
| `s13/AKZEPTANZFAELLE.json` | 30 geplante Abnahmefälle für G2–G4. Nicht ausführbar und noch nicht ausgeführt |
| `s13/CR-13-001-CONTRACT-STORE-PFADE.md` | Konkrete Anforderungen an Source-v2, vorhandenen Store, HTTP-/Jobport und eindeutige Pfadbesitzer |

Öffentliche Schnittstellen, Rustmodule, Dependencies, Lockfiles, SQL, CI, zentrale STATUS-/Gate-/Ownerdateien und Produktionskonfiguration bleiben unverändert. Es entsteht kein zweiter Store, Scheduler, Parser, Quellenadapter oder LLM-Pfad.

## Wichtigste technische Befunde

Der bestehende Assets-Hostname ließ sich beim HTTPS-HEAD-Test von diesem Host nicht auflösen. Die aktuelle API-Spezifikation zeigt entsprechende Assets-Kandidaten unter `/v1/assets/`; die beiden bisherigen Rawrouten haben noch keinen belegten Ersatz. Ein Hostnamewechsel allein würde keine vollständige Adapterparität liefern.

Assets ohne ID können im vorhandenen Code eine Positionsnummer erhalten; falsche Container können in Assets- und normalen Match-Metadatenpfaden als leere Ergebnisse erscheinen. Die API-Antworten werden vor `write_raw` bereits als JSON dekodiert und neu serialisiert. Der vorhandene Hash ist daher nicht automatisch ein Hash der unveränderten HTTP-Antwort.

Der Gitimport klont mit `--depth 1` und liest einen Arbeitsbaum statt einer explizit gewählten historischen Revision. Gitzeit, Parserrevision und belegte Patchgültigkeit brauchen getrennte Behandlung. Vorhandene Gitmetadaten allein belegen weder einen vollständigen Historienimport noch Quellenrechte.

## Nachweise und nicht ausgeführte Prüfungen

| Prüfung | Befehl / Umgebung | Tatsächlicher Stand |
|---|---|---|
| Basis und fremdes WIP | `git status --short --branch`, `git log`, `git worktree list`, `git fetch origin` im bestehenden Repo | Eigener sauberer Worktree ab `3032651`; fremder schmutziger Arbeitsbaum unverändert |
| Gemeinsame Verträge | Lesen von STATUS/PFAD_OWNER/GATES; Suche nach Source-v2-/Schema-/Reconciliationtypen in den Rust-Crates | G0/G1 offen; gesuchte Typnamen an dieser Basis nicht gefunden |
| Bestandssuche | Graphify mit globalem Graph, danach Originaldateien lesen | Vorhandene Assets-, Git-, Metadaten- und Storepfade identifiziert; kein Neubau |
| Öffentliche API-Spezifikation | `curl --fail --max-time 20 --proto =https --proto-redir =https https://api.deadlock-api.com/openapi.json -o /tmp/brain-s13-openapi-20260924.json` | HTTP 200, 380364 Bytes; SHA-256 und Dateizeit in `UPSTREAM_PINS.json` |
| Bisheriger Assets-Hostname | HTTPS-HEAD auf `/v2/heroes`, max. 15 Sekunden | curl Exit 6, DNS-Auflösung fehlgeschlagen; HTTP `000`, nicht 404 und kein Breaking-Schemaurteil |
| Öffentliche Repositorymetadaten | GitHub-GET der drei in `UPSTREAM_PINS.json` angegebenen Ref-/Commit-URLs | Drei vollständige Revisionen beobachtet; nicht als live deployte APIversion oder vollständige Schemasnapshots ausgegeben |
| Dokumentintegrität | JSON-Parsing, CSV-Abgleich mit dem Paket und Pin-/Abnahmefall-/Routenprüfungen | 18 Struktur- und Konsistenzprüfungen bestanden, Exit 0; 26 Quellen, drei Pins, neun Routen und 30 ausschließlich geplante Abnahmefälle. Template-SHA-256 `a4ccfc63480ce704c61d8be7c87caf8dcf6be75aeec58ff5f099e5f06ca68ff8` stimmt mit dem hochgeladenen Paket überein. Dies sind keine Adaptertests |
| Rust-Format, Clippy, Unit-/Integrationstests lokal | Nicht ausgeführt | Keine Ruständerung. Das ist kein Ersatz für die spätere verpflichtende Adapterprüfung |
| GitHub Actions | Unveränderte vorhandene Workflows am neuen PR | Ergebnis wird am tatsächlichen Head-/Merge-SHA nachgelesen und im PR dokumentiert; hier keine vorweggenommene grüne Abnahme |
| Fachliche S13-Abnahme | Die 30 Fälle aus `AKZEPTANZFAELLE.json` | **0 ausgeführte Adapter-/Watcher-/Historienabnahmefälle**, keine echten Match-, Replay- oder Datenbanktests |

Der OpenAPI-Hash pinnt die beobachteten Bytes, stellt sie aber nicht selbst wieder her. Die Diagnosekopie unter `/tmp` ist kein dauerhafter Raw-/Fixturestore. Ein späterer erneuter Abruf kann einen anderen Inhalt ergeben. Rechtegeprüfte unveränderte Fixtures und ein Restorepfad bleiben vor G2 erforderlich; hier wird kein ungeklärter vollständiger Fremddatenbestand in Git kopiert.

## Folgen

Datenmigration, Kompatibilität und Wiederanlauf: Kein produktiver Schreibzugriff, keine Migration und keine Runtimeänderung. Die bestehenden Funktionen bleiben unverändert, einschließlich der belegten Mängel.

Berechtigungen, Secrets und Egress: Nur öffentliche Dokumentation, Schema- und Repositorymetadaten gelesen. Keine Playerhistorien, Matches, Replays oder Game-Assetpayloads abgerufen. Keine neuen Secrets und keine Freigabe für Veröffentlichung oder Provider-Egress. Öffentliche Sichtbarkeit beziehungsweise eine Code-Lizenz erteilt keine Game-Datenrechte.

Latenz, Ressourcen und Kosten: Keine Benchmark- oder Kapazitätsaussage. Abrufgröße der Spezifikation ist keine Performanceabnahme. Kein Service, Timer, produktiver Watcher oder Bot wurde neu gestartet.

Python-/Legacyfreiheit: Kein eigener Runtimepfad hinzugefügt, keine Fremdruntimeabhängigkeit aufgenommen. Diagnoseauswertung ist kein neuer Produktionsparser. Der bestehende Pythonbestand wurde weder entfernt noch als abgeschaltet behauptet.

## Grenzen und Übergabe an nächste Besitzer

**S00/S01:** G0 anhand Inventar, Rechte und Betriebsnachweisen integrieren; die tatsächlichen S13-Teilpfade eindeutig zuweisen. Diesen Draft-PR erst nach Review als Vorbereitung integrieren, nicht als erfülltes S13/G2/G3/G4 zählen.

**S02/S03/S04:** Den Change Request prüfen, gemeinsame Source-v2-/HTTP-/Store-/Jobverträge und Migrationen samt Tests integrieren. G1 braucht konkrete Contract-/Schema-Versionen und freigegebene Pfade. S13 ändert keine fremden Module stellvertretend.

**S13 nach G1:** Auf die dann integrierte Basis rebasieren. Bestehende Adapter härten, Originalbytes und Provenienz sichern, alle neun Assets-Fälle belegen, commitbezogene Historie und separaten Schemawatcher implementieren. Die 30 Abnahmefälle zunächst rot gegen die relevanten alten Verhaltensweisen prüfen und anschließend mit erlaubten Fixtures, echten Piloten und Ressourcen-/Ausfallnachweisen durchführen.

**S05/S12/S14:** Fachliche Population, Wiki-Karten und Replaydecoder übernehmen ihre Ports; gemeinsame Ursprungsartefakte und gezielte Invalidierung bleiben quellenübergreifend prüfbar. Keine parallelen Schreibsessions in `deadlock_api.rs`.

## Integration durch S00

Merge-Commit: keiner.
Gate-/STATUS-Änderung: keine.
Freigegeben von: niemand in diesem Arbeitspaket.
Neue Arbeitswelle: nicht freigegeben.
Produktionsauslieferung: nicht erfolgt.
