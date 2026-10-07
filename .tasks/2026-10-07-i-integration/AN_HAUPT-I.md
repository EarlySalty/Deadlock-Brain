# Paket I: qualifizierte Fachrückgabe vom 07.10.2026

## Urteil

Der fünfte weitere Fortsetzungs-BLOCK ist erreicht. Der letzte konkrete Reader-Befund ist jedoch widerlegt, nicht als Produktbug bestätigt: Der bestehende INNER JOIN auf das angefragte Endpoint-Dokument greift vor ORDER BY/LIMIT. Ein späterer vollständiger Core6-Run derselben Clientversion verdeckt die globalen Assets des älteren vollständigen Runs nicht.

Eine zusätzliche echte Scratch-PG-Gegenprobe mit 25 Stunden Abstand besteht für alle 13 Endpoint-/Sprachkombinationen, sowohl Value als auch Receipt. Globale Receipts nennen den älteren tatsächlichen Run; explizites Pinnen auf den neuen Core6-Run lehnt dessen fehlende globale Endpoints korrekt ab. Nur Tests ergänzt, keine Produktimplementation verändert. Der BLOCK bleibt wirksam. Kein anderer Reviewer, kein Gate-Override und kein weiterer Urteilslauf. Die Core6-Kompatibilität darf nicht gegen die Spec aufgehoben werden.

Vorschlag für die fachliche Fortsetzung: Die gesicherte Gegenprobe dem bisherigen Urteilmodell im bestehenden Gate zur Klärung dieses konkreten Gate-/Spec-Konflikts geben. Keinen zweiten Leser oder eine neue Pipeline bauen. Nach Klärung erst den vollständigen E-Produktstand integrieren und live abschließen; anschließend F gemäß bestehendem Auftrag schließen. Dies ist eine Blockerrückgabe, keine neue Implementierungsfreigabe.

## Gesicherte Stände

- Belegintegration `17974c66` und Schemapinintegration `ca4d877f`: jeweils separat ALLOW und tatsächlich nach main gepusht. Diese erlaubten Teilstufen nicht zurückrollen.
- Vollständiger Produktkandidat `b63569afbf2bc6f686564063d769dbbcf0a5ef90`: Produkt-Gate BLOCK, nicht nach main gepusht. Auf `origin/feat/brain-i-integration-blocked-20261007` gesichert, eigener Arbeitsbaum `/home/nathanael/.worktrees/brain-i-release-20261007`.
- E-Produktbasis `5f4e3668acb4531cfd31efdaa88e63626f772f84`, eigener Branch `feat/brain-deadlock-api-daten`. Tree vor Gegenprobe identisch zum Produktkandidaten: `c1d4b6a250737142b2f97f240bbad8c7e42d304b`.
- F unverändert auf `46fd86743589910d7b92a7223bdd6ab0dcf2b7c8`. Compiler und 36 reine Composer-/Planner-Baselinefälle bestanden; kein F/G-Vertragsanschluss daraus ableiten.

## Prüfbelege und aktuelles Gate

Vollständige E-Integrationssuite vor der zusätzlichen Regression: 553 passed, 0 failed, 24 ignored, 32 Testtargets, Harness-Exit 0. Format und striktes Clippy einschließlich `brain-serve` Exit 0. Originale `/tmp/brain-i-e-main-integration-tests.log`, `/tmp/brain-i-e-main-integration-fmt.log`, `/tmp/brain-i-e-main-integration-clippy.log`.

Finale Gegenprobe: 1 passed, 0 failed, 0 ignored, 225 filtered, Harness-Exit 0. Format und striktes Clippy Exit 0. Vollständiger Befehl und Aussagegrenzen in `.tasks/2026-10-07-i-integration/NACHWEIS-CORE6-GLOBAL.md`; Original `/tmp/brain-i-core6-global-gate-reproduction-final.log`. Die kontrollierte PostgreSQL-Probe ist kein Produktivimport.

Letztes Urteil unverändert von Claude Opus 5.5 gegen `ca4d877f..b63569af`, Exit 1:

```text
BLOCK: Ein späterer Teilimport verdeckt weiterhin vorhandene Assets derselben Clientversion.
```

Vollständige Antwort und Verifikation in `.tasks/2026-10-07-i-integration/REVIEW.md`, Original `/tmp/brain-i-e-product-integration-gate.log`. Die daneben genannte NIT-Frage zu harmlosen Links in Originalpatches ist durch die Gegenprobe nicht geprüft oder geschlossen. Die fünf weiteren erfolglosen Fortsetzungsurteile sind Runden 9, 10, 11, 12 und 14. Kein Fixer 11 gestartet. Zehn bisherige native Fixer sind beendet.

TESTNACHWEIS[TW-1]: 553 passed, 24 ignored | Baseline: keine Altfehler behauptet
TESTNACHWEIS[TW-1]: 1 passed, 0 ignored | Baseline: zusätzliche Reader-Gegenprobe, keine Altfehler behauptet

Zählbereich des folgenden Mergeprotokolls ist ausschließlich die gestufte Main-Integration: Beleg-restore/add/commit/push, Schemapin-merge/push und Produkt-merge, sieben einzelne Git-Schritte. Drei zugehörige Integrationsurteile. Frühere Fixer- und spätere reine Sicherungsschritte sind nicht darin enthalten.

MERGEPROTOKOLL[MS-1]: 7 Git-Schritte einzeln | Anläufe: 3 | Gate: Belege ALLOW, Schemapin ALLOW, vollständiger Produktstand BLOCK

## Offener Abschluss

Kein regulärer Releasebuild/install, eigener Neustart, vollständiger produktiver Assets-/Patch-/Builddatenimport oder Live-Receipt-/Originalhashbeweis. Die Rust-Liveprobe wurde gebaut, aber nicht ausgeführt. Letzter Prozessvorcheck: PID 2645590, tatsächlich laufendes `brain-serve` unter Release `bfda408cb988722ddceadb56bca5b72e12d12731`. Keine frische Health-/Ready- oder Journalprüfung aus dieser Gegenprobe ableiten. Der alte Wrapperplan auf `3ceb504d` ist nach den Teilstufen historisch und vor einem späteren Deploy neu zu erzeugen.

LIVEBEWEIS[DV-1]: PID 2645590->nicht erhoben | exe ohne (deleted) zuletzt vorgeprüft | journal -p err nicht geprüft | Anker "nicht geprüft" in Binary | Funktion: kein eigener Deploy oder vollständiger Liveimport bewiesen | Ort: http://127.0.0.1:8788/readyz, nur früherer Vorcheck

Keine analytics_runtime-Freigabe. F-Budget/Imbues/ursprünglicher Abbruch und dieselben PurchasePlan-/InventoryEvaluation-Belege bleiben offen; G-Rechenkern nicht dupliziert oder fremder WIP übernommen. Keine neue Warden-Veröffentlichung oder hero_build_id. Kein Cleanup und kein Self-Settle. Eigene Branches und Arbeitsbäume bleiben für die Fortsetzung erhalten. Fremder kanonischer WIP bleibt unberührt; nur der ausdrücklich zugewiesene Hauptbericht wird dort ergänzt.
