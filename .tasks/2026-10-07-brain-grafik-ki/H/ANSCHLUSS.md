# H Anschlussvertrag vor Bau

status: aktiv, 07.10.2026

## Grenze

`render_hero_compare` nimmt eine typisierte, rein darstellende Projektion des bestätigten G-Ergebnisses. Die Projektion erhält Ergebnis-ID, gebundene Version, Szenariobedingungen, Quellen und genau zwei gleichartige Boonreihen. H errechnet ausschließlich Bildschirmkoordinaten, keine Spielwerte, Zuwächse oder Überholpunkte.

Ausgabe: feste vollständige HTML-Detailseite und eigenständiges SVG aus derselben validierten Eingabe. Keine Dateioperationen, Netzaufrufe, Modellinhalte als Markup, Zieladressen oder privaten Profile.

## K-Verantwortung

1. Exportzeile in `brain-maintenance/src/lib.rs` und Import im zuständigen Consumer ergänzen.
2. G-Ergebnis nach bestätigtem Vertrags-SHA auf die Darstellungsstruktur abbilden. Fehlende/unsichere Quellen, private Inhalte oder uneinheitliche Versionen vor Rendering ablehnen; nicht durch Legacywerte ersetzen.
3. Rechte und Veröffentlichung serverseitig prüfen, auch bei Abruf, Cache und Widerruf. Eine Eingabemarkierung im Renderer ersetzt diese Prüfung nicht.
4. IDs, Zielpfade, erlaubte Link-/Anhangsausgabe, Speicherung, CSP und Auslieferung im vorhandenen Rust-Sitepfad halten. Keine neue Domain oder Dauerdienst.

## Gelieferte Schnittstelle

```rust
pub fn render_hero_compare(input: &HeroCompareInput) -> anyhow::Result<RenderedHeroCompare>
pub struct RenderedHeroCompare { pub html: String, pub svg: String }
```

K ergänzt in `rust/crates/brain-maintenance/src/lib.rs` genau `pub mod hero_compare_render;`. Diese Registrierungszeile ist absichtlich nicht Teil des H-Diffs. Der separate Integrationstest inkludiert dieselbe Quelldatei direkt, damit sie trotzdem kompiliert und geprüft wird.

Darstellungsfelder:

- `CompareBinding`: serverseitige Ergebnis-ID, `VersionBinding { snapshot_id, client_version }` und ausdrückliche Bedingungen. Beide Helden erhalten exakt diese Bindung. K muss die Rechenregelrevision (`PinnedGameContext.mechanic_revision`), G-Regeln, sämtliche angewandten Szenariowerte und Defaults in den gebundenen Bedingungen ausdrücklich anzeigen; keine erfundene aktuelle Patchzuordnung.
- `HeroCompareInput`: Bindung, Veröffentlichungsstatus, eine `CompareMetric`, gültige angefragte Boonstände, Quellen und genau zwei `HeroCompareSeries`.
- Reihe: Hero-ID/Name, Freigabe, identische Bindung/Kennzahl, Quellen-IDs, sortierte `BoonValue { boon, value }`. `DisplayValue::Missing|Unquantified` wird abgewiesen, nicht als null gezeichnet.
- Quelle: ID, freigegebener Textbeleg, identischer Datenstand und Veröffentlichungsstatus. Keine externen Zieladressen.

Kennzahl-Enum: `BaseDps|MagazineDamage|Hp`. Ausgabe gibt exakte skalare G-Werte in Tabelle und SVG-Beschreibung wieder. Die Skala und Diagrammkoordinaten sind reine Darstellung. Kein Errechnen von Wachstumsraten, Schnittpunkten oder Spielwerten.

Auf aktuellem origin/main fehlt noch der neue G-Werkzeugvertrag. G-WIP wird nicht kopiert oder als atomarer Stand angenommen. Der konkrete G-Anschlussbeleg wird nach dem Workflow im BESTAND-ANSCHLUSS.md ergänzt.
