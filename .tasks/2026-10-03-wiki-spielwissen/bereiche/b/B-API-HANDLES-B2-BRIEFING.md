status: aktiv
Datum: 2026-10-03

# B-API für gebundene Dateiobjekte: Signatur zuerst

[Orchestrator]
BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-b

## Ziel und Vertrag

Root Punkt44 autorisiert engen API-Bereich. Lies zentrale C3-Abstimmung `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-wiki-spielwissen/bereiche/c/C3-B-BYTEVERTRAG.md` vollständig, AN_BEREICHE.md Punkte43/44, eigenen D-B-READERABNAHME-B2.md und D-B-BEREITSCHAFT-B2.md. C3 liefert vollständig inventargebundene private unveränderliche Lesesicherungen und gehaltene lesbare Dateiobjekte; B muss diese im vorhandenen Extraktor verwenden, ohne erneute Namensöffnung oder freie Roottraversierung, auch für VPK-Begleitarchive. Bestehende Git-/Lose-Datei-API kompatibel erhalten, kein zweiter Parser. C3 besitzt Normalisierung/Validierung des D-Inventars und Sicherung der Originalbytes, C3 alleine Caller/Registrierung/Import. D alleine Download/Writer.

**Jetzt nur Phase1:** kleinstmögliche konkrete öffentliche Rust-Signatur/Datentypen vorschlagen, genaue benötigte Pfade bestätigen und wichtige Vertragsbedingungen nennen. Noch keine Edits/Compiler/Commit. Kurzer Rückbericht an B2, dann Eigentum ruhend bis B2 die Signatur an Root bestätigt und ausdrücklich Umsetzung per eigener Fortsetzungsnachricht freigibt. Keine eigenständige Ausweitung oder neue Unteragenten. Keine weitere Nutzerfreigabe erfragen; Point44 ist Umsetzungsauftrag, Eltern hält nur vorgeschaltete Root-Abstimmung ein.

## Eigentum

Für die spätere Umsetzung exklusiv genau `rust/crates/dbrain-sources/src/game_files.rs`, `game_files/anchored.rs`, `game_files/vpk.rs` samt dortigen bestehenden Testmodulen. Phase1 ausschließlich lesen. Vier übrige Parsermodule, Cargo/Lock/lib.rs/CLI, D-/C3-Dateien und Aufgabenakte nicht ändern. Bei echtem zusätzlichem Bedarf konkrete Fundstelle und Grund melden, nicht selbst anfassen. C3 schreibt diese B-Pfade nicht parallel. Keine Secrets/ENV-Konfiguration, Live-Aufgaben, Downloads, Datenbankzugriffe oder andere Sessions.

## Arbeitsstand

Branch feat/brain-wiki-spielwissen-b, HEAD/Eigencommit48b6ce1cf277ec4de4b47ac4a0898fbc223fe6b5, Parent2734c2da4e814ff79953e8e825275b0216a6af16. Eigencommit ist unabhängig abgenommen, remote gesichert und bleibt unverändert, kein Amend/Rebase. Eigene Akte untracked, keine anderen B-Writer/Compiler oder Prüfer aktiv. Elfdatei-Basisfreeze3cef2c79fd36c9462948f90ceb4fa28b6b82b6ad1102769a8b8812a66132055c. Später zusätzlicher Eigencommit nur erlaubte API-Pfade, Eltern führt Git/Root-Übergabe.

## Beweisziel

Phase1: tatsächlich bestehende Dateiöffnungsgrenzen perGraphify zuerst und bekannten Quellen nachlesen; kein Graphneubau. Minimaler zusätzlicher Handle-Eingang derselben Parser, keine Produktions-Cargo-Änderung. Gehaltene Dateien für lose Texte, VPK-Verzeichniscontainer und nummerierte Begleitarchive, exakt übergebene Inventarpfade und physische Herkunft; Ressourcenhash nicht mit physischem Containerhash gleichsetzen. Öffentliche Datentypen/Signatur, File-Eigentum/Lebensdauer/Cursor, erwartete Größe/Hashbindung und Fehler vor belegter Ausgabe konkret genug für C3-Caller. C3 verantwortet vollständig geprüfte private unveränderliche Sicherung; B darf nicht über Originalnamen erneut öffnen. Unbekannte/fehlende Archive und unpassende Bindung nicht still mit Manifestangabe ausgeben. Untere Budget-/VPK-Bounds/Fallible-Reserve unverändert wiederverwenden.

Für spätere Umsetzung passende vorhandene Wechsel-, Begleitarchiv-, Trunkierungs-/Pfadfälle und neuen Eingang prüfen; Format-/Clippy-/Compiler- und bestehende volle Quellenläufe im vorhandenen Prüfharness bewahren. Nie synthetische Fälle als echte Depotabnahme melden. Coverage: JSON/KV1/Text-KV3 generische Werte; sonstiger erlaubter Text nur Rohtext; binäre/kompilierte Assets zunächst Inventar, keine Spiellogikaussagen. Leserplan ist bereits vorbereitet. Kein neuer Prüfwrapper allein für Phase1.

## Routing

Eigener nativer coder im vorhandenen Harness, ausschließlich geerbtes GPT6.1Solhigh, kein Modelloverride/andereModelle/xhigh/max. Gewöhnlicher Worker, keine Unterdelegation. An Eltern B2 8e61edb7-3a14-4274-84bf-569122a7241c zurückmelden. Paketb/Versuch2, alleiniger Statusproduzentteil-b. Eltern schreibt Register/Status/kurze Signaturübergabe an Root, Root vermittelt an C3. Keine Sessionkoordination. B2 meldet Codefortschritt/Blocker, C3 alleine Gesamtintegration/Gate/Deploy/Live/Cleanup. Hook-Forderung nach ganzem22CommitBranchmerge betrifft fremde Basis und ist nicht dieser Auftrag; keine Hookänderung/-umgehung.
