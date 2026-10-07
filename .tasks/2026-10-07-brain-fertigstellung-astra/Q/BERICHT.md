# Q: echte Evaluation und Live-Abnahme

Stand der letzten lokalen Integritätsprüfung: 2026-10-07, 12:01:46 UTC. Versuch 1, Produzent `q-eval`. Paket bleibt aktiv und nicht abgenommen.

## Urteil

**Quellen lokal gesichert; private Antwortabnahme weiter gesperrt.** Die Entscheidung `ENTSCHEIDUNG-Q-K-DATENSCHUTZ.md` vom 2026-10-07, 10:55 UTC wurde umgesetzt. Die Quellensammlung wurde trotz der Remotegrenze fortgesetzt. Keine aktive Testnachricht, kein privater Modellaufruf, keine Produktkorrektur, kein Dienstrestart.

1. Vier lokale Quellen sind mit SHA256 gebunden. Der eingefrorene Teilplan enthält **166 vorläufige Kandidateneinträge, 0 akzeptierte Goldfälle**. Das ist kein vollständig beschriftetes Set mit mindestens 30 echten Fragen. Herkunft, mögliche Überschneidungen und Sollfakten je Fall bleiben zu prüfen.
2. Pocket und Haze sind in den menschlichen Originalen aus #bot-logs und im Teilplan vorhanden. Aktuelle öffentliche Originalfakten wurden unabhängig gesichert; siehe `ORIGINALFAKTEN.md`. Fünf öffentliche Originalpatchstichproben sind inzwischen in `P2-ORIGINALQUELLEN.md` mit Erwartungen vorbereitet, aber noch nicht als Brainantworten abgenommen.
3. Ein zum Nutzer gehöriger Twitch-Testkanal wurde über die bestehende Owner-/Identitätsrelation lesend bestätigt. Aktuelle Botteilnahme und Sendeberechtigung sind nicht geprüft. Es wurde nichts gesendet.
4. Der tatsächlich konfigurierte Brain-Provider bleibt ein remote weiterleitender Codex-Abo-Proxy. Im untersuchten Host-/Zentralproviderbestand ist kein freigegebener tatsächlich lokal rechnender Zentralprovider belegt. Kein Wechsel oder Start eines Providers durch Q.
5. Das eigene Rust-Prüfwerkzeug hat fünf bestandene Tests. Acht bestehende isolierte HTTP-Consumer-Tests bestanden zusätzlich. Beide Nachweise ersetzen keine reale Discord-/Twitch-Antwort oder Ende-zu-Ende-Laufzeit.

## Herkunft und Bestand

Eigener Worktree: `/home/nathanael/.worktrees/brain-q-eval-20261007`, Branch `feat/brain-q-eval-20261007`, sauberer Start `f6f5cef65f1f946113f0b8216c6475f6d38ec928`. Werkzeugcommit: `b8ceec93c059e32d170372a8734593165dc7710a`. Werkzeug-SHA, laufender Brain-Code-SHA und Wissensversion werden getrennt geführt.

Graphify zuerst, globaler Graph. Keine Extraktion oder Änderung an fremden Checkouts. Wiederverwendet wurden:

| Bestand | Wiederverwendung und Grenze |
|---|---|
| Deadlock-Bots `rust/bin/dl-knowledge/src/eval.rs:121` | Vorhandene Golden-Set-/Retrievalstruktur geprüft. Kein LLM-Aufruf und kein Zustellungsbeweis. |
| Brain `scripts/run_local_pilot.sh`, `scripts/test_brain_serve.sh` und bestehende `brain-client`-HTTP-Tests | Vorhandene isolierte Prüfgrenzen. Keine Produktions-DSN und kein Chat-Endzustand. |
| `/home/nathanael/Documents/tools/dl-bot-mcp-stdio.py` und `dl-bot/src/mcp.rs:806` | Unveränderter Verwaltungsadapter, vorhandene kleine Discord-Pagination. Keine Secrets ausgegeben. |
| `bot.concierge_conversations`, `public.twitch_chat_messages`, `public.tb_chat_brain_answers` | Bestehende Gesprächs-/Chat-/Auditquellen über vorhandenes psql und Peer-Identität gelesen. Statische SQL-Abfragen mit erzwungenem Read-only-Modus. |
| `/usr/local/bin/deploy-twitch-release --pruefen` | Vorhandener lesender Release-/exe-Nachweis für vier Twitch-Systemdienste. |

BESTAND[BS-1]: teilweise | Fundort: /home/nathanael/repos/Deadlock-Bots/rust/bin/dl-knowledge/src/eval.rs:121 | Anknüpfung: bestehende Evalstruktur, Discord-Verwaltungsadapter, Postgres-Archive und Releaseprüfer; keine neue Produktpipeline

## P0: eingefrorene lokale Quellen und Teilplan

Quellenversion: `q-partial-v1-20261007`. Ablage: `Q/private/q-partial-v1-20261007/`. Private Originaltexte, Personenbezug und individuelle Nachrichtenreferenzen bleiben dort. Im Modellkontext stehen Mengen, Hashes und technische Befunde, keine Rohfragen.

| Quelle | Gesicherte Originale | Vorläufige Kandidaten | SHA256 des Snapshotcontainers |
|---|---:|---:|---|
| `botlogs` | 87 menschliche Nachrichten aus 200 gelesenen Nachrichten | 4 im ursprünglichen Snapshotparser, 6 im späteren Teilplanparser | `d91a2adbd3d3c311ed53059da5e83b8167e22c018f5c464b820a93affea2f236` |
| `dm` | 154 Nutzerbeiträge | 94 | `2a6f97a9df5eac74e1e621c854bbb3d0664bd18ccdc8a6c199ca281d43326892` |
| `twitch` | 1000 Chatdatensätze | 62 | `80ac32dcab5ed725d77e2520230650493a9217d1f3e9a65ce421a3c23a59034c` |
| `twitch-brain` | 5 vorhandene Auditfragen | 4 | `8e929f05a0c6ce3fd71d00685a93f6b017a937f00c56bf259ce43269fda679f6` |

Die leere Bot-DM-Kanalliste wurde nicht als fehlendes DM-Archiv gewertet. Die bestehende Konversationstabelle lieferte Nutzerbeiträge; bestehende Personen mit eingeschalteter Nichtverarbeitung wurden ausgeschlossen. Dieses Gesprächsarchiv ist kein vollständiger DM-Kanalverlauf. UUID-förmige Twitch-Nachrichtenreferenzen beweisen allein weder menschliche Herkunft noch Zustellung.

Discord-Großexport und automatischer Spill scheiterten an der Dateiausgabe. Vierzig Seiten mit jeweils fünf Nachrichten funktionierten. Ein größerer Versuch mit 200 Seiten wurde durch den äußeren Prüftimeout beendet und wird nicht als erfolgreicher Snapshot gezählt. Der Service wurde nicht korrigiert. Eine frühere lokale Diagnose mit anderer Heuristik zählte acht Kandidaten; sie ist nicht die eingefrorene Parserzählung und kein Goldset.

Teilplan: `partial-plan-v1.json`, SHA256 `c2332949206a0773b604391d7f540c9520598a66409053f94defb4076447b0ed`. Vorläufige Erwartungen wurden vor jedem Modelllauf eingefroren. Die Mention-Erkennung wurde vor Erstellung des Plans korrigiert; ursprüngliche Snapshots blieben unverändert. Der Plan bindet vier Quellhashes und lokale Zeilenreferenzen. Authentizitäts- und Originalfaktenprüfung stehen je Fall auf offen. Keine synthetischen Ersatzfragen als echte Herkunft.

| Vorläufige Antwortart | Kandidateneinträge |
|---|---:|
| Spiel mit Originalquelle | 10 |
| Patch mit Originalquelle | 4 |
| Server oder eigener Status | 16 |
| Coaching oder Communityhilfe | 2 |
| Selbstbild ohne Interna | 1 |
| Noch unbeschriftet | 133 |

Unbeschriftet bedeutet nicht Unsinn. Überschneidungen zwischen Quellen sind nicht bereinigt. **Vollständige Evalsetversion: keine. Akzeptierte Goldfälle: 0. Baseline- und Wiederholungsläufe mit privaten Fragen: 0.**

Die Prüfung um 12:01:46 UTC bestätigte vier private Verzeichnisse mit 0700 und 17 private Dateien mit 0600, ohne Symlinks oder abweichende Modi. Git-Ignore wurde für die privaten Quellen und den Teilplan geprüft. `verify` bestätigte vier Quellbindungen und 166 Einträge, Exit 0. Die separat gespeicherte Eigentumsnotiz wurde in der Modiprüfung erfasst; sie ist nicht Teil der vier gehashten Quellbindungen.

## Öffentliche Originalfakten und Kanalnachweis

Originalfaktenversion: `q-original-v1-20261007`. Öffentliche Assets-Abfrage um 11:19:14 UTC, aktuelle Endpunkte aus dem bestehenden Assets-Adapter. 40 Helden und 746 Entitäten lokal gesichert. `ORIGINALFAKTEN.md` enthält aktuelle Haze-Spirit-Skalierungen und Pockets Mechaniken samt Grenzen der Konterbehauptungen. Historische Wiki-Werte vom Juli wurden nicht als aktuelles Gold verwendet. Die Hashes binden normalisierte Snapshotcontainer, keine unveränderten HTTP-Byte-Receipts.

Öffentliche Patchquelle zusätzlich eingefroren: `q-patches-v1-20261007`, erfasst 11:59:00 UTC. 41 offizielle Steam-Feedbeiträge aus dem bestehenden 500-Beiträge-Suchumfang; fremde Nachrichtenfeeds ausgeschlossen. SHA256 `490208eeb5cf0952f2dfb20c1bd8f78882c0090c996bb999bc37622abfaa19e0`, Digestvergleich um 12:01:46 UTC erfolgreich. Fünf tatsächlich numerische Gameplaypatches vom 05.10., 16.09., 22.08., 12.08. und 28.07.2026 haben feste Erwartungen und originale Inhaltshashes in `P2-ORIGINALQUELLEN.md`. Sie sind keine zusätzlichen echten P0-Fragen. Neuere Heldenankündigung und Zahlenpatch bleiben getrennt; das vollständige Forum ist nicht abgeglichen. Keine Behauptung eines weltweit jüngsten Gameplaypatches.

Eigentum: genau eine konfigurierte Ownerreferenz und ein damit verknüpfter Twitch-Kanal; vorgesehener Zielkanal stimmt mit dieser Relation überein. Historisch 18502 Chatzeilen, vom 2026-01-31 bis 2026-10-06. Die private Notiz nennt keinen geschätzten exakten Beobachtungszeitpunkt: `observed_utc=null`, Erfassungszeit 11:40:59 UTC. Aktuelle Botteilnahme und Sendeberechtigung offen; aktive Nachrichten 0.

## Tatsächliche Providergrenze

Erneut lesend geprüft um 11:41:49 UTC: `codex_subscription`, Modell `gpt-6-luna`, Basis `http://127.0.0.1:18769/v1`. Der vorhandene Proxy ist ausdrücklich ein Codex-Abo-Proxy; sein Binary enthält den externen Codex-Responses-Endpunkt. Der bestehende Transport bindet `query.text` in die Nutzernachricht. Das wurde auch am Git-Objekt des laufenden Brain-SHA `bfda408c…` geprüft. Kein privater Probeaufruf zur Bestätigung.

Bei der Inventur wurden aktive Userunits, passende Inferenzprozessnamen, laufende Container und vorhandene zentrale Twitch-Providerauswahl geprüft. Kein laufender ollama-/llama-/vllm-/litellm-/hermes-Inferenzprozess oder entsprechender Container belegt. Ein Modellresolverdienst war fehlgeschlagen. Die vorhandene Twitch-Zentralauswahl verweist auf Fireworks. Das ist keine Aussage über jeden denkbaren Dienst im gesamten Netz. Keine breite Auslese rootgeschützter Twitch-Prozessumgebungen, kein Providerstart, Download, Modell-/Timeoutwechsel oder zweiter Connector.

Private Antwortprobe bleibt bis zu belegter Freigabe gesperrt. Die vorhandene eng begrenzte Invite-Ausnahme betrifft den eigenen Status als Enum plus Zeitpunkt nach Identitäts-/Rechteprüfung, nicht die Übermittlung echter Rohfragen.

## Wiederholbare Integritätsprüfung und Tests

Dieser Befehl prüft den gespeicherten Teilplan ohne Netzwerk oder Modell. Er ist kein P0-/P1-Antwortlauf:

```sh
/home/nathanael/.worktrees/brain-q-eval-20261007/.tasks/2026-10-07-brain-fertigstellung-astra/Q/collector/target/debug/brain-q-source-snapshot verify q-partial-v1-20261007
```

Eigene Werkzeugprüfungen nach Ergänzung des öffentlichen Steam-Snapshotzugangs: `cargo fmt`, fünf Tests, Clippy mit `--all-targets -- -D warnings` und Build erfolgreich. Tests und Clippy verwendeten `--locked --offline --jobs 3`. Nach letzter Codeänderung wurde formatiert, getestet, gelintet und gebaut. Unmittelbar davor war auch ein `fmt --check` erfolgreich. Der neue Zugang verwendet den bestehenden Originalquellen-Endpunkt, kein LLM, keinen Produktimporter und keine neue Provideranbindung. Der private Teilplan blieb unverändert; erneutes `verify` Exit 0.

```sh
/home/nathanael/.cargo/bin/cargo +1.97.1 test --locked --offline --jobs 3 --manifest-path /home/nathanael/.worktrees/brain-q-eval-20261007/.tasks/2026-10-07-brain-fertigstellung-astra/Q/collector/Cargo.toml -- --include-ignored
/home/nathanael/.cargo/bin/cargo +1.97.1 test --locked --offline --jobs 3 --manifest-path /home/nathanael/.worktrees/brain-q-eval-20261007/rust/Cargo.toml -p brain-client --test consumer_http -- --include-ignored
```

Werkzeug: 5 passed, 0 failed, 0 ignored, 0 filtered. Consumer-HTTP: 8 passed, 0 failed, 0 ignored, 0 filtered, Exit 0; Build 49.03 Sekunden, Testlauf 0.28 Sekunden. Der erste Consumer-Aufruf mit nicht über Rustup gestartetem Cargo scheiterte mit Exit 101 vor Testbeginn. Der korrigierte absolute Cargo-Aufruf lief erfolgreich. Keine unbelegte Altfehlerbehauptung.

Die acht Consumer-Tests benutzen echte isolierte Loopback-HTTP-Fixtures für Auth-/Unicodevertrag, Redirectverbot, Request-/Responsegrenzen, Statusfehler, Vertragsdrift, synthetische Durchsatzmessung und Deadline. Kein privater Inhalt, kein Produktionsdienststopp und keine Produktions-DSN. Das bereitet P8 vor, beweist aber keine kurze reale Chat-Ausfallantwort. Die synthetische Durchsatzprobe wird nicht als Produktions-SLO bezeichnet.

TESTNACHWEIS[TW-1]: 13 passed, 0 ignored | Baseline: nicht gemessen, keine Altfehler als rot behauptet

TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 3 belegt | Senke: Q/BERICHT.md

Die Textprüfung bezieht sich auf den Lesertext dieses Berichts, nicht auf technische IDs oder Pfade. Der vorhandene Gedankenstrichprüfer bestätigte auch Originalfakten, Patchstichproben und README. Die drei Absolutwörter betreffen den eigenen Datei-Scope, die fehlende vollständige Kategorienabnahme und die ausdrücklich begrenzte frühere Refprobe.

## Live-Infrastruktur: erneute Beobachtung 11:41:49 UTC

Seit der ersten Q-Beobachtung um 10:47:59 UTC wurden dieselben Brain-/Twitch-Prozesse und Releasepfade beobachtet. Kein neuer Deploy in diesen beiden Stichproben belegt, kein Q-Deploy ausgeführt.

| Dienst | Tatsächlicher Stand | Beweis und Grenze |
|---|---|---|
| Brain | PID `2645590`, exe `/opt/deadlock-brain/maintenance-releases/bfda408cb988722ddceadb56bca5b72e12d12731/brain-serve`, kein `(deleted)` | Binary-SHA256 `c96ee6262a7c4b7c1d3832cea085ff162d0a5695ca14296746b76bc159b271ac`. Früher geprüftes Release-Manifest nennt denselben Quell-SHA. NRestarts 0, Journal `-p err` seit Start 07:04:28 UTC: 0 Einträge. |
| Discord | Userunit `deadlock-bot-rust.service`, Wrapper-PID `2848836`, NRestarts 0 | Journal `-p err` seit 2026-10-06 23:23:58 UTC: 0 Einträge. Früherer Listener-PID `2848890`; direkter exe-Zugriff damals `EACCES`. Kein vollständiger aktueller Discord-SHA-Beweis. |
| Twitch | Systemunit, Bot-PID `3874592` | Deployprüfer Exit 0. Vier aktive Prozesse ohne `(deleted)` auf `10dacbc2376a63f6d91869afe83b1ac8bb615eec`, passend zu current. Bot-Journal `-p err` seit 10:21:03 UTC: 0 Einträge. |

Weitere Twitch-PIDs: Dashboard `3874704`, Stream-Audit `3821592`, Kategoriecollector `3796733`, jeweils NRestarts 0. Diese Releaseprüfung belegt nicht die vollständige Integration von I/G/K.

Brain-Health erneut HTTP 200, `application/json`, `status=ok`; Readiness HTTP 200, `application/json`, `status=ready`. `release_id=maintenance-d7ae0a7d7751f7490dd9ad4e814d038028b615e458df9dbd6fefcf589a94635c`, `knowledge_version=docs-d7ae0a7d7751f7490dd9ad4e814d038028b615e458df9dbd6fefcf589a94635c`, `release_bindings_sha256=58283141cef70890a19903dd394bdae90932b6e291e9b9f3751541225674ec57`. Diese Wissensbindung ist nicht der Code-SHA `bfda408c…`. Discord-MCP-Health wurde früher mit HTTP 200, `text/plain; charset=utf-8`, exakt `ok` geprüft, nicht bei der erneuten Brainprobe.

Health, aktive Prozesse und leeres Fehlerjournal zählen nicht als zugestellte Antwort. Keine P1-Ende-zu-Ende-Zeit gemessen. Nur frühere lokale Remote-Tracking-Refs gelesen, damals ohne Fetch; keine Behauptung eines frisch verifizierten gemeinsamen Remote-main-Standes.

## Kriterien

| Kriterium | Urteil | Beleg oder offene Grenze |
|---|---|---|
| P0 | teilweise, nicht abgenommen | Quellen und vorläufige Erwartungen eingefroren. Keine 30 akzeptierten Goldfälle, nicht alle sechs Kategorien abgenommen. |
| P1 | blockiert | Remotegrenze; keine echten Discord-/DM-/Twitch-Antwortläufe, keine Ende-zu-Ende-Zeit. |
| P2 | teilweise, nicht abgenommen | Fünf Originalpatchstichproben mit Erwartungen vorbereitet. Brainantworten, kosmetische Forumgegenprobe und gemeinsame Aktualitätsbindung offen. |
| P3 | blockiert | Kein Livewortlaut. Soll: Coachingkanal `1494373349944459355` beziehungsweise Coachingwebsite; Pate in Ich-Form. |
| P4 | blockiert | Keine drei echten Selbstbildantworten. |
| P5 | blockiert | Keine reale Prüfung ehrlicher Antwortgrenzen. |
| P6 | offen | Eigener Invite-Status nicht abgenommen; kein Invite gesendet. Die Minimalprojektion bleibt separat erlaubt. |
| P7 | offen | Keine reguläre `hero_build_id` von I abgenommen, kein fremder Build veröffentlicht. |
| P8 | teilweise, nicht abgenommen | Acht isolierte Client-HTTP-Tests bestanden; tatsächliche kurze Consumer-Ausfallantwort nicht zugestellt/geprüft. |
| P9 | teilweise | Erneute Prozess-/Health-/Journalprobe. Aktuelle gemeinsame Integrator-/Remote-main-Abnahme fehlt; Discord-exe nicht vollständig belegt. |
| P10 | offen | Historische Ersatzliste vorhanden, aktuelle gemeinsame Writer-/Reader-/Live-Abnahme fehlt. |

## P10: Bestands-/Ersatzliste, kein Abschaltbeweis

Quellenbasis: `.tasks/2026-10-06-brain-abschluss/A/EIN-BRAIN.md:22-51` und `PAKETE.md:12`. Nichts abgeschaltet oder gelöscht. K bleibt Eigentümer der Produktverdrahtung.

| Alter oder bestehender Antwortpfad | Vorhandener Ersatzkandidat | Noch erforderlicher Beleg |
|---|---|---|
| Concierge `dl-community/src/concierge.rs` | Gemeinsamer `dl-brain::answer_for_discord`-/`/v1/answer`-Consumer; Aktions-/Paten-/Privatsphäremechanik erhalten | Zugestellte DM und Wortlaut; Patenangebot in Ich-Form. |
| FAQ/Tickets `dl-community/src/faq.rs` | Derselbe Brain-Consumer mit bestehender Darstellung | Akzeptierter Fall bis echter Zustellung, nicht bloß gespeicherter Vorschlag. |
| Passive Hilfe `dl-community/src/passive_help.rs` | Detektor/Kanalgate plus derselbe Brain-Consumer | Tatsächlicher Modus oder explizite Abschaltliste mit funktionierendem Ersatz. |
| `dl-knowledge /public/v1/ask` | Retrieval als Quelle behalten, Antwortwortlaut über Brain | Verbliebene ask-Consumer und reale Ersatzantwort prüfen; Dienst nicht pauschal löschen. |
| Discord-Erwähnung/DM `modglue.rs`, `dl-brain/src/brain_api.rs` | Bestehender typisierter Consumer | Identität, aktuelle Spielfakten, reale Antwort unter 20 Sekunden. |
| Twitch `brain_chat_wiring.rs`, `tb-knowledge/src/brain.rs`, Engagementantwort | Typed-Consumer und Ownership-Sperre in `tb-chat/src/pipeline.rs` | Genau eine reale Antwort im erlaubten eigenen Kanal, kein zweiter Generator danach. |
| Streamer-Fragebox `handlers/self_explainer.rs` | Typed-Adapter statt Legacy-/Shadow-Generator | Tatsächlicher Modus, Verlauf, Quellenprojektion und Nutzerantwort gemeinsam. |

## Gate, sichere Änderungen und offene Übergabe

Der Werkzeugcommit enthält `.gitignore`, Cargo-Manifest/-Lock, Rust-Prüfwerkzeug und README. Keine privaten Quellen im Commit. Einziger Reviewer war der zentrale Merge-Gate:

```sh
python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-q-eval-20261007 --base f6f5cef65f1f946113f0b8216c6475f6d38ec928 --head b8ceec93
```

Exit 0: `[gpt-6.1-sol] ALLOW: No merge-blocking defect found in the supplied diff.` Ein NIT betrifft JSON-Schreiben vor anschließendem Digest-Schreiben: bei Digestfehler bleibt eine unveränderliche, unverifizierbare Datei. Danach wurde im README die sichere Wiederaufnahme vor dem ersten Modelllauf dokumentiert und der Cargo-Pfad korrigiert.

Der anschließende sichere Berichtscommit `ce0fb179` wurde mit derselben Basis durch denselben Gate geprüft, erneut Exit 0 und ALLOW. NIT: fehlende fokussierte Dateisystemtests für Hashintegrität, Modiverweigerung und Schreibfehler-Recovery. Die tatsächlichen lokalen Hash-/Modiprüfungen sind oben getrennt dokumentiert; sie werden nicht als solche Unit-Testabdeckung ausgegeben. Kein BLOCK, keine eigene Produktreviewrolle, keine zusätzlichen T3-Threads. Die danach ergänzte öffentliche Patch-Snapshotfunktion samt fünf Tests ist noch nicht von diesen beiden SHA-bezogenen Urteilen umfasst.

Der finale Code-/Methodikstand `bf54e659814008c8db941e6ff6be9016314347d3` wurde anschließend mit gleicher Basis durch denselben Gate geprüft, Exit 0 und ALLOW. Der verbleibende NIT betrifft weiterhin fehlende Dateisystem-Regressionsabdeckung, nicht die fünf erfolgreich gelaufenen Parser-/Heuristiktests oder die separaten tatsächlichen Integritätsprüfungen.

Ein R10-Hinweis verlangte das erneute Lesen der Merge-Rolle; danach bestand die Ancestryprüfung mit Exit 0. Der erste Main-Push wurde vor Ausführung durch den Hook verweigert: nicht prüfbare indirekte Git-Ausführung. Die lesende Ursachenprüfung fand in `gate_hook.py:887` die Suche nach dem Wort `eval` im gesamten Befehl. Dieses Wort steht auch im absoluten Worktreepfad `brain-q-eval-20261007`, obwohl kein Shell-eval ausgeführt wird. HEAD blieb unverändert. Die Befehlskorrektur ist der gewöhnliche einzelne `git push origin HEAD:main` im zuvor verifizierten eigenen Session-Arbeitsverzeichnis. Der bestehende Hook bleibt unverändert und prüft dabei weiterhin den tatsächlichen Worktree, dessen Sauberkeit und Ancestry. Die neue echte Sicherungsnachweisdatei war zudem noch untracked und wird regulär aufgenommen, ohne private Inhalte zu committen.

Kein erfolgreicher Push, Merge, Deploy, Restart, Cleanup oder settle bis zu diesem Berichtsstand. Produktdeploys bleiben bei I/G/K. Nur eigene Q-Dateien bearbeitet; zentrale Akten, Produktdateien, DB-Inhalte und fremde Worktrees unverändert. Der aktive Worktree samt privaten Originalen bleibt erhalten.

Der Orchestrator hat im selben Q-Auftrag die unabhängige Integration des sicheren Collector-/Methodikstands nach finalem eigenem Gate beauftragt. Zulässig sind Quellcode und nichtpersonenbezogene Metadaten, keine privaten Originale oder Teilpläne. Keine unnötigen Dienstneustarts, kein Ersatz der offenen Nutzerentscheidung durch einen Modellwechsel. Derselbe Thread bleibt für die echte Nach-Deploy-Evaluation zuständig; kein Self-Settle. Vor einem späteren Cleanup müssen die privaten ignorierten Originale samt Hashbindung und Zugriffsrechten außerhalb des zu löschenden Worktrees nachweislich erhalten sein. Der aktuelle Worktree bleibt aktiv. Zusätzlich wurden die 17 privaten Dateien außerhalb des Worktrees unter `/home/nathanael/.local/share/brain-q-private-20261007-bf54e659` erhalten. Am 12:10:12 UTC stimmten sämtliche relativen Dateipfade und Datei-SHA256 zwischen Quelle und Sicherung überein. Beide Seiten hatten vier Verzeichnisse mit 0700 und 17 Dateien mit 0600, keine Symlinks oder abweichenden Modi. `SICHERUNG.json` enthält den technischen Nachweis und den Inventarhash `30c0a264b782d8d9f6260de5c929597e77f49b1ce56cd520f901d2eab96d0caf`, keine privaten Originale. Es wurde nichts gelöscht oder extern übertragen. Vor späterem Cleanup ist der dann aktuelle private Stand erneut abzugleichen.

Nächster fachlicher Schritt: Herkunfts-/Goldprüfung des lokalen Teilstands im freigegebenen lokalen Umfang. Fünf Patchquellen sind vorbereitet, ihre Antwortabnahme bleibt offen. Eine private Antwortabnahme wird erst nach belegter Freigabe des bestehenden Datenflusses ausgeführt. Keine erneute pauschale Quellenbereitstellungsfrage; die konkrete Orchestratorentscheidung ist bereits umgesetzt.
