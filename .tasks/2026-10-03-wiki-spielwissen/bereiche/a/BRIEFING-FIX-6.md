# A: autorisierte enge Präzisionskorrektur6

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-a

## Ziel und Freigabe

Du bist ein frischer nativer Blatt-Fixer, GPT6.1Sol high aus dieser Session geerbt. Kein Modelloverride, Fallback, weiterer Agent oder T3-Thread. Root autorisiert Fix6 ausdrücklich in AN_BEREICHE.md Punkt45, 03.10.2026 11:23UTC. Vor produktivem Code CONTRACT.md sowie Punkt45 und REVIEW-LOCAL-7.md lesen. Konkreter Fixauftrag, keine neue Planfreigabe.

Nutzerziel: strukturierte Zahlenwerte und Quellenlexeme unverfälscht erhalten. Vollständige Eigenabnahme fand282 numerische Abweichungen unter1.109.153 Fakten, Originaltexte exakt. Beispiel Seite2880/Revision18108: /hero_atlas/FalloffStartRange20.000010800000002 wurde20.0000108; /hero_ghost/DPS55.555555555555564 wurde55.55555555555557. A bestätigt beide numerisch ungleich. Bestehendes normalize.rs:586 liest in serde_json::Value, :902-916 gibt Zahlen als Fakten weiter. Bisheriger Harness prüft Originalinhalt/Faktenzahl, nicht jede Originalwertgleichheit. Keine Epsilon-Toleranz, Rundung, stiller Zahlentypwechsel oder zweite Parserimplementierung.

Prüfe den vorhandenen serde_json-Pfad samt Serialization/Deserialization und bestehenden Featuremöglichkeiten, wähle nach tatsächlichem Verhalten. Konkrete benötigte Featurekonfiguration und Typenwirkung an A für Root/C3 melden. Produktive gemeinsame Manifeste und Verbraucher gehören ausschließlich C3.

## Eigentum

Eigene erlaubte Schreibpfade:
- rust/crates/dbrain-sources/src/wiki_inventory/normalize.rs, bestehender Zahlenpfad und eng nötige Regression darin.
- rust/crates/dbrain-sources/src/wiki_inventory.rs ausschließlich bereits belegte collapsible_if-Warnung um455, kein weiterer Umbau.
- .tasks/2026-10-03-wiki-spielwissen/pruefharness-a/Cargo.toml und eng nötige eigene Harnessquelle src/main.rs, eigener Lockfile nur nötige Auflösung; bestehende Abhängigkeiten/Versionen erhalten.
- Eigener FIX-6.md und Erhaltungssnapshots unter bereiche/a/fix6-before/ sowie eigene Prüflogs.

storage.rs nur lesend: Falls eine tatsächliche zusätzliche Spoolursache Werte verändert, konkrete Stelle/Ursache/benötigte Änderung zuerst an A melden, nicht automatisch ändern. tests.rs und41 bestehende Regressionen unangetastet, keine Abschwächung oder Umgehung. Fehlende neue Regression darf in normalize.rs oder dem eigenen Harness eng ergänzt werden. Kein produktives Cargo.toml/Cargo.lock/lib.rs/Core/util.rs/Schema/CLI, keine fremden Dateien, kein DB/Netz/Secrets/ENV/Git/Merge/Push/Deploy. Keine neuen Quellen oder Parserpfade.

Code-Suche zuerst code-suche/Graphify, danach genaue Dateien lesen. Neue A-Module fehlen im globalen Graphen, kein Graphneuaufbau.

## Arbeitsstand und Erhaltung

Absoluter Worktree oben, Branch feat/brain-wiki-spielwissen-a, HEAD2734c2da4e814ff79953e8e825275b0216a6af16. Vier eigene Module uncommittiert; fremder Altbranch und dirty Hauptbaum unverändert. Alle alten eigenen Writer/Compiler/Datenworker abgeschlossen, keine laufenden Kinder/Locks. Du bist alleiniger neuer Sourcewriter.

Alte vier Modulhashs:
- wiki_inventory.rs2507c127aaf8e366f7bb7cf22564074adbaa546b344328d6fdbd593e5b38ef80
- normalize.rs27e63412a7a57c956dc82e602df4f93683adb3eb06866b22381157f85b96dcdc
- storage.rsedd384e621d9a0df11d1a36ad6cac44678285e342f68d4c363db42f1646bd7ea
- tests.rse26f97b9f3a9d666889cefff3445d17a8e07511c7fd527591c12e172356e3f18

AlterHarnessmain d358ec15a692aeb57cbbd7f05e62be4eaa85217c3b5eaf59db4a63fc23f201fd; Cargo.toml9ad50a0515c37ecd19a87406dafb2bf3c6c6164b3c905d60cffb0649f7e05620; Lockfileab2a866b93478823752eaafed59164faf06bcd214335257c597ecc4f8e7d2c6a. Altes gebautes Debugbinary83349db303f5661ba4c6f0ac156b4ec14ab93e36bf85d36b90aa67b163d48578 unter pruefharness-a/target/debug/wiki-pruefharness-a.

Vor erster Änderung sieben Originaldateien als exakte eigene Erhaltungssnapshots sichern und alle Snapshot-/Originalhashs bestätigen. Kein Snapshot überschreiben. Altes Binary und alte target/-Artefakte absolut unverändert: für neue Cargo-Prüfungen explizit --target-dir /home/nathanael/.worktrees/brain-wiki-spielwissen-a/.tasks/2026-10-03-wiki-spielwissen/pruefharness-a/target-fix6 nutzen. Kein pauschales cargo clean.

Alle47 ursprünglichen Inputs und alten normalisierten Ausgaben unverändert erhalten. AlterOutputroot /home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized/. Erst künftiger korrigierter Volllauf in einem neuen eigenen getrennten Root normalized-fix6, niemals alte Spools überschreiben oder erneut hinein deduplizieren. Alter Datenlauf38.273Dokumente/1.109.153Fakten/sechsWiederholungen belegt, numerische Werte nicht freigegeben. Heutige a-live-coverage-Challenges keine Inputs.

## Beweisziel und Ablauf

1. Ursachenpfad und Feature-/Typenwirkung prüfen, alte Dateien sichern, eng korrigieren. Eigener Harness muss künftigen Outputroot ausdrücklich getrennt wählen und alle Faktenwerte/Originaltexte/Identitäten prüfen, ohne zweite Parserimplementierung. Wiederverwende bestehenden Rust-Harness und vorhandenen serde_json-Pfad. Optional eng benötigte Harnessanpassung vor Freeze, keine neue Sammlung.
2. Eigene gezielte Formatprüfung sowie echte vorhandene Tests/Clippy/Debugbau mit neuem target-fix6 durchführen. Vor jedem Cargo/Compiler/Clippy/Tests beide Hostlocks blockierend und dauerhaft: erst /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock, dann /tmp/deadlock-cargo-release.lock. HOSTPROBE.md lesen, unmittelbar vor jedem Compiler frische NonZombie-Probe, -j1, keine fremden Locks/Prozesse. Stabilen Wrapper nie timerbedingt stoppen. Cargo /home/nathanael/.cargo/bin/cargo, kein Release. Bestehender Gesamtfmt1 im fremden util.rs bleibt ehrlich, kein fremdes Formatieren. Tests ungefiltert/--include-ignored, echte Exits und passed/failed/ignored/filtered, vollständige Logs.
3. Konkrete Zahlenkorrektur an den tatsächlichen Originalbeispielen und durch lokale echte Regressionen belegen; numeric values bleiben JSON numbers, Quellenlexeme erhalten. Jeder Serialize-/Deserialize-Übergang beachten, nicht nur erster parse. Keine fake Parser-/Spoolkomponente.
4. Abschließend alle geänderten und unveränderten Modul-/Harness-/Snapshot-/Binaryhashs einfrieren, eigenen Writer beenden, eigene Kinder/FDs/Locks frei bestätigen und native kurze Rückgabe an A mit FIX-6.md. Keine vollständige erneute Normalisierung in diesem Fixauftrag: A beauftragt nach Freeze die unabhängige Prüfung und getrennten erhaltenen Vollbeweis. Keine weiteren Module ändern oder unbeauftragten Datenloop starten.

Falls Cargo unerwartet ungelöste Abhängigkeiten verlangt, tatsächlichen Befund melden, keinen pauschalen Upgrade/Netzweg. Lokale Featurewahl ist an tatsächlichem Verhalten zu prüfen; C3 erhält exakt die bestätigte produktive Requirement, keinen vermuteten Featurevorschlag.

## Routing und Urteil

Auftraggeber A f01cce67-209b-468e-8abb-ec2070beeaa2. Root übergeordnet, Paket a/Versuch1/teil-a alleiniger Statusproduzent. Keine fremden Sessions kontaktieren, kein ListAgents. Native Ergebnisse ausschließlich an A, Rohberichte intern. Nur A schreibt Register/Status/UEBERGABE/AN_HAUPT/HANDOFF, Root synchronisiert C3. Nur C3 finaler Gate/Integrator/Deployer, keine Gate-Defaults mit anderen Modellen starten.

Rückgabe: konkrete Ursache, exakte Feature-/Typenwirkung, geänderte Dateien, tatsächlich gelaufene Prüfungen samt Zahlen/Exits, Freezehashs, eigene Prozess-/Lockfreigabe, Grenzen. Nach abgeschlossenem Auftrag nicht für neue Funde reaktivieren. Keine pauschale Import-/Prod-/Gatefreigabe, finale frische Eigenabnahme und Modulcommit bleiben A.
