status: aktiv
Datum: 2026-10-03

# B2 Punkt47: genaue Zahlenwerte lesend prüfen

[Orchestrator]
BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-b

## Ziel und Vertrag

Lies zentral AN_BEREICHE.md Punkt47 und bereiche/a/FEATURE-VERTRAG-FIX6.md. Prüfe vorhandenen B-JSON-Zahlenpfad und erhaltene zwei vollständige Git-Exports. Unterscheide originale Lexeme/Spans, mathematisch genaue Zahlenwertgleichheit und tatsächlichen Value-Typ. Keine automatische B-Parserfixrunde, kein unbelegter Fund aus A ableiten. json_text::number bewahrt Dezimal-/Exponent-/große Integerlexeme absichtlich als String, normale i64/u64 als Number; prüfe dieses reale Verhalten und Vertrag, klassifiziere Typabweichung separat von Wertverlust. Keine Epsilon-/Floatvergleiche oder Number-PartialEq als alleinigen Gleichheitsbeweis. Keine zweite Parserimplementierung.

## Eigentum

Produktive Quellen, bestehender Prüfharness und alte Outputs ausschließlich lesen. Keine Edits in game_files.rs/vpk.rs/json_text.rs oder weiteren Quellen, Cargo/Lock/lib.rs/CLI. Neuer schmaler lokaler Rust-Messharness sowie Beweise ausschließlich in eindeutigem neuen Unterordner unter /home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/b/, außerhalb Git. Kein Python, keine produktive neue Implementierung oder Parserkopie. Vorhandene Parser/serde_json verwenden. Keine Secrets/ENV-Konfiguration, DB, Downloads, Gitmutationen, fremde Prozesse/Locks/Sessions oder Unterdelegation. Eltern schreibt Akte/Status, eigener numerischer Bericht außerhalb Git erlaubt.

## Arbeitsstand

Worktree oben, Branchfeat/brain-wiki-spielwissen-b, HEAD48b6ce1cf277ec4de4b47ac4a0898fbc223fe6b5. Handle-Zusatz in genau zwei Dateien uncommittiert, komplette Selbstprüfung grün unter alten Rlibs; unabhängiger API-Prüfer a7884bae1d7c646da liest denselben Freeze, nicht verändern. Zahlenauftrag separat. Beweisroot /home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/b/round3-proof/run-CrJzHxm4, beide .jsonl vollständig erhalten; vorherige Basis run-5gzlJFjs und alte Ausgaben ebenfalls erhalten. Vorhandene Wrapper-Rlib libserde_json-e851daea22c6858d.rlib ist laut tatsächlichem Fingerprint default/raw_value/std, ohne arbitrary_precision. Gelesener Hauptcheckout rust/Cargo.lock meldet1.0.150, ist kein Nachweis für tatsächlich gelinkte Rlibversion. Version und Herkunft zuerst anhand wirklichem Rlib/fingerprint/.d-/Registry-/A-Harnessbeleg klären. Bestätigtes Ziel1.0.151 mit arbitrary_precision/default/raw_value/std, kein Versionsupgrade des produktiven Graphen durch B. Nicht bloß Cargo-Text als Laufzeitbeweis verwenden.

## Beweisziel und Messung

Vor Codeorientierung Graphify fragen; eigener Worktree hat keinen Graph, global /home/nathanael/.graphify/global-graph.json vorhanden. Danach konkrete Module/Validator/Outputstruktur lesen. Rust-Messung vorhandener Originalquellen und vollständiger alter Exportfakten, Originalzahlen lexemgenau erfassen mit bestehendem Parser bzw serde_json RawValue/arbitrary_precision. Exakte dezimale Normalisierung mittels Ziffern/Exponent zulässig für Messvergleich, kein Quellparserneubau. Originalcontent-Pointer/qualifiers/Originaldateibytes prüfen, Quelltyp und outputValue getrennt zählen. Tatsächliche alte Wertabweichungen sämtlich quantifizieren, Beispiele mit Dateipfad/json_pointer/Originallexem/gespeichertem Wert/Typ, Quelle/SHA, Umfang je Quelle und Familie. Nullfunde nur bei wirklich vollständiger Prüfung. Kanonisierung von -0/Exponentenschreibweise ist kein mathematischer Verlust, Originallexem separat.

Falls lokales Bauen nötig: vorhandene Hostmechanik blockierend in Reihenfolge FD8 /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock, FD9 /tmp/deadlock-cargo-release.lock, bis Prüfende halten. Frische NonZombieprobe unmittelbar vor jedem Compiler, höchstens2Jobs. Keine zweite volle Wrapperfolge, fremde Compiler/Locks nie stoppen/ersetzen. Eindeutiges eigenes Target außerhalb Git, keine geteilten target-Builds. Kein Skip-Flag/Gate mit anderem Modell. Vor Compilerstart echten Task/Parent/flockPIDs/Log/Startzeit melden. Stabile Wartetask nicht wegen20min beenden.

serde_json Ziel exakt1.0.151 und Features nach Vertrag, raw_value erhalten. Verfügbarkeit empirisch prüfen; keine Shared-Manifeste ändern. Die bestehenden Harnessdateien bleiben während API-Abnahme eingefroren. Enger Messharness darf separates Manifest/Lock außerhalb Git nutzen. Finale B-Harnessfeatureanpassung übernimmt Eltern erst nach stabiler API-Abnahme. Eigene erzeugte Daten niemals alte Outputs überschreiben. Source-/Artefakt-/Featurebindungen und Prozessdauer/Peak nennen, sofern gemessen, nichts erfinden.

## Routing

Eigener nativer coder geerbt GPT6.1Solhigh. ElternB2 8e61edb7-3a14-4274-84bf-569122a7241c, Paketb/Versuch2, Statusproduzentteil-b; Root über kurze eigene Übergabe. C3 besitzt gemeinsame Produktivfeatures/Caller/Gate/Deploy. Bericht mit Urteil tatsächlicher Werteverlust J/N, Typabweichung separat, genauer Umfang, begrenzter Vorschlag; keine eigenmächtige Fixrunde. Handle-Zusatz und Basis48b6ce1 unverändert erhalten. Finale gemeinsame D/B/C-/Steamabnahme offen. Natürliches Deutsch, echte Umlaute, keine Gedankenstriche.
