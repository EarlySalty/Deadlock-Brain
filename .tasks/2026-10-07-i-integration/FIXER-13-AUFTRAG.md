# Fixer 13: vorhandene Patchidentität bei URL-Varianten erhalten

Frischer nativer Kontext nach tatsächlichem gemeinsamen Produkt-BLOCK in Runde 18. Derselbe offene I-Auftrag und Entscheidung `ENTSCHEIDUNG-I-PATCH-ORIGINAL.md` vom 07.10.2026, 16:09 UTC. Vorhandene Eigentums-, Herkunfts-, Netz- und Prüfgrenzen gelten unverändert.

## Arbeitsbaum und Ausgang

Ausschließlich `/home/nathanael/.worktrees/brain-e-deadlock-api`, vorhandener Branch `feat/brain-deadlock-api-daten`. Bestätigter HEAD `d3d3c5b68bea07a3e11c09572b200bbefd7b55cf`, Produktfix b70dc6b6 und Core6-Gegenprobe unverändert enthalten. Hauptsession-Artefakte `PRUEFUNG-KANDIDAT-501.md`, `REVIEW-RUNDE-18.md` und dieses Briefing sind untracked und dürfen weder geändert noch gestaged werden. Kein neuer Arbeitsbaum, Thread oder weitere Subagent. Gemeinsamen Kandidaten 501d3725 und frühere Stände erhalten.

## Fixziel

Originalurteil `/tmp/brain-i-e-patch-candidate-gate-opus55.log`, vollständige Rückgabe und nachgelesener Kern in `REVIEW-RUNDE-18.md`. Einziger BLOCKING-Kern: Der vorhandene Lookup in api_sync.rs:143-153 vergleicht fragmentfreie Anfrage-URL und Feedlink; `post_row` entfernt danach die Query. Eine bestehende queryfreie URL wird bei `?l=english#notes` ohne brain.source_documents nicht gefunden und erhält eine neue negative ID. Der vorhandene Query-Test legt derzeit die queryhaltige Anfrage-URL als Bestand an und deckt diesen Fall nicht ab.

1. Den tatsächlichen vorhandenen Lookup-/Post-ID-Weg konsistent korrigieren, nicht nur Testfixture oder Datenbank per Hand anpassen. Bestand zuerst Graphify/code-suche, dann aktuelle Quelle und Legacyaufrufer lesen. Query-/Fragment-/Slashvarianten derselben konkreten Patchidentität müssen bestehende IDs erhalten; tatsächliche neue Patches behalten eine stabile, getrennte ID.
2. Akzeptierte Steam-Store-/Community-URL-Varianten nur dann zusammenführen, wenn dieselbe konkrete Ereignisidentität nach vorhandenem Vertrag nachweislich gemeint ist. Ereignis-GID und Announcement-GID sind nicht automatisch identisch. Keine Titel-/Zeitgleichheit oder fremde GID als Identitätsbeleg, keine Lockerung der Originalbindung. Positives tatsächliches Gleichheitsbeispiel plus negative fremde Ereignis-/Announcementfälle abdecken. Vorhandene Bindung/Parserstrecke wiederverwenden, keine zweite Pipeline oder neue Persistenz.
3. Vorhandene echte Lookup-Scratchprobe erweitern und ausdrücklich mit eigener isolierter PostgreSQL-Instanz ausführen. Insbesondere queryfreier Bestand gegen Query+Fragment, Slashvarianten in beiden Richtungen und gleiches Ereignis über tatsächlich akzeptierte URLs. Fremde Ereignisse dürfen nicht die alte ID übernehmen. Nur eigene Unixsocket-Instanz ohne TCP; `DEADLOCK_BRAIN_SCRATCH_DSN` ist bereits vorhandener Testeingang, keine produktive ENV-Konfiguration hinzufügen. Test verlangt Datenbank `brain_fixer12_patch_lookup`, kein brain.source_documents. Keine Produktiv-DSNs oder schreibendes psql, keine fremde Scratchinstanz benutzen.

Die zwei NITs zu unerreichbarem altem Original und Bildlinks sind offen, nicht beauftragt; keine pauschale Fehlerunterdrückung und kein Scope-Zuwachs. Die bisher korrigierte konkrete GID-Bindung, Fragmentabrufe, originale Herkunft und HTTP-Guards bleiben erhalten. Keine G/K-Dateien oder deren verweigerte Prüf-/Implementierungsarbeit übernehmen.

## Prüfung, Selbst-Gate und Rückgabe

Passende bestehende Format-/Compiler-/Clippy-/Testprüfungen am eigenen Stand, vorhandene Cargo-Slots, `env -u DEADLOCK_CENTRAL_DSN -u DATABASE_URL SQLX_OFFLINE=true`, `-j 2`, Tests seriell. Format nur eigene Dateien mit rustfmt, cargo fmt ausschließlich --check. Schutzablehnung nie über anderen Worker, Toolweg, Wrapper oder Hookänderung umgehen. Kein globaler Targetwechsel oder Cachelöschen. Keine neuen produktiven Kommentare, Connectoren, Modelle, Timeouts oder Matchablagen.

Nur eigene verifizierte Quelldateien und eigener neuer Fixerbericht gezielt stagen. Ein Git-Schritt pro Bash, literale absolute Pfade. Committrailer `Co-authored-by: GPT 6.1 Sol <gpt-6.1-sol@local>`. Selbstprüfung nach Commit mit `/home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-e-deadlock-api --base d3d3c5b68bea07a3e11c09572b200bbefd7b55cf --head <fix-sha> --model claude-opus-5-5`. Derselbe Reviewer, kein Modellwechsel, Override oder neu gewürfelter unveränderter Diff. Bei BLOCK Urteil und echten Restkern zurückgeben, keine zweite Fixrunde im eigenen Kontext. Bei ALLOW nur Featurebranch sichern und konkreten SHA, Testzahlen/Exitcodes sowie vollständiges Urteil zurückgeben. Die Hauptsession prüft anschließend wieder den tatsächlichen kombinierten Produktkandidaten gegen main.

Kein Main-Merge/Push, Deploy, Restart, Cleanup oder Self-Settle. Keine Sessionkontakte. Secrets NEVER ausgeben; Nutzer-/Community-Daten MUST NOT an externe Anbieter oder externe Codiermodelle, auch nicht über Loopbackproxy. Keine Browserarbeit erforderlich; falls erforderlich zuerst agent-browser.md lesen, nur Moli, MUST NOT Brave oder persönliche Browser. Sonnet/Fable verboten.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-e-deadlock-api
