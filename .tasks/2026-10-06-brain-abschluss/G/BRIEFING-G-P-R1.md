# G-P-R1: Gemeinsamer duplikatsicherer JSON-Eingang

## 1. Ziel und Vertrag

G-P hat den Werkzeugtransport fertiggestellt, aber seine echte Parserprobe bestätigt doppelte JSON-Argumentnamen in beiden Wireformen: `{"query":"A","query":"B","language":"german"}` wird still als Anfrage nach B akzeptiert. Beleg `G/pruefungen/g-p/json-probe.log`, ausführbare Probe `/tmp/brain-g-p-json-probe.rs`. Suite 29 passed und Baseline 19 passed, jeweils 0 failed und 0 ignored; Bereichsführung bestätigte Exits und Quellfingerprints. Vorhandenen WIP fortsetzen, keine komplette Neuimplementierung. Ursprünglicher G-P-Worker und G0 sind tatsächlich abgeschlossen.

Nach Graphify-Bestandssuche durch Bereichsführung: `dbrain-sources/src/external/strict_json.rs` enthält bereits einen rekursiven duplikatsicheren serde-Visitor mit unveränderten serde_json-Rekursions-/Zahlgrenzen. `dbrain-sources` und `brain-providers` hängen bereits an `brain-contracts`. Kleinstes gemeinsames Ergebnis: genau diese vorhandene Implementierung in den bestehenden Brain-Vertragsbereich verlegen, beispielsweise als öffentlicher reiner JSON-Eingang in `provider_input.rs`. Kein neuer Parser und keine neue Crate-/Manifestabhängigkeit. Der bestehende private `strict_json::parse(&[u8]) -> Result<Value, serde_json::Error>` in Sources behält seine Signatur und delegiert an dieselbe gemeinsame Implementierung. Bestehende API-/Schemaauswertung bleibt erhalten. Dieser begrenzte generische Anschluss ist vor Änderung in `AN_HAUPT-G.md` gemeldet; Es Importer, API-Pins, Analytics und Metadatenleser werden nicht verändert.

Provider muss vom tatsächlichen Roh-JSON aus prüfen, bevor serde_json doppelte Felder verlieren kann. Native Antwortobjekte einschließlich verschachtelter Toolargumente, OpenAI-Antwortobjekte und deren separat kodierte Argumentstrings sowie finales JSON derselben Antwortstrecke erfassen. Kein nachträglicher Value-Validator als angeblicher Duplikatbeweis. Fehler fail closed mit bestehender PortError-Abbildung. Keine Secret-/Payloadausgabe und kein Retry mit erneuerter Deadline oder Budgetinstanz.

Bestehenden Transport und acht Werkzeuge erhalten, einschließlich `game_rules`, Boonbereich und Analytics. Keine festen Toollisten, Modell-/Providerwechsel, Proxy-/Unitänderung oder Brückenaufrufe. Grafiken und Webseiten liegen separat beim Nutzer; G liefert strukturierte Zahlenreihen ohne Grafikbau oder Roadmapeintrag.

## 2. Eigentum

Exklusiv `rust/crates/brain-contracts/src/{lib.rs,provider_input.rs}` und unmittelbar zugehörige Vertragstests; `rust/crates/dbrain-sources/src/external/strict_json.rs` ausschließlich als kompatibler Delegationsadapter; `rust/crates/brain-providers/src/{lib.rs,transport.rs,hardening.rs}` und direkt betroffene vorhandene Providertests. Keine Änderung an `tools.rs`, Source-API-/Import-/Analyticsdateien, Manifesten, Kernel oder Reasoner. G-K/G-M schreiben parallel ausschließlich ihre eigenen Bereiche.

Keine neue zweite Parserimplementation, kein neues Crate oder Modul auf Vorrat. Vor Suche Graphify, dann konkret gefundene Stellen lesen. Rust only, keine neuen Code-Kommentare, ENV-Konfiguration, Modelle, festen Timeouts, Dienste oder Produktdatenänderungen. Vorhandene Dokumente und Prüfbelege nicht überschreiben. Eigene neue Rohbelege unter `G/pruefungen/g-p-r1/`.

## 3. Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`, HEAD `1f5ed30f`. Vertrag `3d6890c0` ist geprüft und gepusht, Provider-WIP noch uncommittiert. Vor Änderungen tatsächlichen HEAD/Diff prüfen, Fremd-WIP erhalten. Du bist alleiniger Schreiber der oben genannten Dateien. Kein Git, Gateumgehung, weiterer Agent/Workflow/T3-Thread, Produktions-DB, Releasebuild, Liveprovider, Deploy, Neustart, Tick, Cleanup oder Settle. Bestehender Buildslot, eigener Debugtarget, höchstens drei Cargo-Jobs. Keine konkurrierenden Compilerläufe.

## 4. Beweisziel

Geprüfte echte Providerantworten müssen Duplikate in beiden Wireformen abweisen: Top-Level und verschachtelte Objekte, Argumentstrings, finales Antwort-/Zitatobjekt, durch Unicode-Escapes identische Feldnamen. Gleichnamige Felder in verschiedenen unabhängigen Objekten bleiben gültig; gültige Zahlen, Nullwerte und Arrays dürfen nicht regressieren. Native und OpenAI-kompatible mehrstufige Call-/Result-Bindung sowie erweiterte Toolargumente bleiben grün. Keine Tests am Wortlaut von Nutzertexten oder mit echter Wall-Clock.

Compiler, kontrolliertes Format, striktes Clippy und vorhandene Vertrag-/Providerprüfungen, direkt betroffene bestehende Source-Strict-JSON-Prüfungen und Verbraucherkompilierung. Neue Fehler nicht in fremdem laufendem WIP verstecken. Vollständige Logs, unverdeckte Exits, genaue Testzahlen und unveränderte geprüfte Quellenfingerprints. Testserver beweist Parser/Transport, keine echte Luna-Brücke. Bereichsführung erstellt erst nach verifiziertem Abschluss einen schmalen Paketcommit und fährt den regulären `gate_hook.py --review`; Gate ist einziger Bug-/Securityreviewer, bei BLOCK frischer Fixer. Keine eigene Revieworchestrierung.

## 5. Routing und Rückgabe

Auftraggeber native G-Bereichsführung, Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`; Haupt-Orchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Genau ein Worker ist Statusproduzent dieses Versuchs. Rückgabe tatsächliche Funktionen/Dateien, wiederverwendeter Parser und erhaltene Source-Signatur, reproduzierte Ablehnung beider Probeformen, Befehle, Testzahlen, Quellenbindung und Grenzen. Register, AN_HAUPT und TODO bleiben bei Führung. Wache nach 20 Minuten. Keine Sessionnachrichten oder Nutzerfragen. Deutsche Produkttexte mit echten Umlauten, ohne Gedankenstriche.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
