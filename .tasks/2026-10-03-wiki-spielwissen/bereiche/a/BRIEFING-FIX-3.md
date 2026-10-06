status: aktiv
Datum: 2026-10-03

# Frischer Fixer A, Runde 3

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-a

Rolle: frischer nativer Rust-Fixer ausschließlich für die beiden bestätigten Funde aus REVIEW-LOCAL-3.md. Auftraggeber Teil-Orchestrator A f01cce67-209b-468e-8abb-ec2070beeaa2, Hauptorchestrator /root. Geerbtes GPT 6.1 Sol, nur high/medium, keine Unteragenten oder T3-Threads. Keine Modell- oder Gate-Fallbacks.

## Ziel und Vertrag

wiki-spielwissen-v1: vollständiges erreichbares historisches Wissen mit Original, Revision, Herkunft und Lizenz erhalten. Originale und Erstbeobachtungen nicht überschreiben. Lies zentralen AUFTRAG.md, CONTRACT.md, AN_BEREICHE.md einschließlich Punkt26, dann REVIEW-LOCAL-3.md und FIX-2.md. Lade code-suche und frage Graphify vor Codefragen, konkrete Fundstellen nachlesen, keinen Graph neu bauen.

## Eigentum, Stand und Freigabe

Worktree /home/nathanael/.worktrees/brain-wiki-spielwissen-a, Branch feat/brain-wiki-spielwissen-a, HEAD 2734c2da4e814ff79953e8e825275b0216a6af16. Neue eigene A-Dateien uncommittiert. SCHREIBFREIGABE liegt vor: Datenworker hat ausschließlich seinen eigenen wartenden Task ba5ocz35n vor Cargo beendet und um 05:50:10 UTC keinen eigenen verbleibenden Wrapper, Lock-FD oder Compilerkind bestätigt. Originale und Harness erhalten. Er bleibt bis zum nächsten Freeze pausiert.

Du darfst ausschließlich die vier neuen A-Moduldateien ändern: wiki_inventory.rs und wiki_inventory/normalize.rs, storage.rs, tests.rs unter rust/crates/dbrain-sources/src/. Dein eigener Bericht ist bereiche/a/FIX-3.md. Keine Core-, lib.rs-, produktiven Cargo-/Lockfile-, Schema-, DB-, Harness-, Originaldaten-, Register-, Status- oder TODO-Änderung. Keine Commits, Pushes, Merges oder Deploys. Kein zusätzlicher Refactor.

Eingangs-Freeze gemäß FIX-2.md und unabhängigem Review:
wiki_inventory.rs 2507c127aaf8e366f7bb7cf22564074adbaa546b344328d6fdbd593e5b38ef80
normalize.rs 27e63412a7a57c956dc82e602df4f93683adb3eb06866b22381157f85b96dcdc
storage.rs 88ff521c77032215ba2aef8b4dc4abee20fd43bedbcf5ab218620acbb43e7498
tests.rs 82356a793f8981f021ef6ab9fcf420bfc529fa445243dcff04df2ed2749e7d91

## Genau zwei Korrekturen

1. Konfliktdokumente in dieselbe tatsächliche Gesamtgrößenzählung aufnehmen, beim Öffnen und nach erfolgreichem neuem Schreiben. Bereits vorhandene identische Konflikterfassung nicht doppelt zählen. Vor Konfliktschreiben dieselben zulässigen Größen-/Überlaufgrenzen prüfen wie bei Originalen und Herkunftsergänzungen. Ein abgewiesener zu großer Konflikt darf weder Original überschreiben noch einen anschließend unlesbaren Konflikt erzeugen, der normale Wiederaufnahme verhindert. Grenzen nicht erhöhen. Keine vorhandenen Dokumente oder Konflikte löschen. Ressourcenfehler sichtbar melden; nicht behaupten, ein wegen Grenze nicht gespeicherter Eingang sei vollständig erfasst.
2. Erstmalige Anlage von provenance/ und conflicts/ vor der erfolgreich bestätigten dauerhaften Dateisicherung wirklich dauerhaft machen: neue Verzeichniseinträge und ihre nötigen Eltern synchronisieren, nicht erst zufällig beim späteren Checkpoint. Vorhandene write_atomic-/write_immutable-Kopplung und neue Spoolverzeichnisse systematisch prüfen. Keine nicht ausgeführte Stromausfallprüfung vortäuschen. Keine Host-/Kernel-Crashversuche, keine fremden Daten-/Prozessänderungen.

Erhalte Quellenbindung, monotones Inventar, bekannte/lesbare Revision, Original-/Provenienztrennung, Domain-/Artikelpfade und alle 29 bestehenden Tests. Schreibe gezielte Regressionen für Konfliktbudget einschließlich Wiederöffnen und idempotentem Konflikt, abgewiesenes übergroßes angereichertes Dokument mit intakter Originalwiederaufnahme und passende neue Verzeichnis-/Dauerhaftigkeitsmechanik. Tests und Compiler laufen später beim separaten Datenworker; kein Rot-vor-Fix-Nachweis nötig. Keine Fake-Dauerhaftigkeit als echte Power-Crash-Verifikation ausgeben.

## Prüfung, Routing und Übergabe

Keine Live-Wiki-Anfragen, Secrets, ENV-Dateien oder Zugriffsumgehung. Keine Cargo-/rustc-/Clippy-/Test-/Release-/Datenläufe in diesem Fixauftrag, solange der Datenworker die tatsächliche Prüfung übernimmt. Lies HOSTPROBE.md für seine beiden zwingenden Sperren und die frische NonZombie-Probe; keine fremden Compiler beenden. Gezielt nur eigene Dateien formatieren und abschließend vier SHA-256-Werte messen. Quellenstand und tatsächlich ausgeführte Prüfungen getrennt melden.

Punkt26: get_bounded existiert auf aktueller C-Mainbasis bereits, C2 prüft konkrete Anbindung und Laufzeitgrenze. Kein neuer Core-Netzpfad oder Coreumbau durch dich. gate_hook.py --review ausschließlich mit belegtem vorhandenen Sol-only-Weg, andernfalls als ausstehend führen und keine anderen Defaults starten.

Melde Korrekturen, echte Prüfungen und sicheren abschließenden Freeze ohne weitere Writes. Kein grüner Compiler-/Echtdaten-/Gesamtstatus ohne ausgeführten Beweis. Rückmeldung nur an A; Paket a/Versuch 1, A alleiniger Statusproduzent teil-a. Keine Statusdateien oder TODO.md. Fortschritt nach etwa zwanzig Minuten bei weiterlaufender Arbeit. Bericht auf Deutsch mit echten Umlauten ohne Gedankenstriche.
