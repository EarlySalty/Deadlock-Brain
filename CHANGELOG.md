# Changelog

## #2 — Autonomes YouTube-Lernen: Creator-Wissen fließt in die Wissens-DB

Das System sammelt zwar seit Längerem Videos der kuratierten Deadlock-Creator ein, doch das darin steckende Spielwissen wurde bisher nicht nutzbar gemacht — es gab keinen Weg, die Inhalte automatisch zu verstehen und strukturiert abzulegen. Wer die Erkenntnisse aus einem Coaching- oder Meta-Video wollte, musste es selbst schauen.

Neu ist ein eigenständiger Lern-Lauf: Aus den festen Creator-Feeds werden die frischesten Videos gezogen, und jedes davon wird in zwei Schritten ausgewertet. Zuerst wird das Video vollständig angeschaut und sein Inhalt in natürlicher Sprache zusammengefasst; anschließend wird diese Zusammenfassung in einzelne, überprüfbare Aussagen zerlegt — Builds, Item-Timings, Matchups, Mechaniken, Combos und Meta-Einschätzungen — und jede Aussage mit Bezug auf den betroffenen Helden oder das Item sowie einer Einschätzung ihrer Eindeutigkeit in der Wissensdatenbank abgelegt. Videos ohne verwertbares Spielwissen werden als geprüft, aber leer vermerkt; einzelne Fehlschläge stoppen den Lauf nicht, sondern werden beim nächsten Mal erneut versucht. Bereits ausgewertete Videos werden übersprungen, sodass ein Lauf gefahrlos wiederholbar ist.

Damit füllt sich die Wissensbasis bei jedem Lauf von selbst und ist so gebaut, dass sie unbeaufsichtigt im Hintergrund läuft, ohne manuelles Zutun. Die gesammelten Aussagen sind zunächst als ungeprüft markiert; eine spätere Stufe gleicht sie gegen die harten Patch- und Statistikdaten ab und gewichtet sie.

## #1 — Sheet-Sync: Enrichment-Schritt brach jeden Lauf ab

Der periodische Sheet-Sync-Job (Lauf alle 4 Stunden) hat die ersten Schritte — Sheet-Abgleich, Normalisierung und die Build-Analyse — sauber durchlaufen, ist dann aber im Enrichment-Schritt jedes Mal mit einem Fehler abgebrochen. Weil die Schritte als verkettete Abfolge laufen (jeder startet nur, wenn der vorige ohne Fehler endet), wurden die nachgelagerten Teile — die Patch-Impact-Analyse und die Meta-Trends — danach nie mehr ausgeführt. Der gesamte Job galt damit als fehlgeschlagen.

Ursache war ein fehlender optionaler Schalter: Der Patch-Impact-Teil entscheidet intern anhand eines Trockenlauf-Schalters, ob er nur die Anfrage bauen oder das Modell wirklich aufrufen soll. Dieser Schalter wurde im Code abgefragt, war für genau diesen Teilbefehl aber nie als Option deklariert. Der Zugriff darauf lief deshalb sofort in einen Fehler — noch bevor irgendeine eigentliche Arbeit passierte. Der fehlende Schalter wurde nachgetragen.

Jetzt ist der Trockenlauf-Schalter beim regulären Lauf standardmäßig aus, der Enrichment-Schritt läuft normal durch, und die Kette bis zu den Meta-Trends wird wieder vollständig abgearbeitet.
