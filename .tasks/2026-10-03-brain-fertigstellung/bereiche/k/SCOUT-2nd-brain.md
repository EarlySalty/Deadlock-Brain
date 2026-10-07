status: erledigt
Datum: 2026-10-03

# Bestand Second-Brain

Scout-Workflow wf_5aa6a58a-ad8 endete nach einem erfolgreichen Second-Brain-Worker und zwei terminalen Proxy-403 bei Docs und Twitch. Der Rohbericht liegt im Workflow-Journal. Der Scout untersuchte den älteren PR-Kopf ab83b691befd761a16d971af5c249a604c5d4e0d; währenddessen übernahm die Paketführung den vorhandenen neueren sicheren Sol-Stand afc30f059d3ce4eab9fa402d3c08e505e5dff923. Maßgeblicher eigener Baukopf ist zunächst 641a2bc.

## Belegter produktiver Bestand

- brain-serve PID 3506677 läuft aus maintenance-releases/511a347b653beba13c2bf130f4bead7a7196cc2a. Wirksames Drop-in: /home/nathanael/.config/systemd/user/brain-serve.service.d/90-maintenance.conf.
- Wirksame Kernkonfiguration: /home/nathanael/.config/deadlock-brain/brain-serve.json. TCP-Adresse 127.0.0.1:8788. Die Vorlage des alten PR nannte fälschlich 8787.
- Im aktiven Kern gibt es bisher die Identität twitch-bot mit bot.public, keine freigegebene Second-Brain-Identität und keinen privaten Operatorpfad.
- Es besteht noch kein installierter Second-Brain-Adapterdienst oder belegter Adapter-Deployweg. Der Bot-Neustartwrapper umfasst diesen Consumer nicht.
- Der Wochen-Feeder dl-brain-feeder ist ein eigener Aggregat-/Planpfad. PR #2 betrifft ausdrücklich den internen Operator-Abfrageport, keine Umstellung des Wochen-Planlaufs.

## Vertrag des neueren übernommenen Stands

Die normale Bot-TOML, der geerbte private Bootstrap-FD, das dedizierte Infisical-Secret BRAIN_SERVE_SECOND_BRAIN_TOKEN und der private Operator-Unixsocket bleiben erhalten. Brain-Revision 32f603219ad896182693dd85639ac19793272042 ist fest eingebunden. Identität ist serverseitig am Credential-Grant gebunden; das Präfix einer Request-ID ersetzt diese Bindung nicht.

Q übernimmt laut VON_HAUPT.md den fehlenden Operatortransport und die redigierte serverseitige Anfrageprotokollierung. K baut den aktualisierten Consumer weiter und belegt nach Kernintegration einen echten Aufruf mit Antwort, auflösbarem Beleg und korrelierter Consumer-Journalzeile. Keine Aussage zur Umstellung des Feeders daraus ableiten.
