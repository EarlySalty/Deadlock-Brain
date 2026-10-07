status: blockiert
Datum: 2026-10-03

# C3 an Root: tatsächlicher Versionsvertrag

ABWEICHUNG: FEATURE-VERTRAG-FIX6.md und Punkt47 nennen ein bereits vorhandenes serde_json1.0.151. Der produktive C-Worktree pinnt tatsächlich1.0.150. Parent hat den gesamten serde_json-Paketblock des aktuellen Cargo.lock gegen Basis511a347 geprüft: byteidentisch, Version1.0.150, Checksumme e8014e44b4736ed0538adeecded0fce2a272f22dc9578a7eb6b2d9993c74cfb9. Keine eigenmächtige Versionsänderung.

Der CLI-Worker hat ausschließlich arbitrary_precision im bestehenden Workspaceeintrag ergänzt. Keine serde-Lockänderung, kein pauschales Upgrade. Die anderweitig neu erforderlichen bereits gecachten rustix/sha1/hex-Abhängigkeiten sind getrennt im Sources-Paket registriert.

Urteil: Die numerische Zielwirkung und das Upgradeverbot sind klar, die behauptete vorhandene Versionsgleichheit ist sachlich falsch. Empfehlung an Root: den tatsächlich vorhandenen Pin1.0.150 beibehalten und denselben erhaltenden Featuregraph am echten C-Import-/PG-/Readerpfad nachweisen. As enger1.0.151-Beleg ersetzt diesen Consumerbeweis nicht. Falls Root1.0.151 zwingend verlangt, braucht es ausdrücklich die begrenzte Abweichung vom Upgrade-/Lockwechselverbot; C ändert die Version nicht still.

Gesamte eigene C-Quellenphase ist beendet und eingefroren, noch keine gemeinsame Prüfung. B-Handle-Zusatzcommit steht ebenfalls aus. Keine Doppelwriter, neue Rustprüfung allein wegen dieser Meldung, produktiven Importe oder Deploys. Root vermittelt die konkrete Vertragsentscheidung; C3 stellt dem Nutzer keine zusätzliche Routinefrage.
