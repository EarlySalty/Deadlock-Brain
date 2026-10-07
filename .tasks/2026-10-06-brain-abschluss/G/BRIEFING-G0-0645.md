# G0-06:45: Nutzerentscheidungen im Werkzeugvertrag fortsetzen

## 1. Ziel und Vertrag

Setze den geprüften vorhandenen G0-Vertrag fort, keine Neuimplementierung. Nutzerentscheidung `VON_HAUPT.md`, Abschnitt 06:45, und aktualisierter `G/PLAN.md` sind verbindlich. Der bisherige Sieben-Tool-Vertrag ist in `b4f4b866` gesichert, reguläres Gate ALLOW. Konkrete Anschlussbeschreibung `G/G0-VERTRAG.md`. Drei Ergänzungen im gleichen Vertrag: `game_rules`, Boonkurven/Überholvergleich in `hero_compare`, nach Rang und Zeit filterbare API-Meta-Auswahl in Profilen/Vergleichen. Keine Produktwerte oder Analytics-Implementierung in diesem Paket.

Ergänze geschlossene ToolName-/ToolSubrequest-Varianten für `game_rules` mit konkreten typisierten Argumenten. Themen: Kill-Bounty, Comeback, Urne, Midboss, Resist-Stapelung und Ressourcen. Optional betroffene Entität, Spielzeit und typisiertes Szenario, keine frei ausführbaren Formeln oder beliebigen Parameterobjekte. Namen und fachliche Grenzen sind geschlossen; gültige 0-/Negativwerte nicht durch Ersatzdefaults verlieren. Spielbindung und Mechanikrevision bleiben serverseitig.

`hero_compare` erhält einen optionalen typisierten Boonbereich, damit dieselbe skalare Projektion eine Kurve und Überholpunkte liefert. Die tatsächliche gültige Boondomäne bestimmt später der gepinnte Fachport. Keine erfundenen Obergrenzen oder ungezählten versteckten Parameter. Profile/Vergleiche erhalten optionale typisierte Analytics-Auswahl mit Rang- und Zeitfilter. Prüfe dafür den bestehenden `dbrain-sources/src/analytics_runtime.rs` und Original-API-Pin als lesbare Referenz. Keine frei gesetzte Patchmitgliedschaft und keine aktuelle Clientversion aus Modellargumenten. Endgültige Feldnamen, Einheiten und API-Zuordnung dokumentieren; keine neuen Meta-Scores, Labels oder Sondergewichte.

Alle zusätzlichen Argumente bleiben in ToolRequest, Abhängigkeiten, Wirehistorie und vollständiger Eingabezählung erhalten. Erhalte die kompatiblen alten Textports, sicheren Defaults, Call-ID-/Ergebnisbindung und bestehenden sieben Unteranfragen. Erst Graphify, danach gezielte Fundstellen; kein erneuter Sheet-/Baseline-/Bestandslauf.

## 2. Eigentum

Exklusiv `rust/crates/brain-contracts/src/{lib.rs,provider_input.rs,tools.rs}` und unmittelbar dazugehörige Cratetests im eigenen Worktree. Schreibe zusätzlich deinen eigenen Bericht `G/G0-0645-VERTRAG.md`; den historischen `G/G0-VERTRAG.md` und seine 55-Test-Belege unverändert lassen. Rohlogs nach `G/pruefungen/g0-0645/`. Kein Reasoner, Provider, Kernel, Storage, Analytics-Quellmodul, Manifest oder Lockfile. Keine globale Formatierung. G-M ist ein getrennter aktiver Reasoner-Schreiber; dessen WIP erhalten. Register, PLAN, AN_HAUPT und TODO bleiben bei Bereichsführung. Rust only, keine neuen Code-Kommentare, ENV-Schalter, Modelle oder festen Timeouts.

## 3. Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`, gepushter HEAD `b4f4b866`. Aufgabenakte unter `.tasks/2026-10-06-brain-abschluss/`. G0 ist momentan schreibfrei; du bist der einzige beauftragte Vertragsschreiber. Kein Git, weiterer Agent/Workflow/T3-Thread, Releasebuild, Produktions-DB, Dienst-/Runtimeeingriff oder Sessionnachricht. Release-Hold bleibt bestehen. Native Worker teilen das Dateisystem; fremde Änderungen nicht zurücksetzen.

## 4. Beweisziel

Compiler, Format ausschließlich eigener Dateien, striktes `cargo clippy --package brain-contracts --all-targets --locked --offline --jobs 2 -- -D warnings`, bestehende Suite mit `--include-ignored --test-threads=1`, passende Verbraucherkompilierung. Vorhandener Buildslot `/tmp/deadlock-cargo-release.lock`, eigener Debugtarget, höchstens zwei Jobs. Keine parallelen Cargo-Rennen.

Baseline des geprüften G0-Standes: 55 passed, 0 failed, 0 ignored. Tatsächliche Zahlen und unverdeckte Exits protokollieren, Quellen mit Fingerprints binden. Bestehende Verträge dürfen nicht brechen. Fachbelege: achter Name/Unteranfrage, geschlossene Themen, ungültige Range-/Rang-/Zeitwerte, serverseitige Bindungen, Erhalt der neuen Argumente in allen Abhängigkeiten und beiden Wireformen, Call-/Ergebniszuordnung und vollständige Eingabezählung. Kein Fake-Fachport als Beweis für reale Spielregeln/Analytics. Gate bleibt der einzige Reviewer; Bereichsführung fährt es auf deinem verifizierten Paketcommit. Bei nicht auflösbarer API-/Fachgrenze konkrete belegte Lücke melden, keine Semantik erfinden.

## 5. Routing und Rückgabe

Auftraggeber native G-Bereichsführung, Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`; Haupt-Orchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Worker ist Statusproduzent dieses Pakets. Rückgabe: exakte Signaturen/Feldnamen, Dateiliste, Kompatibilität, Original-API-Anschluss, Befehle/Exits/Testzahlen, Fingerprints und kleinster G-P/G-K/G-V-Anschluss. Erste Wache 20 Minuten. Keine Nutzerfrage oder weitere Delegation. Deutsche Produkttexte mit echten Umlauten, keine Gedankenstriche.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
