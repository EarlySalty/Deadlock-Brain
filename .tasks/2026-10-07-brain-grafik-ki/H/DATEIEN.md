# H Dateieigentum

status: aktiv, 07.10.2026

- `rust/crates/brain-maintenance/src/hero_compare_render.rs`: H-Bauagent, Reihenfixer und Visualfixer nacheinander. Darstellungsdaten, Eingabeprüfung und deterministische SVG-/HTML-Ausgabe. Kein Rechner.
- `rust/crates/brain-maintenance/tests/hero_compare_render.rs`: H-Prüfagent, danach Vorschau-Fixer, zuletzt Teil-Orchestrator für Vollreihen-Strukturtest. Nacheinander, keine parallelen Edits. Isolierter Include-Test ohne Manifeständerung.
- `rust/crates/brain-maintenance/examples/hero_compare_preview.rs`: H-Vorschau-Fixer, danach Visualfixer. Kurzlebiges Rust-Beispiel für synthetische Sichtprüfung, keine produktive Route oder ENV-Konfiguration.
- `H/SICHT.md` und `H/sicht/`: unabhängiger nativer Sichtagent. PNGs sind synthetische Layoutnachweise.
- `H/VISUALFIX.md`: abschließender nativer Visualfixer.
- `.tasks/2026-10-07-brain-grafik-ki/H/`: Teil-Orchestrator, außer ausdrücklich benannten Agentenberichten.

Bestehende `entity_profile_render.rs` und `html.rs` werden nur gelesen. Bestehender Site-Port wird nur gelesen. `lib.rs`, Cargo-Manifeste, Contracts, Reasoner, Kernel, Provider, API, Bot und Site-Routen besitzen K oder G und bleiben unverändert.

Verbraucher: K-Adapter zwischen freigegebenem G-Ergebnis und Renderer; vorhandene Rust-Site; Discord-Anhang und Twitch-Link über K. Ausgaben sind Bytes, keine selbst gewählten Pfade oder URLs.
