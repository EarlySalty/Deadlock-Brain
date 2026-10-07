# K: Eigene Artefakthülle, Speicherung und Rechte

## Ziel und Vertrag

Baue den freigegebenen K-eigenen Vergleichsartefaktpfad im bestehenden Brain. G liefert Berechnung, strukturierte Reihen, Szenario, Version und belegte Werkzeug-/Quellenabhängigkeiten. K baut Artefakt-ID, HTML/SVG-Bindung und Veröffentlichungsquittung, Postgres-Anbindung, aktuelle Rechteprüfung, Speicherung und Auslieferung. Dafür keine neue Produktfreigabe verlangen und keine G-Artefaktquittung abwarten. Ohne bestätigten echten G-Eingang bleiben Veröffentlichung und Livebeweis gesperrt. Lesend den realen G-Vertragsentwurf prüfen und genaue Anschlussmitglieder dokumentieren, nicht als verifiziert übernehmen. Keine neue G-Implementierung oder imaginäre G-Typen.

Referenzen: K/ARTEFAKT-ANSCHLUSS.md, K/H-INTEGRATION.md, vorhandener brain-maintenance::hero_compare_render, brain-contracts/src/store.rs record_publication_allowed, SnapshotReadPort und vorhandene brain-storage Release-/Kopfleser. Der bestehende Rust-Siteport ist in K integriert. Fachlich passende vorhandene Speicherbausteine wiederverwenden, niemals steckbriefspezifische persist_entity_document zweckentfremden.

## Eigentum

Eigener Schreibbereich: neue K-Artefaktmodule in brain-maintenance und brain-storage samt notwendigen eigenen Exports, disjunkte neue zugehörige Tests; additive eigene Migration im vorhandenen regulären Migrationspfad nach Prüfung des frisch gefetchten origin/main; Site src/bin/site/compare.rs und unmittelbar notwendige Site-mod.rs/tests.rs-Anbindung. Notwendige Crate-Manifeste nur minimal und nach konkretem Bedarf. Keine globalen Formatierungen. Nicht schreiben: alle brain-contracts-, brain-kernel-, brain-provider-, brain-serve-, dbrain-reasoner-, asset_mirror-, Analytics- oder anderen aktiven G/E/F/I-Dateien. Renderer/Renderer-Tests/Preview nicht ändern. Keine neuen Kommentare im Produktcode.

G-Worktree /home/nathanael/.worktrees/brain-g-v2-20261007 ist ausschließlich lesbar zur echten Vertragsbestimmung. G-Akte .tasks/2026-10-06-brain-abschluss/AN_HAUPT-G.md nennt tatsächliche APIs und aktive Eigentumsgrenzen. Keine G-WIP-Kopie, keine G-Freigabe behaupten. Graphify zuerst, dann konkrete Fundstellen lesen. Kein voller Graph-Neubau, keine Inventurkaskade wegen leerer Treffer.

## Arbeitsstand

Eigener Worktree /home/nathanael/.worktrees/brain-k-ki-20261007, Branch feat/brain-k-ki-20261007, bisheriger HEAD 99cbf1f8. Haupt-K sichert parallel nur bereits gestagte H-Integration samt H-Bericht; HEAD kann dadurch regulär voranschreiten. H-Fixer bereits abgeschlossen, nicht duplizieren. Eigene Artefakt-/Sitepfade vorher unverändert, andere Taskakten gehören Haupt-K. Kein Commit, Staging oder Push durch dich, keine Main-/Runtimeoperation. K allein Integrator/Deployer. Kein neuer T3-Thread, kein Sessionkontakt, keine weiteren Agenten spawnen. Origin wurde unmittelbar vor Zuteilung regulär frisch geholt.

## Beweisziel

Artefakt-Hülle serverseitig, nicht beliebig vom Modell/öffentlichen Request freigebbar. Ganze Rechnung/Szenario/Version/Mechanik und alle Abhängigkeiten an dieselben gerenderten Bytes und stabile ID binden. Immutable Release-Pins und frische kanonische Köpfe prüfen; explizit öffentliche Sichtbarkeit zusätzlich zu Principalrechten. Widerruf/Tombstone/Hash-/Versionskonflikt/fehlende Rechte führen geschlossen zur Ablehnung, auch bei Abruf oder Cachetreffer. Keine private Nutzer-/Chatdaten in öffentlichen Artefakten. Keine Modelllinks, Linkfilter nicht abschalten. Vor produktiver Anbindung fehlt bestätigter G-Eingang, daher keine synthetische öffentliche Freigabe oder behauptete Liveabnahme.

Compiler, Format, passendes scoped striktes Clippy und bestehende betroffene Tests ausführen. PG-Komponente mindestens einmal mit echter isolierter Postgres-Instanz prüfen, nie gegen Produktionsdaten schreiben. Neu angewandte Migration nur isolierter Testcluster; Prod nicht verändern. Vorhandene PG-Testwerkzeuge wiederverwenden. Testanzahlen und Exits ehrlich melden, Baselinefehler nur Zahl gegen Zahl. Keinen neuen Targetcache erzeugen, vorhandenen zentralen Cache und sccache verwenden. Buildslot tatsächlich halten: /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/build-slot-{1,2,3}.lock, `--jobs 3`. Nicht fremde Compiler stoppen oder Sperren löschen. Absolute moderne Cargo-CLI, passende bereits bewiesene Toolchain +1.97.1 für Maintenance falls nötig. Haupt-K fährt regulären lokalen Gate; echte BLOCK-Fixrunde erhält frischen Kontext. Keine zusätzliche Reviewer-Session.

## Routing und Bericht

Worker innerhalb teil-k, Versuch 1, zuständiger Auftraggeber Haupt-K Session 988eeaea-28ee-424c-b362-e250610cde91; übergeordneter Hauptorchestrator a711a4d2-1cad-4120-97ac-8b648567172b. Bericht ausschließlich K/ARTEFAKT-BAU.md. Keine zentralen Register/TODO-/Ereignisänderungen. Nach etwa 20 Minuten eigener Stand prüfen, spätestens nach 30. Gebaut, geprüft, G-angeschlossen und live getrennt melden. Bei realem unlösbarem Interfacekonflikt vorhandenen Code erhalten und konkret melden, nicht K-Eigentum fälschlich an G verschieben.

NEVER read, print or write plaintext secrets.
MUST NOT send private user/community data to remote models.
Private Nutzerdaten bleiben lokal. Loopback-Proxy zu externem Modell ist keine lokale Verarbeitung.
Infisical und normale Configdateien, keine ENV-Konfiguration.
Keine fremden Konten rotieren/trennen oder echte Moderationsaktionen als Test.
Rust, Postgres, keine neuen Code-Kommentare.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-k-ki-20261007
