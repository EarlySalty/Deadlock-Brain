# H Baunachweis

status: aktiv, 07.10.2026

Nativer Bauagent a7be4647ca08fcf52 hat `hero_compare_render.rs` geliefert. Geerbtes Sol 6.1, Effort high, 30 Toolaufrufe, Lauf circa zehn Minuten. Abschlussmeldung im nativen Tasknachweis. Kein Modellwechsel oder xhigh.

Signatur: `render_hero_compare(&HeroCompareInput) -> anyhow::Result<RenderedHeroCompare>`. Ausgabe `html` und `svg`. Eingabe ist ausschließlich Darstellungsprojektion, keine eigenständige G-Rechnung. Typisierte Kennzahlen Grund-DPS, Magazinschaden und HP. Beide Ansichten enthalten dieselben Werte, Ergebnisbindung, Clientversion, Snapshot, Bedingungen und Quellen.

Prüfungen des Bauagenten: gezieltes rustfmt und berechnete Palettenvalidierung. Noch kein Cargo-/Browser-/Echtdatenbeweis. Der Agent hat keine BAU.md geschrieben; diese Zusammenfassung dokumentiert dessen tatsächliche Abschlussmeldung, keine zusätzliche Prüfung.

Kein eigenes Punktzahllimit: Der Renderer übernimmt alle gelieferten, nicht leeren, eindeutig aufsteigend sortierten Boonstände vollständig, ohne Sampling. K verantwortet die serverseitige Domain- und Budgetvalidierung von G sowie die öffentliche Freigabe. Endliche, nicht negative Werte, übereinstimmende Achsen, Quellen-, Freigabe- und Bindungsprüfungen bleiben unverändert. Diese Eingabemarkierungen ersetzen keine Rechte-/Abruf-/Widerrufsprüfung. Nur die Punktzahlbedingung wurde entfernt; kein neuer Cargo-, Gate- oder Laufzeitnachweis.

Bestehende Renderer haben private Maskierungshelfer; eine lokale reine Escape-Funktion vermeidet Änderung fremder Registrierungen. Kein Publisher, keine Datei-/Netzoperationen, keine Manifeste, G-, K- oder Botdateien verändert.
