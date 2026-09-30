status: aktiv, statisch vorbereiteter Servevertrag; kein Startauftrag
Datum: 2026-09-30

# Serve-, Config- und Rückweganforderungen für G5

Brain-Produktbasis: `9a29b81d230c01e5c03423cc34ba34c1074eab69`. Quellworktree `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930`. Keine neue Architektur; geprüft wurden vorhandenes `brain-serve`, Secret-Exec und `infra/cutover/README.md` samt `RELEASE_MANIFEST.yaml`. Der historische prepare-only-Status des Vertrags wird nicht als erneute Nutzerfreigabe verlangt: „Ja, nach allen Nachweisen“ gilt, konkrete Ressourcenfenster bleiben nötig.

## 1. Port und Consumerbindung

- Bestehender Einstieg `rust/crates/brain-serve/src/main.rs:9-26`: `brain-serve --config <Datei>`, GET `/healthz`, GET `/readyz`, POST `/v1/answer`. Kein neuer API-Kern und kein separater OAuth-/Anbieterpfad.
- `Config.bind` ist eine SocketAddr (`config.rs:10`); gebunden wird erst nach Startupprüfungen (`service.rs:262`). Loopback muss in der tatsächlichen Betriebsconfig ausdrücklich gesetzt werden. Aus dem Datentyp allein folgt keine Loopback-Sperre.
- Read-only-Socketaufnahme am 30.09.2026: `127.0.0.1:8787` belegt durch python3, PID 1117; `127.0.0.1:8789` belegt durch devfeed-api, PID 2867142. Keine Umbelegung und kein Beenden dieser Prozesse. Beispiel- oder Testports sind keine Reservierungen.
- Twitch-Quellmomentaufnahme `d828481624d53408e0c0a4c3ed1a8e4a6d421c40`, `rust/crates/tb-config/src/dashboard_options.rs:12-58`: bestehende Modi Legacy/Shadow/Typed, expliziter lokaler Endpoint, nicht leere Public-Scopes und Timeout bis 60.000 ms. Default Legacy. Der Port 8789 steht dort im Test, nicht als nachgewiesener Produktionsendpoint.
- `tb-knowledge/src/brain.rs` verwendet den bestehenden AsyncBrainClient mit trusted Public-Scopes; kein stiller Rückfall auf den alten Antwortpfad bei Backendfehler. Diese Quellaufnahme beweist weder die aktuell deployte Twitch-Version noch ihre aktive Config.
- Noch zu binden: ein konkret konfliktfreier Loopbackport, identisch in Serveconfig und der vom Integrator verwalteten Consumerconfig. Die reale Consumerconfig wurde nicht gelesen oder geändert. Kein beliebiger Port aus einer Momentaufnahme als endgültig freigegeben ausgegeben.

## 2. Normale Config und bestehender Secretweg

| Bereich | Vorhandener Vertrag | Vor Start tatsächlich nachzuweisen |
| --- | --- | --- |
| JSON | `brain-serve/src/config.rs:174-208`: Datei, maximal 64 KiB, unbekannte Felder verboten, Validierung vor Laufzeit | Absolute lesbare revisionsgebundene Config, keine Secretwerte, kein ENV-Setup |
| PostgreSQL | `config.rs:29-47,211-230`: absoluter Unix-Socket, Port, Rolle, Datenbank, Peer oder Passwortreferenz, Pool 1 bis 16 | Dedizierte tatsächliche Brain-Instanz, bestehende Rolle und Rechte, tatsächlicher Socket/Port; kein Default auf zentrale Bot-DB |
| Release | `config.rs:232-238`, `service.rs:129-165`: feste Release-ID und Knowledgeversion, kein current/latest; Schema, Rechte und Snapshot vor Bind geprüft | Freigegebenes CorpusRelease mit passender Knowledgeversion, Schema und Source-/ACL-/Tombstonezustand |
| Provider | `config.rs:239-287`: bestehender OpenAI-kompatibler Provider, geprüfte URL, Modell, Secretreferenz, Preise/Budgets | Bereits freigegebener Provider-/Modellpfad mit echtem Preis-/Budgetstand; Beispielmodell ist kein Produktmodell |
| Zeitgrenzen | `config.rs:288-303`: DB-/Provider-/Request-/Shutdownfristen konsistent | Bestehende gemessene Budgets übernehmen; keine Erhöhung für grüne Tests |
| Credentials | `config.rs:325-349`, `secrets.rs:33-78`: feste Actor-/Channel-/Scope-/Egressbindungen, unterschiedliche Secretwerte | Bestehende Dienstidentität an passende tatsächlich vorhandene öffentliche Quellen binden; keine privaten Scopes als Standard |
| Analytics | `config.rs:305-323`, `service.rs:132-159`: vorhandener optionaler Pfad mit gepinntem Schema und identischem Releasepatch | Nur aktivieren, wenn der bestehende Vertrag und die dazugehörigen Belege erfüllt sind; keine neue Quelle als Ersatz |

Secret-Exec besteht bereits: `deadlock-brain/src/bin/deadlock-brain-secret-exec.rs:36-60` liest die explizite Infisical-Config und exec't danach genau ein Binary. `deadlock-brain-core/src/pg_secrets.rs:15-25` erwartet Projekt, Umgebung, Secretpfad, Socketpfad, Datenbank-Secretname sowie optional Credential-FD/-Name. `:164-197` unterstützt systemd Runtime Credential oder vorhandenen FD. Die systemd-Zeile `Environment=INFISICAL_CONFIG_FILE=...` aus der Beispielunit wird hierfür nicht benötigt: der bestehende Startweg bekommt beide Configpfade ausdrücklich per `--config`.

Keine neue ENV-Datei oder Umgebungsvariable für normale Konfiguration. Der bestehende interne Übergabemechanismus Secret-Exec zu `Secrets::from_environment` bleibt unverändert; das ist kein Auftrag, Secrets auszulesen, auszugeben oder in Dateien zu schreiben. Ein neuer Secretkanal wäre ausdrücklich nicht Teil dieser Vorbereitung.

Twitch-Referenz in `tb-dashboard-api/src/uplink_config.rs:80-82`: `brain_service_token()` verwendet den bereits vorhandenen `TWITCH_INTERNAL_API_TOKEN` aus der geschützten RAM-Quelle. Für die Serve-Seite muss die Config auf dieselbe bestehende Secretreferenz zeigen und deren Actor-/Channelbindung festlegen. Kein neues Token anlegen. Beispielreferenz `BRAIN_SERVE_API_TOKEN` ist keine belegte Produktionszuordnung. Existenz/Zugriff auf Werte hier nicht abgefragt.

## 3. Unit, Ressourcen und Artefakt

Die vorhandene Vorlage `service/systemd/brain-serve.service.example` ist eine Userunit mit Secret-Exec und LoadCredential. `TimeoutStopSec=20` muss über der tatsächlich gewählten `timeouts.shutdown_ms` liegen; Beispiel 12 Sekunden. Ein absoluter unveränderlicher Binarypfad muss den nach Gate gemergten SHA nachweisen. Der vorhandene Harness-Releasecache ist kein solches Produktivartefakt.

Read-only-Aufnahme der installierten Unit-Dateien und ausgewählter Statusfelder um 10:33 UTC:

- `deadlock-brain-postgresql.service`: active/running, User deadlock-brain-pg; Executable wegen Zugriffsgrenze nicht aufgelöst. Keine Konfiguration oder DB-Verbindung gelesen.
- Keine installierte typed Brain-Serve-Unit in den abgefragten Brain-Namensfamilien nachgewiesen. Das ist kein globaler Beweis über beliebig anders benannte Units.
- `deadlock-brain-site.service`: active/running, PID 1107, Executable `/usr/bin/python3.12.dpkg-new (deleted)`. Dies ist der bisherige Siteprozess, nicht der typed Servepfad und kein Python-freier G5-Betrieb. Nicht angefasst; der spätere Gesamtabschluss muss das bestehende Betriebsinventar entsprechend auflösen.
- `dl-knowledge.service`: active/running, PID 378065, Binary unter `/home/nathanael/.local/share/deadlock-bots/releases/46c4f07c/dl-knowledge`. Keine Gleichsetzung mit Brain `/v1/answer`, kein Neustart.
- Build-Daten und YouTube-Learning: failed. Patchnotes-, Sheet-, Wiki- und Feederdienste zum Messzeitpunkt inactive. Zugehörige Timer sind installiert und teils enabled. Inaktiv heißt nicht, dass der nächste Schreibjob gesperrt ist.

Es wurden ausschließlich Unitnamen, Zustände, PID, User, Fragmentpfad und `/proc/<pid>/exe` gelesen. Keine Environment-/Kommandozeilen-/Secretwerte, keine Journalausgabe und kein Reparaturversuch.

Vor Installation offen: tatsächlicher Unitname, Dienstidentität, absolute Config-/Releasepfade, dedizierte PG-Rechte, funktionierender LoadCredential-/Infisical-Socketzugriff, revisionsgebundenes Binary und Rückweg. Die vorhandene Beispielunit wird nicht blind kopiert. DL/Twitch dürfen unabhängig nach ihrem Gate deployen; ihre Units gehören dem Integrator und werden hier nicht verändert.

## 4. Health ist noch kein Antwortbeweis

`service.rs:129-165` prüft Schema, Rechte, freigegebenen Snapshot und Knowledgeversion vor dem Socketbind. `/readyz` liest denselben Release erneut, vergleicht den vollständigen Releasewert, prüft Rechte und verweigert während Drain (`health.rs:55-86`). `/healthz` ist davon zu unterscheiden.

Nach separat zugeteiltem Start ist erforderlich:

1. Tatsächlicher Listener nur am vereinbarten Loopbackendpoint, laufendes Binary samt gemergtem SHA, Health und Readiness mit richtigem Release.
2. Vorhandener typed Client sendet eine ausdrücklich freigegebene öffentliche Frage mit passender Quelle/Releasebindung; erwartete Zahl/Aussage und Quellenbezug vergleichen, nicht nur HTTP 200 zählen.
3. Falsche Identität, unzulässige Scopes und fehlende Evidenz liefern keine geschützten Inhalte; zulässige Ablehnung nicht als Antworterfolg verkaufen.
4. Geprüfter Provideraufruf nur mit vorhandener Modellfreigabe und eigenem Ressourcenfenster. Die Vorbereitung startet keinen Provider und keinen Testprozess.
5. Integrator prüft vom tatsächlichen Consumer aus, ohne Discord-/Twitch-Nachricht. Consumerumschaltung erst danach; Latenz und Fehlerzustand dokumentieren.

## 5. Rückweg aus bestehendem G5-Vertrag

`infra/cutover/README.md:53-67,79-111` und `RELEASE_MANIFEST.yaml` verlangen:

- Unveränderliches vorheriges Release, das das aktuelle Schema und Datenrelease tatsächlich lesen kann. Ein mutable Checkout oder ein noch vorhandenes altes Binary reicht nicht.
- Backuphash, Restorebericht auf leerem Ziel, Index-/Replikagenerationen, Policy-/ACL-/Tombstonezustand und nachvollziehbare Änderungen nach Umschaltung. Leere Manifestfelder bleiben offen, keine erfundenen Werte eintragen.
- Alte Writer am gemeinsamen Store über Generation/Fencing sperren, laufende Jobs drainen, finale Offsets und Deletes/ACL-Deltas abgleichen. Ein Timerstop oder ein momentan inaktiver Dienst ersetzt das Fencing nicht.
- Bei Rücknahme der Serve-/Consumeraktivierung keine Rücknahme aktueller Sperren, Tombstones oder Daten. Consumer und Provideraktivierung auf den zuvor belegten Zustand zurückstellen; bei fehlendem kompatiblem Rückweg nicht cutovern.
- Der erstmalige Serveanschluss und ein späterer Writerwechsel haben getrennte Wirkungen. Serve allein ist weder abgeschlossener Writerwechsel noch vollständiger G5-Abschluss. Keine alten Jobs für einen bloßen API-Test abschalten.

Replay wird gemäß Nutzerentscheidung außerhalb V1 geführt. Historische Replayfelder/Offets werden nicht als bestanden gefüllt, sondern im späteren tatsächlichen Releasemanifest ausdrücklich als zurückgestellt begründet. Wiki-/Provider-/Datenparitätsnachweise, Fehler der bestehenden Jobs und Python-/Altbetrieb bleiben eigene offene Punkte des Gesamtvertrags.

## 6. Ausführbare nächste Schritte ohne neue Architektur

- Statische Workspace-Testprüfung im bestehenden Sol-Thread abschließen und unabhängig nachvollziehen. Formatprüfung auf der Brainbasis bereits bestanden: `cargo +1.97.1 fmt --all -- --check`, Exit 0, 1307 ms.
- Exakte Testanforderung nach Laufzeitklassen an den Integrator. Kein Cargo-Test/Build/Fetch vor Zuteilung.
- Für die spätere Serveconfig zunächst reale Nicht-Secret-Bindungen aus Betreiber-/Consumerbestand nachweisen. Aus dieser Datei keine betriebsfähige Config oder neue Modellwahl ableiten.
- Release-/Rollbackmanifest erst mit tatsächlich gemessenen Werten ausfüllen. Der Gesamtauftrag bleibt aktiv; weder diese Vorbereitung noch ein grüner Unitlauf ersetzt DB-/Prozess-/Livefälle.
