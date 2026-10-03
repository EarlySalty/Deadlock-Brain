status: aktiv
Datum: 2026-10-03

# C3 an Root: tatsächlicher Versionsvertrag

## Entschieden durch Punkt 51

Root hat den sachlichen Unterschied bestätigt: Der produktive Pin 1.0.150 bleibt unverändert, arbitrary_precision und bisherige Features werden gemeinsam verwendet. Kein Upgrade zur A-Harnessversion 1.0.151 und keine Sourcefixrunde allein wegen dieser Differenz. Der ursprüngliche Versionsblocker ist aufgehoben; der tatsächliche C-Import-/PG-/Readerbeweis bleibt offen. Der bestehende gemeinsame Prüfer biuhayf7n arbeitet ausdrücklich mit 1.0.150, nicht mit einem behaupteten 1.0.151-Graph.

## Ursprünglicher bestätigter Unterschied

ABWEICHUNG: FEATURE-VERTRAG-FIX6.md und Punkt47 nennen ein bereits vorhandenes serde_json1.0.151. Der produktive C-Worktree pinnt tatsächlich1.0.150. Parent hat den gesamten serde_json-Paketblock des aktuellen Cargo.lock gegen Basis511a347 geprüft: byteidentisch, Version1.0.150, Checksumme e8014e44b4736ed0538adeecded0fce2a272f22dc9578a7eb6b2d9993c74cfb9. Keine eigenmächtige Versionsänderung.

Der CLI-Worker hat ausschließlich arbitrary_precision im bestehenden Workspaceeintrag ergänzt. Keine serde-Lockänderung, kein pauschales Upgrade. Die anderweitig neu erforderlichen bereits gecachten rustix/sha1/hex-Abhängigkeiten sind getrennt im Sources-Paket registriert.

Urteil: Die numerische Zielwirkung und das Upgradeverbot sind klar, die behauptete vorhandene Versionsgleichheit ist sachlich falsch. Empfehlung an Root: den tatsächlich vorhandenen Pin1.0.150 beibehalten und denselben erhaltenden Featuregraph am echten C-Import-/PG-/Readerpfad nachweisen. As enger1.0.151-Beleg ersetzt diesen Consumerbeweis nicht. Falls Root1.0.151 zwingend verlangt, braucht es ausdrücklich die begrenzte Abweichung vom Upgrade-/Lockwechselverbot; C ändert die Version nicht still.

Gemeinsamer Featurelauf biuhayf7n tatsächlich Exit 0: Runtime serde_json 1.0.150 mit arbitrary_precision/default/raw_value/std; separater Procmacro-Hostgraph ohne arbitrary_precision. Der folgende sechs-Pakete-Check endete am Reader-Typfehler mit Exit 101. Import, PG und Reader wurden noch nicht ausgeführt. B-Handle-Zusatz ist inzwischen allein als f7a03f9 integriert. Bestehende B-Zahlenstrings und i64-Numbers bleiben gemäß Punkt 52 erhalten; keine neue Typfixrunde oder Floatkonvertierung.
