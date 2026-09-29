status: aktiv
Datum: 2026-09-29

# Paket G: Abbruch wegen ungeklärter Transit-Lizenz

## Befund

Der CI-Befund ist im gesperrten Git-Pin `haste_core` nachvollziehbar: `rust/crates/dbrain-replay/Cargo.toml:10` zeigt auf `deadlock-api/haste` Revision `bfb292d4798031350861ad297aa26753267a1ea6`. GitHub API antwortete für `deadlock-api/haste`, `valveprotos-rs` und `dungers` jeweils HTTP 404. Credential-freie `git ls-remote`-Versuche endeten mit `could not read Username`, bei deaktiviertem Prompt und Credential-Helper.

Die Lockdatei enthält neben `haste` noch `valveprotos-rs` Revision `4f4a3cb1b0c6f19af59a722acb79ecccd01f61f6` und transitiv `dungers` Revision `5e1e2aac76a027987911de3ef3d23ecfd992a7fb` (`rust/Cargo.lock`, Pakete `haste_core`, `valveprotos`, `dungers`). Eine `haste_core`-Änderung deckt den frischen Fetch der Transit-Abhängigkeiten nicht ab.

## Quellen und Lizenzbelege

Die lokalen Cargo-Git-Objekte enthalten die exakt gepinnten Commit-Objekte. Die Prüfungen waren lesend und haben keinen geteilten Cache verändert.

| Quelle | Commit | Tree | SHA-256 des `git archive` | Lizenz im Commit |
|---|---|---|---|---|
| `deadlock-api/haste` | `bfb292d4798031350861ad297aa26753267a1ea6` | `f6079650ff9e4e8f8e976c9c60697bf31768b6cf` | `602e418e0beb6de76b0ea188a42fc09c36f322f0eeac6ccdb12e715173283efc` | `license.txt`, BSD 3-Clause |
| `deadlock-api/valveprotos-rs` | `4f4a3cb1b0c6f19af59a722acb79ecccd01f61f6` | `6baac995594cbacc32fc1e0fc32bb99f65013a6a` | `a251a5dfc473565b16434f19c624822d1e5d977eb2eb89b9c389d19f3a609adc` | `license.txt`, BSD 3-Clause; `license-protos.txt`, Unlicense |
| `deadlock-api/dungers` | `5e1e2aac76a027987911de3ef3d23ecfd992a7fb` | `64a08056dc7000ddd44d1caa33f9efb3b1c8b4fb` | `02ef9416eed45bb9194897ade2277ae09961edb709bc55867a8feae3392b2f5b` | Lizenzangabe im Manifest, Lizenzdatei und Lizenzhinweis im Commit-Inhalt: nicht gefunden |

Der `dungers`-Commit ist eine notwendige Transit-Quelle von `haste_core` und `haste_vartype`. Eine kontrollierte Vendoring-Lösung würde diesen Quelltext mitverteilen. Dafür liegt im verifizierten Commit kein belegbarer Lizenztext vor. Die geprüfte lokale Historie umfasst 29 erreichbare Commits und einen Ref, ohne Lizenzdatei, SPDX-/Lizenzänderung oder Verweis auf eine kanonische Umbenennung des Repositories. Der dokumentierte Rename betrifft `CharCursor` zu `Charsor`. Eine Ersatzquelle oder ein unbestätigter Fork wäre ebenfalls kein belegter identischer Ersatz. Deshalb blieben Manifeste, Lockdateien und Quellen unverändert.

## Prüfungen

- `git cat-file -t <rev>`: drei Commit-IDs lieferten jeweils den Typ `commit`.
- `git archive <rev> | sha256sum`: SHA-256-Werte wie oben.
- GitHub unauthentifizierte API: drei HTTP-404-Antworten.
- Credential-freies `git ls-remote`: drei Abbrüche mit deaktiviertem Prompt und Credential-Helper.
- Erster isolierter Fetch mit `/usr/bin/cargo` 1.75.0 endete vor dem Netzwerk-Fetch mit Exit 101, weil Cargo 1.75 Edition 2024 nicht parsen kann.
- Korrigierter isolierter `cargo fetch --manifest-path rust/Cargo.toml --locked` mit Cargo und rustc 1.98.0, neuem temporärem `CARGO_HOME`, ohne Git-Credentials: Exit 101 beim Abruf von `deadlock-api/haste`; `could not read Username`, danach `revision ... not found`. Der frische Fetch bestätigt den CI-Blocker.
- Cargo 1.97.1 und `stable` zeigen jeweils Cargo/rustc 1.97.1; Cargo 1.98.0 und rustc 1.98.0 sind als passendes Paar vorhanden.
- `cargo check` und `cargo test` wurden nicht ausgeführt. Testläufe und Testresultate: 0.
- `gate_hook.py --review --repo <worktree> --base origin/migration/rust-integration --codex-astra`: Exit 0, `ALLOW: no reviewable changes` (uncommitted report-only Änderung).
- Status vor dem Commit: Produktdateien unverändert; `G-REPORT.md` war untracked. HEAD `305df2d36ec7b5d0513d6c0769051b41538d6a1b`, Branch `fix/pre-g5-locked-dependencies-20260929`.

## Eskalation

Paket G stoppt vor Quellenänderungen, bis eine lizenzbelegte Quelle für den exakt gepinnten `dungers`-Commit oder eine autorisierte Lizenzbestätigung vorliegt. Parser- und Schema-Pins bleiben unverändert. PR, Merge und Deploy sind nicht erfolgt.

INTENT[IA-1]: Stufe klein | Modell Luna | Thread 562a877b-0939-440a-964d-1145d9e9431a | Register: /home/nathanael/.worktrees/brain-technical-closeout-20260929/.tasks/2026-09-29-technical-closeout/REGISTER.md
ORCHESTRIERUNG[OR-1]: Stufe klein | Schritt bau | Artefakt: .tasks/2026-09-29-technical-closeout
TESTNACHWEIS[TW-1]: 0 passed, 0 ignored | Baseline: entfällt, kein Testlauf
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: .tasks/2026-09-29-technical-closeout/G-REPORT.md
