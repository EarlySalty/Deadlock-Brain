status: Teilübergabe geprüft, Paket K arbeitet weiter
Datum: 2026-10-03

# Consumerstände für Z

Die lokal geprüften Docs- und Second-Brain-Stände können in die gemeinsame Integration übernommen werden. Kein eigener main-Merge, Deploy oder Releasewechsel. Twitch-Konfigurationswerkzeug ist vollständig lokal geprüft, frische Intent-Abnahme läuft. Botsübernahme ist mit erhaltenem ungeprüftem Stand blockiert abgeschlossen. Paket K ist noch nicht vollständig übergeben oder produktiv abgeschlossen.

| Consumer | Eigener Quellstand | Prüfung und Abnahme | Zustand |
| --- | --- | --- | --- |
| Docs | 3e570a8aa0bf867bf1baf35b064165804b77fcb4 | fmt/Test/Clippy Exit 0, 22 Tests; frische Intent-Abnahme Ja; lokale Vorabprüfung gpt-6.1-sol ALLOW | Zur gemeinsamen Integration bereit |
| Second-Brain | 54979646adde835335fa24ddd2545e10f996df52 | fmt/Test/Clippy Exit 0, 16 Tests; frische Intent-Abnahme Ja | Zur gemeinsamen Integration bereit |
| Twitch | 84ce376c9ea7aab69116fc7399183032308d712e | 82 Tests, Format und vollständiges Clippy Exit 0; BAU-TWITCH.md, frische Intent-Abnahme wf_4941a30e-893 läuft | Lokal geprüft, frische Intent-Abnahme offen |
| Bots | a96de1d5c00d1771fafc351e2046922d00ce045d plus drei ungeprüfte uncommittierte Korrekturen | Elf sichere Commits übernommen, Launcherfixes erhalten; alle zehn Checks Exit 127 wegen Rustup-PATH, keine Tests oder Compiler; BAU-BOTS.md | Nicht zur Integration bereit; Schutzrecheck running, Sourcewrites und Commits pausiert |

## Quellübergabe

Docs: /home/nathanael/.worktrees/Deadlock-Docs-brain-consumer-fertig, Branch feat/brain-consumer-fertig-20261003, Basis 4b072aee3127564def674f4b53d9be5d1d42cfdf. Fünfzehn vorhandene Adaptercommits übernommen, ganzer Baum identisch mit sicherem Sol-Präfix 14455aebb48db6aef82dfd94d173ebdf25a6cf0e. Eigener Branch ist nach origin gepusht. Fremder divergenter lokaler main bleibt unberührt.

Second-Brain: /home/nathanael/.worktrees/Deadlock-2nd-Brain-brain-consumer-fertig, Branch feat/brain-consumer-fertig-20261003, Basis 44229978e1515712e589fa1df05c6a02ad8c6394. Sieben vorhandene sichere Adaptercommits plus belegte Betriebsdokumentation übernommen. Eigener Branch laut Worker nach origin gepusht. Kein Feeder-Port in diesem Stand, raw/ unverändert.

Prüfbelege jeweils .consumer-ci-reports/ im eigenen Worktree. Auftragsabnahmen: INTENT-DOCS.md und INTENT-SECOND.md. Bauakten: BAU-DOCS.md und BAU-SECOND.md. Lokale Docs-Gateantwort: GATE-DOCS-R1.log und REVIEW.md. Der lokale ALLOW ersetzt die gemeinsame Abnahme und das Gate über Zs Integrationsstand nicht.

## CLI-Betriebsvertrag

Beide Consumer sind on-demand Rust-CLIs, keine neuen Antwortdaemons. Der konkrete Vertrag steht in BETRIEBSVERTRAG-CLI.md: tatsächliche Binarynamen, enges TOML-Schema, SHA-gebundene Standard-Cargo-Installation, bestehender LoadCredential-/FD5-Starter und Request-ID-Korrelation. Installation erst nach gemeinsamer Integration in main, gebunden an den tatsächlichen Repo-SHA und Binaryhash. Releasebau nur im eigenen Worktree, mit beiden vorgeschriebenen Hostlocks, konservativer Compilerprobe und Rustup. Der vorhandene User-Service-Credentialweg gibt das bestehende private Bootstrap /home/nathanael/.config/infisical-tokens/infisical-token-bots als LoadCredential in den vertrauenswürdigen Starter; dieser öffnet FD5. Keine neue persistente Credentialdatei oder Geheimniswerte in TOML, Argumenten oder Ausgabe. Z bestätigt vor Ausführung die dauerhaften Installationsroots und seine echte Releasebindung.

Docs: normale /home/nathanael/.config/deadlock-docs/bot.toml; fester Scope docs.public; benanntes Secret BRAIN_SERVE_DOCS_PUBLIC_TOKEN. Q-Vertrag: docs-client/docs, provider_egress=[public], ausdrücklich gepinntes öffentliches Docs-Release. Kein privater Operatorzugriff. Endpoint zum gemeinsam installierten lokalen Kern; aktuell vorgesehener Port 8788. Fachlicher Beweis: Casual-Lane erstellen und verwalten, Beleg public/discord-server/workflows/voice-lane-erstellen-verwalten.html.

Second-Brain: normale /home/nathanael/.config/second-brain/bot.toml; fester Scope second_brain.internal; benanntes Secret BRAIN_SERVE_SECOND_BRAIN_TOKEN. Q-Vertrag: second-brain/internal, provider_egress=[], genau ein interner Grant mit identischem Release wie internal_operator.release. Z hat /home/nathanael/.local/state/deadlock-brain/operator/brain.sock festgelegt, noch nicht installiert. Privates Verzeichnis 0700, Socket 0600 und Peer-UID-Bindung; vorhandenen Socket nicht löschen. Operator ruft keinen Modellanbieter auf. Nicht auf den alten internen TCP-Weg ausweichen.

Q-Referenz: bereiche/q/AN_HAUPT.md, Kopf 5c220a8f047eb980d953d9f9f285b34739b5ed88 mit noch uncommittierten Ergänzungen. Das ist ein veröffentlichter Vertrag, noch keine abschließend geprüfte oder ausgerollte Kernrevision.

## Anfragebeleg und Twitch-Probe

Q benennt das Journalereignis authenticated_request, Target brain_api::redacted_request, aus brain-api/src/audit.rs und brain-serve/src/audit.rs. Consumer aus authentifiziertem Grant, feste Route und Ergebnis; Request-ID im Journal als client-sha256:<SHA256 der vollständigen Client-ID>. K korreliert die genaue erzeugte oder vorgegebene Anfrage-ID über denselben Hash. Keine Texte, Header, Geheimnisse, Gesprächskennungen oder URL-Parameter im Kernjournal. Ein Ereignis mit insufficient_evidence oder Fehlerstatus ist noch kein fachlich erfolgreicher Antwortbeweis.

Twitch-Erweiterung: ENTWURF-TWITCH-CONFIG.md und BETRIEBSVERTRAG-TWITCH.md. Genau fünf erlaubte Modus-/Endpoint-/Enabled-Felder, atomare Änderung unter bestehender Dateisperre mit erwartetem altem Datei-SHA256, Rechte und Eigentümer erhalten. Dieser Dateihash ist nicht der bisherige semantische Dashboardfingerprint. Produktiver Aufruf ausschließlich über zulässigen privilegierten Verwaltungsweg nach unabhängiger Security-Abnahme. Vorhandener Releaseinstaller verteilt das Binary noch nicht; Z bindet Installation und engen Wrapperaufruf an denselben geprüften Stand. Kein globaler Chat-Cutover vor gemeinsamer Prüfung.

Der Nutzer hat gezielte echte Testfragen und Bot-Replies in earlysalty ausdrücklich freigegeben. TESTFREIGABE-TWITCH.md enthält Grenzen und Beweisziel. Sender und User-ID müssen über vorhandene Auth-/Helix-Wege bestätigt werden; die Kanalfreigabe schafft keine Sendercredentials. Kein Test bisher gesendet. Testumfang bleibt klein, unbeteiligte Chatter werden nicht moderiert.

## Noch fehlend

Gemeinsame Integration, Abnahme und Gate; installierte Consumerbinaries und normale TOMLs; eingerichtete benannte Infisical-Secrets und feste Grants; auflösbare Docs-/Second-Brain-Releases; Operatorsocket und Anfragejournal; Twitch-Konfigurationsinstallation und kontrollierte Aktivierung. Erst danach tatsächliche Antwort-, Restart-/Prozess- und Chatbeweise. Abschluss und Cleanup bleiben offen.
