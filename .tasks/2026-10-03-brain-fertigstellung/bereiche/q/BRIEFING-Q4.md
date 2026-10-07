status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-q

# Worker Q4: echten Provider-Shadow vorbereiten

## Wiederaufnahme und aktualisierter Umfang

Der vorige Worker wurde beim Sitzungsende gestoppt und hat kein Ergebnis oder Shadow-WIP hinterlassen. Starte erst, wenn die Hauptsession dieses Briefing erneut beauftragt, damit höchstens drei Unteragenten gleichzeitig laufen. Inzwischen sind C9-API- und Releasebindungen durch die autorisierten Cherry-picks `e48c189` und `5c220a8` übernommen; aktuellen HEAD und die reale Service-API prüfen.

Der Hauptorchestrator hat in `VON_HAUPT.md:33` mindestens 100 Fragen freigegeben: öffentliche Deadlock-Docs-FAQ, veröffentlichte Patchnotes, freigegebene Wiki-Inhalte im Kern und ausdrücklich markierte synthetische Fragen. Keine privaten Nutzer-/Communityfragen. Die unten genannten sieben veröffentlichten FAQ-Fragen sind ein echter Teilkorpus. Die fünf bereits im Kern freigegebenen öffentlichen Supportseiten haben insgesamt 24 H2/H3-Themen. Du kannst dazu je vier klar als synthetisch gekennzeichnete Varianten plus die sieben Originalfragen erzeugen, also 103 eindeutige Fälle. Jede synthetische Frage bleibt an ihre echte öffentliche Quellseite gebunden; kein aufgezeichneter Verkehr und keine unabhängige Goldlabel-Behauptung. Andere Quellen nur verwenden, wenn deren aktuelle Rechte samt gepinnter Revision externen Versand ausdrücklich erlauben.

Die freigegebenen weiteren Supportseiten sind `public/discord-server/dm-concierge.html`, `public/discord-server/voice-features.html`, `public/twitch-bot/auto-raid.html`. Prüfe alle fünf SourceRecordV2-Policies und aktuellen Release-Pins vor Versand; bloße öffentliche Sichtbarkeit reicht nicht. Q5 ergänzt parallel das redigierte Anfrageereignis; nutze dessen echten Serverpfad später zum Nachweis, verändere ihn aber nicht.

## Ziel und Vertrag

Ein bestehendes Rust-Antwortsystem ist bereits produktiv: `brain-serve.service`, PID beim Start 3506677, Binary `/opt/deadlock-brain/maintenance-releases/511a347b653beba13c2bf130f4bead7a7196cc2a/brain-serve`, API `127.0.0.1:8788`. Die Konfig liegt in `/home/nathanael/.config/deadlock-brain/brain-serve.json`, Infisical in `/etc/deadlock-brain/infisical.json`, Credential-FD 5. Z besitzt Dienst und Konfig, nichts davon verändern. Provider Q1 repariert parallel den vorhandenen zentralen Modellpfad und Denken aus. Kandidat und Bestand sollen auf demselben Corpus-Release, denselben Fragen, demselben Budget und freigegebenem Fireworks-DeepSeek-Flash verglichen werden. Kein Sondermodell und kein zweiter Connector.

Echte Fragen sind hier belegte veröffentlichte Support-FAQ, kein aufgezeichneter Community-Verkehr: Deadlock-Docs `public/discord-server/paten.html` enthält sechs echte H2-Fragen (Was ist ein Pate?, Wie bekomme ich einen Paten?, Was passiert nach meinem Patenwunsch?, Wie stoppe ich Angebote und die Weitergabe?, Wie werde ich selbst Pate?, Was tut ein Pate und was nicht?). `public/twitch-bot/chat-moderation.html` enthält Wie kann ich eine Maßnahme prüfen lassen? Herkunft ist `origin/main` des Docs-Repos bei `4b072aee3127564def674f4b53d9be5d1d42cfdf`. Neue Varianten sind gemäß bestätigtem erweiterten Umfang zulässig und ausdrücklich als synthetisch zu markieren. Aktuelle SourceRecordV2-Quellen `maintenance-docs:Deadlock-Bots:a1f98838c1141919e33035b7` und `maintenance-docs:Deadlock-Twitch-Bot:a7011b366a071ae2f232eb79` tragen Policy: public, Scope bot.public, authorization_ref workspace-request:2026-10-02:reviewed-existing-public-doc-corrections, publication_allowed=true, provider_egress_allowed=true. Lizenz bleibt als unbekannt dokumentiert; es handelt sich um bestehende eigene freigegebene öffentliche Supporttexte. Vor tatsächlichem Versand Policy und gepinnte Revision nochmal programmgesteuert prüfen.

## Eigentum

Baue einen eng begrenzten nativen Rust-Shadow-Runner als neue Binärdatei `rust/crates/brain-serve/src/bin/brain-provider-shadow.rs` mit optional neuen eigenen Untermodulen unter `rust/crates/brain-serve/src/bin/provider_shadow/`. Keine bestehenden Provider-/Service-/Config-Dateien bearbeiten, kein Manifest oder Lock, keine anderen Crates. Proben-/Ergebnisformat unter `architecture/migration/evals/q-shadow/` darfst du ebenfalls neu anlegen. Bestandssuche zuerst Graphify, bestehendes Evalformat und Service/Client/Secret-Funktionen wiederverwenden. Keine neuen Code-Kommentare.

## Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-fertig-q`, Branch `feat/brain-fertig-q-20261003`, HEAD `511a347b653beba13c2bf130f4bead7a7196cc2a`, andere native Worker schreiben parallel in disjunkten Pfaden. Keine Git-Mutationen oder live Aufrufe. Native Workerrolle, keine weitere Delegation. Falls eine Abhängigkeit fehlt, genauen Bedarf melden statt gemeinsame Dateien zu ändern.

## Beweisziel

Runner muss nach Integration read-only den tatsächlichen alten HTTP-Antwortpfad mit dem Kandidatenpfad vergleichen können. Kandidat darf in einem eigenen lokalen API-Port oder direkt über das bestehende Service-/Kernel-Objekt starten. Kein Neustart/Umkonfigurieren des bestehenden Dienstes, keine Source-/Release-Schreibzugriffe. Alle Ergebnisse müssen Release-ID, SHA/Quellstand, Case-Herkunft, Antwortstatus, Belege, Latenz, Nutzungs-/Kostenwerte soweit geliefert und Grenzen enthalten. Secrets und Frage-/Antwortinhalte aus fremden Nutzerdaten nie ausgeben. Die sieben Fragen und genehmigten FAQ-Belege dürfen im internen Ergebnis stehen. Modell aus der vorhandenen zentralen Auswahl, Timeout aus Servicekonfig. Egressrechte fail-closed prüfen; kein unbekannter Corpus, keine internen SourceRecords als Beleg. Bestand und Kandidat dürfen nicht nur ein Fake für den riskanten Provider sein. Du bereitest das Werkzeug vor, die Hauptsession startet den echten Lauf nach Prüfungen.

Tests/Compiler nur mit Rust 1.97.1, höchstens zwei Jobs, beiden blockierenden Hostlocks und frischer NonZombie-Probe nach `HOSTPROBE.md`. Momentan können parallele Manifeständerungen das Lock noch ungültig machen. Dann nicht kompilieren, sondern fertigen Code samt Prüfbedarf melden. Keine fest eingebaute Modell-ID, kein neues Modell oder selbst erfundenes LLM-Zeitlimit.

## Routing

Auftraggeber Teil-Orchestrator Q, Session `c671588c-6192-4bf5-8206-28bb30163666`; Hauptorchestrator `43a4886c-e135-484b-838a-0512d224a634`. Status allein durch `teil-q`, Versuch 1. Nie `TODO.md` oder `REGISTER.md` schreiben. Ergebnisdatei `bereiche/q/Q4-SHADOW.md` im gemeinsamen Auftragsordner `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung`. Kein Bug-/Securityreview außerhalb des Merge-Gates.
