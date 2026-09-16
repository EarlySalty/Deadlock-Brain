# FERTIG-FIX-M2: Fixrunde 2 Paket M (REVIEW-M)

Branch `feat/build-reasoner-population`, neue Commits, kein amend. Urteil war
NACHBESSERN (0 blockierend, 4 wichtig, 3 Nits).

## Commits

- `4385392` fix(reasoner): Staple-Hebel an nicht-negativen Kontext-Marginalwert binden
- (dieser Commit) docs(m): Neumessung Runde 2, Nachweise, Spielerzahl, Fertigmeldung

## Mängel aus REVIEW-M

1. **POPULATION_DB_DSN (wichtig).** Erledigt (schon in Fixrunde 1, Commit
   `09198ff`). Das Mess-Example nutzt `deadlock_brain_core::pg::pg_pool()` über
   `DEADLOCK_CENTRAL_DSN`, kein zweiter Env-Name mehr. Für die Messung zeigt das
   DSN auf die Wegwerf-DB `population_dev` (nachher) bzw. auf eine DB ohne
   Populationstabellen (vorher). Fehlende Population ist im Produktivpfad sichtbar
   (Evidence-Zeile "Population: keine Daten", `to_regclass`-Guard).
2. **Staple-Hebel am Kontext-Marginalwert (wichtig).** Erledigt (Commit
   `4385392`). `is_priority_staple_step` bindet die kategorische Priorität und die
   Umgehung der Marginal-Untergrenze zusätzlich an `marginal_value >= 0` im
   aktuellen Inventar (Auswahl, Beam, Spar-Entscheidung). Ein solo-positiver, im
   Kontext aber anti-synergetischer Staple wird nicht mehr erzwungen.
3. **Begründung ehrlich (wichtig).** Erledigt (Commit `4385392`). Der
   Populations-Stütze-Text nennt die echte Bedingung (positiver Solo-Mechanikwert
   plus nicht-negativer Kontext-Marginalwert) statt "tragfähiger Mechanikwert".
4. **Nachweis-Zeiger (wichtig).** Erledigt. M-MESSUNG zeigt für die 6/9 auf
   `M-SECHS-pop.json`; die irreführenden Einzelläufe `M-WARDEN-pop*.json` und
   `M-WARDEN-nopop.json` sind entfernt.
5. **Spielerzahl (nit).** Erledigt. M-MESSUNG und FERTIG-M nennen 119248
   Spieler-Matches über 10059 Ranked-Matches (38 Helden); 80641 als älterer
   Ingest-Zwischenstand benannt.
6. **build.rs-Sentinel (nit).** Kein Änderungsbedarf, bewusst so (übliches
   Cargo-Idiom, Name trägt den Zweck).
7. **Commit-Betreff-Umlaute (nit).** Ab Fixrunde 1 echte Umlaute in Betreffzeilen.

## Neumessung Runde 2 (gegateter Hebel, FROZEN-V2, Modus plan, Debug)

Vorher = leerer Prior, Nachher = `population_dev`.

- Warden: 3/9 → **6/9** Referenzwaffen (≥ 5/9 erfüllt), Recall 0,316 → 0,421,
  Kendall tau 0,464, Jaccard@12 0,500.
- Sechs weitere Helden: kein Held verschlechtert sich gegenüber seiner
  Vorher-Basis; Infernus 3/7→5/7, Lady Geist 1/7→2/7, Abrams 0/7→3/7, Vindicta
  1/3→2/3, Bebop 0/4→0/4, Ivy 2/3→2/3. Volle Tabelle in M-MESSUNG.md.

## ABWEICHUNG (Warden-Staple-Gate)

Durch den Marginal-Gate fällt Warden das Staple-Gate: Enduring Speed (Item
2447176615, Vitality Tier 2, 75,8 % Kaufanteil, niedrigster der zehn Staples)
hat im Kampfmodell einen strikt negativen Kontext-Marginalwert und wird nicht
mehr erzwungen. Das ist der berechtigte Kern von REVIEW-M Mangel 2 (kein
Erzwingen anti-synergetischer Items); der unmodellierte Nutzen von Enduring
Speed liegt in Mobilität und Ausdauer. Das AUFTRAG-Kriterium "Staple-Gate
bestanden" ist damit für Warden nicht mehr erfüllbar, ohne am Maßstab zu drehen;
die Waffen-Latte (6/9 ≥ 5/9) bleibt erfüllt. Die übrigen neun Warden-Staples und
die Staple-Gates der anderen getrackten Helden bleiben bestanden.

## Tests

- `cargo test -p dbrain-reasoner --lib`: 170 bestanden, 16 ignoriert, 0 rot.
- `cargo test -p dbrain-population`: 24 bestanden, 0 ignoriert, 0 rot.
- Workspace ohne DSN: 386 bestanden, 58 ignoriert, 0 rot.
- `cargo clippy -p dbrain-reasoner -p dbrain-population -p deadlock-brain
  --all-targets -- -D warnings`: sauber. Kein `--release`.
