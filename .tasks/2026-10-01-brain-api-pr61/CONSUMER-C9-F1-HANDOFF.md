status: überholt
Datum: 2026-10-01

# Überholter C9-F1-Fix-Handoff

Der ursprüngliche Fixauftrag beruhte auf einem statischen Consumer-Fixture und wurde durch die aktuelle Producer-/Consumer-Reichweitenprüfung widerlegt. Gegen die verifizierten Heads Bots #459 `46edc1023a819ba0ba3c37b7de96c333f485f797` und Brain #61 `b687f613b3df2c49138d9d2837e005c33e646d9f` sendet `BrainApiAnswerer::query()` `domain: None` und `AnswerProfile::Explain`. Brain erzeugt `BuildRejected` nur nach validierter expliziter Build-Domainroute. Dieser Status ist daher für die aktuelle Anfrage nicht erreichbar.

Es gibt keinen bestätigten F1-Quellenfix im realen Requestpfad. Das alte Handoff nicht als offenen Sourceauftrag verwenden. Die frühere unabhängige Abnahme bleibt historische Evidenz; die gemeinsame Intent-Abnahme muss den Widerspruch am exakten Freeze ausdrücklich auflösen. Die bestehende Bots-Consumer-Fassung wird nicht aufgrund dieses Handoffs verändert.