# Startbrief Agent A: Messwerkzeug / Holdout-Vorbereitung

Basis: `e263b5f69c10f613b1df97e4e035acdf9b71dc71`.

## Sperre

Der Integrationsstand verfehlt aktuell die bekannte Entwicklungsanforderung (Warden 779996 Version 45: 1/9 Referenzwaffen statt mindestens 5/9). Deshalb **noch keinen vollständigen Holdout auswerten und keine Holdout-Ergebnisse an Bauagenten geben**. Das verhindert, dass der unabhängige Autoren-Holdout zum versteckten Tuningziel wird.

## Auftrag bis zur Freigabe

1. Messwerkzeug ausschließlich technisch auditieren: eingefrorener V2-Hash, Algorithmusrevision, Baselinerevision, exklusive Ausgabedateien, Hero-Selektion, Autoren-Exklusionsgruppe und Claims-/Skillfolge-Trennung.
2. Zulässig ist `evaluate`/`plan` auf bekannten Entwicklungsfällen und künstlichen/Unit-Fixtures; `holdout` über reale Autoren bleibt gesperrt.
3. Prüfe, dass eine schmutzige Quellrevision beweisfähige CLI-Messungen verweigert und dass existierende Ausgaben nicht überschrieben werden.
4. Keine Produktionslogik ändern und keine neue Datenquelle laden.

Nach expliziter Integrator-Freigabe (`READY-FOR-HOLDOUT` im Agentenordner) vollständigen Holdout und Sensitivität auf exakt `FROZEN-V2.json` ausführen und nur dokumentieren, nicht tunen.

## Dateibesitz

Nur `examples/build_evaluation.rs`, `examples/support/` und eigene Nachweisdokumente.

## Gate

- Example/Tests/Clippy sauber;
- Report `agents/holdout-ABSCHLUSS.md` mit geprüfter Provenienz und bis zur Freigabe ausdrücklich `real_holdout_run: false`.
