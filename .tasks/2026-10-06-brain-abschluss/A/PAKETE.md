# Paket A: Fertigbaupakete

Integrationsverantwortlich: Paket-A-Session `2c7de4c9-bac4-43ad-b91a-f8ac889f09b4`. Hauptorchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Native Worker, geerbtes GPT 6.1 Sol, keine zusätzlichen T3-Threads und keine weiteren Delegationsebenen.

| Paket | Ziel | Schreibbereich | Abschlussverantwortung |
|---|---|---|---|
| A-F1 | Git-Dienstfehler und Stufe-1-Profilverarbeitung | Brain `brain-maintenance/src/integration/`, konkret nötige bestehende Git-/Importmodule in `dbrain-sources` | Worker baut, prüft, Feature-SHA und Gate; Paket A integriert und deployt Brain |
| A-F2 | Vorhandenen Discord-Antwortfix übernehmen | Bots ausschließlich `dl-brain/src/brain_api.rs` und eigene Tests darin | Worker führt Gate, Merge, bestehenden Bots-Deploy, Restart, Liveprüfung und Cleanup selbst durch |
| A-F3 | Vorhandene Brain-Site auf Rust und bestehende Steckbriefauslieferung anschließen | Brain bestehende Site-Bausteine, gegebenenfalls eigener Site-Bin im `deadlock-brain`-Crate; keine Änderung von `main.rs` | Worker baut, prüft und liefert Feature-SHA/Gate; Paket A deployt gemeinsam mit Profilpfad |
| A-F4 | Sheet-Batches dürfen echte Fehler nicht als Exit 0 melden | Brain `deadlock-brain/src/main.rs`, `dbrain-enrich/src/lib.rs`, vorhandene zugehörige Tests | Worker baut, prüft und liefert Feature-SHA/Gate; Paket A integriert vor Releaseverdrahtung der Unit |
| A-C | Nicht integrierte Altbranches bewerten | Read-only Branchvergleich, Rückgaben an Paket A; allein Paket A editiert `C/OFFEN.md` | C setzt nur belegte Entscheidungen um |

A-F1 und A-F3 hängen bei Auslieferung zusammen. Beide bleiben getrennte Code-Eigentümer, gemeinsame Brain-Integration und Live-Abnahme durch Paket A. Stage 2 und Reasoner starten erst nach dem belegten Stufe-1-Anschluss; deren vorhandene Artefakte werden erhalten. Keine Worker dürfen Änderungen anderer Pakete zurücksetzen oder mitformatieren. Game Invites gehören ausschließlich B.

Root-Akte und Berichte schreibt Paket A. Native Workflow-Worker geben ihre vollständigen Belege als Rückgabe; damit bleiben Berichtspfad und Codeisolation getrennt. Für Sourcearbeit normale Git-Worktrees unter `~/.worktrees/brain-*` mit literalen absoluten Pfaden verwenden, nicht per EnterWorktree die gemeinsam aktive Sitzungsisolation wechseln.
