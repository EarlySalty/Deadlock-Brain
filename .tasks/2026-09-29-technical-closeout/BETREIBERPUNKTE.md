status: aktiv
Datum: 2026-09-29

# Betreiber- und externe Grenzen

Diese Liste ersetzt keine technische Abnahme. Implementierung, Tests und unabhängige Reviews laufen weiter. Kein offener Codefehler wird durch einen Freigabepunkt verdeckt.

## G5

Ausdrückliche Betreiberfreigabe fehlt in diesem Auftrag. PR #40 bleibt Draft nach main. Kein Production-Cutover, keine produktiven Consumer, keine echten Nachrichten oder Steam-Veröffentlichungen.

## Wiki-Realpilot

Kein aktueller Nachweis für die gemeinsam nötige Freigabe von Quelle/Capture, Lizenz und Raw-Aufbewahrung gefunden. VORCHECK.md nennt die vorhandenen Vertragspfade. Deshalb kein echter Netz-Capture. WIKI_REAL_PILOT_PASSED bleibt NEIN, auch wenn Offline- und Storetests grün werden.

## Echter LLM-Shadowtest

Keine belegte Brain-spezifische Freigabe für Provider, Modell, Egress, Budget und Credentials gefunden. Die allgemeinen Bot-Modellregeln sind keine eigenmächtig erweiterbare Freigabe für einen neuen Brain-Test. PROVIDER_SHADOW_PASSED bleibt NEIN, solange dieser Nachweis fehlt. Keine neuen Modelle gewählt und keine Nutzerdaten nach außen gesendet.

## Echter Replay

Kein bereitgestellter berechtigter Replay in den Brain-Worktrees gefunden. Ergänzende lesende Dateinamensuche in /home/nathanael, /srv und /var/lib fand keine .dem; 41 nicht lesbare Verzeichnisse und ausgeschlossene Git-/Build-/Cache-Verzeichnisse begrenzen die Aussage. Keine Datei heruntergeladen, keine Match-ID geraten.

Betreiberentscheidung bleibt offen: Replay in V1 bedeutet G2 wartet auf freigegebene Datei und realen Durchstich. Replay später bedeutet Funktion deaktiviert lassen und eigenes späteres Gate. Diese Entscheidung wurde nicht vorweggenommen. PR #46 enthält einen noch nicht integrierten lokalen Validierungshelfer, keine echte Realmatch-Abnahme.

## GitGuardian

Incident 37635766 ist im aktuellen #40-Check weiterhin offen. Der historische Fund benennt Auth-Modus und Passwort-Environment-Variablennamen, keinen eingebetteten Passwortwert. Normale Dashboard-Aktion: https://dashboard.gitguardian.com/workspace/755270/incidents/37635766?occurrence=299625425 öffnen, Fund bestätigen und als False Positive beziehungsweise Test Credential schließen. Browser-Automation meldete keinen verfügbaren Host; die Aktion wurde nicht ausgeführt. Kein History-Rewrite oder Check-Bypass.

## Neue CI-Abhängigkeitslücke

Frische echte CI-Läufe scheitern an den nicht mehr öffentlich abrufbaren gepinnten Git-Repositories haste, valveprotos-rs und dungers. Lokale exakte Git-Objekte sind vorhanden. Für dungers fehlt bisher ein Lizenzbeleg im gepinnten Commit. Paket G hat 29 erreichbare historische Commits ohne Lizenzbeleg geprüft und den Zugriffsfehler mit einem frischen Cargo-Home und Cargo/rustc 1.98.0 bestätigt (G-REPORT.md, 1aff547). Keine fremde Quelle wurde ohne Lizenzbeleg verteilt. Der verbleibende externe Blocker verlangt einen belegten Quellzugang beziehungsweise Lizenznachweis für den exakten dungers-Pin; lokale Offline-Prüfungen können trotzdem weiterlaufen. Kein erledigter CI-Fix.
