status: aktiv
Datum: 2026-09-29

# R-AC Runde 3: A34 auf bekanntem kombinierten Prüfstand

## A12-Nachtrag

A12 ist auf 8a865d3 sauber abgegeben und gepusht (21 Tests einschließlich Scratch-PG grün). Der Orchestrator hat A12 in den abgegebenen A34-Branch gemergt und PR59 auf c424566dcac4d050fb2426f352703fb2b2a42dea gepusht. Diesen kombinierten A-Head lokal im Reviewbaum aufnehmen und A1 gegen den vollständigen effektiven Upstream-Request erneut prüfen. Die Quelle bleibt derselbe Pin, keine realen Matchabfragen. Neuer C1-Head noch ausstehend. A4-Gegenprobe zu langsamer Snapshot-Abfrage als konkreten Befund dokumentieren, nicht wegen anderer grüner Fälle freigeben.

Astra als unabhängiger Reviewer, selber Thread 52c34332, eigener bestehender Reviewworktree /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929. Intent 562a877b-0939-440a-964d-1145d9e9431a. Keine Unterthreads, Produktfixes oder Kommentare im Code. AUFTRAG.md und Produktionsgrenzen bleiben. Graphify zuerst, keine echten API-Spielerabfragen oder Provideraufrufe.

Auf deinem sauberen Stand 63040ad A34-Head fded2ba lokal zum Review mergen erlaubt. Kein Merge nach migration/rust-integration oder main. A12 und C1 bearbeiten noch gezielte Nachträge in ihren getrennten Worktrees; deren neue Heads werden nach Abgabe übermittelt. Aktuell keine Änderungen aus deren unfertigen Bäumen lesen oder übernehmen.

Bekannte Befunde: R-AC-A1 weiter offen, A2/C1 in REVIEW-AC-R2.md geschlossen. Jetzt R-AC-A3/A4 gegen den A34-Diff 34a2507..fded2ba nachprüfen. Autor Sol, Bericht A34-REPORT.md mit 174 Tests und 12 ignorierten. Prüfe tatsächliche Verdrahtung im normalen ApiService/Kernel, autorisierte Scopes und keinen Provider-Fallback, Meta-/Population-Semantik gegen den gepinnten Upstream 290cedba6cca7d8a07015e9feefecd22de27643c, genaue wirksame Zeitfenster und Herkunft, Patch-Anfragen fail closed statt erfundener Patchmitgliedschaft. Echte positive und negative lokale Loopback-Gegenproben, nicht bloß Autor-Fixtures bestätigen. Absolute Deadline muss Slot-Wartezeit, beide Quellen und Retry-Restzeit umfassen. Cache darf weder Principal/Scope noch Release/Patchgrenzen verletzen. Keine Neuauflistung unbeteiligter Altpfade.

Die neue Population verwendet hero-stats einmal ohne und einmal mit include_item_ids statt build-item-stats. API-Defaults/Parameter und row/count-Semantik genau prüfen. Der Autor dokumentiert als NIT verlorene Usage bei fehlgeschlagenem Retrieval; unterscheiden, ob nur Anzeige oder Budget-/Sicherheitswirkung.

Nachweise und vollständige Mängelliste nur für dieses Delta in REVIEW-AC-R3.md schreiben, fixe SHAs und eigene Tests mit Exitcodes. Geprüfte Berichte auf eigenem Branch committen/pushen. Finales Gesamt-GO erst nach neuen A12/C1-Heads und deren gezielter Nachprüfung; keine alten Lastwerte als finale Integration ausgeben. Bei Blockern weiter konkrete Korrekturen mit Fundstelle statt pauschalem Urteil. Keine Produktion, keine Secrets, keine neuen Dependencies/Modelle/Budgets, keine echten Replays oder Nachrichten.