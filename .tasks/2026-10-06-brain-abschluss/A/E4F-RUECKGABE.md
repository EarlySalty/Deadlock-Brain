# E4f: zwei Funde behoben, neuer Gate-BLOCK

Sauberer Featurecommit `2be2df16d7a9390823a05691bef1ae69f1b8c8c1`, Baum `af971c311ff19b4e8495aa079869c68407380c52`, bestehender eigener Worktree `/home/nathanael/.worktrees/brain-a-invite-bots-20261006`. A hat Status, SHA und Gate direkt gelesen. Kein Push, Main-Merge oder Deploy.

Vorherige Funde laut Fixer behoben: neuere Requestzustände zeitlich gegen historische Beobachtungen ausgewertet; Abschlusszweige an konkrete Reservierung gebunden. Aktueller regulärer Selbstgate gegen e18f522226f8e2dec5a1c03fe97c2aba3200c8d1, Exit 1:

```text
[gpt-6.1-sol] BLOCK: Arbitrary questions can bypass backend rate limits.
```

Fund `dl-brain/src/lib.rs:149`, Zwillinge `reserve_question:59` und `handle_brain_query:233`. Direkt im Code nachvollzogen: Substring „invite status“ in einer beliebigen Frage reicht für die Cooldown-Ausnahme. Beide Wege rufen trotzdem den vollständigen Answerer auf (:198/:247). Nachrichten haben noch Kanal-/Tageslimits, der Kommandoweg keine Ersatzbegrenzung oder gleichwertige laufende Reservierung. Ausnahme muss auf eine begrenzte eigene Statusoperation wirken. Keine eigene Bot-Antwortengine oder bloß strengerer Substring als vollständige Lösung.

G1-Priorität bleibt verbindlich. E4g ist als nächste frische Fixrunde vorgesehen, noch nicht gestartet; kein alter Implementierer wird neu für den Fund verwendet. Aktivierung/Release auf bfda408c bleibt ausschließlich bei live_strecke und enthält dieses blockierte Feature nicht.

## Tatsächliche Prüfgrenzen

A las die Resultatzeilen direkt: Feature Bot 327 passed/7 failed und Brain 15/0; frische Baseline Bot 319/10, Brain 12/0. Übernommene frühere Brain-Baselinezahl 13 gilt damit nicht als nachgewiesen. `tests-baseline-private.log` enthält zusätzlich ein vermischtes Brainsegment mit 15; dafür ausschließlich `tests-baseline-brain-final.log` verwenden. Alle sieben aktuellen Botfehler laut Fixer bereits in der Baseline, keine grüne Vollsuite behauptet.

Rust 1.97.1, private echte PG und `--include-ignored --test-threads=1`; 0 ignored/filtered. Format, Compiler, Clippy `--all-targets --no-deps -- -D warnings` laut Fixer grün. Vorläufe mit falscher Testrolle/gemischten Targets verworfen. Private PG laut Fixer beendet, verbleibende Scratchdatenbanken 0. Keine neue A-Suite.

Belege `/tmp/brain-a-invite-bots-e4f-proof-20261007/`: `tests-feature-final.log`, Botsegment `tests-baseline-private.log`, `tests-baseline-brain-final.log`, `gate.log`. Gemeinsame E3f-/Consumer-/Liveabnahme weiterhin offen.

TESTNACHWEIS[TW-1]: 342 passed, 0 ignored | Baseline: 10 rot
Die folgende Git-Anzahl stammt aus der Fixerrückgabe, nicht aus einer neuen vollständigen A-Transcriptzählung.

MERGEPROTOKOLL[MS-1]: 11 Git-Schritte einzeln | Anläufe: 1 | Gate: gpt-6.1-sol BLOCK, Exit 1; kein Main-Merge
