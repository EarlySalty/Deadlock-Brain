# Paket I: Integration E und F

## 07.10.2026: Gesamt-BLOCK und verweigerter Projektroot-Zugriff des frischen Fixers

**Blockiert, nicht fertig.** Entscheidung `ENTSCHEIDUNG-I-PATCH-ORIGINAL.md`, 16:09 UTC, umgesetzt: die begrenzten Originalquellenfixes aca42a50/b70dc6b6 samt Core6-Gegenprobe in den tatsächlich geprüften gemeinsamen Kandidaten übernommen. HEAD `501d3725e691c713f4b04468fd9d6b77977ae91c`, Tree `4cca98fe791105203b8951a4f24c6c4abaf43cab`, Basis `ca4d877f13042c9a7a7023e54f6bf2c688b69ac4`. Kandidat auf `origin/feat/brain-i-integration-blocked-20261007` gesichert.

Format und striktes Clippy einschließlich brain-serve Exit 0. Vollständige bestehende Suite 558 passed, 0 failed, 25 ignored, 0 filtered, Exit 0. Zusätzlich vorhandene Bestands-ID-Scratchprobe ausdrücklich am Kandidaten gegen eigene isolierte PostgreSQL-16-Instanz mit privatem Unixsocket ohne TCP: 1 passed, 0 failed, 0 ignored, 116 filtered, Exit 0; eigene Instanz danach beendet. Erster gezielter Lauf ohne Scratch-DSN scheiterte mit `Scratch-DB fehlt: NotPresent`, Exit 101, 0 passed/1 failed; korrigierte Prüfvoraussetzung, kein behaupteter vorbestehender Produktfehler. Befehle und Logs im gesicherten `PRUEFUNG-KANDIDAT-501.md`.

Gemeinsamer Gate mit unverändert Claude Opus 5.5, Exit 1: `BLOCK: The patch import can assign an existing patch a new ID.` Tatsächlichen Kernel nachgelesen: Lookup vergleicht fragmentfreie Anfrage-URL und Feedlink, `post_row` entfernt anschließend zusätzlich die Query. Queryfreier Bestand wird bei `?l=english#notes` ohne passenden Quelldokumenteintrag nicht gefunden. Query-/Slash-/nachweislich identische Steam-Ereignis-URL-Varianten benötigen konsistente bestehende IDs; unterschiedliche Ereignis-/Announcement-GIDs nicht vermischen. Neuer Kern statisch bestätigt, noch nicht durch neue echte PG-Regression reproduziert. Zwei NITs zu unerreichbarem altem Original und Bildlinks bleiben separat offen. Volles Originalurteil `/tmp/brain-i-e-patch-candidate-gate-opus55.log`, `REVIEW-RUNDE-18.md`.

**Frischer Fixer 13 regulär gestartet, aber tatsächlich durch Schutzgrenze gestoppt:** Zugriff auf `/home/nathanael/.worktrees/brain-e-deadlock-api/rust/crates/deadlock-brain/src/pg_patchnotes.rs` verweigert, außerhalb des zugelassenen Projektroots. Ausgang d3d3c5b6, Branch und Graphify bestätigt; keine Codeänderung, Commits, Scratchprobe oder Selbst-Gate. Agent beendet. Hauptsession bestätigt unveränderten Produktstand. Kein anderer Zugriffsweg, Ersatzfix, Worker, Wrapper, Hook- oder Berechtigungseingriff. G/K-Prüfsperren nicht übernommen. Dies ist ein Zugriffsblocker, kein weiterer inhaltlicher Gate-BLOCK. Für Fortsetzung muss das reguläre Routing dem vorgeschriebenen frischen nativen Fixerkontext den bestehenden E-Worktree als zulässigen Projektroot bereitstellen. Keine Schutzlockerung durch diese Session.

Qualifizierte Rückgabe und neue Nachweise im E-Commit `b3d22f7713ff4ed50efb76af1a8d2eba610b408e` auf `origin/feat/brain-deadlock-api-daten` gesichert. Fachakte `/home/nathanael/.worktrees/brain-e-deadlock-api/.tasks/2026-10-07-i-integration/AN_HAUPT-I.md`, Schutzbericht `REVIEW-RUNDE-19-SCHUTZBLOCKER.md`, Briefing `FIXER-13-AUFTRAG.md`. Alte Runden unverändert erhalten. Read-only-Rust-Liveprobe gegen Kandidat 501 erneut gebaut, nicht live ausgeführt; eigene Cargo-Artefakte ohne Löschung nach `/tmp/brain-i-verified-target-20261007.vYUnv5/target` erhalten. Kandidatenquelle sauber einschließlich ignorierter Dateien. Kein Wrapper oder Hook geändert.

TESTNACHWEIS[TW-1]: 558 passed, 25 ignored | Baseline: keine Altfehler behauptet
TESTNACHWEIS[TW-1]: 1 passed, 0 ignored | Baseline: ausdrückliche bestehende Scratch-PG-Probe, Erstlauf ohne DSN fehlgeschlagen

Zählbereich nur diese Fortsetzung ab dem bereits vorhandenen Kandidaten 501: Kandidaten-Featurepush, eigenes Akten-Add, Aktencommit, E-Featurepush. Native Fixer 13 ohne Git-Mutation. Frühere Integration/Fixcommits nicht nochmals gezählt.

MERGEPROTOKOLL[MS-1]: 4 Git-Schritte einzeln | Anläufe: 1 | Gate: [claude-opus-5-5] Gesamt-BLOCK; frischer Fixer durch Projektroot-Schutzgrenze gestoppt

Main enthält weiterhin nur die früher separat freigegebenen Beleg-/Schemapinstufen. Vollständiger E-Produktstand nicht nach main gepusht. Kein regulärer Releasebuild/install, eigener Neustart, vollständiger produktiver Import, Live-Receipt-/Originalhashbeweis oder analytics_runtime-Übergabe. F auf 46fd8674 unverändert; Budget/Imbues/ursprünglicher Abbruch, gleiche PurchasePlan-/InventoryEvaluation-Belege, geprüfter G-Kern und F/G-Anschluss offen. Keine neue Warden-Veröffentlichung oder hero_build_id. Kein Cleanup oder Self-Settle bei offener Lieferung. Eigene Branches und Worktrees erhalten; fremder kanonischer WIP unberührt. Zentrale Akte bleibt beim Delegator, hier ausschließlich die zugewiesene Fachrückgabe ergänzt. Keine Sessionkontakte oder neuen T3-Threads.

LIVEBEWEIS[DV-1]: PID 2645590->nicht neu gestartet | exe ohne (deleted) nach Deploy nicht geprüft | journal -p err nicht geprüft | Anker "nicht geprüft" in Binary | Funktion: kein eigener Deploy oder vollständiger Liveimport bewiesen | Ort: http://127.0.0.1:8788/readyz, nur früherer Vorcheck


## 07.10.2026: einmalig freigegebener gemeinsamer Gate bleibt BLOCK, zwei neue Fehler reproduziert

Entscheidung `ENTSCHEIDUNG-I-READER-GATE.md`, 15:09 UTC, ausgeführt. Gegenprobe `90c17800` nach Standprüfung in den tatsächlichen Kandidaten übernommen. Geprüfter HEAD `860793d7f89d2af9f7510d663e56d592fedf18b2`, Tree `9b917e98bbe2ef24c97319991d2d6bdbbf379254`, frisch geholter origin/main `ca4d877f13042c9a7a7023e54f6bf2c688b69ac4`. Gemeinsame Suite 554 passed, 0 failed, 24 ignored, 32 Targets; Reader-Gegenprobe tatsächlich bestanden. Format und striktes Clippy einschließlich brain-serve Exit 0.

Genau ein gemeinsamer Gate mit unverändert Claude Opus 5.5, Exit 1: `BLOCK: Der Patchimport kann fremden Inhalt falsch zuordnen und bei einem zulässigen Link den Tageslauf abbrechen.` Original `/tmp/brain-i-e-evidence-common-gate-opus55.log`. Der früher widerlegte Reader-Verlust wird nicht mehr genannt. Stattdessen zwei neue tatsächliche Producer-Fehler:

1. **Steam-Ereignisbindung bestätigt:** Kontrollierte Seite enthält nur Ereignis GID ...631; angefragt ist GID ...632. Der bestehende Titel-/Zeit-Fallback liefert den fremden Gameplaybody, der bis zum tatsächlichen PreparedPatch unter der angefragten ...632-URL geführt wird. Begrenzt im bestehenden Aufrufer/HTML-Leser an tatsächlich angefragte Originalidentität binden und fehlende passende Originale ablehnen. Legacy-Announcement-Verträge prüfen; keinen zweiten Parser bauen.
2. **Fragmentabbruch bestätigt:** Für Steam und Forum bestehen Fragmentlinks die Feedprüfung. Der echte HTTP-Core weist dieselben unverändert übergebenen Links vor Abruf ab; der Resolver liefert Err. Der Import propagiert Err und das tatsächliche set-e-Skript lässt den anschließenden Builddatenaufruf aus. Begrenzt im vorhandenen Übergang die Abruf-URL konsistent kanonisieren; HTTP-Guard nicht lockern und Herkunft erhalten.

Beide Kerne mit zwei einmaligen Rust-Diagnosezeugen durch unveränderte Produktfunktionen tatsächlich reproduziert: 2 passed, 0 failed, 0 ignored, 112 filtered. Dies bedeutet erkannte Fehler, keine Produktfreigabe. Original `/tmp/brain-i-new-patch-blocker-diagnostics.log`. Striktes Clippy mit den Zeugen Exit 0. Temporäre cfg(test)-Anbindung vollständig entfernt; Produktcode unverändert und mit dem geprüften Kandidaten verglichen. Steam-HTML ist kontrollierte Synthetik, kein abgerufenes Original; die Probe belegt deterministisches Fehlverhalten, nicht dessen Häufigkeit im echten Feed. Fragmentprobe ohne Netzwerk oder produktives SQL; Tageslaufabbruch zusätzlich statisch bestätigt, nicht produktiv ausgelöst. Bildlink-NIT bleibt getrennt offen.

**Urteil und Rückgabe:** Neuer realer Restkern, kein gleicher fortbestehender Reader-Widerspruch. Gemäß Entscheidung Schritt 4 Urteil samt neuen reproduzierbaren Szenarien zurückgegeben. Kein weiterer Gate, Produktfix oder Fixer 11. Kein Core6-Bruch, zweiter Fallback, Modellwechsel oder Override. Gestoppter Haupt-Orchestrator nicht reaktiviert; keine Sessionkontakte oder neuen Threads.

Kandidat `860793d7` auf `origin/feat/brain-i-integration-blocked-20261007` gesichert, nicht nach main. Neue Akte und reproduzierbare Zeugen im E-Commit `16eaecdee8a0331c97c8bda78220cdf5bdd8b20b` auf `origin/feat/brain-deadlock-api-daten` gepusht. Vollständiger Nachweis: `/home/nathanael/.worktrees/brain-e-deadlock-api/.tasks/2026-10-07-i-integration/NACHWEIS-PATCH-GATE-BLOCK.md`; Diagnosequelle `PATCH-GATE-DIAGNOSE.rs`; wörtlicher Gate in `REVIEW.md`.

TESTNACHWEIS[TW-1]: 554 passed, 24 ignored | Baseline: keine Altfehler behauptet
TESTNACHWEIS[TW-1]: 2 passed, 0 ignored | Baseline: einmalige Fehlerdiagnosen, keine Produktfreigabe

Zählbereich dieser begrenzten Fortsetzung: Fetch, Gegenproben-Merge, Kandidaten-Featurepush, gezieltes Akten-Add, Aktencommit, E-Featurepush. Ein gemeinsamer Gate-Anlauf, keine erneute Main-Teilintegration.

MERGEPROTOKOLL[MS-1]: 6 Git-Schritte einzeln | Anläufe: 1 | Gate: [claude-opus-5-5] BLOCK, zwei neue Patchfehler reproduziert

Main enthält weiterhin nur die früher freigegebenen Beleg-/Schemapinstufen. Kein vollständiger E-Produktstand nach main gepusht, kein regulärer Releasebuild/install, eigener Neustart, produktiver vollständiger Import oder Live-Receipt-/Originalhashbeweis. Keine analytics_runtime-Freigabe. F bleibt unverändert; F/G-Vertragsabschluss und Warden-Veröffentlichung samt hero_build_id offen. Kein Cleanup oder Self-Settle. Eigene Branches und Arbeitsbäume gesichert für die fachliche Fortsetzung erhalten; fremder kanonischer WIP unverändert.

LIVEBEWEIS[DV-1]: PID 2645590->nicht erhoben | exe ohne (deleted) zuletzt vorgeprüft | journal -p err nicht geprüft | Anker "nicht geprüft" in Binary | Funktion: kein eigener Deploy oder vollständiger Liveimport bewiesen | Ort: http://127.0.0.1:8788/readyz, nur früherer Vorcheck

## 07.10.2026: erneute qualifizierte Rückgabe, Reader-BLOCK empirisch widerlegt

**Status: blockiert, nicht fertig.** Fünf weitere erfolglose Fortsetzungs-BLOCKs seit Receipt-ALLOW sind erreicht (Runden 9, 10, 11, 12, 14). Keine weitere Fixdelegation und kein neues Urteil ohne fachliche Fortsetzung. Kein Gate-Override und kein Modellwechsel.

Letztes tatsächliches Produkturteil von unverändert Claude Opus 5.5 gegen `ca4d877f..b63569af`, Exit 1: `BLOCK: Ein späterer Teilimport verdeckt weiterhin vorhandene Assets derselben Clientversion.` Der konkret genannte Reader-Kern ist widerlegt: Der vorhandene INNER JOIN auf das angefragte Endpoint-Dokument greift vor ORDER BY/LIMIT. Ein neuer vollständiger Core6-Run ohne globale Endpoints kommt für den globalen Zugriff nicht in die Ergebnismenge.

Die neue echte Scratch-PG-Gegenprobe erzeugt einen vollständigen 13er-Lauf und einen 25 Stunden späteren Core6-Lauf derselben Version. Alle 13 Value-/Receipt-Kombinationen bleiben lesbar; globale Receipts nennen den älteren tatsächlichen Original-Run. Explizites Pinnen auf den neuen Core6-Run weist dessen fehlende globale Daten korrekt ab. Finale Probe 1 passed, 0 failed, 0 ignored, 225 filtered; Format und striktes Clippy Exit 0. Nur Tests ergänzt, kein Produktfix, keine zweite Pipeline und keine Vermischung von Run-Belegen. Die Probe ist kein produktiver Import. Details, Befehl und Ausgabe: `/home/nathanael/.worktrees/brain-e-deadlock-api/.tasks/2026-10-07-i-integration/NACHWEIS-CORE6-GLOBAL.md`.

**Fachliches Urteil:** Kein belegter neuer Reader-Bug im genannten Szenario. Der offene Restkern ist ein Gate-/Spec-Konflikt; Core6-Kompatibilität erhalten und die tatsächliche Gegenprobe im vorhandenen Gate mit demselben Urteilmodell klären, statt den bereits vorhandenen Fallback nochmals zu bauen. Der weiterhin wirksame BLOCK wird durch diese Feststellung nicht aufgehoben. Der Patch-Link-NIT bleibt ungeprüft und wird nicht als geschlossen gemeldet.

### Tatsächlich integrierte und gesicherte Stände

- Belegintegration `17974c66` und Schemapinintegration `ca4d877f`: separat ALLOW, tatsächlich nach main gepusht. Die alten Aussagen „kein Main-Push“ unten sind historisch. Diese erlaubten Teilstufen bleiben erhalten.
- Vollständiger E-Produktkandidat `b63569afbf2bc6f686564063d769dbbcf0a5ef90`: BLOCK, nicht nach main gepusht und nicht deployt; auf `origin/feat/brain-i-integration-blocked-20261007` gesichert. Eigener Release-Arbeitsbaum bleibt erhalten.
- E-Prüf-/Rückgabecommit `90c178001258d050c781a42c4c9b2fd7cef99bbf`: auf `origin/feat/brain-deadlock-api-daten` gepusht. Produktbasis davor `5f4e3668acb4531cfd31efdaa88e63626f772f84`; dessen Tree `c1d4b6a250737142b2f97f240bbad8c7e42d304b` war identisch zum geprüften Produktkandidaten. Neue Tests und Akte ändern nicht die Produktimplementation.
- F bleibt unverändert auf `46fd86743589910d7b92a7223bdd6ab0dcf2b7c8`. Compiler Exit 0, Composer 28 und Planner 8 passed; kein F/G-Vertragsabschluss daraus ableiten.

Vollständige E-Integrationssuite vor der zusätzlichen Regression: 553 passed, 0 failed, 24 ignored, 32 Testtargets, Harness-Exit 0. Format und striktes Clippy einschließlich brain-serve Exit 0. Originale `/tmp/brain-i-e-main-integration-tests.log`, `/tmp/brain-i-e-main-integration-fmt.log`, `/tmp/brain-i-e-main-integration-clippy.log`. Kein zusammengezählter neuer Gesamtlauf behauptet. Finale Gegenprobe `/tmp/brain-i-core6-global-gate-reproduction-final.log`, Format `/tmp/brain-i-core6-global-fmt-final.log`, Clippy `/tmp/brain-i-core6-global-clippy-final.log`.

TESTNACHWEIS[TW-1]: 553 passed, 24 ignored | Baseline: keine Altfehler behauptet
TESTNACHWEIS[TW-1]: 1 passed, 0 ignored | Baseline: zusätzliche echte Scratch-PG-Gegenprobe

Zählbereich nur gestufte Main-Integration: Beleg-restore/add/commit/push, Schemapin-merge/push, Produkt-merge. Drei zugehörige Integrationsurteile, nicht die gesamte frühere Fixhistorie.

MERGEPROTOKOLL[MS-1]: 7 Git-Schritte einzeln | Anläufe: 3 | Gate: Belege ALLOW, Schemapin ALLOW, vollständiger Produktstand BLOCK

Zählbereich nur anschließende Sicherung und Fachrückgabe: eigener Kandidatenbranch, Featurepush, gezieltes Add, Regression-/Aktencommit, E-Featurepush. Kein neuer Main-Anlauf.

MERGEPROTOKOLL[MS-1]: 5 Git-Schritte einzeln | Anläufe: 0 | Gate: bestehender Produkt-BLOCK unverändert, nicht übersteuert

### Nicht abgeschlossen

Kein regulärer Releasebuild/install, eigener Neustart, vollständiger produktiver Assets-/Patch-/Builddatenimport oder Live-Receipt-/Originalhashbeweis. Die Rust-Liveprobe wurde gebaut, aber nicht ausgeführt. Keine analytics_runtime-Freigabe; F-Budget/Imbues/ursprünglicher Abbruch und dieselben PurchasePlan-/InventoryEvaluation-Belege bleiben offen. Keine neue Warden-Veröffentlichung oder hero_build_id. Kein Cleanup und kein Self-Settle. Vor Fortsetzung tatsächlichen aktuellen origin/main und Live-Binary erneut prüfen; alter Wrapperplan auf `3ceb504d` ist historisch.

LIVEBEWEIS[DV-1]: PID 2645590->nicht erhoben | exe ohne (deleted) zuletzt vorgeprüft | journal -p err nicht geprüft | Anker "nicht geprüft" in Binary | Funktion: kein eigener Deploy oder vollständiger Liveimport bewiesen | Ort: http://127.0.0.1:8788/readyz, nur früherer Vorcheck

Vollständige gesicherte Fachrückgabe und aktuelle Statusakte unter `/home/nathanael/.worktrees/brain-e-deadlock-api/.tasks/2026-10-07-i-integration/`. Zehn native Fixer beendet, kein Fixer 11 oder neuer T3-Thread. Fremder kanonischer WIP unverändert; hier ausschließlich den zugewiesenen Hauptbericht ergänzt.

## Historische Abschnitte vor der erneuten Rückgabe

## 07.10.2026: qualifizierter Blocker nach fünf Gate-Runden

E steht auf `e66184cb31f505ca5fe8bcac9511a228194b9210`. Vier frische native Fixer eingesetzt; jede Korrektur kompiliert, ist formatiert, besteht die gezielten Tests und Clippy. Trotzdem insgesamt fünf inhaltliche BLOCKs desselben Urteilmodells Claude Opus 5.5. Deshalb diese Blockermeldung gemäß Fünf-Runden-Grenze, keine Freigabe und kein Main-Push.

Urteil: Der wiederholte Fehler ist keine Buildstörung, sondern eine auseinanderlaufende Gameplay-Freigabe und Ereignisprojektion. Der jetzige Guard erkennt Klauseln, übergibt anschließend aber unverändert die komplette Mischzeile an den bestehenden Parser. Dort hat `added` Vorrang vor numerischen Änderungen. `Added new artwork and increased weapon damage from 50 to 60` kann dadurch tatsächlich als falscher Änderungstyp statt als Waffenbuff landen. Diesen Gate-Kern hat die Hauptsession am Code bestätigt. Außerdem werden gebundene echte Entitätsänderungen wie zusätzlicher Doppelsprung, Knockback und zusätzliche Ladungen noch verworfen.

Vorschlag für die begrenzte Fortsetzung: Originaltext für Herkunft behalten, bestätigte Gameplayklauseln in derselben vorhandenen Vorbereitung-/Parserstrecke als Ereignisse verarbeiten und echte gebundene Entitätsänderungen zulassen. Keine neue Pipeline, kein LLM-Klassifikator, keine gelockerte Freigabe für beliebige Zahlenwechsel oder bloße Heldennennungen. Nächster frischer Fixer arbeitet ausschließlich diesen lokalisierten Kern ab. Kein Neu-Würfeln durch Modellwechsel. Wörtliche fünfte Antwort steht in E unter `.tasks/2026-10-07-i-integration/REVIEW.md`, Original `/tmp/brain-e-fixer4-gate-opus55.log`.

Letzte gezielte Probe: 36 passed, 0 failed, 0 ignored, 514 filtered; Format und Clippy Exit 0. Leere Pflicht-Assets sind bereits am Produzenten und Leser abgewiesen. Schema-Pin hat unverändert GPT-ALLOW. E noch nicht live; kein vollständiger lokaler Spiegel vorhanden. F bleibt unverändert und kann deshalb noch nicht regulär veröffentlichen. Keine Build-ID, kein Cleanup, kein Self-Settle.

TESTNACHWEIS[TW-1]: 36 passed, 0 ignored | Baseline: keine Altfehler behauptet
Gezählt sind verändernde Git-Schritte: Fetch, Main-Integration, eigener Release-Worktree sowie je Add und Commit der vier frischen Fixer. Read-only-Abfragen sind nicht enthalten.

MERGEPROTOKOLL[MS-1]: 11 Git-Schritte einzeln | Anläufe: 5 | Gate: BLOCK, Semantik-Kern lokalisiert; kein Main-Push

## 07.10.2026: zweiter BLOCK, frischer Fixer 2

Erster Fixer abgeschlossen, Produktfix `9b521fdef435de50385d23150409c4776df1e627`. Originaltextabrufe statt Feed-Teasern und echte Fließtextproben für Steam und Forum umgesetzt; 34 gezielte Tests bestanden, fmt und striktes Clippy Exit 0. Erneute Prüfung mit dem urteilsgebenden Claude Opus 5.5 ergibt wörtlich: `BLOCK: Zwei Pfade können unvollständige Daten als gültigen Patch oder vollständigen Spiegel übernehmen.`

Neue blockierende Kerne: kosmetische Forumtexte können als Gameplaypatch das Zeitfenster verschieben; leere Pflicht-Assets-Arrays können `mirror_complete=true` ergeben. Wörtliche vollständige Antwort in E unter `.tasks/2026-10-07-i-integration/REVIEW.md`, Original `/tmp/brain-e-fixer-gate-opus55.log`. Frischer nativer Fixer 2 arbeitet genau an diesen Kernen. Kein Wiederverwenden des alten Fixerkontexts und kein Main-Push.

Die neue Anweisung für kleinere Prüfdiffs ist übernommen. Schema-Pin gegen seinen tatsächlichen Eltern-SHA separat geprüft: `bfda408c..5e70da3a`, Exit 0, wörtlich `[gpt-6.1-sol] ALLOW: this SHA already passed review_gate [reviewer_model=gpt-6.1-sol]`. Der BLOCK wird dadurch nicht aufgehoben. Patch-Korrektur wird gegen `9b521fde` erneut mit demselben Urteilmodell geprüft; API-Spiegel anschließend ohne das große Schema-Diffpaket mit GPT.

TESTNACHWEIS[TW-1]: 34 passed, 0 ignored | Baseline: 0 rot, gezielte Fixproben

## 07.10.2026: E Integrationsprüfung vor Gate-Fix bestanden

Zweiter vollständiger Testlauf vom Harness mit Exit 0 bestätigt: 517 passed, 0 failed, 24 ignored, 0 filtered out, 31 Ergebniszeilen. Original `/tmp/brain-i-e-tests-2.log`. Strikter Clippy-Lauf erreicht `Finished` nach 21m 30s in `/tmp/brain-i-e-clippy.log`; wegen der zuvor gemeldeten Harness-Unterbrechung wird der endgültige Exit-Nachweis mit dem Fixlauf erneut erbracht. Formatcheck Exit 0. Diese Zahlen belegen den Integrationsstand vor Abschluss der Fixrunde, nicht deren noch ausstehende Freigabe.

Der eigene ausschließlich lesende Rust-Beleg ruft tatsächlich den gemeinsamen Leser auf. Baseline vor Import: `Spiegelprüfung fehlgeschlagen: Kein vollständiger lokaler Assets-Spiegel vorhanden`, Exit 1. Zugang über bestehenden Infisical-FD, kein Secretwert ausgegeben. Öffentlicher aktueller API-Versionsstand mit regulärem User-Agent: `client_version=6759`, `version_datetime=2026-10-06T15:31:13`. Steam-Verbindung und GC-Verbindung sind erreichbar.

TESTNACHWEIS[TW-1]: 517 passed, 24 ignored | Baseline: nicht als rot behauptet

## 07.10.2026: E Gate-BLOCK, frischer Fixer aktiv

Eigene E-Prüfung Exit 1: GPT-Eingabe überschritt das Werkzeuglimit von 1.048.576 Zeichen, anschließend urteilt Claude Opus 5.5 inhaltlich BLOCK. Wörtliches Urteil und Befunde im übernommenen E-Arbeitsbaum unter `.tasks/2026-10-07-i-integration/REVIEW.md`, Rohantwort `/tmp/brain-i-e-gate.log`. Kern: einzelne Änderungszeile beweist keinen Volltext; Teaser können bestehende vollständige Ereignisse ersetzen. Fließtext-Originalpatches können am Bulletzwang scheitern. Frischer nativer Fixer arbeitet begrenzt am Patchimport, Folgerunden mit demselben urteilsgebenden Modell.

Erster Testlauf und erster Clippy-Lauf vom Harness nach jeweils zehn Minuten als gestoppt gemeldet. Die gestarteten Cargo-Prozesse liefen danach nachweislich weiter. Ein zweiter Testlauf mit einstündigem Zeitfenster wartet mechanisch auf denselben Cargo-Lock. Noch kein grüner Gesamtnachweis.

Eigener sauberer detached Release-Arbeitsbaum `/home/nathanael/.worktrees/brain-i-release-20261007` auf `f6f5cef6` angelegt. Regulärer `brain-release plan` erfolgreich. Tatsächliche installierte Symlinks zeigen beide `bfda408c`, nicht den in der Steuerungsakte genannten `f6f5cef6`; das laufende `brain-serve`-Binary stammt ebenfalls aus `bfda408c`. Kein fremder Deploy-SHA übernommen oder zurückgedreht. Noch kein Releasebuild oder Deploy.

MERGEPROTOKOLL[MS-1]: 10 Git-Schritte einzeln | Anläufe: 0 | Gate: [claude-opus-5-5] BLOCK, eigener Fixer aktiv

## 07.10.2026: Integration läuft

E-Arbeitsbaum vor Übernahme sauber. Aktueller Remote-main `f6f5cef65f1f946113f0b8216c6475f6d38ec928` ohne Konflikte in E gemergt. Integrationsstand `8107417036228c707582c76e9ffccdeb1b45cb90`. Formatcheck Exit 0; Tests, Clippy und eigene Gate-Prüfung laufen. Noch kein Main-Push und kein Deploy.

F-Stand `46fd86743589910d7b92a7223bdd6ab0dcf2b7c8`. Definitive F-Übergabe nennt zwei Werkzeugausfälle, kein ALLOW und keinen inhaltlichen BLOCK. Deshalb neue Prüfung nach Integration nötig.

Regulärer Release-Helfer: `/usr/local/libexec/brain-release`, Aufrufe `plan QUELLE`, `build QUELLE NEUES_BUNDLE`, `verify QUELLE BUNDLE`, `install QUELLE BUNDLE`. Release benötigt einen vollständig sauberen eigenen Worktree auf aktuellem Remote-main. Keine Betriebsaktion vor Gate und Main-Push.

G und fremde Arbeitsbäume bleiben unberührt. Schutz-Hooks verlangten direkte Reads der Rollenakten trotz geladenem Skill; diese wurden gelesen, danach regulär fortgesetzt.

MERGEPROTOKOLL[MS-1]: 6 Git-Schritte einzeln | Anläufe: 0 | Gate: eigene E-Prüfung läuft; noch kein Main-Push
