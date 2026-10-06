# A: enger frischer Familienfix5

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-a

## Auftrag und Vertragsziel

Frischer nativer Rust-Fixer, ausschließlich geerbtes GPT6.1Sol höchstens high. Genau die zwei belegten Speicherfamilienfunde aus REVIEW-LOCAL-5.md korrigieren, keine neue Implementation oder paralleler Writer. Der vorhandene Fix4-Eigenanteil wird unverändert bewahrt soweit die nötigen engen Korrekturen es erlauben; kein Neubau. Vor Code zentrale CONTRACT.md/AN_BEREICHE.md, eigene FIX-4.md und REVIEW-LOCAL-5.md lesen. Nutzer verlangt vollständige Speicher-/Retry-Konsistenz und Erhaltung originaler Texte/Autoren/Rechte.

Befund öffentlich: OriginaltextA für ID1/Revision101, KonflikttextB erst ohne Autoren gespeichert, danach derselbe B mit tatsächlichem XML-Autor/Contributor/Capture/Rechten. conflict.exists() synchronisiert nur und meldet „Konflikt erhalten“, zusätzliche Herkunft verloren. S297-330 im Fix4-Freeze. Der vorhandene originale Herkunftspfad S385-432 darf sinnvoll eng wiederverwendet werden, aber konfliktbezogene Herkunft eindeutig an Konfliktinhalt binden, nicht Originalautoren vermischen. Original und erster Konflikttext unveränderlich; zusätzliche Belege/Erstbeobachtung erhalten; widersprüchliche Aussagen zeigen, keine automatischen Rechteupgrades. Öffentlicher API-/XML-Reproduktionsfall und idempotente Wiederholung nötig. Bestehende Schnittstelle/JSONL-Vertrag nicht ändern, keine neue Konflikt-Auflösungsplattform.

Befund Same-Spool: nach sichtbarem Installationsziel und fehlgeschlagenem letztem Sync bleibt stored_bytes wegen ? vor Zählerfortschritt veraltet. Retry synchronisiert/dedupliziert ohne Zählerreparatur. Ein weiterer neuer Datensatz kann reale gemeinsame Gesamtgrenze überschreiten; Herkunftsersetzung kann vom falschen alten Zähler abziehen. S323-324,351-352,430-431. Wichtig: öffentliche API/XML-Retries öffnen neu und zählen korrekt; kein öffentlicher Oversize-Lauf behauptet. Trotzdem echten gemeinsamen In-Memory-Zustand nach Fehler/Retry konsistent halten. Keine Grenze erhöhen; checked arithmetic, reale Datei-/Konflikt-/Herkunftsgrößen erhalten. Nach-Installations-Syncfehler plus Same-Spool-Retry plus weiterer Datensatz an exaktem Budget regressieren.

Beide Korrekturen über alle tatsächlich betroffenen gemeinsamen Speicherpfade prüfen: Dokument/Herkunft/Konflikt/Quellenbindung/Checkpoint/Veröffentlichung. Kein Fehler nach sichtbarer Installation darf später ungeprüfte Erfolgs-/Erhaltenbestätigung erzeugen. Bestehende37Regressionen nicht löschen/abschächen/überspringen. Echte Dateien, Fehlerinjektion nur konkrete Fehlerstelle, kein kompletter Fake-Spool. Kein Crashbeweis behaupten.

## Eigentum und Schreibfreigabe

Worktree /home/nathanael/.worktrees/brain-wiki-spielwissen-a; Branch feat/brain-wiki-spielwissen-a; HEAD2734c2da4e814ff79953e8e825275b0216a6af16, neue eigene Dateien uncommittiert.
Nur storage.rs/tests.rs unter rust/crates/dbrain-sources/src/wiki_inventory/ sowie eigener Bereichebericht FIX-5.md gehören dir. Wenn Berichtsdateischreiben nach deiner übergeordneten Rolle unzulässig ist, nativ vollständig zurückgeben, nicht auf einen externen Dokumentdienst ausweichen. A sichert dann lokal.
Keine Änderungen an wiki_inventory.rs/normalize.rs, Core, produktiven Manifesten/Lockfile/lib.rs, Harness, Originaldaten, Register, Status oder TODO. Keine Secrets/ENV, Netzwerk, Community, Git, Deploy oder weiteren Agenten. Native Dateisystemzuständigkeit streng beachten. Bestandssuche zuerst code-suche/Graphify, globaler Graph vorhanden, kein Neubau.

Sicherer eigener Punkt ausdrücklich bestätigt07:49:43UTC: test-6-Wrapper b66lshqbc/PID3660876 war vor erstem Lock/Cargo; nur wegen konkretem notwendigen öffentlichen Herkunftsverlust geordnet beendet. PID verschwunden, eigene FD8/9 geschlossen, keine eigenen Compilerkinder, eigener Startwächter b8reh2cuz regulär Exit0/OWN_WRAPPER_ENDED_WITHOUT_CARGO. Keine fremden Eingriffe oder Timerpause. Datenworker hält an bis neuem Freeze. Damit Schreibfreigabe jetzt, kein anderer Writer.

## Eingangs-Freeze

Vor Änderung alle vier tatsächlichen Hashs messen:
- wiki_inventory.rs2507c127aaf8e366f7bb7cf22564074adbaa546b344328d6fdbd593e5b38ef80
- normalize.rs27e63412a7a57c956dc82e602df4f93683adb3eb06866b22381157f85b96dcdc
- storage.rsf48f4819534870b1aa8b2c1f0042f82d1d1ca1b1056242e55dc6632271d4daea
- tests.rsa0d7cb81ce90c7a729c4ac6e92fbe02d3b670ff5e473a307935589adaf26c6d1

37 geschrieben, Formatcheck0, keine ausgeführte Regression. Einziger echter test-4-Cargo-Start06:47:12UTC, Exit101 nur eigener Harness-Pfadauflösung. Derselbe Datenworker hat diese bereits korrigiert, behält eigenen Harness/Core/Lockfile/Originale. Noch keine normalisierten Vertragsdokumente/Fakten. Du startest KEINE Compiler/Cargo/Test/Clippy/Hostlocks. Gezielter rustfmt nur auf zwei erlaubten Dateien, keine Child-/globale Formatierung.

## Abschluss und Routing

Bericht mit tatsächlichen engen Änderungen/Pfadabdeckung, Regressionen geschrieben versus ausgeführt, Formatcheck/Exit, vier finalen SHA256. Zwei gesperrte Module müssen bytegleich bleiben. Anschließend ausdrücklich einfrieren, keine weiteren Moduländerungen. Keine allgemeine Freigabe oder Laufbeweise erfinden. Echte Compiler-/Datenprüfung danach beim selben erhaltenen Datenworker, unabhängige Nachprüfung frischer Kontext.

Auftraggeber A f01cce67-209b-468e-8abb-ec2070beeaa2, Haupt/root, Paket a/Versuch1, alleiniger Produzent teil-a. Rohbericht nur A, keine fremden Sessions. C2 alleiniger finaler Gate/Integrator/Deployer; bestehendes get_bounded dort integrieren, kein zweiter Corepfad. Fremder Altbranch aus Stop-Hook nach Punkt10 unberührt.
