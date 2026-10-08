# G: Verbraucherfehler nach G-K-R3

status: ausgewertet, ein eigener Fixturefix nötig, 07.10.2026

Lauf `G/pruefungen/g-k-r3/consumers.log`: 414 passed, 14 failed, 0 ignored, 0 filtered; Exit 101. Die Bereichsführung verglich konkrete Fälle mit dem tatsächlichen ursprünglichen `test-cases.json` im kanonischen Baseline-Rohbeleg. Es gibt keinen passenden vollständigen Vorhervergleich der drei Verbrauchercrates. Der ursprüngliche Serve-Lauf war 61 passed/1 failed mit fünf ausdrücklich ausgeschlossenen Fällen und vorbereiteter privater PostgreSQL-Umgebung.

## Bestätigte eigene Testregression

`discord_live::tests::grosse_live_panels_und_doku_passen_gemeinsam_ins_providerbudget`: ursprünglich `ok` in `test-serve`, jetzt `InvalidResponse("invalid chat schema")` an discord_live.rs:833. Quellprüfung nach Graphify bestätigt: Loopbackantwort an discord_live.rs:785 enthält keinen finish_reason. Der neue konkrete kompatible Parser verlangt ihn als String in transport.rs:594 und wertet ihn in Zeile 673 aus.

Urteil: vorhandene Testantwort an den jetzt vollständigen Turnvertrag anpassen, ohne die Produktionsprüfung zu lockern. Abschlussgrund `stop` im bestehenden Antwortfixture ergänzen; Budget-, Pack-, Inhalts-, Freigabe- und Netzrundenassertionen erhalten. Kein Produktbudget oder Testmaterial verkleinern. Eng begrenzten frischen Fixer nach tatsächlichem G-K-R4-Abschluss starten, damit Featurecommits seriell bleiben.

## Fehlende Laufvoraussetzungen, keine Altfehlerbehauptung

| Fälle | Tatsächlich beobachtete Ursache | Aussagegrenze |
| --- | --- | --- |
| Vier pilot_phase-Fälle | pilot database required: NotPresent | Ursprüngliche Baseline schloss sie aus. Kein Regression-/Altfehlervergleich. |
| private_scratch_scram_accepts_runtime_password_and_rejects_wrong_password | scratch socket required: NotPresent | Ursprünglich bestanden mit vorbereiteter privater PostgreSQL-Umgebung. Fehlende Umgebung im jetzigen Lauf ist kein bewiesener Produktfehler. |
| binary_loopback_health_readiness_shutdown_and_no_fallback | scratch socket required: NotPresent | Ursprünglich anderer Fehler: release_unavailable statt knowledge_version_mismatch. Gleicher Name ist kein Beweis gleicher Ursache. |
| runtime::tests::isolated_connection_identity_is_checked_before_import und scratch_raw_ir_facts_release_delta_reparse_and_acl | expliziter Scratchsockel beziehungsweise Scratchwrapper fehlt | Vorhervergleich dieser Sources-Fälle liegt nicht vor. |
| Zwei öffentliche Forum-/Browserfälle | Brave-Debugverbindung fehlgeschlagen | Keine passende Sources-Baseline, keine Parserregression daraus abgeleitet. |
| real_catalog_and_git_documents_report_coverage | reguläres Infisical-Runtime-Credential fehlt | Keine passende Baseline, keine Produktions-DB dafür verwenden. |
| oeffentliche_live_fakten_ueber_bestehenden_resolver | SecretSource | Ursprünglich ausdrücklich ausgeschlossen, kein Altfehlernachweis. |
| live_small_current_assets_contract | erforderliche Freigabe fehlt: Err(NotPresent) statt Ok("1") | Keine passende Baseline. Explizite Probe nicht als gelaufen oder grün melden. |

Für Gesamtbeweise die vorhandenen privaten Testwrapper und zulässigen Livevoraussetzungen verwenden. Fehlende Secrets nicht aus Logs oder Konfiguration im Klartext ausgeben. Keine Produktions-DB, neuen Ignorierungen oder pauschalen Skipfixes. Dieser Bericht ist Ursachenabgleich, kein neuer Testlauf und keine vollständige Verbraucherabnahme.
