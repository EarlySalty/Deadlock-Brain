status: aktiv
Datum: 2026-10-03

# Laufzeit vor dem Auftrag

`brain-serve.service`: MainPID 3506677, NRestarts 0, Start 2026-10-02 23:05:45 CEST. Laufende exe: `/opt/deadlock-brain/maintenance-releases/511a347b653beba13c2bf130f4bead7a7196cc2a/brain-serve`, ohne deleted-Markierung. Der eingebettete Releasepfad entspricht dem frisch geholten origin/main-SHA `511a347b653beba13c2bf130f4bead7a7196cc2a`.

`deadlock-brain-site.service`: MainPID 2002603, NRestarts 0, Start 2026-09-30 19:14:30 CEST. exe `/usr/bin/python3.12`. Bestehender Legacy-Dienst, keine Erweiterung oder Änderung beauftragt. Listener 127.0.0.1:8087 bestätigt. Ports 8088 und 8092 wurden mit derselben begrenzten Listenerprobe nicht gefunden; der Brain-Serve-Port ist damit noch nicht ermittelt.

Wiki-Refresh und Quellen-Synchronisationsdienste sind als bestehende User-Services vorhanden, zum Beobachtungszeitpunkt inaktiv. Es gab keinen Neustart und keine Datenänderung.

Ressourcenprobe vor Cargo: RAM verfügbar 13955 MiB, kein Swap; eigener Integrations-Buildpfad auf Dateisystem mit 609 GiB frei. Vor einem tatsächlichen Compilerstart erneut messen und beide vorgeschriebenen Sperren halten.

Diese Angaben belegen nur den Ausgangsstand. Sie sind kein Funktions- oder Abschlussbeweis.
