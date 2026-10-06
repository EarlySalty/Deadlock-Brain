status: aktiv
Datum: 2026-10-03

# D/B-Lesebindung: enger Bereitschaftsvorcheck

[Orchestrator]
BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-b

## Ziel und Vertrag

Root verlangt jetzt belegte Antwort: Ist die unmittelbare Bindung tatsächlich vom B-Lesepfad verwendeter Dateien an das D-Manifest-/Dateiinventar schon implementiert und passend prüfbar, oder fehlt ein enger Bauteil? Kein neuer Parser, keine Rohdaten erfinden. Gemeinsame Prüfung auf endgültigen D/B-SHAs vor Steam-Deploy bleibt Pflicht; tatsächliche Depotextraktion erst danach.

Zentrale Akte `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-wiki-spielwissen`: CONTRACT.md, AN_BEREICHE.md insbesondere Punkte31/41 und vorhandene Bereiche c/d. Eigener bestehender D-B-LESEVERTRAG-B2.md, EIGENABNAHME-B2.md und UEBERGABE.md. C3 aktiv und alleiniger Integrator/Deployer, C2 beendet. D besitzt Zugang, regulären Download und Writer, kein weiterer Login oder Lizenzrequest.

## Eigentum

Ausschließlich lesender Vorcheck und strukturierter Rückbericht, keine Quellen- oder Berichtsedits, Compiler, neuen Prüfer, Gitmutationen, Datenbank-/Live-Aufgaben, Downloads oder Secrets/ENV-Konfiguration. Abgenommener Parser-Eigencommit und seine sieben Dateien bleiben unverändert. Keine Subdelegation oder Kommunikation mit fremden Sessions. Native Worker teilen das Dateisystem, fremde Writer nur lesen und aktuelle Hashbindung ausdrücklich als vorläufig kennzeichnen.

## Arbeitsstand

B-Worktree wie oben, Branch feat/brain-wiki-spielwissen-b, Eigencommit48b6ce1cf277ec4de4b47ac4a0898fbc223fe6b5, Parent2734c2da4e814ff79953e8e825275b0216a6af16. Eigene Akte untracked. Geprüfter Elfdatei-Freeze3cef2c79fd36c9462948f90ceb4fa28b6b82b6ad1102769a8b8812a66132055c. Keine aktive B-Prüfung/Writer. C3 übernimmt nur Eigencommit.

Bekannte Integrations-Worktreepfade nur lesen:
- `/home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration`, Rust-Brain-Integration, C3 besitzt Cargo/Lock/lib.rs, CLI/Import und gemeinsame Schnittstellen.
- `/home/nathanael/.worktrees/steam-brain-spieldepot-d`, bestehender D-Writer `rust/crates/steam-core/src/task/handlers/game_download.rs` und `game_download/`, D besitzt diese Dateien.

Graphify bei jeder Codefrage zuerst. Globaler Graph enthält neue uncommittierte Module teilweise nicht; danach bekannte Vertragspfade und gezielte Rust-Nebenpfade prüfen. Kein Graph-Neubau. Eltern-Vorprobe im C3 dbrain-sources/src ergab keine Fundstelle für GameFileOptions/extract_game_files/manifest-inventory. Das ist kein Nachweis über den ganzen Rust-Workspace; relevante andere Caller/CLI/Importpfade vollständig nachprüfen, keinen zweiten Pfad vorschlagen ohne Bestandssuche.

## Beweisziel

1. Tatsächlichen D-Publish-/Inventarpfad und aktuellen B/C3-Caller bis zu konkret verwendeten Originalbytes nachzeichnen, aktuelle SHA-/Dateihashbindung und genaue Zeilen nennen. Keine historische D-Prüfung als aktuellen Endstand ausgeben.
2. Prüfen, ob D-Options-/Provenienzmetadaten allein kopiert werden oder ob Root, App/Build/Depot/Manifest, relative Dateiliste, Typ, Größe und SHA-1/SHA-256 unmittelbar im Leseübergang an tatsächlich verwendete Dateien gebunden sind. Hinzugekommene/ausgetauschte/unbelegte Dateien dürfen keine bestätigte Manifestprovenienz bekommen.
3. VPK: physische Container/Begleitdateien aus D-Inventar von Ressourcenbytes/Ressourcenhash unterscheiden. Gehaltene Deskriptoren, Pfadanker und Wechsel zwischen Vorprüfung und tatsächlichem Lesen beachten. Kein Hashvergleich nur gegen erneut separat geöffnete Dateinamen als Beweis der genutzten Bytes ausgeben.
4. Existierende passende Prüfbausteine und reale Zustands-/Dateiwechsel-/Begleitarchivfälle nennen. Vorhandener Git-Validator ist noch keine Steam-/VPK-Abnahme. Keine Tests starten und keine synthetischen Daten als echte Depotabnahme bezeichnen.
5. Ergebnis maximal konkret: Bereitschaft J/N, bestehende Implementierung mit Pfad/Funktion, tatsächlich fehlender enger Bauteil und betroffene Dateien/Eigentümer, nächster ausführbarer Schritt. Bei fehlendem Caller ebenfalls klar melden. Kein breit angelegter Refactor, keine zweite Parserimplementierung, keine Änderung am accepted Eigencommit. Technische Kandidaten erst vorschlagen, nachdem bestehende Nebenpfade geprüft sind.

## Routing

Frischer eigener nativer lesender Worker, ausschließlich geerbtes GPT6.1Solhigh, keine anderen Modelle/xhigh/max. An Elternsession B2 8e61edb7-3a14-4274-84bf-569122a7241c melden. Paket b/Versuch2, alleiniger Statusproduzent teil-b. Eltern hält Bericht und kurze UEBERGABE/AN_HAUPT für Root und C3 fest. Zentrales Register nur Root, TODO nur S. Noch keine endgültige gemeinsame D/B-Abnahme oder Steam-Deployfreigabe; deren finale SHAs und echte Datenbelege fehlen.
