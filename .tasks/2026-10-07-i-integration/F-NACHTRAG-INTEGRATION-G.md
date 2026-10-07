[Orchestrator]
# F: begrenzte Integrationszuständigkeit und echter G-Vertrag

Der bestehende native F-Ausführer bleibt einziger Executor. Sein erster regulärer Mainmerge erzeugte sieben tatsächliche Konflikte außerhalb der zunächst engen F-Dateiliste; er hat den selbst begonnenen Merge zurückgenommen, 46fd8674 und alle Artefakte erhalten, keine neue Codeänderung oder Tests/Gate. Kein Schutzblocker oder Gate-BLOCK daraus gemacht. Kein zweiter Worker gestartet, derselbe Kontext geordnet fortgesetzt.

I präzisiert ausschließlich die dafür nötige Integrationszuständigkeit im eigenen F-Baum: die übernommenen E-Baselinekonflikte brain-storage/src/asset_mirror.rs, brain-storage/src/lib.rs, brain-storage/tests/support/asset_mirror.rs, dbrain-sources/src/assets_api.rs und scripts/run_build_data_with_infisical.sh exakt aus dem tatsächlich gelieferten b7289d11 übernehmen. Beide Storage-Modulnamen asset_mirror und compare_artifact bleiben. Die tatsächlichen F-Produktcommits haben diese E-Dateien nicht geändert, es ist kein neuer E-Funktionsfix. Übernommene AN_HAUPT-E.md-Historie aus aktuellem Main erhalten. Gemeinsame tatsächlich zugewiesene F-CLI-Naht main.rs als Vereinigung der E-Assets-/data-dir-Verträge und F-Publish-/Request-/Herkunftsprüfung auflösen, ohne Ks Query-Literale umzubauen. Verweigerte main.rs-Inspektion im I-Integrationsbaum nicht auf anderen Wegen übernehmen; unveränderlicher Releasebaum bleibt unangetastet.

## Tatsächliche geordnete G-Lieferung

Delegator hat UEBERGABEN-WACHE-032.md zugestellt; I hat sie regulär gelesen und dem bestehenden nativen F-Ausführer weitergegeben:

- S3 2b519b0470fdb2d0ae9fde7791dff0c820ad3d1b.
- S4 a35bd8142146bd1ea0d8b342625ac6cf2e0bd278, enthält S3.
- Gesicherter Originstand feat/brain-v2-g-20261007 253d383eab7a2f018ee1c834370140bf4a55e844, Dokumentcheckpoint, Produkt gleich zu S4. Origin-/Ancestor-Exit 0 durch Delegator belegt, nicht als eigener neuer Test ausgegeben.

S3/S4 regulär ALLOW, Compiler/Clippy/Format Exit 0, 38 Rechenfälle bestanden. G-Reasoner S4 342 passed/12 failed, S2 304/12 wegen dokumentierter DSN-/Fixturevoraussetzungen. Keine grüne Vollsuite und keine Erlaubnis, bestehende Tests still zu überspringen. Kombinierten F/G-Stand tatsächlich prüfen und Voraussetzungen regulär herstellen oder konkreten neuen Blocker melden.

Gemeinsame tatsächliche Eingänge calculation_models_from_payloads, calculate_hero_with_deadline, project_hero, hero_growth und compare_hero_curves. Gesicherte Gitcommits im eigenen F-Baum integrieren, keine fremden WIP-Dateien lesen/kopieren, nicht auf G-main warten. Originalrequestdeadline, dieselben E-Original-/Receipt-/Versionswerte, unveränderte Publish-Abnahme. Keine zweite Rechnung oder Scheinadapter. analytics_runtime ausdrücklich an G freigegeben und nicht von I/F parallel beschrieben.

Normaler build-data --hero all-Job ist inzwischen als eigener Hintergrundbefehl Exit 0 beendet; I prüft tatsächlichen Unitabschluss und Importmetadaten getrennt, statt vollständigen Erfolg aus Exitcode zu behaupten. F läuft parallel weiter. Keine neue Freigabefrage, kein Sessionchat oder zusätzlicher T3-Thread.
