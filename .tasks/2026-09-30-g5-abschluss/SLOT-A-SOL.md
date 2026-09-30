status: aktiv
Datum: 2026-09-30

# Slot A: V1 ohne Replay statisch vorbereiten

Du bist der einzige Implementierer für dieses Paket. Keine Unterthreads oder Unteragenten. Bestehender Thread 66adf9ee-bc03-4ff3-91da-73cd8efc5e72 (Sol); dies ist die Fortsetzung nach abgeschlossenem PR59, kein neuer Match-/Analyticsauftrag.
Intent-Thread: 562a877b-0939-440a-964d-1145d9e9431a.

## Nutzerentscheidung und Umfang

Wörtliche neue Anweisung: „Slot A ist zugeteilt: bestehenden T3-Sol-Codeworker für die statische Quellvorbereitung im vorgesehenen G5-Worktree fortsetzen. Danach unabhängige statische Abnahme. Keine zusätzlichen Unterthreads nötig.“
„Slot B bis E bleiben gesperrt bis zur konkreten Zuteilung durch pr_inventory: keine Compiler, Benchmarks, Releasebuilds, Modellserver, Prozessstarts oder Dienständerungen.“
G5 ist nach allen Nachweisen freigegeben, Replay ausdrücklich später. Keine neuen Produktmodelle, Anbieter oder Kostenpfade. Keine erneute Nutzerfreigabe nötig.

## Verbindlicher Arbeitsort

Ausschließlich /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930
Branch fix/g5-replay-deferred-20260930
Bestätigte Basis/HEAD 1c362bca6d35e7fec10125b2b159e7513a299243
Worktree sauber, Branch bereits nach origin gepusht.

Dein alter Worktree brain-pre-g5-finalize-20260929 wird nicht verändert. Prüfe am Anfang show-toplevel, Branch, HEAD und Status am neuen absoluten Pfad. Falls die T3-Laufzeit dich technisch an den alten Baum bindet, melde den genauen Fehler und den tatsächlichen cwd. Kein Checkoutwechsel im geteilten Baum, kein wiederholtes Verlangen nach alten fehlenden Commits. Keine fremden Dateien verändern.

## Quellen und konkreter Schnitt

Maßgeblich: AUFTRAG.md und PRUEF-SERVE-CUTOVERPLAN.md im Koordinationsordner /home/nathanael/.worktrees/brain-technical-closeout-20260929/.tasks/2026-09-30-g5-abschluss/.

Bereits belegte Fundstellen auf deiner Basis:
- rust/Cargo.toml:28: "crates/dbrain-replay" als Workspace-Member.
- rust/crates/dbrain-replay/Cargo.toml:3,15-18: geerbte Version und serde/serde_json/sha2/tempfile; bei isoliertem Workspace konsistent behandeln.
- rust/crates/dbrain-replay/Cargo.toml:10-12: haste bfb292d4798031350861ad297aa26753267a1ea6, valveprotos 4f4a3cb1b0c6f19af59a722acb79ecccd01f61f6. dungers transitiv. Pins nicht wechseln und keine Quellen vendorisieren.
- architecture/migration/replays/s14/check-decoder.sh:8-12 ruft dbrain-replay derzeit über Rootmanifest auf.
- .github/workflows/ci.yml:183-184 und .github/workflows/rust-core-verification.yml:164-166,214-220,264 erwarten Replaylauf/Artefakt im integrierten Workspace.
- Statische Lockanalyse: 25 Wurzeln, ohne Replay 24; 427 Pakete, davon 386 erreichbar ohne Replay. Keine verbleibende Gitquelle, keine unaufgelöste Kante. Selbst prüfen, Zahlen nicht blind übernehmen.

Implementiere die klare V1-Abtrennung: Replay aus dem aktiven Rootworkspace und dessen Auflösungsgraphen herausnehmen, Replayquellen und Tests als getrennten ausdrücklich zurückgestellten Bereich erhalten. Nur optional/default-members/--exclude ist keine ausreichende Trennung. Replaymanifest/Prüfskripte separat bedienbar halten. V1-CI-Matrix und Artefaktaggregation konsistent auf den freigegebenen V1-Umfang bringen; ausgelassene Replaytests niemals als bestanden zählen. Keinen Ersatzparser und keine neue Brain-API bauen. Native Manifest-/Lock-/Workflowänderungen bevorzugen, keine neuen ausführbaren Python-/JS-Werkzeuge.

Lockdatei: In Slot A kein Cargo aufrufen. Eine statisch beweisbare reine Bereinigung anhand des vollständigen bestehenden Abhängigkeitsgraphen ist zulässig, mit erhaltenen Paketversionen, Quellen und Checksums; die bisherige Empfehlung, Cargo die Bereinigung übernehmen zu lassen, wird damit für diese Vorbereitung ergänzt. Bei Unsicherheit alten Lockstand erhalten und konkreten nötigen Cargo-Metadatenlauf für Slot B melden, nicht raten. Eigenständigen Replay-Lockstand erhalten, keine neuen Git-Fetches oder Cargo-Caches.

Aktuelle V1-Grenze dokumentieren, aber keine früheren Freigaben oder Ergebniszahlen überschreiben. Nur vom Replay-Schnitt betroffene Doku. Kein Refactoring, kein globales fmt, keine Änderung an brain-serve, Auth, Infisical, Providerwahl, DB-Rollen oder Livekonfiguration. Keine neuen Code-Kommentare.

## Erlaubte Prüfungen und verbotene Läufe

Graphify zuerst; eigener Worktree hat keinen Graph, globale Graphdatei /home/nathanael/.graphify/global-graph.json benutzen. Danach konkrete Fundstellen lesen. Statische Diff-/Syntax-/Abhängigkeitsprüfung mit vorhandenen Werkzeugen, keine Paketinstallation. Kein cargo, rustc, rustfmt, clippy, Testlauf, Benchmark, Modellserver oder Dienststart. Kein Load-/Prozessharness. Keine Secrets lesen oder ausgeben, keine Environment-Konfiguration.

Bestehende Coding-Agenten und statische Reviewer sind in Slot A ausdrücklich erlaubt; produktive/lokale Modellinferenz und neue kostenpflichtige Routen nicht. Falls du deinen üblichen gate_hook.py --review ausführst, darf dieser ausschließlich statisch reviewen und keinen Build/Prozessharness anwerfen. Kein Test-Gate als bestanden behaupten. Unabhängige statische Abnahme übernimmt die Hauptsession anschließend im vorhandenen Reviewerthread.

## Git und Abgabe

Nur eigene explizite Dateien stagen, jeden Git-Schritt einzeln mit literalem absoluten Pfad. Eigene Quellvorbereitung committen und den vorhandenen eigenen Branch pushen. Kein Merge, kein Main-/Masterpush, kein Deploy. Ungeprüfte Laufzeit nicht als verifiziert darstellen. Commit-Trailer: Co-Authored-By: Claude Code <noreply@anthropic.com>.

Bericht in deinem Worktree unter .tasks/2026-09-30-g5-abschluss/SLOT-A-ERGEBNIS.md: exakter HEAD, Dateien, statische Befunde, unveränderte Versionspins, erhaltene Replaygrenze, ausgeführte Befehle, nicht ausgeführte Compiler-/Laufzeitnachweise und exakte nächste Buildbefehle mit vorhandenem Targetcache /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target. Kein Schreiben in den fremden Harnesscache in Slot A. Hauptsession aktualisiert BRAIN-G5-BUILD-REQUEST.txt beim Integrator.

Fertigziel Slot A: gepushter minimaler statisch nachvollziehbarer Diff, Replay nicht mehr Teil der V1-Abhängigkeiten, alter Replaybestand erhalten, unabhängige Prüfung möglich. Kein Anspruch auf Compilergrün oder live.

Bei echter Blockade: [Bump-up] Paket Slot A: Grund: ... Erledigt: ... Worktree: ... Offen: ... an Intent-Thread 562a877b-0939-440a-964d-1145d9e9431a in deiner Schlussmeldung. Keine Nachrichten an fremde Sessions.