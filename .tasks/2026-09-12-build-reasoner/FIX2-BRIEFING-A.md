# Fix-Briefing: build-reasoner (Paket A, Fixrunde 2, Merge-Kritiker)

[Orchestrator] Fixer für Paket A nach dem Merge-Kritiker. Paket A ist nach
Review Runde 2 freigegeben; beim Push nach main hat der Merge-Kritiker einen
Fund gemeldet, der vor dem Merge behoben wird.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-a` (Branch
  `feat/build-reasoner-a`, Commit fc74b71, ausgecheckt)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für diese Fixrunde. Keine Unter-Threads oder
  Unter-Agenten spawnen.

## Befund des Merge-Kritikers, wörtlich

1. `rust/crates/dbrain-reasoner/src/ai_roles.rs:230` (run_critic) |
   BLOCKING | `run_critic` ruft `call()` (Zeile 109) auf, das über
   `parse_json` direkt in `CriticResponse` deserialisiert und die Prüfung
   `pass`/`recompose` aus `parse_critic_response` (Zeile 191-200) auslässt.
   Der einzige produktiv nutzbare Kritiker-Pfad validiert das verdict also
   nie: die KI kann `"maybe"` liefern und der Recompose-Gate-Sinn des
   Kritikers ist tot. Der Guard existiert und wird nur im Test über
   `parse_critic_response` geprüft, greift aber im Realpfad nicht. Fix: in
   `run_critic` `parse_critic_response` statt des generischen `call`
   verwenden (oder die Prüfung in `call`/nach dem Parse ergänzen).

2. `rust/crates/dbrain-reasoner/src/data.rs:33` (number) | NIT | `number()`
   liefert bei Parse-Fehler und fehlendem Wert stumm `0.0`. Über die gesamte
   Ladeschicht werden dadurch fehlende Snapshot-Werte nicht von echten Nullen
   unterschieden.

## Was der Code leisten soll

Der Kritiker ist das Gate vor der Recompose-Runde (ARCHITEKTUR.md
Abschnitt 9): nur `pass` oder `recompose` sind gültige Urteile, alles andere
ist ein Parserfehler, der als KI-Ausfall behandelt wird (Build ohne
Kritiker-Runde ausgeben, kein Abbruch). Ursache beheben, nicht Symptom: der
Realpfad muss dieselbe Validierung nehmen wie der Test, und ein Test muss den
Realpfad mit einem ungültigen Verdict abdecken. Für den Nit: `number()` soll
fehlende oder unparsbare Werte als `None` melden, wo der Aufrufer es
unterscheiden muss, oder mindestens einen `tracing::warn!` je Feld einmal
ausgeben; entscheide sparsam und begründe in der Fertigmeldung.

## Nicht anfassen

`types.rs`-Signaturen, `lib.rs`, andere Crates. Keine Code-Kommentare.

## Tests

`export PATH=/home/nathanael/.cargo/bin:$PATH`, im Verzeichnis `rust/`:
`cargo test -p dbrain-reasoner` (Baseline ohne DSN: 7 bestanden, 5
ignoriert, 0 rot), `cargo clippy -p dbrain-reasoner --all-targets -- -D
warnings`, `cargo fmt` nur eigene Dateien. Neue Commits obendrauf, kein
`--amend`. Nur `feat/build-reasoner-a` pushen, nie main. Commit-Trailer
`Co-authored-by: <dein Modell> <modell@local>`.

## Fertigmeldung

In diesem Thread und als Anhang "Fixrunde 2 (Merge-Kritiker)" in
`REVIEW-A.md`: je Fund Datei:Zeile, Änderung, Commit-SHA, Testzahlen.
