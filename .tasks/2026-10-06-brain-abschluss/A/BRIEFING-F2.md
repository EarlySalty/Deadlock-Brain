# A-F2: Discord-Antwortfix abschließen

## Ziel und Vertrag

Nativer Blatt-Worker im Paket A, keine weiteren Worker/T3-Threads. Auftraggeber Paket-A-Session `2c7de4c9-bac4-43ad-b91a-f8ac889f09b4`, Hauptorchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Lies `../AUFTRAG.md`, `../BRIEFING-A.md`, `INVENTUR-BOTS.md` und den Originalvertrag `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-05-nebenfehler/BRIEFING-A.md` als fachliches Soll. Dessen alter Haltepunkt „kein Push/Deploy“ ist durch den heutigen ausdrücklich beauftragten Abschluss ersetzt; heutiger Gate bleibt verbindlich.

Vorhandener fertiger Code: Bots-Commit `b08d9366b06360fa5eae87fbdf3d118154f42930`, sauberer Altworktree `/home/nathanael/.worktrees/brain-antwort-20261006`. Auf aktuellem Bots-main integrieren, nicht neu erfinden. answered/build_rejected mit Links oder Überlänge dürfen keinen Aussetzer auslösen. Links entfernen, verbleibenden Text zeichensicher kürzen und zeigen; nur Link ohne Text ist leerer Wissensfall. Transport/Timeout/falscher Vertrag bleiben Fehler. Warnung nur Klasse, keine Frage/Nutzerkennung/Secrets. Ereignis-, Rechte-, Anfrage- und Scopebindung erhalten.

## Eigentum und Stand

Neuer eigener Worktree `/home/nathanael/.worktrees/brain-a-discord-20261006`, Branch `fix/brain-a-discord-20261006`, frisch von `origin/main` im Bots-Repo `/home/nathanael/repos/Deadlock-Bots`. Inventurbasis `600b832ac90b314512c60bff470f78b6914509dd`, vor Beginn frisch holen. Nur `rust/crates/dl-brain/src/brain_api.rs` und Tests darin. Keine Game-Invite-/modglue-/Knowledge-/Insights-Arbeit, keine weiteren Refactorings. B arbeitet parallel an Game Invites in einem anderen Eigentumsbereich. Keine Sessionnachrichten, kein Warten auf B; Deployment mechanisch über bestehenden serialisierten Wrapper am aktuellen Remote-main.

Nicht EnterWorktree verwenden. Keine alten Worktrees beschreiben, nur passende Altänderung ohne automatischen unverified Commit übernehmen. Eigene Dateien gezielt stagen, nach passenden grünen Prüfungen committen, Featurepush erlaubt. Commitnachricht endet mit `Co-authored-by: GPT 6.1 Sol <modell@local>`.

## Beweis und Abschlussbefugnis

Rust ohne neue Code-Kommentare. Graphify vor Suche. Vorhandene Fixtures, Compiler, fmt und Clippy sowie passende Suites mit den tatsächlichen Bots-Pflichtflags und eigener Test-DB ausführen. Eine vorhandene Warnbaseline mit Zahlen nachweisen, keine fremden Fehler unter diesem Paket fixen oder Tests abschwächen. Gezielte Link-/Überlängen-/Leerfall-/abgebrochene-HTTP-/Vertragsprüfungen aus dem vorhandenen Fix erhalten.

Danach `gate_hook.py --review` gegen frisches Bots-main. Bei BLOCK vollständigen Befund, Modell, Exit und Log zurückgeben, keine eigene weitere Fixerrunde. Nach ALLOW ausdrücklich befugt: regulär nach main integrieren, `git push origin HEAD:main`, bestehenden Bots-Deploy am dann aktuellen Remote-main, betroffene Dienste restarten, Binary/PID/Journal/Anker und normalen Funktionspfad belegen, eigenen Branch/Worktree sicher aufräumen. Pro Git-Aufruf genau ein Git-Schritt mit literalem absolutem Pfad, keine Forcepushes. Branchlöschung erst nach `merge-base --is-ancestor` und geprüftem Exit. Schutz-Hooks nicht umgehen.

Keine neuen Testnachrichten an Community oder andere Personen ohne bestehende ausdrückliche Freigabe. Bestehenden echten Antwortpfad beziehungsweise vorhandenen Betriebsdiagnoseweg prüfen. Falls ein echter neuer Eingang für den vollständigen Zustellbeleg fehlt, den fehlenden Beweisteil ehrlich melden, nicht als fertig ausgeben. Keine neuen LLM-Connectoren, Modelle oder selbst erfundenen Zeitlimits. Privatdaten nicht an zusätzliche Modelle senden.

## Übergabe

Versuch 1, Worker als Statusproduzent, Paket A wacht nach 20 Minuten. Native Rückgabe statt root-Dateischreiben: Feature- und Main-SHA, Gebaut/Reviewt/Gemergt/Live getrennt, Prüfkommandos und Zahlen, Gatewortlaut/Modell/Log, Deployweg, PID/Exe/Journal/Anker/Funktion, Cleanup-Exitcodes und MERGEPROTOKOLL. Keine root-Akte, TODO.md oder AN_HAUPT-A.md schreiben. Bei Gate-/Rechte-/Betriebsblocker Stand sicher erhalten und direkt als Rückgabe melden.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-a-discord-20261006
