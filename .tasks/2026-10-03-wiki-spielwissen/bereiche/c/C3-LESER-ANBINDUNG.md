status: aktiv
Datum: 2026-10-03

# C3: vorhandener interner Leserweg

Nativer lesender Worker a813e5d645f3f8e09, Sol high geerbt. Bestandsmeldung liegt vor, keine Quell-/Configänderung und kein Cargo-/DB-/Live-Lauf. Reine Anbindungsplanung, keine eigenständige Reviewrunde.

Der bestehende brain-maintain query prüft UID1000 und liest internal_doc_scopes frisch. Er filtert heute maintenance-docs und benutzt eigene Wortsuche. Bestehender kanonischer Retriever heißt ReleaseRetriever; release_lexical ist die Serve-Konfigurationswahl. Die minimale Folgeanbindung nutzt bestehenden LocalPgReader und ReleaseRetriever::retrieve, anschließend validate_evidence mit Providerweitergabe=false. Blockierenden bestehenden Reader über spawn_blocking ausführen; Releasepin, Frist und Budget aus vorhandener normaler Konfiguration. Ergebnisumschlag und Maintenance-Dokumentabfragen erhalten, kanonische Evidence samt Herkunft/Originalrevision/Hash/Quelle/Chunk/Faktmetadaten ausgeben. LocalPgReader prüft bestehende Registrierungs-/Widerrufs-/Rechtekontrollen.

Quellen ohne Veröffentlichungsfreigabe ergänzen source.review:<source_id> als Pflichtscope. Document-ACL verlangt sämtliche Scopes. Der normale bestehende Betreiberprincipal benötigt neben internal_docs daher internal_knowledge und die quellenbezogenen source.review-Scopes je tatsächlicher Importquelle. Die öffentliche Twitch-Berechtigung bot.public bleibt unverändert. Keine neue Auth-Plattform, keine Token-Datei, keine Secretwerte. Root-Punkt42 deckt interne Lesen-/Rohhaltefreigabe bereits ab; ImportGrant hat Veröffentlichung und Providerweitergabe=false sowie echte Freigabe-/Herkunftsreferenzen.

Nach tatsächlichem Ende der gemeinsamen eingefrorenen Fix2-Prüffolge mögliche exklusive Baupfade: brain-maintenance/src/integration/runner.rs und runner_tests.rs. Normale /etc/deadlock-brain/maintenance.json ausschließlich durch C3 beim autorisierten gemeinsamen Lauf ändern, erwartete Hashbindung/Bestandserhalt/Configbackup beachten. Worker bekommt keinen ungeprüften Produktions-Schreibauftrag. Kein CLI-/Cargo-/Authschemaumbau nötig.

Release-Budgetprüfung bleibt exklusiv beim CLI-Folgepaket: vorhandener ChunkIndex, tatsächliche releasegepinnten base.revisions plus ausgewählte neue aktive Köpfe. Historische22742A-Revisionen sind nicht22742aktiveSeiten; A-Hauptarchiv hat4417aktiveSeitenkandidaten, endgültiger gemeinsamer Kopfstand folgt aus korrekter Store-Revisionsbehandlung. Keine laufenden A/B/D-Aufträge duplizieren.

Stopbedingungen: ungültige lokale Betreiberidentität, fehlende Pins, unzulässige Quellenherkunft, widerrufene Rechte oder echter Budgetüberschritt. Kein direkter SQL-Ersatzreader oder behaupteter Livebeweis. Folgebau erst nach exklusiver Quellenfreigabe.
