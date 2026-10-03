status: aktiv
Datum: 2026-10-03
Stand: 2026-10-03T07:50:30Z

# Enger Installationsplan C2

Entscheidung: vorhandene unveränderliche SHA-Releases und Startwege beibehalten. Notwendige Installation als einmalige gesicherte Betriebsfolge im Deployauftrag durchführen, keine neue Plattform oder dauerhaften Helfer bauen. Keine laufende Binary überschreiben. Erst nach jeweiliger Sol-high-Freigabe, passenden grünen Compilerprüfungen und Merge/Push; gebaut wird im eigenen sauberen Worktree exakt der frisch verifizierte origin/main-Stand des jeweiligen Repos. Zwischenablagen, Fingerprints und Source-SHA eindeutig binden, niemals Dateialter als Herkunftsbeweis verwenden.

## 1. Exakte Ziele und bestehende Rechte

Brain: neues ausschließlich eigenes Verzeichnis `/opt/deadlock-brain/maintenance-releases/<Brain-main-SHA>/`, Dateien `brain-serve`, `brain-maintain` und einmaliger Importer `brain-knowledge-import`, dazu eigenes `SOURCE_SHA` und `SHA256SUMS`. Neue Binärdateien root:root 0755 entsprechend gemessenen bestehenden Brain-Binaries; Verzeichnis root:root 0755, kleine Provenienzdateien root:root 0644. Fertig vorbereitete Stage auf demselben Dateisystem erst nach Hashprüfung ohne Überschreiben eines vorhandenen Releases unter den finalen SHA-Namen veröffentlichen. Vorhandene Releaseverzeichnisse bleiben unangetastet.

Brain-Zeiger ausschließlich `/opt/deadlock-brain/maintenance-current`. `/opt/deadlock-brain/current` gehört dem früheren V1-Layout und wird für diesen Auftrag nicht verändert. Betroffene Userunit `brain-serve.service`, vorhandener Drop-in `~/.config/systemd/user/brain-serve.service.d/90-maintenance.conf` unverändert. `brain-maintenance.service` nutzt dasselbe Maintenance-Releaselayout; dessen normalen Oneshot-/Timerpfad vor Wechsel auf laufenden Schreibjob prüfen. Keine gleichzeitige Runtime-Aktivierung oder blind ausgelöste Maintenance-Wirkung.

Steam: neues ausschließlich eigenes `/opt/deadlock/steam/releases/<Steam-main-SHA>/` mit beiden vorhandenen Dateien `steam-core` und `steam-bot`, eigenem `SOURCE_SHA` und `SHA256SUMS`; root:root 0555 für Binaries wie tatsächlich gemessen, Verzeichnis 0755. Zeiger ausschließlich `/opt/deadlock/steam/current`. Betroffene Userunits `steam-core.service`, `steam-core-2.service`, `steam-bot.service`. Bestehende Bootstrap-/LoadCredential-/Startskripte, Accounts, Roots und SDK-Loginverfahren unverändert. Keine zweite Steam-Verbindung für den Download aufbauen.

## 2. Sperre und atomarer Wechsel

Hostprüfung und Releasebuild unter den bereits verbindlichen beiden Hostlocks; frische Probe, maximal zwei Jobs, nur eigener Prüflauf. Installation zusätzlich pro Stack per blockierendem Root-flock auf `/run/lock/deadlock-brain-release-install.lock` beziehungsweise `/run/lock/deadlock-steam-release-install.lock`, gehalten durch Vorbereitung, aktuellen Herkunfts-/Zeigercheck, Wechsel, Neustart und Liveprüfung. Ein enger einmaliger Befehlsablauf ist ausreichend; kein ungesicherter separater Current-Schritt.

Unter gehaltener Installationssperre Source-SHA nochmals gegen frisches origin/main prüfen und den tatsächlichen bisherigen Current-Zielpfad als Rückrollziel sichern. Temporären eigenen Symlink neben Current auf vollständig geprüften SHA-Release anlegen, Ziel/Hash prüfen, dann per atomarem Rename auf Current wechseln. Vorhandene Current-Datei vorher inspizieren; unerwarteter Typ, Fremdstage, abweichender Provenienzstand oder unvollständiger Release führt zum Abbruch vor Wechsel. Keine fremden Stages löschen oder übernehmen.

## 3. Daten und Konfiguration

Keine neuen Rollen, Grants, Secretdateien, Credentials, Bootstrapwrapper oder systemd-Templates. Brain benutzt unveränderte normale Infisical-Referenz und minimale Rollen. Der neue CLI erhält vor tatsächlichem Import die noch zu implementierende explizite dedizierte Runtime-Zielbindung und Identitätsprüfung. Erst nach Daten-/Wiederholungs-/Releaseprüfung bestehendes `brain-maintain write-serve-config` für den tatsächlichen Releasepin und die Knowledge-Version einsetzen, mit vorhandener Lock-/Hashvergleichs-/atomarer Schreibmechanik. Aktuelle Config `~/.config/deadlock-brain/brain-serve.json` nathanael:nathanael 0600 und Runtime `/etc/deadlock-brain/maintenance-runtime.json` ebenso erhalten; vor Änderung normale Config bytegetreu privat sichern. Keine Secrets auslesen oder sichern.

D-Zwischeninstallation erst mit geprüftem D-Stand und gemeinsam statisch geprüfter D/B-Anbindung. Späterer tatsächlicher Depotdownload über bestehenden autorisierten Core-Task; unmittelbar beim B-Lese-/Importübergang exakte Inventar-/Dateibindung prüfen. Fremddateien nicht unter Manifestherkunft übernehmen und niemals löschen. Finaler Brain-Deploy erst auf vollständigem gemeinsam abgenommenem Integrationsstand.

## 4. Neustart und Livebeweis

Normale bereits bestehende Unitstarts verwenden und Schutz-Hooks respektieren. Steam über vorhandenen zulässigen `bot-restart`-Weg, nur geplante drei betroffene Units; Brain normaler User-Service-Restart nach kontrolliertem Configwechsel. Tatsächliche Prozess-Binary suchen, nicht nur MainPID: heute sind die Steam-MainPIDs sudo, darunter dl-infisical-env und erst dann die beiden Release-Binaries. Vor/nachher reale Binarypfade und SHA256 mit Release-SHA/Fingerprints, Units/Restartzähler, Health/Ready, kontrollierte Funktionen und laufende Source-/Corpusversion nachweisen. Keine Prozessumgebungen, Secrets oder Communitynachrichten.

## 5. Rückweg

Aktuell gemessene Rückrollziele: Brain `/opt/deadlock-brain/maintenance-releases/511a347b653beba13c2bf130f4bead7a7196cc2a`, Steam `/opt/deadlock/steam/releases/4c5621763d5f01c96d7912400517c08aa1c40df1`. Unmittelbar vor Installation neu messen; ein inzwischen legitim neuerer Current ist das tatsächliche Rückrollziel, nicht blind dieser Bericht. Haupt hat die einmalige Installation nach gültiger Abnahme/Gate/grünen Prüfungen ausdrücklich bestätigt. Zusätzliche verbindliche Rückrollbedingung: unter derselben Sperre unmittelbar vor Rücksetzen nachweisen, dass Current noch exakt auf unseren eigenen gerade installierten Release zeigt. Bei zwischenzeitlich fremd verändertem Zeiger abbrechen und Befund melden, niemals blind zurücksetzen. Für normale Configrückführung ebenso vorher prüfen, dass ihre aktuelle SHA noch unserer eigenen installierten Config entspricht. Erst bei passenden Vorbedingungen den tatsächlich vorher gesicherten Zeiger atomar zurücksetzen, normale Config kontrolliert zurückführen, nötige Units normal starten und alte Prozesshashes/Readiness prüfen. Keine Datenbank-Rücksetzung, Tombstone-Wiederbelebung oder Löschung von importierter Historie.

Noch kein Build-/Install-/Config-/Restart-Schritt dieser Folge ausgeführt. Aktueller Revisionsfixer und sein einzelner sicherer Prüfwrapper bleiben erhalten. Vor nächster Phase wird geordneter C2-Kontexthandoff mit tatsächlichen Worker-/Taskzuständen vorbereitet.
