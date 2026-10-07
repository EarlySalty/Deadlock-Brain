status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-s

# Frischer Fixer für HTTP-Wiederaufnahme

## Ziel und Vertrag

Behebe zwei unabhängig bestätigte Befunde im übernommenen HTTP-Client: Nach einem unklar abgeschlossenen POST wird kein hashgebundener GET versucht, obwohl dieser den fertigen Auftrag liefern könnte. Außerdem prüft die Pollschleife die Frist vor dem bereits validierten Endzustand und kann so eine bestätigte Build-ID verlieren.

Lies AUFTRAG.md, BRIEFING-S.md und bereiche/s/VON_HAUPT.md in der gemeinsamen Akte. Code-Suche zuerst über Graphify. Rolle-wirkungs-pruefer, humanizer und no-em-dashes anwenden.

Vor dem ersten POST den Status der unveränderten Anfrage prüfen. Bei vorhandenen queued/running-Aufträgen pollen, bei vorhandenem succeeded dessen Build-ID übernehmen. Nach einem unklaren POST denselben Anfragehash per GET erholen. Keinen neuen Auftrag und keine neue ID bei unklarem Abschluss erzeugen. Ein nochmaliger identischer POST ist durch den vorhandenen persistenten Idempotenzvertrag abgesichert; vorhandene Endzustände bleiben endgültig. Einen validierten Endzustand vor der weiteren Fristprüfung zurückgeben. Den nicht blockierenden Gate-Hinweis zur Klassifikation von Antwortkörper-Timeouts im selben Client beheben, wenn das mit der vorhandenen transport_error-Funktion möglich ist.

## Eigentum

Nur diese beiden Dateien schreiben:

- rust/crates/brain-feeds/src/build_publish.rs
- rust/crates/brain-feeds/tests/build_publish_endpoint.rs

Keine anderen CLI-Bereiche, Lockfiles, globalen Formatierungen oder fremden Dateien ändern. Kontext lesen erlaubt. Keine Secrets klar lesen, ausgeben oder schreiben, keine Prozessumgebungen lesen. Keine Produktivdaten, Publishes, Deploys oder main-Mutationen. Keine Unteragenten oder T3-Threads.

## Arbeitsstand

Brain: /home/nathanael/.worktrees/brain-fertig-s, feat/brain-fertig-s-20261003, HEAD 9a6f3d5f3ae2349d9fb076821381e45a421a0bbe. Ausgangsbestand committiert und gepusht. Steam: /home/nathanael/.worktrees/steam-publish-fertig, HEAD 9aec0cc897b01b74d417ab9b510314cbbbd02535, nicht anfassen.

GPT-6.1 Sol erben, xhigh. Z integriert und installiert gemeinsam. Erlaubt sind ein eigener Feature-Fixcommit der beiden Dateien mit Modelltrailer und Push ausschließlich nach feat/brain-fertig-s-20261003. Kein rebase während laufender Prüfungen, kein main-Merge oder Releasezeigerwechsel.

## Beweisziel

Bestehende HTTP-Tests nicht abschwächen. Angemessene deterministische Nachweise für Status-vor-POST und Wiederaufnahme nach verlorener POST-Antwort. Keine neuen echten Wartezeiten in Tests. Die Hauptsession hat bislang 20 Brain-Publishtests bestanden: 2 Lib, 16 HTTP, 2 CLI. Steam-Prüfungen scheiterten vor Teststart am gesperrten Lockfile und zählen nicht als grün.

Vor jedem Compilerstart blockierend erst /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock, dann /tmp/deadlock-cargo-release.lock erwerben. HOSTPROBE.md vollständig befolgen: frische NonZombie-Probe, höchstens zwei Jobs, bei fremden Compilern beide Locks halten und nach 30 Sekunden erneut prüfen, keine fremden Prozesse stoppen. Exakte reine cargo-metadata-Aufrufe können nach dem dortigen Vertrag ausgenommen werden. Alle eigenen Kinder beenden und beide Locks freigeben. Verwende ausdrücklich /home/nathanael/.cargo/bin/cargo, PATH mit /home/nathanael/.cargo/bin zuerst, SQLX_OFFLINE=true, --locked und -j 2.

Passende Läufe sind brain-feeds --lib build_publish, brain-feeds --test build_publish_endpoint sowie deadlock-brain --bin deadlock-brain publish, jeweils --include-ignored. Exit-Codes und echte Testanzahlen prüfen; das letzte Werkzeug zeigte bei Steam Cargo-Fehlertext trotz äußerem Exit 0. Bei Fehlern nicht weiter als grün melden. Keine volle Baseline behaupten.

Nach dem Fixcommit selbst prüfen: python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-fertig-s --base 511a347b653beba13c2bf130f4bead7a7196cc2a --timeout 600. Standardmodell aus der Gate-Konfiguration verwenden, keinen Modellwechsel erzwingen. Bei BLOCK Bericht mit offener Liste, nicht selbst weiter orchestrieren. Prüfprotokolle im Fachbereich unter pruefung-v2/fix-* ablegen.

## Routing

Rückgabewert an Teil-Orchestrator S2. Hauptauftraggeber Codex /root, T3 e6c19079-657e-4db9-80bd-8e1313e7f785. Status schreibt allein teil-s2 unter status/s/2. TODO.md und REGISTER.md gehören anderen Rollen. Keine Sessionkoordination. Bericht mit vollem Fix-SHA, tatsächlichen Änderungen, Testanzahlen, Gateurteil und Grenzen. Live-Nachweis bleibt nach Zs Deployment offen.
