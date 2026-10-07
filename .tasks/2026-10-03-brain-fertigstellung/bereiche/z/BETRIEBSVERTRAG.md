status: aktiv
Datum: 2026-10-03

# Betriebsvertrag Z für P, Q und K

Stand nach 18:41 UTC. Letzter bestätigter gepushter Z-Featurekopf: `b86353acca8027431de58a2edadbebcdd01bad10`, Main-Integrationsbasis weiterhin `358ed4ee07d315d4d71dd0438f9446f5b6a22d49`. Die Codex-Übernahme und die aufgehobene Kreisabhängigkeit sind gelesen. Fachpakete liefern lokal geprüfte SHAs; Z verantwortet gemeinsame Abnahme, Gate, Integration und Installation. Fachliche Live-Nachweise folgen nach dieser Installation. Kein Produktivwechsel wurde ausgeführt.

## Tatsächlicher Runtime-Pfad für P

Die aktive Datei `/etc/deadlock-brain/maintenance-runtime.json` wurde lesend auf diese nicht geheimen Felder beschränkt geprüft. Reguläre Datei, UID/GID 1000, Modus 0600:

| Feld | Tatsächlicher Wert |
| --- | --- |
| `serve_config` | `/home/nathanael/.config/deadlock-brain/brain-serve.json` |
| `serve_unit` | `brain-serve.service` |
| `maintenance_config` | `/etc/deadlock-brain/maintenance.json` |
| `artifact_dir` | `/home/nathanael/.local/state/deadlock-brain/maintenance/artifacts` |
| `status_file` | `/home/nathanael/.local/state/deadlock-brain/maintenance/status.json` |

Vorhandener Einstieg: `/opt/deadlock-brain/maintenance-current/brain-maintain --config /etc/deadlock-brain/maintenance-runtime.json`. Keine Serve-Konfiguration oder Secretwerte wurden ausgegeben. Der Pfad ist ein Betriebsnachweis, keine Aussage über den installierten C9- oder Writer-Code.

## Aktivierung wiederkehrender Writer

Z baut den gemeinsamen Einstieg auf dem vorhandenen Rust-Vertrag `brain_maintenance::integration::{activation::ActivationPlan, config_writer::ConfigWriter}`. P/Q rufen ausschließlich diesen Einstieg auf. Die Module sind vorhanden; `ActivationPlan::prepare`, `load`, `needs_rebase`, `activate` und `rollback` bilden den journalgebundenen Austausch mit Restart und erwarteter Readiness. Der Plan ändert den Standardrelease mit ID und Knowledge-Version. Unabhängige C9-Grants und deren Release-Pins dürfen dabei nicht verändert werden.

Der vollständige vorhandene Maintenance-Publikationspfad in `runner.rs:1295` nimmt zuerst `Artifacts::publication_lock` aus `integration/artifacts.rs:177`, danach `ConfigWriter::lock_async` am genannten `serve_config`. Die Publikationssperre liegt unter dem tatsächlichen `artifact_dir` als `publication.lock`. Er liest den aktuellen Release unter dieser Sperre erneut, erhält fremde Quellenpins und behandelt überholte Publikationen über einen Rebase. Die Datenbankaktivierung ist dort über eine konkrete Maintenance-Lease an `activate_maintenance_checked` gebunden.

Für P/Q gilt die gleiche Reihenfolge:

1. Gemeinsame Publikationssperre und denselben ConfigWriter erwerben; aktuellen Standardrelease und Basishash unter den Sperren erneut prüfen.
2. Kandidaten über vorhandene Store-/DocumentSet-Verträge erzeugen. Alle fremden Quellenpins erhalten. Bei veralteter Basis sauber neu aufsetzen oder ohne Aktivierung abbrechen.
3. Aktivierungsjournal vor dem Austausch dauerhaft ablegen; den vorhandenen `ActivationPlan` verwenden. Beide Sperren bis zur bestätigten Readiness oder abgeschlossenem Rückfall halten.
4. Erfolg erst bei aktivem erwarteten Release melden. Ein Store-Commit allein bleibt Kandidatenpublikation. Fehler dürfen keine erfolgreiche Aktivierung vortäuschen.

Grenze des Bestands: Auf Zs Basis besitzt `brain-maintain` keinen allgemeinen CLI-Befehl zum Aktivieren beliebiger P/Q-Kandidaten. `tick` betreibt den registrierten Dokumentpflegepfad. `write-serve-config --input <Pfad> --expected-sha256 <Hash>` prüft und ersetzt die vollständige Konfiguration atomar, startet aber den Server nicht neu und ist kein journalgebundener Writer-Aktivierungsaufruf. Der vorhandene `activate_maintenance_checked` ist ebenfalls kein beliebiger Kandidatenadapter.

Seit der Entscheidung des Hauptorchestrators um 16:24 UTC liegt die gemeinsame Kandidatenorchestrierung ausschließlich bei Z in `brain-maintenance`. P/Q nutzen denselben Einstieg und verdrahten `ActivationPlan` nicht separat.

Angekündigter Schreibumfang vor dem fortgesetzten Bau: neue `rust/crates/brain-maintenance/src/bin/brain-candidate-activate.rs` mit lokalen Tests; für die seit 16:43 beauftragte interne Zielbindung zusätzlich ausschließlich `rust/crates/brain-maintenance/src/integration/activation.rs`, `rust/crates/brain-serve/src/health.rs` und ein enger Zielbindungs-Hashhelfer in `rust/crates/brain-serve/src/config.rs`. Bestehende `brain-maintain.rs`, `integration/mod.rs`, Workspace-Manifeste und Lockdateien bleiben durch den Adapter unverändert. Der Aufrufer verwendet dieselben öffentlichen `Artifacts`, `ConfigWriter`, `RuntimeConfig` und den erweiterten `ActivationPlan`; kein zweiter Configwriter oder Aktivierungsorchestrator.

Der tatsächliche Q-C9-Vertragsstand `e48c189` und `5c220a8` ist als uncommittierter Cherry-pick-Overlay in Z vorbereitet, ohne Konflikt mit den ausschließlich neuen Adapter-/Ops-Dateien. Qs laufende Provider-/Audit-/Retentionsarbeit in seinem Worktree bleibt unberührt; diese zusätzlichen Änderungen sind noch nicht in Z übernommen. Der Overlay ist weder ein geprüfter Gesamtstand noch ein Merge oder Deploy. Q/P verändern die genannten Z-Aktivierungspfade nicht parallel. Zusätzlicher Dateiumfang braucht vorherige konkrete Ankündigung.

Vorbereiteter, noch nicht baubar belegter Aufruf:

```text
brain-candidate-activate
  --config /etc/deadlock-brain/maintenance-runtime.json
  --target standard|second-brain-internal
  --base-release-id <ID>
  --base-release-sha256 <SHA256>
  --candidate-release-id <ID>
  --candidate-release-sha256 <SHA256>
  --expected-serve-config-sha256 <SHA256>
  --allow-source <Source-ID>
  [weitere --allow-source]
  [--apply]
```

Releasehash ist SHA256 über die vollständigen `serde_json::to_vec(&CorpusRelease)`-Bytes des gelesenen Storeobjekts, nicht über Pretty-JSON oder JSONB-Text. Konfigurationshash bindet die exakten Dateibytes. Die Quellenliste enthält genau die erlaubten tatsächlich geänderten Quellen und keine Duplikate. Ohne `--apply` erfolgen nur Prüfungen, kein Konfigurationstausch und kein Restart. P/Q reichen vorhandene Kandidaten weiter und ergänzen deren Serialisierungshashes, ohne eigene Configschreiblogik. Qs derzeitige Ausgabe mit `activation_performed=false` bleibt bis erfolgreicher gemeinsamer Aktivierung korrekt.

Die Adapterfortsetzung ist beendet. Formatierung und Formatcheck liefen mit Exit 0; Clippy, Adaptertests und bestehende Suites endeten jeweils mit Exit 101 vor Compilerstart, weil die vorbereiteten C9-Manifeste nicht zur Workspace-Lockdatei passen. 16 lokale Adaptertests sind definiert, keiner lief. Vor dem nächsten Schreibauftrag angekündigte Erweiterung des Z-Integrationsumfangs: ausschließlich zusätzlich `rust/Cargo.lock`, minimal an die bereits vorbereiteten Manifeste angepasst, keine breit angelegte Abhängigkeitsaktualisierung. Danach frische Prüfung des erhaltenen Vier-Dateien-Adapters und der tatsächlich betroffenen bestehenden Suites. Weiter keine Installation, Aktivierung oder Live-Aussage.

Der Einstieg verlangt ein eindeutiges Aktivierungsziel. P nutzt den Standard-/Patchnotespfad. Qs Sheet-/YouTube-Kandidaten ändern ausschließlich den einzigen Second-Brain-Grant `second-brain/internal` und `internal_operator.release` atomar auf denselben Kandidaten; Standardrelease, öffentliche Docs- und Twitch-Bindungen bleiben unverändert. Basis-ID und Basishash gehören jeweils zur ausgewählten Bindung. Eine bloße Standardrelease-Umschaltung erfüllt den internen Vertrag nicht.

Die Basis wird unter den vorhandenen Sperren frisch geprüft; veraltete Kandidaten werden verweigert. P/Q bleiben für einen fachlich korrekten Rebase verantwortlich. Fremde Quellenpins und alle nicht ausgewählten Konfigurationsfelder bleiben erhalten. Erfolgsbeleg braucht Journal, Restart und Readiness für die tatsächliche ausgewählte Bindung. Die bisherige `/readyz`-Antwort nennt nur den Standardrelease; ein enger Hashnachweis der im Server geladenen Releasebindungen soll die interne Zielprüfung ermöglichen, ohne private Inhalte oder Credentials auszugeben. Ein Fehler braucht belegten Rückfall oder klar gesperrten Zustand. Timerfreigabe erst nach Prüfung des integrierten Einstiegs. Qs Retentions-/Reader-/Cachevertrag wird daran gekoppelt, sobald Q ihn lokal geprüft übergibt. Keine manuelle JSON- oder Datenbankkorrektur.

## Consumervertrag für K

Qs Vertrag vom 16:15 UTC und `docs/c9-consumer-contract.md` sind gelesen. Z legt den vorgesehenen Operatorpfad fest:

`/home/nathanael/.local/state/deadlock-brain/operator/brain.sock`

Dieser Pfad ist noch nicht installiert. Z richtet bei der gemeinsamen Installation das private Elternverzeichnis mit UID 1000 und Modus 0700 ein. Qs Listener erzeugt den Socket mit 0600 und prüft die Peer-UID. Ein vorhandener Socket blockiert nach C9 den Start; kein fremder Socket wird gelöscht. K kann seinen lokalen Client jetzt an genau diesen absoluten Pfad binden.

| Consumer | Serverbindung | Credentialname |
| --- | --- | --- |
| Docs | `actor_id=docs-client`, `channel=docs`, `scopes=[docs.public]`, `provider_egress=[public]`, eigenes öffentliches C9-Release | `BRAIN_SERVE_DOCS_PUBLIC_TOKEN` |
| 2nd-Brain | `actor_id=second-brain`, `channel=internal`, `scopes=[second_brain.internal]`, `provider_egress=[]`, eigenes internes C9-Release | `BRAIN_SERVE_SECOND_BRAIN_TOKEN` |

Die Credentialnamen bezeichnen den vorhandenen Infisical-Weg über `dl_token_secrets`, ohne ENV-Fallback oder Credentialdatei. Z ändert keine Secretwerte. Docs bleibt auf `/v1/answer` und `/v1/retrieve`; 2nd-Brain nutzt ausschließlich `/v1/operator/query` am privaten Socket ohne Modellanbieter. Der interne Grant fehlt in der öffentlichen Registry. Sein festes Release muss mit `internal_operator.release` übereinstimmen. Die vorhandene Twitch-Bindung bleibt erhalten.

Die konkreten C9-Release-IDs und Knowledge-Versionen kommen aus Qs hashgeprüften Importmanifesten, nicht aus frei erfundenen Namen. Z veröffentlicht diese Kennungen nach erfolgreichem Import und geprüfter Konfiguration für K. Bis dahin ist dieser Teil eine festgelegte Transport-/Principalbindung und kein Live-Beweis.

K weist seine Testanfragen anschließend über den SHA256 seiner vollständigen Request-ID und Qs redigiertes `authenticated_request`-Ereignis nach. Keine Inhalte, Credentials oder Gesprächskennungen in Prüflogs. Das Ereignis allein beweist keinen Anbieteraufruf.

## Gemeinsame Binary-Installation und Datenübergang

Die fehlende minimale Binary-Deploymechanik liegt ausdrücklich bei Z. Sie muss den sauberen eigenen Quellstand an aktuellen Remote-main binden, einen echten Herkunftsbeweis der installierten Bytes durchsetzen und beide bestehenden Releasezeiger gemeinsam serialisieren. CLI, Serve und Maintenance dürfen nicht still auf verschiedenen Source-SHAs enden. Der erste Rust-Ops-Stand ist als `b86353a` auf dem Featurebranch gepusht. Zentraler Gate: `[gpt-6.1-sol] BLOCK`, weil frei änderbare Manifesthashes keinen Herkunftsbeweis liefern und unterbrochene Layout-/Journalveröffentlichung nicht vollständig recoverable ist. Frischer eigener Fixkontext übernimmt die erhaltenen Änderungen. Noch kein zulässiger Produktivinstaller und keine Installation; genaue Funde in `REVIEW.md`.

Der frische Legacy-Abgleich bleibt gekoppelt mit Qs Alt-/Neubindung, Widerrufen und Tombstones. Ein September-Backup wird nicht über neu entstandene Kernrevisionen zurückgespielt. Der Vertrag steht in `FRISCHEIMPORT.md`. G5-Sicherheitsvoraussetzungen sowie der volle fehlerfreie Tageszyklus vor G6 bleiben erhalten.
