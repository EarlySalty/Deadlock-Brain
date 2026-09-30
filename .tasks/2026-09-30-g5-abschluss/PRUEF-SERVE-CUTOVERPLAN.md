status: aktiv, Quellvorbereitung und Prüfung warten auf Ressourcenslot
Datum: 2026-09-30

# Bestehenden typed Servepfad bis zum belegten Cutover führen

## 1. Arbeitsort und Stand

Aktuell geprüft:

| Zweck | Worktree | Branch | HEAD |
|---|---|---|---|
| Koordination | /home/nathanael/.worktrees/brain-technical-closeout-20260929 | integration/technical-closeout-20260929 | 1c362bca6d35e7fec10125b2b159e7513a299243 |
| Quelländerung | /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930 | fix/g5-replay-deferred-20260930 | 1c362bca6d35e7fec10125b2b159e7513a299243 |

Der Quellworktree war sauber. Die Koordination enthielt nur den unversionierten neuen G5-Auftrag. Keine gemeinsame Checkoutumschaltung, keine zurückgenommenen Änderungen. Der lokale Graph fehlt im Quellworktree; die globale Graphify-Abfrage lief erfolgreich, lieferte aber überwiegend ältere/benachbarte Knoten. Maßgeblich sind daher die anschließend direkt geprüften Dateien dieses Heads, nicht die Graphlabels.

Nutzerfreigaben gelten: G5 nach allen Nachweisen, Replay später. Neu bindend ist die hostweite Ressourcenabstimmung im Twitch-Auftrag. Keine erneute Freigabefrage und keine automatische Wiederaufnahme eines Workers vor Slotzuteilung.

## 2. Replay: konkrete Abtrennung statt optionaler Abhängigkeit

### Belegte Ursache

- `rust/Cargo.toml:28`: `crates/dbrain-replay` ist weiterhin Workspace-Member. Deshalb betrifft die Git-Auflösung auch Builds anderer Workspacepakete.
- `rust/crates/dbrain-replay/Cargo.toml:10-12`: direkte Bindung an haste und valveprotos; dungers kommt transitiv.
- Andere geprüfte Cratemanifeste enthalten keinen dbrain-replay-Pfad und keine dieser Gitquellen.
- Das Replaymanifest erbt Version sowie serde/serde_json/sha2/tempfile vom Rootworkspace. Nur die Memberzeile zu löschen reicht deshalb nicht für einen unabhängig bedienbaren Replaybestand.

Statische Erreichbarkeitsprüfung des aktuellen Cargo.lock, ohne Cargo-Aufruf: 25 Workspacewurzeln, ohne Replay 24. Von 427 Paketen bleiben 386 erreichbar. Keine unaufgelöste Kante in der statischen Auswertung, keine verbleibende Gitquelle. 41 Pakete sind ausschließlich außerhalb dieser V1-Menge, einschließlich dbrain-replay, haste_core/haste_vartype, valveprotos und der vier dungers-Pakete. Das ist ein belastbarer Änderungshinweis, noch kein Cargo- oder Buildnachweis.

### Vorbereiteter Änderungsschnitt für den vorhandenen Sol-Thread

1. Im Rootmanifest Replay aus `members` nehmen und ausdrücklich aus dem V1-Workspace ausschließen. Nicht nur `optional`, `default-members` oder `--exclude` setzen, denn das belegt keine Git-freie Auflösung.
2. Replay als eigenständigen zurückgestellten Workspace erhalten. Geerbte Paket-/Dependencyangaben konsistent auflösen, ohne Parser-/Schema-Pins zu ändern. Vorhandenen passenden Lockstand für diesen getrennten Bereich erhalten. Kein Vendoring und kein neuer Fork.
3. V1-Lockdatei mit Cargo auf dem vorhandenen Cache neu auflösen. Keine manuellen Einzel-Löschungen von Transitivpaketen und keine pauschalen Versionsupdates. Metadaten müssen für V1 null Gitquellen und keinen Replaypfad zeigen.
4. Bestehenden Decoder-Prüfaufruf auf das abgetrennte Replaymanifest richten: `architecture/migration/replays/s14/check-decoder.sh:8-12` verwendet derzeit das Rootmanifest. Tests und Parsercode erhalten, keine Erfolgsbehauptung für nicht ausgeführte Replaytests.
5. Automatische V1-Prüfpfade nachziehen: `.github/workflows/ci.yml:183-184` sowie `.github/workflows/rust-core-verification.yml:164-166,214-220,264` erwarten Replaylauf/-Artefakte im integrierten Workspace. Replay aus der ausdrücklich definierten V1-Matrix und ihrer Artefaktliste abtrennen, nicht eine fehlende Pflichtprüfung als grün maskieren. Zurückgestellten Replaylauf separat dokumentieren.
6. Aktuelle G2-/G3-/G5-Dokumentation mit der Nutzerentscheidung ergänzen. Historische Replaybefunde nicht umschreiben und nicht als neue Nachweise ausgeben.

Keine Produktquelldatei wurde in dieser Fortsetzung geändert. Dieser Schnitt ist zur isolierten Umsetzung vorbereitet; Implementierer- und Revieweraufrufe unterliegen aktuell ebenfalls der Slotsperre.

## 3. Prüfweg unter der Slotsperre

Die konkreten beantragten Befehle stehen in:
`/home/nathanael/Documents/.tasks/2026-09-30-twitch-alles-live/BRAIN-G5-BUILD-REQUEST.txt`.

Reihenfolge:

1. Bestehenden Sol-Thread am ausdrücklich angegebenen Quellworktree fortsetzen, keine Unterthreads. Vorher dessen Status lesen. Ausschließlich eigene Quelländerungen, kein Compiler/Reviewmodell/Restart ohne passenden Slot.
2. Nach Quelländerung Metadaten und Lockdiff prüfen. Mit vorhandenem Cargo-/Targetcache arbeiten. Ein Fetch aus vorhandenem Cache ist kein vollständiger Nachweis eines leeren Cache; unter dem Verbot neuer Caches wird ein solcher Nachweis nicht erfunden.
3. Format, Clippy und Tests seriell. Toolchain 1.97.1 ist vorhanden. SQLx- und Harness-Konfiguration vor dem Start ohne neue ENV-Konfiguration klären. Bestehende DB-/Prozesstests im isolierten Wegwerfkontext nachweisen; keine produktiven Credentials oder bestehende PostgreSQL-Instanz als Testziel.
4. Releasebuild genau einmal im zugeteilten Fenster aus dem geprüften eigenen Worktree, mit vorhandenem Targetcache und hostweitem Release-Lock. Ein Lock ersetzt die Slotzuteilung nicht.
5. Unabhängiger Review des kombinierten V1-/Consumerstands und lokales Merge-Gate. Jeder Modellreview erst im zugeteilten Slot. Danach reguläre Integration, keine Übersteuerung roter Befunde. Rote Actions allein sind kein Mergehindernis.

Die früheren 1.011 Workspace-Tests und Lastnachweise gehören zum Produktstand 022f8a9. Die Anzahl verändert sich bei ausdrücklich zurückgestelltem Replay. Künftige Ergebnisse werden neu gezählt und mit ausgelassenen Replayfällen getrennt berichtet.

## 4. Tatsächlicher Serve-Bestand und Startlücken

Quellen:

- `rust/crates/brain-serve/src/main.rs:9,16,30-32`: vorhandenes Binary mit `--config`, Health/Readiness und `POST /v1/answer`. Configdatei und bestehender Secretpfad, kein neuer Brain-Servicekern erforderlich.
- `service/systemd/brain-serve.service.example:12-15`: bisher nur Beispielpfade `/opt/deadlock-brain-c1` und Secret-Exec. Die Vorlage enthält eine Environment-Konfigurationszeile und darf unter dem aktuellen Verbot nicht blind installiert werden. Bestehenden Infisical-/Auth-Pfad unverändert benutzen, Konfigurationspfad explizit über den vorhandenen Dateiparameter führen. Keine Secretdateien lesen.
- `config/brain-serve.example.json:2,4-19`: Beispiel bindet an 8787, PostgreSQL-Port 5432 und Loopback-Testprovider. Das ist keine produktive Konfiguration und keine Modellfreigabe.

Live nur lesend festgestellt:

- Brain-PostgreSQL-Systemunit läuft. Kein Neustart durchgeführt.
- In den gelesenen geladenen System-/Userunits kein nachgewiesener typed Brain-Serve-Dienst. Die laufende Brain-Build-Site ist nicht `/v1/answer`.
- Die Beispielpfade für Serveconfig, Infisicalconfig, beide `/opt/deadlock-brain-c1/bin`-Binaries sowie die beiden geprüften Userunitnamen existieren nicht. Das beweist das Fehlen an diesen Stellen, nicht das Fehlen jeder denkbaren Installation.
- 127.0.0.1:8787 und 127.0.0.1:8789 sind bereits belegt. Die Graphify-Architekturunit läuft. Keinen dieser Ports blind umbelegen. Den künftigen Brain-Port mit tatsächlicher Twitch-Consumerkonfiguration abgleichen, nicht nur nach momentan freiem Socket wählen.
- Vorhandene Releasebinaries liegen im bereits genutzten Harnesscache. Ihr Vorhandensein beweist weder den neuen Quellstand noch einen laufenden Dienst.

## 5. Geordneter Serve-Cutover nach Nachweisen und Slot

1. Tatsächliches Inventar vervollständigen: installierte Unit, unveränderlicher Releasepfad/Hash, Consumer-Endpoint, PostgreSQL-Socket und dedizierte Rolle, freigegebenes CorpusRelease samt Knowledgeversion und ACL-/Tombstonezustand. Keine Produktivdaten im Taskbericht speichern.
2. Bestehende Auth-/Infisicalreferenzen und vorhandenen freigegebenen Modellpfad verwenden. Weder Beispielmodell noch neuer Provider, kein zusätzlicher Kostenzweig. Wiki-/Provider-G2-Nachweise anhand tatsächlich vorhandener Freigaben und Daten prüfen. Replay wird hierfür ausdrücklich nicht mehr verlangt.
3. Restore/Rückweg sowie reale Daten-/Consumerparität belegen. Vorhandenes Runbook `infra/cutover/README.md` ist historische Vorbereitung, kein ausgefülltes aktuelles Releaseinventar. Keine alte Markerzeile automatisch auf JA setzen.
4. Nach unabhängigem Review und lokalem Gate erst den exakten Stand regulär integrieren. Produktionsartefakt muss den gemergten Quellstand belegen; ein früher gebautes Branchbinary nicht ohne Baum-/Hashbeweis verwenden.
5. Exakten Startbefehl, Unit, SHA, Ressourcen und Rückweg vor dem Dienstwechsel erneut in der Buildanfrage eintragen. Integrator teilt das Fenster nach dem Twitch-/DL-Cutover zu. Keine parallele Veröffentlichung, kein eigener Twitch-/DL-Neustart.
6. Vor Consumerumschaltung Brain allein starten. `/healthz` und `/readyz` genügen nicht: authentisierten typed `/v1/answer`-Aufruf mit freigegebener öffentlicher Frage, konkreter Zahl/Quelle und passendem Release nachweisen; ungültige Identität und unzulässige Scopes dürfen keine Evidenz liefern. Zunächst Pfad ohne Providerkosten, echte Providerprobe nur im bereits freigegebenen Rahmen und Slot.
7. Integrator prüft denselben Pfad vom tatsächlichen Twitch-Consumer aus. Kein sichtbarer Chatpost und kein Discordpost. Danach kontrollierte Consumeraktivierung, Latenz-/Fehlerbeleg und Betriebskontrolle. Keine Aktivierung nur aufgrund erreichbarer Health-URL.
8. Bei Fehlern neue Aktivierung zurücknehmen, bisherigen Consumerzustand erhalten. Keine Rücknahme aktueller ACL-Sperren, Tombstones oder Daten über altes Backup. Produktive Writer/Legacytimer nicht für einen bloßen API-Start ungeprüft abschalten; vollständiges Writer-Fencing bleibt eigener belegter Schritt des G5-Vertrags.

## 6. Konkrete Blocker und offene Beweise

| Punkt | Tatsächlicher Stand | Nächster zulässiger Schritt |
|---|---|---|
| Ressourcen | Kein konkreter Slot zugeteilt | Buildanfrage liegt beim Integrator; bis dahin nur statisch arbeiten |
| Replay-Abhängigkeiten | Ursache und Git-freier V1-Graph statisch belegt, Änderung noch nicht implementiert | Vorhandenen Implementierer nach Modellslot am bestätigten Worktree fortsetzen |
| Lock-/Compilerbeweis | Neue V1-Auflösung nicht ausgeführt | Slots B/C, vorhandene Caches; Ergebnis nicht vorwegnehmen |
| Serveinstallation | Vorlage/Beispiel vorhanden, kein laufender typed Dienst nachgewiesen | Reale Unit, Config, Port, Release und Consumerendpoint abgleichen |
| G2/G3 | Historische Akte nennt Wiki-/Provider-Echtnachweise und Datenparität offen | Belege prüfen, keine neuen Quellenaufnahmen oder Kostenpfade erfinden |
| PostgreSQL/Auth | Isolierte frühere Tests vorhanden, aktueller produktiver Service-Passwortstart nicht nachgewiesen | Geplanten Serve-Start mit bestehendem sicheren Secretpfad prüfen |
| GitGuardian | Vorheriger offener Vorfall 37635766; heute nicht erneut bewertet | Normalen Prüf-/Abschlussweg, keine Historyänderung |
| Produktionswechsel | Bedingt vom Nutzer freigegeben, Ressourcenfenster und Beweise fehlen | Erst nach Nachweisen und Integrator-Slot |

## 7. Ehrlicher Abschluss dieser Vorbereitung

Keine Cargo-Ausführung, keine neuen Caches, keine Benchmarks, keine Worker-/Reviewmodellaufrufe, keine Secrets gelesen, kein Deploy, kein Restart, kein Communitypost. Kein neuer Produktcommit und kein Live-Nachweis. Die reine Auftragsakte wird auf dem eigenen Branch gesichert; es gibt im Repo keine aktiven Git-Hooks. Kein Main-Merge und kein expliziter Modellreview während der Slotsperre. Übergeordnete Schutz-Hooks bleiben unverändert.
