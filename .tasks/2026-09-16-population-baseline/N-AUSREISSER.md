# N-AUSREISSER: Warum der 04:34-Warden-Build schlecht war

Stand 2026-09-16, Branch `feat/reasoner-planner-produktiv`. Zweite Fassung: die
erste (stale Aggregate) ist widerlegt. Um 04:31 standen laut Koordinator bereits
10 Warden-Staples (`population stats` für alle Helden, `population show Warden`),
und der schlechte Build kam um 04:34 mit dem Release-Binary 57d3a07a (main
7277ff5, gebaut 04:26, mit `load_population_prior`).

## Urteil (erklärt, belegt)

Kein Rennen, keine stale Population, kein Lesefilter. Der 04:34-Build lief **mit
KI** (`used_ai=t`) und hat die Population korrekt gelesen. Ursache ist der
**KI-Kritiker-Recompose**: `run_critic` gab verdict `recompose`, und der
Recompose reicht `critic.issues` (Fließtext-Sätze des LLM) als `blocked` an
`compose_build_with_sources`. In `item_order` (`composer.rs`) wurde jedes Item
entfernt, dessen Name als **Teilstring** in irgendeinem `blocked`-Eintrag
vorkam. Der Kritiker nennt in seinen Sätzen Item-Namen ("die späten Items
Spiritual Overflow, Healing Tempo ..."), und genau diese Items wurden dadurch
still aus dem Kandidatenpool geworfen. Der Recompose fiel deshalb auf defensive
Items zurück (Extra Health, Duration Extender, Restorative Shot, ...), verlor die
Waffen-Staples und nahm sogar Enduring Speed wieder auf. Weil der Kritiker ein
LLM ist, passiert das nichtdeterministisch: mal recompose (04:34, schlecht), mal
nicht (05:06 und alle `--no-ai`-Läufe, gut). Der Build lieferte das degradierte
Ergebnis still aus.

## Belege

- Persistierte 04:34-Zeile in `brain.reasoner_builds` (hero_id 25, `used_ai=t`,
  Confidence `Low`, patch_tag = aufgelöste Steam-Announcement-URL): der Kern ist
  Extra Health, Rapid Rounds, Swift Striker, Fleetfoot, Duration Extender,
  Restorative Shot, Enduring Speed, ... . Die Zeile trägt "Populations-Stütze"-
  Evidence, kann also nicht aus der Zeit vor dem Populations-Ingest stammen; sie
  ist der 04:34-Build.
- Population **wurde gelesen**, nicht leer: Evidence wörtlich (6 Erwähnungen,
  je Staple doppelt gelistet) im `all`-Bucket:
  - "Populations-Stütze: 85% Kaufanteil bei echten Spielern dieses Helden,
    Median-Kaufposition 6." (Swift Striker)
  - "Populations-Stütze: 85% ... Median-Kaufposition 10." (Fleetfoot)
  - "Populations-Stütze: 79% ... Median-Kaufposition 9." (Enduring Speed)
  Der Build sah also den `all`-Bucket mit Staples; nur drei landeten im Kern
  (das gute Build hat neun, daher 18 Erwähnungen). Die Population war nicht dünn,
  die Auswahl war es.
- Die Rationale enthält "Offene Kritikpunkte:" (Position 10088) und zitiert
  "Spiritual Overflow, Healing Tempo", die im finalen Kern **nicht** stehen.
  Dieser Text wird nur nach einem `recompose` angehängt. Also lief ein Recompose,
  der genau die vom Kritiker erwähnten Items entfernte.
- `item_order` filterte mit
  `issue.contains(&lower(&item.item.name)) || issue.contains(&id)`: Teilstring
  eines Fließtexts genügte zum Blocken.
- `--no-ai reason build Warden` liefert in fünf Läufen zeichengleich das gute
  6/9-Build. Ohne Kritiker gibt es keinen Recompose und keine Degradierung.

## Ausgeschlossen (aus der ersten Fassung)

- Stale Aggregate / dünne Population: widerlegt, die Population stand und wurde
  gelesen (drei Staples mit Evidence).
- Torn Read / Rennen während `population stats`: `write_hero` ist atomar je Held
  (ein Commit); und der Bad-Build hat gelesene Staples, kein Leerstand.
- Lesefilter: `PopulationIndex::load` liest ohne `updated_at`/`patch_tag`/
  `run_id`-Filter alle Buckets je `hero_id`; `load_population_prior` nutzt fest
  `BUCKET_ALL`. Kein Filter schneidet Staples weg.
- hero_id-Typ/Bucket: `hero_id 25` trägt in `population_item_stats` 10 Staples im
  `all`-Bucket; genau dieser wurde gelesen.

## Fix und Härtung

1. **Ursache (`composer.rs` `item_order`):** `blocked` blockt nur noch bei
   exaktem, case-insensitivem Namens- oder ID-Abgleich, nicht mehr per Teilstring
   in Fließtext. Ein Kritiker-Satz, der ein Item erwähnt, entfernt es nicht mehr.
   Test `free_text_issue_does_not_block_a_mentioned_item_only_exact_names_do`
   (Rot-Gegenprobe: mit dem alten Teilstring-Verhalten blockt der Fließtext beide
   Items).
2. **Sichtbarkeit (`population_prior.rs`, `composer.rs`):**
   `PopulationPrior::thin_coverage_note` meldet, wenn ein Build weniger als die
   Hälfte der Populations-Staples abdeckt; `compose_build_with_sources` hängt den
   Hinweis in die Rationale und setzt Confidence `Low`. Eine dünne
   Populations-Deckung steht damit sichtbar im Build statt still ein anderes
   Ergebnis zu liefern. Test
   `thin_coverage_note_fires_only_below_half_of_the_staples`.

## Nicht reproduziert

Schritt (3) des Auftrags (Race während `population stats`, paralleler Build)
zielt auf die widerlegte Rennen-Hypothese und ließe sich nur mit Schreibzugriff
auf eine Populations-DB nachstellen; die zentrale DB ist hier nur lesend. Die
echte Ursache ist der KI-Recompose und wurde am persistierten Build und am Code
belegt, nicht über eine Race-Reproduktion. Ein Nachstellen des Recompose bräuchte
KI-Aufrufe (im Auftrag ausgeschlossen).
