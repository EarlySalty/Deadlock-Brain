# A: vollständige unabhängige Speicher-/Retryprüfung5

Datum: 03.10.2026
Native Rückmeldung des frischen lesenden Prüfers a2cb367318b7b1221, GPT6.1Sol höchstens high. A sichert die tatsächliche Rückmeldung, dieses Schreiben ist keine zusätzliche Prüfung. Rohbericht bleibt bei A; Haupt erhält nur kurze Pfadabdeckung/Laufbeweise.

Urteil: gesamte beauftragte Strecke statisch gelesen. Fix4 schließt die vorher bestätigten Elternsync-Lücken in öffentlichen Wiederholungen. Zwei verbleibende Befunde, kein allgemeines ALLOW.

WIRKUNGSPRUEFUNG[WP-1]: 2 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 1/1 geprüft

## Freeze und tatsächlicher Umfang

Alle vier Module, zentrale Vertrags-/Bereichsanweisungen, FIX-4/REVIEW-LOCAL-4, konkrete öffentliche Aufrufer und vorhandener Datenharness gelesen. Graphify zuerst, konkrete Codelektüre danach. Vor und nach gemessene Hashs exakt Fix4:

- wiki_inventory.rs2507c127aaf8e366f7bb7cf22564074adbaa546b344328d6fdbd593e5b38ef80
- normalize.rs27e63412a7a57c956dc82e602df4f93683adb3eb06866b22381157f85b96dcdc
- storage.rsf48f4819534870b1aa8b2c1f0042f82d1d1ca1b1056242e55dc6632271d4daea
- tests.rsa0d7cb81ce90c7a729c4ac6e92fbe02d3b670ff5e473a307935589adaf26c6d1

Anker relativ zu rust/crates/dbrain-sources/src/: H=wiki_inventory.rs, S=wiki_inventory/storage.rs, T=wiki_inventory/tests.rs.

## Eintrittspfade

API H410-424, XML H426-440 führen über H442-479 in denselben Speicherpfad: Öffnen, Checkpoint, Quellen-/Seitenprüfung, Quellenbindung, Dokumente, Seitenzustand, Checkpoint und Veröffentlichung. Bericht H481-488 öffnet und bindet erneut vor Publikation. Live H201-408 speichert Dokumente vor Cursor-/Namespacefortschritt; Fehlerpfad H286-295 liefert weiterhin Err. Harness verwendet echte XML/API-/Berichtsfunktionen. Produktive gemeinsame Registrierung und bounded-Reader-Anbindung bleiben C2.

## Vollständige Pfadmatrix

| Familie | Konkreter Pfad | Statisches Retry-/Bestätigungsergebnis |
| --- | --- | --- |
| Öffnen/Verzeichnisse | S29-90,749-768 | Verzeichnis und absolute Vorfahren auch bei Existenz erneut synchronisiert; Fehler vor erfolgreichem Öffnen propagiert, gemeinsame Größen aus realen Dateien gezählt. |
| Quellenbindung | S95-205 | Quelle vor Effekt geprüft, source.json unveränderlich installiert; sichtbare Bindung beim Retry über read_optional_json gelesen/Elternsync nachgeholt vor Akzeptanz. |
| Original | S272-353,727-746 | Write/File-sync/Hardlink/Elternsync vor Erfolg; existierendes Original S291 vor Deduplizierung synchronisiert, auch Bindung/Publikation lesen nach. Same-Spool-Budgetrest, siehe Befund1. |
| Herkunft | S360-483,706-724 | Atomare Ersetzung mit Dateisync/Rename/Elternsync; vorhandene Assertion wird erst nach lesendem Elternsync dedupliziert. Erste Beobachtung und Original erhalten. Same-Spool-Budgetrest. |
| Inhaltskonflikt | S297-330 | Neuer Konflikt getrennt/unveränderlich und budgetgeprüft; vorhandener Konflikt S326 synchronisiert vor „erhalten“. Zusätzliche Herkunft verloren, Befund2. |
| Checkpoint | S248-270, H388-404 | Neu schreiben/synchronisieren vor Bestätigung. Wiederöffnung synchronisiert Root vor Checkpointlesen. Privater read_checkpoint allein hat keinen eigenen Elternsync; kein öffentlicher Schreibretry allein darüber nachgewiesen. |
| documents.jsonl | S486-563 | Original-/Herkunftsync, BufWriter.flush/File-sync/Rename/Elternsync; Fehler propagiert, Retry erstellt vollständig neu ohne Existenzshortcut. |
| inventory.json | S564-625 | Atomarer Write/Sync vor Ok(report); keine gemeinsame Dateitransaktion mit JSONL behauptet. |
| Live-Fehlerpfad | H286-295 | Checkpoint/Teilpublikation nach Fehler ändern ursprünglichen Err nicht in Ok; nach Persistenzfehler keine weiteren Dokumentwrites desselben öffentlichen Aufrufs. |

Frühe Fehler vor Installation bestätigen kein neues Ziel. Temporärbereinigung best effort ist keine Speichererfolgsbestätigung. Direkte File-Writes benötigen keinen BufWriterflush, JSONL flusht ausdrücklich.

## Befund1: Same-Spool-Budget nach sichtbarer Installation/Syncfehler veraltet

S323-324,351-352,430-431; Retryzwillinge S291,326,390-393.

Installationsziel wird sichtbar, letzter Elternsync scheitert. Das ? verhindert Aktualisierung von stored_bytes. Derselbe Spoolretry holt Sync nach und dedupliziert ohne Budgetkorrektur. Weiteres neues Dokument/Konflikt kann gegen zu kleinen Zähler erfolgreich gespeichert werden, obwohl reale gemeinsame Dateisumme Budget überschreitet. Herkunftsersetzung kann eine größere sichtbare Dateigröße vom veralteten Zähler abziehen.

Wichtige Abgrenzung: Öffentliche API/XML-Retries öffnen neu und zählen korrekt. Öffentliche gelesene Produktionspfade schreiben nach Persistenzfehler keine weiteren Dokumente desselben Objekts. Befund betrifft ausdrücklich geprüften Same-Spool-Fall, kein behaupteter öffentlicher Datenlauf. T178-225 prüft Retry ohne danach neues Datensatzbudget; T310-372 Budget ohne vorherigen letzten Syncfehler.

## Befund2: zusätzliche Herkunft vorhandenen Konflikttexts verloren

S297-330 gegenüber S332,385-432; öffentlicher Aufrufer H470-472.

Original A für Seite1/Revision101. API liefert Text B ohne Autor, B wird als Konflikt erhalten. Später XML derselben Revision mit gleichem B und belegtem Autor/Contributor. Konfliktkey hängt nur von Originalkey und B-Hash ab. Existierendes Ziel erhält nur Elternsync, dann „Konflikt erhalten“; zusätzliche Autoren-/Contributor-/Capture-/Lizenzherkunft nirgendwo gespeichert. Erste ärmere Konfliktdatei bleibt. Original unverändert ist korrekt; keine Forderung, Konflikt zum Original zu machen oder Rechte aufzuwerten.

A hat S297-332 selbst nachgelesen und den öffentlichen Herkunftsverlust bestätigt. Dieser konkrete notwendige Fix begründet geordnete eigene Datenworkerpause; kein Timerabbruch. Noch keine Schreibfreigabe vor sicherem Punkt.

## Regressionen und Beweisgrenzen

37 Tests tatsächlich gezählt und sämtliche Funktionen gelesen. Vier neue nutzen echte Dateien/tatsächlichen Spool; Injektion nur letztes Elternsyncversagen, kein Fake-Gesamtspool. Einschränkungen: Konfliktfehler privater Same-Spool, Veröffentlichungstest injiziert Herkunftssync statt letztem JSONL/Inventar/Checkpointsync, keine Regression für die zwei neuen Funde. Geschriebene Assertions ohne Lauf keine Runtime-/Crashbeweise.

Eigene echte Prüfernachweise: Quell-/Aufruferlektüre,37 Testdefinitionen, acht SHA256-Messungen. Keine Dateien, Compiler, Tests, Formatierung, Gate, Netz, Git, Crash oder Agenten. Kontextdateilesen verweigert, danach native Werkzeuge ohne Schutzänderung. Compiler/Archive/Hostlockstände aus Briefing sind keine neu bestätigten Messungen dieses Prüfers. Datenworker bleibt erhalten, C2 alleiniger finaler Gate/Integrator.
