status: erledigt
Datum: 2026-09-29

# Paket V: Bestandsvorcheck

**Stand:** Read-only-Durchgang am 2026-09-29. Kein Test oder Harness ausgeführt, keine API-Produktdaten geladen, keine Secrets gelesen, kein Code/Git/Deploy ausgeführt.

**SHA:** `AUFTRAG.md:21` nennt `305df2d36ec7b5d0513d6c0769051b41538d6a1b` als Brain-Basis. Wegen des Git-Verbots nicht lokal verifiziert. Der lokale Graphify-Graph fehlt; Kandidaten kamen aus dem globalen Graph, Belege wurden anschließend direkt gelesen.

## API-Repository und Contract

- **Ermitteltes Upstream-Repository:** [deadlock-api/deadlock-api](https://github.com/deadlock-api/deadlock-api). [Projekt-Dokumentation](https://api.deadlock-api.com/docs) und GitHub-Repository waren erreichbar. Die Dokumentation führt als Contract-Endpunkt [https://api.deadlock-api.com/openapi.json](https://api.deadlock-api.com/openapi.json); das OpenAPI-Dokument selbst wurde nicht abgerufen.
- Der Brain-Client setzt `BASE_URL` auf `https://api.deadlock-api.com` in `/home/nathanael/.worktrees/brain-pre-g5-finalize-20260929/rust/crates/dbrain-sources/src/deadlock_api.rs:18-23`; Match-Metadaten gehen an `/v1/matches/metadata` in `:180`. Das ist Brain-Consumer-Code im erhaltenen Worktree, nicht das Upstream-API-Repository.
- **Lokaler Upstream-Checkout:** keiner unter `/home/nathanael/repos` oder `/home/nathanael/.worktrees` gefunden. Keine Match- oder Spieler-Daten geladen.

## Vorhandene Tests und Harnesses

- **Fact-Relevance: bereits abgedeckt, nicht neu bauen.** `rust/crates/brain-kernel/src/fact_relevance.rs:237-300` testet richtige/falsche Entity und Felder, Alias, Zahl aus dem konkreten Feld, Mehrdeutigkeit, konkurrierende Facts und spezifisches gegen generisches Feld. `:301-311` deckt Patch, Mode und Provenienz ab.
- **Assets Starting Stats: bestehende, aber nicht vollständige Vollständigkeitsevidenz.** `rust/crates/brain-feeds/src/deadlock_assets.rs:19-26` verlangt fünf numerische Felder. `:272-313` prüft konkret `max_health` samt Feldbindung, Pointer, Hash und Provenienz, und insgesamt zwölf Records. Die vorhandenen Ausschnitte belegen nicht, dass alle fünf Pflichtfelder pro Assets-Core-Record vollständig getestet werden. `:342-` enthält den Quarantänefall für fehlende oder nichtnumerische Werte. Fixture: `rust/crates/brain-feeds/tests/fixtures/heroes.json`.
- **Wiki-Runtime:** `rust/crates/dbrain-sources/tests/wiki_runtime.rs:99-220` enthält IR-/Template-/Raw-Policy-Regressionen; `:230-` die Scratch-DB- und ACL-Fälle; `:419-` den idempotenten normalen Store-Stage-Test.
- **Geeignete lokale Harnesses, nicht ausgeführt:** `scripts/test_brain_serve.sh:8-29` erzeugt einen frischen Temp-Postgres-Cluster nur über Unix-Socket und startet den echten Brain-Serve-Prozess mit lokalem Providerstub. `scripts/test_wiki_runtime.sh:8-40` setzt ein frisches Temp-Cluster auf, entfernt DB- und App-Credential-Variablen und nutzt Fixtures ohne Live-Wiki-Capture. Beide Skripte verwenden lokales `trust`-Auth; bei Wiki ist das auf den geschützten Temp-Socket und den eigens gestarteten Cluster begrenzt.
- **Nicht verwenden:** `scripts/run_isolated_load.sh:7-25` liest System-Postgres-Metriken und startet `run_isolated_pilot.sh`. Dieses zielt auf `/run/deadlock-brain-postgresql` und Port 5446 (`scripts/run_isolated_pilot.sh:7-17`), bezieht Passwörter via Infisical und exportiert sie in Testprozess-Umgebungen (`:23-27`) und startet den System-Postgres-Dienst neu (`:41`). Kein secret-sicherer Weg für den angeforderten lokalen Lastnachweis.
- `scripts/test_brain_core_postgres.sh:7-22` nutzt zwar einen lokalen Unix-Socket, aber ein festes `.core-test-pg` im Worktree mit `trust`-Auth. Für frische, isolierte E2E-Läufe sind die beiden Temp-Cluster-Runner oben klarer abgegrenzt.

## Freigaben und Replay

- **Wiki Capture/Lizenz/Raw:** keine belegte aktuelle Betreiberfreigabe gefunden. `architecture/migration/handoffs/C5_WIKI_RUNTIME.md:66-72` verlangt separate Zustimmung und Quellenlizenz; der echte Capture startet mit leerem Mapping und ohne Fact-Freigabe. Also offen.
- **Provider/Modell/Egress/Budget:** keine belegte Freigabe. `architecture/migration/handoffs/07-provider-jev.md:34,50-54` sagt, dass Live-Shadow nicht ausgeführt wurde, keine Freigabe vorliegt und G0/G1 offen sind. Also offen.
- **Echter Replay:** kein `.dem` in den beiden Brain-Worktrees gefunden. `architecture/migration/handoffs/14-replays-observations.md:15` nennt keinen freigegebenen echten Replay-Corpus. Keine Datei geladen oder heruntergeladen. Also offen.

## Empfehlung für Paket C

- `fact_relevance.rs` unverändert als bestehende Evidenz verwenden, keine parallelen Fact-Relevance-Fixes.
- Falls nötig, ausschließlich den Testblock in `brain-feeds/src/deadlock_assets.rs` erweitern, um alle fünf Pflichtfelder nachzuweisen; keine Änderungen an Adapterlogik.
- Wiki-Regressionen auf `dbrain-sources/tests/wiki_runtime.rs` und den bestehenden Runner `scripts/test_wiki_runtime.sh` begrenzen. E2E-Nachweise auf `brain-serve/tests/process_e2e.rs` und `scripts/test_brain_serve.sh` begrenzen.
- Für Lasttests einen separaten frischen Temp-Cluster-Runner in einer neuen Datei anlegen. `run_isolated_load.sh` und `run_isolated_pilot.sh` unangetastet lassen und nicht starten.
- Diese Grenzen überschneiden sich nicht mit den in `PAKETE.md:11,13` zugewiesenen Adapter-/Wiring-Dateien von A. Insbesondere `brain-feeds/src/lib.rs`, `dbrain-sources/Cargo.toml`, `dbrain-sources/src/lib.rs`, neue Adapterdateien und `Cargo.lock` C nicht zuweisen.
