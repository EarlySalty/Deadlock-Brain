status: aktiv
Datum: 2026-10-03

# Z: Release-Gate, Runde 1

Geprüfter Commit: `b86353acca8027431de58a2edadbebcdd01bad10`. Basis `main`. Zentraler Gate, Task `bnxwvri2e`, Exit 1. Kein Merge oder Deployment nach diesem Urteil.

Wörtliches Urteil:

> [gpt-6.1-sol] BLOCK: Artifact provenance is unenforced, and interrupted installs can become unrecoverable.

## Offene Funde

1. `ops/brain-release/src/source.rs:344`, blockierend: Binaryhashes und Binärdateien kommen aus derselben betreiberveränderlichen Quelle. Ausgetauschte Binärdateien mit passend veränderten Manifesthashes werden akzeptiert, ohne deren Bytes an den geprüften Quellstand zu binden. Beide Installationsprüfungen, Staging und installierte Prüfungen übernehmen diesen fehlenden Herkunftsbeweis. Der Fix muss eine unabhängige, tatsächlich durchgesetzte Verbindung zwischen geprüftem Sourcebuild und installierten Bytes herstellen. Ein weiteres selbst erklärtes Hashfeld reicht nicht.
2. `ops/brain-release/src/install.rs:158`, blockierend: Ein Abbruch zwischen den beiden Layoutveröffentlichungen hinterlässt einen Zustand, den weder Retry noch Pointer-Recovery reparieren. Direkt zum endgültigen Namen geschriebenes Journal kann nach Schreibabbruch ungültiges JSON enthalten und sowohl Installation als auch Recovery blockieren. Layoutveröffentlichung und Journalanlage müssen nach Abbruch eindeutig wiederaufnehmbar oder rückführbar sein, ohne vorhandene Releases zu entfernen.
3. `ops/brain-release/src/main.rs:157`, Hinweis: Cargoartefakte können mehrere Hardlinks haben, während `regular` sie pauschal ablehnt. Tatsächliche Messung am vorhandenen Debugartefakt am 03.10.2026 nach 18:26 UTC: `nlink=2`. Die Produktionsbauausgabe ist gesondert zu prüfen; einlinkige ELF-Fixtures sind dafür kein Beweis. Sicherheitsprüfungen nicht pauschal abschwächen.

## Zusätzlicher Abnahmebefund

Intent-Abnahme `wf_32c81a69-610`: fertig N, Fix J wegen Formatierung und Prüfnachweis. Die frühere Sammelkette `bnzej9wq0` endete zwar mit Exit 0, ihre Formatdatei enthält aber einen Diff in `source.rs`. Acht bestandene Tests, Clippy und Debugbau auf dem damaligen Stand ersetzen keinen grünen Formatlauf. Jeder Schritt ist künftig mit eigenem tatsächlichem Exitcode nachzuweisen. Spätere Quellen und das inzwischen veränderte Debugartefakt sind nicht durch den alten Lauf gedeckt.

## Schreibzuständigkeit und Übergabe

Der ursprüngliche Releaseworkflow war beendet. Seine durch eine frühere native Wiederaufnahme entstandene zusätzliche Fortsetzung wurde von Z ausdrücklich beendet, damit ein frischer Fixer allein übernehmen kann. Keine fremde Session und kein fremder Prozess gestoppt. Alle uncommittierten Änderungen in `main.rs`, `install.rs` und `source.rs` bleiben erhalten. Der Kandidatenadapter und der Archivfortbau bleiben davon getrennt.

Frischer Fixer erhält diese Liste und den vorhandenen Stand, nicht der ursprüngliche Implementierer. Noch keine Runde 2 und kein ALLOW. Folgeprüfung weiterhin über denselben zentralen Gatepfad mit dem in Runde 1 urteilenden Modell, kein Neuwürfeln bei einem BLOCK.
