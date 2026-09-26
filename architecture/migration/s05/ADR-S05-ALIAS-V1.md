# S05 Aliasvertrag v1

Status: Entscheidung im Review PR 25

## Entscheidung

Die Rust Normalisierung ist ab Contract v1 die kanonische Schreibweise für neue Alias Schlüssel. Sie trimmt Randtrennzeichen, behandelt Unterstriche, Unicode Leerraum und Steuerzeichen als Trenner, reduziert mehrere Trenner auf ein Leerzeichen und verwendet Unicode Kleinschreibung.

Python casefold wird nicht als Schlüsseldefinition übernommen. Dadurch bleibt zum Beispiel `Straße` als `straße` erhalten und wird nicht zu `strasse`. Eine gewünschte alternative Schreibweise wird als eigener Alias gespeichert. Aliase sind keine stabilen Entity IDs.

## Begründung

Die S05 Charakterisierung hat sieben Unterschiede zwischen dem historischen Python Verhalten und der bisherigen Rust Funktion gezeigt. Zwei Unterschiede stammen aus nicht idempotentem Randverhalten im Python Pfad. Ein weiterer betrifft U+001C als Steuertrennzeichen. Der neue Vertrag priorisiert stabile Wiederholung und kontrollierte Alias Kollisionen statt einer bitgleichen Übernahme dieser Altsemantik.

## Regression

Die Unit Tests prüfen Idempotenz, Rand Unterstriche, Steuertrennzeichen und Unicode Buchstaben. Bestandsdaten benötigen vor einer späteren Aktivierung einen Kollisionsbericht und einen versionierten Reindex. Dieser PR führt keinen produktiven Reindex aus.
