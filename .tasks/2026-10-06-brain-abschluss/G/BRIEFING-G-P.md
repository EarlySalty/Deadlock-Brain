# G-P: Bestehender Providertransport mit Werkzeugturns

## 1. Ziel und Vertrag

Implementiere den von G0 verifizierten Turn-Vertrag im vorhandenen `brain-providers`, keine neue Providerart und kein Connector. Lies `G/G0-VERTRAG.md`, G0-Rückgabe, `G/PLAN.md` C3 und Abschnitte 7/8, `G/BESTAND-ANTWORT.md`. Arbeitswurzel für diese Akte: `/home/nathanael/.worktrees/brain-g-v2-20261007/.tasks/2026-10-06-brain-abschluss/`.

Native `codex_subscription`-Messages müssen Toolschemas, Toolwahl, echte Assistant-`tool_use`-Blöcke und gebundene User-`tool_result`-Blöcke senden und lesen. OpenAI-kompatibler Bestand erhält `tool_calls`, nullable Content und zugehörige Call-Ergebnisse. Typisierte Abschlussgründe unterscheiden Werkzeugturn, echte Finalantwort, Abschneiden und ungültige Antwort. IDs/Namen/JSON-Argumente strikt prüfen; kein stilles Verwerfen oder Reparieren beschädigter Blöcke. Finale Antwort behält bestehenden JSON-/Zitatvertrag. Bestehende reine `answer`-Aufrufer bleiben kompatibel.

Der Kernel besitzt den Loop. Provider führt kein Werkzeug aus und darf keine Rechte, Releasebindung, Deadline oder Budgets erneuern. Dieselbe ursprüngliche RequestDeadline gilt für Netzwerk, Retries und alle Turns. Verwendung desselben konfigurierten Providers und Modells, keine fest eingebauten Modelle/Timeouts. Usage, Retries und Kosten tatsächlich zurückgeben. Gesamte Modell-Eingabe einschließlich wiederholter Historie, Definitionen, IDs, Argumente und Resultate mit G0s gemeinsamer konservativer Eingabezählung erfassen. Vor Versand Budget/Deadline prüfen; keine ungezählten Felder oder eigene bytes/4-Schätzung.

Die vorhandene Abobrücke 0.1.43 unterstützt laut öffentlichem Quellstand Übersetzung in beide Richtungen. Kein vorsorglicher Proxyumbau. Eine echte Luna-Probe erfolgt später durch die integrierte vorhandene Providerstrecke; dieser Worker ruft keinen Liveprovider auf und liest keine Secrets oder Prozessumgebung.

## 2. Eigentum

Ausschließlich `rust/crates/brain-providers/src/{lib.rs,transport.rs,hardening.rs}` und unmittelbar dazugehörige bestehende Provider-Cratetests. G0 besitzt `brain-contracts`; fehlende Signatur mit konkretem Bedarf zurückgeben statt den Vertrag parallel zu ändern. Kernel, Reasoner, Storage, Serve, Konfiguration, Units, Proxy und Botdateien bleiben unverändert. Rust only, keine neuen Code-Kommentare, ENV-Schalter, Modelle, Timeouts, Dienste oder zusätzliche Crates.

Graphify vor Bestandssuche über `--graph /home/nathanael/repos/Deadlock-Brain/graphify-out/graph.json`, danach Quellen lesen. Kein Graph-Neubau.

## 3. Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`. Start erst nach tatsächlichem G0-Vertragsabschluss; Bereichsführung gibt dessen HEAD und Nachweise im Startauftrag mit. Dokumentcheckpoint `f129c91a` ist gepusht. G-M schreibt getrennt Reasoner, G-K getrennt Kernel. Uncommittierte eigene und fremde G-Paketstände erhalten. Kein Git, Produktions-DB, Releasebuild, Deploy, Neustart oder neuer Agent/Workflow/T3-Thread durch dich.

## 4. Beweisziel

Provider und bestehende Verbraucher kompilieren; passende fmt-, strikte Clippy- und Bestandssuites mit voller Ausgabe und echten Testzahlen. Debug-Target/Compiler gemäß bestehendem Buildslot, höchstens drei Cargo-Jobs. Kein paralleles Cargo-Rennen.

Wire-Prüfung mindestens: Tooldefinition/Toolwahl, Werkzeug-only-Antwort mit nullable Text, gemischte erlaubte Blöcke, zwei Turns mit unverändertem Call-ID-Binding, unzulässiger Toolname, doppelte/fehlende ID, kaputte Argumente, fremder Ergebnisbezug, unbekannter Abschlussgrund, abgeschnittene Ausgabe, kumulierte Usage/Budgets, Deadline im Retry und finale Zitatvalidierung. Transportnahen bestehenden Testserver verwenden; dessen Erfolg nicht als Abobrückenbeweis ausgeben. Keine Wortlauttests für Nutzertexte und keine echten Wall-Clock-Annahmen.

Übergib einen schmalen prüfbaren Diff; Bereichsführung sichert den verifizierten Paketstand als Commit und fährt `gate_hook.py --review`. Gate ist einziger Bug-/Securityreviewer, bei BLOCK frischer Fixer. Technische Ausfälle nicht umgehen.

## 5. Routing und Übergabe

Auftraggeber native G-Bereichsführung, Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`; Haupt-Orchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Worker ist Statusproduzent dieses Versuchs. Rückgabe über Workflow, keine Nutzerfragen oder Sessionnachrichten. Optional `G/G-P-NACHWEISE.md`; Register, AN_HAUPT und TODO nicht schreiben. Wache nach 20 Minuten. Bericht: exakte Dateiliste/API, beide Wireformen, Usage-/Deadlinebeweis, Befehle und passed/failed/ignored, verbleibende echte Brückenprobe. Deutsche Produkttexte mit echten Umlauten, keine Gedankenstriche.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
