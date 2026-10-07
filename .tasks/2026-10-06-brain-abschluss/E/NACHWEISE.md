# Paket E: Prüfbelege

Stand: eigener Featurebranch `feat/brain-deadlock-api-daten`; kein Main- oder Liveabschluss.

## Deterministischer Schlusslauf

```bash
SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo test --manifest-path /home/nathanael/.worktrees/brain-e-deadlock-api/rust/Cargo.toml -p brain-storage -p deadlock-brain-core -p dbrain-sources -p dbrain-builds -p deadlock-brain -j 2 --no-fail-fast -- --test-threads=1
```

Exit 0. 31 Ergebnisblöcke: 513 passed, 0 failed, 24 ignored, 0 filtered. Vollständiges lokales Log: `E-tests-final-3.log`. Die echte isolierte Postgres-Probe `imported_heads_preserve_base_and_block_preparation_commit_races` und die neue Leername-Gegenprobe liefen erfolgreich. Ignorierte übrige DB-/Browser-/Datenproben sind kein ausgeführter Beleg. Keine produktive DB genutzt.

TESTNACHWEIS[TW-1]: 513 passed, 24 ignored | Baseline: unbekannt rot

Keine Altfehlerbehauptung. Der parallele Sammellauf hatte sechs Wiki-Lockfehler; derselbe vollständige Lauf bestand mit seriellen Testthreads. Der eigene neue Dependency-Zyklus und drei durch das Pin-Update veraltete Analytics-Fixtures wurden korrigiert. Keine Bestandstests gelöscht oder abgeschwächt.

## Öffentliche Adapterproben

Die zwei standardmäßig ignorierten Netzproben liefen zusätzlich getrennt, je 1 passed, 0 failed, 0 ignored. Keine Nutzer-/Community-Kennungen, keine Einzelmatches und keine DB-Schreibvorgänge.

```bash
SQLX_OFFLINE=true DBRAIN_EXTERNAL_LIVE_CONTRACT=1 /home/nathanael/.cargo/bin/cargo test --manifest-path /home/nathanael/.worktrees/brain-e-deadlock-api/rust/Cargo.toml -p dbrain-sources --lib -j 2 assets_api::tests::live_versioned_game_assets_contract -- --exact --ignored --nocapture --test-threads=1
```

Ergebnis aus `E-live-game-assets-2.log`: `client_version=6759`, alle sechs gepinnten Antworten über den echten bounded HTTP-/SourceIr-/Assets-Adapterweg erfolgreich. Die erste Probe fand den unbelegten Mindestlängen-Guard für leere Namen; korrigiert nach dem echten API-Vertrag, ohne Ersatzwerte oder schwächere ID-Prüfung.

| Art/Sprache | Bytes | Entitäten | Roh-SHA256 |
| --- | ---: | ---: | --- |
| items/english | 6037726 | 746 | `86540843b94601ca753068767f1320649e03634e22c070fe2aead8d6faf85287` |
| items/german | 6188422 | 746 | `96bef40a5a95f06477d7841b8e3814af6c12e57d658b2509e3257de51f5aea19` |
| heroes/english | 1296827 | 40 | `f446d606a25fb468924d83910591e8f38363d593871b8c2a7251233da897cc72` |
| heroes/german | 1304756 | 40 | `f3500278a32ea7b3e52154ecc335778ca5aba47ce004ef891b96a4903dc07ff1` |
| heroes_all/english | 1862430 | 65 | `172aa578b4624c6c7800b72467336347c0ce0a3303e3319f8a54e221668a5edb` |
| heroes_all/german | 1871129 | 65 | `1e918fb8e4550b194633a35d64a835a37aaa299c528ad0c6972cd81a48ebc889` |

Bestehende kleine Schema-/Colors-Probe: `E-live-contract-2.log`, 1 passed. OpenAPI: 454617 Bytes, SHA256 `d76b44f82d1da13eed5831e772498a56993ebd56f4976c118f8ecaca79747359`; Colors: 11675 Bytes, validiert. Ihr erster Lauf erkannte die echte Schemaabweichung gegenüber September. Der heutige Pin wurde ausdrücklich geprüft und ergänzt; der alte Original-Pin bleibt erhalten.

## Format, Lints und Wrapper

```bash
/home/nathanael/.cargo/bin/cargo fmt --manifest-path /home/nathanael/.worktrees/brain-e-deadlock-api/rust/Cargo.toml -p brain-storage -p deadlock-brain-core -p dbrain-sources -p dbrain-builds -p deadlock-brain -- --check
SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo clippy --manifest-path /home/nathanael/.worktrees/brain-e-deadlock-api/rust/Cargo.toml -p brain-storage -p deadlock-brain-core -p dbrain-sources -p dbrain-builds -p deadlock-brain --all-targets --no-deps -j 2 -- -D warnings
bash -n /home/nathanael/.worktrees/brain-e-deadlock-api/scripts/run_build_data_with_infisical.sh
```

Alle drei Exit 0, lokale Logs `E-fmt.log` und `E-clippy-own-final.log`. Der breite Clippy-Lauf ohne `--no-deps` blieb an vier `map_or_identity`-Diagnosen in `dbrain-enrich/src/lib.rs` rot (519, 651, 1291, 1442). Nicht als vorbestehend behauptet, keine fremde Produktdatei oder Lint-Ausnahme geändert.

## Gate und Grenzen

Schema-Commit `5e70da3a6f40c0f1eedc78565641d8dfce582f56` wurde separat gegen die freigegebene Basis `bfda408cb988722ddceadb56bca5b72e12d12731` geprüft. Zwei Versuche, beide Exit 2, kein Modellurteil. Originalmeldungen in `E-gate-schema.log` und `E-gate-schema-2.log`: Namespace-Erstellung durch `bwrap` bzw. `unshare` scheitert mit `Cannot allocate memory`. Im ersten Lauf zusätzlich Grok HTTP 402 wegen erschöpftem Guthaben. Keine BLOCK-Codebefunde und kein ALLOW. Kein verlässlich ableitbarer Reset-Zeitpunkt; keine Hook- oder Sandboxumgehung.

API-Implementierung `e65efae2c7c53dd4d17d75f51974753d6fd84d08` wurde separat gegen `5e70da3a6f40c0f1eedc78565641d8dfce582f56` geprüft. Ebenfalls Exit 2 mit derselben Namespace-Fehlermeldung, kein Modellurteil (`E-gate-api.log`). Auch der kleine Implementierungsdiff behebt den Werkzeugausfall nicht. Insgesamt drei Gate-Aufrufe; kein ALLOW, keine Mergefreigabe, unverändert ungeprüftes Feature-WIP. Weiterer Versuch erst nach Reparatur des Gate-Werkzeugpfads, nicht mit anderem Modell oder abgeschalteter Sandbox.

Keine Releasebuilds, Installationen, Neustarts, produktiven Ingests, Korpusaktivierungen oder Main-Pushes. Öffentliche Payloadproben sind kein Import-, Publish- oder Liveantwortbeweis. Parser-/Matchabbau außer den beiden konkret entfernten Wegen ist nur vorgemerkt.
