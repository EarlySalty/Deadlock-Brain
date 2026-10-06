# A: frischer enger Fix 4 nach unabhängiger Nachprüfung

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-a

## Ziel und Vertrag

Du bist ein frischer nativer Rust-Fixer für genau den verifizierten Restdefekt aus `bereiche/a/REVIEW-LOCAL-4.md`. Ausschließlich geerbtes GPT 6.1 Sol, höchstens high, kein Wechsel oder Fallback. Keine neue A-Implementierung, kein zusätzlicher Featurebau. Nutzerziel bleibt vollständige originaltreue Wiki-Erhaltung mit getrennten Quellen, begrenztem Speicher, Autoren-/Lizenzbelegen und korrekter Wiederaufnahme.

Vor Code lesen: zentraler `.tasks/2026-10-03-wiki-spielwissen/CONTRACT.md`, `AN_BEREICHE.md` einschließlich Punkt28, eigener `REVIEW-LOCAL-4.md` und `FIX-3.md`. Bestandssuche mit code-suche/Graphify zuerst; globaler Graph vorhanden unter `/home/nathanael/.graphify/global-graph.json`, kein Neuaufbau.

Konkreter P2: `write_atomic` synchronisiert Herkunftsdatei und rename erfolgreich, letzter `sync_parent` schlägt fehl. Öffentlicher API-/XML-Retry öffnet nur root/documents dauerhaft und `persist_provenance` findet die sichtbare gleiche Assertion, kehrt aber vor nachgeholtem provenance-Elternsync erfolgreich zurück. Zeilen im Freeze: storage.rs387/388,425,712/713. Zwilling: conflict.exists()303-325 meldet „Konflikt erhalten“ ebenfalls ohne Nachholen.

Behebe eng die Wiederaufnahme: Vor entsprechender Erfolgs-/Erhaltenbestätigung muss die vorher fehlgeschlagene erforderliche Synchronisierung nachgeholt sein und jeder erneute Fehler weitergegeben werden. Prüfe dieselben tatsächlichen gemeinsamen Speicher-/Publikationspfade auf diesen konkreten Zwilling. Existenz allein ist keine Dauerhaftigkeitsbestätigung. Budget, unveränderliche Originale, Quellenbindung, Erstbeobachtung und widersprüchliche Herkunft bewahren. Keine Grenzen erhöhen, keine bestehenden Tests abschwächen. Geeignete gezielte Regression auf den tatsächlich nach Rename/Hardlink fehlgeschlagenen abschließenden Elternsync samt Wiederholung; bisherige Injektion vor Dateischreiben reicht nicht. Echte temporäre Dateien verwenden, keinen vollständigen Fake-Spool. Kein Stromausfall-/Kernel-Crashbeweis behaupten.

## Eigentum und Ausgangsstand

Worktree `/home/nathanael/.worktrees/brain-wiki-spielwissen-a`, Branch `feat/brain-wiki-spielwissen-a`, HEAD `2734c2da4e814ff79953e8e825275b0216a6af16`. Eigene neue Module uncommittiert.

Schreibfreigabe ausschließlich für:
- rust/crates/dbrain-sources/src/wiki_inventory/storage.rs
- rust/crates/dbrain-sources/src/wiki_inventory/tests.rs
- .tasks/2026-10-03-wiki-spielwissen/bereiche/a/FIX-4.md

Keine Änderung an wiki_inventory.rs, normalize.rs, Core, produktiven Manifesten/Lockfiles/lib.rs, Harness oder Daten. Native Agenten teilen das Dateisystem. Fremde Arbeit erhalten, keine globale Formatierung, keine Secrets/ENV/Community-/Netz-/Git-/Deployschritte. Keine weiteren Agenten oder fremden Sessions.

Eingangs-Freeze vor Änderungen messen:
- wiki_inventory.rs: 2507c127aaf8e366f7bb7cf22564074adbaa546b344328d6fdbd593e5b38ef80
- normalize.rs: 27e63412a7a57c956dc82e602df4f93683adb3eb06866b22381157f85b96dcdc
- storage.rs: a16a36d1880a5eb3c07856c39e4c7f6b18b9a168aa4cac0de93961ae34f6827b
- tests.rs: 5e71ade9d106a9adffc5935a9d141741b58805a3d4cf15eb01d324901eb9bef4

## Belegter sicherer Schreibpunkt

Der bestehende Datenworker bestätigt um 07:08:16UTC ausdrücklich: eigener test-5-Wrapper bjnkctnph/PID3444003 war noch vor erstem Hostlock/Cargo, wurde nur wegen dieses notwendigen P2-Fixes beendet, PID verschwunden, eigene FD8/9 geschlossen, keine eigenen Compilerkinder/verwaisten flock-Prozesse. Eigener Startwächter regulär beendet mit OWN_WRAPPER_ENDED_WITHOUT_CARGO. Keine fremden Eingriffe. Keine neuen Läufe bis nächstem bestätigten Freeze. Du erhältst damit jetzt die enge Schreibfreigabe. Kein Timerabbruch.

Der vorherige echte test-4-Lauf startete 06:47:12UTC und endete Exit101 mit drei E0583 allein in eigener #[path]-Harness-Anbindung, keine Tests. Derselbe Datenworker korrigierte seinen Harness inzwischen durch normale Modulregistrierung/eigene Symlinks auf die unveränderten Module. Er besitzt weiterhin Compiler-/Test-/vollständige Datenläufe; du startest KEINE Compiler, Cargo, Clippy, Tests oder Hostlockwrapper. Gezielter rustfmt auf deinen erlaubten zwei Dateien ist zulässig, keine Child-/globale Formatierung. Ausführung und finaler Gate bleiben getrennt.

## Nachweis und Routing

Alle 33 bestehenden Regressionen erhalten. Bericht FIX-4.md: tatsächlicher enger Fix, konkreter Zwilling, Regressionen geschrieben versus ausgeführt, tatsächlicher Formatcheck/Exit, abschließende vier SHA256 nach Formatcheck. Die zwei nicht erlaubten Module müssen bytegleich bleiben. Anschließend alle Module ausdrücklich einfrieren, keinerlei weitere Moduländerung durch dich. Keinen Lauf oder allgemeine Freigabe erfinden.

Auftraggeber Teil-Orchestrator A f01cce67-209b-468e-8abb-ec2070beeaa2, Hauptorchestrator /root. Paket a/Versuch1, alleiniger Statusproduzent teil-a. Keine Register/Status/TODO-Schreibrechte. Melde nativ an A; frische unabhängige Nachprüfung und derselbe Datenworker übernehmen danach. C2 alleiniger Integrator/Gate-/Deployer und Eigentümer der vorhandenen get_bounded-Anbindung. Keine zweite Coreimplementierung.
