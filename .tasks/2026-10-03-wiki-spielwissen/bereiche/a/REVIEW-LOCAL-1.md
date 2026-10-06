# Lokale unabhängige Vorabprüfung A, Runde 1

Datum: 03.10.2026
Auftraggeber: Teil-Orchestrator A, f01cce67-209b-468e-8abb-ec2070beeaa2
Vertrag: wiki-spielwissen-v1

Urteil fertig: N
Fix nötig: J
Lokale Codeprüfung abgeschlossen: J
Compiler-, Laufzeit- und Gesamtfreigabe: N

WIRKUNGSPRUEFUNG[WP-1]: 3 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 2/2 geprüft

Die Befunde sind unabhängig am vorliegenden Code bestätigt. Die unten beschriebenen Reproduktionen wurden nicht als Rust-Lauf ausgeführt. Diese Prüfung ersetzt weder die getrennte echte Kompilierung und Datenausführung noch Cs Schluss-Gate auf dem integrierten Gesamtstand.

## Geprüfter Stand

Worktree: `/home/nathanael/.worktrees/brain-wiki-spielwissen-a`
Branch laut Auftrag: `feat/brain-wiki-spielwissen-a`
Zweimal gelesener HEAD: `2734c2da4e814ff79953e8e825275b0216a6af16`

Vollständig gelesene neue Quellen, absolute Pfade und SHA-256:

| Datei | SHA-256 |
| --- | --- |
| `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory.rs` | `b93b2c1a76c9b3e35d12cfb02c2c6ae9ff340f9b866b053dd6471015695cadcf` |
| `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory/normalize.rs` | `515765390baedfed94ddad57868853ef2493d918f51c239f02cade9cbd76d631` |
| `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory/storage.rs` | `8c5f233224165cc80a45b3e07fdda127890799a949e46f38d04c48d0ddd647f1` |
| `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory/tests.rs` | `2aecb0e4e8374eeef508eb834deb8316636692e26bbc72e4cf03c302e7d32f12` |

Der zuerst angekündigte normalize-Hash `7a3cbd314f0dc2424abf3c68b874b1ea7bdfe3b6da07365d774f75097fb28d4a` wich bereits bei der Eingangsprüfung ab. Dies wurde sofort gemeldet. Der Auftraggeber bestätigte danach ausdrücklich den oben angegebenen aktualisierten Freeze-Hash. Eingangs- und abschließende Prüfung ergaben für diesen tatsächlichen Stand dieselben vier Hashs. Das Urteil bezieht sich ausschließlich darauf, nicht auf den ursprünglichen normalize-Stand.

Zusätzlich gelesen: die vier zentralen Vertrags-/Auftragsdateien, PRUEFZIEL.md, RECHERCHE.md, DATEN.md, BAU.md und nach Erweiterung des Auftrags DUMPQUELLEN.md. Der gemeinsame HTTP-Pfad wurde in `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/deadlock-brain-core/src/http.rs` nachvollzogen. Die Rust- und Security-Review-Anweisungen wurden gelesen; ihre Modelldefaults und Bauvorschläge wurden nicht ausgeführt. Es blieb beim geerbten GPT 6.1 Sol und bei der ausdrücklich angeordneten rein lesenden Codeprüfung.

Graphify wurde vor den Codefragen global befragt. Die neuen, uncommittierten Module waren darin nicht sinnvoll verzeichnet; anschließend wurden die konkret zugewiesenen Dateien vollständig gelesen und die Fundfamilien gezielt per Grep verifiziert.

## Bestätigte Befunde

### 1. P2: Bekannte Revision geht bei fehlendem oder unterdrücktem Inhalt verloren

Fehlerklasse: F7, belegte Identität wird verworfen; zusätzlich F10, das Inventar beschreibt den Versionsstand falsch.

Fundstelle: `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory/normalize.rs:304-339` und `:457-460`.

`inventory_page.revision` beginnt mit `None`. Der unterdrückte Inhalt führt auf Zeile 315, fehlender Inhalt ohne Extract auf Zeile 328 zum nächsten Schleifendurchlauf. Erst danach wird `revid` gelesen. Die Revision gelangt außerdem nur im Dokumenterzeugungspfad ins Inventar. Deshalb bleiben bekannte IDs aus genau den nicht verfügbaren Revisionen ungespeichert.

Reproduzierbares Szenario:

1. Gültigen Quellenkontext und Beobachtungszeit verwenden.
2. `normalize_api_response` eine Seite mit `pageid=1`, `ns=0`, `title="Example"`, einer Revision `revid=101` und `slots.main.texthidden=true` übergeben.
3. Ergebnis des gelesenen Pfades: keine Dokumente, Lücke `revision_content_suppressed`, aber Inventarrevision `None` statt der belegten `101`.
4. Dasselbe gilt bei `revid=101` ohne Inhalt und ohne Extract.
5. Bei einer lesbaren Revision 101 und einer neueren unterdrückten Revision 102 im selben History-Block bleibt die Inventarrevision sogar 101 und `content_available=true`. Das Inventar trennt den neuesten bekannten Stand nicht vom zuletzt verfügbaren Inhalt.

Wirkung: unterdrückte und wirklich unbekannte Versionen sind nicht mehr zuverlässig unterscheidbar. Das trifft API und XML, weil XML dieselbe Normalisierung benutzt. Ein Fix muss die neueste belegte Revision unabhängig von der Verfügbarkeit ihres Textes erhalten und die Inhaltsverfügbarkeit eindeutig daran binden beziehungsweise separat ausweisen.

Zwillingssuche: Grep auf `continue;`, `revision_id`, `texthidden`, `contenthidden` und `newer_revision` belegt beide Early-Returns sowie den gemeinsamen XML-Aufruf auf Zeile 528. Der vorhandene Test `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory/tests.rs:290-298` prüft fehlende Ausgabe und Lücke, aber keine erhaltene Revisions-ID. Die vom Auftraggeber genannte erste Vermutung ist bestätigt.

### 2. P2: Älterer Capture oder späterer alter Seitenblock setzt die Inventarrevision zurück

Fehlerklasse: F10, der zuletzt verarbeitete Eingang wird als maßgeblicher Versionsstand ausgegeben.

Fundstellen: `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory.rs:394-395`; Zwilling im Netzpfad `:325-326`.

Beide Stellen ersetzen den vorhandenen `WikiInventoryPage` bedingungslos per `insert`. Der Vergleich `newer_revision` schützt nur Revisionen innerhalb eines einzelnen Aufrufs von `normalize_page`, nicht verschiedene Captures oder wiederholte Seitenblöcke derselben Seite.

Reproduzierbares Szenario:

1. Mit `write_wiki_api_capture` in einen leeren Spool Seite 1, Revision 102, Inhalt `new` schreiben.
2. Danach denselben Spool mit Seite 1, Revision 101, Inhalt `old` beschicken.
3. Beide Revisionsdateien bleiben korrekt erhalten. Der Checkpoint und `inventory.json` zeigen für Seite 1 anschließend jedoch Revision 101.
4. Ebenso zwei XML-Seitenblöcke derselben ID verwenden, zuerst mit Revision 102, danach mit Revision 101. `normalize_mediawiki_export` erhält beide Dokumente, aber `persist_offline` übernimmt den letzten Seitenblock als Inventarstand.

Wirkung: die aktuelle Auswahl des Inventars hängt von der Reihenfolge der Eingabedateien beziehungsweise Seitenblöcke ab. Gerade die neu gemeldeten History-Dumps mit auf mehrere Blöcke verteilten Historien benötigen eine seitenweite Zusammenführung. Die Revisionen im unveränderlichen Spool gehen durch diesen Fehler nicht verloren.

Zwillingssuche: Grep auf `pages.insert`, `persist_document` und `newer_revision` belegt beide ungeschützten Ersetzungen und den nur lokalen Vergleich in normalize.rs:459. Der Test `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory/tests.rs:154-185` behandelt nur die Reihenfolge alt, dann neu. Die zweite vom Auftraggeber genannte Vermutung ist bestätigt.

### 3. P2, Security/Ressourcen: HTTP-Bytegrenze greift erst nach vollständigem Download und Speicherbelegung

Fehlerklasse: F6, Schutz hinter dem zu begrenzenden Effekt; F11, konfigurierte Grenze begrenzt die tatsächliche Ressourcennutzung nicht.

Fundstelle: `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory.rs:175-197`.
Gemeinsamer Aufrufpfad: `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/deadlock-brain-core/src/http.rs:129-132` und `:211-232`.

Der Sammler prüft `response.content.len()` erst, nachdem der gemeinsame HTTP-Client `response.bytes()?.to_vec()` ausgeführt hat. Der vollständige, unbegrenzte Körper wird also vor der 8-MiB-Standardgrenze in den Speicher geladen. Auch 401/403 werden im gemeinsamen Client erst nach dem Einlesen des Körpers als Statusfehler zurückgegeben.

Reproduzierbares Szenario für einen getrennten lokalen Transporttest, ohne Anfrage an die gesperrte Wiki:

- Mit der bestehenden `HttpClient::get_no_redirect`-Testmethode eine lokale Antwort mit einem deutlich größeren Körper als der konfigurierte Sammlergrenzwert ausliefern, etwa 100 MiB bei `max_response_bytes=8 MiB`, oder einen langen Chunked-Body.
- Der gelesene Pfad liest den ganzen Body; der Sammlerfehler entsteht erst danach. Ein Speicherabbruch kann den Grenzfehler und die dauerhafte Speicherung einer Zugangssperre verhindern.

Wirkung: die als harte HTTP-Grenze angegebene Option schützt weder Downloadvolumen noch Spitzenspeicher. Der Fix benötigt einen begrenzten Lesepfad im gemeinsamen HTTP-Baustein oder eine ausdrücklich zugewiesene passende Schnittstelle von C. Nur ein größeres Limit behebt den Fehler nicht.

Zwillingssuche: Grep auf `max_response_bytes`, `response.bytes`, `fetch_once` und `get_no_redirect` bestätigt den gemeinsamen unbegrenzten Leser. In der Spoolfamilie gibt es einen verwandten Nachprüfpfad: `storage.rs:80-85` liest den gesamten Checkpoint vor der Größenprüfung. Die regulären Dokumentdateien werden dagegen bereits beim Öffnen anhand ihrer Dateigrößen gegen die Gesamtsumme geprüft (`storage.rs:43-67`). Dieser Befund ist statisch bestätigt; ein OOM-Versuch wurde nicht ausgeführt.

## Geprüfte unauffällige Pfade und verbleibende Grenzen

### Inventar, Inhalte und Herkunft

- Die realen, nichtnegativen Namespace-IDs stammen aus `siteinfo`; virtuelle Namespaces werden ausgeschlossen. Kein Filter auf `content=true`. In der direkt gelesenen aktuellen Originalantwort sind Update 3000, Data 3002 und Bucket 9592 tatsächlich belegt. Fortsetzungswerte werden einschließlich `continue`, `gapcontinue` und `rvcontinue` wieder in die Anfrage übernommen; unbekannte Schlüssel und wiederholte vollständige Cursor werden abgewiesen.
- Offline-Captures behaupten keine aktuelle globale Vollständigkeit. `content_complete` benötigt einen abgeschlossenen Netzlauf, keine Lücken und verfügbare Seiteninhalte. Statistikzahlen ersetzen nicht den Inventarnenner. Diese Aussagen sind Codebefunde, kein Beweis eines tatsächlich durchlaufenen Live-Inventars.
- API-v1- und v2-Seitenlisten sowie `slots.main["*"]` und `slots.main.content` sind abgedeckt. Im direkt gelesenen echten Ability-Capture ist genau das v1-Format vorhanden, einschließlich der Redirectzuordnung Ability zu Abilities ohne eigene Redirectquell-ID. Der Sammler macht daraus keinen zusätzlichen vollständig gesicherten Redirectartikel.
- Wikitext wird als übergebener String erhalten; der SHA-256 entsteht direkt aus `content.as_bytes()`. Der zusätzliche Extract wird getrennt gespeichert und als nicht gepinnte Renderbeobachtung markiert. Kein Nachweis erzeugter JSONL-Hashes wurde durch diese Prüfung vorgetäuscht.
- Historischer Capture, Revisionszeit, ursprünglicher Abruf und spätere Lizenzbeobachtung sind getrennte Metadaten. Der heutige allgemeine Lizenznachweis wird nicht als bestätigte Lizenz jeder alten Revision ausgegeben. `redistribution_allowed=false` bleibt erhalten.
- Templates und Module werden nicht ausgeführt. Ihre syntaktische Erfassung wird ausdrücklich nicht als vollständige Expansion oder gepinnter Renderstand ausgegeben. JSON-Fakten sind generische Originalblattwerte mit JSON-Pfad; keine Umrechnung und keine Zuordnung zu vermeintlich bestätigten Fähigkeitsmechaniken. Einheit und Bedingungen bleiben im vollständigen Original-JSON erhalten. Die Blattfakten allein sind kein semantischer Mechanikdatensatz.

### Dauerhaftigkeit und Wiederaufnahme

- Der Versionsschlüssel verwendet Dokument-ID und Revision. Gleicher Inhalt ist idempotent; anderer Inhalt unter demselben Schlüssel wird als Konflikt gesichert und führt zu einem Fehler statt zum Überschreiben.
- Im Netzlauf werden alle Dokumente eines Batches dauerhaft geschrieben, bevor dessen Seiten und Cursor in den Checkpoint gelangen. Bei Unterbrechung zwischen Einzeldokumenten bleibt der alte Cursor bestehen; ein erneuter Lauf kann bereits gesicherte Dokumente idempotent wiedersehen. Dies wurde anhand der Reihenfolge geprüft, nicht durch Prozessabbruch getestet.
- JSONL wird in eine neue temporäre Datei geschrieben, geflusht und synchronisiert und erst dann umbenannt. Checkpoint, JSONL und Inventar sind einzelne atomare Veröffentlichungen, keine gemeinsame Transaktion. Der erneute Berichtspfad kann die abgeleiteten Dateien wiederherstellen. Crash-/ENOSPC-Läufe fehlen weiterhin.
- Aus Wiki-Titeln entstehen keine direkten lokalen Dateipfade. Revisionsdateinamen bestehen aus Hashs. Der Spool sperrt konkurrierende Bearbeitung und verwirft Symlinks unter den regulären JSON-Revisionsdateien. Das Ausgabeverzeichnis ist ein vertrauenswürdiger Aufruferparameter, kein geprüftes öffentliches Uploadziel.

### Beide Fremddienst-Pfade

1. siteinfo-Abruf: feste HTTPS-Adresse `https://deadlock.wiki/api.php`, kein Redirect-Follow, ein Versuch, mindestens fünf Sekunden Abstand und explizite Netz-/Zugriffsfreigabe. API-Fehler werden zurückgegeben; die Bytegrenze hat Befund 3.
2. Namespace-/Revisionsabruf: dieselben gemeinsamen HTTP-Einstellungen, keine Ersatzidentität und kein alternativer Host oder Endpunkt. 401/403, benannte API-Zugriffsfehler und als HTML gekennzeichnete Antworten setzen die persistente Sperre. Der ursprüngliche Fehler wird zurückgegeben. Nach einem regulären Nichtzugangsfehler ist Wiederaufnahme über den bestätigten Cursor möglich. Die Bibliothek hat keine eigene vollständige Journalprotokollierung; der kommende CLI-Aufrufer von C muss Erfolg, Fehler, Sperre und Wiederaufnahme sichtbar melden. Ein HTML-Challenge-Körper mit fehlendem oder falschem Content-Type fällt zunächst in die JSON-Fehlerbehandlung statt in den HTML-Sperrpfad; dieser spezielle Serverfall ist hier kein verifizierter tatsächlicher Quellenbefund.

## Echte Daten und nachträglich erweiterter Dump-Scope

Der zwölfseitige Rohordner wurde mit `diff -qr` gegen den vorhandenen ursprünglichen Rohordner verglichen: keine Unterschiede. Die zwölf Dateipfade wurden unmittelbar aufgelistet. Der Ability-Rohkörper und ein Stats-Cache-Sidecar wurden direkt gelesen. Eine vollständige eigene Nachzählung und Hashprüfung aller zwölf erzeugten Vertragsdokumente fand nicht statt, weil noch keine eigene Rust-Ausführung autorisiert war.

Direkt verifizierte Belegdateien:

- `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/source-evidence/wiki-siteinfo.json`: SHA-256 `fb2a84a4fa153f1533b45f4a6b20c30aa22813fcb16a2406e9af96c379917092`.
- `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/source-evidence/wiki-allpages-main-first.json`: SHA-256 `56aa459ae2ebda7c86e15737245ac42551a88b48884fe348e7ab6ccc9c4a143e`. Der Hash bestätigt den erhaltenen Körper, nicht unabhängig den damaligen HTTP-Status; separate Originalheader fehlen laut Recherche.
- `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/dump-evidence/deadlock.wiki-20250415-history.xml.zst`: selbst gemessen 3.612.285 Bytes, SHA-256 `bea69e14f1ec35b8bbfe157b3cf9cf362f888e2321e9aa5a46a03c1670120825`.
- Die SHA-256-Werte der vier weiteren komprimierten Archive wurden ebenfalls unmittelbar nachgerechnet und stimmen mit DUMPQUELLEN.md überein.

Die zusätzlich gelesene historische `dump-evidence/siteinfo.json` enthält `server=https://deadlock.wiki`, Zeit `2025-04-15T17:16:16Z`, Data 3002, Update 3000 und keinen Bucket-Namespace. Sie ist als historischer Kontext von der heutigen siteinfo-Beobachtung zu unterscheiden.

Für den entpackten Hauptdump nennt DUMPQUELLEN.md 177.037.503 XML-Bytes, 4.417 eindeutige Seiten und 22.742 eindeutige Revisionen. Diese Zahlen wurden in dieser Prüfung nicht unabhängig aus dem XML nachgezählt. Ein rein lesender Entpack-/Hashversuch scheiterte daran, dass `zstd` nicht installiert ist. Der dabei vom nachgeschalteten Hashprogramm ausgegebene Leerstream-Hash ist ausdrücklich kein Dumpnachweis. Die komprimierten Originaldateien wurden nicht verändert.

Codeurteil zum erweiterten Scope:

- `write_wiki_export_capture` prüft XML gegen `max_total_bytes`, nicht gegen die 8-MiB-HTTP-Grenze. 177.037.503 Bytes liegen unter dem Standard von 536.870.912 Bytes. Der deklarierte Eingabeumfang wird damit nicht allein wegen seiner Größe abgewiesen.
- `normalize_mediawiki_export` läuft über jeden Seitenblock und jede darin vorhandene Revision; es gibt keinen Filter auf die letzte Revision und keine Stichprobenauswahl für den Dokumentspool. Doppelte identische Revisionsschlüssel können vom Spool idempotent zusammenfallen. Der Inventarfehler aus Befund 2 bleibt bei wiederholten Seitenblöcken bestehen.
- Der komplette XML-Baum und sämtliche normalisierten Dokumente werden vor Beginn der Persistenz im Speicher aufgebaut. Erfolg, Speicherspitze, JSON-Faktengrenzen und vollständig erhaltene Revisionen für den echten 177-MB-Dump bleiben deshalb Gegenstand des getrennten Rust-Datenlaufs. Die Eingabegröße allein beweist keinen erfolgreichen vollständigen Lauf. Ein Abbruch darf nicht durch Weglassen von Historie oder Samples verdeckt werden.
- Der eingefrorene Sammler ist ausschließlich auf `deadlock-wiki` ausgelegt. Abweichende XML-Basisdomains werden auf normalize.rs:482-491 zurückgewiesen; bei vorhandener abweichender siteinfo-Serveradresse gilt dieselbe Ablehnung auf :47-55. Die anderen vier Archive können so nicht regulär als eigenständige Quellen importiert werden. Das ist eine offen benannte Integrationsgrenze, keine Erlaubnis, ihre Domains oder IDs umzuschreiben.
- `WikiSourceContext` besitzt kein eigenes Source-ID-Feld; Dokument-IDs und Locators sind fest an deadlock.wiki gebunden. Fehlende Server-/Basisangaben werden nicht als Fehler behandelt. Für unabhängige Wikis braucht C daher einen ausdrücklich getrennten Quellenpfad oder eine geprüfte Erweiterung. Identische numerische IDs oder der gleiche XML-Wiki-ID-Text sind kein ausreichender Herkunftsnachweis. Diese Prüfung bestätigt keine Vermischung der echten Archive, da kein solcher Import ausgeführt wurde.

## Prüfgrenzen und Übergabe

Keine neuen Agenten oder T3-Threads. Keine Secrets, ENV-Dateien, Live-Wiki-Abfragen oder Community-Nachrichten. Keine Cargo-/Release-/Testläufe, Modulregistrierungen, Codeänderungen, Datenüberschreibungen, Commits, Pushes, Merges, Migrationen oder Deploys durch diesen Prüfer. Ausschließlich dieser Bericht wurde geschrieben.

Die geplante rein lesende Gesamtauswertung des alten Rohbestands per Node wurde vom Harness als indirekte Shell-Ausführung blockiert. Sie wurde nicht über einen anderen Ausführungsweg umgangen. Die stattdessen zulässigen direkten Dateilesungen, Rohordnervergleiche und Standard-Hashaufrufe sind oben mit ihrem engeren Beweisumfang angegeben. context-mode und EnterWorktree wurden gemäß Briefing nicht erneut versucht.

Formatcheck, echte Kompilierung, die elf vorhandenen Tests, vollständiger historischer XML-Lauf, idempotente Wiederholung auf Echtdaten, Crash-Wiederaufnahme und Import in die vorhandene Postgres-/Brain-Lesekette bleiben offen. Synthetische Reproduktionsszenarien sind ausschließlich Fehlerbeschreibungen und zählen nicht als Echtdatenabdeckung.

Die drei bestätigten Befunde gehen ausschließlich an Teil-Orchestrator A zur Bearbeitung durch einen frischen Fixer. Kein Befund wurde an den Modulautor geschickt. Cs unabhängiger Gesamt-Gate auf dem späteren integrierten SHA bleibt unverändert erforderlich.
