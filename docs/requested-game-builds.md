# Builds auf ausdrückliche Anfrage veröffentlichen

Stand: PR vom 24. September 2026, noch nicht ausgeliefert.

Der Discord-Befehl `/brain` kann eine ausdrückliche Bitte wie „Bau mir einen Build für Warden“ an den bestehenden Build-Reasoner und Steam-Publisher weiterreichen. Der Discord-Anteil liegt im Repository Deadlock-Bots auf `feat/brain-direct-build-publish-20260924` und benötigt die hier ergänzte CLI vor seiner späteren Auslieferung.

`publish-build-query -- "Warden Gun Build"` ist ein eigener, regulär geprüfter Schreibpfad. Er benutzt die bestehende Familien-, Patch-, Stichproben-, Kern- und Skillorder-Prüfung. Mehrere nicht ausgewählte Varianten werden nicht stillschweigend entfernt. Fehlt die Freigabe, lautet das Ergebnis `BLOCKED`; es wird kein Steam-Auftrag angelegt. Ein Hinweis erklärt, ob eine Stilwahl oder eine bessere Datenbasis fehlt.

`review-build` bleibt der bestehende, ausdrücklich experimentelle Pfad. Im vorhandenen offenen Brain-Testmodus nutzt der Discord-Verbraucher weiterhin diesen Pfad und kennzeichnet die Ausgabe als Review-Build. Diese Änderung aktiviert keinen Testmodus und verändert kein KI-Modell.

Beide Pfade warten begrenzt auf den bestehenden Steam-Auftrag. `PENDING` oder `RUNNING` bedeutet eingereiht, nicht veröffentlicht. Eine positive `hero_build_id` wird als veröffentlicht ausgegeben, wenn der Auftrag `DONE` meldet. Ein fertiger Auftrag ohne gültige Build-ID ergibt einen Fehler; bei fehlgeschlagenen oder unbekannten Zuständen wird kein Erfolg behauptet.

Die reine Wissensabfrage `ask-context` bleibt ohne Veröffentlichung. Im PR-Testbetrieb werden keine echten Builds hochgeladen und keine Produktionsdienste neu gestartet.
