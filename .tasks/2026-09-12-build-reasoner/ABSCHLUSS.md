# Technischer Integrationsstand, 13.09.2026

Der Gesamtauftrag ist fachlich **nicht erfüllt**. Der echte Warden-Build 779996,
Version 45, enthält 19 Kernitems, darunter 9 Waffenitems laut ItemModel.slot.
Der korrigierte Reasoner trifft davon 3 insgesamt und 2 Waffenitems
(Titanic Magazine und Frenzy). Recall 0,157895, Jaccard 0,09375 bei 16 Käufen.
Die geforderte Waffenmehrheit wird nicht erreicht. Es gibt keinen Nachweis
einer unabhängigen Meta-Vorhersage oder des behaupteten Nightshift-Wechsels.

## Technischer Umfang

F und G sind gemeinsam ab main eae2330 integriert. Stand der Quellenkorrektur:
6fa027f; endgültige Ask-Projektion: 9bcc588; vorheriger gemeinsamer
Mechanik-/Composerstand: be107d7. Ask und
Publish verwenden denselben Reasoner. Der sichere lesende DB-Einstieg bezieht
Secrets im Prozess aus Infisical über einen geerbten Credential-Dateideskriptor.
Bestehende Dienststarts bleiben kompatibel; kein neues Modell, keine neuen
Secret-ENV-Wrapper und kein Testupload in ein Spielkonto wurden eingeführt.

Korrigiert sind die gemeinsame Definition der Pflichtkaufkurve, Phasen nach
Kostenband, Shopbonus im Soulwert, bedingte gegenüber dauerhaften Effekten,
echte Reloadzyklen, flache gegenüber prozentualen Magazingrößen, Spirit-Power
zu Waffenfeuerrate sowie Imbue-Ziele nach tatsächlichem Fähigkeitsnutzen mit
gültigen IDs. Schwellen, Nichtheldenboni und Tooltip-Hilfswerte werden nicht
als garantierter Heldenschaden addiert. Unbelegte bedingte Heilung wird als
unquantifiziert erklärt. Verkaufsbelege und Kombinationen verwenden den
aktuellen Besitz; verkaufte Partner beeinflussen spätere Käufe nicht weiter.

Kombinationen wirken tatsächlich in der schrittweisen Auswahl. Ihre zusätzliche
Bewertung ist beobachteter Paar-Winrate-Lift gegenüber den Einzelitems, kein
mechanisch durchgerechneter Nutzen einer vollständigen Kampfrotation.

## Quellen und Grenzen

Die letzte Ursache war die Vermischung des gesamten GC-Katalogs mit den
beobachteten Autoren. 2550 aktuelle Katalogbuilds standen 64 beobachteten
Builds gegenüber; für Warden waren es 67 Builds von 64 Autoren gegenüber
2 Builds aktiver beobachteter Autoren. Ein gemeinsamer Loader bestimmt zuerst
die neueste Buildversion und löst danach die aktuelle author_account_id gegen
is_active in watched_build_authors auf. Layout, Kern-/Situationsrolle,
Fähigkeitsreihenfolge und Backtest verwenden dieselbe Quellenmenge. Fehlende
eigene Heldenreferenzen sind sichtbar; das globale Layout beobachteter Autoren
ist dann nur ein Behelf mit niedriger Confidence, kein eigener Vergleich.
Der Screenshot-Seed bleibt ausschließlich ein separat ausgewiesener Vergleich.

Auch der beobachtete Autorenvergleich ist in-sample: Autoren beeinflussen die
Auswahl und liefern die Vergleichskurve. Mehr Übereinstimmung wäre deshalb
noch keine unabhängige Prognose. Vor Quellenkorrektur hatte Warden 12 Käufe,
2/19 Gesamttreffer und ebenfalls 2/9 Waffentreffer. Die neue Auswahl hat eine
korrektere Quellenbasis, erfüllt das fachliche Ziel aber weiterhin nicht.

Die verbleibende Ursache ist ein fehlendes gemeinsames Modell kompletter
Builds und Kampfrotationen einschließlich Utility und wechselnder Zustände.
Einzelitem-DPS/EHP, Kaufbonus und empirische Paarstütze ersetzen diese
Bewertung nicht. Globale Warden-Gewichte oder ein Kopieren von 779996 würden
keinen belastbaren Fix liefern und wurden nicht eingebaut. Historisch
unabhängige Patch-/Autorenstände fehlen für den geforderten Wechselnachweis.

## Nachweise und Reproduktion

Nicht sensitive Rohbelege liegen dauerhaft unter
`/home/nathanael/Documents/.tasks/2026-09-13-build-reasoner-abschluss/nachweise/`:

- `FINAL-LIVE.json`: kompletter 38-Heldenstand vor der Quellenkorrektur.
- `WATCHED-FINAL-LIVE.json`: aktueller vollständiger Lauf mit beobachteten Quellen.
- `AUTOR-PROVENIENZ.json`: separate Katalog-/Autorenmengenaufnahme.
- `FINAL-MECHANIK-VERGLEICH.json`: offline, gleiche Modelle/Referenz und identische
  skalare Meta-Signale, Paarstatistik auf beiden Seiten ausgeschaltet. Die
  mechanisch korrigierte Auswahl verbessert die Trefferquote nicht.
- `ask-publish-smoke.json`: echter read-only Ask und Payload-Vertragsprüfung,
  kein Upload. Endgültig 16 Käufe, 5 Kategorien, 3 gültige Imbue-IDs,
  5 Verkaufsprioritäten und 16 Fähigkeitsschritte. Vollständiges BuildObject
  entspricht der direkten Reasoner-Ausgabe. Der KI-Prompt enthält eine
  kompakte Projektion (14.030 statt 125.466 Zeichen); vollständige Belege
  bleiben im Buildkontext. Kategoriegröße 780 × 260 reicht für 16 Items.

Reproduzierbarer Einstieg: `dbrain-reasoner/examples/abschluss.rs`, optional
`warden`, `catalog` oder `provenance`; offline Mechanikvergleich:
`examples/mechanik_vergleich.rs`. Read-only ist im Live-Example explizit geprüft.
Die großen JSON-Rohbelege werden nicht ins Git kopiert. Die kompakte
`ABSCHLUSS-MESSUNG.json` enthält alle 38 Helden; 32 haben zusammen 64
beobachtete Referenzbuilds. Für die anderen sechs stehen Kennzahlen auf null.
Der aktuelle Rohbeleg hat SHA256
`ae77fddcb9c40179f85a603ba94680fda6ee3e8377d05db54a6cf037fabba1f0`.
Patchkennung: `https://steamcommunity.com/games/1422450/announcements/detail/676255623445218602`.

Workspace-Tests und Clippy aller Targets mit `-D warnings` auf 6fa027f sind grün.
Logs: `/tmp/brain-watch-workspace-final.log`, `/tmp/brain-watch-clippy-final.log`.
Formatprüfung der geänderten Dateien und `git diff --check` sind grün.
Das gesamte Workspace-Format hat bekannte Altlasten außerhalb des Fixumfangs.
Technische unabhängige Abnahme, endgültiges Gate und Deployment werden getrennt
im Abschlussnachtrag festgehalten; diese Datei behauptet sie nicht vorab.
