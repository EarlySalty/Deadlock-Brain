# K: Vorrangige zentrale Discord-/Twitch-Antwortstrecke

## Ziel und aktueller Vertrag

Derselbe K-Auftrag wurde autorisiert übernommen. Direkter Delegator 481426fe-b477-42b3-91c6-901811fcba1d, Haupt-Orchestrator d3a1741e-82bc-4a48-865b-2845c663dca7. Lies die neuen verbindlichen AUFTRAG.md und PAKETE.md unter /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-fertigstellung-astra/ sowie A/EIN-BRAIN.md unter .tasks/2026-10-06-brain-abschluss/. Priorität sind echte zentrale Antworten über vorhandene Consumer in Discord-Erwähnung/DM und Twitch: Pocket/Haze, Coaching, Pate als Brain selbst, Selbstbild, eigener Invite-Status und ehrlicher Ausfall. Keine neue Botantwortengine, kein neuer Provider oder paralleler Antwortpfad. P0 gehört einem separaten Paket. H ist bestätigter bestehender Input, kein Teil deiner Änderung.

## Eigentum und Bestand

Eigene bewahrte Worktrees: /home/nathanael/.worktrees/bots-k-guide-20261007 (feat/bots-k-guide-20261007, HEAD 8745a0eb), /home/nathanael/.worktrees/twitch-k-ki-20261007 (feat/twitch-k-ki-20261007, Anfang0452e03c, bisher ohne Produktdiff). Brain-K HEAD2ed1a6b7 ist read-only für dich. Kanonische Checkouts unverändert. Kein weiterer Thread oder Agent, kein Sessionkontakt. Haupt-K ist allein Integrator/Deployer, kein Git-Schreiben durch dich.

Schreibbereich ausschließlich vorhandene zentrale Consumer-/Zustelladapter: Bots dl-brain/src/lib.rs und brain_api.rs sowie notwendige dl-bot/src/modglue.rs/main.rs-Anbindung, disjunkte neue Consumeradapter bei fachlicher Notwendigkeit. Twitch tb-knowledge/src/brain.rs und tb-bot/src/brain_chat_wiring.rs samt unmittelbar notwendigen Consumeraufrufern. Bestehende Vertragsabhängigkeits-Pins/Manifeste nur gegen ausdrücklich bestätigten geprüften Featurevertrag, kein frei gewählter G-WIP-Pin. Nicht schreiben: brain-contracts/provider/kernel/serve, dbrain-reasoner, G/E/F/I-Bereiche und dl-community/src/concierge.rs, dessen unabhängiger Conciergefix wird nicht übernommen. Kein menschliches Patenprogramm ändern, keine globale Formatierung. FAQ/passive Hilfe/Tickets beim tatsächlichen Bestand als angeschlossen oder konkrete Ersatz-/Abschaltliste dokumentieren, nichts löschen.

Graphify zuerst, vorhandene Inventur danach an echten Quellen prüfen. Bestehendes /v1/answer, AsyncBrainClient und authorgebundene Antwort wiederverwenden. Keine Modell-/Timeoutänderung, kein universelles Steuerprotokoll in Query.text, keine privaten Inputs still streichen und Parität behaupten. Modelle liefern keine selbst autorisierten URLs; allgemeine Linkfilter erhalten. Coachingplatzhalter aus vorhandener zentraler Antwort wird vorhandener Plattformprojektion zugeführt, keine neue Antwortphrase im Bot. Pate = Brain = Concierge.

## Datenschutz und tatsächliche Abhängigkeiten

NEVER read, print or write plaintext secrets.
MUST NOT send private user/community data to remote models.
Auch rohe Nutzer-/Communityfragen gehen nicht als Nebenprodukt an externe Modelle. Loopbackproxy ist keine lokale Verarbeitung. Keine private FAQ/DM als sicher freigegeben behaupten. Keine Produktions-/Communitydaten in deinem Modellkontext lesen; keine Livefragen aus History für diese Quellprüfung laden. Nur Code, Dokumentation und vorhandene neutrale Prüfverträge.

Haupt-K hat konkrete Lücke am K-Basisstand gelesen: dl-brain/src/brain_api.rs:95-105 sendet mit new_local nur an lokalen Dienst; dies beweist keinen lokalen Provider. brain-serve/src/config.rs:80-98 und400-436 unterscheiden OpenaiCompatible/CodexSubscription, nicht vertrauenswürdig tatsächliche lokale Inferenz. brain-contracts/src/provider_input.rs:31 verarbeitet query.text unverändert. Diese aktiven G-Dateien bleiben unverändert. Private direkte Botsanfragen werden derzeit in modglue.rs vor Consumer gesperrt. Guard niemals entfernen, bis tatsächliche zulässige Verarbeitung belegt ist. Keine lokal klingende Flagfreigabe ohne Durchsetzung, keine eigens erfundene Handshake-Route. Gegen vorhandene Rechte-/Providerkonfiguration prüfen, ob der Anschluss tatsächlich möglich ist; bei echter Codeabhängigkeit exakte Datei/Typ/Signatur nennen. G ist vom Delegator um geprüften Featurevertrag gebeten; keine Wartepflicht auf G-Builds oder Main, kein WIP kopieren.

A/EIN-BRAIN.md erlaubt genau eigenen Invite-Status als Enum plus Zeitpunkt. Identität intern, keine IDs/Namen/Rohfragen/Auditrohzeilen an Remote. Der existente Skill ist kein Grund, beliebige Nutzerdaten freizugeben. Twitch hat keine angenommene Discord-/Steam-Identität. Reale Quelle und Empfangsbindung prüfen.

## Arbeits- und Beweisziel

Derselbe vorhandene Anfrageweg liefert entweder zulässige zentrale Antwort oder vorhandenen kurzen ehrlichen Ausfall. Keine Doppelantwort, kein Frag-Nani-Fallback für Spielfragen. Bestehende Rate-Limits, Zustimmung, dauerhafte Ablehnung, Zustellrechte und plattformgerechte Coachingzielprojektion erhalten. Implementiere den real möglichen Anschluss ohne Datenschutz-/Modell-/Timeoutabweichung. Fehlende lokale Providerfähigkeit nicht erfinden. Kein früherer Featurebeweis gilt automatisch als Livebeweis.

Nach Änderung passende Compiler/Format/Clippy und bestehende Tests mit echten Anzahlen/Exits ausführen, tatsächlichen freien Cargo-Buildslot halten /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/build-slot-{1,2,3}.lock, `--jobs 3`, moderne absolute Cargo-CLI, bestehender sccache/Cache, keine Targetkopie. Bots dependency-Feature `--features dl-central-db/testing`, nicht vermeintliches dl-bot/testing. Bot-only-Clippy hatte nach sauberer Vorhermessung57 async_trait-Macrolints; neue Befunde getrennt prüfen, keine Unterdrückung. Private-/Timeout-/Ausfallpfade ohne echte Remoteaufrufe lokal prüfen, kein Produktionsschreiben. Keine echte Kanalprobe, kein Deploy oder Neustart durch dich; Haupt-K fährt nach gemeinsamem Gate das reguläre vollständige Release.

## Routing und Übergabe

Native Worker innerhalb teil-k, Versuch1, Auftraggeber Haupt-K Session988eeaea-28ee-424c-b362-e250610cde91. Höchstens drei aktive native high-Agenten; die beiden anderen schreiben nur Brain-K-Artefaktmodule und SVG-Renderer. Kein Pfadkonflikt. Status nach25min, spätestens30. Bericht K/ZENTRALANTWORT-BAU.md im Brain-K oder abschließende Rückgabe mit genauen Belegen, keine zentrale TODO/Registeränderung. Runde nicht in Sessionchat melden. Gebaut, compilergeprüft, gategeprüft, gemergt und live getrennt. Haupt-K übernimmt Commit, regulären Gate, frische Fixschleife bei BLOCK und Featurepush. Echte unlösbare Vertragsabhängigkeit präzise berichten, vorhandene funktionierende Quellen erhalten.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/bots-k-guide-20261007
