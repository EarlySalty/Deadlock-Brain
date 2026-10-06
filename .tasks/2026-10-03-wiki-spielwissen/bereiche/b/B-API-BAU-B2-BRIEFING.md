status: aktiv
Datum: 2026-10-03

# Frischer B-Bauworker: bestätigte Handle-API

[Orchestrator]
BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-b

## Ziel und Vertrag

Punkt46 bestätigt `B-C3-HANDLE-API-B2.md`. Lies diese konkrete Signatur und zentrale C3-Abstimmung `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-wiki-spielwissen/bereiche/c/C3-B-BYTEVERTRAG.md`, AN_BEREICHE.md Punkte44/46, eigenen Readerabnahmeplan. Setze ausschließlich zusätzlichen `extract_game_files_from_handles(BoundGameFiles, &mut impl Write) -> io::Result<GameFileInventory>` mit genau bestätigten öffentlichen BoundGameFile/BoundGameFiles-Feldern um. C3 liefert vollständig inventargebundene private unveränderliche Sicherungen als gehaltene lesbare Dateiobjekte. B keine Originalnamensöffnungen, kein /proc-Reopen oder freie Roottraversierung im gebundenen Eingang, auch VPK-Begleitarchive nur aus Handlebestand. Alter Git-/Lose-Datei-Aufruf bleibt API- und ausgabekompatibel, identische vorhandene Parser/Budgets/Bounds/CRC/fallibleReservation wiederverwenden, kein zweiter Parser.

## Exklusives Eigentum

Genau `rust/crates/dbrain-sources/src/game_files.rs` und `game_files/vpk.rs` samt dortigen bestehenden Testmodulen. Kein anchored.rs oder anderer Parserpfad, keine Cargo/Lock/lib.rs/CLI oder D/C3-Dateien, keine Aktenedits. Neue Hashabhängigkeit verboten; SHA1 nur C3-geprüft weiterführen, B SHA256 am selben übergebenen Objekt gestreamt überprüfen. Keine Secrets/ENV-Konfiguration, Downloads, Live-/Datenbankaufgaben, fremden Prozesse/Locks/Sessions, Unterdelegation oder Gitmutationen. Eltern führt zusätzlichen Eigencommit und Root-Übergabe.

Voriger eigener Signaturworker aa3160d440e87048f hat um11:52UTC ausdrücklich beide Pfade abgegeben, keine aktive Task/Compiler/Prüfer oder Kinder. Er hat nur einen ungeprüften WIP in vpk.rs hinterlassen: read_directory delegiert an read_directory_from_file, File wird übernommen und Cursor auf0 gesetzt. WIP bewahren und fortführen, nicht Parser neu bauen. Aktueller vpk SHA256d87be5d55cbfb42b834175b8400f0ff5cd2eea5c428e9f3b2fe6f064413a80a8; game_files.rs noch Basis SHA256b26a5035019b05c9920c8af14c30857a817d503882025874f73d24b4aee4e3da.

## Arbeitsstand

Branchfeat/brain-wiki-spielwissen-b, HEAD48b6ce1cf277ec4de4b47ac4a0898fbc223fe6b5, Parent2734c2da4e814ff79953e8e825275b0216a6af16. HEAD akzeptiert/remote gesichert, nicht amend/rebase/mergen. Akte untracked, vpk-WIP unstaged. C3 besitzt Caller, Registrierung, private Sicherungen und volle Identitäts-/Inventarbindung, kein paralleler C-Writer in B. D allein Download/Writer. Quelle vor Neubau perGraphify; neue Module teils nicht graphiert, vorhandene konkrete Dateien danach lesen, keinen Graph neu extrahieren.

Kleinstes ausschließlich lokales Prüfharness-Entgegenkommen bereits durch Eltern umgesetzt: `pruefharness-b/src/main.rs:2` ist `pub mod game_files;` statt privatem mod, damit öffentliche Bibliotheks-API im Nichttest-Clippy erreichbar ist. Keine Lintunterdrückung und keine gemeinsame produktive CLI-/Cargo-Änderung. Diesen Harness nicht selbst ändern; erst jetzt neuen vollständigen Sourcefreeze einschließlich dieser Anpassung bilden.

## Beweisziel und Selbstprüfung

Geschlossene normale eindeutige relative Inventarpfade, reguläre Files, tatsächliche Größe/SHA256 an gehaltenen Objekten; fehlende/ungeklärte Archive und Bindungs-/Typ-/Größen-/Hashfehler Err, keine bestätigte Manifestangabe. Build/Depot/Manifest erforderlich, keine Gitrevision als Steamprovenienz; options.root nur Herkunft. Alle regulären Inventareinträge einschließlich übersprungener Assets/Begleitarchive im B-Inventarnachweis sichtbar, vollständiges SourceInventory samt Verzeichnissen bleibt C3. Fileobjekte konsumieren/halten, Cursor ausdrücklich setzen; try_clone teilt Cursor, C3 nutzt Aliasfds nicht parallel. Physische Quellen mit Pfad/Größe/SteamSHA1/SHA256 unter extraction.physical_sources, Ressourcen-/Inhaltshashes getrennt. Bei Err kein Teilimport durch C3.

Gezielte Tests in bestehenden Modulen: neue API gegen gehaltene private Files nach Originalnamens-/Rootwechsel, looseFile-Hash/Größe/Typ/Pfad/Duplikate, VPK eingebettet und externe Archive, fehlendes/ausgetauschtes/trunkiertes Begleitarchiv, Fehler und Provenienzreferenzen. Keine synthetischen Fälle als Depotabnahme, keine echte Rohdatenlage erfinden. Keine Scopefixrunde aus A-Punkt45 oder neue Formatparser.

Nach fertig geschriebenem Sourcefreeze einmal vorhandenen `pruefharness-b/check-round2.sh` vollständig ausführen: beide Hostlocks gemäß HOSTPROBE.md blockierend, frische NonZombieprobe unmittelbar vor jedem Compiler, höchstens2Jobs. fmt, vier Compiler, vierClippy mit-Dwarnings, bestehende Tests --include-ignored --test-threads=2 samt128MiB-VPK-Probe, beide ganzen Git-Export-/Validatorläufe, monotone Dauer/VmHWM und Vorher/Nachher-/Artefakthashes. Alte Beweise erhalten, eindeutiger neuer run-Unterordner. Keine parallele zweite Prüfung. Wrapper/Locks bei Wartephase nicht wegen Timer abbrechen; compilerfreie stabile Wartetask behalten. Ein unvermeidbarer Harnessbedarf mit konkreter Stelle an Eltern, nicht selbst anfassen.

Unmittelbar nach Prüfstart echte Task-ID, Parent/Wrapper/flock-PIDs, Log, Startzeit und Sourcefreeze an Eltern melden. Schlussbericht echte Steps/Exit/Testausführungen/ignored/volle Datenläufe/Ressourcen und Ende eigener Kinder/Locks. Vor Abgabe eigene Änderung passend prüfen, aber kein Review-Gate mit nicht freigegebenem Modell. C3 finaler Integrations-Gate. Frische unabhängige B-Abnahme startet Eltern nach stabiler tatsächlich grüner Selbstprüfung und Eigentumsabgabe.

## Routing

Frischer eigener nativer coder, ausschließlich geerbtes GPT6.1Solhigh, kein anderer Modelloverride/xhigh/max. Keine Unterdelegation oder Sessionkoordination. ElternB2 8e61edb7-3a14-4274-84bf-569122a7241c; Paketb/Versuch2, Statusproduzent nurteil-b. Eltern schreibt Register/Status/kurze Übergabe, TODO nurS, zentrales Register nurRoot. C3 allein Gesamtintegration/Gate/Deploy/Live/Cleanup, gemeinsame endgültige D/B/C-Abnahme vor Steam-Deploy, echte Datenextraktion danach. Ganzer22CommitBranchmerge nicht B2-Scope, keine Hooks ändern/umgehen.
