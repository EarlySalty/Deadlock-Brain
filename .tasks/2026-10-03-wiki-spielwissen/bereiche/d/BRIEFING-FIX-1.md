status: bereit
Datum: 2026-10-03
Stand: 2026-10-03T07:36:50Z

# D-Fixrunde 1

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/steam-brain-spieldepot-d

## Integratorwechsel, Punkt41

C2 ist regulär nach gesichertem Handoff beendet. C3 übernimmt allein Integration/Gate/beide Deploys, Paket c/Versuch3, Sol high, Session f83cb4c1-e8c2-4f44-8e83-ada61c2b246a. Operative C2-Verweise unten gelten jetzt für C3. D/Versuch1, Quellscope und einziger bestehender Endstand-Prüfwrapper bleiben unverändert. Keine Code-/Prüftaskänderung durch diese Übergabe.

## Zusatzauftrag Punkt48, 12:25:24Z

Root-Zuweisung vollständig gelesen. Zusätzlich exklusiv D im vorhandenen Steam-Worktree: rust/crates/steam-core/src/api/mod.rs, rust/crates/steam-core/src/steam/cso.rs und rust/crates/steam-core/src/task/handler.rs. Ausschließlich konkrete async_trait-/Attributursache der vier belegten double_must_use-Makrofunde beheben, keine sonstigen Änderungen in diesen Dateien. Diese enge Ausnahme ersetzt das allgemeine API-Verbot unten nur für den genannten Zweck.

Laufenden einzigen Wrapper b89y6ioc2/PID1066919 ungestört erhalten. Erst tatsächliches Ende sowie eigene PID-/Kinder-/FD9-/FD8-Freigabe bestätigen; danach derselbe offene Fixauftrag ohne Doppelwriter. Graphify vor Ursachenbestandssuche. Kein Lint-Allow oder Abschwächen von -D warnings. Neue Quellenbindung umfasst die drei Zusatzdateien; betroffene Format-/Check-/Clippy-/Testprüfungen neu belegen. Vendor-Probleme getrennt am echten Endergebnis messen, keine zusätzliche Mitgliedschaft, Dependency-Upgrades oder Cargo-/Lockmutation ohne belegte Zuweisung. C3 alleiniger Integrator/Deployer; Grant/Zugang unverändert, kein erneuter Login.

## Zusatzauftrag Punkt49, 12:38:24Z

Punkt49 vollständig gelesen. Root erlaubt rust/Cargo.toml: vorhandenen Vendor als member aufnehmen, die bisherigen elf Standardmitglieder explizit als default-members erhalten. rust/Cargo.lock darf minimal nötige reguläre Auflösung dieses Testziels enthalten. Keine Versionsupgrades, neuen Abhängigkeitsdeklarationen, Vendor-Cargoänderungen oder zweite Workspacekonfiguration. Tatsächlichen Auflösungsdiff knapp ausweisen.

Einzigen laufenden Corewrapper bj8t2m1tc/PID1122243 ungestört bis tatsächlichem sicheren Ende erhalten. Erst eigene Wrapper-/PID-/Kinder-/FD9-/FD8-Freigabe belegen, dann im selben offenen Fixauftrag ändern. Gemeinsamer18-Dateien-Freeze, strenge Core-/Vendor-Clippyprüfungen und passende vorhandene lokale Filter mit echten Einzelexits. Punkt48-Trait-/Lifetime-/Send-/Objektvertrag bleibt ausdrücklich Teil unabhängiger Endabnahme. Keine vorzeitige Commitfreigabe, kein Grant/Login/Steam-Aufruf. C3 alleiniger Integrator/Deployer. Dieser Nachtrag ersetzt das bisherige Mitgliedschaftsverbot nur für exakt diesen freigegebenen Weg.

## Zusatzauftrag Punkt53, 14:18:58Z

Punkt53 vollständig gelesen. Root weist den öffentlichen Payloadwechsel NetworkError::Ws(Box<tungstenite::Error>) zentral in der vorhandenen net.rs ausdrücklich zu. Konkrete nötige Callerzeilen im bereits eigenen transport/websocket.rs sind mit umfasst. Keine weiteren Auth-, Session-, GC-, Vendor- oder Callerdateien ohne belegten Bedarf. Die zwei eigenen message.rs-Lints gehören weiter zum engen Auftrag.

Altes bho5pp33s ist sicher beendet: Caller bestätigt Exit101, eigene PID1515805 nicht mehr vorhanden, Kinder none und geschlossene FD9/FD8. Derselbe einzige Fixer setzt erst danach um, ohne Zweitwriter oder identische rote Gesamtwiederholung. Kein Lint-Allow oder Abschwächen.

From<tungstenite::Error>, Display, Debug und konkrete source()-Semantik erhalten, einschließlich direktem Downcast auf das innere WsError. Bloßes Box-#[source] oder transparent gilt nicht als gleichwertiger Ersatz. Falls nötig gezielte manuelle Error-/Display-/From-Implementierung nur in net.rs, alle 13 Varianten und deren Quellen-/Textmatrix vergleichen. EResult-Werte, Result-/Send-/Sync-/Caller- sowie legitime TLS-/Empfangsverträge bewahren. Neuer gemeinsamer Freeze, strenge Core-/Vendorprüfungen, vorhandene lokale Tests und konkrete API-/Source-Laufbeweise. Danach frische unabhängige Abnahme, noch keine Worker-Commitfreigabe. C3 alleiniger Integrator/Deployer, kein Grant/Login/Steam-Aufruf.

## 1. Ziel und Vertrag

Du bist frischer nativer Rust-Fixer für Paket d, Versuch 1. Ausschließlich das geerbte GPT 6.1 Sol verwenden, high, niemals xhigh/max/ultracode oder anderer Anbieter. Kein eigener Unteragent, kein T3-Thread. Du übernimmst den vorhandenen Downloader, baust keinen zweiten Pfad. Genau bestätigte offene Funde aus REVIEW-STATISCH-1.md beheben und den engen zusätzlichen Ressourcenbefund aus B2s D-B-LESEVERTRAG-B2.md berücksichtigen.

Vor Arbeiten lesen:
- /home/nathanael/.worktrees/brain-wiki-spielwissen-d/.tasks/2026-10-03-wiki-spielwissen/bereiche/d/REVIEW-STATISCH-1.md
- derselbe D-Bereich: LESEVERTRAG.md, REGISTER.md, UEBERGABE.md
- /home/nathanael/.worktrees/brain-wiki-spielwissen-b/.tasks/2026-10-03-wiki-spielwissen/bereiche/b/D-B-LESEVERTRAG-B2.md
- /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-wiki-spielwissen/AN_BEREICHE.md Punkte 19/20/24/27/28/30/31
- /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/HOSTPROBE.md

Vor Codebestandssuche Skill code-suche und Graphify benutzen, anschließend konkrete gefundene Stellen lesen. Kein Vollgraph-Neubau. Kommentare und Stil des Bestands erhalten. Produktiver Code ausschließlich Rust.

## 2. Eigentum und konkrete Fixwirkung

Eigener Schreibscope im Steam-Worktree:
- rust/crates/steam-core/src/task/handlers/game_download.rs und game_download/
- ausschließlich notwendige bestehende Modul-/Registryzeilen in task/handlers/mod.rs und task/mod.rs
- ausschließlich minimal notwendige Downloaderabhängigkeiten/Lockauflösung in rust/Cargo.toml, rust/crates/steam-core/Cargo.toml, rust/Cargo.lock; Download-Lane in task/lanes.rs
- rust/vendor/steam-vent-0.4.2/src/net.rs: bisheriger enger Rohcodeguard plus explizit zugewiesene Headergrenzen
- zusätzlich exakt rust/vendor/steam-vent-0.4.2/src/connection/filter.rs, src/transport/websocket.rs, src/message.rs gemäß Punkt 31

Keine anderen Vendor-, Auth-, Runner-, API-, Konfig-, B- oder C-Dateien ändern. Keine globale Formatierung oder Refactorings. Keine ENV-Dateien, keine produktive Config per ENV. Originale und fremde Änderungen erhalten.

Funde:
1. Hintergrundempfang vor rohem Frame-/Decoderlogging schützen. Lokaler NoSubscriber der Anfrage genügt nicht. Vorhandene TLS-KeyLogFile-Ausgabe ausschließen, ohne ENV oder Geheimdateien zu lesen. Sichere eigene Status-/Fehlerlogs und legitimes TLS-/Empfangsverhalten erhalten. Keine zweite Auth-/Loginverbindung.
2. Manifest-Einträge und ZIP-Zentralverzeichnis vor voller Objektaufnahme begrenzen. Reale Header-/Multi-Unterrahmen-/Dekompressions-/Gesamtgrößen vor Reservation prüfen. Vorhandene Bibliotheken für Kompression/Kryptografie wiederverwenden, keine Ersatzkryptografie. B2s bestätigter Zusatz: store.rs serialisiert zunächst vollständig mit serde_json::to_vec und prüft danach 256 MiB. Vorhandenen JSON-Schreibpfad während der Serialisierung begrenzen, nicht nach der großen Allokation. Keine bloße Limitvergrößerung.
3. Exakte Manifest-/Inventarbindung: zusätzliche oder abweichende Dateien im Rohroot ohne Löschung ablehnen, niemals als vollständig melden. NOFOLLOW, reguläre Einzel-Hardlinks, exklusives Staging/NOREPLACE, Größen-/SHA-Prüfung und VPK-Nachbarschaft erhalten. Keine fremde Datei löschen oder überschreiben. Nachträgliche Änderungen beim echten B-Lese-/Importübergang gehören C2/B2; D liefert präzisen belegten Vertrag, ändert keinen B-Parser. Container- und Ressourcenhashes getrennt halten.

Fester Task AUTH_DOWNLOAD_DEADLOCK_GAME, ausschließlich Payload {}, App 1422450, bestehende ctx.connection, fester D-Zielpfad und Laneparallelität 1. Bestehende Taskzeitgrenze, Login-/GC-Lanes und EResult-Caller erhalten. Unbekannte numerische Headercodes verlustfrei, bekannte Codes/OK unverändert.

## 3. Arbeitsstand und sichere Übergabe

Branch feat/brain-spieldepot-download, HEAD 4c5621763d5f01c96d7912400517c08aa1c40df1. Quellen uncommittiert. Alter Implementierer ab4689901128d6f75 hat Fixpause bestätigt und seine native Aufgabe beendet. Einziger alter Wrapper bewxt00qh regulär Exit 0, FD9/FD8 geschlossen, eigene Restprozessprüfung 0. Caller hat eigene frühere PIDs 3004503/3004515 am 07:36Z als nicht mehr vorhanden bestätigt. Kein konkurrierender D-Writer.

Alter cargo check -p steam-core -j 2 tatsächlich grün in 7m35s. Cargo.lock jetzt aufgelöst, SHA256 7357592c893a474b3261fb29b3c7116a3baf5c3e8a94cb4cbf5f9b9782fdfc83. Quellen-Sammelhash ohne Lock ebad16a7f484703bfa5f3f9ada31b676bb1f8043fff00c5ffe72c519a7e010ef. Warnung zur künftigen binrw0.15.1-Inkompatibilität; keine Unterdrückung hinzufügen. Tests/fmt/clippy fehlen. Alter Check ist keine Behebung und kein Nachweis des neuen Fixstands.

Nur den zugewiesenen vorhandenen Worktree verwenden, keinen neuen Worktree/Branch/Bau anlegen. Falls Harness EnterWorktree verlangt, den bestehenden Pfad benutzen; bei Ablehnung Regeln nicht umgehen. Kein Schreiben im Kanon.

Keine eigenen Commits, Pushes, Merges, Deploys, Neustarts oder Gate-Aufrufe. Übergib geprüfte Änderungen an teil-d; der eigene finale Commit folgt erst nach Abnahme. C2 ist alleiniger Integrations-/Gate-/Merge-/Deployer und User-Unit-Verantwortlicher.

## 4. Echte Prüfungen und Stopgrenzen

Vor JEDEM Cargo einschließlich fmt/metadaten die beiden blockierenden Hostlocks in festgelegter Reihenfolge tatsächlich erwerben und während des gesamten Schritts halten. Exakte Pfade und Probe aus HOSTPROBE.md. Unmittelbar vor Compilerstart frische NonZombie-Probe, höchstens zwei Jobs. Bei Exit75 beide Locks halten, 30 Sekunden warten und frisch prüfen. Reine Metadata-Ausnahme nur exakt nach Vertrag, keine vollständigen Prozessargumente oder Umgebungen ausgeben. Keine fremden Prozesse stoppen, Locks löschen/umgehen oder Globalsettings ändern. Alle eigenen Kinder beenden/abwarten und FD9/FD8 im selben Wrapper vollständig schließen. RAM und Platte vor Start prüfen. Kein Releasebau.

Nach den Fixes passende echte cargo check, gezielte rustfmt-/cargo fmt-Prüfung ohne fremde Formatänderungen, cargo clippy und bestehende passende Tests einschließlich Vendor-Rohcode-/Grenzfälle und vorhandener Downloaderfälle ausführen. Neue Tests nur soweit für konkrete Fehler sinnvoll. Tests nicht als grün melden, wenn nur vorbereitet. Grenzfälle vor Allocation und legitime positive Callerfälle tatsächlich nachweisen. Synthetische Strukturtests klar von realem Steam-/Downloadnachweis trennen.

Keine Steam-Aufrufe, kein neuer Lizenz-/Loginversuch und kein Live-Downloadtask. Task4931460 hat Appgrant bereits gemessen; niemals wieder einreihen. Kein Kauf, kein Sessionexport, kein DRM-/Anti-Cheat-Eingriff, keine Authumgehung, keine Klartext-Secrets. Konto-Zugänge ausschließlich bestehende verschlüsselte DB-/Secret-Manager-Verfahren, keine neuen Token-Dateien. Keine Credentials/ENV/Prozessumgebungen lesen. Rohspielbestand bleibt privat, keine Veröffentlichung.

Bei zusätzlichem notwendigem Pfad, tatsächlichem Harness-/Compiler-/Versionsblocker oder Kontingentgrenze erhaltenen Stand und präzise offene Ursache an teil-d melden. Lockwartezeit ist kein Ablaufgrund. Einen stabilen eigenen Wrapper nicht wegen Wache stoppen. Jede relevante Änderung entwertet betroffene alte statische Abnahme. Finale unabhängige Folgerunde und C2-Gate kommen nach tatsächlichen Checks.

## 5. Routing und Bericht

Direkter Auftraggeber teil-d, Haupt Codex /root, Paket d/Versuch1. Nur teil-d schreibt D-Statusereignisse, Bereichsregister und Übergabe. Keine fremden Sessions kontaktieren, keine TODO/zentralen REGISTER ändern. Regelmäßige kurze Wache nach 20 Minuten, bis 30 Minuten eigenen Stand prüfen; keine doppelte Wartetask starten.

Bericht an teil-d: exakte Pfade/Diffumfang, tatsächliche Befundbehebung, unveränderte Grenzen und Caller, SHA-256-Snapshot der geprüften Dateien, echte Befehle/Exitcodes/Nachweisorte, eigene Wrapper-/FD-/Kinderfreigabe. Gebaut, geprüft, gemergt und live getrennt; kein fertiger Download oder finale SHA-Abnahme behaupten. Rohbefunde bleiben lokal. Deutscher Bericht mit echten Umlauten, ohne Gedankenstriche, Schreib-Skills anwenden.
