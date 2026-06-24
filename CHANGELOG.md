# Changelog

## #3 — Brain auf Rust umgestellt: eine Sprache, gleiche Befehle

Das Brain — das Werkzeug, das Patchnotes, Statistiken, Sheet- und Creator-Wissen einsammelt und zu abrufbarem Spielwissen verdichtet — lief bisher in Python, während der Rest der Plattform längst auf Rust läuft. Eine in zwei Sprachen geteilte Codebasis ist schwerer zu warten und weiterzuentwickeln.

Die gesamte Funktionalität wurde nach Rust überführt. Ein gemeinsamer Kern trägt jetzt Datenbankzugriff, Datenbankschema, Konfiguration, die Netz-Abrufe und die Modellanbindung; darauf sitzen klar getrennte Fachbausteine für Quellen-Import, Normalisierung der Entitäten, Anreicherung der Patch-Daten, die Lern-Auswertung von Builds und Matches sowie den Abruf von Kontext, Timeline und Reviews. Alles ist unter einem einzigen Kommandozeilen-Werkzeug zusammengefasst, das exakt dieselben Befehle anbietet wie zuvor. Das Datenbankschema wurde aus dem Live-Bestand übernommen und nachweislich deckungsgleich nachgebaut, sodass beide Fassungen dieselbe Wissensdatenbank teilen.

Das Rust-Werkzeug arbeitet damit gegen dieselbe Datenbank und liefert dieselben Befehle; gegen eine Kopie der echten Datenbank wurden alle Abruf-Befehle erfolgreich gegengeprüft. Die bisherige Python-Fassung bleibt vorerst der aktive Hintergrunddienst — die Umstellung der Automatik auf die Rust-Fassung folgt als eigener, geprüfter Schritt. Die Vektor-Ähnlichkeitssuche ist bewusst noch ausgeklammert und kommt später.

## #2 — Autonomes YouTube-Lernen: Creator-Wissen fließt in die Wissens-DB

Das System sammelt zwar seit Längerem Videos der kuratierten Deadlock-Creator ein, doch das darin steckende Spielwissen wurde bisher nicht nutzbar gemacht — es gab keinen Weg, die Inhalte automatisch zu verstehen und strukturiert abzulegen. Wer die Erkenntnisse aus einem Coaching- oder Meta-Video wollte, musste es selbst schauen.

Neu ist ein eigenständiger Lern-Lauf: Aus den festen Creator-Feeds werden die frischesten Videos gezogen, und jedes davon wird in zwei Schritten ausgewertet. Zuerst wird das Video vollständig angeschaut und sein Inhalt in natürlicher Sprache zusammengefasst; anschließend wird diese Zusammenfassung in einzelne, überprüfbare Aussagen zerlegt — Builds, Item-Timings, Matchups, Mechaniken, Combos und Meta-Einschätzungen — und jede Aussage mit Bezug auf den betroffenen Helden oder das Item sowie einer Einschätzung ihrer Eindeutigkeit in der Wissensdatenbank abgelegt. Videos ohne verwertbares Spielwissen werden als geprüft, aber leer vermerkt; einzelne Fehlschläge stoppen den Lauf nicht, sondern werden beim nächsten Mal erneut versucht. Bereits ausgewertete Videos werden übersprungen, sodass ein Lauf gefahrlos wiederholbar ist.

Damit füllt sich die Wissensbasis bei jedem Lauf von selbst und ist so gebaut, dass sie unbeaufsichtigt im Hintergrund läuft, ohne manuelles Zutun. Die gesammelten Aussagen sind zunächst als ungeprüft markiert; eine spätere Stufe gleicht sie gegen die harten Patch- und Statistikdaten ab und gewichtet sie.

## #1 — Sheet-Sync: Enrichment-Schritt brach jeden Lauf ab

Der periodische Sheet-Sync-Job (Lauf alle 4 Stunden) hat die ersten Schritte — Sheet-Abgleich, Normalisierung und die Build-Analyse — sauber durchlaufen, ist dann aber im Enrichment-Schritt jedes Mal mit einem Fehler abgebrochen. Weil die Schritte als verkettete Abfolge laufen (jeder startet nur, wenn der vorige ohne Fehler endet), wurden die nachgelagerten Teile — die Patch-Impact-Analyse und die Meta-Trends — danach nie mehr ausgeführt. Der gesamte Job galt damit als fehlgeschlagen.

Ursache war ein fehlender optionaler Schalter: Der Patch-Impact-Teil entscheidet intern anhand eines Trockenlauf-Schalters, ob er nur die Anfrage bauen oder das Modell wirklich aufrufen soll. Dieser Schalter wurde im Code abgefragt, war für genau diesen Teilbefehl aber nie als Option deklariert. Der Zugriff darauf lief deshalb sofort in einen Fehler — noch bevor irgendeine eigentliche Arbeit passierte. Der fehlende Schalter wurde nachgetragen.

Jetzt ist der Trockenlauf-Schalter beim regulären Lauf standardmäßig aus, der Enrichment-Schritt läuft normal durch, und die Kette bis zu den Meta-Trends wird wieder vollständig abgearbeitet.
