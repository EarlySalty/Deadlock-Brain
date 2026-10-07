# Paket D an Haupt-Orchestrator

## 07.10.2026, 04:12 CEST: Root meldet Integration `bfda408c`, Main-/Betriebshold bestätigt

**An Hauptkoordinator `3fcd8f71-443e-48ae-825c-527eb52fbe56`:** Der enge Reasoner-Fix ist als Featurecommit `bfda408cb988722ddceadb56bca5b72e12d12731` gegen die freigegebene Basis `75db93ef91010ccfe6c3d501eb2e7107e79f3c12` abgegeben und auf `origin/fix/brain-spielstil-resolver-20261007` gesichert. Remote-SHA und einziger Parent unabhängig abgefragt, eigener Sourcezustand sauber. Genau eine Produktdatei verändert.

97 gezielte Tests bestanden, 0 fehlgeschlagen, 0 ignoriert; fmt-Check und striktes Clippy aller Reasoner-Targets Exit 0. Eigener exakter Selfgate `[gpt-6.1-sol] ALLOW: No grounded blocking defect in the supplied diff.`, Exit 0, Rohlog `bhemwvxfm.output`. Der NIT zum fehlenden Resolverkontext ist laut Root durch die unabhängige Prüfung des unveränderten Mechanics-Resolvers und der Gegenproben abgedeckt. Exakte Befehle, Filterzahlen und Logorte in `D/RESOLVERFIX.md`; Gate wörtlich in `D/REVIEW.md`; `D/REGISTER.md` aktualisiert.

Root meldet ausdrücklich: frische unabhängige SHA-Abnahme, SHA-identische saubere Vorwärtsintegration dieses Commits auf `origin/main`, Finalsource an `live_strecke` übergeben. Diese Root-Integration ist keine eigene D-Main-/Liveprüfung. **D bestätigt den Main-/Betriebshold auf `bfda408c`: kein weiterer Main-Push, Releasebau, Install, Neustart oder Tick.** Nur `live_strecke` besitzt die Standardbuild-, Installations- und Tickstrecke. Luna-Abo 18769 und Timerfence bleiben unverändert. Keine zusätzliche Review- oder Buildkette; fremde Branches unverändert.

Prüf-/Gateartefakte und Featurequelle bleiben für die gemeinsame Schlussabnahme erhalten. Kein Gesamtfertigbeleg oder Self-Settle. Weitergabe an Haupt über diese Akte; keine direkte Sessionnachricht oder Lesebestätigung behauptet.

TESTNACHWEIS[TW-1]: 97 passed, 0 ignored | Baseline: n/a rot

Keine neue Baseline gemessen; keine Altfehlerbehauptung für diesen engen Fix.

MERGEPROTOKOLL[MS-1]: 13 Git-Schritte einzeln | Anläufe: 1 | Gate: [gpt-6.1-sol] ALLOW (Selfgate; kein Mainmerge durch D)

## 07.10.2026, 04:09 CEST: enger Resolverfix committed, Selfgate läuft

Featurecommit `bfda408cb988722ddceadb56bca5b72e12d12731` auf `fix/brain-spielstil-resolver-20261007`, Basis `75db93ef91010ccfe6c3d501eb2e7107e79f3c12`. Genau eine Datei: `dbrain-reasoner/src/playstyle.rs`, kanonischer Resolver plus vier Gegenproben. Kontextfix und fremde Dateien unverändert.

97 gezielte Tests grün: 9 Spielstil einschließlich der vier Gegenproben, 42 Mechanics, 41 Combat, 5 Item; jeweils 0 fehlgeschlagen und 0 ignoriert. Paketweiter fmt-Check und striktes Clippy aller Reasoner-Targets (`-D warnings`) Exit 0, Clippy 1m 59s. Finale Spielstiltests aus dem frisch kompilierten Testbinary zusätzlich 9/0/0 bestätigt. Exakte Befehle und Logs in `D/RESOLVERFIX.md`, keine neue Vollsuite oder Baseline behauptet.

Regulärer eigener Selfgate auf diesem exakten Commit gegen `75db93ef` läuft jetzt, Task `bhemwvxfm`; noch kein Urteil vorweggenommen. Feature noch nicht gepusht oder als endgültig freigegeben gemeldet. Kein zusätzlicher umfassender Review, kein Main-Push oder Betriebseingriff. Root übernimmt Nachabnahme/Integration, `live_strecke` weiterhin alleiniger Betriebseigentümer.

## 07.10.2026, 03:55 CEST: Ownership und Beginn des engen Reasoner-Fixes

D übernimmt auf ausdrücklichen Root-Auftrag ausschließlich den Fehler in `dbrain-reasoner/src/playstyle.rs` und die betroffenen Tests. Neue saubere Featurebasis `75db93ef91010ccfe6c3d501eb2e7107e79f3c12`. Der vorhandene `mechanics::weapon_spirit_scaling`-Resolver ersetzt die eigene Substring-/Aliasprüfung; geprüft werden dessen drei effektive Koeffizienten. Widersprüchliche Direktwerte/Aliase und unbekannte Schlüssel erhalten gezielte Gegenproben. Relevante Bestandstests, Format, Clippy und eigener regulärer Selfgate folgen vor Featureabgabe.

Kein paralleler zweiter Fix erwünscht. Kontextfix und fremde Änderungen bleiben unangetastet. Kein Main-Push, Releasebau, Install, Neustart oder Tick. Abgabe ausschließlich als Featurecommit mit Gate- und Prüfbelegen an die meldende Hauptsession und unseren Hauptkoordinator `3fcd8f71-443e-48ae-825c-527eb52fbe56` über diese Akte. Integration erst nach deren frischer Nachabnahme; `live_strecke` hält die finale Releasekette. Luna 18769 und Timerfence unverändert.

## 07.10.2026, 03:52 CEST: Release-Hold ausdrücklich bestätigt und an Haupt weitergegeben

**An Hauptkoordinator `3fcd8f71-443e-48ae-825c-527eb52fbe56`:** Der Orchestrator hat den bisherigen gemeinsamen Release-Hold ausdrücklich bestätigt. Gemeldete finale Mainbasis: `75db93ef91010ccfe6c3d501eb2e7107e79f3c12`. Einziger Build-, Installations- und Tickeigentümer ist `live_strecke` aus der meldenden Hauptsession. Bis Abschluss dieser gemeinsamen Strecke keine weiteren Main-Pushes oder Runtimeaktionen durch D, insbesondere kein Releasebau, Install, Neustart, Tick oder Recovery. Luna-Abo auf 18769 sowie Timerfence bleiben unverändert. Fremde Branches bleiben unberührt.

D bestätigt diesen Hold verbindlich. Seit dem eigenen Buildstop kein eigener Releasebau oder Tick; die detached Quelle auf `0ade1a3d` und das unfertige alte Bundle sind ausdrücklich nicht die neue finale Releasequelle. Keine Installation oder Funktionsabnahme aus D behauptet.

Laut Orchestrator sind die Erntecommits `baca936e7d121a9b44f4a81aa914fbe91093b3e6` und `0ade1a3dc881e466a15d81550a560dc512598f2f` in dieser finalen Source enthalten. Die konkreten eigenen Prüf- und Baselinebelege stehen unverändert in `D/REGISTER.md`, die beiden ALLOWs samt NIT-Einordnung in `D/REVIEW.md`; sie stehen für die Schlussabnahme bereit. Neuer Kontext-Ursachenfix laut Orchestrator: Originalmetadaten vor dem Batch einmal je Dokument geparst und indexiert, Writer-, Original- und Identitätsprüfungen erhalten; Suites, Clippy, integrierter Check und Selfgate ALLOW. Das sind gemeldete Belege, keine neue Messung von D.

Weitergabe gemäß Auftrag über diese gemeinsame Akte, nicht per Sessionnachricht. Keine direkte Zustellung oder Lesebestätigung des Hauptkoordinators behauptet. Kein Gesamtfertigbeleg oder Self-Settle vor gemeinsamer Liveabnahme.

## 07.10.2026, 03:39 CEST: Stop-Hook verlangt fremden kanonischen Gesamtmerge

**BLOCKER, keine weitere Mainoperation:** Nach erfolgreichem Cleanup der drei eigenen Featurebranches verlangt der Stop-Hook zusätzlich einen Gesamtmerge von `feat/brain-rust-cutover-20260919`. Read-only bestätigt: kanonischer HEAD unverändert `2734c2da4e814ff79953e8e825275b0216a6af16`, 21 Commits nicht in `origin/main`, 10 geänderte versionierte und 7 unversionierte Einträge. Diesen Branch hat D nicht angelegt oder als Arbeitsbasis gewählt; er war der bereits verschmutzte kanonische Checkout beim Sessionstart. D hat darin nur die beauftragte gemeinsame Akte geschrieben, keine Produktdatei verändert.

**Hookursache eingegrenzt:** `claude-config/hooks/branch-finish-gate.py:29` erkennt mittels `merge\b` auch das rein lesende Unterkommando `merge-base`. `track()` merkt dann in Zeilen 165 bis 177 den aktuellen Branch des kanonischen Checkouts. Genau die vorgeschriebenen Ancestorprüfungen aus dem Cleanup können dadurch den fremden Branch als Sessionarbeit erfassen. Der aus der Datei gelesene Regex wurde read-only mit dem tatsächlichen Ancestorbefehl in JavaScript gegengeprüft: Treffer bis `git ... merge`; die Kontrollprobe `git ... status --short` trifft nicht. Dies ist eine Regexgegenprobe, kein nativer Python-Hooktest. Hook und Zustandsdateien bleiben unverändert; keinen Hookaufruf zur Manipulation der Stop-Zähler ausführen.

Die verlangte Gesamtintegration widerspricht `BRIEFING-D.md` Punkt 3 („alte tote Basis nie voll mergen“), der selektiven Ernte und dem aktuell dokumentierten Main-/Release-Hold. Deshalb kein Review eines nicht beauftragten Gesamtdiffs gestartet, kein Gesamtmerge, Push, Branchwechsel, Stash, Reset oder Löschen des fremden Checkouts. Kein Hook verändert oder umgangen. Die 21 Commits und fremden Änderungen bleiben erhalten und werden ausdrücklich als offen gemeldet. Haupt muss diesen historischen Gesamtstand im zuständigen Paket bewerten; D weitet den Portierauftrag nicht eigenmächtig aus.

Die drei eigenen Featurebranches sind weiterhin vollständig aufgeräumt. Nur detached Integrationsquelle, unfertiges Releasebundle und Prüfartefakte bleiben für die Übergabe. Betriebsabnahme bleibt offen, kein Self-Settle.

## 07.10.2026, 03:37 CEST: eigene Featurebranches aufgeräumt, Hold unverändert

Auf Stop-Hook-Auftrag die drei eigenen Branches `feat/brain-ernte-sicherheit-20261007`, `feat/brain-ernte-spielstil-20261007` und `feat/brain-ernte-patchbelege-20261007` abgeschlossen: nach frischem Fetch jeweils Ancestor-Exit 0 gegen `origin/main`, SHA-Sicherung in `D/CLEANUP.md`, keine unversionierten Änderungen oder laufenden Prozesse in den Worktrees. Beide ignorierten Rust-Targets unverändert nach `/home/nathanael/.local/state/brain-ernte-pruefartefakte-20261007/{sicherheit-target,spielstil-target}` verschoben. Drei Worktrees ohne Force entfernt, drei lokale Branches und die beiden gepushten Remote-Branches gelöscht. Keine passenden lokalen oder Remote-Referenzen mehr; frisches `ls-remote` ohne Treffer.

Nur die detached Integrationsquelle `brain-ernte-integration` auf `0ade1a3d`, das unfertige Releasebundle und die erhaltenen Prüfartefakte bleiben für die Übergabe. Das ersetzt den vorherigen Plan, die Featureworktrees bis zur Betriebsabnahme stehen zu lassen. Kein neuer Commit, Main-Push, Releasebau, Install, Neustart, Tick oder Recovery. Hold und Zuständigkeit des Live-Agent bleiben unverändert. Kein Gesamtfertigbeleg oder Self-Settle.

MERGEPROTOKOLL[MS-1]: 19 Git-Schritte einzeln | Anläufe: 1 | Gate: nicht ausgelöst (nur Cleanup; frühere Ports ALLOW)

## 07.10.2026, 03:32 CEST: Releasebau gestoppt, Auslieferung gesperrt

**Der vorherige Stand „Releasebau läuft“ ist überholt.** Beim erneuten Lesen der A-Akten hat D den gemeinsamen Hold in `A/REGISTER.md` und `A/RELEASEFENSTER.md` entdeckt und den eigenen Buildtask `bzwtcaa0o` unmittelbar per TaskStop angehalten. Werkzeugbeleg: `Successfully stopped task: bzwtcaa0o`. Kein neuer Build, Install, Neustart, Tick oder Recovery durch D. Keine Reverts oder weiteren Main-Pushes nach Entdeckung des Holds; Luna-Konfiguration und Writerfence unverändert.

**ABWEICHUNG:** Beide Portcommits waren bereits auf Main gepusht, bevor D den Hold entdeckt hat: `baca936e7d121a9b44f4a81aa914fbe91093b3e6` und danach `0ade1a3dc881e466a15d81550a560dc512598f2f`. Der zuletzt von D gepushte Stand enthält den vorherigen A-Stand `8d61a949c9856a69543747b0e59dbfb5bbcbe440`; keine Rücknahme eigenmächtig ausführen. Das ist keine frische Messung des heutigen Remote-HEAD. Die letzte Lektüre von `VON_HAUPT.md` enthält weiterhin die Vorabfreigabe 07.10. 01:45, keinen neuen Hold-Abschnitt. Die spätere Sperre und der einzige Live-Agent sind in den A-Akten dokumentiert; D behauptet keine direkte Nutzerbestätigung oder eigene Recoveryprüfung.

Eigene Nachprüfung um 03:32 CEST: keine passenden Cargo-/Rustc-/Wrapper-/Linkerprozesse über Argumente oder Arbeitsverzeichnis der eigenen Releasequelle bzw. des eigenen Bundles; keine passenden offenen Builddateien und keine von `lslocks` gemeldeten Locks unter diesen beiden Pfaden. Das ist kein Beleg über fremde Builds oder globale Sperren. Bundle `/home/nathanael/.local/state/brain-ernte-release-0ade1a3d-20261007` enthält auf oberster Ebene nur `target`, kein `manifest.json`. Unfertige Artefakte bleiben erhalten, kein vollständiger oder verifizierter Release behauptet.

D bleibt vor Auslieferung und Liveabnahme blockiert. Den endgültigen gemeinsamen Zielstand und die Installation übernimmt gemäß Releasefenster der benannte Live-Agent. Eigene Worktrees, Featurebranches und Prüfbelege bleiben erhalten; kein Self-Settle. K10 ist weiterhin nur der kompatible Reasonereinstieg, kein bereits angeschlossener Discord-/Twitch-Spielstilskill. A behält Dispatch, normalen Publish und die überlappenden Quellenpfade.

## 07.10.2026, 03:18 CEST: beide Ports auf Main, regulärer Releasebau läuft

K10 ebenfalls auf Main: `0ade1a3dc881e466a15d81550a560dc512598f2f`. Selbstgate `[gpt-6.1-sol] ALLOW: No blocking defect is established by the supplied diff.` Zwei nicht blockende Hinweise betrafen den im Gate-Kontext fehlenden Composer und zusätzliche Weapon-/Spirit-Planfälle. Bestehenden Composer erneut gelesen: Blockfilter gilt für Kern und Situationslisten, vollständiger Upgrade-Katalog bleibt erhalten. `publish::validate_publish_input` verweigert einen leeren Kern ausdrücklich; kein leerer Plan kann über den normalen Publishweg veröffentlicht werden. Die zusätzliche Tank-Upgrade-Planprüfung ist grün, Weapon/Spirit sind über die gemeinsame Klassifikation geprüft. Keine zusätzlichen Tests als bereits ausgeführt behauptet.

`brain-release plan` für den sauberen eigenen Integrationsworktree: aktueller Remote-main `0ade1a3d`, Exit 0. Regulärer vollständiger Releasebau läuft, Task `bzwtcaa0o`, Bundle `/home/nathanael/.local/state/brain-ernte-release-0ade1a3d-20261007`. Noch keine Installation oder Neustart aus D. Der Aufruf nutzt die bestehenden Buildslots und Sperren, keinen fremden Build.

Eigene Scratch-DB nach allen Prüfläufen gestoppt. Saubere unveränderte Baseline nach geprüftem Ancestor-Exit 0 entfernt, keine Artefakte darin. Verbleibende Codeworktrees und Branches erst nach Betriebsabnahme aufräumen.

MERGEPROTOKOLL[MS-1]: 2 Git-Schritte einzeln | Anläufe: 1 | Gate: [gpt-6.1-sol] ALLOW

Protokollumfang K10: eigener Fast-forward-Merge und regulärer `HEAD:main`-Push. Skilldispatch und Funktionsabnahme für Discord/Twitch bleiben bei A, keine komplette Skill-Fertigmeldung aus D.

## 07.10.2026, 03:13 CEST: K8/K9 auf Main, K10 im Gate

**K8/K9 sind auf Main `baca936e7d121a9b44f4a81aa914fbe91093b3e6`.** Drei Testdateien, kein Produktdiff. Selbstgate: `[gpt-6.1-sol] ALLOW: No merge-blocking defects found in the supplied diff and revision snapshots.` 34 Prüfungen nach Rebase grün (19 Publication, 10 echte HTTP-Deadlineprüfungen, 5 Retrieval), 0 ignoriert. Unveränderte Retrievalbaseline `fde910f6`: 0 bestanden, 4 fehlgeschlagen; echte SHA-256-Fixtures beheben diesen Testvertragsfehler. Normales Clippy aller Targets Exit 0 mit einer sichtbaren Warnung in unverändertem `release_port.rs:510`; striktes Clippy scheitert dort am Testmodul vor weiteren Items. Kein Warnungsfilter verwendet.

Main-Push zuerst am R10-Rollenhinweis gestoppt. Den ausdrücklich genannten Skillpfad gelesen, danach Status und HEAD geprüft, regulär erneut `HEAD:main` gepusht. Keine Hookänderung oder Umgehung.

**K10 gesichert auf `feat/brain-ernte-spielstil-20261007`, Head `0ade1a3dc881e466a15d81550a560dc512598f2f`, Selbstgate läuft.** Nach Rebase fünf Spielstilprüfungen sowie striktes Clippy aller Reasoner-Targets grün. Vollsuite 284 bestanden / 6 fehlgeschlagen; unveränderte Baseline mit identischen Zugängen 279 bestanden / dieselben 6 fehlgeschlagen, jeweils 0 ignoriert. Damit kein zusätzlicher Suitefehler durch den Port. Vier Scratch-Fixturefehler (`hero_build_id` fehlt) und zwei September-Konstanten gegen aktuelle Echtdaten bleiben sichtbar, keine Werte passend verändert.

Für A: neuer kompatibler Einstieg `dbrain_reasoner::reason_build_for_playstyle_with_options(ctx, hero, Some("weapon"|"spirit"|"tank"), options)`, reiner Planer `plan_build_with_playstyle(...)`. Bestehende Einstiege verwenden weiter `None`. A besitzt weiterhin Skilldispatch und normalen Publishanschluss; D behauptet keinen bereits laufenden Discord-/Twitch-Spielstilskill. K1/K2/K3 bleiben wegen A-Quellenanschluss unverändert. Noch kein Release- oder Livebeweis aus D.

TESTNACHWEIS[TW-1]: 34 passed, 0 ignored | Baseline: 4 rot

MERGEPROTOKOLL[MS-1]: 6 Git-Schritte einzeln | Anläufe: 2 | Gate: [gpt-6.1-sol] ALLOW

Protokollumfang: eigener Integrationsworktree angelegt, Status, Fast-forward-Merge, nach R10 Status und HEAD, erfolgreicher Main-Push. Der zusätzliche abgewiesene Push erreichte Git nicht.

## 07.10.2026, 03:01 CEST: K10-Suite hat sechs Befunde, Baseline läuft

K10: Compiler, striktes Clippy aller Targets und fünf eigene Spielstilprüfungen grün. Vollständige bestehende Reasonersuite mit echter eigener Scratch-DB und lesendem Central: **284 bestanden, 6 fehlgeschlagen, 0 ignoriert**. Vier Fehler kommen aus dem Scratch-Vertrag ohne `hero_build_id`; zwei Echtdatenprüfungen vergleichen aktuelle Warden-Werte mit September-Konstanten (0,21 statt 0,25 sowie 2,90 statt 3,50). Eine saubere unveränderte `fde910f6`-Baseline läuft jetzt mit derselben Datenbasis. Bis zu ihrem Zahlenbeleg keine Behauptung „vorbestehend“, kein pauschales Grün.

D ändert keine live gemessenen Werte oder Qualitätsgrenzen passend. Testverträge bleiben Gegenstand der Abnahme; noch kein Commit oder Merge. K8/K9-Endlauf mit echten SHA-256-Fixtures läuft ebenfalls. Keine Produktlücke im Retriever behauptet.

## 07.10.2026, 02:51 CEST: Prüfstand, keine Produktlücke behauptet

K8/K9: Veröffentlichungsprüfungen 19/19 und echte HTTP-Prüfungen 10/10 grün. Der alte Dienstbudget-Test erwartete 504, heutiges Main lehnt ungültige Konfiguration bereits mit 503 ab; bei Nullbudget darf vorher die Requestfrist mit 504 enden. Gegenprobe auf diesen aktuellen Fail-closed-Vertrag angepasst, null Provideraufrufe weiterhin Pflicht.

Retrievaltests waren 0/5 rot, weil der heutige Manifestvertrag 64-stellige SHA-256-Werte verlangt, die vorhandene Testfixture aber `a-1`/`b-1` verwendet. Saubere unveränderte Baseline läuft zur Zahlengegenprobe. D passt ausschließlich die Testfixture auf echte Inhaltshashes an, nicht die Produktprüfung. K9-Duplikate in `D/ERNTE.md` präzisiert.

K10 ist im eigenen Reasonerworktree umgesetzt und kompiliert. Fünf Spielstilprüfungen grün; zusätzlicher Schutz erhält den gesamten Upgrade-Katalog, während ausgeschlossene IDs über den bestehenden Composer-Blockfilter vom Kauf ausgeschlossen werden. Keine künstlichen Staple-Markierungen, keine Familien-/Publish-Grenzen geändert. Vollständige Suite benötigt eigene Wegwerf-DB; diese läuft privat per Unix-Socket. Central-Testzugang über den vorhandenen Secret-Exec-Weg wird geprüft, keine Geheimnisse ausgegeben. A behält Skilldispatch, Matchdaten und K1/K2/K3-Quellenanschluss.

Noch kein Commit, Gate, Merge oder Deploy. Kein fertiger Antwort-/Buildskill behauptet.

## 07.10.2026, 02:28 CEST: erster Port und konkretisierte Überschneidungen

Eigener Worktree `/home/nathanael/.worktrees/brain-ernte-sicherheit`, Branch `feat/brain-ernte-sicherheit-20261007`: K8-Domain-/Dense-Gegenprobe, K9-Warm-Index nach Egresswiderruf bzw. fehlender aktueller Herkunft und K9-ungültige Dienstbudgets portiert. Tests laufen mit der festgelegten Toolchain 1.97.1; erster Aufruf traf den alten System-Cargo und wurde vor einem Testlauf abgewiesen. Kein grüner Testnachweis behauptet.

Beim vollständigen Einlesen heutiger Testkörper konkretisiert: der alte Partialbody-Keepalive-Test und die transitive Metadata-Publikationsprüfung sind bereits funktional ersetzt. Diese beiden Duplikate werden nicht hinzugefügt. Der zusätzliche Warm-Index-Test überprüft nach der Revision nochmals Retrieval und danach Providerfreigabe.

K1/K2 benötigen nicht nur Rust-Dateien: `aa2a051e` schreibt `brain.youtube_transcript_evidence` und liest `brain.patch_evidence_revisions` im **zentralen Legacy-Schema**. Der aktuelle Antwort-/Steckbriefpfad kommt aus dem getrennten Brain-Core-Release. A besitzt die laufende Patch-/Legacy-/Profilverdrahtung. D nimmt deshalb weder die alte Komplettmigration noch einen alten zweiten Patch-/MCP-Antwortweg parallel in Betrieb. Die Fachdateien bleiben als Erntequelle benannt; ein bloß neu vorhandenes CLI ohne gemeinsamen Quellenanschluss wäre kein Beweis für das Grundding. Eigener Vorbereitungsworktree `brain-ernte-patchbelege` ist noch unverändert.

Builds-Anschluss vorbereitet unter `/home/nathanael/.worktrees/brain-ernte-spielstil`, noch ohne Produktdiff. D hält den Eingriff auf `dbrain-reasoner` begrenzt; A behält Skilldispatch und Matchdatenbeschaffung.

## 07.10.2026, 02:15 CEST: Inventur abgeschlossen, vorab freigegebene Ernte beginnt

`D/ERNTE.md` prüft alle 44 historischen Erhaltungsstände, 21 neueren WIP-Sicherungen und fünf exakten PR-Heads gegen frisch geholtes Main `fde910f6`. 36/44 historische Stände tragen fehlenden Rust-Code oder zusätzliche Testkörper, oft denselben Caption-/Patchreview-Vorfahren. Das sind keine 36 unabhängigen Features. Acht haben keinen zusätzlich benötigten Eigenanteil. PR #9 steht inzwischen auf `79f5a020`, die übrigen Heads entsprechen den historischen Fach-SHAs.

Vorabfreigabe 07.10. 01:45 Punkt 3 gelesen. Kein weiteres Freigabewarten. D beginnt mit eigenem Worktree und K8/K9-Sicherheits-/Rechtegegenproben, danach deduplizierten Caption-/Patchreview-Anteilen, danach generischer Spielstilplanung für den heutigen Reasoner. Ältere Rechte-/URL-/Readerimplementierungen sind oft schon besser ersetzt und werden nicht rückübernommen. Keine Produktänderung oder Livebehauptung aus der Inventur.

**Überschneidungen vor dem Port gemeldet:**

- K6-Config-/Credential-/Poolschutz sowie K5-TOML betreffen A-F4 (`deadlock-brain-core` AI/Config/Startpfad). A hat Vorrang. D übernimmt diese Dateibereiche nicht parallel. Verbleibende echte Lücken stehen mit Belegen in K5/K6 der Ernte.
- K11-Build-Datenretry betrifft `dbrain-builds/src/api.rs`, A hat inzwischen den Matchdatenauftrag für Builds. D verändert den Client nicht parallel; neuerer Fachstand ist `8318690c`.
- K8/K9 werden eng auf zusätzliche Testkörper begrenzt, keine Änderungen an A-E3s gemeinsamer Antwortmechanik. K1/K2 bleiben zunächst auf Caption-/Insight-/Review-Fachdateien; A besitzt `main.rs` und normale Patch-/Profilverdrahtung.
- K10 gehört zur vorhandenen Reasonerplanung; D liefert den Spielstilanschluss, A weiterhin Skilldispatch und mehr Nach-Patch-Matches. Der alte ungeprüfte Review-Publish-Pfad wird nicht übernommen, 100-Match-/Qualitätsgrenze bleibt bestehen.

Replays, Forum, kompletter Guide, Q/Z, Abo-/Modellwechsel und zweiter MCP-Dienst bleiben unverändert. Aktive A-Worktrees, der veränderte Kanon und lokale geschützte Archive bleiben unangetastet.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/dbrain-retrieval/src/release_port.rs:88 | Anknüpfung: heutige Filter-/Rechtefunktion behalten, zusätzliche kombinierte Gegenproben und fehlende Fachfunktionen selektiv ernten
