status: aktiv | 2026-09-29

# C1: Prüfung und Betriebsgrenze

Der Brain-Import und `dl-bot` laufen auf demselben Diensthost. Die lokale Dienstliste zeigte `deadlock-brain-patchnotes-sync.service`; `ss -ltnp` zeigte `dl-bot` auf `127.0.0.1:8901`. Es ist kein öffentlicher Proxy erforderlich. Die Ledger-Routen selbst wurden nicht produktiv aufgerufen.

Die beiden Rust-Einstiegspunkte sind `rust/crates/deadlock-brain/src/pg_patchnotes.rs` und `rust/crates/deadlock-brain/src/pg_steam_news.rs`. Beide verwenden die Standardklasse und dasselbe Postgres-Journal. Der interne Schlüssel wird aus der vorhandenen Infisical-Umgebung gelesen; bei fehlendem Schlüssel oder nicht verfügbarem Dienst erfolgt kein Steam-Aufruf.

Ein gespeicherter Reservierungseintrag ohne begonnenen Steam-Aufruf darf nach einem Neustart als Transportfehler gemeldet werden. Vor jeder Reservierung wird ein Sperrvermerk gespeichert; der Steam-Aufruf beginnt erst nach dauerhaftem Journaleintrag und bestätigter Entfernung dieses Vermerks. Bleibt der Vermerk nach einem Abbruch erhalten, wurde noch kein Steam-Aufruf gestartet: Er kann beim nächsten Lauf entfernt werden, und bekannte Reservierungs-IDs ohne gestarteten Versand werden vor dem nächsten Steam-Aufruf als ungenutzt gemeldet. Ein verlorener, nicht im Journal gespeicherter Reservierungseintrag verbraucht weiterhin Kapazität im zentralen Ledger, aber keine Steam-Anfrage. Sobald der Steam-Versand begonnen hat, blockiert eine fehlende Antwort im Journal weitere Steam-Aufrufe, auch bei vorhandenem Cache. Ein unbekannter HTTP-Status darf nicht als Transportfehler gemeldet werden: Vor einer manuellen Freigabe müssen Ledger und verfügbare Upstream-Protokolle für die betroffene Reservierungs-ID abgeglichen werden. Lässt sich insbesondere ein mögliches HTTP 429 samt `Retry-After` nicht klären, bleibt der Import gesperrt. Journalzeilen nicht ohne bestätigte Beobachtung entfernen.

Geprüft: 19 Ledger-Tests einschließlich echter isolierter PostgreSQL-Instanz, Aktualisierung einer alten Journaltabelle und Neustartwiederholung mit `--include-ignored`; vollständige Rust-Workspace-Suite 543 bestanden, 62 ignoriert, 0 fehlgeschlagen; `cargo clippy --workspace --all-targets -- -D warnings` erfolgreich. Der globale Format-Check scheitert bereits an unveränderten Altdateien wie `dbrain-enrich/src/lib.rs`. Die neuen und gezielt geänderten Ledger-Dateien wurden geprüft, ohne fremde Format-Hunks einzubringen.

Keine produktive Migration, kein Merge, kein Neustart und kein Live-Aufruf in diesem Paket. Vor Aktivierung sind die Ledger-Implementierung, alle Verbraucher und der gemeinsame Cutover unabhängig zu prüfen.

## Fortsetzungsprüfung am 2026-10-01

`flush_pending` meldet nach einem Neustart zunächst bekannte Reservierungen ohne gestarteten Steam-Aufruf und erhält zugleich eine Sperre für jede Anfrage mit unbekanntem Ausgang. Die Regression `releases_unused_reservations_before_blocking_uncertain_request` prüft beides gemeinsam: die ungenutzte Reservierung wird an das Ledger gemeldet und entfernt, während die unbekannte Anfrage bestehen bleibt und weitere Steam-Aufrufe blockiert.

Gezielter Lauf: `PATH=/home/nathanael/.cargo/bin:$PATH cargo test -p deadlock-brain steam_web_api::tests -- --nocapture`, 19 bestanden, 0 fehlgeschlagen, 1 ignoriert, 52 gefiltert. Die echte PostgreSQL-Integration blieb ignoriert, da sie `DEADLOCK_BRAIN_SCRATCH_DSN` benötigt. `rustfmt --check rust/crates/deadlock-brain/src/steam_web_api.rs` war erfolgreich. `cargo clippy -p deadlock-brain --all-targets -- -D warnings` endete mit Code 101 an vier `map_or_identity`-Diagnosen in der unveränderten Datei `crates/dbrain-enrich/src/lib.rs`.

Die beiden Brain-Einstiegspfade `pg_patchnotes.rs` und `pg_steam_news.rs` wurden gegen das gemeinsame Journal und die lokalen Reserve-/Observe-Datentypen geprüft. Eine Suche nach den Route- und Fehlertypnamen fand den zugehörigen Produktions-Handler in den zugänglichen Deadlock-Bots-Quellbäumen nicht. Die vorhandene Diensttopologie-Notiz belegt einen Listener auf `127.0.0.1:8901`, aber keinen aktuellen Live-Aufruf dieses Vertrags. Diese Fremddienst-Abnahme bleibt offen.
