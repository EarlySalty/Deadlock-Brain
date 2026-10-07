# G-V: vorhandenen Produktionsport wirklich anschließen

## 1. Ziel und Voraussetzungen

Fortsetzung des ursprünglichen G-Auftrags, kein Neubau. Startvoraussetzung erfüllt: wbhr8f9ck / wf_b82d8d03-5e4 tatsächlich abgeschlossen und durch G konsumiert. Gesichertes S3 2b519b0470fdb2d0ae9fde7791dff0c820ad3d1b, gesichertes S4 und Start-HEAD a35bd8142146bd1ea0d8b342625ac6cf2e0bd278. Beide vollen Gruppengates mit gpt-6.1-sol ALLOW; committed Compiler/striktes Clippy/Format je Exit 0. 38 Rechenfälle bestanden, gesamte S4-Suite 342/12 gegen abgeschlossene Baseline 304/12 mit identischen Fehlernamen. Origin-a35bd814 durch G selbst bestätigt, S3-Ancestorprüfung Exit 0, Rustbaum sauber. S2 e58a5c59 bereits vollständig gegen S1 ALLOW. Noch kein Produktionsport oder Livebeweis.

Nutzer meldete inzwischen I-Spiegel b7289d11 nach echtem Test und Push auf main, Releasebau begonnen. Das ist weder Gs Fetchbeweis noch ein Import-/Livebeweis. Vor Verwendung frisch geordnet den tatsächlich gesicherten aktuellen Stand prüfen.

Den bestehenden Acht-Tool-Vertrag als echten ToolExecutionPort liefern. GameContextResolver muss vor Cache-/Flightschlüsselbildung einen serverseitig gewählten tatsächlichen Spiegelstand binden. Bestehende Provider, Kernel, Konverter, Reasoner und F-Planer verwenden. Kein zweiter Leser, Parser, Planer, Connector oder Antwortweg. K bekommt einen kompilierbaren Konstruktor-/Resolveranschluss mit geprüftem SHA, nicht bloß eine Traitbeschreibung.

Aktuellen origin/main vor Integration frisch prüfen. E/I-Receipt und F-Eingang aus tatsächlich gesicherten Commits geordnet übernehmen, keinen fremden WIP. Keine Wartepflicht auf deren Merges; eine wirklich fehlende Signatur oder Eigentumsübergabe bleibt ein präziser Vertragsblocker. Alte Referenzen 9d17ee52/501d3725/46fd8674 sind nur geprüfte historische Stände. G-V-FORTSETZUNG-BESTAND.md und ANFORDERUNG-G-V.md enthalten den tatsächlichen früheren Befund, keine aktuelle Fehlensbehauptung.

## 2. Eigentum

G-V-Bereich gemäß bestehendem PLAN.md: brain-storage/src/entity_profile.rs und local_pg_reader.rs, dbrain-retrieval/src/lib.rs, freigegebenes brain-contracts/src/entity_profile.rs und brain-serve/src/analytics.rs. Zugehörige eigene Anschlussfälle und eigene Nachweise. Nur nötige kompatible Anbindung, keine globale Formatierung oder zusätzlichen Refactorings. Vor Bestandssuche code-suche/Graphify und vorhandene Nebenpfade prüfen.

brain-contracts/src/lib.rs und provider_input.rs sind nach ENTSCHEIDUNG-K-ORTSVERTRAG.md exklusiv K bis gesicherter kompatibler Übergabe. ENTSCHEIDUNG-K-ORTSVERTRAG-ERGAENZUNG.md reserviert zusätzlich query.answer_context im vorhandenen flight.rs/cache_key_for_purpose-Tupel samt passenden Orts-Cachetests sowie ausschließlich die ausdrücklich gelisteten mechanischen Query-Literalergänzungen. Diese Stellen nicht parallel bearbeiten. Keine zweite Orts-/Cache-/Literalimplementierung. Bereits gesicherten kompatiblen K-Commit geordnet übernehmen; keine fremden WIP-Kopien oder flächige None-Ergänzung. K besitzt zentrale service.rs-Verdrahtung, Consumer und ID-freie Modellprojektion. Zusätzlicher Cache-/Flight-Dateibedarf wird konkret vor gemeinsamem Schreiben gemeldet. Kein aktiver G-Writer besitzt derzeit eine dieser neuen K-Stellen: wbhr8f9ck bleibt auf S3/S4-Reasoner begrenzt.

Is asset_mirror, Import, Receipts und SQL nicht ändern. Fs Loader, Planer, Composer und Publish nicht ändern. analytics_runtime samt Schema/Testvertrag bleibt ohne dokumentierte I-Übergabe geschützt. Keine Arbeit an Q, Bots, Invites, Concierge, ai-coach oder fremden Diensten. Für einen wirklich fehlenden zusätzlichen Anschluss exakten Pfad und kleinste kompatible Änderung begründen, keine pauschale Dateifreigabe erfinden.

## 3. Arbeitsstand und gemeinsame Verträge

Worktree /home/nathanael/.worktrees/brain-g-v2-20261007, Branch feat/brain-v2-g-20261007. Tatsächlichen HEAD, Index und aktiven Writer vor Änderung prüfen. Alte eigene Akten und sämtlichen früheren WIP erhalten. Git exklusiv, einzeln mit literalen absoluten Pfaden, nur eigene Dateien, kein add -A, Reset, Stash oder Forcepush. Commit-Trailer Co-authored-by: GPT 6.1 Sol <modell@local>. Parent während Worker-Gitarbeit ohne Git-Schritte.

ToolExecutionPort und GameContextResolver sind im vorhandenen Vertrag synchron. Nicht spekulativ durch einen zweiten async Kernel ersetzen. Vorhandenen DB-/Runtimeanschluss und ursprüngliche RequestDeadline verwenden; keine neue Runtime oder neues Budget je Turn. dbrain-retrieval hat bereits normale brain-storage-, dbrain-reasoner- und sqlx-Abhängigkeiten, aber keine normale brain-kernel-/tokio-Abhängigkeit. Keine unzulässige Manifest- oder fremde Serviceänderung zur Abkürzung.

Receipt bindet tatsächlichen vollständigen Run, Manifest und Endpoint, Originalhash, URL, Art/Sprache, Parserrevision und Zeiten an die gelesenen Payloadbytes. ModelSource allein ist kein Receipt oder Quellenrecht. Pin, Modellbildung, Rechnung, Toolresult und Neuprüfung müssen denselben Stand verwenden. Version ist kein bestätigter Balancepatch. Kanonische Rechteprüfung aus vorhandenem Store/LocalPgReader wiederverwenden; InternalRead, Modellweitergabe und ForPublication bleiben verschieden.

F muss Toolbudget und gewünschte Imbues tatsächlich anwenden, ursprüngliche Deadline konsumieren und die ursprünglichen PurchasePlan-/InventoryEvaluation-Belege liefern. validate_build_plan prüft das wirkliche Ergebnis gegen Request, Pin und Zweck. Kataloglegalität, leerer Erfolg, alter Buildabruf oder zweite Nachrechnung ersetzen diesen Verifier nicht. Wachstum nutzt Gs gesichertes project_hero/hero_growth/compare_hero_curves.

Profile und game_rules verwenden dieselbe Regelansicht. Analytics verwendet den vorhandenen Client, tatsächlichen Rang-/Zeitfilter und gemeinsame Vergleichspopulation. Patchmitgliedschaft bleibt ohne Beleg Unverified; keine Einzelmatchablage. Ohne nötige Runtimefreigabe fehlende Anwendung melden, nicht vortäuschen.

## 4. Beweisziel und Gate

Cargo ausschließlich /home/nathanael/.local/bin/cargo-slot, bestehender zulässiger Targetweg, locked/offline und gedrosselte Jobs. Passende Compiler-, Format-, strikte Clippy- und bestehende Testprüfungen mit exakten Zahlen, Befehlen, Exits und committed Quellenbindung. Keine Wall-Clock-Tests oder Fakes für Receiptleser, tatsächlichen Receiver oder F-Verifier. Vor Altfehlerbehauptung abgeschlossene vergleichbare Zahlen-/Namensbaseline; rote fehlende DB-/Livevoraussetzungen getrennt dokumentieren.

Einziger Reviewer ist gate_hook.py --review. Kohärente eigene Anschlussgruppen mit tatsächlichem Base/Head und gpt-6.1-sol prüfen; bei echtem BLOCK frischer nativer Fixerkontext durch den steuernden Workflow. Kein Delta-ALLOW als gemeinsames ALLOW ausgeben. Kein anderer Reviewthread, Modellwürfeln, Hook-/Rechte-/Wrapperumbau oder Schutzumgehung. Scopeblocker, fehlender Vertrag und Werkzeugausfall getrennt klassifizieren.

Produktion wird erst durch den echten /v1/answer-Weg bewiesen: tatsächlicher Toolcall, wirklicher Spiegelrun und Originale, gemeinsamer Rechenkern, tatsächlich angewandte F-Parameter, deterministischer Verifier und erneute Prüfung sämtlicher Abhängigkeiten. Unitfixtures oder HTTP 200 allein reichen nicht. Merge/Deploy/Restart/Livebeweis/Cleanup sind der anschließende eigene Gesamtabschluss, kein Abschluss allein wegen dieses Portbaus.

## 5. Routing und Datenschutz

G ee3de2ba-30ab-4558-a57c-6c1de154891e, Delegator 481426fe-b477-42b3-91c6-901811fcba1d. G ist alleiniger Statusproduzent. Keine TODO-/REGISTER-/zentrale Aktenänderung durch Worker. Eigene Belege unter G/pruefungen/g-v-produktionsanschluss/runde-N/. Normale Textrückgabe mit tatsächlicher Signatur, SHA, Rohgate, Zahlen, Grenzen und Originbeleg; kein StructuredOutput voraussetzen. Kein neuer T3-Thread, keine fremde Sessionkoordination und keine doppelte Writerinstanz.

Secrets NEVER ausgeben. Private Originale und Community-Rohdaten MUST NOT an Codiermodelle oder Git. Luna-Testfreigabe ausschließlich für minimalen bereinigten Antwortkontext unter tatsächlicher Sichtbindung, ohne Discord-/Steam-IDs, Mitgliederlisten und fremde Personendaten. Intern geprüfte Identitäts-, Rechte- und Herkunftsbindungen NEVER löschen oder aus untrusted answer_context ableiten. Gesamten tatsächlichen Providerpayload und dessen Budget berücksichtigen; K besitzt die Service-/Consumerprojektion. Kein neuer Provider oder eigenmächtiger Modell-/Timeoutwechsel. Nur Rust und Postgres. Browser nur Moli nach Guide, Brave MUST NOT benutzt werden. Kein Settle bei offener Arbeit.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 30 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
