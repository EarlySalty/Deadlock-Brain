# Grafiken und kleine Webseiten aus dem Brain

Stand: 7. Oktober 2026. Reiner Audit. Keine Produktänderung, Veröffentlichung, Modellprobe, DB-Abfrage oder DB-Schreiboperation. Nur dieser Bericht wurde geschrieben. Öffentliche Seiten wurden per GET ohne JavaScript-Ausführung geprüft.

## Urteil

Ein brauchbarer Ausgangspunkt existiert: Brain erzeugt bereits deterministische HTML-Steckbriefe für Helden, Fähigkeiten und Items. Dazu kommen eine laufende Build-Korpus-Seite, SVG-Bausteine der Website und ein vorhandenes Patch-Dashboard. Eine durchgehende Strecke vom belegten v2-Werkzeugergebnis zur freigegebenen Vergleichsgrafik mit dauerhaftem Link ist in den geprüften Pfaden nicht nachgewiesen. Die Hauptlücke liegt bei Ergebnisvertrag, Rechteprüfung, Veröffentlichung und Bot-Anbindung. Ein weiterer Rechenkern wäre eine Doppelentwicklung. [B1, B2, W1, W2, G1]

Die erste separate Grafikaufgabe sollte eine typisierte, deterministische Darstellung versionsgebundener G-Ergebnisse verwenden. Frei generiertes HTML oder JavaScript braucht der belegte Nutzerwunsch nicht. Discord und Twitch entfernen Webseitenlinks aus normalen Brain-Antworten. Das MVP benötigt einen typisierten, serverseitig geprüften Artefakt-/Linkvertrag mit passendem Bot-Linkbutton oder separatem Publisher. Eine URL im Antworttext reicht nicht. [N1, G1, DB3, T1]

## Was der Nutzer tatsächlich beauftragt hat

| Belegter Wunsch | Einordnung |
| --- | --- |
| Grafiken und kleine Webseiten aus dem Brain sind ausdrücklich interessant; vorhandene Wünsche sollen tiefer geprüft und an den v2-Umbau angeschlossen werden. | Aktueller Rechercheauftrag, keine Baufreigabe. Quelle: `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-feature-audit/AUFTRAG.md:7-15`, unversionierte Aufgabenakte. [N1] |
| Je Held, Fähigkeit und Item ein Steckbrief mit aktuellen Werten und Patch-Story; Brain-Dokument und HTML auf der bestehenden `/brain`-Seite entstehen aus der DB. | Konkreter älterer Produktwunsch und vorhandene Implementierungsstrecke. Quelle: `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung/UEBERGABE-KOPF-GROK.md:16`, Aufgabenakte; Implementierung [B1, B2]. |
| Grafiken und Webseiten werden separat gebaut. G baut nichts dazu, ändert keine Roadmap und liefert strukturierte Zahlenreihen. | Verbindliche aktuelle Scope-Grenze. Quelle: `/home/nathanael/.worktrees/brain-g-v2-20261007/.tasks/2026-10-06-brain-abschluss/G/PLAN.md:13-17`, Worktree-HEAD `3d6890c0ef69c910173563f140e17504ea8612a4`; zusätzlich `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-06-brain-abschluss/VON_HAUPT.md:9`. [G1] |

Die Beispiele Helden-/Itemvergleich, Builddarstellung und Erklärseite in der aktuellen B-Briefingakte sind Recherchefragen. Daraus folgt noch keine ausdrückliche Nutzerentscheidung für jede einzelne Bildschirmfunktion. Heldenkurven und Überholpunkte sind im G-Plan als fachliche Ergebnisse belegt; deren grafische Darstellung ist eine anschließende Empfehlung. Freie HTML-/JS-Erzeugung, ein PNG-Export, ein neuer öffentlicher Publikationsdienst und automatische Veröffentlichung wurden in den geprüften Nutzerakten nicht als eigener Wunsch gefunden. [N1, N2, G1]

Die frühere B-Briefingformulierung, G habe das Grafikziel in die Roadmap aufgenommen, ist überholt. Maßgeblich ist die Korrektur in `G/PLAN.md:17`. [N2, G1]

## Revisionsstand: main, WIP und live getrennt

Alle sechs Remote-main-Stände wurden einzeln mit `git ls-remote origin refs/heads/main` geprüft. Sie entsprachen den lokalen `origin/main`-Refs. Kein Fetch und keine Gitmutation. Die Codebelege zu main stammen aus `git show origin/main:<Pfad>`, nicht aus dem alten, schmutzigen Hauptbaum.

| Repo unter `/home/nathanael/repos/` | Remote-main | Hauptbaum-HEAD | Befund |
| --- | --- | --- | --- |
| Deadlock-Brain | `9711cb630aebacfe959ed4783595b071f479be36` | `2734c2da4e814ff79953e8e825275b0216a6af16` | Alt und schmutzig; main separat gelesen. |
| Deadlock-2nd-Brain | `28b4c078c7ccde8d0708f88f5bf1cc51b01a9eff` | gleicher SHA | Sauber. |
| Deadlock-Docs | `6fa4ca3758d6f259cc0ca128e5350834fe1b4cfe` | `400231af7e8196e145111e4150c2e24c2ab79fb5` | Abweichender, schmutziger Hauptbaum. |
| Website | `dbd2b347014dc733d926c6998d1c03681d61ab15` | gleicher SHA | Eine Statusposition; main gelesen. |
| Deadlock-Bots | `e18f522226f8e2dec5a1c03fe97c2aba3200c8d1` | `dbda52b81cf8e68ea2aa7351a0e7ef2df5c81dbc` | Abweichender, schmutziger Hauptbaum. |
| Deadlock-Twitch-Bot | `e0b0dbaf662d7680c4ceaa210bf15f1443693cd8` | `d828481624d53408e0c0a4c3ed1a8e4a6d421c40` | Abweichender, stark schmutziger Hauptbaum. |

Weitere gezielt geprüfte Nebenpfade:

- G-Worktree: `/home/nathanael/.worktrees/brain-g-v2-20261007`, HEAD `3d6890c0ef69c910173563f140e17504ea8612a4`. Architektur und laufende Arbeit, kein hier geführter Live-Funktionsbeweis. [G1]
- Rust-Site-Port: `/home/nathanael/.worktrees/brain-a-site-20261006`, HEAD `cac8763525c9ba930a61dae2e43c09e13bb0fda0`. Enthält `deadlock-brain-site.rs` und typisierte Assetauslieferung. Die geprüften Pfade sind auf Brain-main nicht vorhanden. Der aktive Siteprozess ist weiterhin Python. [A1, L1]
- Bestehender Korpus: `/home/nathanael/Documents/deadlock-build-corpus`, HEAD `1c2743faa28fd93d1fa26cd97a0a335b1f66ca26`. `site/app.js`, `site/server.py` und `site/index.html` waren gegenüber diesem HEAD unverändert. Kein Remote-main-Beweis dieses zusätzlichen Repos. [K1]
- Caddy: HEAD `c8191db57c0fbaa4d08149b25349b48d9edf60cf`; kein Remote-main-Beweis. `hosts/v50671/Caddyfile` ist lokal geändert. Deshalb werden dessen konkrete Routingzeilen als lokaler Konfigbefund bezeichnet; die öffentlichen GETs bestätigen die Erreichbarkeit und Header, nicht den gesamten Konfigstand. [C1, L2]

### Sichere Live-Belege

Eigene read-only-Prüfung mit `systemctl --user show` und `readlink /proc/<PID>/exe`:

- `brain-serve.service`: active/running, PID `4062119`, Binary `/opt/deadlock-brain/maintenance-releases/bfda408cb988722ddceadb56bca5b72e12d12731/brain-serve`.
- `deadlock-brain-site.service`: active/running, PID `2002603`, Executable `/usr/bin/python3.12`, Startpfad `/home/naniadm/Documents/deadlock-build-corpus/site/server.py`.

Das belegt Prozesse und aufgelösten Binarypfad. Es belegt weder den Binaryhash noch die G-Funktionen oder Vollständigkeit der öffentlich exportierten Steckbriefe. Der laufende SHA unterscheidet sich von Brain-main. [L1]

Öffentliche GETs, ohne Browser und ohne Ausführung der Seitenskripte:

| URL | Ergebnis | Aussagegrenze |
| --- | --- | --- |
| `https://deutsche-deadlock-community.de/brain/site/index.html` | HTTP 200; Titel `Deadlock Build-Korpus`; `Content-Type: text/html`; CSP und `nosniff`; `Last-Modified: Mon, 06 Jul 2026 04:50:41 GMT`; kein `Cache-Control` und kein `ETag` im beobachteten Headerbestand. | Bestehende Seite ist erreichbar. Das Dateidatum ist kein Aktualitätsbeweis der Spieldaten. |
| `https://deutsche-deadlock-community.de/patch/` | HTTP 200; Titel `Patch Timeline · Deutsche Deadlock Community`; CSP, `nosniff`, `ETag` und `Last-Modified`. | Bestehendes Patch-Dashboard ist erreichbar; keine Prüfung seiner dynamischen API-Ergebnisse. |

Quelle [L2]: GET-Beobachtungen am 07.10.2026 in dieser Sitzung. Keine Kommentarroute, Mitgliederdaten oder Chatinhalte abgerufen.

## Vorhandene Bausteine und ihre Grenzen

### Brain-Steckbriefe: vorhandener Rust-Renderer auf main

`RenderedEntityProfile` enthält kompaktes Brain-Dokument, öffentliches HTML und relativen Pfad. `render_entity_profile` erzeugt feste Abschnitte für aktuelle Werte, Kontext, Konflikte, Patch-Story und Quellenlücken. Texte werden maskiert. Pfade folgen `site/entities/{hero|ability|item}/{hex(entity_key)}.html`. Die Seiten enthalten Tabellen und Historie, keine allgemeine Vergleichsgrafik. [B1]

Öffentliche Werte sind derzeit an `ProfileSourceKind::GameFile`, Gitrevision, öffentliche Sichtbarkeit, leere notwendige Scopes und `publication_allowed` gebunden. Ungeeignete Einheiten und Varianten werden zurückgehalten. Der Renderer sagt ausdrücklich, dass Wiki-Belege intern bleiben und die Freigabe der Spielwerte keinen aktuellen Patchstand bestätigt. Ein künftiger Assets-API-Ursprung passt deshalb nicht automatisch in diesen heutigen Vertrag. G nennt diese Anpassung bereits als offene Arbeit. [B1, G1]

Dateiausgabe ist atomar über temporäre Datei und Persistierung. Ziel und Verzeichnisse werden gegen Symlinks geprüft. Die Wartungsintegration legt Dokument-/HTML-Artefakte an, vergleicht exportierte Bytes und kann entfernte eigene Profile samt belegtem HTML zurücknehmen. Diese Strecke wiederverwenden. Ein separater Exporter ohne diese Herkunfts- und Rücknahmeprüfungen würde vorhandene Sicherungen verlieren. [B2]

### HTML-/SVG-Prüfung: vorhanden, keine Freigabe für beliebiges JS

`brain-maintenance/src/html.rs` hat eine HTML-/Attribut-Allowlist. Eventhandler, aktive URL-Schemata, `srcdoc`, fremde SVG-Ressourcen, CSS-Imports und aktive CSS-Konstruktionen werden abgewehrt. Neue Styles und Linkressourcen müssen zum vorhandenen Dokument passen. SVG darf lokale Fragmentverweise unter den geprüften Bedingungen tragen. [B3]

Diese Prüfung erlaubt die sichere Bearbeitung bestimmter Dokumentformen. Sie liefert noch keinen fachlich typisierten Diagrammvertrag und keine Bot-Publikation. Der vorhandene HTML-Retrievaladapter gewinnt deterministisch sichtbaren Text und bindet Roh-/Semantikhashes; er ruft keine Ressourcen ab. Eine Grafik benötigt weiterhin eine lesbare Tabelle oder Textbeschreibung, damit ihr Inhalt über diesen Weg auffindbar bleibt. [B3, B4]

### Bestehende öffentliche Build-Korpus-Seite

Die Seite lädt `builds_registry.json`, `hero_meta_compact.json`, `item_meta_compact.json` sowie Markdowntexte. Sie hat schon Heldensicht, Tabellen und Kommentare. Sie ist ein vorhandener Anzeigeort, aber keine nachgewiesene Ausgabe des aktuellen G-Rechenkerns. [K1]

Die aktive Pythonreferenz verwendet `SimpleHTTPRequestHandler` für den gesamten Korpusroot und eine Kommentarroute. Der lokale Caddy-Pfad entfernt `/brain` und reicht an Port 8087 weiter. Loopback ist hier kein Schutz vor öffentlichem Zugriff. Den Korpusroot daher niemals mit internen Brain-Dokumenten, privaten Quellen oder Arbeitsartefakten anreichern. Ein kontrollierter Rust-Dateipfad ist bereits im A-Worktree vorhanden. [K1, C1, A1]

### Website: SVG-System und Patchdarstellung vorhanden

`dl-landing/src/charts.js` enthält gemeinsame Farben, Formatierung, `svgEl`, Tabellenausgabe und Diagramm-/Tooltipfunktionen. Die Anzeigeprinzipien und Gestaltung sind wiederverwendbar. Nach den geltenden Sprachregeln wären neue produktive Renderer und Berechnungen in Rust umzusetzen; bestehendes JavaScript ist kein Grund, einen zweiten fachlichen Rechenpfad dort einzubauen. [W1]

`dl-patch/src/patch.js` besitzt Timeline, Entitätsraster, Detailansicht und Fehlerdarstellung. Es lädt Assets direkt von `api.deadlock-api.com`. Diesen Weg nicht als Quelle für gepinnte G-Vergleiche übernehmen: G verlangt den lokalen Spiegel, feste Version und Belege. Die Patchseite ist eine vorhandene Publikations-/Designreferenz. [W2, G1]

### Docs und Second Brain

Deadlock-Docs besitzt einen öffentlichen Dokumentbaum und einen vorhandenen Markdown-Renderer. Dieser Legacyrenderer schaltet rohes HTML im Markdownparser aus und maskiert den Titel. Neue Funktionen darin wären nach dem Rustgebot nicht zulässig. Als Veröffentlichungsort ist der vorhandene öffentliche Baum nutzbar, sofern dieselben Freigabe- und Versionsregeln gelten. Caddy liefert die öffentlichen Docs aus einem getrennten `public`-Root aus. [D1, C1]

Der geprüfte Second-Brain-Pfad ist ein interner Rust-Operatoradapter. Seine README belegt einen Consumer- und Integrationsstand, keinen Grafikrenderer oder öffentlichen Artefaktvertrag. Ein zweites Grafiksystem im Second Brain ist aus diesem Bestand nicht begründet. [S1]

## Integration in Discord und Twitch

| Weg | Heute belegt | Fehlender Anschluss |
| --- | --- | --- |
| Discord-Brain-Antwort | Ein öffentliches Embed mit Titel, bereinigtem und gekürztem Antworttext, Farbe und Footer; Mentions deaktiviert. Die Antwort ersetzt den Denkplatzhalter. Schon der main-Transport entfernt HTTP-/HTTPS-Links und kürzt auf 3800 UTF-16-Einheiten. [DB1, DB3] | Kein Grafikattachment, Bildfeld, typisierter Artefaktlink oder separater Vorschau-/Publikationsschritt in diesem Antwortbuilder. Freitext-URLs kommen nicht durch. Eine öffentliche Discord-Antwort ist keine private Vorschau. |
| Discord-Dashboard | Wiki-API liefert eine kanonisierte `.md`-Datei als JSON; Pfadgrenzen und Symlinks werden geprüft. [DB2] | Kein öffentlicher HTMLrenderer. Ein Dashboard-Wikipfad darf nicht ohne eigene Freigabe als öffentliche Grafikquelle verwendet werden. |
| Twitch-Brain-Chat | `safe_chat_answer` entfernt Markdownlinks und linkähnliche Wörter, normalisiert Text und begrenzt die Antwort auf 450 Zeichen. [T1] | Ein generierter Website-Link erreicht den Chat auf diesem normalen Weg nicht. Eine spätere bewusste Freigabe eigener Artefaktlinks braucht einen eng begrenzten Vertrag; den allgemeinen Linkfilter nicht pauschal entfernen. |
| Twitch-Buildlabor | Adminpflicht für Katalog und Buildhandler; vorhandener Reasoner wird verwendet. [T2] | Geeigneter bestehender Ort für eine Betreiberansicht. Kein Beleg für einen öffentlichen Veröffentlichungsweg oder für aktuelle G-Pins. |

Für das MVP getrennte Zustände vorsehen: private Betreiberansicht, ausdrücklich freigegebene öffentliche Seite, Zustellung ihres Links. Eine Vorschau darf keine öffentlichen Chatnachrichten senden. Die nötige Freigabe ergibt sich aus Quellenrechten und gewünschter Veröffentlichung; aus einem lesbaren Ergebnis folgt keine Publikationsberechtigung. Links gehören in ein typisiertes, serverseitig geprüftes Artefaktfeld, das der Bot außerhalb des gefilterten Antworttexts als eigenen Linkbutton oder über einen passenden Publisher zustellt. [B1, DB3, T1, T2]

Der separat geprüfte ältere Bots-Stand `e1f11614` verwirft bei `answered` und `build_rejected` Antworten mit HTTP-Link oder mehr als 3800 UTF-16-Einheiten sogar vollständig. Aktuelles main `e18f5222` entfernt Links und kürzt stattdessen. Das sind verschiedene Codezustände, kein hier belegter Livewechsel. Beide verhindern die naive Freitext-URL-Zustellung. [DB3]

## Sicherheits- und Aktualitätsbefunde

1. **Freies HTML ist am bestehenden Korpuspfad bereits eine Vertrauensgrenze.** `site/app.js:25-27` setzt geladenes Markdown über `marked.parse` direkt in `innerHTML`; in dieser Funktion ist keine zusätzliche Bereinigung erkennbar. Die beobachtete CSP blockiert Inline-JavaScript, macht beliebiges HTML aber nicht ungefährlich. Frei generierte Inhalte können weiterhin Links, Formelemente oder irreführende Oberfläche einschleusen. Keine aktive XSS-Probe ausgeführt. Für neue Ausgaben feste Rust-Templates mit maskierten Texten und endlichen Zahlen verwenden. [K1, L2]
2. **Öffentlicher Root und Kommentarroute müssen getrennt bewertet werden.** Der aktive Legacyserver liefert den Korpusroot aus und hat `GET/POST /api/comments`. Im gelesenen Servercode ist keine Authentifizierung dieser Route erkennbar. Der lokale Caddy-Brainblock zeigt keinen eigenen Schutz. Es wurde weder ein Kommentar gelesen noch geschrieben. Für die Grafikaufgabe keine Kommentare oder internen Dateien als Mitnahmefeature hinzufügen. Der A-Port muss vor öffentlicher Weiterverwendung live abgenommen werden. [K1, C1, A1]
3. **Rechte und Herkunft bleiben Pflicht.** Der aktuelle öffentliche Steckbriefrenderer veröffentlicht nur bestimmte freigegebene Git-Spielwerte. Assets-API, Wiki, Herstellerbilder und frei generierte Erklärungstexte sind dadurch nicht pauschal freigegeben. Originaldaten, Rechnung, Bedingung, Quelle und Veröffentlichungserlaubnis müssen getrennt nachvollziehbar bleiben. [B1, G1]
4. **Lebensdauer ist noch kein Grafikvertrag.** Bestehende Steckbrief-URLs hängen an der Entität, nicht an einer unveränderlichen Ergebnisrevision. Sie werden überschrieben oder bei Rücknahme entfernt. Ein geteilter Vergleich braucht einen Ergebnisstand, sonst kann derselbe Link später andere Zahlen zeigen. Öffentliche Rücknahme darf aber auch unter einem historischen Link nicht umgangen werden. [B1, B2]
5. **Caching braucht Version und Rechtezustand.** Die laufende Korpusseite liefert `Last-Modified`, aber kein beobachtetes `Cache-Control`. Der Rust-Site-Port setzt `no-cache`. Für private Vorschauen wäre `no-store` passend; für öffentliche Ergebnisse ein inhaltsgebundener Identifikator mit ETag und ein klarer Rücknahmeweg. Das ist eine Empfehlung, noch kein gebauter Zustand. [L2, A1]
6. **Technische Version ist kein Patchname.** G verlangt `PinnedGameContext`, Originaldokumentbelege und Mechanikrevision. Eine fehlende belegte Patchzuordnung bleibt unbekannt. Diagrammtitel dürfen aus `client_version` keinen Balancepatch erfinden. [G1]

## MVP-Empfehlung und Abhängigkeiten

### Empfehlung

Eine separate, kleine Rust-Darstellung an den vorhandenen Renderer-/Siteweg anschließen. Als erste Abnahme einen belegten Heldenvergleich mit Kurve und lesbarer Wertetabelle wählen, sobald G die dazugehörigen Zahlenreihen und Belege liefert. Der G-Plan nennt Boonkurven und Überholpunkte bereits ausdrücklich. Alternativ den vorhandenen Entitätssteckbrief erweitern, falls der Vergleichsvertrag noch nicht fertig ist. [G1, B1]

Der Eingabevertrag sollte bereits geprüfte Ergebnisdaten tragen: Version, IDs und Namen, Metrik und Einheit, Szenariobedingungen, Zahlenreihen, unbekannte Werte mit Grund, Quellenbelege sowie Mechanikrevision. Die Darstellung berechnet keine Spielwerte neu und ruft keine fremden Assets-APIs auf. G beschreibt genau diese gemeinsame Ergebnisebene. [G1]

Öffentliche Seiten sollten den Vergleich, seine Bedingungen und den Quellenstand kurz zeigen. Eine lesbare Tabelle bleibt neben dem SVG bestehen. Unbekannt wird als fehlend dargestellt und nicht als null. Quellenverweise dürfen nur öffentliche, erlaubte Ziele enthalten; interne Dokument-IDs und private Quellenpfade gehören nicht auf die Seite. [G1, B1, B4]

### Reihenfolge ohne Doppelbau

1. G-Ergebnisvertrag und tatsächlichen Lieferstand prüfen. G ist hier Architekturabhängigkeit, kein Grafikimplementierer. Insbesondere MirrorReceipt und Assets-Herkunft sind im gelesenen Plan noch offene Lieferungen. [G1]
2. Bestehenden A-Rust-Site-Port und Entitätsrenderer gemeinsam nutzen. Aktiver Pythonserver und ungeprüfte Rootauslieferung sind kein fertiger sicherer Zielzustand. [A1, K1, L1]
3. Vorschau zunächst im bestehenden Betreiberbereich. Quellenfreigabe, Revisionsbindung und Rücknahme vor öffentlichem Link prüfen. [T2, B1, B2]
4. Public-Export über den bestehenden atomaren Artefaktweg; Ergebnisstand und Cachepolitik festlegen. [B2, A1]
5. Discord-Darstellung und Twitch-Zustellung getrennt anschließen. Den geprüften Artefaktlink über ein eigenes typisiertes Feld und Bot-Darstellung beziehungsweise separaten Publisher transportieren; die normalen Antworttext-Linkfilter beider Bots beibehalten. [DB1, DB3, T1]

Buildgrafiken und Itemvergleiche erst nach belegtem Nutzerumfang und vorhandenem fachlichem Ergebnisvertrag aufnehmen. Der G-Plan sieht für Builds einen reinen Adapter zum bestehenden F-Planer vor; AI, Persistenz und Steam-Veröffentlichung gehören nicht in diesen Recheneingang. Eine Grafikfunktion darf daraus keine automatische Steam-Veröffentlichung machen. [G1:85]

### Abnahme für den späteren Bau

- Gleiche gepinnte Ergebnisdaten ergeben dieselben HTML-/SVG-Bytes; jede Zahl ist auf Eingaben, Bedingungen und Regelrevision zurückführbar.
- Unbekannte Version, fehlender Beleg und fehlende Publikationsfreigabe stoppen den Export oder zeigen eine benannte Lücke.
- Unvertrauenswürdige Texte werden maskiert; keine freien Skripte, Eventhandler, Fremdressourcen oder unsicheren URL-Schemata.
- Vorschau bleibt privat und landet nicht im Chat. Öffentlicher Export und Rücknahme sind als getrennte Vorgänge belegbar.
- Öffentlicher GET belegt konkrete Ergebnisrevision und Header. Danach getrennt Zustellung in Discord beziehungsweise Twitch prüfen, ohne aus einem laufenden Prozess die Funktion abzuleiten.

## Quellenverzeichnis

Die folgenden main-Belege beziehen sich auf die SHAs der Revisionstabelle. Angegebene Zeilen stammen aus `git show origin/main:<Pfad>`. Aufgabenakten sind als unversioniert gekennzeichnet.

- **N1:** `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-feature-audit/AUFTRAG.md:7-15`, unversioniert.
- **N2:** `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-feature-audit/BRIEFING-B.md:17`, unversioniert. Beispiele und frühere, inzwischen korrigierte G-Roadmapaussage.
- **G1:** `/home/nathanael/.worktrees/brain-g-v2-20261007/.tasks/2026-10-06-brain-abschluss/G/PLAN.md:9-19,43-56,63,75-89,151`; beobachteter Worktree-HEAD `3d6890c0ef69c910173563f140e17504ea8612a4`. Plan, kein Implementierungs- oder Livebeweis. Ergänzend `G/G0-0645-VERTRAG.md:47-55,75` und `rust/crates/brain-contracts/src/tools.rs:256-300,613-643` im selben Worktree.
- **B1:** `/home/nathanael/repos/Deadlock-Brain/rust/crates/brain-maintenance/src/entity_profile_render.rs:16-29,31-87,131-162,277-341`, main `9711cb630aebacfe959ed4783595b071f479be36`.
- **B2:** `/home/nathanael/repos/Deadlock-Brain/rust/crates/brain-maintenance/src/entity_profile_render.rs:346-401`; `/home/nathanael/repos/Deadlock-Brain/rust/crates/brain-maintenance/src/integration/entity_profiles.rs:243-301,311-379,1067-1131`; gleicher Brain-main-SHA.
- **B3:** `/home/nathanael/repos/Deadlock-Brain/rust/crates/brain-maintenance/src/html.rs:9-37,108-193,194-214,293-329,392`, gleicher Brain-main-SHA.
- **B4:** `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-retrieval/src/html_projection.rs:1-28,163-187`, gleicher Brain-main-SHA.
- **A1:** `/home/nathanael/.worktrees/brain-a-site-20261006/rust/crates/deadlock-brain/src/bin/deadlock-brain-site.rs:11-25`; `site/mod.rs:25-39,43-54`; `site/assets.rs:18-25,45-103`, Worktree-HEAD `cac8763525c9ba930a61dae2e43c09e13bb0fda0`.
- **K1:** `/home/nathanael/Documents/deadlock-build-corpus/site/server.py:8-10,23-50`; `site/app.js:4,25-40,55-76,98,123,153`; `site/index.html:69-70`, HEAD `1c2743faa28fd93d1fa26cd97a0a335b1f66ca26`, genannte Dateien gegenüber HEAD unverändert. Legacyreferenz, nicht geändert.
- **W1:** `/home/nathanael/repos/Website/dl-landing/src/charts.js:18-32,58-104,155-190`, main `dbd2b347014dc733d926c6998d1c03681d61ab15`.
- **W2:** `/home/nathanael/repos/Website/dl-patch/src/patch.js:7-8,88-169,336-382,414-635`, gleicher Website-main-SHA. Graphify-Kandidaten wurden am main-Code nachgelesen; öffentliche GET-Prüfung [L2].
- **D1:** `/home/nathanael/repos/Deadlock-Docs/tools/render_public_markdown.py:4,32-34,75-92`, main `6fa4ca3758d6f259cc0ca128e5350834fe1b4cfe`.
- **S1:** `/home/nathanael/repos/Deadlock-2nd-Brain/tools/brain-adapter/README.md:1,19-24`, main `28b4c078c7ccde8d0708f88f5bf1cc51b01a9eff`. Dokumentierter älterer Integrationsstand, nicht als heutiger Laufzeitzustand übernommen.
- **DB1:** `/home/nathanael/repos/Deadlock-Bots/rust/bin/dl-bot/src/modglue.rs:559-589,828-852`, main `e18f522226f8e2dec5a1c03fe97c2aba3200c8d1`.
- **DB2:** `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-dashboard/src/brain.rs:617-666`, gleicher Bots-main-SHA.
- **DB3:** `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-brain/src/brain_api.rs:138-163,174-225`, main `e18f522226f8e2dec5a1c03fe97c2aba3200c8d1`; Vergleich derselben Datei bei Commit `e1f11614:121-131`, ebenfalls direkt mit `git show` gelesen. Keine Aussage, welcher dieser Botstände live läuft.
- **T1:** `/home/nathanael/repos/Deadlock-Twitch-Bot/rust/bin/tb-bot/src/brain_chat_wiring.rs:438-467,696-733`, main `e0b0dbaf662d7680c4ceaa210bf15f1443693cd8`.
- **T2:** `/home/nathanael/repos/Deadlock-Twitch-Bot/rust/crates/tb-dashboard-api/src/handlers/brain_lab.rs:68-98`, gleicher Twitch-main-SHA.
- **C1:** `/home/nathanael/repos/Caddy/hosts/v50671/Caddyfile:197-200,381-385,418-426`, lokal geänderter Arbeitsstand bei HEAD `c8191db57c0fbaa4d08149b25349b48d9edf60cf`; Routing nicht als unverändertes main verkauft.
- **L1:** Eigene Betriebsbeobachtung dieser Sitzung am 07.10.2026: `systemctl --user show deadlock-brain-site.service brain-serve.service -p ActiveState -p SubState -p MainPID -p ExecStart` und aufgelöste `/proc/<PID>/exe`-Pfade. Keine Credentials gelesen.
- **L2:** Eigene öffentliche GET-Beobachtung dieser Sitzung am 07.10.2026 für die beiden oben genannten URLs; nur Status, Titel und ausgewählte öffentliche Header ausgewertet, keine Skriptausführung.

## Grenzen der Aussage

Graphify wurde global und für Brain, Docs, Second Brain, Website, Bots, Twitch und Caddy zuerst abgefragt. Der erste globale breite Query lief in ein Zeitlimit; der engere Rendererquery war erfolgreich. Der A-Site-Worktree hatte keinen eigenen Graphen. Deshalb wurden dort gezielt die vorhandenen Rust-Sitepfade aus dem Gitbaum gelesen. Keine Graph-Neuerzeugung.

Nicht geführt wurden ein vollständiger XSS-Angriffstest, Abnahme aller öffentlich exportierten Entitäten, Browserfunktionstest, private Vorschauprüfung, Live-G-Rechnung, DB-Bestandstest oder Binaryhashvergleich. Das Vorhandensein eines Renderers, eine alte Aufgabenabnahme und ein laufender Dienst ersetzen diese Beweise nicht. Negative Aussagen gelten für die aufgeführten geprüften Wege, nicht pauschal für jede fremde Aufgabenakte.
