# Community-Sheet: Modell und Befunde

status: erledigt für Sheetanalyse, 07.10.2026. Rust-Umsetzung und Laufzeitabgleich offen.

Vollständiger öffentlicher XLSX-Export, keine Produktänderung, kein Git-Schritt des Sheetworkers, kein Datenbankschreiben und kein Dienstaufruf. Die ergänzenden API-Abrufe betreffen öffentliche Spielwerte.

## Ergebnis und Ablagegrenze

Alle 13 Tabs wurden im unveränderten XLSX untersucht, einschließlich Formeln, Shared-Formula-Ankern und gecachten Werten. Im Workbook sind keine versteckten Tabs vorhanden. Vier versteckte Named Ranges gehören zu Filtern von Heroes. Für jeden Tab liegt ein echter Browserbildnachweis vor. Bei raw_hero_data und hero query ist nur A1:T20 über Googles unformatiertes GViz-HTML visuell geprüft; die vollständige Originalansicht lief in ein Zeitlimit. Diese Einschränkung betrifft die Darstellung, nicht den vollständigen XLSX-Export.

Das Sheet eignet sich als Rechen- und Vergleichsreferenz, nicht als ungeprüfte aktuelle Wahrheit. Es vermischt manuelle Waffenwerte mit importierten Heldendaten und manuell gesetztem Spirit. Es enthält fünf als Text gespeicherte DNS-Fehler und drei kaputte Scratchpad-Formeln. DPM heißt ausdrücklich Schaden pro Magazin. Das Sheet berechnet keine allgemeine Distanz-Falloff-Kurve und keine belastbare TTK mit Nachladezyklen. Meta-Rollenwerte sind persönliche Bewertungen.

Die Isolation des Workers verweigerte Write auf den beauftragten gemeinsamen Pfad. Dieser Bericht liegt in der eigenen Worktree-Akte, gemäß Hauptsteuerung 05:40 der bestätigte Übergabeort. Rohexporte und Browserbilder bleiben unter dem ursprünglich zugewiesenen gemeinsamen G/sheet-Pfad. REGISTER, TODO und AN_HAUPT-G wurden vom Sheetworker nicht verändert.

## Quellen und vollständiger Umfang

Gemeinsames Beweisverzeichnis:

`/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-06-brain-abschluss/G/sheet/`

- Quelle: https://docs.google.com/spreadsheets/d/1fj9XMQmVUY0FY4cbozvB18PMBnpbdsaMFTZRLa74VRY
- Unveränderter Export: `community-sheet.xlsx`, 7.312.248 Bytes.
- Abruf abgeschlossen: `2026-10-07T03:13:15Z`.
- SHA-256: `992d0e5914036a3f4e4de159f91bae9569bf81aaae9c2222242e95da8253a8c0`.
- Abrufadresse: Quelladresse mit `/export?format=xlsx`.
- Originaler HTML-Container mit Tabnamen und gids: `online-htmlview.html`.
- Workbook enthält 3.259.414 XML-Zellen einschließlich leerer formatierter Zellen, davon 13.034 mit nichtleerem Wert oder Formel. Es enthält 5.307 Formelzellen.
- Jede Formelzelle hat ein XML-v-Element. 20 Formelzellen haben einen leeren gecachten Text: 17 in damage calculator, 2 in Hero meta ranking und 1 in raw_items_and_abilities. Das ist kein Beleg einer fehlenden Formel.
- 0 Zellen tragen den Excel-Fehlertyp `t=e`. Trotzdem sind 8 Fehlerstrings vorhanden. Eine Prüfung nur auf `t=e` würde sie übersehen.

Die vollständigen Formeln und gecachten Werte sind in `xl/worksheets/sheet1.xml` bis `sheet13.xml` im XLSX enthalten. Strings werden über `xl/sharedStrings.xml` aufgelöst. Viele Folgezellen haben ein leeres f-Element mit `t=shared` und `si`; ihre Formel ergibt sich durch relative Verschiebung des zugehörigen Ankers. Die hier genannten Shared-Formeln sind mit ihrer echten Ankeradresse wiedergegeben, nicht als vermeintlich wörtliche Folgezellenformeln.

| Nr. | Tab im Browser | gid | XML-Zellen | Wert-/Formelzellen | Formelzellen | Formeldefinitionen mit Text | Bild |
|---|---|---|---:|---:|---:|---:|---|
| 1 | Heroes | 0 | 46.248 | 1.257 | 715 | 379 | 01-heroes.png |
| 2 | damage calculator | 339408087 | 26.257 | 532 | 259 | 98 | 02-damage-calculator.png |
| 3 | Hero meta ranking | 491256401 | 48.982 | 828 | 78 | 41 | 03-hero-meta-ranking.png |
| 4 | Damage Comparison | 339734772 | 26.000 | 450 | 272 | 24 | 04-damage-comparison.png |
| 5 | Hidden Mechanics | 1534505579 | 27.985 | 340 | 33 | 21 | 05-hidden-mechanics.png |
| 6 | Boons/AP | 1183373865 | 25.974 | 151 | 35 | 1 | 06-boons-ap.png |
| 7 | shopBonuses | 1334028442 | 29.000 | 235 | 87 | 9 | 07-shop-bonuses.png |
| 8 | ttk | 1738306151 | 26.000 | 105 | 34 | 34 | 08-ttk.png |
| 9 | haze | 1741510172 | 26.993 | 4.131 | 3.643 | 153 | 09-haze.png |
| 10 | scratchpad | 1251102836 | 26.000 | 194 | 105 | 55 | 10-scratchpad.png |
| 11 | raw_hero_data | 1057880283 | 1.681.848 | 1.827 | 1 | 1 | 11-raw-hero-data.png |
| 12 | raw_items_and_abilities | 1854999973 | 42.354 | 2.937 | 1 | 1 | 12-raw-items-and-abilities.png |
| 13 | hero query | 265096549 | 1.225.773 | 47 | 44 | 44 | 13-hero-query.png |

Alle Tabs haben `state=visible`. Der XLSX-Name von Boons/AP ist `BoonsAP`, da Excel keinen Schrägstrich in Blattnamen zulässt. Die XML-Dateinummer entspricht der Nr. der Tabelle.

### Named Ranges und strukturierte Bezüge

Alle vier Defined Names stehen mit `hidden=1`, `localSheetId=0` in `xl/workbook.xml`:

1. `_xlnm._FilterDatabase`: `Heroes!$A$5:$AS$50`.
2. `Z_2FD415A8_8AB2_477E_A63D_1C69109448D8_.wvu.FilterData`: `Heroes!$A$5:$Y$33`.
3. `Z_BBF4F274_B3E9_4860_860B_AB7C398A906A_.wvu.FilterData`: `Heroes!$A$5:$Y$27`.
4. `Z_AD253C04_E77F_43DF_8E7F_C1F494BDB6AF_.wvu.FilterData`: `Heroes!$A$5:$Y$27`.

`Table1[Hero Name]` wird in Heroes!O3 gezählt. `Table2` wird in den QUERY-Formeln der Meta-Tiers benutzt. Das sind strukturierte Tabellenbezüge, keine zusätzlichen Named Ranges. Der Export enthält außerdem zwei Chart-XMLs und eine eingebettete PNG-Datei.

### Formelabhängigkeiten

- Öffentliche Hero-API nach raw_hero_data über IMPORTJSONAPI; raw_hero_data nach Heroes über VLOOKUP.
- Heroes nach damage calculator, Damage Comparison, ttk und haze.
- raw_hero_data sowie raw_items_and_abilities nach hero query.
- raw_items_and_abilities importiert die öffentliche Item-API.
- hero query ruft für Waffen und Fähigkeiten einzelne Item-URLs auf. Der Rechenblock des damage calculator ist nicht vollständig aus diesem Abfragetab verkabelt, sondern enthält manuelle Auswahlwerte und Basisschäden.
- Hidden Mechanics, Boons/AP, shopBonuses, Hero meta ranking und scratchpad haben keine echten externen Blattbezüge in ihren Formeldefinitionen. `#REF!` im Scratchpad ist ein kaputter Bezug, kein Blatt namens REF.

## 1. Heroes

Visuell: schwarze Heldentabelle, farbige Vergleichszellen, Eingaben Total Boons und Hero Spirit oberhalb. Eine sichtbare Zeile pro Held. Der Ausschnitt zeigt auch unfertige Hero-Labs-Zeilen ohne Waffenwerte.

Eingaben: L3=35 Boons, M3=38 Spirit. O3 zählt 45 Helden über `COUNTA(Table1[Hero Name])`. Waffenwerte in B:G sind überwiegend manuell. M:Q sowie S:U kommen per VLOOKUP aus raw_hero_data. J/K sind Falloff-Grenzen. AE:AJ beschreiben spezielle Spirit-Effekte; AK:AM sind zusammengefasste Ratios mit dem Hinweis, dass sie auf maximalen AP beruhen.

Reale Formeln:

- H12: `B12*C12*G12`, Shared-Anker für Grund-DPS.
- I19: `F19*B19`, einer der DPM-Anker. DPM ist Damage per Magazine, kein Schaden pro Minute. Pellets werden nicht in jedem DPM-Zweig explizit multipliziert; Shotgun-Zweige müssen einzeln geprüft werden.
- M20: `VLOOKUP(A20, raw_hero_data!$A$1:$T$100, 10, FALSE)` für HP.
- S20: `VLOOKUP($A20, raw_hero_data!$A$1:$T$100, 16, FALSE)` für Schaden je Boon.
- T20: `VLOOKUP($A20, raw_hero_data!$A$1:$T$100, 18, FALSE)` für HP je Boon.
- U20: `VLOOKUP($A20, raw_hero_data!$A$1:$X$100, 23, FALSE)` für Spirit je Boon.
- W6: `T6*$L$3+M6`, Shared-Anker für HP bei eingestellten Boons.
- Y6: `ROUND(X6*G6*C6,2)`, Shared-Anker für skalierte DPS.
- AA6: `(Y6-H6)/H6`; AB6: `(W6-M6)/M6`; AC6: `AA6+AB6`. AC ist eine Summe zweier relativer Wachstumswerte, keine belegte Gesamtstärke eines Helden.
- Haze: AG20=`AF20*$M$3`, F20=`25+AG20`, X20=`S20*($L$3)+B20+AJ20`. Ammo skaliert mit dem separat eingestellten Spirit.
- Warden: AG25=`AF25*$M$3`, G25=`3.81+(4*AG25%)`. Bereits die als Base Fire Rate bezeichnete Zelle hängt dadurch von Spirit ab.

Beispiele des unveränderten Exports:

| Held und Zeile | Bullet | Ammo | Fire Rate | Grund-DPS | DPM | HP | Schaden/Boon | HP/Boon | HP bei 35 | Schaden bei 35 | DPS bei 35 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Haze, 20 | 5,26 | 44 | 9,52 | 50,0752 | 231,44 | 730 | 0,143 | 33 | 1.885 | 10,265 | 97,72 |
| Warden, 25 | 17,3 | 17 | 3,962 | 68,5426 | 294,1 | 805 | 0,25 | 60 | 2.905 | 26,05 | 103,21 |
| Wraith, 30 | 5,64 | 52 | 10,6 | 59,784 | 293,28 | 730 | 0,14 | 35 | 1.955 | 10,54 | 111,72 |

Erforderliche API-Felder: hero.id, name und Aktivitätsmerkmale; starting_stats.max_health, base_health_regen, max_move_speed, sprint_speed, stamina; standard_level_up_upgrades für Bullet, HP, Spirit, Melee und Resists; items.weapon_primary und gegebenenfalls weapon_secondary; weapon_info.bullet_damage, bullets, clip_size, cycle_time, intra_burst_cycle_time, burst_shot_count, reload_duration, Falloff-Felder. Besondere Spirit-Effekte aus den Properties und Skalierungsfunktionen der zugehörigen Fähigkeiten/Waffen.

Status: deterministische Arithmetik mit teils manuellen Eingaben. Die Gesamtzahl 45 zählt auch unfertige Lab-Zeilen. Ränge über alle aktiven Helden brauchen einen versionsgebundenen Aktivitätsfilter. L3 und M3 sind unabhängige Eingaben; bei Änderung der Boons passt Spirit sich nicht automatisch an. Max-Level bezeichnet hier 35 Boons mit dieser separaten Spirit-Einstellung.

## 2. damage calculator

Visuell: oben Waffenrechner, darunter Yamatos Fähigkeiten mit Spirit, Resist, Shred und Item-Schaltern; rechts Vergleich von zusätzlichem Weapon Damage, Fire Rate und Shred. Die sichtbare Waffenauswahl ist Warden, die Spirit-Auswahl Yamato.

Eingaben: Held B2, Boons B3=35, Ammo-Bonus B6=0, Weapon Damage D6=200 %, Fire Rate D7=50 %, Shred F1:F5, Enemy Resist I1=40 %. Spirit E12=168; für jede Fähigkeit Basisschaden, Spirit-Multi, DoT-Schalter und Zeit; Enemy Spirit Resist J12=30 %, weitere Eingaben für Damage Reduction, Shred, Amp, Dauer und Item-Schalter.

Waffenformeln:

- D2=`VLOOKUP(B2, Heroes!$A$6:$H$50, 2, FALSE)`.
- O2=`VLOOKUP(B2, Heroes!$A$6:$S$50, 19, FALSE)*B3`; O3=`O2+D2`. Cache 8,75 Wachstumsbonus und 26,05 Schaden.
- B7=`B5+(B5*B6)`.
- N6=`O3*(1+D6)` ergibt 78,15 pro Schuss; N7=`D4*(1+D7)` ergibt 5,943/s.
- D8=`N8`, Cache 464,44545 rohe DPS.
- F6=`1-(1-F1)*(1-F2)*(1-F3)*(1-F4)*(1-F5)` stapelt fünf Shreds multiplikativ.
- F8=`I1-F6`; G10=`D8*(F8*-1)+D8` ergibt 278,66727 DPS gegen 40 % Resist ohne Shred.
- T2=`($D$8+(($O$3*(S2/100))*$N$7))*($F$8*-1)+($D$8+(($O$3*(S2/100))*$N$7))`, Shared-Anker der zusätzlichen Weapon-Damage-Tabelle.
- U2=`($N$7*(1+S2/100)*$N$6)*($F$8*-1)+($N$7*(1+S2/100)*$N$6)`.
- V2=`$D$8*(($F$8-S2/100)*-1)+$D$8`.

Spiritformeln und echte Beispiele:

- B22=`IF(B16=TRUE,(B14+B20*B15)*B18,B14+B15*B20)`. Power Slash mit Base 295, Spirit-Multi 2,4 und Spirit 168 ergibt roh 698,2.
- B24=`(B22*(1+J15)*(1-(J12-J14)))`, Cache 631,4409088.
- J15=`(L21*0.05)+L20` berechnet Amp aus EE-Stacks plus anderem Shred-Feld. Das ist die tatsächliche Sheet-Verkabelung; sie muss gegen die Item-Semantik geprüft werden.
- J14=`1-(1-L12)*(1-L13)*(1-L14)*(1-L15)*(1-(L16+(0.07*L22))*(1-L17)*(1-L18)*(1-L19))`, Cache 0,204384. Die Klammerung ist exakt beibehalten. Sie ist nicht identisch mit einem einfachen Produkt unabhängiger Shred-Anteile.
- O12=`IF(B16=TRUE,(1-(J12-J14))*B19+J15,(1-(J12-J14))*B15+J15)`, Cache 2,1705216.
- O13=`O12*B20`, Cache 364,6476288. Der separat angezeigte modifizierte Spirit-Beitrag addiert Amp innerhalb der Ratio; B24 verwendet Amp dagegen als Multiplikator des Gesamtschadens. Beide Anzeigen dürfen nicht gleichgesetzt werden.
- O21=`IF(F16=TRUE,(1-(J12-J14))*F19+F15,(1-(J12-J14))*F15+J15)`; O22=`O21*F14`. Das multipliziert im dritten Block die Ratio mit dem Basisschaden, nicht mit Spirit. Ergebnis 49,74112 ist daher kein allgemein gültiger Spirit-Beitrag.
- B28=`VLOOKUP(B26,$A$34:$C$50,2,FALSE)`; B29=`B27*$E$12+B28`; B30=`B29*$J$15+B29`. Das sind Item-Schäden mit manuell gepflegter Ratio-Tabelle, etwa Cold Front, Silence Wave, Mystic Shot und Phantom Strike.

API-Felder: Waffenfelder wie Heroes; Item- und Ability-properties einschließlich value, bonus, scale_function und stat_scale; konkrete Ability-Upgrades; Wirkungsdauer, Tickrate, Stackgrenzen; Bullet- und Spirit-Resist; getrennte Shred- und Amp-Arten. Nutzerparameter gehören zu einem Szenario und sind keine Hero-Rohwerte.

Status: Rechenmodell, kein Laufzeitbeweis der Mechanik. Die beschädigten Ratio-Anzeigen und manuell gepflegten Item-Ratios nicht unverändert zur verbindlichen Mechanik machen. Begrenzte VLOOKUP-Bereiche A6:H50 können spätere Helden auslassen. 17 leere Formelcaches sind im Archiv erhalten.

## 3. Hero meta ranking

Visuell: farbige 1-bis-5-Matrix mit Early/Mid/Late, Nuke, Poke, Crowd Control, Engage, Disengage, Mobility, Mid Contest, Frontline, Pick, Support, Wave Clear, Carry und Sustain-DPS; darunter Meta-Tiers.

B1 erklärt ausdrücklich persönliche Bewertungen. Die Eingaben B:S sind Autorurteile. T4=`AVERAGE(J4:S4)` ergibt für Warden 3,8; es mittelt nur Disengage bis Physical Sustain-DPS und lässt B:I aus. Das ist nicht der Durchschnitt aller sichtbaren Kriterien. Die Formel gilt als Shared-Anker für weitere Helden.

B45 enthält einen DUMMYFUNCTION-Export eines Google-QUERY auf `Table2` mit `SELECT A WHERE (T > 4)`. B46 enthält `SELECT A WHERE (T >= 3.7 AND T < 4.)`; der exportierte Fallback ist Warden. Der Wert genau 4,0 fällt in diese beiden Bedingungen nicht hinein. Die vollständige Tierliste bleibt im XLSX erhalten.

API-Rohfelder: keine Quelle für diese Bewertungen. Entitätsnamen können mit Hero-IDs verbunden werden. Status: ausschließlich subjektiv. Die Arithmetik macht die Eingabemeinung nicht zu einem Fakt. Neue vergleichende Rollenmerkmale müssen ihre messbaren Größen und Definitionen nennen.

## 4. Damage Comparison

Visuell: DPS-Liniendiagramm über Boons für gewählten Helden sowie niedrigen, hohen und konkret ausgewählten Vergleich. Darunter ein Resist/Shred/Amp-Diagramm. Beispielauswahl Victor, Vergleichshelden Graves.

Eingaben B2/B7/B12/B16 für Helden, Boon-Gewichtung B3/B8/B13/B17. Reale Formeln:

- D2=`VLOOKUP(B2, Heroes!$A$6:$H$55, 2, FALSE)*D5`.
- D3=`VLOOKUP(B2, Heroes!$A$6:$S$55, 19, FALSE)*B3*D5`.
- I2=`$D$3*H2+$D$2`; J2=`I2*$D$4`, Shared-Anker bis Zeile 33.
- D7=`VLOOKUP(B7, Heroes!$A$6:$H$55, 2, FALSE)`; D8=`VLOOKUP(B7, Heroes!$A$6:$S$55, 19, FALSE)*B8*D10`; N2=`$D$8*M2+$D$7`; O2=`N2*$D$9`.
- D13=`VLOOKUP(B12, Heroes!$A$6:$S$55, 19, FALSE)*B13*B14`.
- B18 sucht Pellets nur bis `Heroes!$A$6:$H$46`, die anderen Bereiche reichen bis Zeile 55.

Echte Caches: Victor Base 13, Wachstum 0,26, Rate 5,85; bei einem Boon 13,26 Schaden und 77,571 DPS. Graves Base 4,2, Wachstum 0,054, Rate 9,8; bei einem Boon 4,254 Schaden und 41,6892 DPS.

API-Felder: Bullet, Pellets, Rate und Bullet-Gain je Boon. Status: faktisch nach bestätigten Eingaben. Die Vergleichsgruppen sind manuell ausgewählt, keine automatisch berechneten besten und schlechtesten Helden. Pellets sind bei verschiedenen Vergleichspfaden uneinheitlich in Basis- und Wachstumsformeln eingebaut. Die sichtbare Kurve reicht im aktuellen Ausschnitt bis 32 Boons; das ist kein vollständiger Max-Level-Nachweis.

## 5. Hidden Mechanics

Visuell: mehrere kleine Blöcke zu Shiv Rage, Lycan Transformation, Wraith Resource, Walkers/Guardians, Comeback/Urn, First Blood, Kills und Midboss.

Die meisten Mechanikwerte sind eingegebene Konstanten, keine API-Abfragen. Beispiele: Shiv Drain 0,35 %, Bullet Gain 0,017, Spirit Gain 0,014; Wraith Resource per Card 100, Resource per Bullet 0,35 und Crit 6. Genau diese Quelle muss je Patch gegen aktuelle Daten geprüft werden.

Reale Formeln:

- E15=`($E$5*D15+$E$4)/$E$3`, erster Wraith-Boon-Damage-Bedarf, Cache 429,0196078.
- B21=`B19*B16` ergibt 7 Resource für Light Melee; B22=`B20*B16` ergibt 17,5 für Heavy Melee.
- A29=`1-(1-A27)*(1-B27)*(1-C27)*(1-D27)*(1-E27)*(1-F27)` versus B29=`SUM(A27:F27)`. Bei 55 % und 30 % zeigt es 68,5 % real gegenüber 85 % bloßer Summe.
- O7=`(O5-P5)-3000`, O10=`(1-P5/O5)`, O12=`(P10*130)*O10` für Comeback; Cache 22.000, 20,83333333 % und 677,0833333.
- R10=`$Q$4+Q10*$Q$7`, Shared-Anker der Urn-Zeitwerte.
- H36=`H26*60+J26`; I36=`MIN(100%,H36/2200)`; J36=`(I38-I39)*I36+250`. Bei 12 Minuten ergeben sich 720 Sekunden, 32,72727273 % Fortschritt und Kill-Grundbounty 888,1818182.
- J29=`J36*0.25+J36`, Cache 1.110,227273; J30=`J36*0.15+J36`; J31=`J36*0.85` usw. für Assister-Varianten.
- O29=`N34+O34*M26`, Midboss-Beispiel 4.700 bei 34 Minuten.

API-Felder: Rage-/Resource-Properties und Level-Up-Werte der jeweiligen Fähigkeiten; Objektiv-Resists, Bounty-, Urn-, Comeback-, Kill- und Midboss-Regeln aus einem versionsgebundenen globalen Spielkonfigdatensatz. Das bisherige Hero-/Item-Importpaar im Sheet liefert für viele dieser Konstanten keine Herkunft. Status: teils deterministische Rechnung, teils ungeprüfte Mechanikbehauptung. Fehlende globale Rohfelder müssen als Datenlücke sichtbar bleiben.

## 6. Boons/AP

Visuell: Souls, Boons, AP, Schrittweite; Markierungen für Lane, Ultimate und die vier T3-Meilensteine.

A:C sind manuelle Tabellenwerte. Einziger Formelanker F3=`A3-A2`, Shared-Bereich F3:F37. Vollständige Exportkurve:

- Start 600 Souls ohne Boon-/AP-Eintrag.
- Boons 1 bis 10: Souls `[900,1200,1500,2100,2800,3600,4400,5200,6000,6800]`; AP `[1,1,2,2,3,3,4,5,6,7]`.
- Boons 11 bis 20: Souls `[7700,8600,9600,10600,11600,12600,13800,15100,16600,18100]`; AP `[8,9,10,11,12,13,14,15,16,17]`.
- Boons 21 bis 35: Souls `[19600,21100,22600,24100,25600,27100,28600,30100,31600,33100,34600,36100,37600,39100,40600]`; AP `[18,19,20,21,22,23,24,25,26,27,28,29,30,31,32]`.
- Marker: 5 Boons/3 AP Lane, 6 Boons Ultimate, 11/8 erster T3, 19/16 zweiter, 27/24 dritter, 35/32 vierter.

API-Felder: `hero.level_info[level].required_gold`, `use_standard_upgrade`, `bonus_currencies` für EAbilityPoints/EAbilityUnlocks sowie Standard-Upgrades. Aktueller öffentlicher Hero-Abruf zeigt andere required_gold-Einträge, beispielsweise Level 2=200 und Level 36=48.600. Die Zuordnung zwischen Level, erhaltenen Boons, Start-Souls und Tabelle ist deshalb explizit zu prüfen. Die Sheet-Kurve darf nicht als aktuelle API-Kurve einprogrammiert werden.

Status: faktische Absicht, manuell gepflegter und nicht versionsgebundener Stand.

## 7. shopBonuses

Visuell: oben aktuelle Weapon-/Vitality-/Spirit-Schwellen mit relativen Zuwächsen; unten ein ausdrücklich als alt markierter Vergleich. Die unteren Werte sind kein zweiter aktiver Bonuspfad.

Obere Schwellen A3:A13, E3:E13 und I3:I13:

`[800,1600,2400,3200,4800,6400,8000,11200,16000,22400,28800]`.

- Weapon C3:C13: `[9,12,15,18,46,54,62,74,86,100,115]`.
- Vitality G3:G13: `[9,12,15,20,38,42,46,50,54,60,66]`.
- Spirit K3:K13: `[7,11,15,19,38,45,52,59,66,75,100]`.

Formeln B4=`A4/A3-1`, D4=`C4/C3-1`, F4=`E4/E3-1`, H4=`G4/G3-1`, J4=`I4/I3-1`, L4=`K4/K3-1`, jeweils Shared-Anker bis Zeile 13. D7 zeigt 155,5555556 % relativen Anstieg von 18 auf 46 Weapon-Bonus. Das ist nicht ein zusätzlicher 155-%-Spielbonus.

Unterer alter Block: D25=`C25/C24-1`, H25=`G25/G24-1`, L25=`K25/K24-1`. Dort etwa Vitality 84 bei 800 und 1.568 bei 28.800, Spirit zuletzt 81 statt 100.

API-Felder: `hero.cost_bonuses.weapon/vitality/spirit[].gold_threshold`, `bonus`, `percent_on_graph`, gegebenenfalls `purchase_bonuses`. Der aktuelle Warden-Abruf bestätigt alle drei oberen Arrays exakt. API percent_on_graph ist ein separates Feld und darf nicht mit dem berechneten relativen Zuwachs verwechselt werden. Status: obere Tabellen gegen echte aktuelle API plausibilisiert, untere ausdrücklich historisch.

## 8. ttk

Visuell: Holliday-Angriff auf Mina, Shred-, Resist-, Ammo-, Weapon-Damage- und Fire-Rate-Eingaben; Trefferquote, Headshot-Anteil und Magazine bis Kill darunter.

Eingaben: Holliday, 24 Boons, Weapon Damage 100 %, Fire Rate 50 %, Ammo 100 %, zweimal 13 % Shred. Defender Mina mit 1 Boon, 500 Item-HP, 45 % Bullet Resist. Trefferquote 50 %, Headshotquote 15 %, Headshot-Zusatz 90 %.

Reale Formeln:

- F6=`1-(1-F1)*(1-F2)*(1-F3)*(1-F4)*(1-F5)` ergibt 0,2431.
- F8=`H1-F6` ergibt Restresist 0,2069.
- I2=`VLOOKUP(B2, Heroes!$A$6:$S$50, 19, FALSE)*B3`; I3=`I2+D2` ergibt 46,256.
- H6=`I3*(1+D6)` ergibt 92,512; H7=`D4*(1+D7)` ergibt 3,18/s.
- B7=`B5+(B5*B6)` ergibt 20 Schuss; B9=`B7/H7` ergibt 6,289308176 Sekunden Magazindauer.
- G10=`D8*(F8*-1)+D8` ergibt 233,3206297 DPS.
- D14=`F13*B14+D13`; D15=`D14*(B15/100)+D14+F14` ergibt 1.187 Defender-HP.
- B19=`D15/G10`, Cache 5,087419837 Sekunden bei voller Trefferquote.
- E20=`G10*D20`; E21=`(E20*D21)*D22`; E22=`E20+E21` ergibt 132,4094574 DPS mit Trefferquote und Headshots.
- B23=`D15/(E21+E20)` und J20=`D15/G20` ergeben 8,964616454 Sekunden.
- B24=`B10*D20` ergibt 925,12 wirksamen Magazinschaden; B25=`ROUNDUP(D15/B24,0)` zeigt 2 Magazine.

API-Felder: Angreifer-Waffenwerte, Defender-HP/-Growth, Resists, Crit-Skalierung, Item-Upgrades, Ammo und Reload. Szenariowerte Trefferquote und Headshot-Anteil getrennt führen.

Status: Durchschnittsschadensmodell. Obwohl zwei Magazine notwendig sind und die berechnete Killzeit über der Magazindauer liegt, addiert die TTK-Formel keine Reload-Zeit. Sie simuliert weder diskrete Treffer noch Regeneration, Schilde, DoT oder Resistwechsel. Ein Rust-Kampfsimulator soll diese Faktoren erfassen und eine abweichende TTK begründen, statt die vereinfachte Zahl als exakte Zeit zu übernehmen.

## 9. haze

Visuell: links Fixation-Damage und Gesamt-DPS bei 40/80/Max-Stacks, Varianten mit Weapon Damage, Fire Rate und Shred; rechts lange kumulative Shot-/Stack-Tabelle.

Eingaben: B1=17 Boons; A3=0,2 Schaden je Fixation-Stack; C3 Weapon-Damage-Bonus; B6 Fire-Rate-Bonus; A6 Grundrate. In der sichtbaren Null-Bonus-Konfiguration B3=7,691, Rate 10/s, Max-Stack-Zeit D6=8 Sekunden.

Reale Formeln:

- B3=`VLOOKUP("haze", Heroes!$A$6:$H$55, 2, FALSE)+VLOOKUP("haze", Heroes!$A$6:$AC$55, 19, FALSE)*B1`.
- E3=`C3*0.03`; G3=`F3*$A$3+$E$3`; H3=`$B$3`; I3=`H3+H3*$C$3`; J3=`I3+G3`.
- K3=`$C$6*J3`; L3=`K3*(1+$D$3)`; M3=`$C$6*(H3+G3)*(1+$D$3)`; N3=`$A$6*(I3+G3)*(1+$D$3)`; O3=`$A$6*(G3+H3)`.
- C6=`A6+A6*B6`; D6=`80/C6`.
- P3=`J3`; P4=`P3+J4`, kumulativer Schaden.
- A14=`P42` ergibt 471,64 bis 40 Stacks; B14=`P82` ergibt 1.263,28 bis 80; C14=`P167` ergibt 1.619,68 in der Max-Stack-Vergleichsvariante.
- A19=`L42`, B19=`L82`, C19=`L167`: 156,91, 236,91 und 324,91 DPS.
- S3=`P88-P3`, S4=`P89-P4` usw. stellen Variante und Baseline gegenüber.

API-Felder: Haze-Waffe plus Fixation-Ability-properties, Schaden/Stack, Grenzen, Wachstum/Upgrades, Anwendungs- und Ablaufbedingungen sowie Item-Effekte. Status: spezielles deterministisches Szenario. Eingetragene Rate 10/s ist nicht die aktuelle API-Basisrate 9,5238095/s. Die Sheet-Konstante 0,2 und die Weapon-Damage-Kopplung 0,03 müssen aus der aktuellen Ability bestätigt werden; sie dürfen nicht als spezielle Haze-Konstanten in Produktcode landen.

## 10. scratchpad

Visuell: Burst-Rate-Verlust für Seven, Paradox, Lash und Sinclair; Vergleich von Schadenszuwächsen weiter unten. Enthält bewusst experimentelle und teilweise kaputte Rechnungen.

Burst-Eingaben: B Intervall im Burst, C Burstzahl, E Cycle Time. Reale Formeln D3=`C3*B3`, F3=`D3+E3`, G3=`E3/F3`, Shared-Anker für Zeilen 3:6. Seven-Beispiel: 0,066 Intervall, 3 Schuss, 0,2625 Cycle; D3=0,198, F3=0,4605, G3=57,00325733 %. Zweiter Block D10=`C10*B10`, F10=`D10+E10`, G10=`E10/F10`; Seven dort 0,084 und 2 Schuss, F10=0,4305.

Wachstumsexperiment A28=`A27+B28`, B28=`A27*B27`; A29=`A28*0.07+A28` vergleicht wiederholtes Multiplizieren mit linearen Zuwächsen C29=`A28*0.07` und halben Zuwächsen F29=`A28*0.07*1/2`.

Kaputte Formeln F62=`$C$6*(A62+#REF!)*(1+$D$3)`, G62=`$A$6*(B62+#REF!)*(1+$D$3)`, H62=`$A$6*(#REF!+A62)`. Alle drei Caches enthalten wörtlich `#REF!`.

API-Felder: cycle_time, intra_burst_cycle_time, burst_shot_count, bullets, Reload; API liefert zusätzlich fertige shots_per_second sowie damage_per_second mit und ohne Reload. Die Bedeutung der Burstintervalle ist gegen diese gelieferten Felder zu prüfen. Die Sheet-G3-Größe ist ein Verhältnis von Zeitanteilen, keine eigenständige RPM-Formel. Status: Versuchsblatt, nicht als verbindliche Mechanik behandeln.

## 11. raw_hero_data

Visuell: echtes GViz-Rendering A1:T20 mit Hero-IDs, Bewegung, Melee, HP und Wachstum. Die Originalformatierung und die restlichen Spalten wurden nicht vollständig visuell angezeigt. Alle Daten sind im XLSX vorhanden.

A2 ruft `IMPORTJSONAPI("https://api.deadlock-api.com/v1/assets/heroes/", "$.*", ..., 1)` auf. Der Ausdruck setzt die Feldliste durch Stringverkettungen zusammen. Vollständige angefragte Rohfelder:

- name, id, disabled.
- starting_stats.max_move_speed.value, sprint_speed.value, crouch_speed.value, move_acceleration.value, light_melee_damage.value, heavy_melee_damage.value.
- starting_stats.max_health.value, stamina.value, stamina_regen_per_second.value, base_health_regen.value, crit_damage_received_scale.value und nochmals max_health.value.
- standard_level_up_upgrades.MODIFIER_VALUE_BASE_BULLET_DAMAGE_FROM_LEVEL, MODIFIER_VALUE_BASE_BULLET_DAMAGE_FROM_LEVEL_ALT_FIRE, MODIFIER_VALUE_BASE_HEALTH_FROM_LEVEL, MODIFIER_VALUE_BASE_MELEE_DAMAGE_FROM_LEVEL, MODIFIER_VALUE_BONUS_ATTACK_RANGE, MODIFIER_VALUE_BULLET_ARMOR_DAMAGE_RESIST, MODIFIER_VALUE_TECH_ARMOR_DAMAGE_RESIST, MODIFIER_VALUE_TECH_POWER.
- items.weapon_primary, weapon_secondary, signature1, signature2, signature3, signature4.

Die aktuelle API nutzt bei Warden für Spirit-Resist `MODIFIER_VALUE_TECH_RESIST`, nicht den vom Sheet angefragten Namen `MODIFIER_VALUE_TECH_ARMOR_DAMAGE_RESIST`. Fehlende Felder dürfen nicht still als null oder 0 interpretiert werden. Der Import lässt zudem `scaling_stats`, `level_info` und `cost_bonuses` aus. Diese sind für die Rechenschicht ausdrücklich nötig: Haze hat `scaling_stats.EClipSize={scaling_stat: ETechPower, scale: 0.5}`, Wraith `ESprintSpeed` mit scale 0,05. Warden liefert `ERoundsPerSecond` mit scale 0,008 und `EFireRate` mit scale 0,21. Das Sheet verwendet dagegen 0,06 für Wraiths Sprint-Scaling und 0,1 in Wardens manueller Spirit-Spalte. Die Unterschiede sind im aktuellen Rohdatensatz belegt; wie Wardens zwei Scaling-Felder zusammenwirken, muss die bestehende Mechanik klären.

Status: importierte Fakten, aber keine client_version im Import und keine Vollständigkeitsgarantie. Importierte Lab- und nicht aktive Helden sind vor Rängen zu filtern. Ein Rust-Parser muss IDs verwenden, nicht nur englische Namen.

## 12. raw_items_and_abilities

Visuell: Original-HTML-Tabelle mit hero, id, code name, game name und heroes; enthält Waffen, Fähigkeiten und Bewegungs-/Systemfähigkeiten, nicht bloß Shopitems.

A2=`IMPORTJSONAPI("https://api.deadlock-api.com/v1/assets/items", "$.*", "hero, id, class_name, name,  heroes", 1)`.

API-Felder: hero, id, class_name, name, heroes. Dieser Tab ist ein Index und importiert selbst keine vollständigen Weapon-/Ability-properties. Diese kommen erst über die Einzelabfragen des nächsten Tabs. Status: faktenorientierter Katalog, nicht versionsgebunden; A2 hat einen leeren Cache, nachgelagerte Indexzellen sind trotzdem im Export vorhanden.

## 13. hero query

Visuell: echtes Google-GViz-Rendering A1:T20, Yamato-Hero-ID 27, zugehörige Waffen/Fähigkeiten und sichtbare DNS-Fehler. Keine vollständige Originalformatierung; die übrigen Spalten und Inhalte sind nur über das vollständige XLSX belegt.

44 Formeldefinitionen. Auswahl und Lookups verbinden raw_hero_data mit raw_items_and_abilities. Abfragen in A11/G11/M11/S11/Y11 verwenden Einzel-URLs von `https://assets.deadlock-api.com/v2/items/<id>` und IMPORTJSONAPI. Beispiel M11:

`IMPORTJSONAPI(CONCAT("https://assets.deadlock-api.com/v2/items/",B7), "$..[?(@.value>0 || @.bonus)]", "~, value, scale_function, stat_scale")`

Alle fünf abgefragten Blöcke haben einen gecachten DNS-Fehler:

- A11, Item 3334760137, Yamato-Waffe.
- G11, Item 3255651252, Power Slash.
- M11, Item 2566573207, Flying Slash.
- S11, Item 2366960452, Crimson Slash.
- Y11, Item 3319782965, Shadow Transformation.

XLSX-DUMMYFUNCTION-Fallbacks bewahren Google-Funktionen beziehungsweise gecachte Ergebnisse für Excel. Ein lokaler Excel-Neuberechnungslauf kann IMPORTJSONAPI und QUERY nicht ohne Weiteres ausführen. Das Archiv muss deshalb unverändert bleiben.

API-Felder: hero.id und items.signature*/weapon_primary, item.id/class_name/name, properties mit value/bonus und scale_function/stat_scale. Status: vorgesehener Faktenzugriff, im vorhandenen Cache fehlgeschlagen. Die Filterbedingung value>0 kann außerdem relevante nullwertige und negative Properties ausblenden.

## Aktueller öffentlicher API-Abgleich

Zusätzliche unveränderte Quellen im gemeinsamen Beweisverzeichnis:

| Datei | Abruf UTC | SHA-256 |
|---|---|---|
| current-api-heroes.json | 2026-10-07T03:27:54Z | 172aa578b4624c6c7800b72467336347c0ce0a3303e3319f8a54e221668a5edb |
| current-api-items.json | 2026-10-07T03:28:13Z | 86540843b94601ca753068767f1320649e03634e22c070fe2aead8d6faf85287 |

Adressen: `https://api.deadlock-api.com/v1/assets/heroes` und `/v1/assets/items`. 65 Hero- und 746 Itemdatensätze. Diese Abrufe sind nicht explizit an eine client_version gebunden. Sie belegen aktuelle Endpointwerte zum Abrufzeitpunkt, keinen freigegebenen Gleichstand mit Paket E oder dem aktiven Produktionspatch.

| Held | API Bullet | API Ammo | API Schüsse/s | API Grund-DPS | API DPS mit Reload | API Magazinschaden |
|---|---:|---:|---:|---:|---:|---:|
| Haze | 5,26 | 25 | 9,523809524 | 50,095238095 | 25,167464115 | 131,5 |
| Warden | 17,34 | 17 | 3,80952381 | 66,057142857 | 38,652068446 | 294,78 |
| Wraith | 5,64 | 52 | 10,582010582 | 59,682539683 | 36,733466934 | 293,28 |

Nachladezeiten aus `weapon_info.reload_duration`: Haze 2,35 Sekunden, Warden 2,914 Sekunden, Wraith 2,82 Sekunden. `RPM = shots_per_second * 60` ergibt ohne Nachladen 571,4285714, 228,5714286 und 634,9206349. RPM beschreibt Schüsse pro Minute. Die Sheet-Spalte DPM beschreibt Magazinschaden und ist davon unabhängig. Ein Reload-Modell muss angeben, ob der Zeitpunkt des ersten Schusses und die letzte Schusswartezeit im Zyklus enthalten sind; die API-eigenen Werte mit und ohne Reload erlauben den direkten Abgleich.

HP, Bullet-Gain und HP-Gain der drei Helden stimmen mit den genannten Sheet-Importwerten überein. Waffenabweichungen haben unterschiedliche Ursachen:

- Haze-Sheet hat 44 Ammo statt API-Basis 25, weil M3=38 Spirit über 0,5 Ammo/Spirit weitere 19 Ammo erzeugt. Das ist ein anderer Eingabezustand, kein Beweis eines veralteten Clip-Wertes.
- Warden-Sheet hat 3,962/s statt API-Basis 3,80952381/s wegen Spirit-Verkabelung. Zusätzlich ist Bullet 17,3 gegenüber 17,34 gerundet oder anders gepflegt.
- Wraith 10,6/s gegenüber 10,582010582/s und Haze 9,52/s gegenüber 9,523809524/s sind gerundete Sheet-Eingaben.
- Sheet-Grund-DPS und Max-DPS enthalten keine Reload-Zyklen. API liefert damage_per_second_with_reload ausdrücklich separat. Diese Größen dürfen nicht gegeneinander als dieselbe Metrik getestet werden.

API-Falloff-Rohwerte bei Haze: start 787, end 1811, start_scale 1, end_scale 0,1, bias 0,5. Warden start 708,661, end 1850,39; Wraith start 708,661, end 2047,24. Das Sheet zeigt gerundete Entfernungen in Metern, bei Warden 20/58 und Wraith 18/52. Vor einer Bewertung ist die Einheit und Umrechnung des API-Datensatzes zu belegen. Aus zwei Sheet-Grenzen allein lässt sich weder bias noch Endskalierung herleiten. Es wurde keine erfundene Falloff-Formel eingesetzt.

## Beweisorte und verbleibende Arbeit

Die Bilder 01 bis 10 stammen aus der öffentlichen Google-htmlview mit jeweiligem gid, 1920×1080. Bild 12 stammt aus `htmlview/sheet?headers=true&gid=1854999973`. Bilder 11 und 13 stammen aus `gviz/tq?tqx=out:html&gid=<gid>&range=A1:T20`. Sie sind echte Browserdarstellungen der Daten, keine aus Text rekonstruierte Beweisdarstellung. GViz erhält nicht die Originalgestaltung.

Geprüft wurden 01-heroes.png und die Kontaktbögen contact-02-05.png, contact-06-09.png, contact-10-13.png. Die Kontaktbögen verkleinern lediglich die gespeicherten echten Screenshots. Damit wurde jeder der 13 Tabs visuell nachvollzogen. Nicht belegt sind vollständige Scans aller Zeilen/Spalten, interaktive Filter und das Original-Layout der beiden genannten Roh-/Abfragetabs. Der Preview-MCP konnte mangels Automation-Host nicht geöffnet werden; der vorhandene Headless-Browser lieferte die Nachweise.

Für die Rust-Umsetzung:

1. Paket E als versionsgebundene Wahrheit verwenden. Öffentliche Endpoint-Proben und dieses Sheet liefern keine gemeinsame Patch-ID.
2. Grundwerte, Boon-Zustand, Spirit, Shopausgaben und Szenarioparameter separat führen; keine Max-Level-Spirit-Konstante übernehmen.
3. API-eigene Waffenmetriken und vorhandene Mechanik wiederverwenden. Burst-/Reload-/Falloff-Modell gegen deren Roh- und abgeleitete Felder abgleichen.
4. Sheet-Formelparität und reale Mechanikparität getrennt ausweisen. Kaputte Formeln, DNS-Caches, alte Shopboni und Meta-Meinungen dürfen keine Produkt-Wahrheit werden.
5. Für Hidden Mechanics fehlen im Sheet belastbare Quellen vieler globaler Konstanten. Benötigte globale API-/Konfigfelder als eigene Lücke führen, keine Werte erfinden.
6. Die fünf hero-query-Einzelabrufe durch vorhandene lokale API-Entitäten ersetzen, sofern die vollständigen Properties vorliegen. Kein zweiter Importer nötig.
7. Ränge und Perzentile sind nicht im Sheet vorhanden. Sie werden aus demselben versionsgebundenen, aktiven Hero-Satz berechnet; Stichprobengröße, Gleichstände und Richtung der Metrik müssen sichtbar sein.

Offen: vollständige Originaldarstellung von raw_hero_data und hero query sowie versionsgebundener Abgleich von Boon-Kurve, Ability-Upgrades, globalen Hidden Mechanics und den manuellen Ratios. Export und Untersuchung umfassen die 13 vorhandenen Tabs; der Bericht bleibt am bestätigten eigenen Worktree-Übergabeort.
