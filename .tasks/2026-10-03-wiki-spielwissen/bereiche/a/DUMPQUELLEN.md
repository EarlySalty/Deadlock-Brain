status: erledigt
Datum: 2026-10-03

# Öffentliche Wiki-Dumps und Betreiberquellen

Recherche am 03.10.2026, nativer Recherche-Worker für Bereich A. Gelesen wurden CONTRACT.md, AUFTRAG.md, PAKETE.md und AN_BEREICHE.md aus dem zentralen Aufgabenordner sowie RECHERCHE.md aus dem A-Worktree. Die vier Vertragsdateien lagen beim Einstieg nicht im A-Aufgabenordner. Der engere Schreibauftrag dieses Workers gilt unverändert.

## Ergebnis für A und C

Ein regulär öffentlich herunterladbarer historischer XML-History-Dump von **deadlock.wiki vom 15.04.2025** wurde gefunden, vollständig heruntergeladen und geprüft. Er enthält **4.417 unterschiedliche Seiten-IDs und 22.742 unterschiedliche Revisions-IDs**. Damit steht erheblich mehr echtes Wiki-Material als der zwölfseitige Altcache zur Verfügung. Die zwölf bereits gesicherten Artikelrevisionen aus April/Mai 2026 bleiben ergänzende, jüngere historische Quellen.

Der neue Fund beweist keine vollständige aktuelle Abdeckung vom 03.10.2026. Seine Originaldatei liegt außerhalb Git:

`/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/dump-evidence/deadlock.wiki-20250415-history.xml.zst`

Zusätzlich wurden vier ähnlich benannte öffentliche Textarchive geprüft und gesichert. Eines enthält einen umfangreichen älteren Stand von deadlocked.wiki; zwei enthalten eigenständige Deadlock-Wikis, eines lediglich eine Hauptseite. Alle fünf Archive bleiben anhand ihrer ursprünglichen Domain unterscheidbar. Keine Bild-, Audio-, Modell- oder Spielbinärdateien wurden heruntergeladen.

## Hauptfund: Internet Archive, deadlock.wiki

- [Archivobjekt](https://archive.org/details/wiki-deadlock.wiki-20250415)
- [Öffentliche Metadaten mit Dateiinventar und Prüfsummen](https://archive.org/metadata/wiki-deadlock.wiki-20250415)
- [Originaler XML-History-Download](https://archive.org/download/wiki-deadlock.wiki-20250415/deadlock.wiki-20250415-history.xml.zst)
- [Historisches siteinfo](https://archive.org/download/wiki-deadlock.wiki-20250415/deadlock.wiki-20250415-dumpMeta/siteinfo.json)
- [Erfassungskonfiguration](https://archive.org/download/wiki-deadlock.wiki-20250415/deadlock.wiki-20250415-dumpMeta/config.json)
- [Erfassungsfehler](https://archive.org/download/wiki-deadlock.wiki-20250415/deadlock.wiki-20250415-dumpMeta/errors.log)

Metadaten und sämtliche oben aufgeführten Downloads lieferten HTTP 200. Die Dateidownloads folgten ausschließlich den normalen öffentlichen 302-Weiterleitungen von archive.org auf dessen Downloadserver. Keine Anfrage ging dabei an deadlock.wiki.

Das Archiv bezeichnet den Ersteller als WikiTeam3, Version 4.4.1. Es ist ein öffentliches Drittarchiv, kein nachgewiesener vom Wiki-Betreiber veröffentlichter Dump. Als Originalquelle nennt es `https://deadlock.wiki/api.php`, als Erfassungsdatum `20250415`. Der XML-Kopf bestätigt Wiki-ID `deadlock`, Basisdomain `https://deadlock.wiki/`, MediaWiki 1.43.0 und Sprache `en`.

### Integrität und tatsächlicher Umfang

| Prüfung | Ergebnis |
| --- | --- |
| Originaldatei, komprimiert | 3.612.285 Bytes |
| SHA-1, gegen Archivmetadaten geprüft | `b7feb13c5410017d3a550f9a873d15c6414d0350`, identisch |
| SHA-256 der Originaldatei | `bea69e14f1ec35b8bbfe157b3cf9cf362f888e2321e9aa5a46a03c1670120825` |
| Entpackter XML-Inhalt | 177.037.503 Bytes |
| SHA-256 des entpackten Inhalts | `4a64288c2776b46a3722054aa4fe7f61cae167b1f5e3d4125c94080c37ab240e` |
| XML-Prüfung | XML::Parser 2.47 liest das gesamte XML fehlerfrei; externe Entitäten sind deaktiviert |
| Seitenblöcke | 4.418 |
| Unterschiedliche Seiten-IDs | 4.417 |
| Revisionsblöcke | 22.755 |
| Unterschiedliche Revisions-IDs | 22.742 |
| Doppelte Seite | `Main Page`, Seiten-ID 1, zweimal bytegleicher Seiteninhalt |
| Doppelte Revisionen | Genau die 13 Revisionen dieser doppelten Seite; keine abweichenden Inhalte bei gleicher Revisions-ID gefunden |
| Älteste enthaltene Revisionszeit | 2024-08-24T03:03:09Z |
| Jüngste enthaltene Revisionszeit | 2025-04-15T05:39:34Z |
| Redirect-Seiten, unterschiedliche IDs | 400 |

Die 22.755 Revisionsblöcke verteilen sich auf 20.676 `wikitext`, 742 `css`, 6 `text`, 25 `javascript`, 274 `json` und 1.032 `Scribunto`. Dies sind Blockzahlen vor Entfernung des doppelten Hauptseitenblocks. Lua und JavaScript werden ausschließlich als Quellentext behandelt und nicht ausgeführt.

Der XML-Dump enthält Seiten-IDs, Revisions-IDs, Zeitstempel, Inhaltsmodelle, Quellentexte und Beitragszuordnungen. Er ist damit maschinenlesbar und für einen Rust-Import mit echter Herkunft geeignet. Dieser Worker hat noch keine JSONL-Normalisierung und keinen Datenbankimport vorgenommen.

### Historisches Namespace-Inventar

Die folgenden Nenner sind die tatsächlich im historischen XML vorhandenen unterschiedlichen Seiten-IDs. Sie sind keine aktuellen Namespace-Größen und kein Prozentwert für das heutige Wiki.

| Namespace | ID | Seitenblöcke | Unterschiedliche Seiten-IDs |
| --- | ---: | ---: | ---: |
| Hauptnamespace | 0 | 772 | 771 |
| Talk | 1 | 31 | 31 |
| User | 2 | 220 | 220 |
| User talk | 3 | 10 | 10 |
| Deadlock | 4 | 7 | 7 |
| Deadlock talk | 5 | 0 | 0 |
| File | 6 | 2.751 | 2.751 |
| File talk | 7 | 0 | 0 |
| MediaWiki | 8 | 45 | 45 |
| MediaWiki talk | 9 | 0 | 0 |
| Template | 10 | 277 | 277 |
| Template talk | 11 | 2 | 2 |
| Help | 12 | 5 | 5 |
| Help talk | 13 | 0 | 0 |
| Category | 14 | 113 | 113 |
| Category talk | 15 | 0 | 0 |
| Module | 828 | 78 | 78 |
| Module talk | 829 | 1 | 1 |
| Update | 3000 | 72 | 72 |
| Update talk | 3001 | 0 | 0 |
| Data | 3002 | 34 | 34 |
| Data talk | 3003 | 0 | 0 |
| Gesamt | | 4.418 | 4.417 |

Von den 771 unterschiedlichen Hauptnamespace-Seiten sind 234 Redirects und 537 ohne Redirectmarkierung. Das historische siteinfo meldet dagegen 548 Artikel, 4.418 Seiten, 2.637 Bilder und 24.542 Bearbeitungen. Diese Verwaltungsstatistik ist nicht mit dem XML-Inventar gleichzusetzen. Die Bildzahl ist insbesondere kein Nenner für File-Beschreibungsseiten. Die beobachtete Übereinstimmung zwischen 4.418 Seitenblöcken und der Statistik beweist wegen des doppelten Hauptseitenblocks keine lückenlose historische Erfassung.

Bucket 9592 und Bucket talk 9593 sind im historischen Namespace-Schema nicht enthalten. Ihr Fehlen darf nicht als heutiger Nullbestand ausgegeben werden. Die aktuelle siteinfo-Antwort aus der vorherigen Recherche nennt diese beiden zusätzlichen Namespaces.

### Dokumentierte Erfassungsgrenzen

`config.json` nennt `namespaces=["all"]`, keine ausgeschlossenen Namespaces, `curonly=false`, XML aktiviert und die Erfassung historischer Revisionen. Die Konfiguration allein beweist keine vollständige Historie.

`errors.log` nennt drei fehlende Seitentitel: `Talk:Data:HeroData.json`, `Talk:Data:ItemData.json` und `--END--`. Der Ersteller beschreibt sie als vermutlich gelöscht. Der Worker hat keine aktuelle Existenz dieser Titel geprüft. Gelöschte oder versteckte Revisionen, Uploaddateien und eine transaktionsgenaue gemeinsame Renderfassung sind nicht durch diesen Textdump garantiert.

Der separate [komprimierte Titellistendownload](https://archive.org/download/wiki-deadlock.wiki-20250415/deadlock.wiki-20250415-dumpMeta/deadlock.wiki-20250415-titles.txt.zst) lieferte beim einmaligen normalen Abruf nach 302 HTTP 500. Es erfolgte keine Wiederholung. Der 170-Byte-Fehlerkörper liegt als `titles-download-http500-body.txt` vor, die Header als `deadlock.wiki-20250415-titles.txt.zst.headers`. Dieser Körper ist ausdrücklich keine Titelliste. Das Inventar oben wurde aus dem erfolgreich geladenen Original-XML ermittelt.

### Lizenzbelege

Die Archivmetadaten und das historisch mitgelieferte siteinfo nennen [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/). Die normale öffentliche Creative-Commons-Lizenzseite wurde mit HTTP 200 gesichert.

Der Dump enthält zusätzlich `MediaWiki:Copyright`, Revision **2577**, Zeit **2024-08-31T00:47:43Z**. Der Quellentext nimmt Nicht-Text-Medien und Spielinhalte ausdrücklich von der allgemeinen Inhaltslizenz aus. `Deadlock:Copyrights`, letzte enthaltene Revision **13564** vom **2024-10-29T19:25:55Z**, enthält weitere Regeln zu Bildern und Fair Use. Dieser historische Originalbeleg bestätigt eine konkrete Lizenzgrenze, die bislang nur aus älterer Planung bekannt war.

Die allgemeine Lizenz darf deshalb nicht pauschal auf Valve-Spielinhalte, Bilder oder andere Assets übertragen werden. Bei einer Weitergabe redaktioneller Wiki-Texte müssen Namensnennung, nichtkommerzielle Nutzung und ShareAlike geprüft werden. Eine kommerzielle Freigabe ist nicht belegt. C entscheidet über die zulässige interne Verarbeitung und gegebenenfalls dokumentbezogene Veröffentlichungsgrenzen. Die im XML erhaltenen Beitragszuordnungen dürfen nicht durch eine erfundene einzelne Autorenangabe ersetzt werden.

## Weitere öffentlich gesicherte Textarchive

Eine breitere Archive.org-Suche nach `identifier:wiki-deadlock* OR title:"Deadlock Wiki"` ergab zehn Objekte. Fünf davon betreffen offensichtlich andere benannte Wikis, darunter Battlestar Galactica, Celestial Deadlock und DeadlockAssociationCanon. Für die fünf verbleibenden Kandidaten wurden Metadaten geprüft; ihre normalen XML-Downloads lieferten jeweils HTTP 200 und ihre SHA-1-Werte entsprechen dem jeweiligen Archivdateiinventar.

| Quelle | Archivdatum | Unterschiedliche Seiten-IDs | Unterschiedliche Revisions-IDs | Tatsächliche jüngste Revisionszeit |
| --- | --- | ---: | ---: | --- |
| [deadlock.wiki](https://archive.org/details/wiki-deadlock.wiki-20250415) | 15.04.2025 | 4.417 | 22.742 | 2025-04-15T05:39:34Z |
| [deadlocked.wiki](https://archive.org/details/wiki-deadlocked.wiki-20241107) | 07.11.2024 | 2.076 | 13.301 | 2024-11-07T08:23:10Z |
| [deadlockwiki.org](https://archive.org/details/wiki-deadlockwiki.org_mw-20260130) | 30.01.2026 | 561 | 2.172 | 2025-03-20T18:20:30Z |
| [deadlock.miraheze.org](https://archive.org/details/wiki-deadlock.miraheze.org_w-20240616) | 16.06.2024 | 27 | 45 | 2024-05-22T12:24:19Z |
| [deadlockwiki.miraheze.org](https://archive.org/details/wiki-deadlockwiki.miraheze.org_w-20231203) | 03.12.2023 | 1 | 1 | 2023-12-02T13:55:45Z |

Wichtige Unterschiede:

- **deadlocked.wiki:** XML-Wiki-ID `deadlock`, 1.875 Seiten mit gleicher Seiten-ID und gleichem Titel wie im 2025-Dump. 12.793 Revisions-IDs kommen in beiden vor; 508 seiner Revisions-IDs fehlen im neueren Dump. Das ist ein konkreter Hinweis auf zusätzlich erhaltenes historisches Material. Eine Betreiberbestätigung des Domainwechsels wurde nicht gefunden. Das Archiv bleibt daher mit seiner ursprünglichen Domain erhalten, bis A/C die Quellenzuordnung anhand der Inhalte prüfen. Es enthält 8.307 Seitenblöcke für 2.076 IDs, weil Historien auf wiederholte Seitenblöcke verteilt sind. Ein Import darf nicht einfach nur den letzten Seitenblock pro ID behalten.
- **deadlockwiki.org:** XML-Wiki-ID `endeadlockwiki`, 207 unterschiedliche Hauptnamespace-Seiten, 227 File-Beschreibungsseiten und weitere Namespaces. Beispieltitel und Inhalte betreffen Valve-Deadlock. Es ist keine nachgewiesene Betreiberquelle von deadlock.wiki und kein aktueller 2026-Inhaltsstand, obwohl das Archiv im Januar 2026 veröffentlicht wurde. Seine 1.533 Seitenblöcke enthalten wiederholte IDs. Archivmetadaten nennen CC BY-SA 4.0. Numerisch gleiche Seiten- oder Revisions-IDs verschiedener Wikis belegen keine gemeinsame Herkunft.
- **deadlock.miraheze.org:** 12 Hauptnamespace-Seiten und 15 File-Beschreibungen, darunter Ivy und Infernus. Eigenständiges historisches Wiki mit sehr kleinem Bestand; Metadaten nennen CC BY-SA 4.0. Keine Bilddateien geladen.
- **deadlockwiki.miraheze.org:** Genau eine Hauptseite und eine Revision aus Dezember 2023. Die Bezeichnung „Official“ im Archivtitel ist eine Quellenbezeichnung, kein Betreiber- oder Valve-Nachweis. Kein nachgewiesener brauchbarer Spielwissenskorpus.

Originaldateien im gemeinsamen Belegverzeichnis:

| Datei | Bytes | SHA-256 |
| --- | ---: | --- |
| `deadlocked.wiki-20241107-history.xml.zst` | 2.380.787 | `2744ead3789a2fc01fe02c5cce395cd5a50c229e3cb4b606bda8adab302cf716` |
| `deadlockwiki.org_mw-20260130-history.xml.zst` | 299.552 | `b314ab370dad21b43a4e0388cdcc82f15ff3b9a61fcbc411a08508a8b10925f6` |
| `deadlock.miraheze.org_w-20240616-history.xml.zst` | 6.637 | `12473543191bcfa6821b1d7d34c54a9e3c3be51239c18dc3f987fb3631dc6c97` |
| `deadlockwiki.miraheze.org_w-20231203-history.xml.zst` | 1.478 | `e2da412176e1db15c085ed445f29722fe2b25519053801a5ccc64116c915d1d8` |

## Öffentliche Quellen der Wiki-Betreiber

[GitHubs öffentliche Organisationsmetadaten](https://api.github.com/orgs/deadlock-wiki) nennen `The Deadlock Wiki` und verlinken deadlock.wiki. Die [normale öffentliche Repositoryliste](https://api.github.com/orgs/deadlock-wiki/repos?per_page=100&type=public) liefert acht Repositorys, passend zur gemeldeten Zahl `public_repos=8`, ohne Fortsetzung. Beide Abrufe lieferten HTTP 200.

| Repository | Geprüfter Umfang | Ergebnis für Dump-/Exportrecherche |
| --- | --- | --- |
| [deadlock-data](https://github.com/deadlock-wiki/deadlock-data) | Öffentliche Repository-Metadaten und bereits belegter Scope aus RECHERCHE.md | Maschinenlesbare Spielwerte, Lokalisierungen und Patchtexte. B verantwortet Sammlung und Extraktion, einschließlich der bereits gefundenen 125 Wikitext-Patchdateien. Hier keine zweite Datensammlung. |
| [deadbot](https://github.com/deadlock-wiki/deadbot) | Öffentliche README | Betreiberwerkzeug: GameTracking-Daten werden verarbeitet und nach deadlock-data sowie in die Wiki übertragen. Code-MIT-Lizenz; kein vollständiger Wiki-Prosa-Dump dokumentiert. Werkzeug nicht ausgeführt. |
| [pages](https://github.com/deadlock-wiki/pages) | Vollständiger öffentlicher Git-Baum: 18 Einträge, nicht abgeschnitten; README | Generator und Uploadwerkzeug, keine eingecheckte Seitenbibliothek. README verlangt ein Moderatorenkonto für seine Nutzung. Nicht gestartet, keine Zugangsdaten oder ENV-Dateien gelesen. Keine Repository-Lizenz in den geprüften Metadaten oder im Baum belegt. |
| [wiki_analysis](https://github.com/deadlock-wiki/wiki_analysis) | Vollständiger Baum: 15 Einträge, nicht abgeschnitten; README; vorhandene öffentliche Revisions-CSV | Aktivitätsauswertungen statt Artikeltexten. CSV enthält 2.207 Datenzeilen vom 19.01. bis 25.01.2026 mit `page,timestamp,user,type`, ohne Seiten-ID, Revisions-ID oder Inhaltsfeld. Kein vollständiges Seiteninventar. Keine Repository-Lizenz belegt. |
| [DeadlockEntityHelper](https://github.com/deadlock-wiki/DeadlockEntityHelper) | Vollständiger Baum: 12 Einträge; README | Werkzeug zur Extraktion von Kartenentitäten, AGPL-3.0 laut GitHub-Metadaten. Kein Wiki-Seitendump in den geprüften Pfaden. |
| [cloudflare-worker-develop](https://github.com/deadlock-wiki/cloudflare-worker-develop) | Vollständiger Baum: 3 Einträge; README | README bezeichnet das Projekt als recent-changes-Worker. Kein veröffentlichter Dump im Baum oder dokumentierter Dumpdownload. Worker nicht aufgerufen. |
| [cloudflare-worker-master](https://github.com/deadlock-wiki/cloudflare-worker-master) | Vollständiger Baum: 13 Einträge; README | Worker-Entwicklungsprojekt. Kein veröffentlichter Dump im Baum oder dokumentierter Dumpdownload. Keine Variablen-/ENV-Dateien gelesen. |
| [mediawiki-extensions-CocoaTweaks](https://github.com/deadlock-wiki/mediawiki-extensions-CocoaTweaks) | Vollständiger Baum: 81 Einträge; README | MediaWiki-Erweiterung für Oberflächen- und Lizenzhinweise. Kein veröffentlichter Wiki-Dump in den geprüften Pfaden. |

Die sechs genannten Git-Bäume lieferten HTTP 200 und `truncated=false`. Die sieben abgerufenen READMEs und die Revisions-CSV lieferten ebenfalls HTTP 200. Die beiden vollständigen älteren Bäume sind auf `acc31295ceb33dc6af651e469b6e661371508065` für pages und `4e830089765af89fd7fc77430fefdc6fa0ae6bca` für wiki_analysis identifiziert. Es wurden ausschließlich öffentliche Dateien gelesen, keine Programme dieser Repositorys ausgeführt.

In dieser konkreten Organisationsliste, den genannten Bäumen und READMEs wurde kein aktueller kompletter Wiki-Dump oder dafür dokumentierter Download gefunden. Diese Aussage betrifft die geprüften Quellen und behauptet nichts über private Betreiberbackups oder andere unentdeckte Veröffentlichungen.

## Zugriffsgrenzen und reguläre Exportwege

Der zuvor beobachtete HTTP-403-/Managed-Challenge-Befund für allpages bleibt bestehen. Dieser Worker hat **keine neue Anfrage an deadlock.wiki** gestellt, keine Parameter- oder Header-Varianten ausprobiert und keine Cookies, Konten, Ersatzidentitäten oder Proxys eingesetzt.

Die während der Recherche übermittelte Präzisierung des Auftraggebers erlaubt einen einmaligen normalen Abruf eines öffentlich dokumentierten offiziellen Special:Export- oder Dumpwegs. Ein solcher aktueller Betreiberdownload wurde in den geprüften Veröffentlichungen nicht gefunden. Special:Export wurde hier nicht zusätzlich live geprüft; seine Erreichbarkeit und ein aktueller Vollbestand bleiben daher offen. Der gefundene öffentliche Archivdownload liefert bereits einen wesentlich größeren echten historischen Bestand und umgeht keine Zugriffssperre der Wiki-Domain.

Durchgeführt wurden außerdem Websuchen nach `"deadlock.wiki" "dump"`, öffentlichen Backups/Exports, `"deadlock.wiki" "archive.org"`, Special:Export-/Mirror-Hinweisen und `"deadlock-wiki" "dump" github`. Die belastbaren Funde stammen aus den direkt gemessenen GitHub- und Archive.org-Antworten. Die erste exakte Archive.org-Suche nach `"deadlock.wiki"` hatte einen Treffer; die breitere Titelsuche erschloss die vier weiteren gesicherten Kandidaten.

## Übergabe und verbleibende Arbeit

1. A kann den echten 2025-XML-Dump als zusätzliche historische Quelle vollständig inventarisieren und in Rust zum Vertrag normalisieren. Doppelte Seiten und Revisionen sind vor dem Import idempotent zu behandeln. Namespace 3002 ist Data, Namespace 3000 ist Update.
2. C muss Herkunft, Lizenzgrenzen und historischen Stand beim Import erhalten. Ein Abruf vom 03.10.2026 macht keine der enthaltenen Revisionen aktuell. Die 2024-Archive benötigen eine eigene nachvollziehbare Quellenzuordnung; unabhängige Wiki-IDs dürfen nicht mit deadlock.wiki kollidieren.
3. Das Material erlaubt keine vollständige aktuelle Wiki-Abdeckung. Aktuelle Namespace-Nenner, zwischen April 2025 und Oktober 2026 hinzugefügte Seiten und aktuelle Abhängigkeitsrevisionen bleiben offen. Auch die historischen Verwaltungszähler beweisen keine vollständige öffentlich erhaltene Historie.
4. Die aktuelle Betreiber-Datenquelle und ihre 125 Wikitext-Patchdateien bleiben bei B/C. Kein Doppelimport oder zweiter normalisierter Datenbestand wurde erzeugt.
5. Kein Code, Cargo-Manifest, Schema, Core-Modul, REGISTER, Statusereignis oder TODO wurde verändert. Keine Cargo-Läufe, Commits, Pushes, Merges, Datenbankänderungen, Dienstneustarts oder Deployments. Synthetische S12-Fixtures wurden nicht verwendet.

## Dauerhafte Originalbelege

Alle neu gespeicherten Originalantworten und Downloadheader liegen unter:

`/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/dump-evidence/`

Dort liegen die fünf komprimierten Original-XML-Dateien, Archive.org-Suchantworten, fünf Archivmetadatensätze, das historische `siteinfo.json`, `config.json`, `errors.log`, die öffentliche Creative-Commons-Lizenzseite, Organisations-/Repository-Metadaten, sechs vollständige Git-Bäume, sieben README-Originale und die öffentliche Betreiber-Aktivitäts-CSV samt HTTP-Headern. Die drei historischen Begleitdateien wurden zusätzlich gegen die publizierten SHA-1-Werte geprüft und stimmen jeweils überein. Der fehlgeschlagene Titeldownload ist gesondert als Fehlerkörper bezeichnet.

Frühere Belege, darunter die aktuelle siteinfo-Antwort und der allpages-Challenge-Körper, liegen unverändert im benachbarten `source-evidence/`. Dieser Bericht und die neuen öffentlichen Originalbelege sind die einzigen Schreibprodukte dieses Workers.
