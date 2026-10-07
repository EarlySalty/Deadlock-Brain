# A-F3: Brain-Site und vorhandene Steckbriefauslieferung

## Ziel und Vertrag

Nativer Blatt-Worker, keine weitere Delegation. Auftraggeber Paket A `2c7de4c9-bac4-43ad-b91a-f8ac889f09b4`, Hauptorchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Lies `../AUFTRAG.md`, `../BRIEFING-A.md`, `PAKETE.md`, `INVENTUR-KERN.md`, `INVENTUR-WISSEN.md` und die D5-Siteverträge gezielt.

Der Steckbriefrenderer und Entitätsreader bestehen, Ausgabeordner und Seiten fehlen. A-F1 repariert die gemeinsame Profil-/Gitverarbeitung. Dein Bereich ist ausschließlich der bestehende Site-/Auslieferungspfad. Zuerst mit Graphify und realer Laufzeit klären, welche tatsächliche URL hinter der vorhandenen öffentlichen `/brain`-Seite auf Port 8087 steckt und wie der Renderer dort angeschlossen ist. Einen durch fehlende generierte Dateien erklärten 404 nicht als neue Route behandeln.

Die aktive Site läuft laut Inventur mit Python. Python bleibt reine Referenz, nicht produktiv erweitern oder fixen. Vorhandenen Rust-Siteweg global suchen und wiederverwenden. Fehlt er tatsächlich, den vorhandenen Dienst mit seinen relevanten bisherigen Routen und sichtbaren Funktionen direkt in Rust portieren, bestehende rendererzeugte HTML-Dateien verwenden. Keine neue Standalone-Website, kein Redesign, keine Grafiken, kein Steam-Depot oder neuer Reader. Nicht heimlich Nutzerfunktionen weglassen: nötige API-/Funktionsabweichung exakt melden statt passend biegen.

## Eigentum und Stand

Neuer eigener Worktree `/home/nathanael/.worktrees/brain-a-site-20261006`, Branch `fix/brain-a-site-20261006`, frisch von Brain `origin/main`; Inventurbasis `d6131cc52711a3e8b02d299704244f8d7dbdbce6`. Keine Sitzungsisolation per EnterWorktree ändern, alle Befehle/Edits absolut.

Eigentum: vorhandener Rust-Sitebaustein, soweit er nicht zu A-F1 gehört; bei fehlendem Dienst eigener Bin `rust/crates/deadlock-brain/src/bin/deadlock-brain-site.rs` mit ausschließlich zugehörigem neuem Modul sowie `rust/crates/deadlock-brain/Cargo.toml` für konkret nötige bestehende Dependencies, vorhandene Site-Unit-/Betriebsdoku unter `ops/`. Nicht `deadlock-brain/src/main.rs`, `dbrain-enrich`, Profilintegration, Binder oder Bots ändern. Bei zwingendem gemeinsamen Pfad vor Mutation präzise Übergabe an Paket A. Keine globale Formatierung. Rust, keine neuen Code-Kommentare.

Eigene Featurecommits/-push erlaubt. Kein Main-Push, produktiver Dienstwechsel, ConfigWriter, Migration, Restart oder Deploy durch diesen Worker. Paket A integriert und deployt Site und Profilpfad gemeinsam. Alte Artefakte nur lesen und vorhandenen Stand gezielt nutzen.

## Beweis und Stop

Aktuelle echte Site-/Rendererartefakte und bestehende Contracts als Grundlage. Private Originale, interne Pfade, Credentials und Roh-Wikitexte dürfen nie durch Dateiserver oder JSON-Route öffentlich werden. Pfadnormalisierung, Symlink-/Traversal-/Unknown-File-Grenzen am echten Server prüfen. Keine freie Verzeichnisauflistung oder neues Veröffentlichungsrecht. Bestehenden freigegebenen Profil-HTML-Bestand dienen, dessen Publikationsentscheidung nicht umgehen.

Compiler, gezieltes fmt, Clippy und passende vorhandene Suites; bestehende relevante Antworten/Routen am eigenen isolierten lokalen Port mit gebauten Artefakten vergleichen. Nicht auf 8087 binden oder laufenden Dienst stören. Kein globaler CARGO_TARGET_DIR, kein fremder Cache-Cleanup. Proberoute ohne Datengrundlage ist kein Profil-Fertigbeleg. Reine Erreichbarkeit und HTML-Inhalt getrennt nachweisen. Sichtbare Texte auf Deutsch, echte Umlaute, no-em-dashes/humanizer, bestehende Gestaltung erhalten.

Geprüften Feature-SHA mit `gate_hook.py --review` gegen frisches Main prüfen. Gate einziger Reviewer; BLOCK vollständig zurückgeben, frischer Fixer durch Paket A. Keine weiteren Reviewer/Fixer spawnen. Committrailer `Co-authored-by: GPT 6.1 Sol <modell@local>`.

## Übergabe

Versuch 1, Worker produziert Status; Wache nach 20 Minuten. Vollständige native Rückgabe: tatsächlicher Bestandsfund, URLs und Betriebsvertrag, unveränderte/abweichende Funktionen, Worktree/Branch/Basis/HEAD, Dateien, Prüfungen mit Zahlen, Gatewortlaut/Modell/Exit/Log, konkreter regulärer Dienstwechsel für Paket A. Root-Akte, TODO.md und Hauptberichte nicht editieren. Echte Produkt-/API-/Privatsphäreentscheidung als konkrete offene Frage melden; keine eigene Abweichung.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-a-site-20261006
