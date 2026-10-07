# Inventur A-I4: Build-Reasoner und Draft-PRs

Read-only Rückgabe des nativen Workers `A-I4`, von Paket A als Akte gespeichert. Nichts veröffentlicht, keine Produktänderung. Soll: `.tasks/2026-09-16-reasoner-item-zweck/BEFUND.md`, Warden 779996 und aktueller Nutzerauftrag. Codebelege gelten für `origin/main`.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/dbrain-reasoner/src/lib.rs:194 | Anknüpfung: vorhandene Mechanikrechnung, Familienplanung, Backtests und HTTP-Publish

## Ist: halb

1. Main `d6131cc52711a3e8b02d299704244f8d7dbdbce6`; installierte CLI/Serve `e56e075d486a75f83f4954b58d8113588082d3f1`, beide Manifeste und `/proc/1327506/exe` geprüft. Frischer Warden-Lauf mit `--no-ai --no-persist --json`: Exit 0, zehn Core-Items, 16 Skill-Schritte, `confidence=Low`, 47 Nach-Patch-Matches. Familiengesamtzahl 1102 ist kein aktueller Patchbeleg.
2. Spirit-Konversionen, bedingte Heilung, Schadenkanäle und endliche Schilde liegen bereits auf Main. Zentrale Mechanikdateien und `examples/build_evaluation.rs` sind blobgleich mit PR 9. Noch unquantifiziert: unter anderem `OutOfCombatHealthRegen`, `SlowResistancePercent` und Warden-Fähigkeitswerte. Konfidenzdeckel ist in `composer.rs:697` verdrahtet.
3. Lesender Backtest: Populations-Staple-Gate bestanden, Jaccard@12 0,8333, Kendall 0,7641. Kein Nachweis gegen exakt 779996, da CLI nach Autoren aggregiert. Eingefrorene Phase-0/A-Artefakte für 779996 weiterhin 6/9 Referenzwaffen und rotes Staple-Gate. Reguläres Publish verlangt mindestens 100 Nach-Patch-Matches und verwirft Low (`publish.rs:109`). HTTP-Weg liefert positive `hero_build_id` erst nach tatsächlichem Erfolg. Keine Veröffentlichung in dieser Inventur.
4. PRs mit gh geprüft: 3 bis 6 sowie 9 offen als Draft, 7 und 8 geschlossen und nicht gemergt. Head-SHAs jeweils kein Vorfahr von Main, Ancestor-Exit 1. Zentrale Reasoner-Inhalte liegen trotzdem identisch auf Main. Titel oder fehlende Ancestry allein sind kein Übernahme- oder Löschbeleg.
5. Drei Reasoner-Worktrees sauber. `docs/reasoner-all-heroes` enthält zusätzliche Doku. WIP `3e7864a` bietet selektiv brauchbare generische Spielstil-Helfer in `lib.rs`; sein Situations-Staple-Fix liegt bereits auf Main. Alternativen Review-Publish nicht als regulären Qualitätsabschluss übernehmen. WIP `4269337` enthält schemaunabhängige Queries, entsprechende Laufzeitqueries bestehen auf Main. Beide WIPs nicht vollständig integriert.

## Nötiger Fertigbau

Vorhandene Daten-/Mechanikrechnung unter `rust/crates/dbrain-reasoner/src/` weiterführen, insbesondere `data.rs`, `item.rs`, `mechanics.rs`, `combat.rs`. Reproduzierbaren Nachweis gegen exakt 779996 über vorhandene Examples und frühere Nachweise herstellen. Mit aktuellen echten Matchdaten bewerten. Qualitäts- und Publishgrenzen bleiben unverändert. Kein eigener Alternativpublisher und keine neue Architektur.
