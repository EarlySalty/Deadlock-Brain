status: erledigt
Datum: 2026-10-03
Rolle: frischer nativer Recherche-Blattworker für A
Modell: geerbtes GPT 6.1 Sol, kein Modellwechsel oder Fallback

# Aktuelle öffentliche Wiki-Abdeckung

## Urteil und tatsächliche Übergabe an A

**Die zwölf vorhandenen Artikel sind historische Captures vom 02.05.2026, keine Artikelabrufe vom 03.10.2026.** Ihre identifizierten Revisionen liegen zwischen dem 07.04.2026 und dem 01.05.2026. Alle zwölf Originalsidecars und zugehörigen Originalantworten wurden erneut gelesen. Die Seiteninhalte in `legacy-raw` stimmen für Titel, Seiten-ID, Namespace, Revisionen und Klartextauszug mit den zugehörigen Cache-Payloads überein: zwölf Gruppen mit jeweils zwei identischen Repräsentationen.

Die neue öffentliche Probe hat zwei tatsächliche Antworten geliefert:

- `robots.txt`: 03.10.2026 um **09:25:15 UTC**, HTTP **403**, Managed Challenge statt Robots-Regeln.
- Ein ausdrücklich erlaubter Revisionsabruf für den belegten Titel **Mechanics**: **09:27:59 UTC**, HTTP **403**, Managed Challenge statt Artikelantwort.

Keine neuen erfolgreichen Artikelcapturen und kein aktuelles Sitemapinventar. Das ist **kein Nachweis, dass jede öffentliche Einzelseite gesperrt ist**, und keine vollständige aktuelle Abdeckung. Die konkrete Artikel-API-Route wurde nach der Challenge beendet. Keine Varianten, anderen Titel oder HTML-/Exportdarstellungen nachprobiert. Der bestehende Datenauftrag verarbeitet weiterhin seinen unveränderten eingefrorenen Bestand; diese Recherche ersetzt ihn nicht.

## Gelesener Vertrag und unveränderte Grenzen

Gelesen: zentrale `CONTRACT.md` und `AN_BEREICHE.md`, insbesondere Punkt 40, sowie As `RECHERCHE.md`, `DUMPQUELLEN.md`, `DATEN.md` und das konkrete Briefing. Kein Code, Parser, Modul, Harness, Manifest, Dateninput, Originalhashinventar oder Statusartefakt verändert. Keine Git-, Compiler-, Format-, Test-, Gate-, Datenbank- oder Deploy-Schritte. Keine Agenten, T3-Threads oder fremden Sessions benutzt; keine Secrets, ENV-Dateien oder Nutzerdaten gelesen.

Die angebotenen Kontextwerkzeuge `ctx_execute` und `ctx_batch_execute` wurden vom laufenden Berechtigungsmodus verweigert. Keine Settings geändert oder Schutzgrenze umgangen. Die regulären nativen Read-/Bash-/Write-Werkzeuge waren für die konkreten Dateilesungen, vorhandenen `jq`-/Hash-CLIs und öffentlichen curl-Proben verfügbar. Keine neue Automatisierung oder Parserimplementierung gebaut.

## Zwölf historische Originalcapturen

Alle Zeiten sind UTC. Die Abrufzeit stammt jeweils aus dem tatsächlichen Sidecarfeld `fetched_at`, nicht aus dem Dateialter und nicht aus der heutigen lokalen Prüfung. Jede Zeile ist ausdrücklich **historische Capture**; ihre heutige aktuelle Revision ist unbekannt.

| Angefragter Name → tatsächlicher Titel | Seiten-ID | Revisions-ID | Revisionszeit | Originalabrufzeit | fetched_at |
| --- | ---: | ---: | --- | --- | ---: |
| Ability → Abilities | 1446 | 66275 | 2026-04-28T15:56:55Z | 2026-05-02T14:38:52Z | 1777732732 |
| Crowd Control | 904 | 65911 | 2026-04-26T12:19:12Z | 2026-05-02T14:57:47Z | 1777733867 |
| Damage Resistance | 922 | 65375 | 2026-04-23T21:06:22Z | 2026-05-02T14:57:36Z | 1777733856 |
| Item → Items | 379 | 67038 | 2026-05-01T03:58:33Z | 2026-05-02T14:38:57Z | 1777732737 |
| Level → Boon | 799 | 67377 | 2026-05-01T19:01:37Z | 2026-05-02T14:38:48Z | 1777732728 |
| Mechanics | 546 | 66541 | 2026-04-29T07:25:54Z | 2026-05-02T15:00:27Z | 1777734027 |
| Mo & Krill | 83 | 67291 | 2026-05-01T16:32:01Z | 2026-05-02T14:24:46Z | 1777731886 |
| Souls | 368 | 65184 | 2026-04-21T17:50:13Z | 2026-05-02T14:38:42Z | 1777732722 |
| Stats | 947 | 66280 | 2026-04-28T16:22:50Z | 2026-05-02T14:59:46Z | 1777733986 |
| Status Effects | 943 | 64024 | 2026-04-07T17:38:00Z | 2026-05-02T14:57:07Z | 1777733827 |
| The Curiosity Shop | 502 | 66710 | 2026-04-30T06:55:07Z | 2026-05-02T14:38:37Z | 1777732717 |
| Weapon Damage | 850 | 67060 | 2026-05-01T04:43:35Z | 2026-05-02T14:57:59Z | 1777733879 |

Alle zwölf Seiten haben Namespace 0 und einen identifizierten Wikitext unter `revisions[0].slots.main["*"]`. Die Sidecars nennen `application/json; charset=utf-8` und die tatsächliche öffentliche API-URL. Es gibt zwölf unterschiedliche Seiten-IDs und zwölf identifizierte Revisionen, nicht fünfzehn archivierte Seiten durch Mitrechnung der drei Redirectnamen. Autoren, Lizenzen und Revisionen eingebundener Templates/Module sind in diesen Antworten nicht belegt. Historische Auszüge sind deshalb kein aktuell oder vollständig revisionsgebunden belegtes Zahlenwerk.

### Konkrete Sidecar- und Bodybelege

Unveränderter gemeinsamer Wurzelpfad:

`/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/cache-provenance/`

Jeder folgende Stamm benennt die tatsächlichen Dateien `<Stamm>.bin.json` (Originalsidecar) und `<Stamm>.bin` (Originalantwort). Die zweite Spalte ist SHA-256 über die vollständigen Bytes der `.bin`-Antwort, nicht der Vertragshash über einen später normalisierten Artikeltext.

| Titel und tatsächlicher Cache-Stamm | SHA-256 des Originalbodys |
| --- | --- |
| Abilities: `36fa1bd42ad6bf905f3397ddbc407918521ce3f7948a1889bb89110f8383f07e` | `855c0a9c4760b90d559376ec4771b1c08507464d10023f66b8819e9adf18261f` |
| Crowd Control: `2ee2c4b647b09530c3f1df5c3851b2bfebed3843f3b0a8c98bd3d2167e14f89e` | `7633321d0a21c334ac80b773f813435ec9baf70a8bfc414819d1effdfd4b9d58` |
| Damage Resistance: `8fb5f677e8b2d80b634eb1d35a6d5d3b5e39bf9881c8cf13d1762ada0c319840` | `f27a708373c7219b348db39d41a9e9adb9c2564da0fae5f6f6edb633ccfeb4a2` |
| Items: `29500c307f4a07db726b8451b0edfd9c59d4a4be8010fc456561480b7b673cd9` | `50a4a4845cf666f90917abf95e7f77e8d80bdc879c0fa6d92f8a4a800ec38714` |
| Boon: `fff86c357ee7838a4f87eb1ddb7ac16510386724f1d49a07e8eeb1cfcd236516` | `1c18c1d9832273509b49edfa2575eea66ae817ef81fc4c0c954ccdb73566425e` |
| Mechanics: `eb4520d48ae5e9a1d53052c50cab2b66abf67991acb62a6ac46b659eea0027f3` | `6f784cd9146c0912558ca4da60c8053ceb820108b3b34e64e454a3a281a50c1f` |
| Mo & Krill: `192f4ce8ad11b49b72ed66714422f2dfbf748451c02803b3b339a0a2ce44d643` | `e6068eae29fd4cfc0aaf4ceeb1ee214e1e65384f398104fa17e153f094e4fde5` |
| Souls: `2f5449ff2f9bfbebeacea057cdd3827411070c84827224d2f128ad0fcfb15a60` | `3141e99f62afbf2c369edc0f2a70f8d63dbc468cc494d68ab237b8334e444d7a` |
| Stats: `c2c596450b44dfeba7e7817c1dcdc25bb2c47491a4e900abf7c9c422bc8e3948` | `cdd8daadf90664b3d14175d8b7f590f96834101918866cac40197fd0cd9c8562` |
| Status Effects: `767e65bfdaf826376a885ec4147745857ca1a600a856b75db3d96c4678ee3a35` | `647abd09e8d6ce17e123e91cbdf77be935ed91095487b73de1ab0698edd4e23e` |
| The Curiosity Shop: `ce4d47a2b36eae85ad0a9bbe39ad63035fd391e73a0b12767f92a42fab89b400` | `56d2757cf1cffb635ca280a0a9c3741469bc06c1b597733a7fb45fb4c78e58fb` |
| Weapon Damage: `e88020b7089b5e57d146de257d840ce1b3d0a31945794032c613ae5d1e328093` | `07c4175b520d6ef6a34cd55b1ee94e05fb60e1a00bf05816be3c08d4cf06924e` |

Die zwölf Gegenstücke unter `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/legacy-raw/` wurden nur gelesen. Identisch ist der geprüfte Seiteninhalt, nicht pauschal jede Bytefolge der unterschiedlich formatierten Gesamtdateien.

## Bereits vorhandene heutige Quellenprobe unabhängig verifiziert

Originalbelege ausschließlich lesend unter:

`/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/source-evidence/`

- `wiki-api-headers.txt`: tatsächlich HTTP 200, Date 2026-10-03 03:18:42 UTC, JSON, 5.939 Bytes, `cache-control: private, must-revalidate, max-age=0`, `cf-cache-status: DYNAMIC`.
- `wiki-siteinfo.json`: eigenes Quellzeitfeld `query.general.time=2026-10-03T03:18:42Z`, MediaWiki 1.46.0, `articlepath=/$1`, Statistik 101.914 Seiten, 939 Artikel, 97.391 Bilder, allgemeine CC-BY-NC-SA-Lizenzangabe. SHA-256 erneut `fb2a84a4fa153f1533b45f4a6b20c30aa22813fcb16a2406e9af96c379917092`; Headerhash erneut `46b614e94ee12b08f35fee722e63f18d8c3467b65e4b63abacd2c321f1fed7fb`.
- Robots-Header: der frühere 301 zeigt tatsächlich auf `https://deadlock.wiki/Robots.txt`; die Folgeheader enthalten HTTP 404 um 03:18:59 UTC. HIT/Age sind Cacheangaben, keine Artikelrevision oder Erlaubnis. Die frühere leere 301-Bodydatei ist kein Robots-Regeltext.
- `wiki-allpages-main-first.json`: tatsächliches Challenge-HTML mit `cType: managed`, allpages-URL und Challengezeit `1790997545`, entsprechend 03:19:05 UTC. SHA-256 erneut `56aa459ae2ebda7c86e15737245ac42551a88b48884fe348e7ab6ccc9c4a143e`. Der frühere HTTP-403-Status bleibt mangels Originalheader nur durch die damalige direkte Messung dokumentiert. Er wurde **nicht** neu abgefragt.

Siteinfo ist kein Artikelabruf und kein vollständiges Namespaceinventar. 939 ist kein gesicherter Nenner für aktuelle Hauptnamespace-Seiten einschließlich Redirects. Die heute gemeldete allgemeine Lizenz wird nicht auf die historischen Capturen zurückdatiert und beweist weder konkrete Autorenschaft noch kommerzielle Freigabe.

## Neue reguläre öffentliche Probe

Zunächst genau ein normaler Robots-GET. Kein expliziter Disallow-Regeltext empfangen, aber auch keine auswertbare aktuelle Robots-Datei oder darin benannte Sitemap. Die Challenge dieser Route wurde nicht wiederholt. Anschließend genau eine im Briefing ausdrücklich zugelassene unabhängige Artikelabfrage zu einem belegten Titel. Der Abstand betrug **164 Sekunden**, deutlich mehr als fünf Sekunden. Das war kein erneuter allpages-Versuch und keine Inventarvariante.

Beide Requests: curl ohne Konfigdatei (`-q`), unauthentifiziertes HTTPS, keine Cookies, Sonderheader oder Identitätsänderung, keine automatische Weiterleitung, keine Retrys, maximal 30 Sekunden und 2 MiB Responsebudget. Die Artikelabfrage enthält `maxlag=5`.

| Route | tatsächlicher Start/Ende UTC | Status | endgültige URL/Redirect | Inhaltstyp | Bodybytes |
| --- | --- | ---: | --- | --- | ---: |
| Robots | 2026-10-03T09:25:15Z / 09:25:15Z | 403 | `https://deadlock.wiki/robots.txt`, kein Redirect | `text/html; charset=UTF-8` | 5.391 |
| Mechanics-Revisionsabfrage | 2026-10-03T09:27:59Z / 09:27:59Z | 403 | identisch mit nachstehender Anfrage, kein Redirect | `text/html; charset=UTF-8` | 6.057 |

Artikelanfrage:

`https://deadlock.wiki/api.php?action=query&prop=revisions&rvprop=ids%7Ctimestamp%7Ccontent&rvslots=main&titles=Mechanics&format=json&formatversion=2&maxlag=5`

Beide Antworten haben `cf-mitigated: challenge`, Titel `Just a moment...`, `cType: managed` und den Hinweis auf JavaScript/Cookies. Das sind ausdrücklich keine Quelldokumente. Keine neue Seiten-ID, Revisions-ID, Revisionszeit, Lizenz oder Autorenangabe erhalten. `Age` und `Last-Modified` wurden für diese Antworten nicht geliefert. HTTP-Date und lokaler Abrufzeitpunkt stimmen jeweils auf Sekundenebene überein; sie ersetzen keine Revision.

### Neue Belege ausschließlich außerhalb des alten a-Roots

Ziel war vor Anlage nicht vorhanden. Alle neuen Dateien liegen getrennt unter:

`/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a-live-coverage/`

| Datei | Beleg |
| --- | --- |
| `20261003T092515Z-robots-body.html` | Challengebody, 5.391 Bytes; SHA-256 `c4dd222bfd09f84f3b0f1d96eb4f5ae056b7ee4e7227e35098f5415d306b8f2f` |
| `20261003T092759Z-mechanics-api-original-body.base64` | Base64-Erhaltung der ursprünglichen literalen Responsezeichen; nach Dekodierung 6.057 Bytes und SHA-256 `39f27b0a41d95413e25698429f11a49819c7472064fc6dbf57d7e269ca324d79` |
| `20261003T092515Z-092759Z-public-observations.json` | Tatsächliche lokale UTC-Zeiten, Status, endgültige URLs, ausgewählte gemessene Header, Bodypfade/-hashs, fehlende Revisionsfelder und Stopentscheidung |
| `20261003T092759Z-mechanics-api-body.html` | **Nicht autoritative Transkriptionsableitung**, kein bytegleicher Originalbody und kein Artikelinput |

Beweisgrenze der Ablage: Die Bodys wurden aus der nativen curl-Toolausgabe mit Write erhalten, nicht direkt in eine Originaldatei gestreamt. Die ersten Mechanics-Transkriptionsbytes dekodierten literale Backslash-u0026-Sequenzen der Response in Ampersands. Die separat gespeicherte Base64-Fassung stellt genau diese sichtbaren ursprünglichen Sequenzen wieder her. Sie wurde mit dem vorhandenen `base64 --decode` und `sha256sum` geprüft und entspricht wieder den gemessenen 6.057 Bodybytes. Die SHA-Werte sind lokale Nachablagehashs, kein unabhängig während des Netzempfangs erhobener zweiter Hash. Diese Grenze wird nicht als byteweiser Netzwerkvergleich ausgegeben. Die ausgewählten Header sind ein beschreibender Messbeleg, keine separat gespeicherte rohe Headerdatei.

## Sitemap, Stop und Abdeckungsgrenze

In den konkret gelesenen Quellenbelegen wurde kein freigegebener Sitemap-Index benannt. Die aktuelle Robots-Challenge liefert keinen Index. Deshalb kein geratener Sitemapname, kein Sitemapnamensfächer und kein als vollständig ausgegebenes Inventar. Nach der zusätzlichen Artikel-API-Challenge keine weiteren HTTP-Abfragen. Insbesondere kein `/Mechanics` als alternative Darstellung des gerade gesperrten Abrufs, kein Special:Export, keine Challenge-URLs, Proxyänderungen, Browser-, Cookie- oder Loginwege.

Belegt sind zwölf historische Artikelcapturen und in dieser neuen Probe null erfolgreiche aktuelle Artikelantworten bei einer versuchten bekannten Artikel-API-Abfrage. Das ist eine Stichprobe mit enger Stopgrenze, **kein Prozentwert über die heutige Wiki und kein Beweis fehlender öffentlicher Artikel insgesamt**. Aktuelle Namespace-Nenner, heutige Revisionen und aktuelle Renderabhängigkeiten bleiben unbekannt. Die fünf historischen Archive gehören weiterhin ausschließlich in den bereits laufenden Datenauftrag und wurden hier nicht erneut ausgewertet oder erweitert.

## Übergabeentscheidung

A erhält diesen Bericht und die getrennten Roh-/Messbelege durch die native Rückgabe. **Nichts aus `a-live-coverage` ist ein neuer Artikelinput oder wird jetzt importiert.** Den bestehenden Sammler, Normalisierungspfad und Datenharness unverändert weiterführen. Ein später tatsächlich vom Betreiber benannter und erlaubter öffentlicher Inventarweg müsste separat geprüft werden; aus diesen zwei Challenges wird weder eine neue Retryserie noch ein zweiter Sammler abgeleitet. Finale Integration, Rechteentscheidung und Gesamt-Abnahme bleiben bei A/C2 gemäß Vertrag.
