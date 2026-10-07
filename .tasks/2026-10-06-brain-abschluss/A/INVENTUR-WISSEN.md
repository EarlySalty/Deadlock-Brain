# Inventur A-I2: Spielwissen und Wiki

Read-only Rückgabe des nativen Workers `A-I2`, von Paket A als Akte gespeichert. Der Worker konnte den Bericht während des zwischenzeitlichen Worktree-Isolationsschutzes nicht schreiben. Diese Grenze wurde nicht durch Shellzugriffe oder Delegation umgangen. Produktivcode unverändert.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/brain-maintenance/src/integration/runner.rs:668 | Anknüpfung: vorhandene Faktenprojektion, Originalbindungen, Git-Quittungen, Entitätsreader und gemeinsamer Renderer

## Stufe 1: halb

- Main `d6131cc52711a3e8b02d299704244f8d7dbdbce6`, laufender Release `e56e075d486a75f83f4954b58d8113588082d3f1`.
- Neun Wartungsticks melden am GameTracking-Pin `8c7cf4e` einen Fehler im Schritt „Git-Dateiliste“. Frühere fehlende Source-Scopes stehen inzwischen in der Konfiguration. Dieselbe Git-Leseabfrage funktioniert außerhalb des Dienstes. Eine Dienst-/Ausführungskontextabweichung ist damit der erste konkrete Diagnosepunkt, kein Anlass für neue Allokations- oder Scopeänderungen auf Verdacht.
- `/home/nathanael/Documents/deadlock-build-corpus/site/entities` fehlt vollständig, `/site/entities/` HTTP 404. Reader und Renderer sind angeschlossen. Health/Ready 200 belegt nur den aktiven Docs-Release. Kein vollständiger Beleg konsumierter Spielprofile oder drei normaler Antworten.

## Stufe 2: halb

Wiki-Eingaben tatsächlich geprüft: 38.273 Dokumentversionen, 1.109.153 Fakten, passende Hashes. Der produktive Quittungskern akzeptiert ausschließlich Git-Spielprofile. Erweiterte Intervalle fehlen absichtlich auf Main. Wiederverwendbarer Fachkern: `/home/nathanael/.worktrees/brain-spielwissen-zuordnung-20261004/rust/crates/brain-storage/src/entity_intervals.rs`.

## Altarbeit

Produktbasis ist Main. Produktive D5-Fixbranches sind integriert; Stufe-1-, Zuordnungs- und Wiki-Branches haben patchäquivalente Commits. Drei Diagnosecommits bis `bb3ebf3` sind optional. Uncommittierten Bestand unter `/home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration` erhalten und selektiv prüfen, nicht pauschal anwenden. Depotbestand bleibt zurückgestellt.

## Schreibbereiche und Reihenfolge

Zuerst vorhandene Integration unter `rust/crates/brain-maintenance/src/integration/` für Git-Laufzeit, Profilaktivierung und HTML fertigstellen. Erst nach belegter Stufe 1 gemeinsame Profilmodule unter `rust/crates/brain-storage/src/` und Binder unter `rust/crates/dbrain-sources/src/` für Wiki-Quittungen und erweiterte Intervalle weiterführen. Kein zweiter Profil- oder Rendererpfad.
