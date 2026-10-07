status: beauftragt
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: vorhandener eigener Twitch-Worktree, nur lesen

# Vorhandenen earlysalty-Sender jetzt rein lesend prüfen

## Neue Entscheidung

VON_HAUPT.md Abschnitt 17:41 UTC lesen. Die bisherige Beschränkung deines ersten Briefings auf reine Quellen ist für genau diese bestehende Identitäts-/Scopeprüfung aufgehoben. Prüfung jetzt vor Deployment, ausschließlich lesend über den vorhandenen sicheren Auth-/Tokenweg. Nutzer earlysalty ist für den Funktionstest ausdrücklich freigegeben. Kein neuer Thread oder Modellwechsel, deinen bereits erarbeiteten Quellenkontext erhalten.

## Enger Prüfauftrag

Über vorhandene geschützte Auth-/Helixwege früh die stabile Twitch-User-ID des freigegebenen Kontos earlysalty bestätigen. Für genau dieses Konto prüfen, ob ein vorhandenes tatsächliches Usercredential gültig ist und user:write:chat wirklich erteilt ist. Bereits validierte Botidentität ebenfalls nur als Nicht-Bot-Abgrenzung verwenden. Kein Scopebeleg aus bloßem Quellprofil oder Fixture. Keine fremden Konten enumerieren oder Credentials anderer Streamer abrufen.

Vorhandene reine Metadaten-/Validierungswege bevorzugen. Credentials niemals ausgeben oder speichern: keine Dumps, vollständigen Antworten, Argumente, Umgebungswerte, Dateien, neue Tokenablage oder CI-/Chatlogs. Schutztransport wie im vorhandenen Code: benannte Secrets ausschließlich über bestehenden geschützten Infisical-Weg, keine Redirects oder Proxys, genaue vorhandene Projekt-/Pfadbindung. Nicht gesamte Secretlisten laden. Auth- und Helixantworten nur im Speicher auswerten und ausschließlich erlaubte stabile ID und Scopeboolean ausgeben. Kein interaktiver credentialhaltiger Response als Ausgabe.

Rein lesend bedeutet auch: keinen Tokenrefresh mit DB-Schreibfolge auslösen, keinen neuen Grant, keinen OAuthstart, keine erneute Autorisierung, keine Config-, Konto- oder Chatmutation. Vor der Nutzung eines GET-Tokenwegs dessen mögliche Refreshnebenwirkung am bestehenden Code prüfen. Falls nur ein schreibender oder tatsächlich unzulässiger Weg verfügbar ist, konkrete bestehende Aktion und fehlende Voraussetzung melden. Einen tatsächlichen Rechte-/Genehmigungs-Deny nicht umgehen, keinen generischen privilegierten Helfer, Sender oder HTTP-/Diagnoseweg bauen.

## Ergebnis und nächste bestehende Aktion

Rückgabe nur stabile Twitch-User-ID, Bezug zum ausdrücklich freigegebenen Konto, tatsächliches user:write:chat Ja/Nein beziehungsweise präzises nicht prüfbar mit belegter Aktion/Grund. Wenn verfügbar, den bereits vorhandenen Helix-/Authweg für genau eine spätere Testfrage festlegen, aber nicht ausführen. Falls Scope wirklich fehlt, die bestehende OAuth-Erweiterung/erneute Autorisierung als konkreten Nutzer-Schritt benennen, ohne sie jetzt anzustoßen. Keine fremde Identität und kein zweiter OAuthweg.

## Grenzen und Routing

Kein Code, Compiler, Tests, Review, Gate, Installation, globaler Chatmodus, Chatnachrichten, Moderation oder Discord-Nebenwirkung. Graphify vor neuen Codefragen, Codepfade aus deinem bisherigen Quellenkontext wiederverwenden. Keine weiteren Agenten oder T3-Threads. REGISTER.md und TODO.md nicht schreiben. Haupt e6c19079-657e-4db9-80bd-8e1313e7f785, teil-k Versuch 1. Rückgabe an teil-k; Akten pflegen sie. Deutsch mit echten Umlauten, humanizer und no-em-dashes.
