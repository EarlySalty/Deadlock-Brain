status: beauftragt
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: nur vorhandene eigene Consumerstände lesend

# On-demand-CLI-Betriebsvertrag vorbereiten

## Ziel und Quellen

Die lokal geprüften CLIs sollen nach gemeinsamer Integration dauerhaft installierbar und aufrufbar sein. Lies UEBERGABE.md, INTENT-DOCS.md, INTENT-SECOND.md, TESTFREIGABE-TWITCH.md und aktuelle bereiche/q/AN_HAUPT.md. GEMEINSAM.md, AUFTRAG.md, UEBERNAHME-CODEX.md und PAKETE.md gelten. Z verantwortet gemeinsame Abnahme, Gate, Installation und Produktivwechsel. Keine eigene Umschaltung.

## Eigentum

Nur lesende Untersuchung der vorhandenen Rust-CLI-Manifeste, Kommandos und Credential-/Releasewege. Eigene Worktrees: Deadlock-Docs-brain-consumer-fertig und Deadlock-2nd-Brain-brain-consumer-fertig unter /home/nathanael/.worktrees/. Keinen Quellstand ändern, keine Compiler, Tests, Git-Mutationen, Geheimnisabrufe, neuen Daemons oder echten Requests. Graphify vor Codefragen. Kein weiterer Agent oder Thread. Keine Bug-/Security-Review.

## Konkreter Vertrag

Liefere ausführbare administrative Kommandos für spätere Installation, gebunden an den dann tatsächlich integrierten Repo-SHA und Binaryhash. Vorhandene Cargo-/Releasemechanik und zentrale Buildablage verwenden; Produktionsbinary nicht aus einem ungeprüften Cache oder anhand des Datei-Alters auswählen. Releasebau erst aus dem gemeinsam integrierten eigenen Worktree mit frischem origin/main, Rustup und beiden Hostlocks. Standard-Cargo-Installation ist einem neuen Installer vorzuziehen, sofern sie den vorhandenen Betriebsvertrag erfüllt. Benenne vorhandene Zielpfade oder kennzeichne eine noch benötigte enge Installationsfestlegung ausdrücklich. Kein generischer privilegierter Dateischreiber.

Vorhandener sicherer Bootstrapweg: user brain-serve.service nutzt LoadCredential=infisical-token:%h/.config/infisical-tokens/infisical-token-bots und öffnet FD5. Nur Metadaten dazu lesen, nicht den Credentialinhalt. Daraus den vorhandenen on-demand Starter ohne neue persistente Credentialdatei oder ENV-Datei ableiten. Normale TOMLs für Docs und Second-Brain müssen deren genaues dediziertes Schema erfüllen; keinen unbekannten Socketpfad einsetzen. Z liefert reale Socket- und Releasebindungen, Q feste Grants. Neue Systemdienste ausgeschlossen.

Zeige je CLI den tatsächlichen Request-/Outputvertrag, über den K später die exakte Anfrage-ID zu Qs client-sha256-Journalhash korrelieren kann. Wenn automatische query-IDs nicht außen sichtbar sind, vorhandenen answer/prepare-Pfad mit expliziter synthetischer Abnahme-ID bevorzugen. Harmloser Docs-Beleg: Casual-Lane erstellen und verwalten. Second-Brain fragt eigenes Betriebsgedächtnis ohne Modellanbieter. Ein erfolgreicher Ablauf darf nicht mit Exit 0 oder insufficient_evidence allein behauptet werden.

## Ausgabe

Knapper Betriebsvertrag mit exakten bestehenden Binarynamen, Post-Integration-Build-/Installationskommandos, sicheren Starterkommandos, nicht geheimen TOML-Feldern, Rechte-/Pfadbindung, Anfrage-ID-Korrelation, benötigten Z-Eingaben und noch offenen Grenzen. Keine tatsächliche Installation oder Anfrage. Rückgabe genügt, teil-k pflegt die Akte.

## Routing

teil-k, Paket K, Versuch 1, Haupt e6c19079-657e-4db9-80bd-8e1313e7f785. Geerbtes Sol-Modell, high. TODO.md und REGISTER.md nicht schreiben. Deutsch, echte Umlaute, humanizer und no-em-dashes.
