# K: Bestehende Ortskontextnaht tatsächlich bestimmen

## 1. Ziel und Vertrag

Verbindliche NACHTRAG-K-ORTSKONTEXT.md und ENTSCHEIDUNG-PARALLEL-FERTIGSTELLEN.md unter /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-fertigstellung-astra/ lesen. Discord: sichtbarer Kanal, Kategorie, Thema oder Server-as-Code-Zweck, Thread/DM und Eingangsart. Twitch: Kanal und bekannter Partnerstatus. Kein Selbstverweis in denselben Bereich. Nur Daten, keine Anweisungen oder Rechteerweiterung; fehlenden Ort nicht erfinden. Kein zweiter Antwortweg, kein Provider-, Modell- oder Timeoutwechsel. Kein Warten auf I/G-main. G besitzt Verträge, Kernel und Rechnung. Gesicherter ANTWORTPORT-VERTRAG.md im G-Worktree ist Referenz, keine Übernahme aktiven WIPs oder Ersatzadapter.

Primary hat Graphify global zu AuthorizedContext abgefragt und aktuelle bestehende Quellen nachgelesen: brain-contracts/src/lib.rs Query enthält request_id, conversation_id, text, domain, requested_scopes, profile, patch, mode, aber kein freies Ortsmetadatenfeld. AuthorizedContext.discord enthält nur DiscordRequestContext mit user_id, request_id, scope. Das ist ein Befund, kein Auftrag zur eigenmächtigen G-Vertragserweiterung. dl-brain/brain_api.rs baut Query im vorhandenen Adapter und bindet answer_for_discord an die lokale user_id. tb-knowledge/brain.rs besitzt answer_with_context, nimmt aber query_text entgegen. Tatsächlichen gesamten Weg und vorhandene Metadaten-/Kontextbausteine prüfen, nicht ungeprüft Kontext in den Fragetext stopfen oder ein Feld missbrauchen.

## 2. Eigentum

Native enger Worker, keine weitere Delegation oder T3-Threads. Diese Runde ist read-only auf Produktcode. Ausschließlicher Schreibbereich ist K/ORTSKONTEXT-BESTAND.md im Brain-K-Worktree. Keine Source-, Manifest-, Register-, TODO-, fremden Docs- oder Konfigänderungen. Keine Reviews außerhalb des regulären Gates. Graphify zuerst, danach gefundene Stellen nachlesen. Die tatsächliche ctx_execute_file-Projektrootgrenze nicht umgehen oder Settings ändern; konkrete Ablehnung präzise melden. Nicht dieselbe bereits verweigerte Quelle über ein anderes Werkzeug lesen. Keine alten Prüfrunner oder Cargoaufrufe nötig für diese Bestandsrunde.

## 3. Arbeitsstand

Brain /home/nathanael/.worktrees/brain-k-ki-20261007, feat/brain-k-ki-20261007, HEAD 38280ca8. Drei uncommittierte Budgetdateien provider_input.rs, brain-providers/lib.rs, answer_provider_tests.rs sowie eigene Dokumentänderungen bleiben unangetastet. Retrievalfixture wurde von einem früheren Worker nicht geändert, weil sein ctx_execute_file-Zugriff verweigert wurde. Dieselbe Datei nicht wieder über einen anderen Weg lesen.

Bots /home/nathanael/.worktrees/bots-k-guide-20261007, fix/brain-discord-conversation-20261007, HEAD 0fb873c6, auf main gepusht. Sourcecommit 0758b1f2 ist regulär ALLOW, 81 scoped Tests und Compiler/Format/Clippy grün. Eigener dl-bot-Releasebau mit cargo-slot +1.97.1 läuft auf diesem unveränderten Stand; daher dort KEINE Änderungen. Twitch /home/nathanael/.worktrees/twitch-k-ki-20261007, detached 2ead4d55, bereits main und deployed. Keine Git-Schreibschritte, kein Push, Merge, Release, Restart oder Liveprobe durch Worker. K-Primary integriert später separat.

## 4. Beweisziel und Stopgrenze

Kurzer belastbarer Bestandsbericht mit Datei:Zeile: vorhandene Ortsmetadaten und Rechteprüfung je Discord-Eingang; tatsächliche Request-/Providernaht; vorhandener Twitch-Partnerstatus; vorhandene lokale Bereinigung; wiederverwendbarer Anschluss mit minimalem eigenem Folgeschritt. Discord-/Steam-IDs, Mitgliederlisten und fremde Personendaten NEVER ans Modell senden. DMs erhalten keine fremden Serverdaten, private Threads nicht allein aus VIEW_CHANNEL freigeben. Keine harte Kategoriesperre. Tatsächlich fehlenden Vertrag ausdrücklich benennen, keinen Scheinanschluss oder unbewiesenen Datenschutz versprechen. Falls ein G-Vertrag fehlt, konkreten kleinsten benötigten Vertrag nennen, nicht fremde Quelle ändern. Test-/Livebeweis noch nicht behaupten.

NEVER read, print or write plaintext secrets. MUST NOT send private user/community data to remote coding models. Keine echten privaten Fragen, Rohlogs oder Identitäten lesen. Der Discord-MCP-Endpunkt lieferte HTTP 401. Keine Authentifizierungsumgehung, Secretsuche oder neuen Kanallesewerkzeuge. Keine Sessionkontakte und keine Arbeit am fremden Docs-Thread 59740e62. Q wird nicht gestartet.

## 5. Routing und Übergabe

Laufende CLI-Session 988eeaea-28ee-424c-b362-e250610cde91, teil-k, Versuch 1, Delegator 481426fe. Bestehender K-T3-Thread 79c97ab5, kein Ersatzthread. Worker produziert nur diesen Bestandsbericht. Rohbericht an K-Primary; keine Frage an den Nutzer. Nächsten konkreten sicheren Umsetzungsanschluss nennen oder den technischen Blocker mit Ort und fehlendem Vertrag. Keine Zwischenberichte nach jedem Suchschritt.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 30 min | Worktree: /home/nathanael/.worktrees/brain-k-ki-20261007
