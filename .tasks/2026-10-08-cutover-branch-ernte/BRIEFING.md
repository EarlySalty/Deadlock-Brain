# Auftrag: Altbranch feat/brain-rust-cutover-20260919 prüfen, Brauchbares fertig machen, Rest archivieren

Rolle: Blatt-Worker (GPT 6.1 Sol). Keine weiteren T3-Threads; native Subagenten nur für Recherche oder als frische Fixer bei Gate-BLOCK.
Auftraggeber: Claude-Session `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Nutzerentscheidung 08.10.2026: „Wenn das brauchbar ist, nehmen und fertig machen, sonst halt nicht.“

## Ausgangslage

- Branch `feat/brain-rust-cutover-20260919`: 21 Commits (18.09. bis 24.09.), rund 9.800 Zeilen, nicht auf main. Inhalt: unabhängige Patch-Prüfentwürfe mit dauerhaftem Belegverlauf, kanonische Patch-Identität (`item_or_ability`, `first_observed_at`, Caption-Hash), Release-Härtung (R1 bis R6, revisionssichere Patch-Sync, Verlaufs-MCP-Tool), Rust-Planpaket v1.0 unter `architecture/migration`.
- Der Haupt-Checkout `/home/nathanael/repos/Deadlock-Brain` steht auf diesem Branch mit rund 20 nicht committeten Dateien einer früheren Sitzung.
- Parallel laufen im Auftrag `.tasks/2026-10-07-brain-fertigstellung-astra/` die Pakete I (API-Spiegel live, F-Builds), G (Rechenschicht, Werkzeuge) und K (Bot-Anbindung). Deren Dateien und Branches nicht anfassen.

## Auftrag

1. **Sichern zuerst:** Die nicht committeten Dateien im Haupt-Checkout prüfen (nur ansehen, keine Secrets ausgeben) und als `wip:`-Commit auf `feat/brain-rust-cutover-20260919` sichern und auf diesen Branch pushen. Danach Archiv-Tag `archiv/brain-rust-cutover-20260919` auf diesen Stand setzen und pushen. Kein Reset, kein Stash, kein Force-Push.
2. **Brauchbarkeit beurteilen** gegen den aktuellen origin/main (Graphify zuerst, Skill `code-suche`): je Commit bzw. Thema einordnen in (a) schon auf main oder durch Brain v2 ersetzt (API-Spiegel, Patch-Import wie auf main, G-Werkzeuge), (b) noch fehlend und nützlich, (c) veraltet oder widerspricht heutigen Entscheidungen (keine Matchdaten, API-Spiegel statt Eigenbau, ein Antwortweg). Ergebnis als Tabelle in `BEURTEILUNG.md` hier.
3. **Nur (b) fertig machen:** Den Eigenanteil per Cherry-pick bzw. sauber neu auf einem eigenen Branch `feat/brain-cutover-ernte-20261008` im Worktree `~/.worktrees/brain-cutover-ernte` aufsetzen, nicht den alten Branch mergen. Keine Überschneidung mit I/G/K-Dateien; bei Überschneidung nicht bauen, sondern in `BEURTEILUNG.md` als Übergabe an das zuständige Paket notieren. Tests über `cargo-slot`, Gate `gate_hook.py --review` bis ALLOW (je BLOCK frischer nativer Fixer), Merge nach main regulär über HEAD:main als Einzelschritt, Deploy über `/usr/local/libexec/brain-release` falls Laufzeitcode betroffen, Live-Beweis.
4. **Aufräumen:** Haupt-Checkout zurück auf `main` (sauber, nichts verloren). Alten Branch lokal und remote erst löschen, wenn der Archiv-Tag gepusht ist und die Tag-Prüfung mit Exit-Code belegt ist. SHA-Backup in `BEURTEILUNG.md`.
5. Ist nichts unter (b): nur Schritt 1, 2 und 4.

## Grenzen

Rust only, Postgres only, keine Code-Kommentare, keine ENV-Konfiguration, Secrets nie im Klartext, keine Matchdaten speichern. Deutsch ohne Em-Dashes, echte Umlaute.

## Bericht

`AN_HAUPT.md` hier: Archiv-Tag und SHA, Beurteilung kurz, was übernommen wurde (Commits, Tests, Gate wörtlich, Deploy), was verworfen wurde und warum. Pflichtzeilen `MERGEPROTOKOLL[MS-1]`, bei Deploy `LIVEBEWEIS[DV-1]`. Fertig heißt gemergt bzw. archiviert, deployt falls nötig, aufgeräumt; dann `python3 ~/Documents/tools/t3-thread.py settle --selbst`.
