status: aktiv
Datum: 2026-09-29

# R-AC: unabhängige Abnahme des gemeinsamen Brain-Pfads

Startfreigabe für A auf 799c68b (PR #59), Basis 305df2d36ec7b5d0513d6c0769051b41538d6a1b. Eigener Reviewbaum /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929, Branch review/pre-g5-core-abnahme-20260929, clean bei 799c68b. C wird noch selbst geprüft; dessen feste Commit-ID folgt durch Orchestrator. Zunächst A unabhängig prüfen und konkrete Mängel zeitnah melden. Keine endgültige gemeinsame Freigabe vor C. Wenn A vor C fertig ist, Zwischenbericht committen/pushen und abgeben; nicht selbst pollen oder andere Threads starten. Derselbe Reviewer erhält C als Fortsetzung.

A-REPORT.md im eigenen Baum ist Autorenevidenz. Letzter Codecommit 8488891, spätere Commits nur Bericht. Besonderheiten: privater CLI-Revoke jetzt atomar über brain-storage (pg_jobs.rs/pg_release.rs). Zwei Lease-Retry-Hinweise aus Selbstprüfung offengelegt. Die Analytics-Route /v1/analytics/observation ist optional und liefert patch_membership unverified: prüfen, ob die vom Nutzer geforderte echte Meta-/Population-Laufzeitintegration damit tatsächlich erreicht wird oder nur ein isolierter neuer Endpunkt entstanden ist. Kein unbelegtes META_RUNTIME_PATH_READY oder POPULATION_RUNTIME_PATH_READY vergeben.

Astra, frischer Reviewer ohne Autorenschaft. Einziger Thread für dieses Paket, keine Unterthreads oder Unteragenten. Intent 562a877b-0939-440a-964d-1145d9e9431a. AUFTRAG.md ist der verbindliche Nutzer-Intent. Graphify zuerst, vorhandenen globalen Graphen benutzen. Keine Produktänderungen und keine Code-Kommentare. Nur eigener Berichtbranch darf committed/gepusht werden. Keine Merges oder Produktion, keine realen Nachrichten/Builds, keine produktiven DB-Zugangsdaten, keine Replaydownloads oder Wiki-Captures. Keine Änderungen an Policies, Budgets oder Tests zur Erzeugung eines grünen Ergebnisses.

## Fokus A

Ein- und Ausgabevertrag zusammen prüfen: Match-Antwort aus echter API-Form, account-bound SourceRecordV2, normaler Store, Release, brain-serve. Nicht nur Funktionen isoliert ansehen. Fehlender/duplizierter Match, falscher Account, unangefragte weitere Spieler, öffentliche Sichtbarkeit, Schema-/Raw-Hash-Fehler müssen vor Veröffentlichung geschlossen scheitern. Demo-Belege nicht stillschweigend einem fremden Account zuordnen. Der CLI-Pfad muss tatsächlich Commit/Release/Readback nutzen, nicht nur ein unbenutzter Bibliotheksadapter sein.

Idempotenz und Recovery einschließlich Transaktionen prüfen: mehrere Dokumente eines Matchs, Tombstone, Revoke, Release, Checkpoint und Lease. Teilfehler dürfen keine halb widerrufenen aktiven Belege hinterlassen. Jede neue brain-storage-Änderung ist mit den alten Upgrade-/Storepfaden gemeinsam zu bewerten.

Meta/Population müssen einen echten bounded Laufzeitpfad in brain-serve haben, kanonische HTTPS-Origin statt Phantomdienst. Byte-, Row-, Zeitfenster-, Timeout-, Retry- und Gesamtdeadlinegrenzen, Content-Type/HTTP-Status/Redirects/Proxies prüfen. Das reale hero-stats-Endpoint hat keinen hero_ids-Parameter: Filter muss nach validierter Antwort wirken. Patch-/Schema-Pins müssen ehrlich sein, lokale Request-Labels sind kein Beweis für serverseitige Patch-Mitgliedschaft. Keine direkte DL-Main-/ClickHouse-Verbindung und kein stiller Altdatenfallback. Herkunft und Beobachtungszeit müssen erhalten bleiben.

## Fokus C und Wechselwirkung

Wegwerf-Postgres mit geschütztem eigenem Socket und Peer-Zugang; kein Start/Stop bestehender 5446- oder Produktionsinstanzen. Historische Wrapper dürfen die Sicherheitsregeln nicht mehr verletzen. Keine geerbten echten Zugangsdaten, kein Passwort in Prozessumgebung oder Protokoll. Tatsächliche Postgres-Tests dürfen nicht als Erfolg verschwinden, wenn DB fehlt.

Testqualität statt bloßer Zahl: erhaltene 18 Prozess-E2E-Fälle plus geforderte Negativfälle, beidseitige Scope-Isolation, echte Zahlen-/Aliasantwort, Konflikt trotz Retrieval-Limit 1, falscher Patch/Mode, ungültiger Zugang, legaler/illegaler Build, Hero Card, Provider- und DB-Ausfall samt Recovery, Delete/Revoke. Alle fünf Starting-Stats aus Assets belegt; keine pauschale Sheets-Ersatzimplementierung. Wiki IR/Raw/Facts im normalen Store/Release, Legacy-Import und Tombstones verifiziert. Max_connections/Pool/Timeout-Budget nicht künstlich erhöht.

Last-Runner muss 600 Requests je 8/16/32 Worker auf Pool 4 prüfen und falsche unauthorized_evidence, unbegründete 429/503, Connectionexplosion und fehlende Recovery als Fehler erkennen. End-to-end vom echten Prozess, nicht nur Mock-Erfolg. Frische Schlussmessung nach Integration ist separat; Vorabmessungen auf C allein ersetzen sie nicht.

## Urteil

Zuerst Status/Diffstat, dann Autorenberichte. Eigene sichere gezielte Gegenproben ausführen, keine unkontrollierten externen Tests. Fertig J/N, Abweichungen, Fix nötig J/N. Alle bestätigten Mängel mit Datei:Zeile, tatsächlichem Szenario, minimalem Fix. Vermutungen getrennt. Auftragsgrenzen (Wiki/LLM/Replay/G5-Freigabe) nicht als technische Codefehler umetikettieren, technische Mängel nicht darunter verstecken. Exakte geprüfte SHAs und eigene Befehle/Exitcodes angeben. Bei auftragsrelevanter Lücke BLOCK. Abschlussbericht REVIEW-AC.md im eigenen Taskordner, eigener Berichtbranch pushen.
