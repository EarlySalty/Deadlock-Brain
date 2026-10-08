# G-V: tatsächlicher Anschlussbestand der Fortsetzung

Stand: 07.10.2026. Lesender nativer Workflow wqg72jmc6 / wf_3ed37a17-4a7 abgeschlossen. Graphify zuerst, danach Quellen und Nebenpfade. Kein Produktwriter, Reviewer, Test, Browser, Modellaufruf oder Runtimeeingriff. Vollständige Rückgabe: /tmp/claude-1000/-home-nathanael--worktrees-brain-g-v2-20261007/1380f548-176f-4f3f-ab8f-0fd210bc3947/tasks/wqg72jmc6.output. Worker lieferte den Bericht inline; diese Akte wurde von G übernommen.

## Geprüfte Referenzen und Grenzen

G-Start 8feb8b6ec0bf3dac7a8e180bfacc59ed001d3206. Lokal gespeicherter main ca4d877f13042c9a7a7023e54f6bf2c688b69ac4, kein Fetch in diesem Vorcheck. Committed Referenzen I 501d3725, E 9d17ee52, F 46fd8674 und K 38280ca8 gelesen, keine uncommittierten Fremdquellen übernommen. Keine Produktions- oder aktuelle origin/main-Aussage daraus. Kernel/Tools/Serve/Storage/Retrieval entsprechen dem G-Startstand. data.rs/lib.rs und calculation.rs bleiben S3-WIP.

## Produktionsport und K-Anschluss

- ToolExecutionPort in rust/crates/brain-contracts/src/tools.rs:874 bietet definitions, execute, execute_accounted, validate_dependencies und validate_build_plan. GameContextResolver in brain-kernel/src/lib.rs:62 bietet resolve und validate, Kernel::with_tools steht dort ab :118.
- In den geprüften G-Rustquellen keine Produktionsimplementierung beider Traits. Vorhandene Implementierungen sind Testfixtures, darunter brain-serve/tests/tool_evidence_loop.rs:304,329. service.rs:431 erzeugt im G-Stand noch Kernel::new.
- Ks committed Referenz delegiert die accounted- und Toolturnmethoden bereits in service.rs:126. Kein erneuter Wrapperbau nötig. Dort fehlt bei :475 weiterhin with_tools. K besitzt diese zentrale Verdrahtung und Pool-/Resolverintegration.

## E/I hat einen echten Receipt geliefert

Die alte Fehlensmeldung zum Value-only-Leser ist für die geprüften E/I-Commits überholt. I 501d3725 und E 9d17ee52 enthalten bytegleich brain-storage/src/asset_mirror.rs mit:

- AssetDocumentReceipt, AssetMirrorReceipt und MirroredAssets { payload, receipt } bei :51,65,80.
- load_mirrored_assets_with_receipt(&PgPool, i64, &str, Option<&str>) -> Result<MirroredAssets> bei :196.
- load_mirrored_assets_for_run(&PgPool, source_run_id, client_version, kind, language) -> Result<MirroredAssets> bei :206.

Der Leser bindet Endpoint-/Manifestdokumente an den gewählten vollständigen Run, prüft Hashes, Parserrevision, Herkunft, Validierung und Zeiten (:152-169,231-343). Der kompatible Value-Eingang bleibt. Globale Arten sind vorhanden; modifiers ist sprachunabhängig. Vollständigkeit des sechsfachen Helden-/Itemstands ersetzt nicht die Verfügbarkeitsprüfung zusätzlicher Arten je Rechnung.

asset_mirror ist im geprüften G-Baum und lokal gespeicherten main noch nicht vorhanden. Die committed Lieferung ist deshalb noch kein G-Anschluss oder Livebeweis. G verändert weder diesen Leser noch Is SQL-/Importbereich. Fs committed Loader wählt bei data.rs:728-786 Payloads und Herkunft noch getrennt; I bindet den tatsächlichen Receipt in seinem Bereich an.

## F-Buildvertrag bleibt konkret offen

Fs vorhandener reiner Eingang plan_build_with_playstyle in dbrain-reasoner/src/lib.rs:110 nimmt HeroModel, Items, MetaIndexWithSources, Patchdaten, ReasonerConfig und optionalen Spielstil. Spielstil wird tatsächlich angewandt; kein AI-, Netzwerk-, Persistenz- oder Publishaufruf in dieser reinen Strecke.

Es fehlen im geprüften F-Stand Toolbudget, gewünschte Imbues und ursprüngliche RequestDeadline. PlannedBuild hält die strukturierten PurchasePlan-/InventoryEvaluation-Belege noch nicht fest; variant_scores bleibt leer. Die bestehende Kaufauswertung in composer.rs:409,433,530 wiederverwenden. Eine zweite Planungsrechnung ersetzt die ursprünglichen Ergebnisbelege nicht.

validate_build_plan in tools.rs:909 verweigert ohne echten Prüfanschluss. Kernel prüft tatsächlichen Request, Ergebnis, Pin und Zweck in lib.rs:205-258 und erneut bei Wiederverwendung (:397-408). Kataloglegalität und alter Buildabruf sind kein deterministischer F-Verifier. Is committed Übergabe lässt diese F-Lieferung ausdrücklich offen. G darf sie nicht durch einen zweiten Planer oder Erfolgsshim ersetzen.

## Herkunft, Rechenbindung und Analytics

- calculation_models_from_payloads in data.rs:736 und calculate/project/growth/curve-Eingänge in calculation.rs:778,787,1876,1961 sind vorhandener S3-WIP. ModelSource in types.rs:599 bindet Version/Dokument/Pointer, aber keine echten Receipts oder Quellenrechte.
- Profilprojektion in brain-storage/src/entity_profile.rs:44-64 akzeptiert bisher Wiki/GameFile; Vertragsenum in brain-contracts/src/entity_profile.rs:17. Echte Assets-Herkunft ist hier G-eigene Anschlussarbeit.
- Kanonische Prüfbausteine existieren in local_pg_reader.rs:1153,1208,1246 und brain-contracts/src/store.rs:26,260,361. Dieselben Prüfungen für Receiptbelege nutzen, keinen zweiten Rechtepfad.
- PinnedGameContext in tools.rs:776 enthält Version, Sprache und Mechanikrevision, bisher keinen Run oder Manifest. Folgewerkzeuge müssen denselben echten gewählten Run binden.
- brain-serve/src/analytics.rs:303-335 nutzt den vorhandenen Client mit ursprünglicher Deadline für Hero-/Item-/Patch-/Zeitfenster. provider-Egress wird bei :472-492 noch verweigert. Rang-/Fensterselektoren im Toolvertrag sind noch kein Nachweis ihrer tatsächlichen Laufzeitwirkung oder gemeinsamer Vergleichspopulation.
- analytics_runtime wurde im Vorcheck nicht gelesen oder verändert. I hat dessen Eigentum in der geprüften Übergabe nicht freigegeben. Kein zweiter Analyticsconnector über dbrain-builds/api.rs.

## Minimaler eigener Bauumfang

G-eigen: bestehender Retrievaladapter in dbrain-retrieval/src/lib.rs, Assets-Profilrepräsentation in contracts/storage entity_profile.rs, nötige kanonische Leserfreigabe in local_pg_reader.rs und vorhandene Analyticsfläche in brain-serve/src/analytics.rs. Keine parallelen Reasoneränderungen während S2. K behält service.rs, konkrete Pool-/Kernelverdrahtung und Consumer; I behält Receipt/SQL/Import und Fs Planer. Retrieval hat bislang keine normale Kernelabhängigkeit; einen konkreten GameContextResolver dort erst mit ausdrücklich begrenzter kompatibler Anschlussentscheidung bauen.

## Produktionsbeweis

Echter /v1/answer-Weg mit konfiguriertem Provider und tatsächlichem Werkzeugaufruf. Ein serverseitig gewählter Spiegelrun, gebundene Originale, derselbe Rechenkern, wirklich angewandte F-Planparameter und deterministische Ergebnisprüfung. Alle Abhängigkeiten einschließlich unzitierter Belege bei Cache/Flight unter ursprünglicher Deadline und kumuliertem Budget erneut prüfen. Toolplanung ohne AI, Persistenz, Steam-Publish oder erfundene Build-ID.

Bereinigter Antwortkontext unter intern geprüfter tatsächlicher Person-/Kanalsicht darf gemäß aktueller Luna-Testfreigabe weitergegeben werden. IDs, Mitgliederlisten, fremde Personendaten und private Originale NEVER an Codiermodelle oder Git. Dieser Vorcheck ist kein Produkt-, Datenschutz- oder Liveabschluss.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/brain-contracts/src/tools.rs:874 | Anknüpfung: vorhandene Tools/Kernel, committed rungebundener Spiegelleser, reiner F-Planer und kanonische Quellenprüfung
