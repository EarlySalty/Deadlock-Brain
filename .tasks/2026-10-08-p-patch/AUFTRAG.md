# P: Patch-Erkennung erhalten und liefern

Auftraggeber: 481426fe-b477-42b3-91c6-901811fcba1d. Entscheidung: ENTSCHEIDUNG-TEMPO-0200.md, 8. Oktober 2026.

Bestehenden Import und Discoverystand erhalten. Originalbeitrag eindeutig binden, Metal Skin nicht als Kosmetik verwerfen, Feedauszüge nicht als Volltext importieren und kompakte Forumtexte erhalten. Keine zweite Pipeline, keine Migration oder privaten Daten.

Eigentum: pg_patchnotes.rs, pg_patchnotes/api_sync.rs, unmittelbare vorhandene Tests. CLI und vorhandener Timer ausschließlich als minimaler Patchanteil. Keine Änderungen an I/F/G/K-Arbeitsbäumen, keine weiteren T3-Threads.

Start: eigener Worktree /home/nathanael/.worktrees/brain-p-patch-20261008, sauberer Branch feat/brain-patch-discovery, HEAD af4736089cc5ce5d41ed442d445c49a30d5c6375. Aktuelle Mainbasis wird separat geprüft. Der Altstand enthält viele fremde Deltas und wird nicht pauschal gemergt. Nur zusammengehörige Patchdateien werden in einen eigenen Lieferbranch auf aktuellem Main übernommen. Archivbranch bleibt erhalten.

Prüfung: cargo-slot, Format, striktes passendes Clippy, vorhandene Tests, regulärer gate_hook.py --review. Tatsächlicher BLOCK führt zu frischem nativem Fixer mit demselben Urteilmodell. Abschluss: Main-Hook, HEAD:main, regulärer Release, Neustart, Livebeweis, ausschließlich eigenes Cleanup.
