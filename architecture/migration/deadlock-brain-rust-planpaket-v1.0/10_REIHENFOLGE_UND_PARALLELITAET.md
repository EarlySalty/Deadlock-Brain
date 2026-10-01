# Welche Prompts in welcher Reihenfolge?

**Nicht alle gleichzeitig implementieren. Parallelität ist nach Abhängigkeiten erlaubt, nicht nach Chatnummer.** Ein Chat darf vorher zur Vorbereitung geöffnet werden, bekommt dann aber ausdrücklich nur einen Lese-/Entwurfsauftrag. Kein fehlender Vertrag wird durch eine private Ersatzarchitektur überbrückt.

## Praktischer Start

| Schritt | Freigegebene Chats | Erforderliches Ergebnis vor dem nächsten Schritt |
|---|---|---|
| 1 | **00 Koordination** | Planversion, Quellen, STATUS, Besitzer und Arbeitsgrenzen angelegt |
| 2 | **01 Inventar + 10 Testdesign parallel** | G0: belegter Code-/Datenbestand, laufende Pfade, Test- und Lastprofil |
| 3 | **02 Fundament + 03 Schemaentwurf** | G1: integrierte Rust-/API-/Quellen-/Fact-/Replayverträge und verbindliche Pfadbesitzer |
| 4 | **03, 04, 05, 06, 07, 12, 13, 14 parallel möglich** | Eigene geprüfte Module; 08/09 dürfen gegen feste Mocks vorbereiten; 10 prüft laufend |
| 5 | **08 echte Kernelintegration + 09 echte Consumerintegration**, mit den jeweiligen Modulbesitzern | G2: kleiner echter Durchstich; danach vollständige Verarbeitung und G3 |
| 6 | **10 Endabnahme + 11 Cutover-/Restoreprobe**, Modulbesitzer nur für Fehlerbehebung | G4: vollständige Qualität, Sicherheits-/Lasttests und getesteter Rückweg |
| 7 | **11 Produktivwechsel**, Freigabe durch Betreiber mit 00 | G5; anschließend Betriebskontrolle und G6 Legacy-Ende |

Schritt 4 muss nicht vollständig fertig sein, bevor Schritt 5 für einen kleinen Pilot beginnt. Entscheidend sind die benötigten **integrierten echten Abhängigkeiten dieses Pilots**. Pilot zuerst; vollständige Kataloge, Historien und Replays danach. G2 ist keine Abnahme der Gesamtmenge.

## Genaue Startmatrix

| Chat | Früh erlaubte Vorbereitung | Implementierung ab | Echte Abnahme benötigt |
|---|---|---|---|
| 00 | sofort | Plan-/Integrationsführung | tatsächliche Übergaben und Gateberichte |
| 01 | nach Initialisierung von 00 | Inventar/Baseline, read-only | Zugriff und Belege; keine erfundenen Scans |
| 02 | nach G0 | Fundament und Contracttests | 01-Inventar, mit 03 abgestimmte Datenverträge |
| 03 | nach G0 mit 02: Schemaentwurf | Store/Tools nach G1 | echte Quellen; G2 vor Vollmigration; 04/12/13/14 für Deltas |
| 04 | Quellen-/Jobinventar nach G0 | nach G1 | 03-Storage und Connectoren; 07 nur für freigegebene Modellaufgaben |
| 05 | Feature-/Goldenanalyse nach G0 | nach G1 | 03-Store + 12/13-Facts; 14-Observations für empirische Erweiterung |
| 06 | Probe/Benchmarkentwurf nach G0 | nach G1 | 03/04-Dokumente, 05 strukturierte/Graph-Evidenz und 07-Embeddingport |
| 07 | Vertrags-/Fixtureprüfung nach G0 | nach G1 | erlaubter echter Providerzugang für Live-Verträge, 10 für Jev-Freigabe |
| 08 | Ablaufentwurf; nach G1 feste Mocks | nach G1 gegen fixierte Ports | echte 03–07-Module und freigegebene 12–14-Daten für G2/G3 |
| 09 | Consumer-/Commandinventar nach G0 | nach G1 gegen Contract-Test-API | echte 08-API und Consumerchecks; keine Produktivumstellung |
| 10 | sofort mit 01: Testdesign | paketlokale Tests laufend | Gesamtstack, Gesamtdaten und Zielhardware für G4 |
| 11 | Runbook/Restoreentwurf nach G1 | isolierte Stagingproben nach G2 | G4 + ausdrückliche Freigabe für Produktion; G5 vor Legacy-Ende |
| 12 | Wiki-Discovery/erlaubte Fixtures nach G0 | nach G1 | 03-Store, 04-Worker, 05-Fachnormalisierung/Karteninput |
| 13 | Repo-/API-/Schemafixtures nach G0 | nach G1 | 03-Store, 04-Jobs, stabile Source-/Schema-Ports |
| 14 | Replay-Zugang/Referenzfixtures nach G0 | nach G1 | 03-Store, 04-Queue, 13-Schema-Revision; 05/10 für empirische Abnahme |

Vorbereitung vor G1 verändert keine gemeinsamen Contracts, keine Produktivdaten und keine Root-Dependencies. Rechtefreigabe gilt auch für Testfixtures und Abrufe.

## Abhängigkeitskette ohne vermeintlichen Alles-gleichzeitig-Modus

```text
00 initialisiert
        |
01 Inventar + 10 Testdesign
        |
G0 -> 02 Contracts + 03 Schema -> G1 (integrierter Commit)
        |
        +-- 03 Store/Migration -----------+
        +-- 04 Worker/Feeder -------------+
        +-- 05 Fachlogik/Builds ----------+
        +-- 06 Suche --------------------+--> 08 echter Kernel --> 09 echte Adapter
        +-- 07 Provider/Jev --------------+             |
        +-- 12 Wiki/Karten ---------------+             G2 Pilot
        +-- 13 API/Git/Schema ------------+             |
        +-- 14 Replays -------------------+       Gesamtdaten + G3
                                                        |
                                           10 Abnahme + 11 Probe
                                                        |
                                                       G4
                                                        |
                                          11 freigegebener Cutover
                                                        |
                                                    G5 -> G6
```

Diese Textskizze ist ein Start-/Integrationsplan, keine Zusicherung unabhängiger Laufzeitdienste. Alle späteren Module dürfen Tests gegen dieselben fixierten Ports nutzen. Mocking löst eine Arbeitsabhängigkeit, aber keine Abnahmeabhängigkeit.

## Was nicht parallel geändert wird

| Gemeinsames Artefakt | Besitzer / Integrationsweg |
|---|---|
| Root-Workspace, Toolchain, Lockfile, zentrale CI | 02; Anforderungen von Modulbesitzern, Integration mit 00 |
| Öffentliche Contracts / gemeinsame Typen | 02 mit zuständigen Fachbesitzern; CR vor Änderung |
| DB-Schema und Migrationsnummern | 03; andere liefern Schemaanforderungen |
| Gesamt-STATUS, Freigaben, Pfad-/Ownerregister | 00 |
| Produktiver Writer, aktive Knowledge-Version, Deployment | 11 nur nach Gate und ausdrücklicher Freigabe |

Ein Chat pro Branch/Worktree; kein gemeinsamer schreibbarer Checkout. Isolierte Testdatenbanken bzw. Schemas/Queues nutzen. Parallel entwickelte PRs **nacheinander** in den Integrationsbranch übernehmen. Dabei zuerst Verträge, dann tatsächlich benötigte Implementierungen integrieren; nach Rebase die betroffenen Tests wiederholen. Kein Merge nur deshalb, weil der einzelne Chat "fertig" sagt.

## Was du jedem gestarteten Chat mitgibst

Den eigenen Auftrag, Plan v1.0, `STATUS.md`, Basis-Commit, Contract-/DB-Schema-Version, reale erlaubte Pfade, freigegebene Arbeitsart (`prepare_only` oder `implement`) und benötigte Übergaben. Vorlage: [CHAT_START](vorlagen/CHAT_START.md).

Bei nur wenig Koordinationskapazität zunächst wenige implementierende Chats gleichzeitig betreiben und die nächsten nach integrierten Übergaben öffnen. Die Tabelle zeigt **erlaubte** Parallelität, keinen Zwang, acht Implementierungen gleichzeitig zu betreuen.

**Sofortiger nächster Schritt:** 00 starten, danach 01 und 10. Keine Vollmigration und kein Abschalten alter Repositories allein durch diese Prompts.
