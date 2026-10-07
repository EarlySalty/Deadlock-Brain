# Prüfung Paket F

Basis für F-Abnahme: E `f3c84fb4ee442196964387347773a75d704ebafd`. Der frühere fehlende Storage-Leser ist in diesem Commit vorhanden und regulär übernommen. Keine E-Dateien verändert.

## Eigene Wirkungsprüfung vor dem Gate

1. Populationszwilling in `planner.rs`: pauschaler Staple-Vorrang in Kandidatenliste und Beam sowie Null-Mehrwert-Zulassung entfernt. Zuschlag verwendet nur positiven gemeinsamen Marginalwert und das bestehende Gewicht. Mechanische Imbue-Auswahl kommt vor beobachteten Bindungen. Starker ungespielter Kandidat schlägt den schwachen Staple in der Regression.
2. HTTP-Zwilling in `main.rs`: reguläre Wiederaufnahme verlangt gespeicherten Build-Kontext und exakte Übereinstimmung mit der unveränderten Anfrage. Der gemeinsame Sender prüft den Build erneut gegen aktuellen Patch und Spiegel. Keine Neuplanung oder KI. Alte Dateien ohne Build-Provenienz senden nicht regulär.
3. Aktuelle Spielwerte in `data.rs`, `lib.rs`, `publish.rs`: ausschließlich Es gemeinsamer `brain_storage::asset_mirror`-Leser. Ein Ladegang bindet Held, Items, Fähigkeiten, Primärwaffe und Flexslots an dieselbe Clientversion. Eigenen Snapshot-Membership-Guard, produktive Snapshot-Abfragen für diese Werte und Item-Card-/Katalogwerte-Fallbacks entfernt. Bestehende mechanische Konverter und API-Klassifikation wiederverwendet. Zwei alte Snapshothelfer bleiben ausschließlich für historische Bestandstests erhalten.
4. Der gemeinsame Leser bietet auch ältere vollständige Historie an. F verlangt zusätzlich, dass der neueste Assets-Run erfolgreich und vollständig genau die gelesene Version bestätigt. Fehlgeschlagener, laufender, unvollständiger oder neuerer Abgleich lässt alte Werte nicht als aktuell durch. Publish verlangt Prüfung nach Patchbeginn; unveränderte Originalwerte dürfen früher erfasst worden sein. Clientversion wird nicht als historisch bestätigter Balancepatch ausgegeben.

Zwillingssuche: zuerst Graphify zu Planung, Publisher und BuildObject-Konsumenten, anschließend `rg` an den gefundenen Reasoner-, Composer-, Item-, Planner-, Meta- und CLI-Stellen. Matchzahlen bleiben Diagnose beziehungsweise ergänzende Statistik. Kein `post_patch_player_matches`- oder `eligible_for_planning`-Abnahmefilter im neuen Planungs-/Publishweg.

Fremddienst-Pfade: regulärer HTTP-Sender samt Wiederaufnahme und regulärer Queue-Sender verlangen denselben aktuellen Guard. Explizite HTTP-/Queue-Review-Pfade bleiben sichtbar getrennt. HTTP-Erfolg verlangt bestätigten Status mit positiver Build-ID. Timeout und Fehler bleiben unbestätigt mit unveränderter Anfrage. Queue-Einreihung ist kein Steam-Erfolg.

WIRKUNGSPRUEFUNG[WP-1]: 4 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 4/4 geprüft

## Finale technische Prüfung

Suite, Exit 0, `tests-mirror.log`:

```bash
PATH=/home/nathanael/.cargo/bin:/usr/local/bin:/usr/bin:/bin /home/nathanael/.cargo/bin/cargo test --manifest-path /home/nathanael/.worktrees/brain-f-publish/rust/Cargo.toml -p dbrain-reasoner -p deadlock-brain -p brain-storage --no-fail-fast
```

460 passed, 0 failed, 23 ignored, 0 filtered out, einschließlich bestehender CLI-Integration, Storage-Suites und Doctests. Reasoner: 286 passed, 17 ignored. Bestehende ignorierte Tests benötigen isolierte Postgres-Datenbanken beziehungsweise echte Central-Snapshots. Keine Tests gelöscht, abgeschwächt oder neu ignoriert. Drei aktuelle Leserregressionen ergänzt. Diese Unit-Suite ersetzt den realen Veröffentlichungsbeleg nicht.

TESTNACHWEIS[TW-1]: 460 passed, 23 ignored | Baseline: nicht erhoben rot

Keine rote Baseline behauptet. Früherer eigener Fixturefehler und acht eigene Lints wurden behoben. Die ursprüngliche breite Clippy-Prüfung fand vier `map_or_identity`-Diagnosen in der unveränderten Abhängigkeit `dbrain-enrich`; keine Zahl-gegen-Zahl-Baseline dazu erhoben und keine fremde Produktdatei geändert.

Eigene Zielcrates strikt geprüft, Exit 0, `clippy-mirror-3.log`:

```bash
PATH=/home/nathanael/.cargo/bin:/usr/local/bin:/usr/bin:/bin /home/nathanael/.cargo/bin/cargo clippy --manifest-path /home/nathanael/.worktrees/brain-f-publish/rust/Cargo.toml -p dbrain-reasoner -p deadlock-brain --all-targets --no-deps -- -D warnings
```

Alle neun F-geänderten Rust-Dateien mit `/home/nathanael/.cargo/bin/rustfmt --edition 2021 --check --config skip_children=true` geprüft, Exit 0. Eigene Debug-CLI mit `profile.dev.package.dbrain-reasoner.opt-level=2` gebaut, Exit 0, `build-cli-mirror.log`. Kein Release oder Installationsartefakt.

## Reale Veröffentlichung: noch blockiert

Regulärer Warden-Lauf mit dem finalen Leserstand, Exit 1, `warden-publish-mirror.log`:

```text
Datenfehler: API-Spiegel: Kein vollständiger lokaler Assets-Spiegel vorhanden
```

Lesender SELECT in `deadlock` bestätigt 0 vollständige erfolgreiche versionierte Assets-Runs. Kein eigener Ingest, Tick oder produktiver DB-Handeingriff. Keine Veröffentlichung und keine `hero_build_id`. Live-Aktivierung und Ingest bleiben bei live_strecke.

## Gateversuche 1 und 2

Geprüfter Kandidat: `3b964d577a6818fa1a37d56ff250c043096f0932`. Basis: E `f3c84fb4ee442196964387347773a75d704ebafd`.

```bash
python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-f-publish --base f3c84fb4ee442196964387347773a75d704ebafd --head 3b964d577a6818fa1a37d56ff250c043096f0932
```

Beide Läufe Exit 2, `gate-1.log` und `gate-2.log`. Nach dem ersten Ausfall zuerst `git status` und `git log -1`, danach genau ein unveränderter Retry. Nach dem zweiten Ausfall erneut sauberer Status und derselbe HEAD.

Wörtliche identische Antwort beider Läufe:

```text
kein Modell der Kette hat geurteilt: gpt-6.1-sol: review_gate: codex failed: bwrap: Creating new namespace failed: Cannot allocate memory | claude-opus-5-5: gate_hook: Kritiker-Wrapper ohne BLOCK: unshare: unshare failed: Cannot allocate memory | grok-4.6: gate_hook: Kritiker-Wrapper ohne BLOCK: unshare: unshare failed: Cannot allocate memory
```

Kein inhaltliches Urteil, weder ALLOW noch Code-BLOCK. Namespace-Erzeugung scheitert vor dem Modellurteil. Read-only-Diagnose: Host verfügbarer Speicher 14380 MiB, `user.max_user_namespaces=2147483647`. Daraus keine bestätigte Ursache oder Resetzeit abgeleitet. Keine Kontingentmeldung. Keine Schutzumgehung, kein eigener Modellwechsel, kein dritter Versuch und keine Eingriffe an fremden Diensten.

Der verifizierte F-Quellstand ist auf dem Featurebranch gesichert. Gate bleibt ein echter Abschlussblocker. Ausdrücklicher Release-Hold erneut gelesen: kein Main-Push oder Cleanup. Main-Abschlussversuche und schreibende Git-Schritte nach main: jeweils 0.

MERGEPROTOKOLL[MS-1]: 0 Git-Schritte einzeln | Anläufe: 0 | Gate: kein Urteil, 2 Reviewversuche mit Namespace-Speicherfehler
