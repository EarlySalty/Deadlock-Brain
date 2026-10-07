# A-F1b: Steckbrief-Stufe 1 bis zum Live-Beweis abschließen

Native Blattrolle Implementierer für den bestehenden Profilpfad. Auftraggeber Paket A `2c7de4c9-bac4-43ad-b91a-f8ac889f09b4`. Keine weiteren Agenten/Threads/Sessionnachrichten, keine eigenen Reviewer. Kein Stage-2- oder Reasonerbau.

## Verifizierter Stand

A-F1 hat den GameTracking-Teilklonfehler belegt, keinen zweiten Fix gebaut. Main enthält 37cfc6c6. A prüfte jetzt live Main 10ebbb208aaaac9ddf6e07bc89a6af4a620f86ef: Serve PID 2554801, Manifest-/Binaryhash passend; beide Releasezeiger derselbe SHA. Neuer Core-/Profilschemacheck mit installiertem brain-migrate erfolgreich, drei gespeicherte Header und nötige Grants vorhanden. Diesen fremden bereits erfolgten Deploy nicht als eigene Leistung melden.

A startete normalen Maintenance-Tick 00:26:39, um 00:28:58 TERM und abgebrochener systemctl-Auftrag, kein Tickabschluss. Ursache des TERM noch unbelegt, Timeout zwei Stunden, OOMScoreAdjust 200. Unprivilegiertes earlyoom-Journal war nicht zugänglich genug für ein Urteil. Kein Beleg für einen Gitfehler dieses neuen Releases. Live 378 Profilentitäten/64342 Profilfakten (13295 GameTracking, 51047 Deadlock-Data), null Ableitungsquittungen. Neueste Docs-Reviewzustände JEV_RESPONSE_INVALID, einmal STAGE_FAILED. Das ist ein Beobachtungsbefund, noch kein bewiesener Profilblocker. Details A/BETRIEB.md, A/F1-RUECKGABE.md und A/STAND.md.

## Ziel

Vorhandene Stage-1-Kette tatsächlich zu Ende bringen: beide Originalimporte, verifizierte Fakten/Quittungen, Steckbriefdokumente samt Patch-Story, normale aktive Releasebindung und drei normale Brain-Consumerantworten intern. Bestehende Entrypoints nutzen, keine zweite Pipeline, keine gefälschten Quittungen oder manuellen DB-Zustandskorrekturen. Weiterhin nur abgeleitete freigegebene Fakten nach außen; Originaldateien, wörtliches Wiki, interne Pfade und Communityrohtexte bleiben intern. Keine öffentlichen Testnachrichten. Keine Modelle/Zeiteinstellungen eigenmächtig ändern oder Gates senken.

## Eigentum und Wirkung

Vorhandenen eigenen Worktree `/home/nathanael/.worktrees/brain-a-profile-20261006`, Branch fix/brain-a-profile-20261006, sauberer damaliger HEAD d6131cc5 zuerst prüfen und fortsetzen, keine Neukopie. Aktuelles origin/main holen und normal integrieren, fremde Kanonänderungen/andere A-Worktrees tabu. Eigentum nur eng nötige Stage-1-Profile-/Materialisierungs-/Ableitungs-/Wartungspfade in brain-maintenance und zugehörige vorhandene Profil-Storage-/Sourcebausteine. Schema.rs, brain-migrate, Grants, neue Site-Migrationen gehören F3b und sind tabu. Brain-api/kernel/providers/discord_live und Inviteadapter gehören E3, tabu. deadlock-brain/src/main.rs und dbrain-enrich gehören laufender F4-Integration, tabu. Kein Wikiintervall-/Stage-2-Ausbau.

Normale bestehende Profil-/Wartungsoperationen, registrierte Jobs, status und zugelassene requestgebundene interne Funktionsproben sind freigegeben. Credential-Lader und bestehende Konfigwriter/CAS/Sperren verwenden, nicht frei Runtime-Dateien oder DB-Status ändern. Bestehende Jobauswahl darf die beauftragten Profile priorisieren, sie ist kein Gatebypass. Bei TERM Ursache empirisch lesen statt als Quellfehler auszugeben oder den Dienst in einer Schleife zu starten. Keine fremden Dienste/PIDs beenden. Wenn Docs-JEV tatsächlich diesen Pfad sperrt, tatsächliche vorhandene Antwort/Vertragsabweichung prüfen und den berechtigten Fehler im bestehenden Pfad korrigieren, keinen neuen Provider, Modellwechsel oder scheinbare Zustimmung erfinden. Bei benötigtem Dateieigentum außerhalb der Grenzen konkret an A melden und unabhängige bestehende Schritte fertigstellen.

## Code und Abschluss

Nur Rust ohne neue Code-Kommentare, Graphify zuerst, vorhandene Implementierung weiterverwenden. Höchstens ein Cargo-Job angesichts Hostlast. Bestehende passende Compiler-/fmt-/Clippy-/Testläufe vollständig nachhalten. Keine eigenen Kurzlimits und kalten Cache-Neukopien. Neue private Belege /tmp/brain-a-profile-final-proof-20261007/, vorhandene .core-test-logs erhalten. Neue Änderungen nur Featurecommit/-push, Selbstgate über vorhandenen Helfer, BLOCK für frischen Fixer zurückgeben. Kein Main-Push oder Binarydeploy durch den Blatt-Worker. Root-Akten schreibt nur A.

Rückgabe mit exaktem Source-/Runtime-SHA, Ursache, eigener Änderung und Grenzen, tatsächlichen Prüfzahlen, Gatewortlaut/Modell/Exit, Quittungs-/Dokument-/Aktivierungs-/Antwortbeweisen. Gebaut, geprüft und live sauber trennen. Wache 20 Minuten, spätestens 30, kein Abbruchbudget.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-a-profile-20261006
