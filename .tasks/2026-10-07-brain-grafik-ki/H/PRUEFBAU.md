# Aufrufvorlage für die zentralen H-Prüfwerkzeuge

status: aktiv, 07.10.2026

Diese Vorlage ist Eingabe für den zentralen Prüflauf, kein Prüfergebnis. Alle Fixturewerte sind synthetisch. Die simulierten Veröffentlichungsmarkierungen gelten ausschließlich innerhalb der Strukturtests und der Vorschau. Sie bestätigen keine Freigabe echter Quellen.

## Formatprüfung nur der eigenen Dateien

```bash
/home/nathanael/.cargo/bin/rustfmt +1.97.1 --edition 2021 --config skip_children=true --check /home/nathanael/.worktrees/brain-h-grafik-20261007/rust/crates/brain-maintenance/tests/hero_compare_render.rs /home/nathanael/.worktrees/brain-h-grafik-20261007/rust/crates/brain-maintenance/examples/hero_compare_preview.rs
```

## Zentraler Compiler-, Clippy- und Testaufruf

Keine parallelen Builds starten. Die folgenden Aufrufe gehören dem Orchestrator. Das zentrale Bauverzeichnis wird ausschließlich über Cargo angegeben, ohne Umgebungsvariablen oder neue Konfigurationsdatei.

```bash
/home/nathanael/.cargo/bin/cargo +1.97.1 check --manifest-path /home/nathanael/.worktrees/brain-h-grafik-20261007/rust/Cargo.toml --config 'build.build-dir="/home/nathanael/.cache/rust-build/{workspace-path-hash}"' -p brain-maintenance --locked --offline --jobs 3 --test hero_compare_render --example hero_compare_preview
/home/nathanael/.cargo/bin/cargo +1.97.1 clippy --manifest-path /home/nathanael/.worktrees/brain-h-grafik-20261007/rust/Cargo.toml --config 'build.build-dir="/home/nathanael/.cache/rust-build/{workspace-path-hash}"' -p brain-maintenance --locked --offline --jobs 3 --test hero_compare_render --example hero_compare_preview -- -D warnings
/home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-h-grafik-20261007/rust/Cargo.toml --config 'build.build-dir="/home/nathanael/.cache/rust-build/{workspace-path-hash}"' -p brain-maintenance --locked --offline --jobs 3 --test hero_compare_render -- --include-ignored --test-threads=1
```

Die zwölf Strukturtests enthalten keinen Vorschautest, keine Umgebungsabfrage und keine zeitgesteuerte Vorschau. Ein erfolgreicher Testlauf bestätigt nur Struktur, Sicherheitsgrenzen und Darstellung des echten Renderers mit synthetischen Eingaben, weder G-Rechenwerte noch den produktiven Auslieferungspfad.

## Separater lokaler Vorschauaufruf

Das automatisch von Cargo erkannte Beispiel nutzt das echte Renderermodul über `#[path = "../src/hero_compare_render.rs"]`. Es benötigt keine Manifeständerung. Sein öffentliches Modul vermeidet ungenutzte private Varianten, ohne Warnungen zu unterdrücken.

```bash
/home/nathanael/.cargo/bin/cargo +1.97.1 run --manifest-path /home/nathanael/.worktrees/brain-h-grafik-20261007/rust/Cargo.toml --config 'build.build-dir="/home/nathanael/.cache/rust-build/{workspace-path-hash}"' -p brain-maintenance --locked --offline --jobs 3 --example hero_compare_preview
```

Das reine Rust-Beispiel bindet ausschließlich Loopback und lässt das Betriebssystem einen freien Port vergeben. Die tatsächliche URL steht auf stderr. Es liefert nur GET `/` und GET `/compare.svg`. GET `/close` beendet den eigenen Prozess, alternativ Strg+C. Nach der Vorschau den Prozess ausdrücklich beenden. Fehler einzelner Browserverbindungen werden auf stderr ausgegeben und beenden nicht die gesamte Vorschau. Keine produktive Route, Speicherung oder echte Datenfreigabe. Seitentext, Ergebnis-ID, Datenstand und Bedingungen kennzeichnen die Synthetik ausdrücklich.

Für einen separaten Browser-Screenshot die HTML-Seite bei 390 Pixeln Breite öffnen. Den Screenshot ausdrücklich als synthetische Darstellungsvorschau kennzeichnen. Mobile Nutzbarkeit, tatsächliches Scrollen und Lesbarkeit sind zusätzlich im Browser zu prüfen; die automatisierten Strukturprüfungen bestätigen nur die vorhandene Semantik.

## Offener echter G-Beweis

Eine echte freigegebene G-Probe und deren Ergebnisbindung bleiben offen. Der synthetische Screenshot ersetzt weder diesen Beweis noch einen Livebeweis des produktiven Auslieferungspfads. Keine erfundenen Werte als echte G-Ergebnisse ausgeben.
