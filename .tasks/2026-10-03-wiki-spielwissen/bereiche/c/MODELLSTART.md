status: aktiv
Datum: 2026-10-03

# Sicherer Modell- und Startnachweis C

Die Elternsitzung hat die aktiven Startparameter am eigenen Prozessvorfahren geprüft und ausschließlich erlaubte Modell-/Proxy-/Effortwerte ausgegeben. Keine Prozessumgebungen, Tokens oder API-Schlüssel wurden gelesen.

Beobachtete aktive Parameter der C-Hauptsession:

```text
--model gpt-6.1-sol[1m]
--effort high
ANTHROPIC_BASE_URL=http://127.0.0.1:18768
ANTHROPIC_MODEL=gpt-6.1-sol[1m]
ANTHROPIC_DEFAULT_OPUS_MODEL=gpt-6.1-sol[1m]
ANTHROPIC_DEFAULT_SONNET_MODEL=gpt-6.1-sol[1m]
ANTHROPIC_DEFAULT_HAIKU_MODEL=gpt-6.1-sol[1m]
ANTHROPIC_SMALL_FAST_MODEL=gpt-6.1-sol[1m]
```

Zwei aktive CLI-Prozessvorfahren zeigten dieselben Werte, jeweils mit ausdrücklich gesetztem `--effort high`. Die ausgewählte lokale Settings-Datei ist damit belegt; globale Standardsettings bestimmen diesen Auftrag nicht. AN_BEREICHE.md Punkt 5 bestätigt zusätzlich den isolierten Startweg für diesen Auftrag.

Native Worker werden ausschließlich mit dem Agent-Tool dieser Hauptsession gestartet, ohne Modelloverride oder alternative Starter. Der lokale Typ `coder` hat keinen Modell- oder Effortoverride und beschreibt ausdrücklich Modellvererbung. Der Agent `fork` erbt den Modellkontext der Elternsitzung. Kein xhigh oder max angefordert. Die Worker-Briefings begrenzen Effort auf high.

Dieser Nachweis dokumentiert die bereits erfolgreich ausgeführte sichere Elternprüfung. Worker sollen ihn als Voraussetzung übernehmen; eine erneute Prozessprüfung, die ein Hook verweigert, wird nicht umgangen. Falls der native Harness eine tatsächliche abweichende Modell-/Effortwahl meldet, stoppt der betroffene Worker und meldet sie C.
