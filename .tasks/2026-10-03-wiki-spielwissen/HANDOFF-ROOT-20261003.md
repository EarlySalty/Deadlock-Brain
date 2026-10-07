# Handoff: Wiki, Spieldateien und Deadlock-Brain

Stand: 03.10.2026, 18:42 UTC. Nutzerauftrag weiterhin offen. Dieses Handoff ersetzt die historischen Fortschrittsabschnitte in ORCHESTRATOR-KURZ.md als Einstieg. Keine Dienste, Compiler oder Sitzungen wurden für die Übergabe gestoppt.

## Ziel und fehlender Gesamtabschluss

Das regulär zugängliche Deadlock-Wiki mit Originalen, Revisionen, Herkunft, Rechten und ehrlicher Abdeckung in Deadlock-Brain übernehmen. Über den vorhandenen berechtigten Steam-Zugang echte Spieldateien herunterladen, prüfen und mit dem bestehenden Parser auswerten. Daten importieren, ihre Nutzung im Reader nachweisen, gemeinsam abnehmen, mergen, pushen, deployen, neu starten und live prüfen. Anschließend eigene Branches und Worktrees gesichert entfernen und Abschlussbericht liefern.

Bisher sind für unseren Wiki-Auftrag kein produktiver Import, echter Depotdownload, eigener Main-Merge, Deployment oder neuer Liveabschluss belegt. D meldet null Spieldateien. As lokale Datenabnahme und Bs Parserabnahme ersetzen diese Nachweise nicht.

## Erste Schritte der Übernahme

1. Dieses Handoff, AN_BEREICHE.md Punkte 55/56, C3-WEBSITE-EXPORTVERTRAG.md und die aktuellen kurzen Bereichsübergaben lesen. TODO.md wird ausschließlich von S2 gepflegt. Keine Rohworkerlogs in den Hauptkontext ziehen.
2. Beim vorhandenen externen Auftraggeber den tatsächlichen Launcherabschluss und die anschließende Abstimmung zu Relay und Streamstatistik anhand aktueller Akten klären. Vorher keine neue eigene Prüf- oder Releaseschleife einreihen. Keine Coaching-Wartebedingung mehr.
3. Bereits bestehende eigene Sitzungen und Kinder tatsächlich prüfen. Alle vier eigenen Eltern wurden beim Metadatenabgleich dieser Übergabe durch `claude agents --json` als `idle` gemeldet. Das ist kein Sessionende oder globaler Kinder-/FD-Freiheitsbeweis. Nicht parallel kopieren, nicht blind mit resume duplizieren.
4. Nach erfüllter Fortsetzungsbedingung C3s engen Clippy-Fix und As Eigenanteil integrieren; D führt seinen tatsächlichen Endlauf und die unabhängige Abnahme zu Ende. Zusammengehörige D/B/C-Schnittstellen gemeinsam prüfen.
5. Steam-Cutover mit dem bestehenden fremden Abschlussverbund koordinieren. Erst nach gemeinsamer Abnahme, aktuellem Gate, Merge und Push deployen. Anschließend echter Download und bei tatsächlichem Bedarf frischer B3-Kontext zur Extraktion.
6. Vollständige Brain-Import-/PG-/Reader-/Wiederholungsbeweise und öffentliche Websitebindung nach den unten genannten Verträgen führen. Erst danach endgültige Gesamtfreigabe und Deployment. Cleanup bleibt letzter Schritt.

## Zuständigkeiten und Sitzungen

Wiki-Hauptzuordnung laut Bereichsbericht: T3 `a02207b7-fd5d-485e-882f-941c1f985f94`. Root orchestriert und schreibt REGISTER.md, AN_BEREICHE.md und diese Übergabe. Bauen und Datensammeln bleiben bei den Bereichen. Nur eigene Sitzungen verwalten.

| Bereich | Native Sitzung | Aktueller Stand | Nachgewiesene PID bei Statusprobe |
| --- | --- | --- | --- |
| A, Wiki | f01cce67-209b-468e-8abb-ec2070beeaa2 | Übergabe unabhängig abgenommen; idle; keine neue Facharbeit nötig | 2217564 |
| B2, Parser | 8e61edb7-3a14-4274-84bf-569122a7241c | Regulär beendet um 13:52 UTC; nicht wieder aufnehmen | Historisch, nicht aktuell |
| C3, Integration und Deployment | f83cb4c1-e8c2-4f44-8e83-ada61c2b246a | idle; Prüfung beendet, Clippy rot; Folge zurückgestellt | 2543115 |
| D, Steam-Downloader | cdbe72ce-ea3f-459f-ad8a-02f9e0436158 | idle; enger Altprozessabgleich beendet, neuer Endlauf zurückgestellt | 2217548 |
| S2, Aufgabenstand | 0dfa154f-fd94-4f68-bed5-b8f34558218b | idle; alleiniger TODO-/STATUSKONFLIKTE-Schreiber | 2230284 |

Werkzeugverbindungen zuletzt: C3 exec45216 im Vordergrund; D attach exec60929; S2 attach exec46310; A attach exec51369. Diese Kennungen können nach einem Turn ungültig werden. Fehlende Werkzeugkennung bedeutet keinen Prozess- oder Jobexit. Native Sitzungsmetadaten erneut prüfen.

Normale Nachrichten mit bracketed paste und anschließend separatem Enter zustellen. Kein Ctrl+X/Ctrl+S: Das hat früher einen Elternwerkzeugaufruf unterbrochen. Ctrl+Z ausschließlich zum Trennen einer attach-Verbindung, nicht im Vordergrund. Keine lebende Prüfung, keinen fremden Compiler oder Prozess unterbrechen. C1, C2, B1 und S1 bleiben beendet.

Modelle ausschließlich GPT 6.1 Sol. Unsere Eltern high; Kinder maximal high oder medium. xhigh ist nur die ausdrücklich erlaubte Elternausnahme bei Problemen mit UltraCode. Kein max, anderer Anbieter oder ungefragter Botmodellwechsel.

## A: fertig übergeben, noch nicht integriert

Worktree `/home/nathanael/.worktrees/brain-wiki-spielwissen-a`, Branch `feat/brain-wiki-spielwissen-a`. Frisch gelesener HEAD und Eigencommit:

`116f643aac1752fcc419a6c32537ed050d3173d2`

Parent `2734c2da4e814ff79953e8e825275b0216a6af16`. Ausschließlich vier neue Rust-Dateien: `rust/crates/dbrain-sources/src/wiki_inventory.rs` und `wiki_inventory/{normalize,storage,tests}.rs`. Keine gemeinsame Registrierung, Cargo-, Core- oder Schemaänderung. Nur diesen Eigenanteil übernehmen, keine alte Gesamtbasis mergen. C3 hat den Commit gelesen, aber noch nicht übernommen.

Unabhängige Abnahme: fertig J für die lokale eingefrorene Daten-/Modulübergabe, Fix nötig N. 47 Tests sowie eigene Formatierung, Clippy und Debugbau belegt. Alle 35 Endartefakthashes nach Commit erneut bestätigt. Gesamtformat besitzt einen unveränderten fremden util.rs-Befund; daraus keine neue eigene Änderung ableiten.

Korrigierte Importquelle ausschließlich:

`/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized-fix6/`

38.273 Dokumentversionen, 1.109.153 Fakten, davon 130.107 JSON-Zahlen. Sechs vollständige Wiederholungen bytegleich; Originaltexte exakt; 19 Vertrags-/Typenfehlerzähler null. 851.127.133 JSONL-Bytes. Größte Zeile 4.762.643 Bytes ohne LF. Exakt 282 Zahlenkorrekturen über 15 Revisionen gegenüber dem alten Bestand. Alter `a/normalized/`-Bestand bleibt wegen Zahlenfehlern gesperrt und erhalten.

Historische Archive und zwölf Captures vom 02.05.2026 sind keine heutige Vollabdeckung. Alle Complete-Flags bleiben false. Heutige reguläre Wiki-Proben erhielten 403 und wurden gestoppt, keine Umgehung. Rechte bleiben unverified/redistribution_allowed=false; interne Verarbeitung erteilt keine öffentliche Freigabe.

Zentral gesichert: UEBERGABE.md, DATENLAUF-FIX6.md, REVIEW-LOCAL-8.md, FEATURE-VERTRAG-FIX6.md, COMMIT-PROTOKOLL.md und endbeleg-fix6-pruefartefakte.sha256 unter bereiche/a. Der unversionierte lokale Harness und sämtliche weiteren Nachweisartefakte müssen vor Worktree-Cleanup noch vollständig gesichert werden.

Offen bei C3: Registrierung, gemeinsame Features, echter Import und präzise Zahlen-/Typenübergänge durch Serialize/Deserialize, PostgreSQL und Reader. A-Harness serde_json 1.0.151 bleibt eingefroren; C3 produktiv 1.0.150 mit arbitrary_precision. Keine Versionsangleichung als Ersatz für den tatsächlichen Übergangsbeweis.

## B: Parser übergeben, echte Depotextraktion fehlt

Worktree `/home/nathanael/.worktrees/brain-wiki-spielwissen-b`, Branch `feat/brain-wiki-spielwissen-b`. HEAD `0aa0d9ee2660e9c8ef6fb04a4295c842d1047bb6`.

Nur eigene Commits `48b6ce1cf277ec4de4b47ac4a0898fbc223fe6b5` und `0aa0d9ee2660e9c8ef6fb04a4295c842d1047bb6`. Bereits durch C3 als `7168574` und `f7a03f9` übernommen. Nicht nochmals implementieren oder den historischen Branch mit fremden Basiscommits mergen.

Vorhandene Git-Daten: 660 Dokumente und 676.363 Fakten, insgesamt 506.412.456 JSONL-Bytes. Größte Zeile 117.157.259 Bytes ohne LF. 121.556 numerische Werte ohne mathematischen Verlust belegt. 68.871 Werte sind absichtlich Zahlenstrings mit Originallexemen und Qualifiern; 52.685 i64-Number. Typvertrag gemäß Punkt 52 erhalten, keine automatische Number-Konvertierung und kein Floatzwischenpfad. Git-Spieldaten sind keine echte Steam-Build-/Depotabdeckung, keine vollständige Binärextraktion.

Handle-Vertrag: Originaldatei an gehaltenen FD binden, streamende SHA256-Prüfung; physische Ressource und Containerherkunft getrennt. C3 nutzt private unverlinkte schreibgeschützte Eingaben, verwirft Teiloutput bei Fehler und prüft D-Inventar-/Dateibindung beim Übergang. Keine zweite Parserimplementierung.

Offen: gemeinsame produktive Feature-/PG-/Readerabnahme und echte Depotextraktion nach geprüftem D-Download. Erst bei diesem Bedarf B3 mit frischem Kontext auf vorhandenen Artefakten beginnen; die aktuell geltende Grenze gegen neue Threads zuvor beachten. B2 nicht reaktivieren. Belege unter bereiche/b, round3-proof/run-CrJzHxm4 und punkt47-zahlenwerte-20261003-solhigh im lokalen Datenroot erhalten.

## C3: integrierter Stand und nächster Fix

Produktiver Tree `/home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration`, Branch `feat/brain-wiki-spielwissen-c-integration`, frisch gelesener HEAD `f7a03f9aa4c62f0a05814de487cd8ec92b0cb6ea`. Basis `511a347b653beba13c2bf130f4bead7a7196cc2a`. Uncommittierte Integration erhalten. Koordination und Berichte liegen im getrennten Tree `/home/nathanael/.worktrees/brain-wiki-spielwissen-c/.tasks/2026-10-03-wiki-spielwissen/bereiche/c/`.

Letzter realer Prüfer bxzed33l5, Wrapper 2401768, regulär beendet: C3_EXIT=101 und LOCK_FDS_CLOSED. Alle 16 Formatierungen und 16 Formatchecks Exit 0. Sechs-Pakete/all-targets-Check Exit 0. Strenges Clippy Exit 101: unbenutztes BTreeSet in brain-knowledge-import.rs:14, Digest/Sha256 in Zeile 11. Fixrichtung gesichert, nicht umgesetzt. CLI-Owner a48b3e3dfac497915 abgeschlossen, keine lebenden eigenen Kinder laut tatsächlichem Endbeleg. Keine PG gestartet, keine Tests oder Echtdatenprüfungen dieses Laufs ausgeführt.

389 Dateibindungen im zulässigen Freeze erhalten. Endliste-SHA `eee298261b076a013d9e01811c4fe55c332482d96130e3970da07ce80fbd860d`. Vollbelege C3-CLI.md und `/tmp/brain-c3-cli-check.trDu7q/` erhalten. Frühere ps -g-Auswahl war fehlerhaft und kein Prozessgruppenbeweis; der tatsächliche spätere Endbeleg ersetzt diesen alten Befund. Frühere Fixcommit-ALLOWs sind kein Gesamt-Allow.

Nach Fortsetzungsfreigabe offen:

- Enge bestätigte Import-/Clippy-Funde beheben und A-Eigenanteil sicher integrieren. Geteilte Manifeste und Crateroots ausschließlich bei C3.
- Endgültigen Quellenstand einfrieren, passende Format-/Check-/Clippy-/Testprüfungen unter beiden Hostlocks. Bestehende unabhängige Readeränderungen erhalten; keine fremden Fixturepfade reparieren.
- Isolierte PG- und vorhandene Reader-PG-Tests tatsächlich führen, Testserver ohne geerbte Locks starten und regulär beenden.
- Echte Großzeilen und gesamten A/B-Bestand messen. Vorvalidierung, globale Konflikte und bounded Verarbeitung erhalten. Vorhandene Grenzen 256 MiB/500.000 Chunks/10.000 Dokumente an realem Material prüfen; keine Kürzung oder blinde Grenzerhöhung.
- Dediziertes Runtimeziel und DB-Identität prüfen; vollständige Import-, Altbestand-, Wiederholungs-, Typen-, Projektions-, Index- und Readerbeweise führen. Gepinnte Store-Revision und Originalrevision getrennt erhalten.
- Aktuellen Mainstand vor Integration erneut prüfen. Externe RESTSTATUS-Akte nennt Brain-Trackingmain f7b02cc1; das ist gegenüber unserer Basis neuer und kein aktueller eigener Fetchbeweis. Kein Wiki-Abschluss aus dem Forum-Merge ableiten.

Runtimeziel laut C3: `/run/deadlock-brain-postgresql`, Port 5446, DB brain, Rolle brain_ingest, max_connections 2, vorhandener Infisical-Verweis BRAIN_PG_INGEST_PASSWORD. Keine Secretwerte, ENV-Dateien oder Prozessumgebungen lesen.

## D: Downloader noch ohne abgenommenen Eigencommit

Produktiver Tree `/home/nathanael/.worktrees/steam-brain-spieldepot-d`, Branch `feat/brain-spieldepot-download`, HEAD `4c5621763d5f01c96d7912400517c08aa1c40df1`. Änderungen uncommittiert erhalten. Bereichsakte im Tree `/home/nathanael/.worktrees/brain-wiki-spielwissen-d/.tasks/2026-10-03-wiki-spielwissen/bereiche/d/`.

Bestehender Fixer a6bfafbf1f31d680f. Punkt 53: öffentlicher NetworkError::Ws-Payload zentral in net.rs boxed, bestehende From-/EResult-/Display-/Debug-/Error-Wirkung und direkte source()-Downcasts erhalten. 41 Fälle nur vorbereitet, nicht gelaufen. Historischer Vor-Punkt53-Lauf bestand Format/Check/40 Tests, strenges Core-/Vendor-Clippy war rot. Diese Zahlen gelten nicht als Endbeweis des geänderten Stands.

Aktueller Altprozessabgleich aus PRUEFUNG-ALTPROZESSE.md, 17:39:41 UTC: bdy6husup an originalen Start 14:46:16.615Z und UUID c031fd6a-caed-43cc-bfdf-aa5af16b14c2 gebunden. Erhaltenes Log nur Vorlock-PIDmarker; in diesem Ablauf konnten nur flock-Wartekinder, keine Compiler oder PG starten. Direkte Metadaten fanden keine dazugehörigen Kinder oder Log-FDs. Kein regulärer alter Testexit behauptet. bk881ym4u gehört nicht zu den neun dokumentierten Backgroundtasks des Fixers; nur spätere [killed]-Datei, keine sichere eigene Startbindung. Nicht als eigenen laufenden Job behandeln oder fremde Prozesse stoppen. Keine globale Freiheitsbehauptung.

Offen nach Punkt 55: genau ein regulärer Endlauf mit gemeinsamem Freeze und strengen Core-/Vendorchecks, tatsächlich 41 Fälle ausführen. Unabhängige API-/Bug-/Securityabnahme einschließlich Punkt48-Traitverträge, Punkt49-Workspace-/Featuregraph und Punkt53 aller 13 Fehler-Varianten, Source-Ketten und Send/Sync. Danach eigener geprüfter Commit an C3; noch keiner vorhanden.

Kostenlose App-Lizenz für 1422450 erfolgreich gewährt. Das belegt keine Depot-/Manifest-/Dateiübertragung. Keine neue Anmeldung oder erneute Grantaktion. C3 deployt den geprüften Downloader, danach bestehender Task AUTH_DOWNLOAD_DEADLOCK_GAME mit Payload {}. Daten ausschließlich unter lokalem Datenroot `.../2026-10-03/d/`.

Vor Download Speicher-/Depotgrößen prüfen. Inventar muss tatsächlichen Build, Depot, Manifest, Dateien, Größen und Hashes binden. complete nur nach vollständiger Manifest-/Chunk-/Größen-/Hashprüfung. Fehlerphase und numerisches EResult erhalten, keine Transportzugangswerte oder Secrets speichern. Danach B liest exakt gebundene Originaldateien; kein erfundener oder fremder Rohbestand.

## Website: Vertrag entschieden, Umsetzung und Abnahme offen

Quellenbericht `/home/nathanael/Documents/.tasks/2026-10-03-branch-restbefunde/WEBSITE-READER-ABGLEICH.md`. C3-Entscheidung zentral unter bereiche/c/C3-WEBSITE-EXPORTVERTRAG.md.

Website-Owner des externen Auftrags behebt in eigenem Tree Shardabdeckung, variable JSON-Fences und Filterung interner Felder in Tabellen/Nested sowie allen Ausgabe-/Suchpfaden. Keine parallelen Änderungen an C3-, Writer-, Import-, Persistenz- oder Releasepin-Dateien. Eine konkrete ausführende neue Website-Session-ID wurde uns nicht mitgeteilt; nicht erfinden oder fremde Sitzung übernehmen.

C3 bleibt zuständig für spätere technische Brücke vom CorpusRelease zum bestehenden atomischen Dateiexport. Öffentlich freigegebene Dokumentmenge, genaue Store- und getrennte Originalrevision sowie Attribution binden. Kein zweiter Writer oder Datenbestand. Für den neuen C3-Bestand ist keine konkrete öffentliche Dokumentfreigabe nachgewiesen. Interner Pin und Quellenname reichen nicht; A bleibt ausgeschlossen. Git-MIT erlaubt nicht automatisch enthaltene Valve-Assets. Freigabe strukturierter Werte erlaubt nicht automatisch Originaltext oder Bilder.

Geplant, nicht implementiert: versionierter Exportbindungsnachweis in derselben unveränderlichen Generation, vor Dateidigest geschrieben; intern bindet er Release, ausgewählte Revisionen, Rechte, Projektionsversion und Shards. Statusschema 1 erhalten. Counts ausschließlich öffentliche Teilmenge. Kein Durchreichen interner Release-/Dokument-IDs, Hashes oder Pfade. Gemeinsame fachliche Positivgruppen vereinbart; konkrete erlaubte Feldzuordnung je Artikelart noch zu bauen.

Vereinbarter HTTP-Vertrag: ausdrücklich freigegebene gültige leere Generation 200; fehlend/defekt/nicht freigegeben 503 ohne interne Details; unbekannter Artikel bei gültiger Generation 404; ungültige Suche 400; gültige Suche ohne Treffer 200. Writer und Reader lehnen heute beide Nullmengen ab, deshalb gemeinsame Änderung nötig. Fehlende Rechte niemals als erfolgreiche Leermenge darstellen. Ein weiterhin gültiger alter öffentlicher Snapshot darf erhalten bleiben, mit ehrlichem Quellenstand.

Offen: gemeinsame Handler-HTML-Beweise für Artikel, alle Shardtypen, längere Fences, interne Felder, Suche, leer und defekt; aktuelle Compilerprüfungen, unabhängige Intent-Abnahme, Standardgate, gezielte Caddyintegration und Liveprüfung. Alte Screenshots oder statisches ALLOW ersetzen das nicht.

## Fremde Abschlussbereiche und Reihenfolge

Direkter externer Auftraggeber T3 `64cab234-aaf8-48e0-ab7c-440c2468a199`, Akte `/home/nathanael/Documents/.tasks/2026-10-03-branch-restbefunde/`. Laut letzter Nutzermitteilung wartet Launcher-Root 77552; ältere 17073-Kennung überholt. Tatsächlichen aktuellen Zustand dort erneut prüfen, kein zweiter Build. Alte LAUNCHER-ABSCHLUSS.md beschreibt einen früheren Fixkopf und ist keine aktuelle ae490cd9-Endmeldung.

Vor neuen eigenen Prüfschleifen: Launcher ae490cd9e9f6d647f5107cc530db63e6a11d231f tatsächlich abschließen lassen; anschließend Relay-Suite 2413e1f0 und Streamstatistik-Suite 307a6df3 abstimmen. Coaching 2e4d38e3 ist laut Auftraggeber Main/live, kein weiterer Slot nötig. Relay hat positive Quellenabnahme, technische Suite und neues Standardgate fehlen; danach nur Gitintegration, kein alter Relaydeploy. Beide eigenen Integratoren haben Punkt 55 bestätigt. Kein zentraler Hostkoordinator oder FIFO-Garantie belegt.

Launcher-Cutover ändert ausschließlich private Launcherpfade von dl-bot, steam-core, steam-core-2, steam-bot und turnier-bot. Apps, Config und Argumente bleiben erhalten; Steam-Apps weiterhin `/opt/deadlock/steam/current`. Jede neue Startvertrags-/Apphashänderung vor Cutover führt zur erneuten Koordination. Unser Steam-Cutover bleibt bis gemeinsamer Verbundprüfung zurückgestellt.

Separater Brain-/Steam-Abschluss: Hauptorchestrator `e6c19079-657e-4db9-80bd-8e1313e7f785`, Akte `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung/`. Aktiver Publishowner S2 T3 `c6ddac1c-c0d2-4bbc-b6f8-7bc462570a52`, native `36b5f0c0-6349-4454-8cf4-c20d7a9e8fa0`, Tree `.worktrees/steam-publish-fertig`, gemeinsame Installation bei deren Integrator Z. Historisch gelesener Publishkopf 9aec0cc897b01b74d417ab9b510314cbbbd02535 und Brainpaar 9a6f3d5f3ae2349d9fb076821381e45a421a0bbe sind vor eigener Übernahme neu zu prüfen. Fremder Diagnosezweig aeadf11821d878e3dc4be22d33ecfc098f2aefd8 ebenfalls gemeinsam abgleichen. Kein konkurrierender Current-Wechsel oder zusätzlicher Publisher.

Weitere fremde Reste sind in RESTSTATUS-AKTUELL.md verzeichnet: C9/Community/Guide, Scrim, Lurker, Cast, Startup-Snapshots, Website und alter Purposeanteil. Vorhandene Eigentümer bleiben zuständig. Diese Übergabe übernimmt deren Aufgaben nicht. Fremder Forum-Parent 2075980 und brain-fertig-q gehören nicht zum Wiki-Bereich.

## Host, Gate, Deployment und Cleanup

Alle Compilerprüfungen und Releasebuilds blockierend unter beiden Locks: zuerst `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock`, dann `/tmp/deadlock-cargo-release.lock`. Unmittelbar vor jedem Compiler frische NonZombie-Probe; maximal zwei Jobs und ein Releasebuild auf dem Host. Aktiven oder unklaren Cargo-/rustc-/Clippy-/rustdoc-Prozess nicht ignorieren. Einzige enge Metadata-Ausnahme steht in Documents/.tasks/2026-10-02-offene-branches/HOSTPROBE.md. Keine fremden Signale, Lock-/Permitmutation oder Zeitabbruch.

Test-PG startet ohne geerbte FD8/9. Nach Prüfung eigene Server, Kinder und FDs tatsächlich regulär beenden. FD offen bedeutet nicht automatisch Lock gehalten; fehlende PID bedeutet nicht Erfolg. Der abgeschlossene C3-Wrapper ist keine aktuelle Sperrfreiheitszusage.

Nach Integration unabhängige Intent-Abnahme gegen Nutzerziel, dann Bug-/Security-Standardgate auf exakt dem endgültigen SHA mit Sol high. Fixer prüft sich selbst vor Übergabe. Gemeinsame Schreib-/Lesepfade zusammen abnehmen. Kein Gate-ALLOW eines alten Fixcommits übertragen. Keine PRs/Actions und kein Force-Push.

Installation nach INSTALLATIONSPLAN-C2.md, mit frisch gemessenen Starts, Mainständen, Apphashes und Rückrollzielen. Brain immutable `/opt/deadlock-brain/maintenance-releases/<sha>`, Zeiger maintenance-current; V1-current unberührt. Steam immutable `/opt/deadlock/steam/releases/<sha>`, Zeiger `/opt/deadlock/steam/current`. Zusätzliche Installationslocks `/run/lock/deadlock-brain-release-install.lock` und `/run/lock/deadlock-steam-release-install.lock`. Keine laufenden ELFs überschreiben. Bestehende Infisical-/Config-/OAuth-/Startverträge erhalten.

Nach Änderungen passende normale Units neu starten und tatsächliche ELFpfade/Hashes, Source-SHA, Health, Readiness, Corpusversion und Funktionen live messen. MainPID allein reicht bei Launcher/sudo-Ketten nicht. Rücknahme nur wenn Zeiger und normale Config noch exakt dem eigenen gerade installierten Stand entsprechen; keine fremden neueren Releases zurückrollen oder Datenbankhistorie löschen.

Eigener Proxy: temporäre User-Unit brain-wiki-sol-proxy.service, Port 18768, zuletzt MainPID 2558697/active/running. Startweg und claude-sol.json unter zentralem startweg/. Sol high wurde um 18:16 UTC mit PROXY_OK tatsächlich geprüft. Nur unkritische lokale Modell-/Proxy-ENV erlaubt, niemals Tokens. Keine globalen Claude-Settings oder fremden Proxys ändern. Normale Werkzeugfreigaben und don'tAsk erhalten; keine Hooks deaktivieren. Proxy erst nach Ende eigener Sitzungen beim tatsächlichen Gesamtcleanup stoppen.

Zum Abschluss alle eigenen SHAs und unversionierten Nachweise sichern, insbesondere A-Harness sowie B-/D-Rohdaten und C-Integration. Nur nach belegtem Merge/Push/Deploy/Live eigene Branches und Worktrees entfernen, fremde oder dirty Hauptcheckouts nicht resetten. TODO/ENDE nur nach wirklichem Abschluss aktualisieren lassen. Alle Texte natürliches Deutsch mit Umlauten, humanizer und no-em-dashes. Keine Secrets, Configinhalte, ENV-Dateien oder Rohlogs in Übergaben.

## Maßgebliche Akten

Zentraler Root: `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-wiki-spielwissen/`.

AUFTRAG.md, CONTRACT.md, PAKETE.md, REGISTER.md, AN_BEREICHE.md, TODO.md sowie bereiche/a bis d. Neuester S2-Aufgabenstand beim Lesen: 18:27:53 UTC, A43/B28/C3-10/D40. Ds Nachtrag von 17:39:41 und C3s Exportvertrag sind zusätzlich maßgeblich; nicht allein aus alten Ereigniszählern auf heutigen Zustand schließen. Unveränderliche Ereignisse unter status/ nie überschreiben.

Eine belastbare Restzeit liegt nicht vor. Abschlussbericht muss historische Wikiabdeckung, Git-Spielwerte und echte Depotabdeckung getrennt ausweisen und verbleibende Grenzen nennen.
