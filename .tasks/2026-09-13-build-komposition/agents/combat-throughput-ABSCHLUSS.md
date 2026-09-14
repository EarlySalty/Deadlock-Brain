# BR-05a Abschluss: Combat-Throughput-Regression aus 6f40f03

Stand: 14.09.2026. Branch `feat/build-reasoner-interactions-audit`,
Fix-Commit `823b0e5` (Basis `b29db00`). Werkzeugstand 1.97.1.
`cargo test -p dbrain-reasoner --lib`: 167 bestanden, 0 Fehler,
16 ignoriert. `cargo clippy -p dbrain-reasoner --all-targets -- -D warnings`:
bestanden. Warden dient nur als Regressionserkennung; im Produktionscode
stehen keine Referenz-Itemnamen und keine heldenspezifischen Gewichte.

## Ursache

6f40f03 führte Zieltod, Ziel-HP-Capping und die Kettenziele unter Druck ein
und bewertete den Schadensdurchsatz auf tatsächlich verstrichene Kampfzeit.
Derselbe Commit reskalierte zugleich das effektive Leben um
`Fenster / Kampfzeit`, wenn Duell oder bewegliches Ziel vorzeitig beim
Zieltod enden. Der so hochgerechnete Bestand wurde anschließend durch das
volle Kampffenster geteilt (`survival_score = effective_health / window`).

Das effective health war damit kein Zeitmittel mehr über das Kampffenster,
sondern ein Zeitmittel über die kurze Kampfphase am Szenarioanfang. In
dieser Phase sind Aktivierungsboni, Barrieren und Schildzustände voll
wirksam, später nur noch in ihren Uptime-Fenstern. Der Überlebensbeitrag
bewertete dadurch den Spitzenzustand des Kampfbeginns statt des
Fenstermittels. Schnellere Kills verkürzten die Mittelungsphase und erhöhten
die Überlebensbewertung, obwohl die ausgewiesene Annahme genau dies
ausschließt: kurze TTK erhöht ihr Gewicht nicht.

Gemessen an identischen Eingaben (Warden, eingefrorene Itemmodelle aus
`FROZEN-V2.json`, zwei feste Endinventare, Rohheld ohne Progression):

| Inventar | bcc3f6e | b29db00 (kaputt) | 823b0e5 (Fix) |
|---|---:|---:|---:|
| Waffenlastiges Inventar | 589,3 | 331,4 | 298,3 |
| Widerstandslastiges Inventar | 335,8 | 502,1 | 262,0 |

Das kaputte Scoring drehte die Rangfolge vollständiger Inventare um. Die
Marginalwerte zeigten die Mechanik: Vitality- und Sustain-Kandidaten stiegen
auf 90 bis 137 Prozent über den bcc3f6e-Stand (Bullet Resilience 42,0 auf
89,5, Weapon Shielding 43,4 auf 81,7, Divine Barrier 15,7 auf 61,2,
upgrade_frenzy 119,5 auf 273,6), während kleine Waffenstat-Verbesserungen
auf Null oder darunter fielen (upgrade_clip_size_fixed 9,8 auf -1,4,
Lucky Shot 2,1 auf -1,6). Glass Cannon v2 auf dem Widerstandsinventar
fiel von +103,6 auf -52,9. Der Planer folgte diesen Marginalwerten, kaufte
Widerstand statt Waffendurchsatz, und die Waffen-Kerntreffer für Warden 779996
fielen von 4/9 (bcc3f6e) auf 1/9.

Die HP-Caps, Kill-Zeiten, Schwellen-Resets und Aura-Regeln von 6f40f03
selbst waren rechnerisch korrekt: Schaden wird an jeder Stelle vor
Wertgabe am verbleibenden Zielleben begrenzt, ohne Doppelzählung des
Zielpools; Unterzählung bestand keine.

## Geänderte Mechanik

Eine Stelle in `simulate()` (`rust/crates/dbrain-reasoner/src/combat.rs`):

- Vorher: `effective_health = health_sum * window / elapsed` bei Zieltod,
  ohne Hochrechnung bei eigenem Tod.
- Nachher: `effective_health = health_sum` in allen Endgründen. Das
  effektive Leben ist der zeitlich gemittelte Ressourcenbestand der
  tatsächlich gekämpften Fensterzeit; das Restfenster eines gewonnenen
  Duells trägt nichts nach, das Restfenster eines eigenen Todes bleibt als
  Wirkungsausfall unbelohnt.

Unverändert bleiben: Zieltod als Szenarioende in Duell und beweglichem Ziel,
Kettenziele mit 1,0 s Wechselpause unter Druck, HP-Capping inklusive
Schussbegrenzung am Restleben, Schwellen- und Zielwechsel-Resets,
Aura-Fortbestand über Zielwechsel, Schadensdurchsatz auf tatsächlicher
Kampfzeit, Selbsttod mit Restfenster-Abschlag. Der Fix ist helden- und
itemneutral; er ändert nur die Zeitbasis des Überlebensbeitrags.

## Generische Gegenproben

- Neu: `faster_kills_earn_damage_credit_but_not_survival_credit`. Gleicher
  Held, zwei Kugelschadensstufen, beide töten im Fenster. Der schnellere
  Kill erhält vollen Schadenskredit (250 auf 750 DPS, TTK 2,4 s auf 0,8 s),
  die Überlebensbewertung fällt dabei (72,0 auf 24,0 effektives Leben,
  Survival 3,6 auf 1,2). Killgeschwindigkeit und Überlebenskredit sind
  entkoppelt.
- Vertragstest `target_death_ends_duel_and_does_not_reset_own_cooldown`
  pinnt die neue Konvention exakt: effektives Leben 30,0, Survival 7,5 im
  4-s-Fenster nach Kill bei 0,2 s.
- `max_health_loss_applies_to_bonus_health_and_healing_cap` und der
  data.rs-Test zur Glass-Cannon-Rückrechnung prüfen ihre Bestandsformel
  jetzt in einem killfreien Kurzfenster und bleiben darum von der
  Kill-Zeitquantisierung unabhängig.
- Bestehende Gegenproben zu Zieltod, Kettenzielen, Aura-Fortbestand,
  Magazin-Imbue, Selbsttod und Fast-/Detailed-Parität laufen unverändert
  grün; der Druck-Szenarioanteil des Scorings ist von der Änderung
  unberührt, weil dort Kampfzeit gleich Fensterzeit ist.

## Warden Vorher/Nachher

Referenz Warden 779996, Version 45, 19 Kernitems, davon 9 Waffen.
Messgrundlage jeweils unverändert `FROZEN-V2.json`
(SHA256 `5451b0b4f3cde928f12f09a4ea5f4fa3128c93fd8ca227f8208a1357f6cb7b1a`),
Example `build_evaluation`, Modus `plan`, reine Release-Läufe.

| Stand | Revision | Waffen-Kerntreffer | Referenz-Recall | Jaccard |
|---|---|---:|---:|---:|
| Dokumentierter Stand vor der Regression | bcc3f6e-Umfeld | 4/9 | 0,263158 | 0,166667 |
| Integrationsbasis dieses Pakets | b29db00 | 1/9 | 0,052632 | 0,029412 |
| Nach Fix | 823b0e5 | 3/9 | 0,315789 | 0,206896 |

Nach dem Fix trifft der Plan Titanic Magazine, High-Velocity Rounds und
erstmals Opening Rounds. Opening Rounds ist der konkrete Fall, für den das
Ziel-Lebensmodell nach der Diagnose in URSACHEN-KAUFKURVE gebaut wurde:
`EnemyLifeThreshold` greift am echten Restleben und verliert den bedingten
Waffenbonus am toten Ziel. Das Endinventar enthält wieder durchgehend
Waffendurchsatz (Rusted Barrel, Quicksilver Reload, Active Reload,
Mercurial Magnum, Glass Cannon, High-Velocity Rounds), der frühere
Ausreißer im effektiven Leben ist weg (Endbewertung 3934 gegenüber 35659
im kaputten Stand bei gleichem Held).

Nachweisdateien im Documents-Ordner
`2026-09-13-build-reasoner-ganzbuild/nachweise/`, nicht eingecheckt:
`COMBAT-FIX-HEAD-b29db00-WARDEN.json` (kaputt),
`COMBAT-FIX-AFTER-823b0e5-WARDEN.json` (Fix),
`COMBAT-FIX-PARITY-HEAD-b29db00.json` (228 Inventarauswertungen der
Kaputt-Basis).

## Grenzen und offene Punkte

- Der Fix stellt die Mechanik her, nicht die Pflicht von 5/9: Warden liegt
  nachher bei 3/9, der dokumentierte alte Stand bei 4/9. Der Restabstand
  hat nach jetzigem Messbild drei Quellen ohne neuen Fehlerbefund:
  1. Die feste 1,0 s Wechselpause begrenzt den gutgeschriebenen Durchsatz
     auf Zielpool pro (TTK plus Pause) und bestraft zunehmenden DPS
     zunehmend; das ist die in URSACHEN-KAUFKURVE gewählte
     Kettszenario-Konvention, kein Rechenfehler.
  2. Duell und bewegliches Ziel enden beim Zieltod und messen deshalb nur
     die Eröffnungsphase. Spät anlaufende Effekte wie der RampUp-Auslöser
     von Swift Striker kommen dort nicht mehr zum Zug; im Kaputt-Stand
     galten sie über die Reskalia indirekt als Überleben.
  3. Zweitrangiges Ausrichtungsrauschen: kleine Munitions- und
     Magazinverbesserungen können über Neuausrichtung von Nachladen und
     Restschaden am letzten Teilsziel marginal negativ landen
     (beobachtet -1,4). Der Effekt ist klein gegen reguläre
     Marginalwerte, ist aber nicht monoton.
- Sechs-Helden-Plausibilität, Holdout und Sensitivität sind hier nicht
  gelaufen und bleiben beim Orchestrator; dieses Paket umfasst nur die
  mechanische Korrektur samt Warden-Erkennungsmessung.
- Das Anzeigefeld `spirit_power` rechnet weiterhin auf Kampfzeit hoch, ist
  aber nicht Bestandteil des Scorings und der Planerisierung.
- Die Diagnoseprobes (Sweep über alle Kandidaten, 2x2-Inventarmatrix) waren
  einzige untracked Hilfsdateien und sind vor dem Commit entfernt; die
  Zahlen in diesem Bericht stammen aus den genannten Nachweisdateien.
