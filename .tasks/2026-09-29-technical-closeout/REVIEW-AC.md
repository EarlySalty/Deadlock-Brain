status: Zwischenbericht A und vorläufige C-Wechselwirkung, BLOCK
Datum: 2026-09-29

# R-AC: unabhängige Abnahme des gemeinsamen Brain-Pfads

## Fortsetzung nach Freigabe des vorläufigen C-Stands

A-Bericht zuerst als `e5de1dd` committed und auf den eigenen Reviewbranch gepusht. Danach erlaubte der Orchestrator ausdrücklich die lokale Zusammenführung mit C `e540e979a2a91b613887913d9a3791d2b30b47b6` (Produktcode `db35673` und `f83e01f`). Prüfmerge im eigenen Reviewbaum: `f5570646b9d37a523dbbd01481d1e4e5b35e2b68`. Kein Merge oder Push nach `migration/rust-integration` oder `main`. A `34a2507` wurde als reine Berichtpflege angekündigt, nicht als behobener Produktstand.

**Gesamturteil weiterhin BLOCK.** Die vier A-Befunde bleiben offen. C hat zusätzlich die unten beschriebene Abnahmelücke R-AC-C1. Der endgültige C-Delta wurde für diese Fortsetzung noch nicht übermittelt; daraus folgt keine finale gemeinsame Freigabe. Die nachfolgenden A-Abschnitte dokumentieren den vorherigen, unveränderten A-Prüfstand.

### R-AC-C1: P1, sicherer Ersatzrunner erhält nicht den vollständigen Prozess-Abnahmevertrag

**Stelle:** `scripts/run_isolated_serve_checks.sh:1-4`, Ersatzaufruf `scripts/test_brain_serve.sh:65-66`, Szenarien `rust/crates/brain-serve/tests/process_e2e.rs:397-424`, `:472-549` und `:806-835`. Fehlerklasse F10, Vertragsverlust beim Austausch des Prüfpfads.

**Szenario:** Der neue sichere Runner kann grün werden, obwohl `brain-serve` bei inkompatiblem oder fehlendem Schema fälschlich startet oder selbst migriert. Der alte Runner prüfte dies ausdrücklich in `305df2d:scripts/run_isolated_serve_checks.sh:118-126`; er ist jetzt gesperrt. Im Ersatz-Prozesslauf wird die Datenbank vor jedem Service-Start korrekt migriert, es gibt keinen Start gegen Schema-Version 99 oder eine leere Datenbank und keine Prüfung auf unerlaubtes Anlegen des Schemas. Die Suche über die übrigen `brain-serve/tests` fand dafür ebenfalls keinen gleichwertigen Prozessersatz. Das Sperren des unsicheren Altrunners ist richtig, ersetzt aber die geforderten erhaltenen Abnahmefälle nicht.

Auch neue verlangte Prozessnachweise fehlen: kein englischer Alias-Aufruf `Guardian`, keine wirklich unbekannte Entity und kein Konfliktfall mit Retrieval-Limit 1. `Haze max health` verwendet eine vorhandene, nur anders berechtigte Entity und ist deshalb kein Ersatz für eine unbekannte Entity. Der Rückrichtungs-ACL-Test in Zeilen 542-548 akzeptiert jedes `ClientError` über `.is_err()`, also auch HTTP 503 oder einen Transportfehler, statt die Rechteablehnung präzise nachzuweisen. Nach dem DB-Ausfall wird zunächst nur Readiness geprüft und der Prozess beendet, nicht eine fachlich erfolgreiche Antwort desselben wiederhergestellten Prozesses.

**Beleg und Zwillingssuche:** Alter Wrapper gegen den gesamten neuen `process_e2e.rs` und die übrigen Serve-Tests abgeglichen. Die vorhandene Bibliotheksregression `dbrain-retrieval/tests/chunked_retrieval.rs:176-200` prüft Alias-Konflikt bei Limit 1 korrekt; sie ist kein echter Prozessnachweis. Die fachliche Umsetzung wird daher nicht als fehlend behauptet, sondern die ausdrücklich geforderte Ende-zu-Ende-Abdeckung. Der tatsächlich grüne eigene Lauf unten zeigt, dass diese Lücke auch bei erfolgreichem Runner bestehen bleibt.

**Minimaler Fix:** Die fehlenden ursprünglichen Start-/Nichtmigrationsprüfungen und die genannten Pflichtfälle im geschützten Wegwerf-Cluster ergänzen. Rückrichtungs-ACL auf die vorgesehene Berechtigungsantwort prüfen; nach DB-Recovery vor Neustart eine normale Anfrage erfolgreich beantworten lassen. Bestehende Bibliotheksfälle wiederverwenden, weder Budgets erhöhen noch unsichere historische Wrapper wieder freischalten.

### Eigene vorläufige A+C-Prozess- und Lastmessung

`./scripts/test_brain_serve.sh > .tasks/2026-09-29-technical-closeout/REVIEW-AC-provisional-serve.log 2>&1` auf Prüfmerge `f5570646b9d37a523dbbd01481d1e4e5b35e2b68`: **Exit 0**, drei explizit aktivierte Tests bestanden, null fehlgeschlagen, null ignoriert. Zwei Legacy-Tests prüfen echten CLI-Import beziehungsweise Store/Release/Tombstone/Revoke; der dritte startet den echten Serve-Prozess mit synthetischem Loopback-Provider und echtem Wegwerf-PostgreSQL.

| Worker | Requests | answered | Clientfehler | Laufzeit | Beobachtete Reader-Verbindungen |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 8 | 600 | 600 | 0 | 2694 ms | 4 |
| 16 | 600 | 600 | 0 | 4653 ms | 4 |
| 32 | 600 | 600 | 0 | 2466 ms | 4 |

Poolbericht: Maximum 4, Peak 4, 5 insgesamt erzeugte Verbindungen einschließlich Ausfall/Recovery, 9106 Wiederverwendungen, 3165 Warteereignisse. Genau eine absichtlich ausgelöste Pool-Wartezeitüberschreitung im separaten Sättigungstest, `wait_max_micros=150876`; keine falsche `unauthorized_evidence` und kein HTTP-/Clientfehler in den drei Laststufen. Pool-Wartebudget blieb 150 ms, Serverlimit blieb 12. Der Runner prüfte das PostgreSQL-Log auf Verbindungsüberlauf und räumte seinen eigenen Cluster auf.

Dies ersetzt weder den vom Autor gemeldeten roten C-Lauf noch die vorgeschriebene Schlussmessung nach endgültiger Integration. Es ist eine unabhängige positive Vorabmessung genau des genannten Prüfmerges. Der Prozessharness konfiguriert die neue Analytics-Route nicht und führt keinen Match-API-Ingest aus; sein Erfolg widerlegt A1 oder A3 deshalb nicht. Hostlast wird nicht als Ausnahme vom Abnahmekriterium verwendet. Shellcheck der neun geänderten Runner: Exit 0. Die vier gesperrten historischen Wrapper zusätzlich in bereinigter Umgebung ausgeführt: jeweils Exit 2 mit Sperrhinweis, ohne Produktionszugriff.

Die auf demselben Prüfmerge anschließend ausgeführten Wrapper `./scripts/test_brain_core_postgres.sh` und `./scripts/test_brain_storage_upgrade.sh` bestanden jeweils ihren expliziten PostgreSQL-Test: je Exit 0, 1 passed, 0 failed, 0 ignored. Upgrade-Evidenz umfasst v1 nach v2, DDL-freien Servicezugriff, Checkpoint-Replay, konkurrierendes Upgrade/Fencing, tatsächliches pg_dump/pg_restore sowie atomare Ablehnung beschädigter oder zukünftiger Schemas. Das ersetzt nicht die in R-AC-C1 fehlenden separaten Startprüfungen des Serve-Prozesses.

`./scripts/test_wiki_runtime.sh` ebenfalls Exit 0: 294 passed, 0 failed, 9 zunächst ignored in den summierten Cargo-Ergebnisgruppen, darunter der anschließend explizit bestandene Scratch-Test. Zusätzlich liefen die tatsächlichen CLI-Unterbefehle `brain-wiki-pilot plan` und `stage` erfolgreich mit lokalen Fixture-Artefakten. Kein Wiki-Netzabruf. Zusammen ergeben die vier eigenen A+C-Wrapper 299 bestandene unterschiedliche Tests; drei der neun zunächst ignorierten Fälle wurden in den expliziten Store-, Upgrade- und Wiki-Läufen ausgeführt, sechs blieben nicht ausgeführt.

TESTNACHWEIS[TW-1]: 299 passed, 6 ignored | Baseline: nicht erhoben, kein Altfehlerurteil

Wörtliche Folge nach dem Serve-Runner:

```sh
./scripts/test_brain_core_postgres.sh > .tasks/2026-09-29-technical-closeout/REVIEW-AC-core.log 2>&1 && ./scripts/test_brain_storage_upgrade.sh > .tasks/2026-09-29-technical-closeout/REVIEW-AC-upgrade.log 2>&1 && ./scripts/test_wiki_runtime.sh > .tasks/2026-09-29-technical-closeout/REVIEW-AC-wiki.log 2>&1
```

Gesamtexit 0. Die fünf Cargo-/Wrapper-Rohlogs und `review_probe.rs` wurden nach Abschluss unverändert aus dem Taskordner nach `/tmp/brain-rac-pg.ebzixc/` verschoben. Dort liegen auch das Gegenproben-Executable und die gestoppte A-Scratchinstanz. Die oben angegebenen Befehle dokumentieren die ursprünglichen Ausführungspfade. Die Wrapper stoppten und entfernten ihre eigenen erfolgreichen Wegwerf-Cluster. Der Reviewbaum enthält damit keine uncommitteten Arbeitsbelege.

Protokoll bis einschließlich Prüfmerge:

MERGEPROTOKOLL[MS-1]: 4 Git-Schritte einzeln | Anläufe: 1 | Gate: kein main-Merge beantragt; A-Bericht add/commit/push, danach ausdrücklich erlaubter lokaler C-Prüfmerge

WIRKUNGSPRUEFUNG[WP-1]: 5 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 3/3 geprüft


## Urteil der A-Einzelprüfung vor der C-Fortsetzung

**Fertig: N. Fix nötig: J. A: BLOCK. Gemeinsame Freigabe A/C: nicht erteilt.**

A wurde unabhängig auf `799c68b266555951248c408466f3834ff06eef37` gegen `305df2d36ec7b5d0513d6c0769051b41538d6a1b` geprüft. Letzter Produktcodecommit: `84888918f529f8502e2731d8b09b16d0ee7dbe5b`. Zum Abschluss der A-Einzelprüfung lag noch kein freigegebener C-Prüf-SHA vor. Der A-Zwischenbericht wurde deshalb ohne C-Prüfung und ohne gemeinsame Freigabe zuerst abgegeben. Die inzwischen erlaubte vorläufige C-Wechselwirkungsprüfung steht am Anfang dieses Dokuments.

- `MATCH_RUNTIME_PATH_READY=NEIN`: Die erzeugte API-Anfrage fordert die anschließend zwingend benötigten Spielerfelder nicht an.
- `META_RUNTIME_PATH_READY=NEIN`, `POPULATION_RUNTIME_PATH_READY=NEIN`: Beobachtungsroute vorhanden, aber keine nachgewiesene Anbindung an den fachlichen Brain-Antwortpfad und keine verifizierte Patch-Zuordnung.
- Atomare Store-/Release-Transaktion und CLI-Revoke funktionieren in der separat ausgeführten PostgreSQL-Fixture. Das beweist keinen funktionierenden API-Ingest und keinen vollständigen Prozess-E2E.
- Keine Produktänderung, kein Integrationsmerge, keine Produktion, keine echten Match-/Replay-Abfragen, keine Wiki-Aufnahme, keine Provider-Aufrufe. Keine Unteragenten oder anderen Threads.

Die Befunde R-AC-A1 bis A4 sind technische beziehungsweise auftragsrelevante Lücken. Keine davon wird zu einer ausstehenden Betreiberfreigabe umbenannt. Die tatsächlichen Freigabegrenzen für Wiki, Provider, Replay und G5 bleiben davon getrennt.

A-Einzelprüfung: 4 Befunde, Zwillingssuche grep-belegt, 3/3 Fremddienst-Pfade geprüft.

TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 20 belegt | Senke: interner Reviewbericht im Taskordner

## Prüfstand und Vorgehen

Eigener Baum: `/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929`, Branch `review/pre-g5-core-abnahme-20260929`. Zu Beginn sauber auf dem genannten A-SHA. Diffstat vor Autorenbericht gelesen: 18 Dateien, 2809 Einfügungen, 103 Löschungen. Anschließend `A-REPORT.md`, verbindlichen `AUFTRAG.md` und `R-AC-BRIEFING.md` gelesen.

Graphify-Abfragen gegen `/home/nathanael/.graphify/global-graph.json` liefen vor der Code-Suche. Der globale Graph lieferte überwiegend ältere Symbole, deshalb wurden die konkreten Treffer und der eingefrorene Git-Diff nachgelesen. Prüfung direkt durch diesen Reviewer, nicht durch den Implementierer.

## Bestätigte Befunde

### R-AC-A1: P1, Match-Ingest fragt die benötigten Spielerfelder nicht ab

**Stelle:** `rust/crates/brain-feeds/src/deadlock_match.rs:125-130`, Gegenvertrag `:208-221` und `:257-260`; tatsächlicher Aufrufer `rust/crates/brain-feeds/src/bin/brain-match-ingest.rs:144-166`. Fehlerklasse F10, Typ- und Semantik-Drift an Systemgrenzen.

**Szenario:** Eine gültige konfigurierte Match-/Account-Kombination wird mit `match_ids`, `account_ids`, `only_filtered_players=true`, `limit=1` und `format=json` angefragt. Keines der `include_player_*`-Felder ist gesetzt. Upstream sind diese booleschen Felder standardmäßig false. `only_filtered_players` filtert vorhandene Spielerprojektionen, aktiviert sie aber nicht. Ohne Spielerprojektion fehlt `players` in der Antwort. Der neue Adapter verlangt anschließend genau einen Spieler mit der angefragten Account-ID und verwirft daher auch die korrekte Antwort. Es entsteht kein veröffentlichbarer Batch.

**Unabhängiger Beleg:** Öffentliches Upstream-Repository `deadlock-api/deadlock-api`, eingefrorener Stand `290cedba6cca7d8a07015e9feefecd22de27643c`, Datei `api/src/routes/v1/matches/bulk_metadata.rs`: Zeilen 98-100 setzen `include_player_info` auf serde-default false; Zeilen 292-300 liefern ohne ausgewählte Spielerfelder keine Spielerspalten; Zeilen 568-572 ergänzen `players` nur bei vorhandenen Spielerspalten. Der lokal eingecheckte OpenAPI-Vertrag enthält dieselben Auswahlparameter. Keine reale Match-Abfrage war nötig.

Eigene Rust-Gegenprobe gegen die unveränderten A-Bibliotheken:

```text
generated_url=https://api.deadlock-api.com/v1/matches/metadata?match_ids=456&account_ids=123&only_filtered_players=true&limit=1&format=json
author_shape: ACCEPT records=1 validated=true schema=0.1.0
upstream_default_no_player_projection: REJECT quarantined source: match metadata does not bind the requested account
```

Die IDs 123/456 wurden ausschließlich in lokalen synthetischen Antworten verwendet, niemals an die API gesendet. Die Autoren-Fixture enthält `players` unabhängig von der erzeugten URL und verdeckt diesen Fehler.

**Minimaler Fix:** Die benötigte Spielerprojektion explizit anfordern, denselben Parameter in der exakten Locator-Validierung erlauben und den URL-zu-Antwort-Vertrag mit upstreamgetreuer Fixture prüfen. Ein Test darf `players` nicht unabhängig von den gesetzten Auswahlparametern hinzuerfinden.

**Zwillingssuche:** `match_metadata_url`, `validate_match_locator`, `contains_account`, CLI-Ingest, `prepare_match_response` sowie `tests/match_store.rs:27-36` geprüft. Die Bibliotheks- und PostgreSQL-Positivtests verwenden dieselbe künstlich vollständige Antwort. Der Revoke-Pfad fragt keine Metadaten ab und ist von diesem Fehler nicht betroffen.

### R-AC-A2: P1, Schema-Pin ist kein Schema-Guard für den persistierten Match-Inhalt

**Stelle:** `rust/crates/brain-feeds/src/deadlock_match.rs:241-244`, `:289-315`; übernommener Helfer `rust/crates/dbrain-sources/src/deadlock_api.rs:1085-1157`, Account-Prüfung `rust/crates/brain-feeds/src/deadlock_match.rs:213-220`. Fehlerklassen F10 und F14, wirkungsloser Wächter.

**Szenario:** Die Antwort enthält korrekte Match-/Account-IDs, aber `hero_id` wird vom Integer zu einem Objekt geändert oder ein neues verschachteltes Spielerfeld erscheint. `prepare_match_response` prüft nur Identitäten und die grobe Array-/Objektform. Es hängt den lokalen OpenAPI-Hash und dessen Version an, validiert aber nicht den später vollständig als Fakt gespeicherten Inhalt. Unbekannte Top-Level-Felder werden nur vermerkt; unbekannte Spielerfelder und deren Typen werden nicht geschlossen abgewiesen. Die neue Adapterfunktion serialisiert trotzdem die ganze Zeile in `Metadata`.

**Unabhängiger Beleg:** Eigene Gegenprobe lieferte für alle folgenden Antworten einen gültigen `SourceBatch` mit `schema_version=0.1.0`:

```json
[{"match_id":456,"players":[{"account_id":123,"hero_id":{"drift":true}}]}]
[{"match_id":456,"players":[{"account_id":123,"hero_id":18,"new_private_players":[{"account_id":999}]}]}]
[{"match_id":456,"players":[{"account_id":123,"accountId":{"drift":true},"hero_id":18}]}]
```

Ergebnis jeweils `ACCEPT records=1 validated=true`. Beim dritten Fall ignoriert `contains_account` einen vorhandenen, aber nicht parsbaren Alias, sofern `account_id` gültig ist. Beim zweiten Fall gelangen fremde verschachtelte Spielerinformationen ohne überprüften Projektionsvertrag in den accountgebundenen Dokumentinhalt. Dies ist kein Nachweis eines aktuell solchen Upstream-Payloads, sondern ein reproduzierbarer fehlender Drift-Guard vor dem Store.

**Minimaler Fix:** Einen überprüften Antwort-/Projektionsvertrag für die tatsächlich angeforderten und gespeicherten Matchfelder verwenden. Identitätsaliase bei Vorhandensein vollständig validieren. Bei unerwarteten Feldern, Typen oder zusätzlicher Spielerstruktur vor `CoreDocument`/Batch geschlossen abbrechen oder quarantänisieren. Ein Hash der lokal vorhandenen OpenAPI-Datei ersetzt diese Prüfung nicht. Der OpenAPI-Metadatenendpunkt beschreibt einen Byte-Stream und bietet allein keinen vollständigen JSON-Feldvertrag.

**Zwillingssuche:** Match und Demo getrennt geprüft. `demo_evidence_documents` prüft die beiden direkten IDs explizit, hat aber ebenfalls keinen fachlichen Feldschema-Validator. Analytics verwendet dagegen `validate_consumed` und lehnt zusätzliche Felder ab (`analytics_runtime.rs:319-335`); diesen Schutz nicht mit dem Matchpfad verwechseln. Die vorhandene Match-Drift-Fixture testet nur `players` als String, nicht Drift innerhalb gültiger Spielerobjekte.

### R-AC-A3: P1, Beobachtungsroute schließt die geforderte Meta-/Population-Integration nicht ab

**Stelle:** `rust/crates/brain-serve/src/service.rs:223-243` und `:250-259`, `rust/crates/brain-serve/src/analytics.rs:68-73`, `rust/crates/dbrain-sources/src/analytics_runtime.rs:397-412`. Fehlerklasse F1, Baustein vorhanden ohne geforderte Wirkung.

**Szenario:** Nach Konfiguration der neuen Route kann ein berechtigter Aufrufer Rohbeobachtungen abrufen. Die normalen Brain-Anfragen verwenden weiterhin unverändert `ReleaseRetriever -> Kernel -> CachedKernel -> ApiService`. Die Analytics-Instanz wird lediglich als separate Router-Erweiterung angehängt. Im eingefrorenen Baum gibt es keine Übergabe von `AnalyticsObservation` an diesen fachlichen Antwort-/Buildpfad oder an `PopulationSlice`. Ohne solche Anbindung kann eine Meta-/Population-Frage die neue Laufzeitquelle nicht nutzen.

Zusätzlich gibt es ausschließlich `PatchMembership::Unverified`. Der lokale Request-Patch und das konfigurierte Fenster attestieren keine serverseitige Patch-Mitgliedschaft. Dass dies ehrlich ausgegeben wird, ist richtig, stellt aber die verlangte patchgebundene fachliche Laufzeitintegration nicht her. `Population` verwendet `build-item-stats`, also Anzahlen von Builds pro Item, nicht automatisch eine empirische Matchpopulation. Die fachliche Zuordnung muss belegt werden, nicht allein durch die Enum-Bezeichnung entstehen.

**Beleg:** Repo-weite Symbolsuche nach `AnalyticsObservation`, `AnalyticsLookupRequest`, `DeadlockAnalyticsClient`, `/v1/analytics/observation` und `PopulationSlice`: Laufzeitverwendung ausschließlich im neuen Sources-Modul und in der separaten Route; `PopulationSlice` nur als bestehender Vertrag. Die Service-Komposition übergibt keine Analytics-Abhängigkeit an Kernel oder Retrieval. Der Autorenbericht bestätigt selbst, dass patchbezogene Nutzung blockiert bleiben muss und kein Prozess-E2E dieser Route ausgeführt wurde.

**Minimaler Fix:** Den bestehenden fachlichen Meta-/Population-Pfad mit der bounded API-Quelle verbinden und über eine normale Brain-Anfrage nachweisen. Eine überprüfte Patch-/Fensterzuordnung oder explizit nicht beantwortbare fachliche Antwort muss deterministisch wirken. Build-Häufigkeiten nur für die belegte Semantik einsetzen. Bis dahin beide READY-Marker NEIN. Keine Produktionsaktivierung oder neue Anbieterfreigabe ist erforderlich, um diese technische Lücke zu benennen oder offline zu schließen.

**Zwillingssuche:** Meta und Population, Kernel, Brain-API, Brain-Client, Retrieval, `dbrain-reasoner/src/meta.rs`, `population_prior.rs` und vorhandener `PopulationSlice`-Vertrag berücksichtigt. Optionales Einschalten ist an sich kein Fehler; fehlende fachliche Verdrahtung und fehlender Patchnachweis sind die offenen Abnahmekriterien.

### R-AC-A4: P2, Analytics startet nach dem Slot-Warten ein neues Zeitbudget

**Stelle:** `rust/crates/brain-serve/src/analytics.rs:88-101`, Budgetprüfung `rust/crates/brain-serve/src/config.rs:313-316`. Fehlerklasse F11, inkonsistentes Gesamtdeadlinebudget.

**Szenario:** Zulässige Konfiguration: `analytics.request_timeout_ms=5000`, `timeouts.request_ms=10000`. Vier Lookups belegen alle Slots. Eine weitere Anfrage kann fast 10000 ms auf einen Slot warten und danach einen neuen Lookup mit nochmals bis zu 10000 ms HTTP-Gesamtbudget beginnen. Der äußere Lookup-Timeout kommt zusätzlich auf 10250 ms. Damit kann die Route deutlich nach der konfigurierten 10000-ms-Anfragedeadline antworten. Sie liegt außerhalb der `ApiService`-Deadlinebehandlung; deren Konstruktor erhält zwar `request_ms`, die neue Analytics-Route jedoch nicht.

**Beleg:** Statisch durchgehender Aufrufpfad und Budgetrechnung: `timeout(2T, acquire_owned)` gefolgt von `timeout(2T+250 ms, spawn_blocking(...))`, innen `total_timeout=2T`. Kein gemeinsamer Startzeitpunkt, keine verbleibende Deadline und keine übergreifende Middleware. Dieser Befund wurde nicht durch eine Lastmessung zeitlich reproduziert und wird nicht als solche ausgegeben.

**Minimaler Fix:** Eine einzige absolute Anfragedeadline einschließlich Slot-Wartezeit verwenden und nur das verbleibende Budget an HTTP/Lookup weiterreichen. Queue-Wartezeit darf das konfigurierte Requestbudget nicht verdoppeln. Regression mit vier belegten Slots und verzögerter fünfter Anfrage.

**Zwillingssuche:** Innerer Analytics-HTTP-Client, CLI-Match-Budget und normale `ApiService`-Komposition geprüft. Der innere HTTP-Lookup hat eine bounded Gesamtdeadline; der Fehler entsteht durch das vorgeschaltete Warten mit anschließend neuem Budget.

## Unauffällige Pfade und getrennte Hinweise

1. **Store und Recovery:** `commit_batch_tx` wird von altem Einzelcommit und neuem Mehrfachcommit gemeinsam benutzt. `commit_batches_and_publish` kapselt beide Batches, Release-Publikation und Readback in derselben Transaktion. Die separat ausgeführte echte PostgreSQL-Fixture bestätigt Match-/Demo-Revoke, historische ACL-Invalidierung, Checkpoints und Rollback bei falscher zweiter Fence. Es blieb kein halb geschriebener Match-/Demostand sichtbar. Alte Upgrade-Suites wurden statisch berücksichtigt, aber ihre ignorierten PostgreSQL-Fälle hier nicht zusätzlich ausgeführt.
2. **HTTP-Grenzen:** Kanonische HTTPS-URL, keine Userinfo, keine Redirects, keine transparente Dekodierung, Byte-/Row-/Retry-Grenzen und Identitätsfilter sind vorhanden. Hero-Stats wird nach validierter Antwort lokal auf `hero_id` gefiltert; kein erfundener `hero_ids`-Parameter. Keine direkte DL-Main-/ClickHouse-Anbindung und kein stiller Datenbankfallback im neuen Analytics-Modul gefunden.
3. **Proxy-Beobachtung, nicht als zusätzlicher Defekt gezählt:** Der gemeinsame `HttpClient` übernimmt Umgebungsproxies. Eigene lokale Gegenprobe mit `HTTPS_PROXY` sah `CONNECT review.invalid:443 HTTP/1.1`. Kein externer Host wurde kontaktiert. `.no_proxy()` fehlt am bounded Client (`deadlock-brain-core/src/http.rs:100-109`), sodass die kanonische URL nicht zugleich einen direkten Transport beweist. Ob ausdrücklich erlaubte Proxies zulässig sind, legt der vorliegende Auftrag nicht eindeutig fest. Vor einer entsprechenden Egress-Aussage ist dies festzulegen; hieraus wird kein TLS-Bypass oder Datenabfluss behauptet.
4. **Lease-Hinweise des Autors bestätigt:** Claim erfolgt außerhalb der Mehrfachtransaktion. Fehlender zweiter Claim oder zurückgerollter Commit kann eine Lease bis zu 60 Sekunden halten. Dies ist eine begrenzte Retry-Verzögerung, kein Beleg einer halb widerrufenen Veröffentlichung. Kein zusätzlicher Blocker dafür aufgenommen.
5. **Diagnostik:** Die neue Route vereinheitlicht Join-Fehler, Timeout und Source-/Quarantänefehler zu `analytics_unavailable` ohne ereignisspezifischen Log; die CLI verwirft Ursachen über generische `map_err`-Texte. Bei den Pflichtfixes sichere strukturierte Fehlergründe ergänzen, ohne Credentials oder private Rohdaten zu protokollieren. Separater Wartbarkeitshinweis, nicht Teil der vier Blockbefunde.

Fremddienst-Wirkungsprüfung: Match, Meta und Population liefern bei Transport-/Validierungsfehlern keinen Erfolg. Bounded Transport-Retries sind vorhanden; Match-Store-Retry hat die genannten Lease-Grenzen. Fehlerursachen und Rohstatus gehen an den neuen Service-/CLI-Grenzen teilweise verloren. Demo wurde als Eingangsadapter ohne Download geprüft.

## Eigene Ausführungsnachweise

Arbeitsstand bei allen Proben: A-SHA `799c68b266555951248c408466f3834ff06eef37`, keine Änderung an Produktcode oder Bestandstests. Eigener neu aufgebauter Debug-Targetbaum, keine Wiederverwendung eines fremden Buildartefakts.

| Probe | Tatsächliches Ergebnis |
| --- | --- |
| Vier betroffene Cargo-Pakete, offline/locked | Exit 0; 25 Ergebnisgruppen, 179 passed, 0 failed, 15 ignored |
| Separater echter PostgreSQL-Matchtest | Exit 0; 1 passed, 0 failed, 0 ignored, 1 filtered |
| Eigenes Rust-Gegenprobenprogramm gegen A-rlibs | Compile Exit 0, Lauf Exit 0; die dokumentierten Vertragsfehler reproduziert und lokaler Proxy-CONNECT beobachtet |
| Öffentlicher Upstream-Quellvertrag via GitHub | Exit 0 für `deadlock-api/deadlock-api` und den genannten SHA; keine API-Nutzdaten abgefragt |

TESTNACHWEIS[TW-1]: 180 passed, 14 ignored | Baseline: nicht erhoben, kein Altfehlerurteil

Die 14 sind die nach dem separaten PostgreSQL-Lauf weiterhin nicht ausgeführten unterschiedlichen Testfälle. Der reguläre Cargo-Lauf meldete 15 ignored; genau einer davon wurde anschließend explizit ausgeführt. Die Gegenproben werden nicht als zusätzliche bestandene Cargo-Tests gezählt. Die 18 Prozess-E2E-Fälle und 600er-Lastreihen werden hier ausdrücklich nicht behauptet; sie gehören zur noch ausstehenden C-/Integrationsabnahme.

### Wörtliche Befehle

Zuerst, im Verzeichnis `rust/`, noch vor dem Harness-Wechsel in den registrierten Reviewbaum:

```sh
env -i HOME=/home/nathanael USER=nathanael PATH=/home/nathanael/.cargo/bin:/usr/bin:/bin CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target /home/nathanael/.cargo/bin/cargo +stable test --locked --offline -p brain-feeds -p dbrain-sources -p brain-serve -p brain-storage > /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/.tasks/2026-09-29-technical-closeout/REVIEW-A-tests.log 2>&1
```

Exit 0, über Background-Abschlussereignis bestätigt, keine Pipeline zur Maskierung des Cargo-Exitcodes. `env -i` ließ keine echten Zugangsdaten oder Proxywerte in den Testprozess gelangen.

Wegwerf-PostgreSQL, ausschließlich eigener privater Scratchbaum:

```sh
mktemp -d /tmp/brain-rac-pg.XXXXXX
mkdir -m 700 /tmp/brain-rac-pg.ebzixc/.match-test-pg
env -i HOME=/tmp/brain-rac-pg.ebzixc USER=nathanael PATH=/usr/lib/postgresql/16/bin:/usr/bin:/bin /usr/lib/postgresql/16/bin/initdb -D /tmp/brain-rac-pg.ebzixc/data --auth-local=peer --auth-host=reject --no-locale -E UTF8 > /tmp/brain-rac-pg.ebzixc/initdb.log 2>&1
env -i HOME=/tmp/brain-rac-pg.ebzixc USER=nathanael PATH=/usr/lib/postgresql/16/bin:/usr/bin:/bin /usr/lib/postgresql/16/bin/pg_ctl -D /tmp/brain-rac-pg.ebzixc/data -l /tmp/brain-rac-pg.ebzixc/postgres.log -o "-k /tmp/brain-rac-pg.ebzixc/.match-test-pg -p 55481 -c listen_addresses='' -c unix_socket_permissions=0700" -w start
env -i HOME=/tmp/brain-rac-pg.ebzixc USER=nathanael PATH=/usr/lib/postgresql/16/bin:/usr/bin:/bin /usr/lib/postgresql/16/bin/createdb -h /tmp/brain-rac-pg.ebzixc/.match-test-pg -p 55481 -U nathanael brain_match_test_55481
```

Alle ausgeführten Schritte Exit 0. Kein TCP-Listener, lokaler Peer-Zugang, privates Verzeichnis und Socket, kein Passwort. Keine bestehende 5446- oder Produktionsinstanz gestartet oder gestoppt.

Der Harness blockierte anschließend zwei Varianten des bereinigten Cargo-Aufrufs wegen seiner Worktree-Kommandoerkennung (`HOME` und `env ... test`). Sie wurden nicht als ausgeführte Tests gewertet. Stattdessen lief das unmittelbar zuvor aus demselben A-SHA erzeugte Testexecutable direkt:

```sh
env -i USER=nathanael PATH=/usr/bin:/bin BRAIN_MATCH_TEST_PG_SOCKET=/tmp/brain-rac-pg.ebzixc/.match-test-pg BRAIN_MATCH_TEST_PG_PORT=55481 /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target/debug/deps/match_store-639e3ce198db8aff postgres_match_commit_release_readback_replay_and_revoke --ignored --exact --nocapture
/usr/lib/postgresql/16/bin/pg_ctl -D /tmp/brain-rac-pg.ebzixc/data -m fast -w stop
```

Beide Exit 0. Die eigene PostgreSQL-Instanz ist gestoppt. Scratchbelege bleiben lokal erhalten.

Eigene Rust-Gegenprobe, aus dem Review-Root:

```sh
/home/nathanael/.cargo/bin/rustc +stable --edition=2021 .tasks/2026-09-29-technical-closeout/review_probe.rs -L dependency=rust/target/debug/deps --extern brain_feeds=rust/target/debug/deps/libbrain_feeds-18a4ea1cfd2f0846.rlib --extern brain_contracts=rust/target/debug/deps/libbrain_contracts-c2a3d5430dd3a1d0.rlib --extern deadlock_brain_core=rust/target/debug/deps/libdeadlock_brain_core-4c422d1932efad79.rlib --extern tempfile=rust/target/debug/deps/libtempfile-6fb3e2d82ec0a5bb.rlib -o /tmp/brain-rac-pg.ebzixc/review-probe
env -i PATH=/usr/bin:/bin /tmp/brain-rac-pg.ebzixc/review-probe
```

Je Exit 0. Das lokale Probeprogramm und der Cargo-Rohlog sind unveröffentlichte Arbeitsbelege, keine Produktänderungen. Nur dieser Bericht wird committed und gepusht. Die Match-Gegenfälle sind oben vollständig angegeben: Status 200, Content-Type `application/json`, `observed_at=1790000000`, `attempts=1`, Scope `account:123`, Private, alle drei Policy-Freigaben false; URL aus `match_metadata_url`. Jeweils `prepare_match_metadata_batch(..., None)` und anschließend `batch.validate()` aufrufen.

### Recherche- und Ausführungsgrenzen

Die ersten drei Quellabrufversuche gegen das nicht vorhandene Repository `deadlock-api/deadlock-api-rust` scheiterten mit 404. Danach wurde das tatsächliche öffentliche Repository über die Organisationsliste ermittelt; nur dessen oben genannter fester Stand ist Beleg. Ein erster Diffstat-Aufruf enthielt einen Tippfehler im Basis-SHA und scheiterte mit Exit 128; alle Bewertungen verwenden die korrigierte vollständige Basis. Context-mode konnte Dateien außerhalb seines ursprünglichen Projektroots nicht direkt öffnen; diese wurden über das native Read-Werkzeug gelesen. Keine Schutzregel wurde geändert oder umgangen.

Quellbeleg: [Upstream-Spielerprojektion am geprüften SHA](https://github.com/deadlock-api/deadlock-api/blob/290cedba6cca7d8a07015e9feefecd22de27643c/api/src/routes/v1/matches/bulk_metadata.rs#L292-L300).

## Nächster Abnahmeschritt

R-AC-A1 bis A4 im A-Paket beheben und erneut unabhängig prüfen lassen. Dieser Bericht ist keine gemeinsame A/C-Freigabe. C wird erst auf ausdrücklich übermitteltem festem SHA in derselben Reviewfortsetzung geprüft.
