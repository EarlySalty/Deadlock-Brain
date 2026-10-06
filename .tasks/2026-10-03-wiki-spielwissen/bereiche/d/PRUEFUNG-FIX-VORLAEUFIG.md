status: offen
Datum: 2026-10-03
Stand: 2026-10-03T12:10:20Z

# Vorläufige echte Fixstandprüfung

Eigener Wrapper b1hk2rmx7, Start-PID 4154112. Registrierter Log:
/tmp/claude-1000/-home-nathanael--worktrees-brain-wiki-spielwissen-d/cdbe72ce-ea3f-459f-ad8a-02f9e0436158/tasks/b1hk2rmx7.output

Caller liest den 3561-Byte-Log gezielt und bestätigt am 03.10.2026 um 11:54:54Z:

- FORMAT_STORE_EXIT=0, FMT_CORE_EXIT=0, FMT_VENDOR_EXIT=0.
- cargo check -p steam-core -j 2 --locked tatsächlich Exit 0, Finished dev profile in 13,80 s.
- Clippy-Aufruf Exit 101: error: no library targets found in package steam-core. Falsche eigene --lib-Zielauswahl, kein fachlicher Clippy-Nachweis und keine danach ausgeführten Tests.
- OWN_FD9_FD8_CLOSED=1, OWN_WRAPPER_EXIT=101, SDK-Log [exited with code 101]. Worker bestätigt abgewartete Kinder/none; Caller bestätigt eigene Start-PID 4154112 nicht mehr vorhanden.

Keine Quellkorrektur allein aus dem Befehlsfehler. Derselbe Fixer ermittelt tatsächliche vorhandene Cargo-Targets und korrigiert nur die normale Prüfbefehlsauswahl. Kein ersatzweiser Parent-Prüflauf, kein neuer Scope und keine parallele Task. Endstandchecks, Clippy/Tests und unabhängige Folgerunde weiter offen.

## Quellbindung nach Formatierung

Alle 15 im Prüflog aufgeführten Paketdateien wurden vom Caller erneut gehasht. Alle aktuellen Dateibytes entsprechen den Loghashes, keine Abweichung. Diese Bindung ist keine finale ALLOW-Abnahme. Jede spätere Änderung entwertet betroffene Nachweise.

| Datei relativ zum Steam-Worktree | SHA256 |
| --- | --- |
| rust/Cargo.lock | 7357592c893a474b3261fb29b3c7116a3baf5c3e8a94cb4cbf5f9b9782fdfc83 |
| rust/Cargo.toml | 4a9b9c7259fed3aa682ba1fa59f4db28b483509f422ff0bff89111f38d59b7c5 |
| rust/crates/steam-core/Cargo.toml | 8dabb5cca0a9f959e57a9f047471167fc7db10787a1102bdbfe2ff2188deab5c |
| rust/crates/steam-core/src/task/handlers/game_download.rs | 472f0e22b5febe381eb7a36d673df1c4d0cb2e79e298bcc2c090e0871da0cce5 |
| rust/crates/steam-core/src/task/handlers/game_download/manifest.rs | abbcdf5be6c5de152e0985fac858381785aeaabce3308cde625a8f641f2ad5a7 |
| rust/crates/steam-core/src/task/handlers/game_download/protocol.rs | 200eed337f586bbf8385b369f7ec21cd57f1c16fcd66fba1aeb10425274fe129 |
| rust/crates/steam-core/src/task/handlers/game_download/store.rs | 8e673c97f3e8e7175a80dac6357be3984c0ddbf06f4d5af4d72226a56a382dea |
| rust/crates/steam-core/src/task/handlers/game_download/tests.rs | 0de8db64669def9eb57acc4b61f806e3e5e37453d7a247beb3351819144c681d |
| rust/crates/steam-core/src/task/handlers/mod.rs | 42286e8f3af438de611fdb6d93b0db8eb4bec7d3e975e40b4ecb95eb8a9da702 |
| rust/crates/steam-core/src/task/lanes.rs | bfb054ba93410493647faad41853d10d39fb555967cbd4b7811aede32528657c |
| rust/crates/steam-core/src/task/mod.rs | f4743e2536f2a7efe84fc1895f7f59d39f0ed74794f465e995b1d9f6fb4f2ef7 |
| rust/vendor/steam-vent-0.4.2/src/connection/filter.rs | 90da5fda2d6fd7c2a734968d5354586cabc2bde36d1688c52a974cfdf1359534 |
| rust/vendor/steam-vent-0.4.2/src/message.rs | bfdf0e692e9c053ecbaab8f6c894d95e580fefdca6da1c2fc27fc98ff9fc8fdb |
| rust/vendor/steam-vent-0.4.2/src/net.rs | 5834e0692990db0da3385d5139a8dba8a6255d1793d17f3c817d0b4064a3ef8b |
| rust/vendor/steam-vent-0.4.2/src/transport/websocket.rs | 1bba6aa54b58e4cd80653a6007ddd8d315327bf33487bbf08e132b530f0d6dbf |

Zusätzlicher Hashmarker im Log: 868d17ac01dfcb198ccf75b501a2f9b079fd3ffca4bb72527f915955d77ec18c. Sammelhashalgorithmus nicht mit dem historischen ebad-Snapshot gleichgesetzt.

## Korrigierter Binary-Prüflauf b3jmsrhoo

Log /tmp/claude-1000/-home-nathanael--worktrees-brain-wiki-spielwissen-d/cdbe72ce-ea3f-459f-ad8a-02f9e0436158/tasks/b3jmsrhoo.output vom Caller gelesen. Start-PID917572. FMT_CORE_EXIT=0, FMT_VENDOR_EXIT=0, CHECK_EXIT=0 in0,67s. Tatsächlich ausgeführtes Core-Clippy mit --bin steam-core --tests --features testing -- -D warnings meldet CLIPPY_CORE_EXIT=101.

Eigene drei Funde in freigegebenem game_download/manifest.rs:82 manual_range_contains, :471 op_ref, :519 useless_vec. Zusätzlich vier double_must_use aus async_trait: zwei an api/mod.rs:105, einer steam/cso.rs:49 und einer task/handler.rs:103. Caller bestätigt git diff --exit-code HEAD für genau diese drei Fremddateien mit Exit0. Unveränderter Ausgangsbestand, keine Freigabe zur Mutation durch D.

Tests und Vendor-Clippy wegen Abbruch noch nicht ausgeführt. Log bestätigt OWN_CHILDREN_BEFORE_CLOSE=none, OWN_FD9_FD8_CLOSED=1 OWN_WRAPPER_EXIT=101 und SDK-Exit101. Worker bestätigt frisches ps mit Exit1 für die eigene Start-PID. Kein Nachher-Snapshot im Wrapper; Worker vergleicht danach lesend alle15 aktuellen Quellen mit Startzeilen,15/15 identisch und Listenhash868d17ac01dfcb198ccf75b501a2f9b079fd3ffca4bb72527f915955d77ec18c. Dieser nachträgliche Abgleich bleibt vom Wrapperbeweis getrennt.

Derselbe offene Fixauftrag setzt ausschließlich die drei eigenen Korrekturen fort. Normale strenge Checks und bestehende gezielte Downloader-/Vendor-Tests unabhängig mit Einzelexits ausführen. Keine Lintunterdrückung oder Änderung fremder Dateien. Fremde Grenze über AN_HAUPT.md an Root/C3 gemeldet. Jede folgende Quellenänderung entwertet betroffene Teilprüfungen. Gesamtstand rot, keine finale Folgerunde oder Commitfreigabe.

## Laufender Endstandwrapper bmgzuij38, Teilbeweis 12:16:47Z

Caller liest registrierten Log /tmp/claude-1000/-home-nathanael--worktrees-brain-wiki-spielwissen-d/cdbe72ce-ea3f-459f-ad8a-02f9e0436158/tasks/bmgzuij38.output. Start-PID995295 und OWN_BOTH_LOCKS_HELD=1. FMT_CORE_EXIT=0, FMT_VENDOR_EXIT=0, CHECK_EXIT=0 in8,66s. Neuer manifest.rs-Hash41198734b6cb332ff0796510e6dc9aa8a1d05c215835ab5d00a7814332723e53, Startlistenhash474b9607053b96addf3d4fa72310864cf0605b0cbf9b1707a12a89a2bee24a67. Kein abschließender Nachhervergleich behauptet.

Core-Clippy101 enthält nur die vier bereits gemeldeten Fremdbefunde, keine manifest.rs-Funde mehr. Vendor-Clippy101: eigene neue Tests benötigen protobuf::Enum in net.rs/message.rs und eindeutigen Assert-Typ; zusätzlich tracing_subscriber nicht verfügbar. Worker meldet bestehende Vendor-dev-dependency, deren Manifest nicht zugewiesen und unverändert bleibt. Gezielt19Downloader-/Manifest-/Storetests tatsächlich bestanden:19passed,0failed,303filtered,1,73s, TEST_DOWNLOAD_EXIT=0. Das beweist lokale Tests, keinen echten Steamdownload.

TEST_RESULTS_EXIT=101 und TEST_MULTI_EXIT=101 mit exakter Cargo-Meldung: package steam-vent cannot be tested because it requires dev-dependencies and is not a member of the workspace. Empfangsfilter meldet dieselbe Ursache; sein Exit und Wrapperende zum Lesezeitpunkt noch nicht im gelesenen Log. Kein Ende/Kinder-/Lockfreigabe behauptet.

Aktiven Wrapper erhalten, keine Quellenänderung. Nach regulärem Ende eigene Testimporte/-typpräzisierung nur in freigegebenen Quellen. Workspace-Mitgliedschaft noch NICHT freigegeben: root Cargo.toml nur für minimal nötige Downloaderabhängigkeiten zugewiesen. Derselbe Fixer prüft zunächst lesend vorhandene Cargo-Testzielauflösung und direkten Manifestweg. Kein zusätzlicher Vendor-Cargo-/Lockpfad, keine neue Dependency oder Lintunterdrückung. Konkreten Konflikt bei nötiger Scopeausweitung an Root/C3. Gesamtstand rot, Folgerunde/Eigencommit offen.

## Bestätigtes Ende bmgzuij38, 12:21:39Z

Caller liest Logfortsetzung: TEST_RECEIVE_EXIT=101, SOURCE_SNAPSHOT_UNCHANGED=1, OWN_CHILDREN_BEFORE_CLOSE=none, OWN_FD9_FD8_CLOSED=1 OWN_WRAPPER_EXIT=101 und SDK-Exit101. Beide15-Dateienlisten mit Listenhash474b9607053b96addf3d4fa72310864cf0605b0cbf9b1707a12a89a2bee24a67 identisch. Caller ps -p995295 -o pid=,stat=,comm= liefert Exit1 ohne Ausgabe. Eigener Wrapper regulär beendet, keine fremde Prozessaktion.

Ergebnis: fmt/check0; strenges Core-Clippy101 durch vier unveränderte fremde Makro-Lints, Vendor-Clippy101 durch Testkompilierung;19Downloader-/Manifest-/Storetests tatsächlich bestanden. Alle drei Vendor-Testfilter Exit101 vor Testlauf wegen fehlender Workspace-Mitgliedschaft/dev-dependency-Auflösung.

Erst nach bestätigtem Ende korrigiert derselbe Fixer laut Bericht testlokale protobuf::Enum-Importe in freigegebenen net.rs/message.rs und eindeutigen MsgKind::from im eigenen Assert. Root-/Vendor-Cargo unverändert. Betroffene frühere Vendorchecks entwertet, kein Endstandgrün. Vorhandener Vendor-Cargo.lock laut Worker vorhanden; nächster einziger Hostlockwrapper prüft normale direkte --manifest-path Vendor-Cargo.toml --locked-Zielauflösung, ohne neue Lock-/Manifestdatei oder Mitgliedschaft. Noch kein neuer Wrapperstart registriert.

## Bestätigtes Ende b89y6ioc2, 12:30:17Z

Caller wertet registrierten Log gezielt aus: /tmp/claude-1000/-home-nathanael--worktrees-brain-wiki-spielwissen-d/cdbe72ce-ea3f-459f-ad8a-02f9e0436158/tasks/b89y6ioc2.output. Start-PID1066919. METADATA_ROOT_EXIT=0, METADATA_VENDOR_EXIT=101, FMT_CORE_EXIT=0, FMT_VENDOR_EXIT=0, CHECK_EXIT=0, CLIPPY_CORE_EXIT=101, CLIPPY_VENDOR_EXIT=101, TEST_DOWNLOAD_EXIT=0; TEST_RESULTS_EXIT/TEST_MULTI_EXIT/TEST_RECEIVE_EXIT jeweils101.19Downloader-/Manifest-/Storetests bestanden,0failed,0ignored,303filtered,1,96s.

Vendor-Metadaten/Clippy/Testwege scheitern am direkten vorhandenen Manifest mit „current package believes it's in a workspace when it's not“. Root-Metadata laut Worker11Mitglieder, Vendor kein Mitglied. Vorher/Nachher-Quellenlisten identisch, SOURCE_SNAPSHOT_UNCHANGED=1 und READONLY_VENDOR_UNCHANGED=1. Vorhandene Vendor-Cargo.toml/Cargo.lock nur lesend kontrolliert. OWN_CHILDREN_BEFORE_CLOSE=none, OWN_FD9_FD8_CLOSED=1 OWN_WRAPPER_EXIT=101, SDK-Exit101. Caller ps auf eigene Start-PID liefert Exit1 ohne Ausgabe. Sicheres reguläres Ende bestätigt, keine Fremdprozessaktion.

Punkt48-Corefix kann jetzt im selben offenen Auftrag ausschließlich die drei Zusatzdateien für konkrete double_must_use-Attributursache bearbeiten. Danach neuer18-Dateien-Freeze und passende Checks. Kein Lint-Allow oder Abschwächen. Vendor-Vertragsrest getrennt über AN_HAUPT.md: tatsächliche Standardprüfwege belegen Bedarf enger Root-Cargo-Testzielzuweisung, noch keine Mitgliedschaft/exclude/vendor-[workspace] oder Dependency-Upgrades. Gesamtstand rot; neue Quellenänderungen entwerten betroffene Beweise.

## Tatsächlich grüner Punkt48-Corestand bj8t2m1tc, 12:48:40Z

Caller wertet eigenen registrierten Log /tmp/claude-1000/-home-nathanael--worktrees-brain-wiki-spielwissen-d/cdbe72ce-ea3f-459f-ad8a-02f9e0436158/tasks/bj8t2m1tc.output aus. Start-PID1122243, OWN_BOTH_LOCKS_HELD=1. FMT_CORE_EXIT=0, FMT_VENDOR_EXIT=0, CHECK_EXIT=0, CLIPPY_CORE_EXIT=0. TEST_DOWNLOAD_EXIT=0:19passed/0failed/0ignored/303filtered in2,96s. TEST_API_EXIT=0:6passed/0failed/0ignored/316filtered in0,01s. TEST_CSO_EXIT=0:6passed/0failed/0ignored/316filtered in0,05s.

18-Dateien-Snapshot vorher/nachher identisch, SOURCE_SNAPSHOT_UNCHANGED=1. OWN_CHILDREN_BEFORE_CLOSE=none, OWN_FD9_FD8_CLOSED=1 OWN_WRAPPER_EXIT=0, SDK-Exit0. Caller ps auf eigene Start-PID1122243 liefert Exit1 ohne Ausgabe. Sicheres reguläres Ende und echte31Testausführungen bestätigt. Runner-TestDb-Laufzeittests nicht gestartet, vorhandene Implementierungen über --tests mitkompiliert. Makrovertragsgleichheit noch unabhängig abnehmen.

Erst danach Root-Cargoänderung gemäß Punkt49 durch einzigen Fixer gemeldet: Vendor in members, dieselben11default-members explizit erhalten, keine anderen Cargodateien geändert. Alte Teilchecks beweisen diesen neuen gemeinsamen Graph nicht. Reguläre minimale Root-Lockauflösung und neuer18-Dateien-Freeze/Core-/Vendor-/Testchecks folgen. Worker-Baseline vor Auflösung:477Pakete, Versionen-/Quellen-/Checksummenhash ebca487b9839035d43bb985b39d315d7f687fa35d8ce4d727f190897753942fc. Kein Upgrade oder neuer deklarierter Dependency, tatsächlichen Minimaldiff neu belegen. Noch keine finale Folgerunden-/Commitfreigabe.

## Reguläre Punkt49-Auflösung b0kk4c3y4, 13:44:50Z

Caller liest registrierten Log gezielt. RESOLVE_CHECK_EXIT=0; Paketbaseline477/ebca487b9839035d43bb985b39d315d7f687fa35d8ce4d727f190897753942fc wächst auf480/55d70cc73e8704a607ca9e20b49977870677d43d76e7447847a8a8d4077e3e6f. Eigener technischer Gleichheitsguard meldet PACKAGE_VERSIONS_SOURCES_UNCHANGED=0 und bricht vor finalem Freeze/Core-/Vendor-/Testprüfsequenz ab. Das ist kein Merge-Gate-Urteil. OWN_CHILDREN_BEFORE_CLOSE=none, OWN_FD9_FD8_CLOSED=1 OWN_WRAPPER_EXIT=101, SDK-Exit101; eigenePID1209726 per Caller-ps Exit1 beendet.

Tatsächlicher Lockdiff: vorhandener Vendor erhält tracing-subscriber/vdf-reader als vorhandene dev-dependencies; steam-vent-proto erhält drei bereits optional deklarierte Pakete steam-vent-proto-csgo/dota2/tf2 jeweils0.5.2. Caller liest Registrymanifest0.5.2: alle drei mit optional=true und entsprechenden vorhandenen Featuredeklarationen. Root-Cargodiff nur zugewiesene member/default-members und historische Downloaderdeps, keine neue Featuredeklaration. Worker rechnet ursprüngliche477Name-/Version-/Quellen-/Checksummenidentitäten separat zurück auf exaktenBaselinehash; kein Upgradebefund. Ob die optionalen Features im echten Zielgraph aktiv sind, separat in nächstem Prüflauf belegen.

Caller vergleicht aktuellen18-Dateienstand mit grünem bj8t2m1tc-Snapshot:16/18identisch, ausschließlich rust/Cargo.toml und rust/Cargo.lock geändert. Aktuelle SHA256: Cargo.toml f215daa66c9dece301f9462dcc5eb238b8e834e33afdc10502edd4549c6390f6, Cargo.lock e86c51e53a9a47f1caa406234c54da4c87df6944034906caff05e6ec17fe86b7. Diese Bindung ersetzt keine finale Prüfung.

Urteil: Gesamtpaketanzahlguard war enger als ausdrücklich erlaubte minimale reguläre Testzielauflösung. Punkt49 wird nicht erweitert; keine neue Dependencydeklaration/Upgrade/Lockmanipulation. Derselbe offene Fixer darf weiter die ursprünglichen477Identitäten exakt und genau diese drei vorhandenen optionalen Additionen offen prüfen, Featureaktivierung gesondert belegen und einen neuen einzigen vollständigen18-Dateien-Endstandlauf starten. Keine Lintunterdrückung oder Gateumgehung. Unabhängige Abnahme/Eigencommit weiter offen.

## Vollständiger gemeinsamer Lauf bho5pp33s, 14:06:48Z

Caller wertet eigenen registrierten Log aus. Regulärer Wrapperexit101, eigenePID1515805 per ps Exit1 beendet. Quellenlisten18Dateien vorher/nachher identisch, Listenhash26cb27548cec41a0c8051a06eecb97ad28bb60f36b03c6e2c2f7c7f4496c4777 zweimal vorhanden. READONLY_VENDOR_UNCHANGED=1, OWN_CHILDREN_BEFORE_CLOSE=none, OWN_FD9_FD8_CLOSED=1 OWN_WRAPPER_EXIT=101 und SDK-Exit101.

METADATA_EXIT=0, FEATURES_CORE_EXIT=0, FEATURES_VENDOR_EXIT=0; Caller parst tatsächliche Metadaten:12Mitglieder/11default-members, Vendor nicht default. Beide echten Featurebäume enthalten steam-vent-proto, keine csgo/dota2/tf2-Aktivierung. FMT_CORE_EXIT=0, FMT_VENDOR_EXIT=0, CHECK_EXIT=0. Alle sechs lokalen Filter Exit0:19Downloader (1,84s),6API(0,01s),6CSO(0,05s),5EResult(0,00s),3Multi(0,05s),1Empfang(0,00s), insgesamt40, jeweils0failed/ignored. Keine echten Steam-/Download-/DB-Runner-Laufzeitbeweise daraus ableiten.

CLIPPY_CORE_EXIT=101 und CLIPPY_VENDOR_EXIT=101. Konkrete eigene message.rs-Funde:223 manual_range_contains,455 useless_vec. Zusätzlich result_large_err aus136ByteNetworkError::Ws:55Bibliotheks-/56Testhinweise, insgesamt111im Log gezählt. Keine pauschale Behauptung, sämtliche Folgefunde seien unveränderter Vorbestand. Neuer Workspace-Lintpfad deckt Auth/Session/GC mit ab.

Derselbe Fixer darf nur eigene zwei message.rs-Lints eng beheben. Zentrale Empfehlung an Root: net.rs NetworkError::Ws(Box<tungstenite::Error>) mit ausdrücklich erhaltenem From<tungstenite::Error>, Display/Debug/source-Kette/Caller/EResult-Vertrag. Öffentlicher Variantpayload ist API-Vertragsänderung und nicht ohne ausdrückliche Zuweisung umgesetzt. Keine Auth-/Session-/GC-Änderung oder Lint-Allow, keine weitere teure Gesamtprüfung mit unverändertem selben API-BLOCK. Exakten Konstruktor-/Match-/From-Bedarf zunächst lesend eingrenzen. Gesamtstand rot, unabhängige Folgerunde/Eigencommit weiter offen.

B-Vertragsbasis: geprüfter Eigencommit 48b6ce1cf277ec4de4b47ac4a0898fbc223fe6b5, konkrete Teilbindung in LESEVERTRAG.md. C3 alleiniger Integrations-/Deployer, keine Steam-Aufrufe oder tatsächliche Depotabnahme. Appgrant nicht wiederholen.
