# K: Rust-Siteport, erste native Prüfung

Prüfung durch nativen Sol-high-Worker, keine Produktänderung. Bestehender Port unter /home/nathanael/.worktrees/brain-a-site-20261006, geprüfter Gitstand cac8763525c9ba930a61dae2e43c09e13bb0fda0. Binary und vier `src/bin/site/`-Module bytegleich mit diesem Stand. Auf K-Anfangs-main f6f5cef6 fehlt der Port.

## Befund

Alte A-Akten führen den Port als pausierten A-F3b-Bereich. Das ist kein neuer Release-Halt. Der aktuelle H/K-Auftrag weist gemeinsame Routen und Integration ausdrücklich K zu; vor einer Übernahme sind trotzdem die konkreten Dateien und übrigen Produktabhängigkeiten zu beachten. Kein fremder WIP wird kopiert, nur bestätigter Gitstand selektiv verwendet.

Der Port braucht zusätzliche Paketabhängigkeiten und Cargo.lock-Integration. Der bisherige Router setzt außerdem bestehende Kommentarmigration und isolierte PostgreSQL-Rolle brain_site voraus. Diese Teile gehören nicht zur Diagrammberechnung. Bereits dokumentierte 44 Prüfungen sind fremde historische Belege, keine eigene Abnahme.

## Wiederverwendbarer Vertrag

Explizite Dateiliste, begrenzte Entitäts-HTML-Pfade, maximal 16 MiB je Datei, keine Symlinkverfolgung, nosniff, CSP und no-cache. HTML wird unverändert geliefert, keine Inhaltsbereinigung und keine neue Profilpublikation. H-Artefakte sind bislang nicht in der Dateiliste. Ops-Referenz bindet Loopback 8087 und maintenance-current. Kein aktueller Umschalt- oder Funktionsbeweis.

## Grenze

Keine zweite Site bauen. Selektive Übernahme innerhalb K-Eigentum prüfen; nicht ungefragt Kommentarfeatures und fremde Datenmigrationen als Voraussetzung erweitern. G-Ergebnis-/Quellenfreigabe und H-Vertrag fehlen weiterhin für die öffentliche Grafikstrecke. Kein Deploy, keine Änderungen an alten A-Worktrees.

BESTAND[BS-1]: teilweise | Fundort: /home/nathanael/.worktrees/brain-a-site-20261006/rust/crates/deadlock-brain/src/bin/deadlock-brain-site.rs:1 | Anknüpfung: vorhandener Rust-Siteport und feste Assetauslieferung
