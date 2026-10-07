# Prüfung Paket F

## Eigene Wirkungsprüfung vor dem Gate

1. Populationszwilling in `planner.rs`: Staples bekamen neben dem Kernfilter zusätzlich pauschalen Vorrang in Kandidatenliste und Beam sowie eine Zulassung bei null Mehrwert. Entfernt. Statistischer Zuschlag verwendet jetzt den positiven gemeinsamen Marginalwert mit dem bestehenden Gewicht. Mechanische Imbue-Auswahl kommt vor beobachteten Bindungen. Regression mit starkem ungespieltem Kandidaten ergänzt.
2. HTTP-Zwilling in `main.rs`: Wiederaufnahme sendete ohne erneute Patchprüfung. Reguläre gespeicherte Anfragen müssen nun den ursprünglichen Build-Kontext enthalten und mit dessen Publish-Payload übereinstimmen. Gemeinsamer Sender prüft den Build erneut. Keine Neuberechnung, kein KI-Aufruf. Dateien ohne Build-Provenienz lösen keine neue reguläre Veröffentlichung aus.
3. API-Provenienz in `publish.rs`: Aktiver Patch, vollständiger lokaler Spiegel mit nach dem Patchbeginn geprüfter Clientversion, dokumentgebundene Mitgliedschaft der tatsächlich geladenen Werte, gültige Skills, Shop-/Inventarregeln und mechanische Bewertung aller Käufe. Keine Matchzahl, Familieneligibility oder globale Confidence-Abnahme. E-Store-Deduplizierung kann unveränderte Werte noch an die alte Version binden; F ändert diesen fremden Pfad nicht.

Zwillingssuche: Graphify zu Planungs-/Publish-Pfaden, anschließend `rg` über Reasoner, Composer, Item-Score, Planner, Meta und CLI. Verbleibende `min_matches`-/`sample_ok`-Verwendungen gehören zu ergänzender Statistik und deren Tests. `post_patch_player_matches` und `eligible_for_planning` kommen im neuen Publish- und Planungsweg nicht als Abnahme vor.

Fremddienst-Pfade: regulärer HTTP-Sender einschließlich Wiederaufnahme und regulärer Queue-Sender geprüft. Beide verlangen denselben aktuellen Guard. Explizite Review-Sender bleiben sichtbar als Review getrennt. HTTP-Erfolg verlangt einen bestätigten Status mit positiver Build-ID; Timeout und Fehler bleiben unbestätigt mit unveränderter Anfrage für Wiederaufnahme. Queue-Einreihung ist keine Erfolgsmeldung über Steam-Publish.

WIRKUNGSPRUEFUNG[WP-1]: 3 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 4/4 geprüft

## Eigene technische Prüfung

Suite, Exit 0:

```bash
PATH=/home/nathanael/.cargo/bin:/usr/local/bin:/usr/bin:/bin /home/nathanael/.cargo/bin/cargo test --manifest-path /home/nathanael/.worktrees/brain-f-publish/rust/Cargo.toml -p dbrain-reasoner -p deadlock-brain --no-fail-fast
```

401 passed, 0 failed, 19 ignored, 0 filtered out, einschließlich CLI-Integration und Doctests. Reasoner: 283 passed, 17 ignored. Die ignorierten Bestandstests benötigen isolierte Postgres-Datenbanken beziehungsweise einen echten Central-Snapshot. Sie wurden nicht entfernt oder neu ignoriert. Der reale Warden-Publish wird separat geprüft; die Unit-Suite ersetzt ihn nicht.

TESTNACHWEIS[TW-1]: 401 passed, 19 ignored | Baseline: nicht erhoben rot

Eine rote Baseline wird nicht behauptet. Der frühere eigene Fixturefehler wurde behoben. Der erste Clippy-Befehl ohne `--no-deps` scheiterte mit vier `map_or_identity`-Befunden in der unveränderten Abhängigkeit `dbrain-enrich`. Keine Zahl-gegen-Zahl-Baseline dazu erhoben.

Eigene Crates strikt geprüft, Exit 0:

```bash
PATH=/home/nathanael/.cargo/bin:/usr/local/bin:/usr/bin:/bin /home/nathanael/.cargo/bin/cargo clippy --manifest-path /home/nathanael/.worktrees/brain-f-publish/rust/Cargo.toml -p dbrain-reasoner -p deadlock-brain --all-targets --no-deps -- -D warnings
```

Alle acht geänderten Rust-Dateien mit `/home/nathanael/.cargo/bin/rustfmt --edition 2021 --check --config skip_children=true` geprüft, Exit 0. Eigene Debug-CLI mit `profile.dev.package.dbrain-reasoner.opt-level=2` gebaut, Exit 0. Kein Release oder Installationsartefakt.

## Gate-Runde 1

Noch offen. Kein ALLOW behauptet.
