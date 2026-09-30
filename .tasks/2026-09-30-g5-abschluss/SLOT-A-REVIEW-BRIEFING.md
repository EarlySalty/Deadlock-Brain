status: vorbereitet, erst nach gepushter Autorenabgabe ausführen
Datum: 2026-09-30

# Slot A: unabhängige statische Abnahme

Bestehender Reviewerthread 52c34332-8cdf-4772-9e1f-42aba432c6cf. Kein neuer Thread und keine Unteragenten. Du bist nicht der Implementierer. Intent-Thread 562a877b-0939-440a-964d-1145d9e9431a.

## Bindung und Quellen

Zu prüfender Quellworktree: /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930
Branch: fix/g5-replay-deferred-20260930
Basis: 1c362bca6d35e7fec10125b2b159e7513a299243
Prüfhead wird in der Auftragsnachricht konkret genannt; ohne diesen SHA nicht beginnen. Nur den vollständigen Diff Basis..Prüfhead prüfen, nicht die alten Reviewbranch-Deltas gegen main.

Dein eigener Berichtsworktree: /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929
Branch: review/pre-g5-core-abnahme-20260929
Zuletzt sauber auf 7eb844dd604935495d3fd42165620690f059c739, vor Schreibbeginn neu prüfen. Diesen historischen Baum nicht zurücksetzen, keinen Code daraus als aktuellen Stand prüfen. Lies den Zielcommit im benannten Quellworktree read-only. Schreibe ausschließlich deinen neuen Bericht .tasks/2026-09-30-g5-abschluss/SLOT-A-REVIEW.md im eigenen Reviewbaum.

Maßgeblicher Intent: AUFTRAG.md und SLOT-A-SOL.md unter /home/nathanael/.worktrees/brain-technical-closeout-20260929/.tasks/2026-09-30-g5-abschluss/ sowie die Nutzerentscheidungen G5 nach allen Nachweisen, Replay später. Vorhandener typed Servepfad bleibt unverändert, kein Neubau und keine Produktmodelländerung.

## Harte Ressourcenregel

Nur Slot A ist zugeteilt: statische Coding-Agentenarbeit/Abnahme. Kein Cargo, Compiler, rustfmt, clippy, Test, Benchmark, Releasebuild, Prozessharness, Modellserver, Restart oder Deployment. Keine Paketinstallationen, neuen Caches, Secretlesung oder ENV-Konfiguration. Keine Unterthreads, kein zusätzlicher Reviewer durch dich. Keine Code-Kommentare. Kein neuer kostenpflichtiger Provider oder Produktmodellpfad.

## Prüfung

Erst git status und vollständigen Diff-Stat, dann Produktdiff und unabhängige Gegenprüfung, Autorenbericht zuletzt. Graphify zuerst, bei fehlendem Repo-Graph die globale Datei nutzen; keine Extraktion starten.

1. Rootworkspace: dbrain-replay samt privaten/transitiven Gitquellen tatsächlich von V1 abgetrennt, nicht nur durch default-members/optional/--exclude verdeckt. Keine andere fachliche Pflichtcrate versehentlich entfernt.
2. Der erhaltene Replaybereich ist in sich konsistent: eigene Workspace-/Manifestgrenzen, geerbte Versionen/Dependencies korrekt, unveränderte haste/valveprotos-Pins und vorhandene Replaytests/-quellen, sauberer getrennt benannter Lockstand. Keinen Quellen-/Lizenznachweis für Replay erfinden.
3. Lockdiff unabhängig als Graph gegenprüfen: sämtliche 24 V1-Wurzeln und deren Test-/Build-Abhängigkeiten erhalten; keine neue Version, Quelle oder Checksumme; keine übrig gebliebene Gitkante. Mehrere Versionen eines Paketnamens und source-qualified Dependencies korrekt behandeln. Eine statische Gegenprüfung ist kein Cargo-Nachweis.
4. Alle relevanten Skript-, CI- und Artefakt-Aufrufer prüfen, insbesondere ci.yml, rust-core-verification.yml und s14/check-decoder.sh. Fehlende Replayartefakte dürfen V1 nicht weiterhin blockieren. Weggelassene Replayjobs dürfen nicht als bestandene Prüfung erscheinen. Der zurückgestellte Replaypfad soll weiterhin ausdrücklich bedienbar sein, ohne V1 zur Auflösung seiner Quellen zu zwingen.
5. Existing typed /v1/answer, Auth/Infisical, Modelle, Budgets und Services unverändert. Keine Quell-/Dokumentationsänderung außerhalb des Replay-Schnitts. Keine Abschwächung verbleibender Tests.
6. Aussagen zu Build, frischem Fetch und Live prüfen. Heute keine Compilergrün- oder Livebehauptung. Exakte noch ausstehenden Befehle und Cachebedarf für Slots B bis E angeben. Historische G2/G3-Befunde sauber von neuem V1-Scope trennen.

## Abgabe

Urteil ausschließlich über Slot A: Fertig J/N, Fix nötig J/N, konkrete Befunde mit Datei/Zeile und Gegenbeleg. Getrennt: statisches GO/BLOCK, Compilerprüfung NICHT AUSGEFÜHRT, G5-Livebeweis NICHT ERBRACHT. Keine G5-/Deployfreigabe aus statischem GO ableiten.

Nur neuen eigenen Bericht committen und eigenen Reviewbranch pushen, Git-Schritte einzeln und absolute Pfade. Keine fremden Dateien stagen, kein Main-Merge, keine PR-Änderungen. Commit-Trailer Co-Authored-By: Claude Code <noreply@anthropic.com>. Bericht-SHA plus geprüften Produkt-SHA melden. Bei technischem Bindungsproblem konkreten cwd und Fehler melden, nicht unpassende alte Commits verlangen.