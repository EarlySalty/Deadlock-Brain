status: aktiv
Datum: 2026-10-01

# WIP modglue C9 Vertragsabgleich

## Umfang und exakte Gruppenheads

Alle untenstehenden PR-Heads waren auf GitHub gegen `main` geöffnet und wurden für diesen Abgleich exakt abgefragt. Das ist eine Vertragsprüfung, kein Gruppen-Gate.

| PR | Head-Branch | Head-SHA |
|---|---|---|
| Deadlock-Bots #450 | `feat/brain-image-input-20260924` | `a4ebd88ecaf5533a9166579db18498ccdb88dd66` |
| Deadlock-Bots #451 | `feat/brain-direct-build-publish-20260924` | `68589a1ba676a9ce329539a6a4e884aee0677419` |
| Deadlock-Bots #459 | `codex/fix-c9-consumer-wiring` | `46edc1023a819ba0ba3c37b7de96c333f485f797` |
| Deadlock-Brain #61 | `codex/brain-functional-closeout-20260930` | `b687f613b3df2c49138d9d2837e005c33e646d9f` |
| Deadlock-2nd-Brain #2 | `codex/fix-c9-consumer-wiring` | `ab83b691befd761a16d971af5c249a604c5d4e0d` |
| Deadlock-Docs #4 | `codex/fix-c9-consumer-wiring` | `ff20af7a8e3fcacc349cb2d97eda34dfa0897957` |

WIP-Source `6a274ac348e50acc1e57b864b4c15a318c87448a`, Parent `de34acc80dc3c1723397021593576ca94b20558d`, `origin/main` `5ed4cf7d6132d750cf024d6abccf90ebaac4a7ef`. Die Änderung an `rust/bin/dl-bot/src/modglue.rs` besteht ausschließlich aus rustfmt-Formatierung.

## Entscheidung je WIP-Hunk

| Hunk | Ergebnis an aktuellen Heads | Beleg |
|---|---|---|
| `@@ -91,4 +91` | Bereits enthalten | Alle drei Bots-Heads verwenden die einzeilige `entries.push`-Form in `modglue.rs:91`. |
| `@@ -320 +317,2` | Formatierung enthalten, alter Kontext in #451 überholt | #450/#459 haben die getrennte `format!`-Argumentformatierung. #451 hat die Formatierung ebenfalls, aber eine neuere Status- und Review-Ausgabe. |
| `@@ -3983 +3981,4` | Bereits enthalten | Die mehrzeilige Emoji-Tupelform steht in allen drei aktuellen BrainEmoji-Tests. |
| `@@ -3987,4 +3988` | Bereits enthalten | Die einzeilige `assert_eq!`-Form steht in allen drei aktuellen Tests. |
| `@@ -4005 +4003,3` | Alter Testkontext in #451 erweitert | Die mehrzeiligen Item-Literale sind in den aktuellen Heads vorhanden. #451s Receipt-Test enthält zusätzlich Status- und Review-Prüfungen, daher den alten Testhunk nicht als eigenständigen Delta übertragen. |
| `@@ -4008 +4008,3` | Alter Testkontext in #451 erweitert | Die mehrzeiligen Situations-Literale sind vorhanden; die Testsemantik in #451 ist erweitert. |
| `@@ -4023,6 +4025,3` | Bereits enthalten | Die kompakte `brain_answer_embed_body`-Initialisierung steht in allen drei Heads. |
| `@@ -4072,6 +4071,2` | Bereits enthalten | Der kompakte Newline-Testaufruf steht in allen drei Heads. |

Ergebnis: kein WIP-Hunk ist als Source-Delta nötig. Keine Hunkübernahme und kein Sicherungscommit-Merge.

## Vertragsbeleg

- Brain #61 `rust/crates/deadlock-brain/src/main.rs:1794-1799` bestätigt eine Build-ID nur bei Status `DONE` und positivem ID-Wert. `requested_build_receipt_requires_real_completion` prüft die negativen Status und fehlende, nullwertige sowie negative IDs. Die CLI gibt `status`, `task_id` und nur die bestätigte `hero_build_id` aus.
- Bots #451 `rust/crates/dl-brain/src/build_request.rs:29-36` setzt dieselbe Regel in `confirmed_build_id` um. `modglue.rs:441-449` prüft zulässige Status, positive Task-ID und `DONE` mit bestätigter Build-ID. Tests `modglue.rs:4396-4410` verhindern eine Erfolgsanzeige bei `PENDING`, `RUNNING`, `FAILED`, `CANCELLED`, `BLOCKED` oder Build-ID `0`.
- Bots #450 und #459 haben an `modglue.rs:280-287` noch den älteren Formatter, der eine vorhandene `hero_build_id` direkt als veröffentlicht darstellt. Ihre Aufrufpfade behandeln `FAILED` und `CANCELLED`, setzen aber die `DONE`- und Positiv-ID-Regel nicht selbst durch. Der Brain-#61-Produzent verhindert dies für seinen aktuellen kanonischen CLI-Output. #451 enthält die stärkere Consumer-Sicherung. Das ist eine semantische Divergenz zwischen PR-Heads, keine Wirkung des formatierenden WIP-Diffs. Beim Gruppenabgleich die #451-Sicherung erhalten.
- 2nd-Brain #2 und Docs #4 ändern die `tools/brain-adapter`-Dateien, nicht `modglue.rs` oder den Receipt-/Embed-Code. Sie besitzen für diese acht Hunkstellen kein Source-Delta.
- Die gezielte Suche deckte nur diese acht `modglue.rs`-Hunks ab. PR #472/ClipGlue, TempVoice, Scam, Scrim, Patchnotes, unzugeordnete Hunks und Discord-Exportdateien blieben außerhalb des Scopes. CSV-Inhalte wurden nicht gelesen.

Keine Compiler, Tests, Gates, Sourceübernahmen, Produktionsaktionen oder Änderungen am Bots-WIP-Worktree ausgeführt. Kein Gruppen-PASS abgeleitet.
