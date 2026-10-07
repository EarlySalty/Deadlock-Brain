# Paket A: Brain-Antwort erhalten

Stand: 06.10.2026. Implementierung lokal abgeschlossen, fremde Prüfung und Freigabe stehen aus.

- Worktree: `/home/nathanael/.worktrees/brain-antwort-20261006`
- Branch: `fix/brain-antwort-nicht-verschlucken-20261006`
- Ausgangs-SHA: `5e77e7ef744876c82cf5b2c530778ec1549a4e62`
- Commit: `b08d9366b06360fa5eae87fbdf3d118154f42930`
- Geänderte Quelldatei: `rust/crates/dl-brain/src/brain_api.rs`, einschließlich der Tests in dieser Datei.
- Worktree nach dem Commit sauber.

## Verhalten

`answered` und `build_rejected` werden nach Entfernung von HTTP-/HTTPS-Links als Antwort weitergegeben. Markdown-Beschriftungen bleiben erhalten; Linkadressen und ihre umgebenden Linkklammern werden entfernt. Großgeschriebene Schemes werden ebenfalls erkannt. Bleibt nur Leerraum oder gar nichts übrig, entsteht `BrainOutcome::NoAnswer`.

Antworten werden auf höchstens 3800 UTF-16-Einheiten gekürzt. Die Kürzung erfolgt an einer Zeichengrenze, sodass auch ein Emoji am Rand vollständig erhalten oder vollständig weggelassen wird. `InsufficientEvidence` bleibt `NoAnswer`. Transportfehler, Zeitüberschreitungen und ungültige Antwortverträge bleiben Backend-Fehler.

Warnungen tragen das Feld `klasse` mit `transport`, `vertrag`, `link` oder `laenge`. Weder Fehlerkörper noch Anfrage, Token oder Nutzerkennung werden dafür ausgegeben.

## Belege

Alle Befehle liefen im Unterordner `rust` des Worktrees mit dem vorhandenen Cargo 1.99.0. Das systemweite Cargo 1.75.0 kann die vorhandene Lockdatei der Version 4 nicht lesen.

```sh
/home/nathanael/.cargo/bin/cargo test -p dl-brain brain_api::tests --locked
```

Ergebnis: 4 bestanden, 0 fehlgeschlagen. Der HTTP-Fixture-Test prüft für beide Antwortstatus einen Link, Überlänge und eine reine Linkantwort. Er prüft außerdem `insufficient_evidence`, einen vorzeitig abgebrochenen HTTP-Körper und eine falsche Vertragsversion. Der bestehende Test zur Ereignisidentität und Trennung der Anfragekontexte bleibt grün. Weitere Tests prüfen Linkschreibweisen, die UTF-16-Grenze und die Warnklassen ohne private Werte im Protokoll.

```sh
/home/nathanael/.cargo/bin/cargo test -p dl-brain --locked
/home/nathanael/.cargo/bin/cargo clippy -p dl-brain --all-targets --locked --no-deps
/home/nathanael/.cargo/bin/rustfmt --check --edition 2021 crates/dl-brain/src/brain_api.rs
git diff --check
```

Ergebnis: vollständige Crate-Suite mit 15 bestandenen Tests, keine Fehler. Clippy, Formatprüfung und Diff-Prüfung enden mit Exit 0. Clippy meldet ausschließlich Warnungen an unveränderten Stellen: `fetch_update` und zwei `double_must_use`-Warnungen aus `async_trait`.

## Offene Abnahme

Keine Freigabe erteilt. Die unabhängige Intent-Abnahme und das Rust-Review müssen durch den fremden Prüfer erfolgen. Als Blatt-Worker wurden keine weiteren Threads oder Agenten gestartet. `gate_hook.py --review` wurde nicht ausgeführt: Das Skript ist weder im Worktree noch an den dokumentierten lokalen Hook-Pfaden vorhanden.

Kein Push, kein Merge, kein Deploy und kein `systemctl`. Der laufende Bot wurde nicht verändert; eine Live-Prüfung fand entsprechend dem Auftrag nicht statt. Branch und Worktree bleiben für die fremde Prüfung erhalten.
