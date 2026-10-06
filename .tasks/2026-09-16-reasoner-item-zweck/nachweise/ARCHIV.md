# Historische Messausgaben

Die drei großen JSON-Ausgaben der früheren Mechanikmessung liegen unverändert in der Git-Historie. Sie sind keine Eingaben eines produktiven Brain-Pfads und werden von den Rust-, Skript- und Testpfaden nicht automatisch geladen. Die früheren Messprotokolle verweisen weiterhin auf ihre historischen Dateinamen.

Aufbewahrter, auf origin gesicherter Commit:

`22c8ebe07fe90963e063508551b417b71bd50dc2`

| Datei in diesem Ordner | Bytes | Git-Blob |
|---|---:|---|
| PHASE0-EVAL.json | 1298855 | fb368bb19236648b3f2afe91057b340f7229e16f |
| PHASE0-POP.json | 657133 | 3e1a02c94bbcd2e5fb41552476c29104466170c5 |
| PHASEA-EVAL.json | 1298855 | fe39c21bd4c52bba5725ee6783e95998656c21c5 |

Beispiel für den unveränderten Abruf des ersten Artefakts:

```sh
git show 22c8ebe07fe90963e063508551b417b71bd50dc2:.tasks/2026-09-16-reasoner-item-zweck/nachweise/PHASE0-EVAL.json
```

Für eine lokale Wiederholung die benötigten Originale in einem neuen privaten Messverzeichnis ablegen, nicht vorhandene Nachweise überschreiben. Die Herausnahme aus dem aktuellen Baum entfernt weder die ursprünglichen Git-Objekte noch die zugrunde liegenden Commits. Sie reduziert den aktiven Integrationsdiff, nicht den Prüfumfang des produktiven Codes.

Die Messungen vom 21.09.2026 liegen getrennt unter `/home/nathanael/.local/share/deadlock-brain/releases/20260921-final/`. Ihre Zusammenfassung und Datenfrischegrenzen stehen in `.tasks/2026-09-21-brain-final/REPORT.md`. Die historischen Dateien sind kein Nachweis eines aktuellen Patches oder einer fachlichen Freigabe.
