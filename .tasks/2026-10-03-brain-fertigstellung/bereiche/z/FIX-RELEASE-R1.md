status: aktiv
Datum: 2026-10-03

# Frischer Fixer: Release-Gate Runde 1

[Orchestrator] Paket Z, Versuch 2. Frischer Fixkontext, kein erneuter Start des ursprünglichen Implementierers.

## 1. Ziel und Vertrag

Lies zuerst `bereiche/z/REVIEW.md`, `BETRIEBSVERTRAG.md` und `VON_HAUPT.md` in derselben Akte. Repariere die beiden blockierenden Gatefunde im bestehenden Rust-Releaseinstaller, den gemessenen Cargo-Hardlinkpfad und den Format-/Prüfnachweis. Keine zweite Deploymechanik. Vor Codefragen zuerst Graphify, danach die tatsächlichen Fundstellen prüfen. Der vorhandene globale Graph kann die gerade neu gebauten Ops-Dateien noch nicht kennen; aus diesem Umstand keine Nichtexistenz ableiten.

Die verlangte Herkunft verbindet tatsächliche installierte Binarybytes mit dem geprüften, frisch bestätigten Remote-main-Quellstand und einem echten Build. SHA-Verzeichnisnamen, frei änderbare Manifeste, zusätzliche selbst erklärte Hashfelder oder ein frei erzeugbares angebliches Buildzeugnis erfüllen das nicht. Bestehende Root-/Privilegiengrenzen erhalten: Git und Cargo nie als Root ausführen, keine generische sudo-Brücke, feste zulässige Helfer- und Zielpfade. Falls ein wirklich neuer Sicherheits- oder Betriebskontrakt nötig wird, zuerst die konkrete Abweichung fachlich an Z melden; nicht still einen neuen Dienst, Secret oder Trust-Root voraussetzen.

Unterbrochene Veröffentlichung beider Layouts und Journalanlage müssen Retry/Recovery erlauben und bestehende Releases erhalten. Vorhandene Root-, Symlink-, Traversal-, Ownership-, Zeiger-, Sperr- und Rückfallprüfungen erhalten. Echte Cargo-Hardlinks gezielt behandeln, nicht jede Mehrfachverknüpfung ungeprüft zulassen.

## 2. Eigentum

Alleiniger Schreibbereich: `/home/nathanael/.worktrees/brain-fertig-z/ops/brain-release/`. Kein Schreiben in gemeinsame Kern-/C9-/Adapterquellen, andere Ops-Verzeichnisse, fremde Worktrees, Hauptcheckout oder Produktionspfade. Keine neuen Codekommentare. Keine globale Formatierung. Keine Produktivinstallation, Units, Neustarts, Datenänderungen oder Branch-/Worktreelöschung. Keine weiteren Agenten oder T3-Threads.

Uncommittierte Änderungen in `src/main.rs`, `src/install.rs` und `src/source.rs` stammen aus einer eigenen inzwischen ausdrücklich beendeten Fortsetzung. Übernehmen und prüfen, nicht wegsetzen oder neu anfangen. C9 liegt im Index; Archiv und Kandidatenadapter gehören anderen eigenen Workern und werden nicht angefasst.

## 3. Arbeitsstand und Git

Worktree `/home/nathanael/.worktrees/brain-fertig-z`, Branch `feat/brain-fertig-z-20261003`, bestätigter HEAD und Remote-Featurekopf `b86353acca8027431de58a2edadbebcdd01bad10`. Der geprüfte Commit hat BLOCK, kein neuer main-Merge. Quellen und Debugbinary haben sich danach verändert; alte Prüfnachweise gelten dafür nicht.

Du darfst nach belastbarer Eigenprüfung ausschließlich deine Ops-Dateien auf diesem Featurebranch committen, damit du deine tatsächliche Änderung selbst erneut mit dem zentralen Gate prüfen kannst. Git-Schritte einzeln, literale absolute Pfade. Explizite Dateiauswahl, kein `git add -A`; bestehende C9-Indexeinträge im Commit ausschließen, etwa mit korrekt begrenztem `commit --only`. Keine Commits fremder Dateien. Kein Push und kein main-Merge; Z integriert und pusht anschließend. Committrailer gemäß Nutzerregel mit tatsächlichem geerbtem Modell. Nach Gate-Deny zuerst Status und Log, Befund fachlich berichten; ein BLOCK gehört in einen weiteren frischen Fixkontext, nicht in eine eigene Review-Schleife.

## 4. Beweisziel

Passende bestehende Tests erhalten und nachziehen. Tatsächliche Rustup-Cargo `/home/nathanael/.cargo/bin/cargo`, nicht System-Cargo mit inkompatiblem Lockformat. Fmt, Clippy, Tests und Build jeweils expliziten Exitcode erfassen. Kein Erfolg aus einem abschließenden Druckbefehl oder Sammel-Exit ableiten. Logs privat im eigenen Ops-Bereich oder dessen ignoriertem Target. Testanzahlen mit passed/failed/ignored/filtered und `TESTNACHWEIS[TW-1]` melden. Neue Tests sind keine pauschale Pflicht, aber der Produktivbaupfad und die gefürchteten Abbruchzustände dürfen nicht durch einlinkige Binärfixtures oder bloße Funktionssignaturen als bewiesen gelten.

Jeder Compilerlauf hält zuerst `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock`, danach `/tmp/deadlock-cargo-release.lock`. Identitäten frisch prüfen, NonZombie-Probe, höchstens zwei Jobs. Fremde aktive Cargo-/Rustc-/Rustfmt-/Clippyprozesse bedeuten warten bei gehaltenen Locks, nach 30 Sekunden neu prüfen. Keine Wartezeitbegrenzung und keine fremden Prozesse stoppen. Nur exakt `cargo metadata --format-version 1 --no-deps --manifest-path /absoluter/pfad/Cargo.toml` ist die belegte Ausnahme; zusätzliche Metadataflags oder andere Compiler zählen nicht dazu. Keine zweite parallel laufende eigene Prüfchain starten.

Nach geprüftem eigenem Commit den vorhandenen zentralen `gate_hook.py --review` auf diesen tatsächlichen Featurekopf gegen `main` ausführen. Denselben Modellpfad wie Runde 1 verwenden, dessen Urteil kam von `[gpt-6.1-sol]`. Kein eigener Reviewer, kein Reviewmodellwechsel und keine Gateumgehung. Noch nicht geprüft, BLOCK oder Werkzeugausfall genau so melden.

## 5. Routing und Rückgabe

Auftraggeber ist ausschließlich diese native Z-Hauptsession. Gesamt-Hauptorchestrator Codex /root, T3 `e6c19079-657e-4db9-80bd-8e1313e7f785`, wird nur über Zs Dateien erreicht. Fachliche Rückgabe an Z: genaue Änderungen, tatsächlicher Commit, Scope, Prüfbefehle und einzelne Exits, Tests, echte Cargo-Linkmessung, tatsächlicher Herkunftsmechanismus und Gateantwort. Gebaut, geprüft, gemergt und live getrennt melden. Kein Schreiben in TODO.md, REGISTER.md oder fremde Statusdateien; Z produziert `status/z/2/` als `teil-z`. Deutsche Berichte, echte Umlaute, humanizer und no-em-dashes anwenden.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-z
