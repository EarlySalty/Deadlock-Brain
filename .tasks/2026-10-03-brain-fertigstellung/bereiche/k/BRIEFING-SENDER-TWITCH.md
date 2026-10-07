status: beauftragt
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/Deadlock-Twitch-Bot-brain-consumer-fertig, nur lesen

# Bestehenden Senderweg für die autorisierte Twitch-Abnahme bestimmen

## Ziel

TESTFREIGABE-TWITCH.md, aktuelle VON_HAUPT.md, SCOUT-twitch.md und GEMEINSAM.md gelten. Kanal earlysalty ist für wenige gezielte Testfragen und zugehörige Bot-Replies freigegeben. Noch keine gemeinsam geprüfte Installation und kein verifizierter vorhandener Sender. Die Kanalfreigabe schafft keine Credentials. Kein neuer T3-Thread oder Modellwechsel.

Nur lesend den bereits vorhandenen Rust-Auth-/Helix-/Chatweg bestimmen, der später den broadcaster_user_id von earlysalty, den echten Bot-Login und einen vorhandenen erlaubten Sender nachweisbar auflösen kann. Der Sender darf nicht der vom Bot als eigene Nachricht ausgeschlossene Bot selbst sein. Keine fremde Identität annehmen oder ein Konto neu anlegen.

## Grenzen

Graphify vor Codefragen, tatsächliche Quellen und unveränderliche Git-Objekte verifizieren. Keine Dateien ändern, Compiler oder Tests, Reviews, Gate, Deployment, Configänderung, Nachrichten, Moderation oder Kontomutation. Keine HTTP-/MCP-/Datenbankanfragen und keine Credentials abrufen. Geschützte Bot-TOML und fremde /proc-Daten nicht lesen. Die abgeschlossene Brain-Konfigurationssuche nicht wieder öffnen und keinen neuen Sender-/Diagnoseweg bauen. Kein weiterer Agent oder T3-Thread. Nur Auth-/Helix-/Chatquellen lesen, nicht den parallel bearbeiteten Testinitialisierer in dashboard_options.rs bewerten.

## Konkrete Rückgabe

1. Vorhandene genaue Module, Funktionen oder Verwaltungsaufrufe für Userauflösung, Botidentität und berechtigtes Chat-Senden mit Originalnachrichten-ID.
2. Welche vorhandenen Senderidentitäten und OAuth-Scopes die vorhandenen Wege unterstützen; Quelle und echte Laufzeitverfügbarkeit sauber trennen. Keine Existenz oder Rechte produktiver Sender aus Code erfinden.
3. Enger späterer Prüfschritt für IDs/Auth, ausschließlich über vorhandenen geschützten Authweg und ohne Tokenausgabe. Falls ein vorhandener Token oder erlaubter Sender nicht nachweisbar ist, den konkreten fehlenden Rest nennen. Keine Rechtebeschaffung oder generische neue API empfehlen.
4. Höchstens ein oder zwei harmlose fachliche Fragen mit sachlicher echter Bot-Erwähnung nach gemeinsamer Installation, niemals sofort senden. Kein führendes !, keine eigenen Botnachrichten oder Spam-/Moderationstests. Bestehende Eingangsfilter und globale Aktivierungsreichweite benennen.
5. Welche Belege anschließend Eingang, Brain-Request-ID, korrelierte redigierte Kernzeile, auditierte Antwort und Reply zur Originalnachricht verbinden. Keine Erfolgsbehauptung aus HTTP 200 oder insufficient_evidence.

Rückgabe knapp mit Quellen und ehrlichen Grenzen. teil-k pflegt die Akte; REGISTER.md und TODO.md nicht schreiben. Geerbtes Modell, high. Deutsch mit echten Umlauten, humanizer und no-em-dashes.
