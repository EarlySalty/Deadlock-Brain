status: aktiv
Datum: 2026-10-03

# C2 Modell- und Gatevorprüfung

Aktiver Elternprozess 2854855: `--model gpt-6.1-sol[1m] --effort high --settings /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-wiki-spielwissen/startweg/claude-sol.json`. Eigene Session a17ac7e9-7f41-44b0-a6e4-901bfafe544f, Auftragproxy 127.0.0.1:18768. ANTHROPIC_MODEL und die drei nativen Modellzuordnungen zeigen ausschließlich gpt-6.1-sol[1m]. Kein ultracode/xhigh/max. Native coder-Definition ohne Modell-/Effortoverride, daher Vererbung. Kein fremder Starter verwendet.

Die im Briefing genannten Codex-Skills wurden mit den geladenen nativen Skills verglichen. rolle-teil-orchestrator, rolle-worker-briefing, t3-threads-managen, humanizer und no-em-dashes haben identische Inhalte ohne Frontmatter. graphify unterscheidet sich bei Plattformausführung und Ausgabeformat der Neuindexierung; vorhandene Graphify-Abfragen sind in beiden vorgesehen, hier keine Neuindexierung oder semantischen Fremdmodelle.

Vorhandener Gate: /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py. Ein expliziter Einzelmodelllauf ist vorgesehen: `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration --base origin/main --head <integrierter-SHA> --model gpt-6.1-sol --effort high`. Noch nicht ausgeführt.

Quellbelege: Argumentparser Zeilen 5439-5496, `run_review` 5362-5370, Codex-Kindaufruf 5198-5214. GPT-Namen verwenden den regulären isolierten Codex-Gatepfad, dessen Kind explizit `-m gpt-6.1-sol` und `model_reasoning_effort="high"` erhält. Kein direkter eigener Codex-Reviewstarter und keine Änderung am Gate.

Aktuelle T3-Pyramide enthält gpt-6.1-sol als ersten Reviewer in review_1 und ausschließlich Sol in review_5. `modellkette_fuer_bereich` berücksichtigt einen gültigen SHA-bezogenen ALLOW aus der konfigurierten Kette. Vor Merge muss nachgewiesen werden, dass exakt der integrierte SHA durch Sol freigegeben ist, damit der automatische Hook diesen echten Nachweis verwenden kann. Ein fehlendes Urteil ist kein ALLOW. Bei fehlender Sol-Freigabe keine automatische Kette starten, deren technische Rückfälle andere Modelle wählen könnten. Keine globale Pyramiden-/Settingsänderung oder Hook-Umgehung.

Bisher nur statischer Auswahlbeleg, kein Gate-Urteil und kein Mergebeweis. Tatsächliche Kindauswahl und Urteil werden beim autorisierten finalen Einzelmodelllauf dokumentiert.
