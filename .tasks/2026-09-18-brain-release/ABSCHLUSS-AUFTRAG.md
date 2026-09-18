# Release-Abschluss nach Übergabe

Nutzerauftrag: „Schließe alles ab, dass ich das deployen kann“, danach „weiter“. Basis 9ead461, einziger beschreibbarer Worktree /home/nathanael/.worktrees/brain-deploy-completion-20260918, Branch codex/brain-deploy-completion-20260918. Der Hauptcheckout und andere Worktrees bleiben unverändert. Orchestrator: aktuelle ChatGPT/Codex-MCP-Sitzung (keine T3-Intent-ID vorhanden). Vorhandener Worker-Thread: 57ed8534-ea8b-41bb-8822-f9a310739238. Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder Unter-Agenten spawnen.

## Jetzt abschließen

1. Vier noch ignorierte Datenbank-Vertragstests von deadlock-brain-yt gegen ein tatsächlich isoliertes Scratch-Schema durchführen und nötige Fixture-/Codefehler beheben. Die historische 11941-Claim-Parität nur mit exakt passendem Bestand testen, andernfalls ausdrücklich getrennt ignoriert lassen und zusätzlichen deterministischen Query-/Filtervertrag mit eigenen Fixturezeilen abdecken. Keine Assertions abschwächen. Neuer spezifischer Fixturepfad darf niemals Produktivdaten überschreiben; Test-DSN muss auf eigene Testdatenbank zeigen.
2. Consumer-Revalidierung im Brain und vorhandenen MCP-Transport prüfen. Insbesondere caption-/claim-Revalidierungsmarkierungen dürfen bei aktuellen Antworten nicht als verifiziertes Wissen herausgegeben werden. Relevante Verbraucher im Retrieval/Claims-Pfad gezielt korrigieren und mit Regressionstests belegen. Historische Quellenfakten und ausdrücklich als solche markierte alte Fassungen bleiben lesbar. Kein pauschales Löschen und keine Datenaufbewahrungsfrist.
3. CI so ergänzen, dass die neuen Tests tatsächlich laufen, Format nur eigene Dateien prüfen, keine repoweite Formatierung. Bestehende Tests und Clippy ausführen; im Report genaue Zahlen und Grenzen melden.
4. Sicherheitsreview des vorhandenen versionierten Sync-Betriebswegs: Fehler sichtbar, keine Verarbeitung veralteter Events als frischer Erfolg, kein unkontrollierter Modelllauf, keine neuen Provider, kein Modellwechsel. Bestehende AiClient-Konfiguration bleibt maßgeblich.

## Grenzen

Keine Secrets oder ENV-Dateien öffnen/ausgeben. Bestehender lokaler Postgreszugang funktioniert mit psql -X -w -d deadlock über Unix-Socket. Produktion nur lesend; eigene Scratch-Datenbank unter eindeutigem Namen brain_completion_* erlaubt. Keine Produktionsmigration, kein Merge, kein Dienstneustart durch Worker. Orchestrator führt Merge-Gate, produktive Migration, Installation, Modelllauf und Live-Smokes in derselben Sitzung aus. Keine öffentliche Veröffentlichung von Analysen oder Videos. Keine Creator-Daten in den ersten unabhängigen Modelllauf.

## Abschluss

Nur eigene Dateien committen, sofort Feature-Branch pushen. Bericht in .tasks/2026-09-18-brain-release/ABSCHLUSS-TESTS.md mit Commit, Testbefehlen, Ergebnissen, tatsächlichen Grenzen. Keine Behauptung fachlich autonomen Spielverständnisses. Bei konkretem Blocker genau Ursache und vorbereiteten Lösungspfad berichten, keine weiteren Threads starten.
