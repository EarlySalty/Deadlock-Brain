# Paket-A-Integration

## Eigentum

Eigener sauberer Startworktree: `/home/nathanael/.worktrees/brain-a-abschluss-20261006`, Branch `feat/brain-a-abschluss-20261006`, Basis `d6131cc52711a3e8b02d299704244f8d7dbdbce6`. Produktive Änderungen nur in eigenen Worktrees unter `~/.worktrees/brain-*`. Geteilter Kanon und alte Threads bleiben unberührt.

Die Bestandsaufnahme ist als nativer Workflow `wf_5a3557d7-878` mit vier read-only Workern abgeschlossen. Paket A hat die beauftragten Berichte und `A/STAND.md` im Kanon gespeichert. Die Teil-Orchestrator-Session ist `2c7de4c9-bac4-43ad-b91a-f8ac889f09b4`, Modell `gpt-6.1-sol[1m]`, UltraCode tatsächlich gestartet, Worker mit geerbtem Modell und `xhigh`.

Historischer Isolationsschutz: EnterWorktree blockierte das nachträgliche Editieren der Root-Akte. Nach regulärem ExitWorktree mit keep schreibt Paket A wieder ausschließlich seine beauftragten Akten im Kanon. Eigene Sourcearbeit bleibt in den getrennten eigenen Worktrees. Kein globaler Worktreewechsel während laufender nativer Worker und keine Umgehung gesperrter Edits.

## Bereits belegte Betriebsmechanik

- Aktuelles `origin/main`: `d6131cc52711a3e8b02d299704244f8d7dbdbce6`, vor Beginn frisch geholt.
- Regulärer Release-Helfer existiert root-eigen unter `/usr/local/libexec/brain-release`.
- `brain-release plan` am sauberen Startworktree: Exit 0, Quell-SHA und Remote-main gleich, Sourcefingerprint geprüft. CLI- und Wartungszeiger zeigen beide `e56e075d486a75f83f4954b58d8113588082d3f1`. Das ist ein Herkunftsbeleg, noch kein Funktionsbeleg.
- `gate_hook.py` liegt tatsächlich unter `/home/nathanael/Documents/.claude/gpt-workers/`. Aufruf: `python3 .../gate_hook.py --review --repo <absoluter Worktree> --base <Basis-SHA> --head <Kandidat-SHA>`. Ohne Modelloverride läuft die konfigurierte Kette. `--help` ohne `--review` ist kein gültiger CLI-Aufruf und lieferte JSON-stdin-Fehler; dieser Versuch ist kein Gateurteil. `--review --help` funktioniert.
- Der Release-Helfer akzeptiert nur saubere eigene Quellen auf tatsächlichem Remote-main. Releasebau und Herkunftsnachbau verwenden private Targets außerhalb der Quelle. Für den späteren Release ein eigenes sauberes Quellen-Worktree verwenden, keine Testtargets oder Task-WIP als saubere Quelle ausgeben.
- ConfigWriter ist bereits vorhanden: `rust/crates/brain-maintenance/src/integration/config_writer.rs`. `write_serve_config` ersetzt atomar unter gemeinsamer Sperre mit erwartetem Hash. Keine direkte Änderung produktiver DB-Zustände und kein zweiter Konfigurationspfad.

## Aktueller Arbeitsvertrag

Inventur und vier erste Fertigbaupakete sind belegt. F1 verarbeitet Profile/Git, F4 Batchstatus; F3b portiert den bestehenden Sitedienst einschließlich eng zugewiesener PostgreSQL-Kommentare. Der erste Discord-Fix ist als `c4508fb3` remote gesichert, Gate gpt-6.1-sol BLOCK wegen URL-Rekombination; frischer Fixer F2c, kein Merge oder Deploy.

Neue ausdrückliche Nutzerentscheidung: Priorität 1 neben den Steckbriefen ist ein gemeinsames Brain mit Skills. A-E1/E2 ermitteln vorhandene Antwortpfade und Invite-Statusvertrag; A übernimmt anschließend den lesenden Skill am gemeinsamen Antwortweg. B besitzt weiterhin Invite-Mechanik und entfernt die fehlerhafte verspätete Lounge-Antwort sofort. Für den eigenen Invite-Status ist ausschließlich Enum plus Zeitpunkt im bestehenden Provider ausdrücklich zulässig, keine Steam-IDs/Namen/Fremddaten. Bestehende gewollte funktionierende Antworten bleiben bis zum live belegten Ersatz erhalten.

Kein Verdachtsfix, Lock-/Scope-Bypass oder eigenmächtiger Modellwechsel. Stufe 1 vor Stufe 2, Reasoner danach mit regulärer Qualitätsgrenze und echtem 779996-Beleg. Zurückgestellte Funktionen bleiben unangetastet. `C/OFFEN.md` ist entschieden: 44 alte Stände erhalten, 18 verwerfen; alle 21 neuen WIPs und geschützten lokalen Artefakte erhalten. A hat nichts gelöscht.
