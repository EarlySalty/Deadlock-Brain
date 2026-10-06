status: vorbereitet
Datum: 2026-10-03

# D: unabhängige Folgerunde nach finalen Checks

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/steam-brain-spieldepot-d

Dieses Briefing vorbereitet lassen. Kein Prüferstart vor bestätigtem endgültigem Quellfreeze, tatsächlichen passenden Checks, eigener Wrapper-/FD-/Kinderfreigabe und festgehaltenem Prüfsnapshot. Keine Abnahme anhand des vorläufigen b1jf0zj7l-Laufs. teil-d ergänzt vor Start tatsächliche Beweisdatei und Snapshotbindung.

## 1. Ziel und Vertrag

Du bist frischer unabhängiger nativer Rust-Prüfer, ausschließlich geerbtes GPT6.1Sol high. Kein anderer Anbieter, Modellwechsel, xhigh/max oder eigene Unteragenten/T3-Threads. Ausschließlich lesen, keine Compiler, Formatierung, Tests, Sperren, Netzwerk-/Steam-Aufrufe oder Quelländerung.

Prüfe den endgültig eingefrorenen Downloader-/Vendorstand gegen den genehmigten Nutzerauftrag und die konkrete Wirkung der Fixes. Rust-Codebereitschaft für C3 und Erfüllung des gesamten D-Downloadziels getrennt beurteilen. Null gemessene Spieldateien bleiben null; grüne Struktur-/Lokaldaten-/WebSockettests beweisen keinen Steam-/Depotdownload. Keine eigene zentrale Merge-Gate-Abnahme, die gehört C3.

Vor Bestandssuche Skill code-suche/Graphify, konkrete gefundene Quellen danach lesen. Skills rolle-wirkungs-pruefer und no-em-dashes anwenden, natürliche deutsche Berichte mit echten Umlauten. Pflichtzeile WIRKUNGSPRUEFUNG[WP-1]: <n> Befunde | Zwillingssuche: <grep-belegt|keine Fundstelle> | Fremddienst-Pfade: <k>/<k> geprüft.

## 2. Eigentum und Referenzen

Steam-Worktree /home/nathanael/.worktrees/steam-brain-spieldepot-d, Branch feat/brain-spieldepot-download. Ausgangs-HEAD4c5621763d5f01c96d7912400517c08aa1c40df1. Keine Quellen oder eigene Berichtsdateien schreiben. Fachbericht nur an direkten Parent main; teil-d ist Produzentenname, keine fremde Sessionadresse. Nicht mit Implementierer/Fixer oder fremden Sessions koordinieren.

D-Akte /home/nathanael/.worktrees/brain-wiki-spielwissen-d/.tasks/2026-10-03-wiki-spielwissen/bereiche/d/: REVIEW-STATISCH-1.md, BRIEFING-FIX-1.md, LESEVERTRAG.md, REGISTER.md und aktuelle UEBERGABE.md. B-Bereich im bestehenden Brain-B-Worktree: D-B-LESEVERTRAG-B2.md. Zentrale AN_BEREICHE.md Punkte19/20/24/27/28/30/31 sowie43/46/48/49 nur lesen. Punkte43/46 beschreiben zusätzliche gehaltene B-Leseobjekte unter C3/B-Eigentum, nicht D-Implementierung oder alleinige D-Abnahme. Die späteren B-Handleänderungen zum48b6-Basiscommit getrennt binden; keinen unverändert aktuellen B-SHA voraussetzen.

Geprüfter Scope umfasst game_download.rs/game_download/, minimale Downloader-Cargo-/Lock-/Modul-/Registry-/Laneänderungen sowie exakt steam-vent-0.4.2-Vendordateien connection/filter.rs, transport/websocket.rs, message.rs, net.rs. Punkt48 ergänzt exakt steam-core/src/api/mod.rs, steam/cso.rs und task/handler.rs für vier belegte double_must_use-Makrofunde. Punkt49 ergänzt im Root-Cargo nur vorhandenen Vendor als member, bisherige elf Defaultziele explizit als default-members sowie minimale reguläre Root-Lockauflösung. Keine Upgrades, neuen Dependencies, Vendor-Manifeste oder weitere Workspacekonfiguration. Keine unbemerkten sonstigen Auth-/Vendor-/Runner-/Konfig-/B-Dateien zulassen. B-Snapshot separat hashen, weil B2 weiterarbeiten kann; keine falsche Behauptung finaler B-SHA-Bindung.

B-Commit für diese Runde: 48b6ce1cf277ec4de4b47ac4a0898fbc223fe6b5, laut Root geprüft. Caller bestätigt am 03.10.2026 um 10:31:31Z Commitobjekt und bytegleiche aktuelle Kopien für game_files.rs, game_files/vpk.rs und game_files/anchored.rs. SHA256: Hauptmodul b26a5035019b05c9920c8af14c30857a817d503882025874f73d24b4aee4e3da; VPK 142cefcbeafdb89c78b59d46dd6b0a2f3ee647f6401889c16d22c519834bf046; anchored e6bd4e5221d8806e85f043dc90a116db87848f38a45bad57dd37047858e09474. Alte B-WIP-Hashes keine finale B-Bindung. Vertrag erneut gegen diesen Commit prüfen, übrige Module und aktuellen Zustand selbst binden.

## 3. Konkret zu prüfende Wirkung

1. Empfangsschutz: niedrige Frame-/Decoderlogs im schon gestarteten Hintergrundempfänger deterministisch vor Geheimwertausgabe gesperrt; lokale Anfrage-Wrapping alleine reicht nicht. Zwillingssuche umfasst auch separat gestartete HTTP-/Transporttasks: Geheimwerte in Anfragepfaden, Headern oder Frames dürfen nicht durch außerhalb des lokalen Dispatchers gepollte Bibliotheksfutures protokolliert werden. Tatsächliche Abdeckung anhand der konkreten Bibliothekspfade prüfen, keinen zusätzlichen Fehler ohne Beleg behaupten. TLS-KeyLogFile ausgeschlossen, sichere Status-/Fehlerlogs erhalten. Legitime Empfangs-/TLS-/Login-/GC-Caller bewahren; kein neuer Login oder Sessionexport. Nie echte Geheimwerte lesen/erzeugen oder Trace in Produktion anstellen.
2. Ressourcen: reale Header-/Multi-Unterrahmen-/Dekompressions-/Gesamtgrößen und Manifest-/ZIP-Einträge vor Reservation/Objektaufnahme begrenzen. B2s Inventarserialisierung während des Schreibens begrenzen, nicht erst nach to_vec. Grenzen wirksam bei konkreten adversen Eingaben und positive legitime Fälle erhalten. Keine bloße Limitvergrößerung oder Ersatzkryptografie.
3. Exakte Rohrootbindung: zusätzliche/abweichende Dateien erhalten, aber unbelegte Manifestprovenienz und falsche Vollständigkeit verhindern. Relative Pfade, Typ, Größen und Originalhashes verbinden. NOFOLLOW je Komponente, Einzel-Hardlinks, exklusives Staging/NOREPLACE und Resumeprüfungen bewahren. VPK-Nachbarschaft und separate physische Container-/Ressourcenhashes. C3/B2s späterer echter Lese-/Importübergang bleibt eigener Laufzeitnachweis.
4. Enger nachgelagerter Pfadbudgetfix: Root::inventory in game_download/store.rs klonte zunächst jeden vollständigen Pfad und implizite Elternstrings vor 300.000-Aufnahmegrenze. Endstand muss vorhandene Manifeststrings wiederverwenden und vorhandene Bestandsgrenze vor neuer Aufnahme durchsetzen. Viele lange/tiefe Pfade nicht erneut ungebunden vervielfachen. Keine neue Budgetanhebung.
5. Zentraler EResult-Guard: unbekannte i32-Headercodes verlustfrei, bekannte Codes/OK und fehlende Headerwerte unverändert. Caller einschließlich job/job_multi, service_method, one/on, Login, GC, Notifications und gepufferte Friends-/Persona-Nachrichten nachverfolgen. Fester Task/Payload{}/App1422450/Ziel, vorhandene Verbindung, Lane1 und bestehende Zeitgrenze. Alle acht Fremddienstpfade erneut auf Nein/Unsicher/Timeout/Fehler und tatsächlich passenden Rohcode/Sicherheitskontext prüfen.

6. Punkt48 unabhängig und ausdrücklich abnehmen: vier neue BoxFuture-Traitdeklarationen gegen tatsächliche vorhandene async_trait0.1.89-Makroexpansion prüfen, einschließlich unabhängiger Borrow-Lifetimes/Outlives, Self-Bounds, Future-Ausgabe, Send, dynamischer Traitobjekte und bestehender Implementierungen/Aufrufpfade. Ursache muss ohne Lint-Allow/Abschwächen behoben sein. Änderungen in den drei Zusatzdateien nur dafür. Vorhandene API-/CSO-/TaskHandler-Tests und tatsächlich kompilierte Runnerimplementierungen als Beleg einordnen; nicht gelaufene DB-Runner-Tests als offen nennen, niemals aus Kompilierung Laufzeit ableiten.
7. Punkt49 Workspacevertrag: Vendor im gemeinsamen tatsächlich aufgelösten Graph als Testziel, elf bisherige default-members exakt erhalten, normale Standardziele unverändert. Root-Lockdiff auf minimal nötige Auflösung prüfen, keine Versionsupgrades oder neuen Abhängigkeitsdeklarationen. Vendor-Cargo.toml/Cargo.lock bytegleich. Strenge Core-/Vendor-Clippy-Läufe und vorhandene Vendor-Testfilter müssen tatsächlich auf dem finalen gemeinsamen Snapshot gelaufen sein; bloße Metadata oder vor Änderung grüne Downloaderprüfungen reichen nicht.

Punkt49-Auflösung ergänzte laut tatsächlichem Diff genau drei bereits optionale steam-vent-proto-csgo/dota2/tf2-Pakete0.5.2 (477 auf480); ursprüngliche477Identitäten/Versionen/Quellen/Checksummen unverändert. Der eigene Gesamtanzahlguard stoppte, nicht der Merge-Gate. Diese Einordnung unabhängig am echten Diff prüfen, insbesondere optionaler Lockeintrag gegen tatsächliche Featureaktivierung, vorhandene Deklarationen und exakten Baselineerhalt. Keine pauschale Ausnahme für weitere Ergänzungen.

## Punkt53: öffentlicher Fehlervertrag ausdrücklich zugewiesen

Punkt53 vollständig gelesen. Root erlaubt Ws(Box<tungstenite::Error>) nur zentral in net.rs, nötige konkrete Calleranpassung im bereits eigenen transport/websocket.rs mit umfasst. Kein weiterer Auth-/Session-/GC-/Vendorpfad oder Lint-Allow. Endgültige Lösung und tatsächlichen neuen Prüfsnapshot vor Prüferstart ergänzen.

Öffentlichen Konstruktor-/Matchpayloadwechsel exakt benennen. Vorherige konkrete From<tungstenite::Error>-, Display-, Debug- und source()-Semantik unabhängig erhalten prüfen, einschließlich source().downcast_ref::<tungstenite::Error>() auf den direkten inneren Fehler. Nur Text-/Fehlerkettengleichheit reicht nicht. thiserror-Box-#[source] exponiert laut lesendem Befund den Box-Typ, transparent überspringt den unmittelbaren Source-Schritt; tatsächliche Generierung und Laufbeweise nachlesen.

Vertragsmatrix vor Punkt53 laut Fixer:13 öffentliche nicht erschöpfende Varianten, vier generierte From-Konvertierungen für std::io::Error, WsError, MalformedBody, CryptError und manuelles From<EResult>. Genau IO/Ws/MalformedBody/CryptoError liefern konkrete innere source(), übrige neun None. Korrigierter Vorbeleg: CryptoError hatte den Präfix „Crypto error: “; DifferentServiceMethod nutzte auch für den erhaltenen Namen Debugformatierung. Die frühere pauschale {0}-Behauptung war für CryptoError unvollständig. Sämtliche alten Formattexte gegen den tatsächlichen Vor-Punkt53-Derivevertrag prüfen, ApiError über EResult-Debug und UnknownApiError mit originalem i32. Result<T,E=NetworkError>, Send/Sync und numerische Headerguards erhalten. Matrix selbst gegen vorigen belegten Snapshot verifizieren, nicht bloß Workerbehauptung übernehmen. Vergleichsorakel darf keine aus dem neuen Code nachgebauten Erwartungswerte als unabhängigen Beweis verwenden.

Lesende Referenzen thiserror2.0.18: thiserror-impl/src/expand.rs:241-259 source.as_dyn_error(), :235-240 transparent; thiserror/src/aserror.rs:9-12 blanket AsDynError gibt konkreten Typ zurück. Eine nötige manuelle Error-/Display-/From-Umsetzung nur in net.rs gegen jede alte Variante und passende tatsächliche Laufbeweise vergleichen. Caller-/TLS-/Empfangs-/Login-/GC-Verträge erhalten, keine pauschale Vorbestandsfreigabe. Unbekannte/bekannte/OK EResult-Fälle nach Änderung tatsächlich neu geprüft.

## 4. Beweise und Urteil

Snapshot vor und nach der lesenden Runde hashen und Übereinstimmung zum vom Parent ergänzten tatsächlichen Prüfsnapshot feststellen. Bestehende echte check/fmt/clippy/test-Logs lesen und Befehle/Exitcodes von bloß vorbereiteten Testfällen unterscheiden. Historischer ursprünglicher check und vorläufiger erster Fixcheck reichen nicht für nachfolgende Sourceänderungen. Keine Tests selbst starten oder ausführen.

Nur verifizierte Funde melden: Datei:Zeile, konkrete Eingabe/Zustand→falsche Wirkung, Fehlerklasse, Zwillingssuche und vorgeschlagener enger Fix. Fehlende Laufzeit-/Download-/B-C-Beweise getrennt von tatsächlichen Codefehlern. Auch unauffällige Wirkungsketten mit Urteil festhalten. Bei Quellenänderung während der Runde keine finale Abnahme behaupten, sofort Parent melden.

Ausgabe: WIRKUNGSPRUEFUNG-Pflichtzeile, nummerierte verifizierte Befunde nach Schwere, Scope/Quellenhashbindung, tatsächliche Nachweise und offene echte Laufzeitbeweise. Intenturteil konkret: Codeübergabe bereit J/N, Abweichungen, Codefix nötig J/N, gesamter gemessener D-Downloadauftrag fertig J/N. ALLOW für Code bedeutet nie tatsächlich gemergt/deployt oder vollständig heruntergeladen.

## 5. Routing und Sicherheitsgrenzen

Direkter Auftraggeber teil-d, Haupt Codex /root, Paket d/Versuch1. Parent main erhält Rohbericht und veröffentlicht kurze Übergabe/Status. Nur Parent schreibt D-Register/Status, zentrale TODO/REGISTER unangetastet. Nach etwa20 Minuten kurze Wache, keine eigene neue Bearbeitung starten.

Keine Datei-/Permission-/Settingsänderung, keine Credential-/ENV-/Prozessumgebungslesung, keine Token-Datei, kein Kauf, Lizenz-/Loginversuch, Sessionexport, Steam-Aufruf oder DRM-/Anti-Cheat-/Authumgehung. Kein Commit/Push/Gate/Merge/Deploy/Neustart. C3 alleiniger Integrations-/Deployer. Bei tatsächlichem Harness-Deny nicht auf anderem Weg gleiche verweigerte Aktion ausführen, genaue Grenze an Parent melden.
