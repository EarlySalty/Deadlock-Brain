# Q2: bestehender Consumerweg tatsächlich gebunden

Der übergebene K-Wirefall wurde im eigenen, unveränderten Quellencheckout von `8e1b8f03cf0d84f4754eedc8fe848ef0fceffce8` ausgeführt: **1 passed, 0 failed, 0 ignored, 332 filtered, Exit 0.** Kein Modelllauf und kein zugestellter Discord-/Twitch-Abnahmefall.

## Quellenbindung

| Beleg | Bindung |
|---|---|
| K-Übergabe | `CONSUMERWEG-Q-UND-DATEIGRENZE-20261008.md`, explizit vom Delegator übergeben |
| Bots-main | `git ls-remote` bestätigte `8e1b8f03cf0d84f4754eedc8fe848ef0fceffce8` |
| Eigener Testcheckout | `/home/nathanael/.worktrees/bots-q2-consumerproof-20261008`, detached auf genau diesem SHA; nach Test keine Quelländerungen |
| Fixture | `rust/bin/dl-bot/src/modglue.rs:5328`, `modglue::tests::privatfix_consumer_staff_thread_dm_binden_lesesperre_im_echten_wire` |
| Consumerdatei | SHA256 `adb7137ab3d791c2b1a7e79cdee9982e6086523b421795f9ae46f3a9455a81d2` |
| Cargo.lock | SHA256 `554c546b91727d15cf6512c5304b556d7a298263fb0eb0b1c96f3d947a9f5377` |
| Cargo.toml | SHA256 `254d01eeb9ca0c8380f9c3f314e307bb93a4052b3d5b50c786d1bdac20befe90` |
| Tatsächlicher Lauf | Geschütztes Protokollhash `453ddf1f941b94781851d06c207fa640a4f10a22eb8dc26868fa8ea0882e110c` |

Der zuerst angelegte Build wurde vor Testbeginn durch `ENOSPC` unterbrochen. Bei Wiederaufnahme waren 244 GB frei. Derselbe Quellenstand und derselbe vorhandene Test wurden erneut ausgeführt, ohne Cachelöschung, Compilerumleitung oder fremden Dienststopp. Tatsächlicher Testprofilbau: 10m46s; Test: 0,03s. Diese Testzeit ist keine P1-Antwortzeit.

## Wiederholbarer vorhandener Kontrolllauf

```sh
SQLX_OFFLINE=true /home/nathanael/.local/bin/cargo-slot +1.97.1 test --manifest-path /home/nathanael/.worktrees/bots-q2-consumerproof-20261008/rust/Cargo.toml -p dl-bot --bin dl-bot privatfix_consumer_staff_thread_dm_binden_lesesperre_im_echten_wire --locked --offline --jobs 3 --no-fail-fast -- --include-ignored
```

Dieser unveränderte Fall benutzt die bestehende Callback-/Consumer-/SDK-Kette und kontrolliertes HTTP-Loopback. K benennt dafür 17 neutrale Wireanfragen sowie Öffentlich-/Staff-/Thread-/DM-Prüfungen, Antwortort, can_reply, Widerruf und ursprünglichen Fragetext. Das erfolgreiche Q-Einzeltestergebnis wird nicht mit Ks vollständigen 61 modglue-Tests verwechselt.

Die bestehende produktive Kette lautet `answer_discord_event` → `handle_discord_query_with_read_access` → `answer_discord_query_with_read_access` → `BrainApiAnswerer` → `AsyncBrainClient::answer_for_discord_with_read_access` → `POST /v1/answer`. Private Anfragen setzen `x-discord-read-access: disabled`. Rollen-/Requestbindung und Tagesquote bleiben im Consumer, nicht im Modell. Keine zusätzliche Pipeline oder direkte Providerverbindung.

## Grenze zum echten Vergleich

Der Kontrolllauf bindet die übergebenen Quellen und den vorhandenen Consumerweg. Er bindet keine laufenden G/K-Prozesse und exportiert weder tatsächliches Accounted noch ToolExecution. Die K-Lieferakte benennt am gelesenen Stand noch laufende Aktivierung, keine vollständige Nutzerantwortabnahme.

Weiterarbeit erfolgt auf derselben festen Schicht: lokale Original-/Sollprüfung und vollständiges 30er-Set, tatsächlicher sicherer G-Messnachweis, dann belegte gemeinsame Live-/Consumerbindung. Vorher bleiben Providerläufe, Kanaltestnachrichten und P1-Messung bei 0 beziehungsweise unbekannt. Der bestandene Kontrolllauf ist ein vorbereiteter Baustein, kein Auftragsabschluss.

TESTNACHWEIS[TW-1]: 1 passed, 0 ignored | Baseline: erster Versuch vor Testbeginn abgebrochen, keine Altfehlerbehauptung
