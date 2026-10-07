# Paket D: enger Weapon-Spirit-Resolverfix

## Eigentum und Stand

Basis: `75db93ef91010ccfe6c3d501eb2e7107e79f3c12`.
Worktree: `/home/nathanael/.worktrees/brain-ernte-spielstil-resolver`.
Branch: `fix/brain-spielstil-resolver-20261007`.

D besitzt ausschließlich den Reasoner-Fehler und seine Tests. Nur `rust/crates/dbrain-reasoner/src/playstyle.rs` geändert; Kontextfix und fremde Dateien unverändert. Der bestehende `mechanics::weapon_spirit_scaling`-Resolver liefert die drei effektiven Koeffizienten. Mindestens einer muss endlich und positiv sein. Keine eigene Alias- oder Substringerkennung mehr.

Featurecommit: `bfda408cb988722ddceadb56bca5b72e12d12731`, einziger Parent ist die oben genannte 75db-Basis. Featurebranch auf origin gepusht; `ls-remote` bestätigt denselben SHA, Worktree ohne uncommittete Sourceänderungen. Regulärer Selfgate `[gpt-6.1-sol] ALLOW`, Exit 0. Ein nicht blockender Kontext-NIT; laut Orchestrator durch unabhängige Resolver-/SHA-Prüfung abgedeckt.

07.10.2026, 04:12 CEST: Root meldet SHA-identische saubere Vorwärtsintegration dieses Commits auf `origin/main` und Übergabe der Finalsource an `live_strecke`. D bestätigt den Main-/Betriebshold auf `bfda408c`. Kein D-Main-Push, Releasebau, Install, Neustart oder Tick. Keine zusätzliche Review- oder Buildkette. Luna 18769 und Timerfence unverändert. Prüf- und Gateartefakte sowie die eigene Featurequelle bleiben für die gemeinsame Abnahme erhalten; fremde Branches unverändert. Kein Gesamtfertigbeleg oder Self-Settle aus D.

## Vier Gegenproben

1. `ERoundsPerSecond` 0 oder -0,1 überschreibt `EFireRate` 50 in beiden Reihenfolgen; reine Spirititems bleiben ausgeschlossen.
2. Unbekannte und nur ähnlich benannte Schlüssel erlauben keine Spirititems, einschließlich `BulletDamage` und `eFireRate`.
3. Alle drei unterstützten Achsen sowie der zulässige Prozentfallback: positive endliche Werte erlauben das echte Spirititem; null, negative und nichtendliche Werte nicht.
4. Prozentfallback bei Basisrate 0 oder nichtendlich wird ausgeschlossen; nichtendliche Direktwerte dürfen durch einen validen Alias gemäß Resolver wiederhergestellt werden.

Die Tests prüfen `Playstyle::Weapon.allows_item` mit einem echten `ItemModel` und dem vorhandenen Itemklassifikator. Keine reine Achsenattrappe für diese Gegenproben.

## Ausgeführte gezielte Tests

| Filter | Bestanden | Fehlgeschlagen | Ignoriert | Herausgefiltert |
| --- | ---: | ---: | ---: | ---: |
| `playstyle::tests` | 9 | 0 | 0 | 285 |
| `mechanics::tests` | 42 | 0 | 0 | 252 |
| `combat::tests` | 41 | 0 | 0 | 253 |
| `item::tests::fix_e_` | 2 | 0 | 0 | 292 |
| `item::tests::r` | 2 | 0 | 0 | 292 |
| `item::tests::passive_spirit` | 1 | 0 | 0 | 293 |

97 unterschiedliche gezielte Tests bestanden. Keine Vollsuite oder neue Baseline für diesen engen Fix gemessen; frühere Vollsuite-/Baselinezahlen sind historische Belege und werden nicht als neue 75db-Abnahme ausgegeben.

Wörtliche Befehle, keine zusätzlichen Env-Werte:

```text
/home/nathanael/.cargo/bin/cargo +1.97.1 test --locked --jobs 2 --manifest-path /home/nathanael/.worktrees/brain-ernte-spielstil-resolver/rust/Cargo.toml -p dbrain-reasoner --lib playstyle::tests -- --include-ignored
/home/nathanael/.cargo/bin/cargo +1.97.1 test --locked --jobs 2 --manifest-path /home/nathanael/.worktrees/brain-ernte-spielstil-resolver/rust/Cargo.toml -p dbrain-reasoner --lib mechanics::tests -- --include-ignored
/home/nathanael/.cargo/bin/cargo +1.97.1 test --locked --jobs 2 --manifest-path /home/nathanael/.worktrees/brain-ernte-spielstil-resolver/rust/Cargo.toml -p dbrain-reasoner --lib combat::tests -- --include-ignored
/home/nathanael/.cargo/bin/cargo +1.97.1 test --locked --jobs 2 --manifest-path /home/nathanael/.worktrees/brain-ernte-spielstil-resolver/rust/Cargo.toml -p dbrain-reasoner --lib item::tests::fix_e_ -- --include-ignored
/home/nathanael/.cargo/bin/cargo +1.97.1 test --locked --jobs 2 --manifest-path /home/nathanael/.worktrees/brain-ernte-spielstil-resolver/rust/Cargo.toml -p dbrain-reasoner --lib item::tests::r -- --include-ignored
/home/nathanael/.cargo/bin/cargo +1.97.1 test --locked --jobs 2 --manifest-path /home/nathanael/.worktrees/brain-ernte-spielstil-resolver/rust/Cargo.toml -p dbrain-reasoner --lib item::tests::passive_spirit -- --include-ignored
```

Spielstil-Log: `/tmp/claude-1000/-home-nathanael-repos-Deadlock-Brain/a29c0fb6-7ae5-410d-ad7a-93376eb76266/tasks/bswt4jx0l.output`, Exit 0.
Item-Log: `/tmp/claude-1000/-home-nathanael-repos-Deadlock-Brain/a29c0fb6-7ae5-410d-ad7a-93376eb76266/tasks/bwf20sfy0.output`, drei sequenzielle Aufrufe per `&&`, alle grün, Exit 0.
Mechanics und Combat: Vordergrundläufe im Sessiontranscript, jeweils Exit 0.

## Format und Clippy

```text
/home/nathanael/.cargo/bin/cargo +1.97.1 fmt --manifest-path /home/nathanael/.worktrees/brain-ernte-spielstil-resolver/rust/Cargo.toml -p dbrain-reasoner -- --check
/home/nathanael/.cargo/bin/cargo +1.97.1 clippy --locked --jobs 2 --manifest-path /home/nathanael/.worktrees/brain-ernte-spielstil-resolver/rust/Cargo.toml -p dbrain-reasoner --all-targets -- -D warnings
```

Paketweiter fmt-Check nach lokaler Formatkorrektur Exit 0. `git diff --check` Exit 0. Striktes Clippy aller Reasoner-Targets mit `-D warnings`: Exit 0, 1m 59s. Rohlog `/tmp/claude-1000/-home-nathanael-repos-Deadlock-Brain/a29c0fb6-7ae5-410d-ad7a-93376eb76266/tasks/bhat95ssh.output`.

Finale Spielstiltests nach letzter Sourceänderung zusätzlich aus dem frisch kompilierten Testbinary ausgeführt: 9 bestanden, 0 fehlgeschlagen, 0 ignoriert, 285 herausgefiltert. Keine Doppelzählung in den 97 unterschiedlichen Tests.

```text
/home/nathanael/.worktrees/brain-ernte-spielstil-resolver/rust/target/debug/deps/dbrain_reasoner-d4d6a66a5ed7a34e playstyle::tests --include-ignored
```

TESTNACHWEIS[TW-1]: 97 passed, 0 ignored | Baseline: n/a rot

Baseline n/a: keine neue Baseline gemessen und keine Altfehlerbehauptung für diesen engen Fix.

## Selfgate und Übergabe

Exakter Aufruf, Urteil und Kontext-NIT stehen in `D/REVIEW.md`. Rohlog `/tmp/claude-1000/-home-nathanael-repos-Deadlock-Brain/a29c0fb6-7ae5-410d-ad7a-93376eb76266/tasks/bhemwvxfm.output`: `[gpt-6.1-sol] ALLOW: No grounded blocking defect in the supplied diff.`, Exit 0. Kein BLOCK oder Reviewerwechsel.

MERGEPROTOKOLL[MS-1]: 13 Git-Schritte einzeln | Anläufe: 1 | Gate: [gpt-6.1-sol] ALLOW (Selfgate; kein Mainmerge durch D)

Umfang der Featureabgabe: Fetch, Worktree/Branch anlegen, Diffstat, Whitespaceprüfung, Status, nur eigene Datei stagen, Stagingumfang prüfen, Commit, SHA erfassen, Featurepush, Remote-SHA prüfen, sauberer Sourcezustand, Parentprüfung. Keine Mainoperation aus D. Root hat die SHA-identische Mainintegration ausdrücklich gemeldet; der Main-/Betriebshold bleibt bestehen.
