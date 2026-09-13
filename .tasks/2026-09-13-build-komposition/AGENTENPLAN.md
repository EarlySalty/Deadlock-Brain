# Agentenplan: Build-Reasoner Abschluss

Stand: 13.09.2026. Integrationsbasis ist `feat/build-reasoner-integration`.

## Ziel

Mehrere Agenten arbeiten parallel am noch offenen fachlichen Ziel, ohne sich gegenseitig dieselben Dateien umzuschreiben. Der bekannte Warden-Fall 779996 bleibt Entwicklungsreferenz; mindestens 5/9 Referenz-Waffen sind Pflicht, aber keine unabhängige Meta-Prognose. Holdout-Ergebnisse werden nicht als Tuningziel an Bauagenten zurückgespielt.

## Gemeinsame Regeln

- Keine heldenspezifischen Gewichte, keine Referenz-Itemlisten im Algorithmus, kein Seed-Kopieren.
- Keine neuen LLMs, keine Python-Strecke, kein produktiver Upload und kein Dienstrestart in Agentenbranches.
- Mechanische Zahlen bleiben deterministisch in Rust; unbekannte Effekte werden sichtbar gelassen statt geraten.
- Jeder Agent arbeitet nur in seinem Worktree und auf seinem Branch.
- Gemeinsame Schnittstellen `types.rs` und `lib.rs` gehören dem Integrator. Braucht ein Agent dort eine Änderung, dokumentiert er die benötigte Schnittstelle in seinem Report statt parallel dieselbe Datei anzufassen.
- Jeder Agent liefert Tests plus einen kurzen Abschlussreport mit gemessenen Auswirkungen und offenen Grenzen.

## Agent A – Holdout / Messung

Branch: `feat/build-reasoner-holdout-final`
Worktree: `/home/nathanael/repos/wt/build-reasoner-holdout`

Dateibesitz:
- `rust/crates/dbrain-reasoner/examples/build_evaluation.rs`
- `rust/crates/dbrain-reasoner/examples/support/`
- ausschließlich eigene Nachweisdokumente unter `.tasks/2026-09-13-build-komposition/agents/holdout-*`

Auftrag:
- eingefrorene V2-Messbasis unverändert verwenden;
- Warden-Entwicklungsmessung, vollständigen Autoren-Holdout und Sensitivität reproduzieren;
- Quellrevision, Dateihash, Trainingsquellen und Exklusionsgruppe im Report prüfen;
- keine Produktionslogik anhand der Holdout-Ergebnisse ändern.

## Agent B – Kampf- und Interaktionsabdeckung

Branch: `feat/build-reasoner-interactions-audit`
Worktree: `/home/nathanael/repos/wt/build-reasoner-interactions-audit`

Dateibesitz:
- `rust/crates/dbrain-reasoner/src/combat.rs`
- `rust/crates/dbrain-reasoner/src/ability_interactions.rs`
- `rust/crates/dbrain-reasoner/src/item_interactions.rs`
- `rust/crates/dbrain-reasoner/src/data.rs`
- `rust/crates/dbrain-reasoner/src/hero.rs`
- `rust/crates/dbrain-reasoner/src/item.rs`
- passende `testdata/`-Fixtures
- ausschließlich eigene Nachweisdokumente unter `.tasks/2026-09-13-build-komposition/agents/interactions-*`

Auftrag:
- die in `FAEHIGKEITSABDECKUNG.md` und den Mehrheldenmessungen sichtbaren Null-/Unknown-Pfade prüfen;
- nur aus Rohsnapshot und belegter Spielsemantik typisierte Effekte ergänzen;
- Proc-, Heilungs-, Eigenkosten-, Cooldown-, Imbue- und Zielwechselereignisse mit Gegenproben absichern;
- keinen Planner, keine Autorenmetrik und keine Referenzlisten ändern.

## Agent C – Kaufplanung / Inventar

Branch: `feat/build-reasoner-planner-audit`
Worktree: `/home/nathanael/repos/wt/build-reasoner-planner-audit`

Dateibesitz:
- `rust/crates/dbrain-reasoner/src/planner.rs`
- `rust/crates/dbrain-reasoner/src/inventory.rs`
- `rust/crates/dbrain-reasoner/src/progression.rs`
- `rust/crates/dbrain-reasoner/src/composer.rs`
- ausschließlich eigene Nachweisdokumente unter `.tasks/2026-09-13-build-komposition/agents/planner-*`

Auftrag:
- Budget, Sparentscheidung, Upgradeverbrauch, Verkauf, Slots, Active-Limit, feste Imbue-Bindung und Skillfortschritt auf Invarianten prüfen;
- Kaufkurve muss genau aus den berechneten Übergängen entstehen und Publish-Reihenfolge erhalten;
- keine Kampfsemantik und keine Holdout-/Referenzdaten ändern.

## Agent D – unabhängiger Kritiker

Branch: `review/build-reasoner-final`
Worktree: `/home/nathanael/repos/wt/build-reasoner-critic`

Dateibesitz:
- nur `.tasks/2026-09-13-build-komposition/agents/critic-*`

Auftrag:
- Produktionscode lesend prüfen;
- Warden-Mehrheit nicht als alleinige Abnahme akzeptieren;
- Mehrhelden-Plausibilität, unbekannte Effekte, In-Sample-vs-Holdout-Trennung, Inventarübergänge und Messprovenienz kontrollieren;
- konkrete blockierende Befunde mit Pfad/Funktion/Test nennen, keine Produktionsdateien editieren.

## Integrator

Branch: `feat/build-reasoner-integration`
Worktree: `/home/nathanael/repos/wt/build-reasoner-integration`

Der Integrator hält die gemeinsame Fassade, `types.rs`, `lib.rs`, Mergeauflösung und das endgültige Gate. Vor einem Merge nach `main`: Reasoner-Tests, Clippy `-D warnings`, eingefrorene Vorher-/Nachhermessung, unabhängige Kritik und saubere Git-Provenienz. Kein Deploy, solange die fachliche Abnahme offen ist.
