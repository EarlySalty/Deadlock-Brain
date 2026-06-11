# Changelog

## #1 — Sheet-Sync: Enrichment-Schritt brach jeden Lauf ab

Der periodische Sheet-Sync-Job (Lauf alle 4 Stunden) hat die ersten Schritte — Sheet-Abgleich, Normalisierung und die Build-Analyse — sauber durchlaufen, ist dann aber im Enrichment-Schritt jedes Mal mit einem Fehler abgebrochen. Weil die Schritte als verkettete Abfolge laufen (jeder startet nur, wenn der vorige ohne Fehler endet), wurden die nachgelagerten Teile — die Patch-Impact-Analyse und die Meta-Trends — danach nie mehr ausgeführt. Der gesamte Job galt damit als fehlgeschlagen.

Ursache war ein fehlender optionaler Schalter: Der Patch-Impact-Teil entscheidet intern anhand eines Trockenlauf-Schalters, ob er nur die Anfrage bauen oder das Modell wirklich aufrufen soll. Dieser Schalter wurde im Code abgefragt, war für genau diesen Teilbefehl aber nie als Option deklariert. Der Zugriff darauf lief deshalb sofort in einen Fehler — noch bevor irgendeine eigentliche Arbeit passierte. Der fehlende Schalter wurde nachgetragen.

Jetzt ist der Trockenlauf-Schalter beim regulären Lauf standardmäßig aus, der Enrichment-Schritt läuft normal durch, und die Kette bis zu den Meta-Trends wird wieder vollständig abgearbeitet.
