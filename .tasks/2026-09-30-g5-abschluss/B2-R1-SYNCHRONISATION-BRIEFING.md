status: aktiv, enger Testquellenrest aus Nachreview eceb14c
Datum: 2026-09-30

# B2-R1: wirkliche Sperrkonkurrenz nachweisen

Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder Unter-Agenten spawnen. Bestehenden Autor 66adf9ee-bc03-4ff3-91da-73cd8efc5e72 und dessen Modell beibehalten. Intent-Thread: 562a877b-0939-440a-964d-1145d9e9431a.

## Ausgangsstand und Auftrag

Eigener Worktree /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930, Branch fix/g5-replay-deferred-20260930. Letzte gepushte Abgabe 392267b133a9a5a8a91602247a441ff7bf609c4b enthält Produktfix e878530 und getrennten CI-Fix 0c56f85. Arbeitsbaum vor Dispatch sauber. Bestehende Arbeit erhalten, kein Reset, kein neuer Worktree oder Modellwechsel. Eigene Änderungen getrennt committen und auf denselben Featurebranch pushen; nicht nach main mergen, kein Deploy.

Unabhängiger Reviewer seit 14:42 fertig, beide Berichte auf eceb14c gepusht:
/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/.tasks/2026-09-30-g5-abschluss/B2-R1-R3-NACHREVIEW.md
/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/.tasks/2026-09-30-g5-abschluss/B1-R1-N1-NACHREVIEW.md

B1-CI hat GO. R2 und R3 sind statisch geschlossen; R1-Produktkorrektur ist statisch akzeptiert. Diese Teile nicht neu entwerfen. Nur den belegten R1-Testquellenrest nachziehen.

## Konkreter Befund

Reviewer eceb14c, B2-Bericht Zeilen 20 bis 32; Codebindung e878530, rust/crates/brain-legacy-import/src/bin/brain-legacy-import.rs:921-940 und 973-988:

> Im Tombstonefall wird der Writer mit tokio::spawn gestartet und mit competing_delete.await.unwrap() vollständig beendet. Erst danach werden Cutover-Lease und commit_batches_and_publish_checked angefordert. Im Scopefall passiert dasselbe mit competing_scope.await.unwrap().

Die Join-Awaits bei 928 und 976 beenden die vermeintliche Konkurrenz vor der Veröffentlichung. Auch ohne neue Quellsperren würden Checkpoint-CAS oder vollständiger Headvergleich die veraltete Baseline erkennen. Der geforderte Gegenbeweis für Änderungen nach erfolgreichem Vergleich und vor Commit fehlt.

## Enges Beweisziel

1. Bestehende Gegenbeweise deterministisch synchronisieren: Veröffentlichung hält nachweislich ihre Quellsperren beziehungsweise hat den geschützten Vergleichspunkt erreicht, während ein zweiter Vorgang einen Batch-Tombstone oder direkten Scope-Apply auf dieselbe Quelle versucht.
2. Beobachten, dass der konkurrierende Writer bis Commit beziehungsweise Rollback blockiert und danach sein Tombstone-/Widerrufszustand erhalten bleibt. Keine bloße Schlafpause als Synchronisationsbeweis.
3. Leeren Batch und vorhandenen Zwei-Quellenpfad einbeziehen. Die Testassertionen müssen auf Entfernen der relevanten Schreibsperren reagieren, statt lediglich eine zuvor geänderte Baseline zu erkennen. Im Bericht genau begründen, welche Assertion ohne welche Sperre scheitern würde; keine ausgeführte Mutation behaupten.
4. Bestehende serielle Negativfälle dürfen als Baseline-/CAS-Beweis erhalten bleiben, aber nicht mehr als überlappende Sperrkonkurrenz bezeichnet werden.

Scope: bestehende Rust-Testquellen des Importer-/Storagepfads und eng nötige testlokale Synchronisation sowie Ergebnisbericht. Keine neue allgemeine Testsuite, kein neuer Harness, keine Produktarchitektur, keine Runtimekonfiguration, Rollen, Migrationen, Abhängigkeiten, Cache- oder Budgeterhöhung. Bei einer erforderlichen produktiven Schnittstellenänderung zuerst den konkreten Bedarf melden, statt akzeptierte Produktteile umzubauen. Keine Code-Kommentare schreiben. Kein breitflächiges Refactoring oder Formatieren.

## Prüf- und Laufzeitgrenzen

Nur Quellarbeit und statische Prüfungen einschließlich gezieltem rustfmt --check, bash -n und git diff --check. Kein Cargo, Compiler, Testlauf, PG-/Rollenfixture, DB-Schreiben, Import, Lastlauf, Modell-/Serve-/Dienststart oder Deploy. Keinen Reviewhelper ausführen, der diese Grenzen unbemerkt überschreitet. Keine Secrets oder Rohinhalte ausgeben, keine ENV-Konfiguration. Der zentrale Slot bleibt beim Twitch-Integrator. Gültige Cutoverbeauftragung bleibt bestehen, ersetzt aber keine konkrete Laufzuteilung und keine echten Policy-/Snapshotbelege.

## Abgabe und Freigabepunkt

Eigenen Quellfix und getrennten Bericht B2-R1-SYNCHRONISATION-ERGEBNIS.md pushen, sauberer Head und vollständige SHAs. Beschreiben: Synchronisationspunkt, beobachtetes Warten, Commit-/Rollbackfolge, unveränderter Runneranschluss, neue Testselektoren und tatsächliche Ressourcenwirkung. Geschriebene Testquellen klar von ausgeführten Beweisen trennen. Danach stoppen für enges unabhängiges Nachreview durch denselben Reviewer. Kein neuer Gesamtplan.

Melde dich bei echtem Blocker mit `[Bump-up] Paket B2-R1: Grund: ... Erledigt: ... Worktree: ... Offen: ...` an den Intent-Thread 562a877b-0939-440a-964d-1145d9e9431a und stoppe danach.

ORCHESTRIERUNG[OR-1]: Stufe groß | Schritt bau | Artefakt: .tasks/2026-09-30-g5-abschluss/B2-R1-SYNCHRONISATION-BRIEFING.md
