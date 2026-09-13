# Startbrief Agent D: unabhängige Kritik

Basis: `e263b5f69c10f613b1df97e4e035acdf9b71dc71`.

## Bekannter Befund

Aktuelle saubere Release-Auswertung auf `FROZEN-V2.json`: Warden Build 779996 Version 45 trifft nur 1/9 Referenzwaffen (`Titanic Magazine`), Recall 0,0526316, Jaccard 0,0294118. Ein dokumentierter Zwischenstand lag bei 5/9. Integration ist damit nicht abnahmefähig.

## Auftrag

Produktionscode ausschließlich lesen. Unabhängig bestimmen:

- welcher Commit-/Funktionsbereich den 5/9 -> 1/9 Rückgang plausibel erklärt;
- ob die neueren Kampfannahmen mechanisch korrekter sind oder ein Regressionsfehler vorliegt;
- ob Planner, Combat oder deren Kopplung eine Invariante verletzt;
- welche Unknown-/Nullpfade aus den Sechs-Helden-Messungen weiterhin die Abnahme blockieren;
- welche minimalen, generischen Gegenproben vor Merge zwingend sind.

Keine Produktionsdateien editieren, keine Holdout-Ergebnisse als Tuningmaterial verwenden. Ergebnis ausschließlich in `agents/critic-ABSCHLUSS.md`, priorisiert nach `BLOCKER`, `HOCH`, `MITTEL`, jeweils mit Datei/Funktion/Testidee.
