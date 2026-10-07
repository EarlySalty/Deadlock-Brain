# Herkunft und vorhandene Belege der zwei D-Erntecommits

Auf direkten Orchestratorauftrag nur lesend geprüft, keine Main-/Runtimeoperation. Einziger Build-/Install-/Neustart-/Tickeigentümer bleibt live_strecke. A hat seit dem Hold keinen Main-Push ausgeführt. Frisch geholtes Remote-main ist 75db93ef91010ccfe6c3d501eb2e7107e79f3c12, nicht mehr 0ade1a3. Das ist eine Herkunftsmessung, keine Freigabe oder Auslieferungsbehauptung.

## baca936e7d121a9b44f4a81aa914fbe91093b3e6

Git direkt gelesen: Parent 8d61a949c9856a69543747b0e59dbfb5bbcbe440. Committer/Autor EarlySalty, Attribution GPT 6.1 Sol, Commitzeit 07.10.2026 03:03:52 CEST. Paket D bestätigt Herkunft K8/K9 aus selektiver Archiverntung. Ausschließlich drei Testdateien, kein Produktdiff: brain-api/tests/publication.rs, brain-api/tests/review_deadlines.rs, dbrain-retrieval/tests/core_retrieval.rs. 94 Zeilen hinzugefügt, 3 entfernt.

Vorhandener regulärer Gate in D/REVIEW.md:3-17, Basis 8d61a949, Head baca936e. Wortlaut:
[gpt-6.1-sol] ALLOW: No merge-blocking defects found in the supplied diff and revision snapshots.

Prüfbelege laut D/REGISTER.md:20-21 und AN_HAUPT-D.md:59-73: 34 bestandene Prüfungen nach Rebase (19 Publication, 10 echtes HTTP, 5 Retrieval), 0 ignoriert. Unveränderte fde-Retrievalbaseline 0 passed/4 failed wegen ungültig kurzer Fixturehashes. Normales Clippy grün mit sichtbarer release_port.rs:510-Warnung, strikt dort rot; keine Warnungsunterdrückung. A hat diese Suites nicht neu ausgeführt, die vorhandene Akte und Commitgrenze gelesen. Erstes Pushkommando wurde am R10-Rollenhinweis gestoppt, Rollenpfad gelesen, normaler zweiter Push ohne Hookbypass.

## 0ade1a3dc881e466a15d81550a560dc512598f2f

Git direkt gelesen: Parent baca936e. Committer/Autor EarlySalty, Attribution GPT 6.1 Sol, Commitzeit 07.10.2026 03:11:22 CEST. Paket D, K10. Zwei Reasonerdateien: src/lib.rs und src/playstyle.rs, 343 Zeilen hinzugefügt, 8 entfernt. Selektiver generischer Weapon-/Spirit-/Tank-Einstieg, kein bereits verdrahteter Discord-/Twitch-Buildskill und kein Publishbypass.

Vorhandener Gate D/REVIEW.md:19-41, Basis baca936e, Head 0ade1a3. Wortlaut:
[gpt-6.1-sol] ALLOW: No blocking defect is established by the supplied diff.

Zwei erhaltene NITs: Composer-Blockfilter im Snapshot nicht gezeigt sowie nur Tank als Planintegration geprüft. D belegt vorhandenen Composer und unveränderte Publishgrenze nochmals; zusätzliche Weapon-/Spirit-Planfälle wurden nicht als ausgeführt behauptet.

Prüfbelege laut D/REGISTER.md:22 und AN_HAUPT-D.md:47-67: Compiler, striktes Clippy aller Reasoner-Targets und fünf eigene Prüfungen grün. Vollsuite 284 passed/6 failed/0 ignored, unveränderte Baseline 279 passed/dieselben 6 failed/0 ignored. Vier Scratch-Fixturefehler ohne hero_build_id, zwei veraltete September-Konstanten gegen Echtdaten. Keine pauschale grüne Vollsuite. A hat diese vorhandenen Zahlen nicht neu gemessen.

## Aktueller Hold und neue Fachgrenze

D/REGISTER.md und AN_HAUPT-D.md dokumentieren die Abweichung: beide Main-Pushes erfolgten, bevor D den in A-Akten veröffentlichten Hold entdeckte. D stoppte seinen eigenen Releasebuild am 07.10.03:32 und bestätigt seitdem keine Main-/Runtimeoperation. Keine eigenmächtige Rücknahme durch A oder D.

D bestätigt am 03:52 live_strecke als exklusiven Eigentümer und gemeldete finale Source 75db93ef. Am 03:55 übernimmt D auf direkten Root-Auftrag eng den neuen playstyle.rs-Resolverfehler auf separatem Feature von 75db93ef, mit bestehendem mechanics::weapon_spirit_scaling statt eigener Aliasprüfung. Dieser neue Fix ist nicht Bestandteil der alten K10-ALLOWbehauptung. A baut diesen D-Fehler nicht parallel. ENV-Fix bleibt ausschließlich bei A-R2 auf Feature. Keine Main-Pushes, kein Releasebau, Install, Neustart, Tick oder Writer-/Lunaänderung aus A.

Die hier genannten Gate-/Prüfbelege sind vorhandene D-Akten, keine neue unabhängige A-Codeprüfung oder Lesebestätigung der Hauptsession. Rohlogpfade standen in den gelesenen D-Akten nicht bei. Kein neuer Reviewer gestartet.
