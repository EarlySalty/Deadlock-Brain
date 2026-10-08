# Q2: lokale Collector-Befehle

Im zugewiesenen Q2-Worktree bauen:

```sh
RUSTUP_TOOLCHAIN=1.97.1 cargo-slot build --locked --offline --jobs 3 --manifest-path .tasks/2026-10-07-brain-fertigstellung-astra/Q/collector/Cargo.toml
```

Die neue Befehlsgruppe arbeitet ohne Netzwerk und Modell. Bestehende Snapshotbefehle bleiben erhalten. Ein absoluter, kanonischer Quellpfad verhindert versehentliche Abhängigkeit von der im Binary eingebetteten alten Quellenwurzel.

```sh
.tasks/2026-10-07-brain-fertigstellung-astra/Q/collector/target/debug/brain-q-source-snapshot compare-private /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-fertigstellung-astra/Q/private /home/nathanael/.local/share/brain-q-private-20261007-bf54e659
.tasks/2026-10-07-brain-fertigstellung-astra/Q/collector/target/debug/brain-q-source-snapshot audit /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-fertigstellung-astra/Q/private/q-partial-v1-20261007
```

Für ein neues lokales Prüfverzeichnis mit 0700:

```sh
.tasks/2026-10-07-brain-fertigstellung-astra/Q/collector/target/debug/brain-q-source-snapshot prepare-review /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-fertigstellung-astra/Q/private/q-partial-v1-20261007 /absoluter/geschützter/neuer/pruefordner
.tasks/2026-10-07-brain-fertigstellung-astra/Q/collector/target/debug/brain-q-source-snapshot check-review /absoluter/geschützter/neuer/pruefordner/review-v1.json
```

`prepare-review` verweigert ein vorhandenes `review-v1.json`. Das inzwischen vorbereitete lokale Ziel wird nicht erneut erzeugt. Der Platzhalterpfad oben ist ein bewusstes Beispiel, kein existierender oder ausgeführter Befehl.

`review-v1.json` enthält private Originaltexte. MUST NOT per Read oder Dump in eine Codiermodellkonversation, Git oder einen Bericht übernehmen. Lokal Herkunft, Bereinigung, Sollfakten, Belege und Transport prüfen. Strukturcheck bleibt bei weniger als 30 geprüften Fällen rot. Ein grüner Strukturcheck beweist keine fachliche Abnahme und erlaubt keinen Provideraufruf.

Die neue Prüfliste ist außerhalb des Worktrees erhalten unter `/home/nathanael/.local/share/brain-q2-private-20261008/review/`. Öffentliche Originalassets liegen dort getrennt unter `public-assets/`. Beide Kopien wurden byte-, eigentümer- und rechtegeprüft. Historische private Sicherungen bleiben unverändert.

## Tatsächliche Prüfungen

```sh
RUSTUP_TOOLCHAIN=1.97.1 cargo-slot test --locked --offline --jobs 3 --manifest-path .tasks/2026-10-07-brain-fertigstellung-astra/Q/collector/Cargo.toml -- --include-ignored
RUSTUP_TOOLCHAIN=1.97.1 cargo-slot clippy --locked --offline --jobs 3 --all-targets --manifest-path .tasks/2026-10-07-brain-fertigstellung-astra/Q/collector/Cargo.toml -- -D warnings
RUSTUP_TOOLCHAIN=1.97.1 cargo-slot fmt --manifest-path .tasks/2026-10-07-brain-fertigstellung-astra/Q/collector/Cargo.toml --check
```

11 Tests, darunter echte Dateisystemprüfungen für Modiverweigerung, Symlinks, Digestabweichung und Überschreibungsverbot. Keine Produktions-DSN, keine Botnachricht und kein Ausfall eines fremden Dienstes erforderlich.
