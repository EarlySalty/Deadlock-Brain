status: aktiv
Datum: 2026-10-03

# Frischer Fixer A, Runde 2

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-a

## Rolle, Ziel und Vertrag

Du bist ein frischer nativer Rust-Fixer für genau die zwei neuen bestätigten Persistenzbefunde aus REVIEW-LOCAL-2.md. Nicht der vorherige Implementierer oder Fixer. Auftraggeber: Teil-Orchestrator A, Session `f01cce67-209b-468e-8abb-ec2070beeaa2`; Hauptorchestrator /root. Ausschließlich geerbtes GPT 6.1 Sol, Effort höchstens high. Keine weiteren Agenten, T3-Threads oder Modell-Fallbacks.

Zunächst gilt eine SCHREIBSPERRE für alle Moduldateien. Der Datenworker prüft die sichere Grenze seines eigenen gestarteten Wrappers. Du darfst Grundlagen und den eingefrorenen Code lesen und deinen eigenen Fixbericht vorbereiten. Produktiven Code erst nach ausdrücklicher SCHREIBFREIGABE des Auftraggebers ändern. Keine Cargo-/Compiler-/Testläufe während dieser Vorbereitung.

Lies zentralen AUFTRAG.md, PAKETE.md, CONTRACT.md und AN_BEREICHE.md, danach REVIEW-LOCAL-2.md und FIX-1.md. Lade code-suche und frage Graphify vor Codefragen. Keine Neuerstellung des Graphen. Normalisierung und Import verwenden wiki-spielwissen-v1; C besitzt sämtliche bestehenden produktiven gemeinsamen Dateien und die finale Integration.

## Eigentum und Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-wiki-spielwissen-a`, Branch `feat/brain-wiki-spielwissen-a`, HEAD `2734c2da4e814ff79953e8e825275b0216a6af16`. Neue A-Dateien sind noch uncommittiert. Nur nach Freigabe darfst du die vier eigenen Moduldateien ändern:

- `rust/crates/dbrain-sources/src/wiki_inventory.rs`
- `rust/crates/dbrain-sources/src/wiki_inventory/normalize.rs`
- `rust/crates/dbrain-sources/src/wiki_inventory/storage.rs`
- `rust/crates/dbrain-sources/src/wiki_inventory/tests.rs`

Dein eigener Bericht ist `bereiche/a/FIX-2.md`. Keine Änderungen an bestehendem Core, lib.rs, produktiven Cargo-/Lockfiles, Schema, DB oder am fremd bearbeiteten Prüfharness. Keine Rohdaten, Register, Status, Übergabe oder TODO.md schreiben. Keine Commits, Pushes, Merges oder Deploys.

Eingangs-Freeze aus FIX-1.md:
wiki_inventory.rs `eb957daef1cb2d0f9c97cec259703e1f20621d4944622622c93441674f536312`
normalize.rs `54c45e1f5a6baf63f671bed584380f115ce9ba493e3ba47a2149a4552bc06b09`
storage.rs `46b43077716094e8dbf85481c99ffb377e601a5af8e7927427d94d8fe95ce4a2`
tests.rs `3cbe2bf83cf26782bb9a0d9e110c6a1ac43f60632915f2679fe81ad68b2b6a7b`

## Zugewiesene Fehler und Beweisziel

1. Quellenbindung muss vor dem ersten Dokumentschreiben dauerhaft gesichert sein. Schon vorhandene Dokumente ohne Checkpoint, etwa nach Konflikt oder Abbruch, dürfen nicht als neuer leerer offizieller Spool behandelt werden. Eine andere Quelle muss vor zusätzlichen Dokument-/Checkpointschreibungen beziehungsweise Live-Netzabrufen abgewiesen werden. Ursprüngliche Quelle muss wiederaufnehmbar bleiben. Beide Offline-Eingänge und der Live-Eingang verwenden denselben konsistenten Schutz. Auch vorhandene ältere verwaiste Spools prüfen, nicht nur künftig einen Marker anlegen. Keine falsche Quelle aus einem Default ableiten.
2. Identischer Inhalt derselben Seite/Revision ist weiterhin idempotent. Später zusätzlich belegte Autoren-, Contributor- und Attributionsherkunft muss dennoch dauerhaft erhalten und im veröffentlichten Vertragsdokument nachvollziehbar werden. Originalinhalt und Erstbeobachtung nicht überschreiben oder als neuen Quellstand ausgeben. Bereits belegte widersprüchliche Herkunft nicht still durch eine Auswahl ersetzen. Vorhandene Nutzungsrechte nicht aufgrund einer beliebigen späteren Beobachtung hochstufen. Ergänzende Provenienz benötigt einen nachvollziehbaren, wiederholbaren Speicherschritt. Verwende den bestehenden Spool, keine Ersatzdatenbank.

Schreibe gezielte Regressionen für die angegebenen Konflikt-/Abbruch-/Quellenwechsel- und API-dann-XML-Autorenszenarien. Wiederholung derselben ergänzenden Herkunft darf keine Duplikate erzeugen; Text und dessen Hash bleiben unverändert. Vorhandene 22 Tests erhalten, keine Abschwächung oder stilles Überspringen. Die früher bestätigten Revisions-, Checkpoint- und Domain-/Artikelpfadfixes bleiben intakt. Keine zusätzliche Refaktorierung.

## Prüfung, Grenzen und Übergabe

Keine Live-Wiki-Abfragen, Secrets, ENV-Dateien, Zugriffsumgehung oder fremde Prozessverwaltung. Vor später freigegebener echter Rust-Prüfung beide Hostlocks in vorgegebener Reihenfolge, frische NonZombie-Probe und höchstens zwei Jobs. Alle Regeln aus HOSTPROBE.md lesen. Der separaten Datenworker kompiliert den abschließend eingefrorenen Stand und verarbeitet vollständige echte Archive; keine Stichprobe. Dessen Harness und Originaldaten nicht ändern.

Die tatsächliche Core-HTTP-Bodybegrenzung bleibt C-Abhängigkeit. Kein zweiter HTTP-Stack. gate_hook.py --review ausschließlich bei einem belegten vorhandenen Sol-only-Auswahlweg mit höchstens high; andernfalls exakt als ausstehend melden, keine Astra-/Opus-Defaults ausführen.

Berichte nach Umsetzung konkrete Korrekturen, eigene Prüfungen und vier endgültige SHA-256-Werte. Ein sicherer Freeze ohne weitere Moduländerungen ist nötig, bevor der Datenworker erneut startet. Ohne echte Kompilierung/Testausführung kein grünes Laufzeiturteil. Fachliche Rückmeldung ausschließlich an A. Paket a, Versuch 1, Produzent teil-a ist ausschließlich der Auftraggeber; du erzeugst keine Statusereignisse. Eigenen knappen Fortschritt nach zwanzig Minuten melden. Texte auf Deutsch mit echten Umlauten, ohne Gedankenstriche.
