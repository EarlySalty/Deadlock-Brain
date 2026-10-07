status: beauftragt
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/Deadlock-Twitch-Bot-brain-consumer-fertig

# Enger Entwurf und Bauauftrag: Twitch-Brain-Konfiguration

## Ziel und Entscheidung

VON_HAUPT.md 15:46 UTC beauftragt die minimale Erweiterung eines bestehenden Rust-Konfigurationswerkzeugs. Bestandssuche ist abgeschlossen. Nicht MCP, Browser oder Pool-Editor erneut ausprobieren. Keine neue HTTP-/Diagnose-API, kein generischer Dateischreiber, keine neue Unit, keine neuen Provider. GEMEINSAM.md, AUFTRAG.md, BRIEFING-K.md und UEBERNAHME-CODEX.md gelten. Kein globaler Chat-Cutover und keine Nachrichten. Z koordiniert gekoppelte Aktivierung nach gemeinsamer Abnahme.

## Eigentum und wiederverwendeter Bestand

Eigener Branch feat/brain-consumer-fertig-20261003, sauberer Startkopf 9a6356a679e05d0868f1178cf64a79c99b3e5105. Du besitzt nur rust/crates/tb-config/src/bin/tb-config-check.rs, rust/crates/tb-config/src/editor.rs und betroffene vorhandene Tests unter rust/crates/tb-config/tests/. Eine kurze zugehörige Rust-Betriebsdokumentation unter rust/docs/ ist erlaubt. Keine Frontenddateien, kein neuer API-Handler, keine Kerndateien, kein Release-Wrapper-Umbau in diesem Bauschritt. Manifest nur falls zwingend erforderlich, dann im Bericht nennen. Bestehenden tb-config-check und editor mit BotConfigSnapshot, typed Schema-Validierung, SHA-Revision, Sperre und atomarem Speichern wiederverwenden. Vor Codefragen Graphify, dann Quellen verifizieren. Keine neuen Code-Kommentare.

## Enger Kommandovertrag

Den bisherigen reinen Config-Check unverändert nutzbar lassen. Neue explizite Brain-Inspektion gibt nur den erwarteten Hash der gespeicherten TOML und nicht geheime Modus-/Endpoint-/Enabled-Felder der zwei Consumer aus. Keine gesamte TOML, keine Credentials, Identitäten, Profile oder Modell-/Providerfelder ausgeben.

Neue explizite Brain-Änderung erhält strukturierte, streng typisierte nicht geheime Eingabe und zwingend den erwarteten alten Hash. Ändern darf sie ausschließlich bot.brain_client.mode, bot.brain_client.endpoint, bot.brain_chat.enabled, dashboard.options.brain_client.mode und dashboard.options.brain_client.endpoint. Nicht angegebene Felder sowie Scopes, Timeouts, Zugangsdaten und alle anderen Einstellungen erhalten. Bestehende Modus-/Endpointvalidierung nutzen; nur vorhandene lokale Brain-Transporte, kein beliebiger entfernter Endpoint. Für die geplante Aktivierung ist http://127.0.0.1:8788 vorgesehen. Der produktive Schreibaufruf ist fest auf /var/lib/deadlock-twitch/config/bot.toml begrenzt, kein frei auswählbares Ziel oder neuer privilegierter Mechanismus. Bestehende reine Prüfung anderer Configpfade bleibt lesend.

Vor atomarem Austausch alten Hash unter derselben bestehenden Dateisperre prüfen. Eigentümer, Gruppe und Rechte der Config erhalten; auch ein privilegierter Lauf darf weder die Config noch die bestehende Editor-Sperrdatei für den Dienst unzugänglich machen. Unsichere Dateitypen/Symlinks, veränderte Hashes oder nicht verfügbare Rechte fail-closed melden. Keine Geheimnisinhalte in Fehlermeldungen. CLI selbst beschafft keine erhöhten Rechte; Installation und spätere Nutzung liegen beim zulässigen bestehenden privilegierten Verwaltungsweg nach gemeinsamer Abnahme.

## Prüfungen und Freigabepunkte

Nur synthetische private Configfixtures verwenden. Vor Tests rolle-test-waechter laden. Format, Clippy und bestehende betroffene tb-config-Suite ausführen, Rustup /home/nathanael/.cargo/bin, höchstens zwei Jobs. Beide Sperren zuerst host-checks.lock, dann /tmp/deadlock-cargo-release.lock, blockierend anfordern und für ganzen Prüfschritt halten. HOSTPROBE.md unmittelbar vor Compilerstart beachten, Exit 75 unter gehaltenen Locks 30 Sekunden später neu prüfen. Keine fremden Prozesse beenden und keinen wartenden eigenen Prüflauf nur wegen zehn Minuten Sperrwartezeit als Bauabschluss beenden. Keine parallelen Compiler. Bestehende zentrale Buildablage nutzen, keine Cachelöschung. Kein Releasebau vor Integration.

Kein eigener Bug-/Security-Review und kein Gate in diesem Implementierer. Unabhängige Intent-Abnahme und das einzige Security-Gate organisiert teil-k. CLI nicht produktiv aufrufen, auch keine echte Konfiguration zum Entwickeln lesen. Keine Secrets, Testkonten oder Chat-/Discord-Aufrufe. Eigene Commits erlaubt, kein main-Merge, Push nach main oder Deploy. Kein weiterer Agent oder T3-Thread.

## Routing und Rückgabe

teil-k, Paket K, Versuch 1; neuer Hauptorchestrator e6c19079-657e-4db9-80bd-8e1313e7f785. Melde exakt HEAD, geänderte Dateien, tatsächliche Prüfexits und Testzahlen, CLI-Vertrag, Installations-/Ausführungsanforderung und echte Rechteblocker. Rückgabe statt Berichtsdatei genügt. TODO.md und REGISTER.md nicht ändern. Deutsch mit echten Umlauten, humanizer und no-em-dashes.
