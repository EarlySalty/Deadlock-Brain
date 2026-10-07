# G-K: Begrenzter Werkzeugloop im bestehenden Kernel

## 1. Ziel und Vertrag

Erweitere den vorhandenen Kernel mit G0s verifiziertem Providerturn und lesendem ToolExecutionPort. Lies `G/G0-VERTRAG.md`, die bestätigte G0-Rückgabe, `G/PLAN.md` C3/C5 und Abschnitte 6/8, `G/BESTAND-ANTWORT.md`, `G/BASELINE.md`. Aktenwurzel: `/home/nathanael/.worktrees/brain-g-v2-20261007/.tasks/2026-10-06-brain-abschluss/`.

Kernel besitzt die Schleife: Providerturn, geschlossene validierte Toolcalls, serverseitige Ausführung, geprüfte Toolresultate, nächste Runde desselben Providers, echte finale Antwort. Keine Toolausführung im Provider. Vorhandene reine Text- und Domainaufrufer bleiben kompatibel. Eine leere initiale Belegliste darf beim echten Werkzeugweg nicht die Werkzeugbeschaffung verhindern; gleichzeitig darf eine unbelegte Finalantwort nicht als beantwortet durchgehen. Aktuelle Spielzahlen werden nicht aus einem alten Textprofilpaket beschafft. G-V liefert später den echten Dispatcher; hier keine Fake-Spielwerte oder neue Datenpipeline.

Actor, Consumer, einschränkbare Scopes, Release, Anfrage-Pin und ursprüngliche RequestDeadline bleiben serverseitig. Kein Modellargument kann Rechte erweitern oder Deadline/Version erneuern. Ausführung und Validierung über den autorisierten Toolport, unbekannte oder beschädigte Calls fail closed. Sämtliche Belegabhängigkeiten behalten konkrete typisierte Unteranfragen. Vor Modellweitergabe und finaler Ausgabe aktuelle/historische Rechte, Quellen und Publikationsfreigaben neu prüfen, auch nicht zitierte Eingaben. Quellen dürfen nicht durch berechnete Zahlen gewaschen werden.

Budgets über vollständige Anfrage kumulieren: Netzwerkrounds einschließlich Retry und externer Faktenabfrage, gesamte wiederholte Modelleingabe, Tooldefinitionen/Argumente/Ergebnisse, Output, Usage und Kosten. Bestehende Budgetfelder/G0-Zählung verwenden. Keine frische Deadline oder Budgetinstanz pro Turn, kein eigener Timeout/ENV-Schalter. Lokale Rechnung verursacht keine Netzrunde, ihre Weitergabe zählt als Input. Vor und nach jeder Runde sowie während Tool-/CPU-Ausführung Abbruch beachten.

Cache/Single-Flight bindet bestehenden Release-/Actor-/Rechtekontext, Mechanik-/Provideridentität und echten Anfrage-Pin/Szenario. Toolabhängigkeiten und Unteranfragen mitspeichern und vor Trefferausgabe neu prüfen. TTL allein reicht nicht. Keine Actor-/Scope-Vermischung und kein teilweiser Cachebeweis nur anhand zitierter Quellen. Verwende den bestehenden Cacheweg statt einen zweiten Cache aufzubauen.

## 2. Eigentum

Exklusiv `rust/crates/brain-kernel/src/{lib.rs,execution.rs,flight.rs}` und direkt betroffene bestehende Kerneltests/Fixtures. Wenn ein weiterer bestehender Cachebaustein tatsächlich geändert werden muss, Fundort und kleinsten Änderungsbedarf an Bereichsführung zurückgeben, nicht spontan Scope erweitern. G0 besitzt Brain-Verträge, G-P Provider, G-M Reasoner, G-V Storage/Serve. Keine Änderungen dort, an Policy/API oder Konfiguration ohne begrenzte Übergabe. Rust only, keine neuen Code-Kommentare, Modelle, ENV-Schalter, festen Timeouts, Dienste oder Crates.

Graphify vor Codebestandsfragen: vorhandener Graph `--graph /home/nathanael/repos/Deadlock-Brain/graphify-out/graph.json`, danach gezielt Code lesen. Kein Extraktionslauf.

## 3. Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`. Start nach tatsächlich verifiziertem G0-Vertrag; Bereichsführung nennt HEAD und Nachweise. Dokumentcheckpoint `f129c91a` ist gepusht. Eigene und andere G-Paketstände nicht zurücksetzen. Keine Agenten-/Sessionnachrichten und kein weiterer Agent/Workflow/T3-Thread. Kein Git, Produktions-DB, Releasebuild, Deploy, Neustart oder Tick. Vorhandener Buildslot, eigener Debug-Target, höchstens drei Cargo-Jobs, kein Compiler-Rennen.

## 4. Beweisziel

Eigene und vorhandene Verbraucher kompilieren. Passende fmt-, strikte Clippy- und bestehende Kernelprüfungen mit vollständiger Ausgabe, unverdeckten Exits und tatsächlichen Testzahlen. Baseline Kernel: 34 eindeutige passed, 18 failed, 0 ignored. Ursachen und ausgeschlossene Grenzen stehen in `G/BASELINE.md`; keine neue Regression darin verstecken. Gebrochene betroffene Fixtures nachvollziehbar nachziehen, nicht Tests löschen/ignorieren/abschwächen.

Fachbeweise: echte zweiründige Provider-/Toolzustandsmaschine, mehrere Calls mit passender Zuordnung, unbekannter Name/Felder, Actor-/Versionmanipulation, unzulässiger Quellen-/Publikationsbeleg, nicht zitierter ungültiger Toolinput, Budgetbruch in Folgerunde/Retry, Deadline während Toolausführung, aktueller Grant-/Quellen-/Versionswechsel vor Cachehit und finaler Ausgabe, verschiedene Scopes/Actors bei Single-Flight. Fakes sind für isolierte Kernelzustände zulässig; sie beweisen nicht E-Import oder Luna-Brücke. Keine Nutzertextwortlaut- oder Wall-Clock-Tests.

Schmalen prüfbaren Diff und Rohbelege an Bereichsführung. Sie fährt `gate_hook.py --review` auf dem verifizierten Paketcommit. Gate ist einziger Reviewer, bei BLOCK frischer Fixer. Technische Gateausfälle nicht umgehen.

## 5. Routing und Übergabe

Auftraggeber native G-Bereichsführung, Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`; Haupt-Orchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Worker ist Statusproduzent dieses Versuchs. Optional `G/G-K-NACHWEISE.md`, Register/AN_HAUPT/TODO bleiben bei Führung. Wache 20 Minuten. Rückgabe exakte API/Dateien, Rechte-/Budget-/Deadline-/Cachebeweise, Befehle und Testzahlen sowie verbleibende echte Dispatcher-/Providerintegration. Keine Nutzerfrage, deutsche Produkttexte mit echten Umlauten, keine Gedankenstriche.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
