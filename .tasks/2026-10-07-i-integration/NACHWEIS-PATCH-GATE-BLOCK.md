# Neuer Patch-BLOCK nach dem einmalig freigegebenen gemeinsamen Gate

## Umfang

Entscheidung `ENTSCHEIDUNG-I-READER-GATE.md`, Stand 07.10.2026, 15:09 UTC: Gegenprobe übernehmen, kombinierten Kandidaten prüfen, genau ein gemeinsames Gate mit unverändert Claude Opus 5.5. Bei BLOCK Urteil samt neuem reproduzierbarem Szenario zurückgeben; kein Merge bei BLOCK.

Gegenprobe `90c178001258d050c781a42c4c9b2fd7cef99bbf` wurde in den eigenen Integrationskandidaten übernommen. Geprüfter HEAD `860793d7f89d2af9f7510d663e56d592fedf18b2`, Tree `9b917e98bbe2ef24c97319991d2d6bdbbf379254`, Basis `ca4d877f13042c9a7a7023e54f6bf2c688b69ac4`. Produktcode in E `90c17800` und Kandidat `860793d7` ist identisch: `git diff --exit-code 90c178001258d050c781a42c4c9b2fd7cef99bbf 860793d7f89d2af9f7510d663e56d592fedf18b2 -- rust scripts`, Exit 0.

## Gemeinsame Prüfung

```text
env -u DEADLOCK_CENTRAL_DSN -u DATABASE_URL SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo test --locked --manifest-path /home/nathanael/.worktrees/brain-i-release-20261007/rust/Cargo.toml -p brain-storage -p deadlock-brain-core -p dbrain-sources -p dbrain-builds -p deadlock-brain -j 2 -- --test-threads=1
```

554 passed, 0 failed, 24 ignored, 0 filtered, 32 Testtargets, Harness-Exit 0. Die Core6-/Global-Gegenprobe läuft tatsächlich im kombinierten Kandidaten und besteht. Original `/tmp/brain-i-e-evidence-candidate-tests.log`. Formatcheck der fünf genannten Crates Exit 0. Striktes Clippy der fünf Crates plus brain-serve, `--all-targets --no-deps -j 2 -- -D warnings`, Exit 0, 4m 35s. Originale `/tmp/brain-i-e-evidence-candidate-fmt.log` und `/tmp/brain-i-e-evidence-candidate-clippy.log`.

TESTNACHWEIS[TW-1]: 554 passed, 24 ignored | Baseline: keine Altfehler behauptet

Einziger neuer gemeinsamer Urteilslauf:

```text
python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-i-release-20261007 --base ca4d877f13042c9a7a7023e54f6bf2c688b69ac4 --head 860793d7f89d2af9f7510d663e56d592fedf18b2 --model claude-opus-5-5 --effort high --timeout 900
```

Harness-Exit 1. Original `/tmp/brain-i-e-evidence-common-gate-opus55.log`:

```text
BLOCK: Der Patchimport kann fremden Inhalt falsch zuordnen und bei einem zulässigen Link den Tageslauf abbrechen.

1. rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:372 | BLOCKING: Bei Steam-Seiten ohne passende Ereigniskennung wählt der aufgerufene HTML-Leser ersatzweise irgendein Ereignis nach Titel und Zeit. Enthält die Seite nur ein anderes Ereignis mit Spieländerungen, wird dessen Inhalt unter der URL des angefragten Patches importiert. Kein gleichartiger neuer Auswahlpfad im Diff gefunden.
2. rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:124 | BLOCKING: trusted_original_url akzeptiert Fragmente, get_bounded verwirft sie aber vor dem Abruf. Ein Feed-Eintrag mit #… beendet deshalb den gesamten Patchimport und über das Skript auch den Builddatenlauf. Das betrifft beide akzeptierten Quellen, Steam und Forum.
3. rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:173 | NIT: Auch ein harmloser Bildlink lässt einen vollständigen Originalbeitrag als Vorschau durchfallen. Eine Original-Volltextprobe mit Bildlink würde den produktiven Umfang klären.

WIRKUNGSPRUEFUNG[WP-1]: 3 Befunde | Zwillingssuche: keine Fundstelle | Fremddienst-Pfade: 6/6 geprüft
```

Der vorher widerlegte Reader-Verlust ist in diesem Urteil nicht mehr genannt. Dies sind zwei neue, tatsächlich reproduzierte Producer-Befunde, kein fortgesetzter identischer Reader-Widerspruch. Kein weiteres gemeinsames Gate und kein Produktfix ausgeführt.

## Reproduktion durch unveränderte Produktfunktionen

`PATCH-GATE-DIAGNOSE.rs` enthält zwei einmalige diagnostische Zeugen. Sie wurden ausschließlich über ein vorübergehendes cfg(test)-Modul in E angebunden. Der Import wurde anschließend vollständig entfernt; `git diff --exit-code -- rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs` ist leer und Exit 0. Die Diagnose liegt nicht in der regulären Testsuite und schreibt das Fehlverhalten nicht als gewünschtes Produktverhalten fest.

```text
env -u DEADLOCK_CENTRAL_DSN -u DATABASE_URL SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo test --locked --manifest-path /home/nathanael/.worktrees/brain-e-deadlock-api/rust/Cargo.toml -p deadlock-brain --bin deadlock-brain gate_return_diagnostics -j 2 -- --test-threads=1 --nocapture
```

2 diagnostische Zeugen bestanden, 0 failed, 0 ignored, 112 filtered; Harness-Exit 0, 18.16s Kompilation und 0.02s Ausführung. Das bedeutet: Fehlverhalten erkannt, nicht Produkt freigegeben. Original `/tmp/brain-i-new-patch-blocker-diagnostics.log`:

```text
DIAGNOSE: Quelle=steam; Feed akzeptiert Fragment=true; echter HTTP-Guard liefert Fehler vor Abruf=true; Resolver liefert Err=true
DIAGNOSE: Quelle=forum; Feed akzeptiert Fragment=true; echter HTTP-Guard liefert Fehler vor Abruf=true; Resolver liefert Err=true
DIAGNOSE: angefragt=703281025618282632; einziges Ereignis=703281025618282631; fremder Inhalt unter angefragter URL vorbereitet=true
```

Striktes Clippy mit dem vorübergehend angebundenen Diagnosemodul Exit 0; Original `/tmp/brain-i-new-patch-blocker-diagnostics-clippy.log`. Diagnosequelle separat mit rustfmt formatiert. Der geprüfte Integrationskandidat blieb bei der Diagnose unverändert.

### Urteil und begrenzte Lösungsrichtung

1. **Bestätigt:** Der tatsächliche Steam-Leser in `pg_patchnotes.rs:462-513` fällt bei fehlender passender GID ohne Mindestbindung auf den besten beliebigen Titel-/Zeit-Treffer zurück. Kontrollierte HTML-Probe: angefragt GID ...632, einzig vorhandenes Ereignis GID ...631, dessen Gameplaybody wird unter der angefragten ...632-URL in einem echten PreparedPatch-Ereignis geführt. Lösung im bestehenden Aufrufer/Leser: die tatsächlich angefragte Ereignisidentität binden und fehlende passende Originale ablehnen, statt Fremdereignisse zu verwenden. Legacy-Aufrufer und vorhandene Announcement-Verträge dabei prüfen; keinen Ersatzparser bauen.
2. **Bestätigt:** Sowohl Steam- als auch Forum-Links mit Fragment passieren validate_post; resolve_api_source reicht den unveränderten Link an den echten HTTP-Core weiter. `http/bounded.rs:128-138` liefert vor Netzwerkzugriff Err. Der Resolver reicht Err weiter; `sync_patchnotes` propagiert ihn mit `?`. `run_build_data_with_infisical.sh` hat set -euo pipefail und führt den Builddatenaufruf erst danach aus. Lösung im bestehenden Übergang: erlaubte Original-URL für den Abruf konsistent kanonisieren; HTTP-Core-Sicherheitsguard nicht lockern und Source-Bindung nicht verlieren.

### Aussagegrenzen

Die Steam-Probe verwendet kontrolliertes synthetisches HTML, keine abgerufene offizielle Originalseite. Sie belegt deterministisches Auswahl- und Herkunftsverhalten des tatsächlichen Parsers, nicht die Häufigkeit des Fehlers in aktuellen echten Feed-Daten. Die Fragmentprobe benutzt echte Validator-/HTTP-/Resolverfunktionen, aber absichtlich keinen Netzwerkzugriff und keinen Produktiv-DB-Schreibzugriff. Der Tageslaufabbruch ist zusätzlich am unveränderten Skript-/Fehlerpfad bestätigt, nicht durch einen absichtlich ausgelösten Produktivabbruch. Bildlink-NIT bleibt offen. Kein Deploy, Import oder Veröffentlichung daraus ableiten.
