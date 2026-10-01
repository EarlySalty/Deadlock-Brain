# Externe Repositories, Datenfeeds und Schemaüberwachung

**Grundlage:** U4 „Priorisierte Repository-Landschaft“, „Integrationsarchitektur“, „Risiken“, „Nächste Schritte“. Die folgende Matrix ist ein Integrationsplan aus dieser Quelle, kein erneut erhobener aktueller Repo-/Lizenzstatus. Bestehende Adapter zuerst prüfen und härten, nicht blind neu schreiben.

## 1. Festgelegte Datenfamilien und Zuständigkeiten

03/02 besitzen Source Contract/Store; 04 gemeinsame Jobs; 13 API-/Git-/Schemaquellen; 12 die Wiki-Seiten; 14 Replayparsing; 05 fachliche Diff-/Reasoner-/Population-Auswertung. 10 prüft unabhängig; 09 veröffentlicht nur freigegebene Knowledge-Exports.

Pflicht-Datenfamilien dieses Ausbaus: strukturierter gegenwärtiger Game-State, historische Game-Datensnapshots, offizielle Änderungsnachweise, Quellen-/Schemaänderungen, beobachtete Match-/Populationdaten und validierte Replay-Beobachtungen. Sekundäre Tools bleiben Prüf-/Referenzkandidaten; jedes erhält eine Entscheidung im Register. Ein Ersatz einer Pflichtquelle verlangt eine dokumentierte Gleichwertigkeits- oder Scopeentscheidung.

## 2. Vollständige Einordnung der in U4 priorisierten Projekte

| Projekt/Kandidat aus U4 | Geplante Rolle | Owner | Grenze / Abnahme |
|---|---|---|---|
| `deadlock-api/deadlock-api` | Match-/Hero-/Population-Feed | 13 → 05 | vorhandenen Adapter prüfen; Sampling, Abrufstatus, Zeitfenster, Patch und APIvertrag festhalten |
| `deadlock-api/deadlock-api-assets` | strukturierter Game-State und Extraktionsreferenz | 13 | externe Ergebnisse konsumieren; Client-/Gameversion und Feldprovenienz; keine Python-Pipeline als eigener Kern |
| `deadlock-wiki/deadlock-data` | Git-gepinnte historische Snapshots/Diffs | 13 | Commit/Pfad/Hash/Generator, Game-Zeit getrennt; Idempotenz und Reparse |
| `deadlock-wiki/deadbot` | Herkunfts-/Parserreferenz für obige Daten | 13 | Derivationskette; nötige eigene Transformierung in Rust, kein notwendiger Pythonprozess |
| `SteamTracking/GameTracking-Deadlock` | Roh-/Proto-/Gamefile-Change-Signal | 13 | Rechte/Verfügbarkeit prüfen; Referenz/Hash statt pauschaler Repo-Kopie |
| `deadlock-api/openapi-clients` | Vertrags-/Codegenerierungsreferenz | 13 mit 02 | Versionen pinnen, verbrauchte Felder testen; kein unkontrolliertes tägliches Dependency-Update |
| `deadlock-api/haste` | Kandidat für Rust-Replaydecoder | 14 | tatsächliche Felder/Patches testen, stabile eigene Observations statt fremde Rohtypen exportieren |
| `ValveResourceFormat/SchemaExplorer` | Source-2-Feld-/Enum-/Schemaänderungen | 13 | Schemahash und Mapping-Abhängigkeiten; Änderung ist nicht automatisch ein Gameplay-Fakt |
| `deadlock-api/valveprotos-rs` | Kandidat für Rust-Protokolltypen | 14 mit 13 | Version/Feature pinnen, unbekannte Felder/Enums sichtbar, keine stillen Schema-Upgrades |
| `ValveResourceFormat/ValveResourceFormat` | optionale Offline-Extraktions-/Schema-Referenz | 13 | kein notwendiger Fremd-Runtimeprozess; Code und Game-Testdateien separat prüfen |
| `saul/demofile-net` | optionale Golden-Replay-Gegenprüfung | 14/10 | Referenzergebnisse versionieren; kein notwendiger .NET-Produktionspfad |
| `skadistats/clarity` | alternative Golden-Replay-Gegenprüfung | 14/10 | gleiche Semantik/zeitliche Toleranzen, kein notwendiger JVM-Produktionspfad |
| `deadlock-api/deadlock-api-ingest` | Ingest-/Discovery-/Idempotenzreferenz | 13/14 mit 04 | bestehende eigene Queue weiterverwenden; keine zweite Jobplattform |
| `0xThiagoAmaral/deadlock-open-assets` | sekundäre Codename-/Assetmapping-Referenz | 13 | Game-Assetrechte getrennt; keine kanonischen Stats nur wegen eines vorbereiteten Dumps |
| `Zehmosu/kv3parser` | optionale Parser-Testreferenz | 13 | nicht als benötigte Python-Runtime übernehmen |

Weitere U4-Quellen ebenfalls im Register: offizielle Patchnotes/Steam- oder Forum-Änderungen (13), Entwicklerposts vs. Communityclaims (13/05), Google Sheet und Statlocker als kontextgebundene Zusatzdaten (13/05), Reddit als Hypothesenquelle (13/05), Deadlock.io als sekundärer Plausibilitätscheck (13). Alte alternative `deadlock-data`-Projekte und Consumerportale nur prüfen, wenn sie einen belegbaren zusätzlichen Datenwert liefern; nicht als unbemerkte kanonische Dublette aufnehmen. Cheats/Memory-Manipulationsprojekte sind außerhalb dieses Wissensplans.

Eigene Repos gehören ins Inventar: Brain, Docs, Second Brain, Twitch sowie `Deadlock-Bots`, sofern als Consumer tatsächlich vorhanden. Deren Brain-relevante Serverlogik/Adapter im Monorepo konsolidieren; fremde Codebasen werden dagegen **nicht** unter das Zielrepo zusammenkopiert. U4s erwähnte Support-Evals und Build-Publishing-Verträge prüfen, erhalten und in die gemeinsame Abnahme übernehmen.

## 3. Source Contract v2 und Ableitungsfamilien

Verbindliche Felder in [06](06_VERTRAEGE_UND_GRENZEN.md). Mindestens Source-ID/-Typ/-Revision, Abruf-/Beobachtungszeit, Hash, Parsername/-version, Quellpfad/Verweis, Game-Version soweit bekannt, Trust-Klasse, Ableitungsfamilie, Upstreamreferenzen, Rechte und Prüfstatus.

Zusätzlich zwischen gemeinsamem Ursprungsartefakt und gemeinsamem Parser unterscheiden. Zwei Parser können denselben Spielstand unterschiedlich lesen; zwei Portale können denselben Parseroutput spiegeln. Ein einfacher Quellenzähler beweist daher keine unabhängige Bestätigung. Provenienz als nachvollziehbare Kette/gerichteter Graph, nicht nur ein Textfeld `source`.

Raw unverändert speichern; Normalisierung in reproduzierbarem Staging. Schema-/Semantikprüfung und Reconciliation vor Veröffentlichung. Ungeklärte widersprüchliche Pflichtwerte bleiben quarantiniert und blockieren betroffene Builds/Knowledge-Freigaben. Letzte konsistente Version mit ausgewiesenem Aktualitätsstatus weiter nutzbar, wenn aktuelle ACLs und Produktregeln es zulassen.

## 4. API-Verträge ohne unzuverlässige CI

Genutzte Endpoint-/Feld-/Enum-/Unit-Verträge aus dem tatsächlich eingesetzten API-Stand pinnen. U4 meldet einen Assets-Hostnamewechsel; diesen Bericht als Anlass zur Prüfung, nicht als ungeprüfte aktuelle URLkonfiguration übernehmen.

Deterministisches PR-Gate nutzt gepinnte Spezifikation, Fixtures und Adaptertests; es hängt nicht von einem gerade erreichbaren Upstream ab. Separater Rust-Watcher prüft Upstreamdrift und legt einen geprüften neuen Snapshot/Änderungsvorschlag an. Netzwerkfehler sind „Quelle nicht erreichbar“, nicht „keine Änderung“ oder automatisch „Breaking Change“.

Breaking Changes in tatsächlich verwendeten Feldern verhindern Veröffentlichung des betroffenen Imports. Additive unbekannte Felder dürfen gemäß dokumentierter Kompatibilitätsregel weiterlaufen. Nicht nur JSON-Typen vergleichen: Units, ID-Mappings, Enum-Bedeutung und Semantik durch Sentinel-/Fixturetests absichern. Vollständiger OpenAPI-Diff ohne Auswirkungsanalyse genügt nicht.

Generierte Rust-Clients können übernommen werden, wenn Lizenz, Größe, Version und Coverage passen; alternativ bestehende Clients härten. Eigene Wiretypen nach innen stabil halten. Keine automatische Änderung des Lockfiles auf jedem Upstreamcron.

## 5. Historie und Parserwechsel unterscheiden

Ein Commit-A→B-Diff nennt Entity, Feld, alter/neuer Wert, Units, Mode, Upstreamrevision und Parser-/Generatorversion. Gitcommit-Zeit ist keine Patchzeit. Veröffentlichte Versionszuordnung mit Beleg erfassen, unbekannte Gültigkeit nicht schätzen.

Vier prüfbare Varianten: gleiche Raw-Daten/gleicher Parser → gleiches Ergebnis; neue Raw-Daten/gleicher Parser → möglicher Quelldiff; gleiche Raw-Daten/neuer Parser → Parserkorrektur; neue Raw-Daten/neuer Parser → Faktoren getrennt untersuchen. Ein Parserfix darf nicht ungeprüft als Balancepatch publiziert werden.

Datenhistorie nur soweit der zugelassene Upstream sie tatsächlich enthält. Fehlende Archive erhalten sichtbare Lücken; nichts aus aktuellen Werten rückwärts erfinden. Bei Gitfeeds Shallow-History, gelöschte Dateien, umbenannte Pfade und nichtlineare Historie explizit behandeln.

## 6. Schemaüberwachung und gezielte Invalidierung

SchemaExplorer-/GameTracking-/Protobufrevisionen gepinnt erfassen. Abhängigkeiten: Quellfeld → Parsermapping → Fact/Observation → Synergie/Materialisierung/Karte/Index/Buildtest. Ein Schemawechsel erzeugt `SchemaChange`, betroffene Abhängigkeiten werden gesperrt oder neu validiert. Nicht blind den gesamten Datenbestand löschen oder jede Antwort blockieren, wenn nur eine isolierte Klasse betroffen ist.

Syntaktisch gültige, semantisch geänderte Daten durch Feld-/Unitinvarianten und Golden-Beobachtungen erkennen. Unbekannte neue Felder nicht als bestätigte neue Mechanik ausgeben. Bis erfolgreicher Prüfung aktive konsistente Knowledge-Version beibehalten und Freshness-Status ausweisen.

## 7. Nutzungs- und Betriebsfreigaben

Keine juristische Freigabe aus Repo-Metadaten ableiten. Code-Lizenz, Datennutzungsbedingungen, Wiki-Text, Game-Assets, Fixturedateien und Veröffentlichungsrechte getrennt dokumentieren. Bei ungeklärten Rechten kein pauschales Copy/Vendor/Mirror. `LicensePolicy` und Egressstatus gelten auch für Replays, Modellprompts und öffentliche Quellenexports.

Repo-/Security-Gates früh in G0/G1 prüfen, nicht nach dem Import als Schlussaufgabe. U4s PR-/Auditbefunde sind historische Hinweise und müssen am tatsächlichen Lockfile überprüft werden. Abhängigkeiten erst nach freigegebener Policy aufnehmen; keine blanket Ignore-Liste für Warnungen. Fremdquellen niemals als Buildscript ausführen.

## 8. Nachweise

G1: Quellenregister mit Pflicht-/Referenzstatus, Rechteentscheid oder sichtbarem Blocker, Source Contract und Schema-Abhängigkeiten.

G2: ein gepinnter APIimport, ein historischer Gitdiff sowie ein absichtlich inkompatibles Schema führen reproduzierbar zu Daten bzw. Quarantäne. Beispiel bestätigt idempotenten Reparse und die Trennung von Parseränderung/Spieländerung.

G3: alle freigegebenen Pflichtfeeds samt belegtem historischen Umfang, Delta-/Deletepfad und Quellenstatus verarbeitet; keine stillen Dubletten oder überschriebenen Konflikte. Sekundäre Repos haben eine begründete Nutzung/Nichtnutzung.

G4: Upstreamausfall, Rate Limit, Feldentfernung/Unitwechsel, Parserupgrade, korrelierte Quellen und Konfliktpublikation getestet. Externe Live-Watcher und fest gepinnte PR-Gates separat geprüft; Rebuild arbeitet ohne Upstream-Python-/Fremd-Extraktionsprozess, soweit gespeicherte freigegebene Raw-Snapshots verwendet werden.
