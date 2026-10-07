# H Prüfprotokoll

status: aktiv, 07.10.2026

## Rust-Nachweise

Alle abschließenden Cargo-Aufrufe mit `+1.97.1`, `--locked --offline --jobs 3`, freiem Hostslot und `--config 'build.build-dir="/home/nathanael/.cache/rust-build/{workspace-path-hash}"'`. Manifest: `/home/nathanael/.worktrees/brain-h-grafik-20261007/rust/Cargo.toml`, Paket: brain-maintenance.

| Prüfung | Ergebnis | Nachweis |
| --- | --- | --- |
| check, `--test hero_compare_render --example hero_compare_preview` | Exit 0 | check-pinned.log |
| clippy, gleiche Targets, `-- -D warnings`, nach Visualfix | Exit 0, keine Warnungen | clippy-final.log |
| test, `--test hero_compare_render -- --include-ignored --test-threads=1`, nach Visualfix | 12 passed, 0 failed, 0 ignored, 0 filtered | tests-final.log |
| bestehende HTML-Suite, `--lib html::tests`, gleiche Testflags | 5 passed, 0 failed, 0 ignored, 55 filtered | tests-html-pinned.log |
| bestehende Entityrenderer-Suite, `--lib entity_profile_render::tests`, gleiche Testflags | 17 passed, 0 failed, 0 ignored, 43 filtered | tests-profile-pinned.log |

TESTNACHWEIS[TW-1]: 34 passed, 0 ignored | Baseline: 0 rot

Die zwölf neuen Prüfungen verwenden den tatsächlichen Renderer via Include und prüfen synthetische Struktur-/Sicherheitsfälle. Vollständige 36-Punkt-Reihe bleibt in Tabelle und SVG erhalten. Keine Pflichtsuite gelöscht oder geschwächt. Nicht behauptet: komplette Workspace-/DB-Suite oder G-Rechenabnahme.

Alle drei eigenen Rust-Dateien abschließend gezielt formatgeprüft. Nach dem gebündelten Visualfix wurden Renderer und Vorschau erneut mit 1.97.1 kompiliert, Clippy mit -D warnings lief grün und zwölf Strukturtests bestanden erneut. Finaler Debugbuild des Beispiels: Exit 0, build-final.log. Erster Vorschauport 18767 war bereits belegt, kein fremder Listener angefasst. Der Vorschau-Listener wählt jetzt selbst einen freien Loopback-Port. Keine ENV-Konfiguration. Erste eigene Vorschau unter buutl0scu auf Port 40044 nach Erstpass per /close beendet, Exit 0; finale eigene Vorschau bb8zvwo7n auf Port 42401.

Die erste Compilerbaseline lief mit aktivem Cargo 1.99 und war grün. Ein Testversuch mit 1.99 wurde vor Ergebnis gestoppt; er zählt nicht. Sämtliche oben gemeldeten Abschlussprüfungen liefen ausdrücklich mit 1.97.1. Ein nach Portanpassung roter Formatcheck wurde durch gezieltes rustfmt behoben.

## Darstellung und Quellen

Palette `#b08b20,#995108`, Fläche `#111110`: alle fünf berechneten Checks PASS. CVD-Abstand 14,2, normaler Abstand 15,7; beide Markfarben mindestens 3:1. Legende, gestrichelte zweite Linie, unterschiedliche Punktformen, Direktlabels und Tabelle erhalten. Impeccable-Detektor auf Renderer: `[]`.

T3-Preview konkret nicht verfügbar: kein Automationhost in Environment e5040875-1ea7-43d3-9664-2a076d8443f5. Native unabhängige Sichtprüfung über vorhandenen lokalen Chrome gestartet. Desktop-/Mobil-Screenshots werden als synthetische Darstellungsvorschau dokumentiert, nicht als echter G-/Livebeweis.

Echte historische G-Probe vorhanden, aber nicht zur Veröffentlichung freigegeben und historischer G-Lauf rot. Keine echten Zahlen aus dieser Probe veröffentlicht. Details BESTAND-ANSCHLUSS.md. Öffentliche G-Datenabnahme und produktiver Schreib-/Lesepfad sind offener K-Anschluss.

## Ressourcen

HOSTPROBE.md und machines/v50671/RUST-BUILDS.md gelesen. Vorhandener sccache, keine Target-Kopie, kein Releasebuild. `cargo +1.97.1 metadata` mit identischem CLI-Override bestätigt `build_directory=/home/nathanael/.cache/rust-build/c1/b420b9232450b2`; finale Programme bleiben im normalen Worktree-`rust/target`. Cargo serialisiert eigene Aufrufe am identischen Workspace. Kein fremder Compiler oder Dienst gestoppt.
