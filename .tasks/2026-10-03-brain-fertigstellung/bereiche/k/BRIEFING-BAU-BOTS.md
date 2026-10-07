status: beauftragt
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/Deadlock-Bots-brain-consumer-fertig

# Bots: sicheren bestehenden Brain-Consumer übernehmen

## Ziel und Grundlage

Lies GEMEINSAM.md, AUFTRAG.md, PAKETE.md, BRIEFING-K.md, UEBERNAHME-CODEX.md und die aktuelle bereiche/k/VON_HAUPT.md. SCOUT-bots.md enthält die bereits abgeschlossene Bestandssuche. Vorhandene Arbeit wiederverwenden, keinen Consumer neu bauen. K liefert lokal geprüften SHA und Betriebsvertrag, Z führt gemeinsame Integration, unabhängige Abnahme, Gate und Installation durch. Kein eigener main-Merge oder Produktivwechsel.

Vor Schreibbeginn die Repoanweisungen von Deadlock-Bots und nochmals die zwei geschützten Threads ausschließlich lesend mit dem bestehenden t3-thread.py read prüfen. Bekannter Ausgabeheader ist session: ('stopped', '') beziehungsweise session: ('ready', ''). Bei running keine Schreibarbeit in dem Consumerbereich beginnen, konkret melden und fremde Threads unangetastet lassen. Kein new, send, stop oder settle für fremde Threads. Die letzte Elternprobe ergab stopped/ready. Separate Guide-/Migrationsresolver bleiben unberührt.

## Eigener Arbeitsstand und Übernahme

Wenn der vorgesehene eigene Worktree bereits existiert, dessen Herkunft und Auftragseigentum prüfen und erhalten. Keine fremde Arbeit bereinigen. Andernfalls origin/main frisch fetchen und genau den eigenen Branch feat/brain-consumer-fertig-20261003 im vorgesehenen Worktree anlegen. Git-Schritte einzeln, mit literalen absoluten Pfaden; keine Mutation des fremden main-Checkouts.

Vorhandener sicherer Quellbereich: 1e6cdf648429e50014e548b0b4204c8ac48cb4ec..e3e649ccc13193c9dee16cca7651cabb26f25ab3, elf Commits aus sol/abschluss/7bf0e0375ee34a00. Die vollständige chronologische Commitliste und ihre Pfade aus unveränderlichen Git-Objekten bestätigen und per Cherry-pick übernehmen. Bereits tatsächlich integrierte identische Änderungen sauber erkennen statt neu zu bauen. Nicht nur den ersten Bindingcommit 32732443735e2371cb5872fd4b01c896802d3672 übernehmen. Keine ganzen alten Dateien auf aktuellen main kopieren.

Aktueller main bei der Bestandsprobe: ae490cd9e9f6d647f5107cc530db63e6a11d231f, mit vier jüngeren Launcherkorrekturen. Deren Verhalten beim Konfliktauflösen erhalten. Kein vollständiger Import des gekoppelten Integrators 635f6b6be005899cb747e82ab095d7123eb9a276, des alten PR-Kopfes e805fbed60a9176984538f2edaa3960209c589ab oder des Guidebranches.

## Eigentum und Vertrag

Enger Consumerumfang:

- rust/crates/dl-brain/
- rust/crates/dl-core/, nur nötige Consumerkonfiguration und vorhandene zugehörige Tests
- rust/crates/dl-token-secrets/src/lib.rs und bestehende zugehörige Tests
- rust/bin/dl-bot/src/main.rs und modglue.rs, nur Consumer-/Credentialverdrahtung
- rust/Cargo.lock und das vorhandene consumerbezogene Prüfscript, soweit im bestätigten Quellbereich

Keine Community-/Privacy-/zentralen DB-Migrationen, Migrationsnummern, Guide-, Clip-, Punkte-, Systemd-, Provider- oder Admin-API-Arbeit. Zusätzlichen nötigen Pfad vor Schreiben mit genauer Begründung melden. Keine neuen Codekommentare. Produktiv ausschließlich Rust.

Vertrag: typisierter BrainClient, Pin 3b86d3cbe5ea39a67b8b1fbd8a3d48ab935982ef, fester Scope bot.public, bestehender Principal und normale TOML, geschützter bestehender Infisical-/FD-Credentialweg ohne ENV-Fallback. Vorhandenen Testmodus und Modus-/Fallbackgrenzen erhalten. Keine Credentials ausgeben, keine echten Konten oder Daten ändern, keine Produktionsanfrage.

## Prüfung und Abschluss

Graphify zuerst für jede neue Codefrage; stale Graphstellen am tatsächlichen eigenen Code verifizieren. Verwende vorhandene betroffene Suites und den bestätigten Consumerprüfweg. Rustup, höchstens zwei Jobs, beide blockierenden Hostlocks in der vorgeschriebenen Reihenfolge und konservative Probe direkt vor jedem Compilerstart nach HOSTPROBE.md. Keine Warte-Deadline für Hostlocks, keine fremden Prozesse stoppen. Vorhandene zentrale Rust-Buildablage benutzen. Format, vollständiges betroffenes Clippy mit -D warnings und betroffene Tests tatsächlich ausführen; vorhandene Tests nicht abschwächen. Falls ein bestehender Prüfschritt externe oder produktive DB-Daten erfordert, dessen Grenze melden und keinen fremden Datenbankstand verändern.

Eigene Codekorrekturen, soweit für genau diese Übernahme nötig, committieren; Commitnachrichten mit Co-Authored-By: Claude Code <noreply@anthropic.com> abschließen. Prüfung an vollständigen HEAD, Lockhash, Rust-/Cargo-Version und tatsächliche Exitcodes binden. `.consumer-ci-reports/` oder einen anderen nicht commitfähigen eigenen Belegpfad verwenden. Fremde Grünmeldungen gelten nicht als eigene Tests. Kein eigenes Review, Gate, Push, Merge, Deploy, Aktivieren oder Cleanup. Draftstatus von PR #459 nicht ändern, keinen PR neu anlegen.

Rückgabe: Worktree, Branch, frische Basis, vollständiger HEAD, wiederverwendete elf Quellcommits und Ziel-SHAs, Konfliktauflösung und Erhalt der Launcherkorrekturen, geänderte Pfade, echte Prüfzahlen/Exitcodes, sauberer Arbeitsbaum und konkreter Consumer-/Credential-/Request-ID-Vertrag für die spätere Z-Installation. Grenzen ehrlich benennen. Keine neuen Agenten oder T3-Threads, kein Modellwechsel. Eigene Kinder und Locks vollständig schließen.

## Routing

teil-k, Versuch 1; Haupt e6c19079-657e-4db9-80bd-8e1313e7f785. Höchstens drei aktive native Worker in K, kein weiterer Orchestrator. REGISTER.md und TODO.md nicht schreiben. Rückgabe an teil-k, sie pflegen die Akte. Deutsch, echte Umlaute, humanizer und no-em-dashes.
