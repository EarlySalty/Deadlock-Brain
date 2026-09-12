# FERTIG-E

Paket E ist umgesetzt und geprüft, einschließlich des verbindlichen Nachtrags zur Spirit-Skalierung der Feuerrate. Stand: 13.09.2026, 00:10 Europe/Berlin. Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`. Ein Thread, keine Unter-Threads oder Unter-Agenten.

Worktree: `/home/nathanael/.worktrees/deadlock-brain-e`. Branch: `fix/build-reasoner-patch-delta`. Ausgangsstand: `e958158736f9f92a9e1601fdf80fcb9d6a00e3cb`. Verifizierter Code-Commit: **`4434a522d5f7843371aecabab23f31354044095d`**, auf dem eigenen Branch gepusht. Kein Merge oder Push nach main, kein Release-Build, kein Deployment.

Der Debug-Lauf erzeugt 19 positive Kern-Items und genau zwölf optionale Items. Juggernaut und Mercurial Magnum sind im Kern. Die drei weiteren erwarteten Referenz-Items bleiben wegen ihrer Scores unter dem unveränderten Kern-Cutoff; die Ursachen stehen unten. Es wurden keine Scoring-Gewichte auf die Referenz-Auswahl eingestellt.

Ursachenbelege am Ausgangsstand `e958158`, vor der Implementierung gelesen:

| Befund | Ursache und Beleg |
| --- | --- |
| Historie verfälscht aktuelle Werte | `patch.rs:212` verarbeitet sämtliche Events ohne Vergleich mit einem Snapshot-Zeitpunkt; `lib.rs:78` wendet sie vor dem Scoring an. Die SELECTs in `data.rs:506`, `:581`, `:635` und `:639` geben nur Payloads weiter, obwohl sie nach `fetched_at` sortieren. Der Briefing-Livebefund nennt 416 ältere Warden-Events und 395 von 397 älteren Kern-Item-Events. Der lesende Loader-Lauf bestätigt 416 Warden-Events. |
| Falsche Richtung und Größenordnung | `patch.rs:146` bevorzugt das Label `buff`/`nerf`; `:183` berechnet eine absolute Differenz. `:233`, `:249` und `:274` addieren diese auf breit passende Felder. Eine Damage-Zeile kann damit fremde Damage-/Bullet-Felder treffen; Skalierungen landen sogar in `per_level` (`:262`). Das erklärt die im Briefing gemeldeten Scores bis Mercurial Magnum −4378,81 und Express Shot −4225,00. |
| Duplikate mit widersprüchlichen Zahlen | Die eigene lesende DB-Abfrage findet die Mercurial-Zeile vom 22.05.2026 fünfmal: drei Labels `buff`, zwei `nerf`. Die drei Buff-Fassungen haben `0.465 → 0.49`; die beiden Nerf-Fassungen fälschlich `0.465 → 0`, obwohl ihre Rohzeile ebenfalls `0.49` nennt. Quellen sind Changelog, Steam-Appnews, CDN-URL und Steam-URL. Die vorhandene ID-Deduplizierung in `data.rs` beseitigt JOIN-Kopien, aber nicht diese unterschiedlichen Event-IDs. |
| Mehrfache numerische Patch-Wirkung | Zusätzlich zur Modelländerung addiert `item.rs:169` einen Patch-Bonus zum Score. `composer.rs:77` addiert das Delta erneut für die Sortierung. Damit ist die Reihenfolge nicht mehr allein aus dem ausgegebenen `total` erklärbar. |
| Negative Kern-Items, 144 optionale Items, irreführende Konfidenz | `composer.rs:216` prüft nur `core.len() < 19`; `:219` hängt alle übrigen Items an Optional. `:135` übernimmt `confidence` ungeprüft. |
| Spirit-Feuerrate fehlt im Item-Score | `data.rs:242` lädt die Schlüssel bereits korrekt. Der aktuelle Warden-Assets-Snapshot vom 06.07.2026 enthält `EFireRate: ETechPower × 0.25` und `ERoundsPerSecond: ETechPower × 0.01`. `mechanics.rs:275` bewertet Spirit Power jedoch ausschließlich über Ability-Skalierungen; die Verbindung zum Waffenprofil fehlt. Die beiden neuen Item-Tests waren vor der Ergänzung rot. |

Fixbelege im Code-Commit; alle folgenden Dateiangaben liegen unter `rust/crates/dbrain-reasoner/src/`:

| Stelle | Resultierendes Verhalten |
| --- | --- |
| `data.rs:578`, `:606`, `:697`, `:792` | Die tatsächlich ausgewählten Assets-/Card-Snapshots liefern Feldwert, Quelle und Epoch-Zeitpunkt gemeinsam. Waffen-Fallbacks behalten den Zeitpunkt ihres Primärwaffen-Snapshots; Card-Proc-Cooldowns den Zeitpunkt ihrer Item-Card. Das aktuelle Helden-Modell stammt ausschließlich aus Assets-Snapshots, ohne zusätzliche ältere Profil-Skalierungen. |
| `data.rs:931`, `lib.rs:78`, `:153` | Die Fassaden laden auch die Patch-Events der tatsächlich modellierten Items und Abilities. Daher umfasst der neue Bericht mehr historische Belege als der alte, auf den Heldennamen beschränkte Lauf. |
| `patch.rs:67`, `:181` | Deduplizierung nach Patchtag, aufgelöster Entität und normalisierter Rohzeile. Der Kalendertag fasst die beobachteten unterschiedlichen Quellen-IDs desselben Updates zusammen; der früheste Zeitpunkt seiner Fassungen entscheidet konservativ. Zahlen aus einer eindeutigen `from/to`-Rohzeile haben Vorrang vor fehlerhaften extrahierten Spalten. Labels bestimmen kein Vorzeichen. |
| `patch.rs:159`, `:253` | Nur ein eindeutig passendes Modellfeld mit `posted_at > fetched_at` wird zugelassen. Gleichzeitige, ältere, undatierte oder nicht sicher zuordenbare Zeilen bleiben als „Nicht anwendbar“ im Bericht. Upgrade-/Per-Boon-Zeilen werden nicht auf Basiswerte geraten. |
| `patch.rs:263`, `:305`, `:392` | Anwendung als Faktor `new / old`, Prozentänderungen ebenfalls als Faktor. Einzelner und kumulierter Feldfaktor müssen zwischen 0,5 und 2 liegen. Vor Übernahme wird neu gescort: kein verändertes Ergebnis unter null, kein Score-Faktor außerhalb 0,5 bis 2. Verwerfungen behalten einen Beleg. Auch der bestehende öffentliche Apply-Einstieg verwendet diesen Schutz. |
| `lib.rs:90`, `composer.rs:77` | Patches wirken einmal auf Modellwerte. Die Fassade übergibt danach keine numerischen Patch-Boni an `score_items`; der Composer sortiert ausschließlich nach dessen `total`. Historische Belege bleiben getrennt verfügbar. |
| `composer.rs:121`, `:206`, `:213`, `lib.rs:483` | Kern nur bei positivem, endlichem Score; Optional maximal zwölf in Score-Reihenfolge. Nichtpositive finale Scores und Build-Items erhalten `Low`, auch bei vorhandenen Meta-Zeilen. |
| `item.rs:44`, `:122` | Ausschließlich die nachträglich autorisierte Verkettung ergänzt: Spirit Power verändert über die Helden-Skalierung die Feuerrate und damit den Waffen-DPS. Aktiv-, Passiv- und Kaufbonus-Beiträge fließen in die vorhandenen Score-Komponenten ein. Zustandsfaktoren und Gewichte bleiben bestehen; `mechanics.rs` ist unverändert. |
| `types.rs:308` | Additiver Snapshot-Anwendungsbeleg mit Feld, Ausgangswert, Quelle und beiden Zeitpunkten. Alte JSON-Objekte ohne diesen Beleg bleiben lesbar und werden nicht auf aktuelle Modelle angewendet. |

Die Grenze 0,5 bis 2 ist eine konservative Anwendbarkeitsgrenze, kein Scoring-Gewicht. Nicht eindeutig modellierte Effekte werden ausdrücklich verworfen. Die Quellen-Deduplizierung behandelt identische Zeilen am selben Patchtag konservativ als ein Ereignis; für eine feinere Unterscheidung mehrerer Updates am selben Tag fehlt eine gemeinsame Quellen-ID.

Der Nachtrag ist mit echten Snapshot-Werten geprüft. Wardens Primärwaffen-Snapshot vom 30.06.2026 liefert 17,34 Schaden, 3,809524 Schuss/s, 17 Schuss und 2,914 s Nachladen. Der unveränderte Damage-Plan beträgt 38,652068 Waffen-DPS, 25,318627 Spirit-DPS und 60,421523 Prozent Waffenanteil.

Für die numerische Verkettung wird der direkt in Schuss/s vorliegende Wert `ERoundsPerSecond` verwendet. `EFireRate` ist ausschließlich der Prozent-Fallback, wenn dieser direkte Wert fehlt; die beiden Darstellungen werden nicht addiert. Diese Priorität ist eine explizite Modellentscheidung. Die Werte selbst kommen aus dem gespeicherten Assets-Snapshot. Der [Upstream-Typ der Assets-API](https://github.com/deadlock-api/deadlock-api-assets/blob/master/deadlock_assets_api/models/v2/raw_hero.py#L121) übernimmt `scale` und `scaling_stat` aus den Spiel-Feldern `flScale` und `eScalingStat`.

Berechnung: `r_neu = r_snapshot + SpiritPower × 0,01`; `DPS = Schaden × Magazin / (Magazin / r + Nachladezeit)`. Der Kaufbonus wird zusätzlich berücksichtigt. Der Waffenbeitrag im Score folgt den bereits vorhandenen Zustandsfaktoren. Bei Mercurial Magnum gilt für den Item-Anteil weiter 0,604962; dessen Kaufbonus bleibt berücksichtigt.

| Item | Item-SP | Kaufbonus-SP | Gewinn Schuss/s, Item | Gewinn Schuss/s, gesamt | Zusätzliche Waffen-DPS im Score |
| --- | ---: | ---: | ---: | ---: | ---: |
| Mercurial Magnum | 7 | 13 | 0,070000 | 0,200000 | 1,073792 |
| Boundless Spirit | 30 | 13 | 0,300000 | 0,430000 | 2,612327 |
| Improved Spirit | 18 | 7 | 0,180000 | 0,250000 | 1,546427 |

Der Live-Test entfernt zur Gegenprüfung ausschließlich `EFireRate` und `ERoundsPerSecond` aus einer Kopie des geladenen Helden. Für jedes gescorte Item entspricht die Differenz des `total` exakt dem ausgewiesenen Waffenbeitrag, Toleranz `1e-9`. Doppelte Properties-/Passive-Einträge werden dabei einmal gezählt. Die zusätzliche Spirit-Wirkung auf Abilities bleibt erhalten.

Warden-Nachweis mit dem Debug-Binary aus diesem Worktree, Central-Verbindungen serverseitig lesend und mit `--no-persist`; alle Befehle Exit 0:

```text
reason build Warden --no-ai --no-persist --json
reason backtest --hero Warden --no-persist --json
reason patch-impact Warden --no-persist --json
```

Der Patch-Impact umfasst 7762 geladene Events und 3260 deduplizierte Belege: **0 angewendet, 3260 nicht anwendbar, 0 Items verschoben**. Die fünf Mercurial-Fassungen werden zu genau einem Beleg mit Vorzeichen +1, relativer Größe 0,053763440860215 und ohne Anwendung. Für sämtliche Items ist der aktuelle Score mit Patch-Verarbeitung exakt gleich dem Kontrolllauf ohne Patch-Anwendung.

Die folgende Vergleichsspalte reproduziert die reviewte Scoring-Basis ohne Patch, Meta und die neu ergänzte Spirit-Feuerrate. Differenzen zum aktuellen `total` bestehen ausschließlich aus vorhandener Meta-Unterstützung und der neuen Feuerraten-Verkettung. Die zehn in REPORT-D ausgewiesenen Basiswerte werden reproduziert.

| Kern-Item, aktuelle Reihenfolge | total | Review-B-Basis ohne Delta |
| --- | ---: | ---: |
| Spirit Burn | 105,304459 | 104,139994 |
| Juggernaut | 73,951349 | 73,451349 |
| Express Shot | 72,666751 | 72,665889 |
| Frenzy | 65,951358 | 65,451358 |
| Mystic Conduit | 65,321602 | 61,962848 |
| Frostbite Charm | 59,869616 | 58,870817 |
| Mystic Reverb | 55,856704 | 55,038547 |
| Rebuttal | 55,057760 | 54,557760 |
| Lucky Shot | 53,172100 | 52,672100 |
| Mercurial Magnum | 51,820777 | 50,246984 |
| Prism Blast | 49,166056 | 48,167256 |
| Diviner's Kevlar | 49,117971 | 46,180641 |
| Omnicharge Signet | 48,416301 | 47,417501 |
| Titanic Magazine | 47,655524 | 47,155524 |
| Unstable Concoction | 42,841613 | 40,564637 |
| Escalating Resilience | 38,243282 | 37,743282 |
| Boundless Spirit | 36,781131 | 33,668804 |
| Healing Tempo | 36,460735 | 35,960735 |
| Cheat Death | 35,778915 | 35,460310 |

Die Referenz-Erwartung wird teilweise erfüllt. Es gibt keine Sonderbevorzugung einzelner Namen:

| Referenz-Item | total | globaler Score-Rang | Ergebnis und Ursache |
| --- | ---: | ---: | --- |
| Juggernaut | 73,951349 | 2 | Im Kern, historische Waffen-Verzerrung entfernt. |
| Mercurial Magnum | 51,820777 | 10 | Im Kern, zusätzlich 1,073792 Waffen-DPS aus Spirit-Feuerrate. |
| Siphon Bullets | 28,649679 | 29 | Im Optional-Block. Unter dem letzten Kernwert 35,778915; unveränderte Mechanik-Basis 28,149679 plus 0,5 Meta. |
| Quicksilver Reload | 24,766835 | 37 | Unter Kern-Cutoff und dem begrenzten Optional-Cutoff. Basis 23,825812 plus 0,5 Meta und 0,441024 Waffen-DPS aus dem Kaufbonus. `Lane` ist nur die Kaufphase und schließt im Composer keinen Kernplatz aus. |
| Veil Walker | 14,981379 | 75 | Unter beiden Cutoffs. Basis 13,853285 plus 0,5 Meta und 0,628094 Waffen-DPS. Der Composer verwendet keine Sonderregel für diesen Referenznamen. |

Trophy Collector ist mit −1,843262, Golden Goose Egg mit −2,727461 und Spirit Sap mit −1,005723 aus dem Kern ausgeschlossen; alle drei haben finale Konfidenz `Low`. Spirit Burn bleibt aus der unveränderten, reviewten Mechanik oben, nicht aufgrund eines historischen Patch-Bonus.

Der Backtest enthält aktuell 42 Datenbank-Builds plus den Seed. Build 779996 wird für die Einzelmessung ausdrücklich über `hero_build_id=779996`, Version 45, geladen und durch denselben Autoren-Parser und dieselben Metriken ausgewertet. Es wird nicht über einen unklaren Autorennamen oder nur die Versionsnummer ausgewählt.

| Vergleich | Kern-Überdeckung | Jaccard | Reihenfolge-Nähe, 0 ist identisch | Patch-Wechsel |
| --- | ---: | ---: | ---: | --- |
| Seed Lightbringer x Situation | 0,210526 | 0,117647 | 0,150321 | nicht messbar |
| Build 779996, Version 45 | 0,210526 | 0,093023 | 0,362080 | nicht messbar |
| CLI-Aggregat, 43 Vergleiche | 0,176255 | nicht aggregiert | 0,268413 | nicht messbar |

Der Seed-Vergleich hat vier gemeinsame Kern-Items bei 19 Reasoner-Kern-Items und 34 Items in der Union. Der alte REPORT-D-Lauf hatte drei gemeinsame Kern-Items und Jaccard 0,085714. Für den zeitlichen Patch-Wechsel gibt es weiterhin keine belastbare Gegenüberstellung zweier Reasoner- und Autorenstände; historische Events werden dafür nicht auf einen aktuellen Snapshot zurückgerechnet.

Testzahlen, Baseline und Endstand:

| Prüfung | Baseline e958158 | Endstand 4434a52 |
| --- | --- | --- |
| `cargo test --workspace`, ohne DSN | 260 bestanden, 0 Fehler, 56 ignoriert | 273 bestanden, 0 Fehler, 58 ignoriert |
| Reasoner ohne DSN | 72 bestanden, 14 ignoriert | 85 bestanden, 16 ignoriert |
| Reasoner `--include-ignored`, Central lesend plus lokale Scratch-DB | nicht erneut vollständig gefahren | 101 bestanden, 0 Fehler, 0 ignoriert |
| `cargo clippy -p dbrain-reasoner --all-targets -- -D warnings` | vorheriger Stand laut Briefing grün | grün |
| Formatter nur eigene Dateien, jeweils mit Check | unverändert | grün |
| `cargo build -p deadlock-brain` | kein Release-Build | Debug-Build grün |
| Drei Debug-CLI-Läufe gegen Central, `--no-persist` | Live-Befund aus Briefing | alle Exit 0, JSON gültig |

Rot-Gegenproben zu allen neuen Regressionen:

| Gegenprobe | Ergebnis |
| --- | --- |
| Fünf erste Tests am unveränderten Patch-/Composer-Code | 0 bestanden, 5 fehlgeschlagen: Duplikate/Richtung, fehlender Snapshot-Bezug, negativer Kern, 41 statt 12 optionale Fixture-Items, doppelte Sortierwirkung. |
| Spirit-Verkettung vor Implementierung | 5 bestanden, 2 fehlgeschlagen: direkter RPS-Weg und prozentualer Fallback/Passivweg. |
| Zeitfilter vorübergehend deaktiviert | 5 bestanden, 3 fehlgeschlagen: Zeitgrenze, früheste Duplikat-Fassung, getrennte Item-/Card-Zeitpunkte. |
| Faktorgrenze vorübergehend auf 1000000 erhöht | 6 bestanden, 2 fehlgeschlagen: kumulierter Faktor und unplausible Einzeländerung. |
| Score-Prüfung vorübergehend übergangen | 0 bestanden, 1 fehlgeschlagen: plausibler Feldfaktor hätte negativen Item-Score erzeugt. |
| Profil-Skalierungen wieder zugemischt und Feuerraten-Beitrag deaktiviert | 11 bestanden, 4 fehlgeschlagen: Snapshot-Loader, beide Spirit-Unit-Tests und echter Warden-Nachweis. |
| Wiederhergestellter, abschließend geprüfter Code | 101 Reasoner-Tests bestanden; Workspace und Clippy ebenfalls grün. |

Ein erster Include-Ignored-Zwischenlauf hatte 88 bestandene und zwölf fehlgeschlagene Scratch-Tests, weil `PGOPTIONS=default_transaction_read_only=on` auch die lokale Wegwerf-DB schützte. Danach wurde die Lesebeschränkung verbindungsspezifisch in der Central-DSN belassen und für Scratch nicht vererbt. Der letzte vollständige Lauf oben ist grün. Sämtliche DDL-/Persistenztests liefen ausschließlich auf der eigens gestarteten lokalen DB `reasoner_a_fix` über Unix-Socket. Central-Zugänge erzwingen `default_transaction_read_only=on`; die CLI nutzt zusätzlich `--no-persist`. Kein KI-Aufruf.

Selbstprüfung abgeschlossen: Feldherkunft und Grenzzeitpunkte, Rohzahlen gegen fehlerhafte Extraktionen, Duplikate, Einzel-/Kumulativfaktor, negative Score-Ergebnisse, einmalige Patch-Anwendung, Finale-Konfidenz, Optional-Begrenzung, Snapshot-only-Heldenpfad und Spirit-/Feuerraten-Beiträge abgeglichen. Keine Änderung an `mechanics.rs`, keine neue Abhängigkeit, keine Code-Kommentare. Unabhängige Freigabe, Release-Build und Deployment bleiben beim Orchestrator.

Maschinenlesbare Belege im Task-Ordner:

- [WARDEN-E.json](WARDEN-E.json): alle Item-Scores, Kontrollwerte, Feuerraten-Beiträge und exakter Vergleich mit Build 779996.
- [WARDEN-E-BUILD.json](WARDEN-E-BUILD.json): unveränderte Ausgabe der Debug-Build-CLI.
- [WARDEN-E-BACKTEST.json](WARDEN-E-BACKTEST.json): unveränderte Ausgabe der Debug-Backtest-CLI.
- [WARDEN-E-PATCH-IMPACT.json](WARDEN-E-PATCH-IMPACT.json): Berichtssumme und deduplizierter Mercurial-Beleg aus der Debug-CLI.
