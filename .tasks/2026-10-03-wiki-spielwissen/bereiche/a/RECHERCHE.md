status: erledigt
Datum: 2026-10-03
Rolle: nativer Recherche-Worker für Teilbereich A, Versuch 1
Ausgangsstand: 2734c2da4e814ff79953e8e825275b0216a6af16
Vertrag gelesen: wiki-spielwissen-v1, CONTRACT.md im zentralen Aufgabenordner

# Wiki-Quelle und vorhandener Bestand

## Öffentliche Quelle und Zugriffsgrenze

Die im vorhandenen Rust- und Python-Quellcode eingetragene Wiki-Adresse ist `https://deadlock.wiki/api.php`. Eine öffentliche lesende Abfrage hat diese Quelle am 2026-10-03T03:18:42Z bestätigt:

`https://deadlock.wiki/api.php?action=query&meta=siteinfo&siprop=general%7Cnamespaces%7Cstatistics%7Crightsinfo&format=json&formatversion=2&maxlag=5`

HTTP 200, JSON, `The Deadlock Wiki`, MediaWiki 1.46.0, Sprache `en`, Wiki-ID `deadlock`, Basisadresse `https://deadlock.wiki/`.

| Statistik der Quelle | Wert |
| --- | ---: |
| Seiten | 101914 |
| Artikel | 939 |
| Bilder | 97391 |
| Bearbeitungen | 186754 |

Diese Statistik ist kein vollständiges Seiteninventar und kein geeigneter Nenner für die Abdeckung eines einzelnen Namespaces. Die 939 Artikel sind insbesondere nicht gleichbedeutend mit allen Hauptnamespace-Seiten einschließlich Redirects.

Die Quelle meldet 24 reale, nichtnegative Namespaces: `0`, `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`, `9`, `10`, `11`, `12`, `13`, `14`, `15`, `828`, `829`, `3000`, `3001`, `3002`, `3003`, `9592`, `9593`. Dazu kommen die virtuellen Namespaces Media `-2` und Special `-1`.

| Wichtiger Namespace | ID | Besonderheit |
| --- | ---: | --- |
| Hauptnamespace | 0 | einziger Namespace mit `content=true` in dieser Antwort |
| File | 6 | Dateibeschreibungen, keine pauschale Asset-Lizenz |
| Template | 10 | kann gerenderte Werte beeinflussen |
| Category | 14 | Kategorien und Zuordnungen |
| Module | 828 | Scribunto-/Lua-Quellen als Daten behandeln |
| Update | 3000 | historische Patchseiten |
| Data | 3002 | Standard-Inhaltsmodell `json` |
| Bucket | 9592 | Standard-Inhaltsmodell `json` |

Ein Filter ausschließlich auf `content=true` würde die für Spielwerte wichtigen Data-, Template-, Module- und Update-Seiten auslassen. Das Namespace-Schema muss aus der echten Antwort kommen. Die frühere synthetische S12-Fixture verwendet für Data die ID 3000; auf der echten Quelle ist Data 3002 und Update 3000.

`https://deadlock.wiki/robots.txt` lieferte zunächst HTTP 301 nach `https://deadlock.wiki/Robots.txt`. Der Zielpfad lieferte HTTP 404 mit einer HTML-Seite. Es liegt damit aus dieser Probe kein brauchbarer Robots-Regeltext vor. Das ist kein Nachweis einer ausdrücklichen Freigabe für einen Vollabruf.

Die erste reguläre Inventarabfrage wurde am 2026-10-03T03:19:05Z mit HTTP 403 und einer Cloudflare Managed Challenge beantwortet:

`https://deadlock.wiki/api.php?action=query&list=allpages&apnamespace=0&aplimit=500&apfilterredir=all&format=json&formatversion=2&maxlag=5`

Die Antwort verlangt JavaScript und Cookies. Keine Challenge gelöst, keine Cookies oder Anmeldung verwendet, keine andere Identität oder Ersatzadresse eingesetzt und keine weiteren Netzabrufe an diese Wiki-Quelle durchgeführt. Ein vollständiges Live-Inventar, aktuelle Artikelrevisionen und ein offizieller vollständiger Export sind deshalb in dieser Recherche nicht nachgewiesen.

## Lizenz und tatsächlich vorhandene Zeitfelder

Die erfolgreiche `siteinfo`-Antwort enthält:

```json
{"url":"https://creativecommons.org/licenses/by-nc-sa/4.0/","text":"Creative Commons Attribution-NonCommercial-ShareAlike"}
```

Damit ist die am 03.10.2026 gemeldete allgemeine Wiki-Lizenz CC BY-NC-SA 4.0 belegt. Eine seiten- oder medienspezifische Lizenzprüfung und eine Freigabe zur kommerziellen Veröffentlichung sind dadurch nicht belegt. Der ältere Integrationsplan im Repository beschreibt weitere Ausnahmen für Nicht-Text-Medien und Game Content; diese wurden hier nicht erneut live auf einer Lizenzseite geprüft. Keine Bilder, Audio- oder Modelldateien heruntergeladen.

Die zwölf vorhandenen Wiki-Rohdateien besitzen folgende echte Felder:

- Wurzel: `batchcomplete`, `query`.
- Seite: `extract`, `ns`, `pageid`, `revisions`, `title`.
- Revision: `parentid`, `revid`, `slots`, `timestamp`.
- Inhalt unter `revisions[0].slots.main["*"]`; dazu `contentformat=text/x-wiki`, `contentmodel=wikitext`.
- Bei drei Abrufnamen zusätzlich `query.redirects` mit `from` und `to`.

In diesen Rohantworten fehlen Lizenz, Autorname, Beitragendenliste, Abrufzeit, Patchkennung und ein Vollständigkeitsvermerk. Diese Felder dürfen nicht aus Dateialter oder heutiger Wiki-Lizenz erfunden werden. Ein später ergänzter allgemeiner Lizenznachweis muss als separate Beobachtung vom 03.10.2026 erkennbar bleiben. Die Artikelrevisionen stammen aus dem Zeitraum 07.04.2026 bis 01.05.2026 und sind heute historische Quellen.

Die echten Abrufzeiten sind separat in den vorhandenen HTTP-Cache-Sidecars gespeichert, jeweils als Unix-Sekunden in `fetched_at`. Die zugehörigen `.bin`-Payloads wurden auf dieselben Seitennamen, Seiten-IDs und Revisions-IDs geprüft. Alle unten angegebenen Abrufzeiten sind UTC am **2026-05-02**. Das ist vorhandene Provenienz, keine neue Abrufzeit.

## Die zwölf vorhandenen Wiki-Rohseiten

Verzeichnis: `/home/nathanael/repos/Deadlock-Brain/data/raw/deadlock_wiki/`.
Alle zwölf Seiten liegen im Namespace 0. Die Dateinamen verwenden teils den angefragten Redirectnamen, während `query.pages` den tatsächlichen Zielnamen enthält.

| Rohdatei | Tatsächlicher Seitentitel | Seiten-ID | Revision | Revisionszeit UTC | Abrufzeit UTC am 02.05.2026 |
| --- | --- | ---: | ---: | --- | --- |
| `Ability.00a476f210a384f0.json` | Abilities | 1446 | 66275 | 2026-04-28T15:56:55Z | 14:38:52 |
| `Crowd_Control.6c109c2ae0031a19.json` | Crowd Control | 904 | 65911 | 2026-04-26T12:19:12Z | 14:57:47 |
| `Damage_Resistance.d7e459a192591c76.json` | Damage Resistance | 922 | 65375 | 2026-04-23T21:06:22Z | 14:57:36 |
| `Item.31e2834f1d82e649.json` | Items | 379 | 67038 | 2026-05-01T03:58:33Z | 14:38:57 |
| `Level.b7b37cb768f3260a.json` | Boon | 799 | 67377 | 2026-05-01T19:01:37Z | 14:38:48 |
| `Mechanics.bfd15af08ee5a02d.json` | Mechanics | 546 | 66541 | 2026-04-29T07:25:54Z | 15:00:27 |
| `Mo___Krill.10580c36cb1bab60.json` | Mo & Krill | 83 | 67291 | 2026-05-01T16:32:01Z | 14:24:46 |
| `Souls.f1f8e13f5e511fe6.json` | Souls | 368 | 65184 | 2026-04-21T17:50:13Z | 14:38:42 |
| `Stats.4a8999ca26d0fca0.json` | Stats | 947 | 66280 | 2026-04-28T16:22:50Z | 14:59:46 |
| `Status_Effects.358a32804f7eabe4.json` | Status Effects | 943 | 64024 | 2026-04-07T17:38:00Z | 14:57:07 |
| `The_Curiosity_Shop.0cf22431835061c4.json` | The Curiosity Shop | 502 | 66710 | 2026-04-30T06:55:07Z | 14:38:37 |
| `Weapon_Damage.ffff5c9a2cbccd21.json` | Weapon Damage | 850 | 67060 | 2026-05-01T04:43:35Z | 14:57:59 |

Belegte Redirectzuordnungen: `Ability → Abilities`, `Item → Items`, `Level → Boon`. Die drei Redirect-Quellseiten haben in diesen Antworten **keine eigenen Seiten- oder Revisions-IDs**. Eine Zuordnung kann erhalten werden; daraus drei vollständig archivierte Redirectseiten zu machen wäre falsch.

Alle zwölf Rohdateien enthalten Wikitext und zusätzlich einen gerenderten Klartextauszug. Die Wikitextrevision ist identifiziert, die Revisionen der beim Rendern verwendeten Templates und Module sind nicht gespeichert. Konkrete Abhängigkeiten sind sichtbar:

| Seite | Sichtbare Module beziehungsweise dynamische Inhalte |
| --- | --- |
| Damage Resistance | `#invoke:HeroDataArrays`, `#invoke:ItemData`, Item-Stat-Templates |
| Weapon Damage | `#invoke:HeroDataArrays`, `#invoke:ItemData` |
| Boon | `#invoke:LevelTables`, `#invoke:SoulUnlock` |
| Souls | `#invoke:GenericData` |
| Items | `Items count`, `Infobox ShopItems`, `Active Items` und weitere Templates |

Diese Abhängigkeiten beeinflussen Zahlen und Tabellen. Der historische Klartextauszug darf deshalb nicht als durch die Artikelrevision allein vollständig gepinntes aktuelles Zahlenwerk ausgegeben werden. Historische Prosa und deren Quellrevision bleiben verwendbar; fehlende Abhängigkeitsrevisionen müssen erkennbar bleiben.

## Weitere lokale Wiki-Artefakte

Die vorhandenen HTTP-Rohcache-Dateien liegen unter `/home/nathanael/repos/Deadlock-Brain/data/cache/`. Für die Wiki sind genau diese zwölf Sidecar-Stämme gefunden und ihre `.bin`-Payloads auf die oben genannten Revisionen geprüft:

| Abrufname | Cache-Stamm für `.bin` und `.bin.json` |
| --- | --- |
| Ability | `36fa1bd42ad6bf905f3397ddbc407918521ce3f7948a1889bb89110f8383f07e` |
| Crowd Control | `2ee2c4b647b09530c3f1df5c3851b2bfebed3843f3b0a8c98bd3d2167e14f89e` |
| Damage Resistance | `8fb5f677e8b2d80b634eb1d35a6d5d3b5e39bf9881c8cf13d1762ada0c319840` |
| Item | `29500c307f4a07db726b8451b0edfd9c59d4a4be8010fc456561480b7b673cd9` |
| Level | `fff86c357ee7838a4f87eb1ddb7ac16510386724f1d49a07e8eeb1cfcd236516` |
| Mechanics | `eb4520d48ae5e9a1d53052c50cab2b66abf67991acb62a6ac46b659eea0027f3` |
| Mo & Krill | `192f4ce8ad11b49b72ed66714422f2dfbf748451c02803b3b339a0a2ce44d643` |
| Souls | `2f5449ff2f9bfbebeacea057cdd3827411070c84827224d2f128ad0fcfb15a60` |
| Stats | `c2c596450b44dfeba7e7817c1dcdc25bb2c47491a4e900abf7c9c422bc8e3948` |
| Status Effects | `767e65bfdaf826376a885ec4147745857ca1a600a856b75db3d96c4678ee3a35` |
| The Curiosity Shop | `ce4d47a2b36eae85ad0a9bbe39ad63035fd391e73a0b12767f92a42fab89b400` |
| Weapon Damage | `e88020b7089b5e57d146de257d840ce1b3d0a31945794032c613ae5d1e328093` |

`/home/nathanael/repos/Deadlock-Brain/game-wiki/pages/deadlock-wiki/wiki-page.md` bündelt dieselben zwölf Einträge mit Quellen-URLs, Rohpfaden, Snapshot-/Dokument-IDs, Hashs und Abrufzeiten. Es ist eine bestehende Ableitung, kein zusätzlicher Korpus mit zwölf neuen Seiten.

`/home/nathanael/.worktrees/brain-fix-c5-wiki-runtime-integration/architecture/migration/s12/fixtures/pilot.capture.json` existiert, enthält 19 **synthetische** Seiten und nennt `source_key=synthetic_wiki`. Diese Fixture eignet sich zur Parserprüfung und belegt keine echte Wiki-Abdeckung. Die frühere Fixture in `brain-s12-wiki-20260924/architecture/migration/s12/fixtures/` ist ebenfalls vorhanden. Keine der beiden als echte Wiki-Rohdaten ausgegeben.

In den gezielt geprüften Wiki-Completion-Worktrees wurde kein weiteres passendes `data/raw/*wiki*/`- oder `data/cache/*wiki*/`-Unterverzeichnis gefunden. Das ist eine Aussage über diese konkreten geprüften Pfade, keine Behauptung über alle Dateien des Hosts.

## Wiederverwendbare Rust-Bausteine

Der zugewiesene Ausgangsstand enthält `rust/crates/dbrain-sources/src/wiki.rs`:

- `DEFAULT_API_URL` auf Zeile 18 ist die belegte Quelle.
- `pull_wiki_page` beginnt auf Zeile 41 und importiert genau einen Titel über den vorhandenen `SourceStore`.
- Die Abfrage auf Zeilen 70 bis 79 fordert Klartext und Revisionsinhalt an, löst Redirects und führt kein Inventar durch.
- Der vorhandene Defaultabstand ist fünf Sekunden auf Zeile 35.

Bereits gebaute spätere Komponenten liegen in früheren, fremden Worktrees und wurden nur gelesen:

1. `/home/nathanael/.worktrees/brain-wiki-completion-20260925/rust/crates/dbrain-sources/src/wiki_corpus.rs`: Inventarfortsetzung mit `generator=allpages`, Revisions-URLs, Lizenz und Attribution. Sein dokumentierter Umfang ist `main_namespace_nonredirect_articles`; Redirects und zusätzliche Namespaces sind damit nicht vollständig abgedeckt. `rendered_templates_pinned=false` wird ausdrücklich gespeichert. Fortsetzungszyklen und unvollständige Antworten werden geprüft.
2. Daneben `wiki_capture_io.rs`: HTTP-Capture mit festem API-Pfad, `get_no_redirect`, ohne Auth-, Proxy- oder Host-Fallback. `stage_sources_with_pool` stellt Revisionen im vorhandenen `SourceStore` bereit; Canonical-Publikation ist davon getrennt.
3. `/home/nathanael/.worktrees/brain-fix-c5-wiki-runtime-integration/rust/crates/dbrain-wiki/`: vorhandener produktiver S12-Parser und Adapter zum gemeinsamen `brain_contracts`-IR. Laut geprüftem README bleiben Bedingungen, Varianten, Aliasdaten, Abhängigkeiten und fehlende Werte im gemeinsamen IR. `project_card` flacht Bedingungen und Varianten nicht in unbedingte Fakten ab.

Der bestehende Importer `rust/crates/dbrain-sources/src/deadlock_data.rs` verarbeitet bereits das von der Wiki gepflegte öffentliche Datenrepository einschließlich Heroes, Abilities, Items, NPCs, Lokalisierungen und historischer Patchdateien. Dieses Quellenrepository ist **kein vollständiger Export der Wiki-Prosa oder des Wiki-Seiteninventars**.

Die GitHub-Quelle [deadlock-wiki/deadlock-data](https://github.com/deadlock-wiki/deadlock-data) ist im Bestand unter `/home/nathanael/repos/Deadlock-Brain/data/external/deadlock-data/` vorhanden. Gelesener HEAD: `0d46cdecfccf77adec16aac01af6d30173e0ebb8`, Commitzeit 2026-10-01T22:50:39Z, Client 6731. README: master-Daten werden über deadbot an die Wiki übertragen. Lokale LICENSE nennt MIT und Copyright 2024 deadlock-wiki; GitHubs öffentliche Repository-Metadaten bestätigen MIT. Eine MIT-Lizenz des Repositorys ersetzt keine Rechteprüfung für sämtliche zugrunde liegenden Game Assets. Im lokalen Bestand sind 136 rohe Changelogdateien, 125 Wiki-Wikitext-Patchdateien und 109 strukturierte Versionsdateien gezählt. Datensammlung und Nutzung dieses Repositorys liegen nach ausdrücklicher Bereichszuweisung bei B, nicht bei diesem Recherche-Worker.

## API-Vertrag und verbleibende Grenzen

Die [offizielle MediaWiki-Dokumentation zu allpages](https://www.mediawiki.org/wiki/API:Allpages) bestätigt: `apnamespace` wählt jeweils einen Namespace, `aplimit=500` ist das normale Maximum, `apfilterredir=all` erhält Redirects, `apcontinue` setzt die Liste fort. Fortsetzungsobjekte müssen vollständig erhalten werden; kleine oder leere Zwischenbatches sind bei Filterung kein Ende. Diese allgemeine API-Dokumentation beweist keine erfolgreiche Inventarisierung von deadlock.wiki.

Für die vorhandenen historischen Quellen sind stabile Wiki-IDs aus den tatsächlichen `pageid`-Werten und Revisionsstrings aus `revid` gemäß CONTRACT.md ableitbar. Der SHA-256 im JSONL-Vertrag muss über genau den übergebenen `content` berechnet werden; ein Hash der ganzen API-Antwort ist dafür nicht austauschbar. Wiki-Revisionszeit, vorhandene Cache-Abrufzeit und heutige lokale Beobachtung sind drei getrennte Zeitangaben. Eine Wiki-Revision ist keine belegte Spiel-Patchkennung.

Belegte Grenzen dieses Rechercheauftrags:

- Vollinventar und aktuelle Seitenrevisionen durch HTTP 403 nicht ermittelt; kein Prozentwert für Vollabdeckung behauptet.
- Kein brauchbarer Robots-Regeltext und keine live geprüfte seiten- oder medienspezifische Nutzungserlaubnis.
- Zwölf historische Artikelrevisionen, drei Redirectzuordnungen, keine eigenständigen Redirectrevisionen und keine gepinnten Template-/Modulrevisionen.
- S12-Fixtures sind synthetisch; Daten-Git gehört zu B und zählt nicht als vollständiger Wiki-Export.
- Keine produktiven Codeänderungen, keine Cargo-Läufe, keine Datenbankänderungen, keine Commits, kein Merge oder Deploy durch diesen Recherche-Worker.

## Nachweisorte und Quellen

Die bereits gespeicherten öffentlichen HTTP-Originalantworten wurden ohne weitere Netzabrufe bytegleich nach `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/source-evidence/` kopiert. Jede Zieldatei wurde mit `cmp` gegen ihre unveränderte Originaldatei im temporären Auftragsordner `/home/nathanael/.claude/jobs/f01cce67/tmp/` geprüft. Es handelt sich um die beim Abruf gespeicherten Antwortdateien, nicht um neu erzeugte Zusammenfassungen:

| Datei | Inhalt |
| --- | --- |
| `wiki-siteinfo.json`, `wiki-api-headers.txt` | erfolgreiche offizielle API-Antwort mit Namespaceinventar, Statistik und Lizenz |
| `wiki-robots-headers.txt`, `wiki-robots.txt` | erste Robots-Antwort mit 301 |
| `wiki-robots-final-headers.txt`, `wiki-robots-final.txt` | Redirectziel mit HTTP 404 |
| `wiki-allpages-main-first.json` | HTML-Antwort der ersten Inventarabfrage, HTTP 403 wurde beim Abruf gemessen |

Diese temporären Nachweise sind keine dauerhafte Datenübergabe und keine Freigabe für Wiki-Automatisierung.

Quellen: [Deadlock Wiki API](https://deadlock.wiki/api.php), [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/), [MediaWiki allpages](https://www.mediawiki.org/wiki/API:Allpages), [Wiki-Datenrepository](https://github.com/deadlock-wiki/deadlock-data), [Wiki-GitHub-Organisation](https://github.com/deadlock-wiki).
