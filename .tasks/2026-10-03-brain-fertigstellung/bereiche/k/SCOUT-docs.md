status: erledigt
Datum: 2026-10-03

# Bestand Docs-Consumer

Bestandsworker a8dd7a89cda02986d, Workflow wf_0bde6080-123, nach einmaligem Proxy-403 erfolgreich beendet. Rohbericht im Workflow-Journal.

Der übernommene eigene Kopf 3e570a8aa0bf867bf1baf35b064165804b77fcb4 ist im vollständigen Baumvergleich identisch mit dem sicheren Sol-Präfix 14455aebb48db6aef82dfd94d173ebdf25a6cf0e. Die fünfzehn Adaptercommits sind damit bereits wiederverwendet. ae023d3 und 600abfd enthalten ausschließlich gekoppelte Community-Hilfetext-/Auditänderungen und bleiben außerhalb von K.

## Vertrag

DocsBrainAdapter verwendet AsyncBrainClient::new_local mit festem Scope docs.public. Request-IDs tragen docs-brain-. Serveridentität ist docs-client im Kanal docs. BrainClient ist auf 3b86d3cbe5ea39a67b8b1fbd8a3d48ab935982ef gepinnt. Bootstrap kommt positionsunabhängig über einen privaten regulären FD5 oder die ausdrücklich gewählte bestehende Credentialdatei. Der Adapter fragt über den lokalen Infisical-Socket genau BRAIN_SERVE_DOCS_PUBLIC_TOKEN ab. Der private Second-Brain-Operatorweg ist ausgeschlossen.

## Laufzeit

Die normale /home/nathanael/.config/deadlock-docs/bot.toml fehlt. Der aktuelle Kern auf 127.0.0.1:8788 hat keinen Docs-Grant. Release: maintenance-6edd9236639f9b06067a3b6b67fa410e5e668aa264db1b2baf28b7e1733940de. brain-serve läuft aus maintenance-releases/511a347b653beba13c2bf130f4bead7a7196cc2a, PID 3506677. Das Journal enthält keinen bisherigen Docs-Verbraucherbeleg.

Vorhandener Korpusdeploy: public-corpus-refresh.service verwendet /opt/deadlock-docs/current/tools/deploy_corpus.sh und ops/public-corpus-refresh.json. current zeigt auf releases/brain-hilfefragen-20261002; Profil ist mit fetch=false eingefroren. Ziel des Reloads ist dl-knowledge auf Port 8896. Dieser Weg installiert den CLI-Query-Adapter nicht. Die Docs-README startet ausdrücklich keinen eigenen Antwortdienst; keinen neuen Antwortdaemon daraus bauen.

## Sicherer Live-Beweis

Echte lesende Frage: „Wie erstelle und verwalte ich eine Casual-Lane auf dem Discord-Server?“ Öffentliche Belegseite: public/discord-server/workflows/voice-lane-erstellen-verwalten.html. Ein echter Aufruf muss eine fachliche Antwort mit auflösbarem öffentlichem Beleg und korrelierter Docs-Request-ID im authentifizierten serverseitigen Anfragejournal liefern. unavailable oder Exit 0 reicht nicht.

Offen: Compilerprüfungen, dauerhafter CLI-Ausführungsweg, normale TOML, vertrauenswürdiger FD-Starter, Q-/Z-Serverfreigabe und öffentlicher Docs-Release. Keine Secrets abgefragt, keine fremden Thread- oder Deployzustände verändert.
