# REVIEW-E (Runde 1)

**Urteil: FREIGABE.**

Diff `e958158..4434a52`, Branch `fix/build-reasoner-patch-delta`, Worktree
`deadlock-brain-e`. Lesend geprüft, kein Code geändert. Reviewer ist nicht der
Implementierer.

## TLDR

Der Fix bindet Patch-Deltas sauber an das Snapshot-Feld, aus dem der Wert
stammt, dedupliziert nach Patchtag und Rohzeile, zieht Vorzeichen und Faktor aus
den Zahlen statt aus dem Label und begrenzt Einzel- wie Kumulativfaktor auf 0,5
bis 2. Composer und Fassade wenden den Patch genau einmal auf den Modellwert an.
Die Spirit-Feuerrate ist additiv und ohne Doppelzählung im Item-Score. Tests,
Clippy und die Nachweiszahlen stimmen. Es bleiben zwei nicht blockierende Punkte
für Paket F.

## Nachweislauf (Reviewer)

```
TESTNACHWEIS[TW-1]: 273 passed, 58 ignored | Baseline: 0 rot
```

- `cargo test --workspace` ohne DSN: 273 bestanden, 0 Fehler, 58 ignoriert
  (Exit 0). Deckt sich mit der Fixer-Angabe.
- `cargo clippy --workspace --all-targets -- -D warnings`: grün (Exit 0).
- `cargo test -p dbrain-reasoner -- --include-ignored`: **nicht gefahren**, in
  dieser Session ist keine Central-DSN gesetzt und nicht beschaffbar. Die 58
  ignorierten Tests (DB-Pfad) sind damit vom Reviewer nicht unabhängig belegt.
  Fixer meldet 101 bestanden mit lokaler Scratch-DB. Gilt als offener
  Fremdnachweis, kein Mangel am Code.
- Nachweis-JSON stichprobenartig gegen `FERTIG-E.md` geprüft: Kern 19 Items
  (Spirit Burn, Juggernaut, Express Shot vorn), Optional exakt 12, Juggernaut
  73,951349, Mercurial Magnum 51,820777, Backtest Kern-Überdeckung 0,210526
  (Seed und 779996), Patch-Impact „3260 deduplizierte Belege, 0 angewendet,
  3260 nicht anwendbar“. Alle Werte konsistent.

## Prüfpunkte gegen das Briefing

1. **patch.rs Zeit- und Dedup-Logik**: `posted_at > fetched_at` wird in
   `compute` (Zeile 253) und erneut in `apply_one` (Zeile 309) erzwungen; nur
   genau ein eindeutig passendes Feld darf anwenden (Zeile 248). Vorzeichen und
   Faktor kommen aus `values()` (Rohzeile `from/to` vor Spalten), nicht aus
   `change_type`. Prozentänderung wird als Faktor `(100±x)/100` behandelt.
   Einzel- und Kumulativgrenze 0,5 bis 2 vorhanden (Zeile 314, 355), Ergebnis
   nie unter null (Zeile 352).
2. **Kalendertag-Frage (aus dem Briefing)**: geprüft, unkritisch. Der
   Dedup-Schlüssel ist `(Patchtag, Entität, normalisierte Rohzeile)`. Zwei echte
   Updates am selben Tag haben unterschiedliche `from/to`-Werte und damit
   unterschiedliche Rohzeilen, werden also nicht verschluckt. Nur textgleiche
   Zeilen desselben Tages fallen zusammen; das ist konservativ gewollt.
3. **„Frühester Zeitpunkt der Fassungen“ (aus dem Briefing)**: unkritisch. Das
   Minimum wird nur innerhalb einer Gruppe gebildet, und die Gruppe teilt sich
   denselben Kalendertag. Der frühere Zeitpunkt macht die „posted <= fetched“
   -Historienregel strenger, nicht laxer, kann also nichts fälschlich anwenden.
4. **data.rs**: Snapshots liefern Feldwert, Quelle und `fetched_at` gemeinsam;
   Waffen-Fallback erbt den Zeitpunkt des Primärwaffen-Snapshots, Card-Proc-
   Cooldown den der Item-Card. Heldenmodell kommt aus Assets-Snapshots ohne die
   alten Profil-Skalierungen (Stats-Parameter auf `&[]`). Loader lädt jetzt auch
   Events der modellierten Items und Abilities.
5. **item.rs Feuerrate**: `ERoundsPerSecond` direkt, `EFireRate` nur als
   Prozent-Fallback über `or_else`, nie beide addiert. Aktiv-Spirit schließt in
   `passive_properties` vorhandene Schlüssel aus, daher keine Doppelzählung.
   `weapon_dps_in_score` und die Score-Komponenten sind so aufgeteilt, dass der
   Test die Gesamtdifferenz exakt auf den Einzelbeitrag festnagelt (`total` -
   Diff == erwarteter Waffen-DPS). `mechanics.rs` ist unverändert (nicht im
   `--stat`).
6. **composer.rs und lib.rs**: Delta wirkt einmal auf den Modellwert;
   `score_items` bekommt überall `&[]`, der alte Score-Bonuspfad ist damit tot.
   Sortierung nur nach `total`, `patched_score` entfernt. Kern nur bei
   positivem endlichem `total` (`item.score.total > 0.0`, `is_finite`), Optional
   auf 12 gekappt, Konfidenz erzwingt `Low` bei `total <= 0` sowohl in
   `finish_scores` als auch in `build_item`.
7. **types.rs**: `application` mit `#[serde(default)]`; alte JSON-Zeilen ohne das
   Feld bleiben deserialisierbar und werden mangels `application` nie angewendet.

## Befunde

Keine blockierenden Mängel.

### Wichtig (für Paket F, keine Live-Wirkung heute)

- **patch.rs:411 bis 425, Score-Wächter zu streng.** Der Kumulativ-Check
  verwirft ein Delta, sobald irgendein Item durch die Änderung von einem
  Basis-Score 0 auf einen positiven Wert wechselt (`old > 0.0` ist Pflicht für
  jedes geänderte Item). Bei einem echten, neueren-als-Snapshot Helden-Patch
  kann so ein gültiges Delta still fallen, weil ein unbeteiligtes Null-Score-Item
  mitwandert. Heute ohne Folge, da für Wardens aktuelle Daten ohnehin 0 Deltas
  anwendbar sind. Vorschlag: den Score-Check auf Items mit Basis-Score > 0
  beschränken und einen Übergang 0 auf positiv als zulässiges Wachstum werten,
  statt das ganze Delta zu kippen.

### Nit

- **patch.rs:32 bis 38 und 67 bis 92, Komma-Dezimalzahlen.** `numeric_tokens`
  trennt an Kommas, „0,49“ liefert zwei Token und scheitert im `from/to`-Pfad.
  Für die aktuellen Quellen ohne Belang, die Deadlock-Changelogs nutzen Punkte
  (Mercurial 0.465 auf 0.49 wird korrekt zu relativer Größe 0,053763 aufgelöst).
  Latente Annahme, nur relevant, falls je eine Quelle Komma-Dezimalzahlen
  liefert.

## Kalibrierung, nicht Teil von E

Siphon Bullets, Quicksilver Reload und Veil Walker bleiben unter dem
unveränderten Kern-Cutoff. Geprüft: keine gedrehten Gewichte, keine Sonderregel
auf einzelne Item-Namen im Composer. Die Ursachen sind Cutoff und Meta,
Kalibrierungsthema für Paket F.
