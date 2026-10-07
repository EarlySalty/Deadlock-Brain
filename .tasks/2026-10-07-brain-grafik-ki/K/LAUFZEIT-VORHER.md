# K: Lesender Laufzeitbestand vor Integration

Nur Bestandsprobe, kein K-Deploy oder Inhalts-/Kanalnachweis. Keine Konfiguration geändert, keinen Dienst gestartet oder neu gestartet und keine Nutzerfrage gesendet.

## Brain

brain-serve.service aktiv, MainPID2645590, NRestarts0. /proc/2645590/exe zeigt /opt/deadlock-brain/maintenance-releases/bfda408cb988722ddceadb56bca5b72e12d12731/brain-serve ohne deleted. /opt/deadlock-brain/maintenance-current zeigt denselben Releasepfad. Loopbacklistener127.0.0.1:8788. Vorhandene /healthz und /readyz jeweils HTTP200, Body nicht gelesen. Dieser alte Release ist nicht K-Feature oder aktueller origin/main.

deadlock-brain-site.service aktiv, MainPID2002603, NRestarts0, exe /usr/bin/python3.12. Der in K integrierte bestätigte Rust-Siteport ist noch nicht ausgeliefert. Python-Laufzeit weder geändert noch erweitert. Dienst erst nach regulärem gemeinsamem Abschluss gegen Rust ersetzen, kein falscher Port-/Deploybeweis.

Erster readlink-Befehl erhielt Exit1 allein wegen einer falschen zusätzlichen Repo-Linkadresse; beide Prozess-exe-Ausgaben waren vorhanden. Tatsächlichen /opt-Link gesondert erfolgreich gelesen. lsof zeigte Docker-Mountwarnungen; der konkrete Brainlistener war sichtbar. Keine Rechte-/Containeränderung wegen der Warnungen.

## Twitch

/usr/local/bin/deploy-twitch-release ist vorhanden. Lesender `deploy-twitch-release --pruefen` Exit0. current /opt/deadlock/twitch/releases/10dacbc2376a63f6d91869afe83b1ac8bb615eec. Alle vier geprüften System-Units aktiv, exe gleicher Release-SHA, kein deleted, NRestarts0:

| Unit | MainPID |
| --- | --- |
| deadlock-twitch-bot-rust | 3874592 |
| deadlock-twitch-dashboard-rust | 3874704 |
| deadlock-twitch-stream-coaching-watch | 3821592 |
| tb-category-collector | 3796733 |

Keine Twitch-Antwortprobe, kein K-Deploy oder aktueller Mainvergleich aus diesem Baselineaufruf abgeleitet. Nach eigener Auslieferung Wrapper erneut ausführen sowie Journal und tatsächliche Funktionsprobe getrennt belegen.
