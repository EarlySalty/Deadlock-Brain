# Direkte Build-Erstellung: CLI-Abnahme

Stand: 24. September 2026. Branch `feat/direct-build-publish-20260924`, Basis `dfefc8f2e0df623d838e0ec7c1f18d86492ae1b5`. Nutzerauftrag: Brain soll auf Anfrage Builds im Spiel erstellen.

## Umsetzung

Der vorhandene Alias `publish-build-query` ist jetzt ein eigener regulärer Befehl. Er nutzt den bestehenden Reasoner und die bestehende Veröffentlichungsprüfung, ohne Varianten, fehlende Patch-Abnahme oder fehlende Skillorder zu übergehen. Fehlende Freigabe liefert `BLOCKED` ohne Steam-Auftrag. Der bisherige explizite Review-Pfad bleibt getrennt und experimentell gekennzeichnet.

`DONE` und eine positive von Steam zurückgelieferte `hero_build_id` sind erforderlich, bevor die CLI einen Build als veröffentlicht ausgibt. Ein wartender Auftrag erhält keine Build-ID. Fehler und unbekannte Zustände werden nicht zu Erfolg umgedeutet.

## Tests

Toolchain: Cargo/Rust 1.98.0, `--locked -j 2`, isolierter Feature-Worktree.

- `cargo test -p deadlock-brain --bin deadlock-brain`: 48 bestanden, 0 Fehler, 0 ignoriert. Enthält die zwei neuen Tests zur getrennten Befehlsführung und bestätigten Veröffentlichung.
- `cargo test -p dbrain-reasoner --lib families::tests::regression_tests`: 23 bestanden, 0 Fehler, 0 ignoriert. Belegt unter anderem den regulären Veröffentlichungsstopp vor DB-Zugriff bei fehlender Familie oder Patch-Abnahme, Mindeststichprobe und Schutz vor stillschweigendem Verwerfen mehrerer Varianten.
- `cargo test -p dbrain-reasoner --lib publish::tests`: 1 bestanden, 0 Fehler. Payload-Roundtrip erhält Annotationen, Imbues, Verkaufsreihenfolge und Layout.
- Geänderte Veröffentlichungsfunktionen mit Rustfmt formatiert, ohne fremde Module umzuformatieren.

Insgesamt 72 ausgewählte Tests bestanden. Kein echter Build-Upload, keine Produktions-DSN und keine erfundene Build-ID als Funktionsbeweis.

## Integration und offene Abnahme

Discord-Verbraucher: `EarlySalty/Deadlock-Bots`, Branch `feat/brain-direct-build-publish-20260924`. Dieser benötigt die neue CLI vor seiner späteren Auslieferung. Die Konfiguration des bestehenden offenen Testmodus wird nicht verändert. Kein KI-Modellwechsel.

GitHub-Actions werden nach PR-Erstellung am aktuellen Head ausgewertet. Unabhängiger Fremdreview und Live-Funktionsprüfung sind nicht belegt. Der aktuelle PR-first-Testbetrieb untersagt Merge, Deploy, Neustart und Produktions-Uploads. Der lokale Teststand ist keine Live-Auslieferung.

MERGEPROTOKOLL[MS-1]: kein Merge: PR-first-Testbetrieb
