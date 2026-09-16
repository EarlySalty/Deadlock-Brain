# Register: Reasoner für alle Helden

Stand: 16.09.2026. Delegator/Intent: aktueller ChatGPT-Chat via codex-mcp. Es gibt keine T3-Intent-Thread-ID für diesen Chat; keine erfinden. Repository /home/nathanael/repos/Deadlock-Brain. Ausgangs-HEAD 706b129. Main inzwischen ee440f3 (nur bisherige Auftragsdokumente). Kein Produktmerge oder Deploy durch diese Orchestrierung.

## Thread-Register (T3)

| Paket | Tatsächliche Thread-ID | Modell | Status | Worktree / Branch | Letzte verifizierte Meldung |
|---|---|---|---|---|---|
| 0 + A | 803d3e94-9b1d-42c5-9bb7-1905c8146acc | claude-opus-4-8, high | AKTIV, nicht freigegeben | /home/nathanael/repos/wt/brain-purpose-a ; feat/reasoner-purpose-a ; Basis 706b129 | T3 running; prüft bestehende spirit_rate-Konversion, Doppelzählung vermeiden; Baseline-/Mechanics-Sichtung, noch kein geprüfter Code-Commit |
| Review A | noch nicht gestartet | frischer unabhängiger Reviewer | WARTET auf fertigen A-Commit | separater Read-only-Review-Kontext | Keine Selbstfreigabe |
| B / Review B | noch nicht gestartet | je Implementierung opus48 | WARTET auf abgenommenes A | eigener neuer Worktree | Auftrag in PAKETE.md |
| C / Review C | noch nicht gestartet | je Implementierung opus48 | WARTET auf abgenommenes B | eigener neuer Worktree | Auftrag in PAKETE.md |
| D / Review D | noch nicht gestartet | je Implementierung opus48 | WARTET auf abgenommenes C | eigener neuer Worktree | Auftrag in PAKETE.md |
| E | noch nicht gestartet | unabhängige Gesamtabnahme | WARTET auf D und alle Qualitätsnachweise | Release nur durch Delegator | Kein Publish, keine neue hero_build_id |

## Orchestrierungsartefakte

Eigener reiner Doku-Worktree /home/nathanael/repos/wt/brain-purpose-control, Branch docs/reasoner-all-heroes, Basis 706b129. AUFTRAG.md erweitert den vorhandenen Warden-Befund um dynamische Alle-Helden-Abdeckung, PAKETE.md enthält konkrete B/C/D/E-Abnahmeaufträge und zusätzliche vorgeprüfte Lücken. Der Worker besitzt ausschließlich seine Implementierungsdateien und Phasenberichte; REGISTER.md/PAKETE.md pflegt der Delegator, nicht parallel der Worker.

## Betriebsregeln

Nur ein aktiver Implementierer. Keine Unter-Threads/Unter-Agenten. Reviewer erst am fertigen prüfbaren Patch. Gestoppte historische T3-Threads vom 12.-14.09. werden nicht neu gestartet oder verändert. Keine Fremdprozesse/Worktrees löschen. Keine Geheimnisse in Artefakten. Zentrale DB nur technisch erzwungen read-only. Kein zentraler Sync/Publish/Migrate/Service-Wechsel durch Worker.

45-Minuten-Wache ist NICHT eingerichtet: ChatGPT-Automationen erlauben höchstens stündliche Ausführung. Keine ersatzweise Wiederholung ohne ausdrückliche Freigabe, keine zugesagte Hintergrundfortsetzung. Während der aktuellen Bearbeitung wird der echte Threadzustand direkt gelesen. Nachfolgende Phasen entstehen nicht automatisch durch ihre Eintragung in diese Tabelle.

## Ende einer Phase

Fertigmeldung alleine genügt nicht. Commit/Tests/berichtete Metriken gegen Dateien prüfen; frischen Reviewer beauftragen; Befunde zurück an denselben Implementierer; erst nach unabhängiger Abnahme und nicht schlechterem eingefrorenen Vergleich weiterintegrieren. Gesamt-Staple-Gate und offene Abdeckungen bleiben bis E sichtbar rot. Nach tatsächlichem Merge/final gelesenem Review settle; Worktree erst nach gesicherter Integration/Live-Beleg entfernen. Niemals AKTIV als fertig ausgeben.
