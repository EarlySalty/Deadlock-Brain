# Paket E: offene Übergabe

Code und Schnittstelle zu G sind im Featurebranch umgesetzt und lokal verifiziert. Kein Main-, Gate- oder Liveabschluss.

1. **Gate-Werkzeugpfad wieder funktionsfähig machen.** Drei Aufrufe lieferten Exit 2 statt Urteil: `bwrap`/`unshare` können keinen Namespace anlegen (`Cannot allocate memory`). Betroffen sind sowohl der Originalschema-Pin als auch der getrennte kleine API-Diff. Erster Lauf zusätzlich Grok HTTP 402. Kein verlässlich ableitbarer Reset-Zeitpunkt. Schutzmechanik nicht abschalten, keine Ersatzreviewer oder Modellwechsel. Nach Reparatur beide Grenzen regulär nachprüfen:

   ```bash
   python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-e-deadlock-api --base bfda408cb988722ddceadb56bca5b72e12d12731 --head 5e70da3a6f40c0f1eedc78565641d8dfce582f56
   python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-e-deadlock-api --base 5e70da3a6f40c0f1eedc78565641d8dfce582f56 --head e65efae2c7c53dd4d17d75f51974753d6fd84d08
   ```

2. **Breiten Dependency-Lint einordnen.** `dbrain-enrich/src/lib.rs:519,651,1291,1442`: vier `map_or_identity`-Diagnosen mit aktueller Clippy-Version. Kein gemessener Altfehlerbeleg. Die fünf E-Prüfpakete bestehen `--all-targets --no-deps -- -D warnings`. Nur nach Dateiabgrenzung die vier identischen Umformungen zu `unwrap_or(...)` durch den zuständigen Worker nachziehen, nicht E-Dateien oder Lint-Gates verbiegen.
3. **Integration erst nach ALLOW und aufgehobenem Release-Hold.** Den Schema-Pin und anschließend den API-Commit auf dem freigegebenen aktuellen Main integrieren. Konflikte im schmalen Ingestbereich von `deadlock-brain/src/main.rs` mit A abgleichen. Kein Fremd-Gesamtmerge. Main-Push nur regulär über den Hook. G liest `brain_storage::asset_mirror`; keine eigenen Rohdatenkopien. Jüngster vollständiger Spiegel ist nicht automatisch aktive Balancepatchversion, siehe `AN_HAUPT-E.md`.
4. **Betriebsbeweis ausschließlich durch `live_strecke`.** Geprüften Main bauen und installieren, vorhandenen täglich um 03:30 Europe/Berlin laufenden Builddaten-Wrapper übernehmen, Assets-/Patch-/Builddatenlauf prüfen. Für zeitnahe Versionswechsel den bereits vorhandenen 5-Minuten-Patch-Timer über seinen bestehenden Wrapper vor der Patch-Discovery auch `pull assets` ausführen lassen, ohne neuen Dienst oder zweite Pipeline; bei unveränderter Version ist das ein lokaler Wiederverwendungscheck nach Manifestabruf. Diese Betriebsverdrahtung wurde durch E nicht geändert. Drei Arten in beiden Sprachen, Manifestversion, vollständige Runbindung und fehlgeschlagene Läufe prüfen. Unaufgelöste große Patchvorschauen sichtbar lassen. Erst anschließend Antworten über G/A und `/v1/answer` prüfen. Keine Einzelmatches importieren. Alte Parser und Matchablagen erst nach belegtem Gleichstand gesondert abbauen.

Worktree und Branch bleiben erhalten. Keine Installation, kein produktiver Tick, kein Main-Push und kein Settle durch E. Quellen und reproduzierbare Befehle: `E/NACHWEISE.md`.
