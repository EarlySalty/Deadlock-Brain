# Fixer 12: Legacy-GID-Token und kanonische Bestandszuordnung

Frischer nativer Kontext nach tatsächlichem BLOCK des Fixers 11. Derselbe offene I-Auftrag, autorisiert durch `ENTSCHEIDUNG-I-PATCH-ORIGINAL.md` vom 07.10.2026, 16:09 UTC. Bestehende Regeln und Briefing `FIXER-11-AUFTRAG.md` gelten weiterhin.

## Ausgang und Restkerne

Eigener Worktree `/home/nathanael/.worktrees/brain-e-deadlock-api`, vorhandener Branch `feat/brain-deadlock-api-daten`, HEAD `aca42a505ff3b6c427352c4fde5651511f589678`. Erhalte den verifizierten bisherigen Fix, Gegenprobe 90c17800 und Integrationskandidat 860793d7. Ausschließlich vorhandene Patch-/HTML-/URL-Strecke und zugehörige Regressionen. Kein neuer Thread oder Arbeitsbaum, keine G/K-Datei oder Umgehung ihrer Prüfablehnungen.

1. Begrenzten berechtigten Kern des GID-BLOCKs prüfen: Bloßes abschließendes `\"&gt;` wird bereits normalisiert. Direkt anschließender HTML-Linktext ohne Leerzeichen kann aber Teil des URL-Tokens bleiben, die neue GID-Erkennung verhindern und Fremdinhalt über Legacy-Fallback freigeben. Quelle/URL/GID-Vertrag lesen, vorhandene Normalisierung und Kennungsextraktion wiederverwenden. Konkrete Identität darf durch HTML-Tokenreste nicht zu einer unspezifischen Anfrage werden. Negative und positive Legacy-/Original-Regressionsfälle; keine pauschale Fremdinhaltsfreigabe oder zweite Pipeline. Begründen, welche wörtliche Gatevariante bereits abgefangen ist und welchen tatsächlichen Restkern du korrigierst.
2. Kanonische URL bis in die Bestandszuordnung verwenden. Aktueller Importlookup nutzt den unveränderten Feed-Link; Fragmentlink findet deshalb einen bestehenden fragmentfreien changelog_posts-Eintrag nicht, wenn noch kein brain.source_documents-Eintrag existiert. Bereits vorhandene Patch-ID erhalten; Prüfung, Abruf und Lookup konsistent, ursprünglicher Feedlink/Herkunft nachvollziehbar. Tatsächlichen vorhandenen Lookup-Weg korrigieren, nicht künstlich DB-Zustände reparieren. Grenze für Origin/Schema/Redirects/Größe/Timeout und andere Fehler unverändert.

Originalurteil `/tmp/brain-e-fixer11-gate-opus55.log`, Runde 16 in `REVIEW.md`. Bisherige Prüfung des Fixers 11: 42 Patchfälle und 151 gesamte Cratefälle bestanden, 2 ignored im Gesamtlauf; passende Compiler-/Format-/Clippyprüfungen Exit 0. Kein ALLOW oder Mainfreigabe daraus ableiten.

## Vorgehen und Rückgabe

Graphify/code-suche zuerst. Keine neuen produktiven Kommentare, ENV-Konfigurationen, Modelle/Connectoren, Matchablagen oder neue Pipeline. Nur Rust und vorhandener zugelassener I-Prüfweg; bei Deny keinen Worker-/Wrapper-/Toolwechsel oder Hookänderung als Umgehung. Format nur eigene Dateien mit rustfmt, cargo fmt nur --check. Passende Tests mit entfernten Produktiv-DSNs, SQLX_OFFLINE=true, vorhandenen Cargo-Slots und isolierten Scratch-DBs. Keine produktiven SQL-Schreibproben. Kontrollierte HTML-Proben nicht als echte Liveantwort ausgeben; Bildlink-NIT bleibt separat offen.

Committe verifizierten Fix mit Projekttrailer `Co-authored-by: GPT 6.1 Sol <gpt-6.1-sol@local>`, nur eigene Quelldateien und eigener Fixerbericht. `FIXER-11-AUFTRAG.md`, `FIXER-12-AUFTRAG.md`, REGISTER/REVIEW und Hauptbericht sind Hauptsession-Artefakte und bleiben ungestaged/unverändert. Ein Git-Schritt pro Bash, absolute Pfade. Eigene Selbstprüfung gegen Ausgang ACA42 mit `/home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review`, zwingend `--model claude-opus-5-5`. Bei BLOCK Urteil/Restkern zurückgeben, keine weitere Fixrunde im eigenen Kontext und kein Modellwechsel. Bei ALLOW nur Featurebranch sichern und Fix-SHA, konkrete Prüfungen/Anzahlen/Exitcodes und vollständiges Urteil zurückgeben. Kein Main, Deploy, Restart, Cleanup oder Self-Settle.

Keine Sessionkontakte oder weitere Subagenten. Sonnet/Fable verboten. Nutzer-/Community-Daten MUST NOT an externe Modelle, auch nicht über Loopbackproxy; Secrets NEVER ausgeben. Falls Browserarbeit: zuerst agent-browser.md lesen, ausschließlich Moli, MUST NOT Brave oder persönliche Browser.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-e-deadlock-api
