# K: Rust-Siteport, erste native Prüfung

Prüfung durch nativen Sol-high-Worker, keine Produktänderung. Bestehender Port unter /home/nathanael/.worktrees/brain-a-site-20261006, geprüfter Gitstand cac8763525c9ba930a61dae2e43c09e13bb0fda0. Binary und vier `src/bin/site/`-Module bytegleich mit diesem Stand. Auf K-Anfangs-main f6f5cef6 fehlt der Port.

## Befund

Alte A-Akten führen den Port als pausierten A-F3b-Bereich. Das ist kein neuer Release-Halt. Der aktuelle H/K-Auftrag weist gemeinsame Routen und Integration ausdrücklich K zu; vor einer Übernahme sind trotzdem die konkreten Dateien und übrigen Produktabhängigkeiten zu beachten. Kein fremder WIP wird kopiert, nur bestätigter Gitstand selektiv verwendet.

Der Port braucht zusätzliche Paketabhängigkeiten und Cargo.lock-Integration. Der bisherige Router setzt außerdem bestehende Kommentarmigration und isolierte PostgreSQL-Rolle brain_site voraus. Diese Teile gehören nicht zur Diagrammberechnung. Bereits dokumentierte 44 Prüfungen sind fremde historische Belege, keine eigene Abnahme.

## Wiederverwendbarer Vertrag

Explizite Dateiliste, begrenzte Entitäts-HTML-Pfade, maximal 16 MiB je Datei, keine Symlinkverfolgung, nosniff, CSP und no-cache. HTML wird unverändert geliefert, keine Inhaltsbereinigung und keine neue Profilpublikation. H-Artefakte sind bislang nicht in der Dateiliste. Ops-Referenz bindet Loopback 8087 und maintenance-current. Kein aktueller Umschalt- oder Funktionsbeweis.

## Grenze

Keine zweite Site bauen. Selektive Übernahme innerhalb K-Eigentum prüfen; nicht ungefragt Kommentarfeatures und fremde Datenmigrationen als Voraussetzung erweitern. G-Ergebnis-/Quellenfreigabe und H-Vertrag fehlen weiterhin für die öffentliche Grafikstrecke. Kein Deploy, keine Änderungen an alten A-Worktrees.

## Übernahme und eigene Verifikation

Nativer Worker a95a021e918f5fd82 hat ausschließlich den bestätigten Site-Gitstand übernommen. K ergänzt dessen Crate-Abhängigkeiten und Lockdatei; nur die Testfixture wurde auf isolierte Postgres-Prüfung angepasst. Keine Migration, Produktionsrolle oder fremde Arbeitskopie geändert.

Build, Format und Site-Clippy mit `--no-deps -- -D warnings` bestanden. Vollständiges Abhängigkeitsclippy: vier Befunde in unverändertem dbrain-enrich, unveränderte separate Baseline ebenfalls vier. Kein grünes Gesamtclippy behauptet.

Eigener K-Nachlauf b4muxk6af: Cargo 1.99, Slot 2, `--locked --jobs 3 -p deadlock-brain --bin deadlock-brain-site -- --include-ignored --nocapture --test-threads=1`, bestehendes Target `/home/nathanael/Documents/Deadlock-Brain/rust/target`. Log `/tmp/k-site-tests-20261007.log`, Exit 0. Drei echte isolierte HTTP-/Postgres-Fälle: elf Korpusdateien bytegleich, drei Entitätspfade, Asset-/Symlinkgrenzen und dauerhafte Kommentare über Neustart mit Rollenrechten. Kein Produktions- oder echter G-Grafikbeweis.

TESTNACHWEIS[TW-1]: 3 passed, 0 ignored | Baseline: 0 rot

BESTAND[BS-1]: teilweise | Fundort: /home/nathanael/.worktrees/brain-a-site-20261006/rust/crates/deadlock-brain/src/bin/deadlock-brain-site.rs:1 | Anknüpfung: vorhandener Rust-Siteport und feste Assetauslieferung
