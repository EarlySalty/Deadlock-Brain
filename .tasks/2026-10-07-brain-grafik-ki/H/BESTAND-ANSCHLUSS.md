# H G-Bestand und Anschluss

status: aktiv, 07.10.2026

Belegt durch tatsächlichen nativen Workflow wf_97de2070-ec3: ein Agent abgeschlossen, kein Fehler/Skip, 62 Toolaufrufe, circa 18 Minuten, geerbtes Sol 6.1 mit high. Der Workflow konnte keine Berichtsdatei schreiben; diese Datei destilliert dessen Rückgabe.

## Atomare Grenze

H origin/main: `f6f5cef65f1f946113f0b8216c6475f6d38ec928`. G beobachteter HEAD: `5a442391227420cdc7f2d13ba3b072299fd6e9ca`, Requestcheckpoint `3d6890c0`. G-Ergebnistypen und calculation.rs sind nicht committetes WIP; calculation.rs änderte sich während der Prüfung. Nicht kopiert und nicht als atomarer Stand behandelt.

`brain-contracts/src/tools.rs`: HeroCompareRequest mit hero_ids/metrics/scenario/ranking_population/boon_range/analytics. ToolResult transportiert generisches JSON, evidence_ids und is_error. Kein integrierter typisierter HeroCompare-Antwortpayload. PinnedGameContext enthält client_version/language/mechanic_revision.

WIP unter `dbrain-reasoner/src/types.rs`: HeroCurveComparison mit client_version/left/right/metric/points/overtakes/tied_boons/status. HeroGrowth mit client_version/hero_id/scenario/range/base/points/per_boon/early_to_late. MeasuredValue unterscheidet Known(value/unit/sources/rule), Unknown und NotApplicable. Kennzahlen weapon_dps, damage_per_magazine, health. `base` ist Boon 0, auch wenn die angefragte Reihe später beginnt. Nur `points` zeichnen, kein erfundenes Voranstellen von `base`.

## Echter Datennachweis, keine Veröffentlichungsfreigabe

G-Probe `rust/crates/dbrain-reasoner/testdata/calculation/sheet-6759.json` und historisches `G/pruefungen/g-m-0645/calculation-tests.log:46`: aufgezeichneter Haze/Warden-HP-Vergleich, Clientversion 6759, Boons 0..35, Gesamt-Spirit 38, ohne Items, no_overtake. Drei aufgezeichnete Stände: Haze 730/1390/1885 und Warden 805/2005/2905 bei 0/20/35.

Die Probe verneint aktive Produktionsversion und bestätigten Balance-Patch. Veröffentlichungsfreigabe fehlt. Historischer Testlauf war 32 passed, 1 failed und damit rot. Diese Zahlen werden nicht als freigegebene Renderfixture oder Livebeweis benutzt. Vollständige 0..35-Reihe hat 36 Punkte; ein unabhängiges Rendererlimit von 32 passt nicht und wird entfernt, ohne Werte nachzubauen.

## Wiederverwendung und K-Anschluss

Private Escapehelfer im bestehenden Entityrenderer sind ohne fremden Produktedit nicht nutzbar. Feste Abschnitte, Tabellen und restriktive Freigabe werden übernommen. html.rs bietet validate_writing/validate_html, aber dessen Mutation-Validator erlaubt frische Styles/gestrichelte Linien nicht pauschal. Keine pauschale Abschaltung oder Änderung der Schutzfilter.

Rust-Site-Port liegt tatsächlich unter `/home/nathanael/.worktrees/brain-a-site-20261006/rust/crates/deadlock-brain/src/bin/site/`, sauberer HEAD `cac8763525c9ba930a61dae2e43c09e13bb0fda0`. Allowlist akzeptiert Entity-HTML, noch kein Vergleichs-HTML/SVG. Statisches Serving ersetzt keine rechtegebundene Artefaktroute. K erweitert erlaubte feste Artefakte, prüft Rechte/Version/Quelle/Widerruf auch bei Abruf und Cache und koppelt strukturierte Discord-/Twitch-Ausgabe.

ModelSource ist kein Rechtebeweis. K muss sämtliche beitragenden Quellen an vorhandenen Records mit Revision/Hash, öffentlicher Sicht, Veröffentlichungsrecht, geprüfter Lizenz und ohne private Scopes prüfen. Assets-API nicht als GameFile umlabeln.

Zentrale Buildablage und sccache wurden bestätigt und in H-Prüfaufrufen ausdrücklich gesetzt. Kein G-/Site-/API-/Bot-Edit.
