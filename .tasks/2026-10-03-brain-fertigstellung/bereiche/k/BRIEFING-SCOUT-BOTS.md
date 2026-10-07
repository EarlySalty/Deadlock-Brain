status: beauftragt
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: noch nicht angelegt, ausschließlich Bestandsaufnahme

# Geschützten Bots-Consumer zur Übernahme erfassen

## Ziel und Vertrag

Paket K umfasst nur BrainClient-Anbindung in Deadlock-Bots, bestehenden PR #459 abschließen. Lies GEMEINSAM.md, AUFTRAG.md, BRIEFING-K.md, UEBERNAHME-CODEX.md und aktuelle PAKETE.md zwei Ordner höher. Gemeinsame Integration und Produktivwechsel bei Z, keine Einzelmerges. Geschützte Threads a99dc9e9-3ce1-41bc-ba6b-391cc190f518 und c0b1d111-e402-4ed3-bb0b-68958a1ba699 nicht anschreiben, stoppen oder settlen. Letzte Wache: erster stopped, zweiter ready; ein eigener Migratorresolver des fremden Threads war noch aktiv. Verwende ausschließlich unveränderliche Git-Objekte, keine fremden Arbeitsdateien übernehmen oder ändern.

## Eigentum

Nur lesende Bestandsaufnahme: Git-Refs/Objekte und deklarierte Aufgabenartefakte. Kein fremder Worktree wird beschrieben. Kein eigener Worktree, Cherry-pick, Commit, Push, Merge, Gate, Test, Release, Deploy, Secretzugriff oder Produktivaufruf in diesem Auftrag. PR bleibt Draft, keine GitHub-Actions-Aufträge. Graphify vor Codesuche, dann exakten Quellstand verifizieren. Keine weiteren Agenten oder T3-Threads.

## Bestand und Beweisziel

Frische geschützte Threadzustände über ausschließlich t3-thread.py read ermitteln. Existierenden PR-Zustand und Remote-Kopf lesen. Fremder Integrator meldete zuletzt Quelle 635f6b6b mit geprüften Privacy- und Consumer-Suiten, aber ohne gültiges Standardgateurteil und mit noch offenem Resolveranschluss. PR-Kopf e805fbed60a9176984538f2edaa3960209c589ab ist älter und darf nicht ungeprüft bevorzugt werden.

Ermittle genaue Branch-/Commit-SHAs und den kleinsten wiederverwendbaren Consumerstand. Vergleiche gegen origin/main. Benenne welche Commits sauber die BrainClient-Anbindung enthalten und welche untrennbare Privacy-/Community-/Migratorabhängigkeiten mitbringen. Vorhandene neuere Sicherheitskorrekturen erhalten, keinen alten unsicheren Adapter zurückbringen. Nichts neu bauen. Quellcode und tatsächliche Tests unterscheiden; fremde grüne Meldung ist kein eigener Prüfnachweis.

## Ausgabe

Knappe Übernahmekarte: Schutzstatus, PR offen/Draft/Kopf, Basis-SHA, vorhandene eigene Worktrees, unveränderliche Quellkandidaten mit genauen SHAs und betroffenen Pfaden, verwendbarer Commitbereich und gekoppelte Integrationsabhängigkeiten. Keine eigene Bug-/Security-Review. Wenn Übernahme nicht sicher möglich ist, konkrete Grenze statt neuem Featureentwurf melden. Ergebnis als Rückgabe, Berichtdatei nicht erforderlich.

## Routing

teil-k, Paket K, Versuch 1. Neuer Hauptorchestrator e6c19079-657e-4db9-80bd-8e1313e7f785. Native Workflow-Session Sol, high, keine Modellwechsel oder zusätzliche Orchestratorebene. TODO.md und REGISTER.md unverändert. Deutsch, echte Umlaute, no-em-dashes.
