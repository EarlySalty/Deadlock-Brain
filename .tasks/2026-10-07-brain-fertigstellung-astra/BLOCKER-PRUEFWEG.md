# Technischer Blocker: vorgeschriebene Prüfwege werden abgewiesen

## Neuer I-Abschlussblocker, 07.10.2026, 20:35 UTC

Eigener Worktreezugriff funktioniert. Gesicherter Spiegel b7289d11 hat laut aktueller Fachrückgabe 605 bestandene Tests/24 ignorierte Fälle und expliziten Spiegelgate ALLOW. Beide regulären Main-Pushes wurden jedoch vor Ausführung verweigert, weil der Test-Gate keinen grünen Lauf findet. Nach dem ersten Deny direkt sichtbar und ohne Umleitung über cargo-slot nochmals sechs echte Receipt-/Core6-Scratchfälle bestanden, Exit 0. Zweiter Push derselbe Deny. Kein Main-Kritikerurteil aus diesen verweigerten Aufrufen, kein Main-Push oder Deploy.

Ob Befehlsfilter oder Transcriptzuordnung die Ursache ist, ist nicht verifiziert. Hookdiagnosezugriff über ctx_execute_file außerhalb des I-Roots verweigert, keine Umgehung. Präziser Nachweis: /home/nathanael/.worktrees/brain-e-deadlock-api/.tasks/2026-10-07-i-integration/BLOCKER-SPIEGEL-TESTGATE.md. Zuständiger Harness-/Gatebereich muss echte Pflichtprüfungen dieser Session regulär erkennen. Keine Markerfälschung, fremden Workerlogs, Hookabschaltung oder dritte identische Probe. Bestehenden Spiegelstand und Discovery erhalten; G/K laufen unabhängig weiter.

## Aktualisierung 19:05 UTC: eigener I-/G-Zugriff bestätigt

Beide frischen Fortsetzungen melden tatsächlichen Produktzugriff im eigenen Worktree. I hat pg_patchnotes.rs über seinen regulären MCP-Root gelesen und Fixer 15 gestartet. G konnte sein früher abgewiesenes Originallog lesen; Delegator las dasselbe Log über normales Read: E0609, combat.rs:3478, DamageModifiers besitzt kein resistances-Feld. Compilerabbruch Exit 101 vor Testausführung. Der bisherige eigene Worktree-/Logrootblocker ist damit behoben, kein neuer Test- oder Gateerfolg daraus abgeleitet.

Separate Zugriffe außerhalb des jeweils eigenen MCP-Roots bleiben abgewiesen (I: G-Hauptbericht; G: globale ABLAUF.md). Keine Umgehung. I-Vertragsfrage geordnet beantwortet: kein gesicherter S3-Commit bestätigt, daher kein erfundener Rechenvertrag oder Kopieren des abgewiesenen Berichts. Eigene E-/G-Fixarbeit läuft weiter.

## Aktueller Restblocker nach Freigabe: MCP-Projektroot, 07.10.2026, 18:14 UTC

Die Host-/Arbeitsrootfreigabe und cargo-slot sind vorhanden. I hat dennoch im genau einen frischen Fixer einen konkreten neuen Ablehnungsnachweis: `ctx_execute_file` verwendet weiterhin /home/nathanael/repos/Deadlock-Brain als MCP-Projektroot und verweigert pg_patchnotes.rs aus dem korrekt zugewiesenen E-Worktree mit `resolves outside the project root`. Hauptsession-pwd und Worktreezuordnung stimmen laut Fachrückgabe. Keine Produktänderung, Tests oder neue Gateprüfung; Bericht und Nachweise auf 9d17ee52 gesichert. Beleg: /home/nathanael/.worktrees/brain-e-deadlock-api/.tasks/2026-10-07-i-integration/REVIEW-RUNDE-20-MCP-ROOT.md.

K meldet denselben Toolroot-Befund im frischen Fixtureworker; kein dortiger Fix/Test. Gs Fixworkflow ist inzwischen beendet: cargo-slot wurde tatsächlich ausgeführt und endete mit Exit 101. Die Logauswertung wurde ebenfalls wegen kanonischem MCP-Projektroot verweigert; Ursache und Testzahlen bleiben ungeprüft. Drei Regressionen und zwei Testhelfer sind als uncommittierter Test-WIP erhalten, kein Produktfix oder Gate. Beleg: G/G-M-S2-CARGO-SLOT-SPERRE.md im G-Worktree. Der neue Aufruf beweist eine funktionierende Slot-Ausführung, aber keinen erfolgreichen Testlauf. K-Tagesquotenarbeit ist separat aktiv; native Prüfworker-Beauftragung am 07.10.2026 um 18:35 UTC belegt.

Erforderlich ist die reguläre MCP-Projektrootbindung an den bereits freigegebenen zugewiesenen Worktree. Kein erneuter identischer Fixer, kein versteckter Wrapper oder Toolwechsel zur Umgehung. Die eine erlaubte I-Fixrunde hat fachlich noch nicht stattgefunden; die Schutzablehnung ist kein neuer Produktfund und löst den Discovery-Schnitt nicht aus. K-Tagesquoten-Kleinschritt bleibt im getrennten eigenen Schreibbereich priorisiert.


## BEHOBEN 07.10.2026 ca. 20:15 CEST (Nutzerauftrag, claude-config fd78852)

1. **Slot-Testweg:** Neuer einfacher Befehl `cargo-slot <cargo-argumente>` (`~/.local/bin/cargo-slot`, Quelle `claude-config/bin/cargo-slot`) nimmt einen der drei Slots, bei optimierten Profilen zusätzlich die Release-Sperre, und gibt beides frei. In den Rechten erlaubt. Die alte `exec 8>`-Schleife nicht mehr benutzen. Regel in `orchestrierung/ABLAUF.md` und `HOSTPROBE.md` angepasst.
2. **Projektroot:** `~/.worktrees`, `~/repos` und `/tmp` sind als zusätzliche Arbeitsverzeichnisse freigegeben. Frische Fixer dürfen ihren zugewiesenen Worktree lesen und schreiben.

Sessions, die vor dieser Änderung gestartet wurden, sehen die neuen Rechte eventuell erst nach Neustart. Frischer Fixer für I-Runde 19 und die G/K-Testläufe können jetzt regulär weiterlaufen.

Stand: 07.10.2026, 17:31 UTC. I, G und K, bestehender Fertigstellungsauftrag.

## Zusätzliche Zugriffssperre bei I

I-Fixer 13 wurde beim Zugriff auf seinen zugewiesenen E-Worktree als außerhalb des zugelassenen Projektroots abgewiesen. Kein anderer Zugriffsweg, Produktfix oder Teststart. Bestehender E-Stand d3d3c5b6 und Kandidat 501d3725 bleiben erhalten. Beleg: /home/nathanael/.worktrees/brain-e-deadlock-api/.tasks/2026-10-07-i-integration/REVIEW-RUNDE-19-SCHUTZBLOCKER.md.

Der gemeinsame Kandidat besteht 558 Tests mit 25 ignorierten Fällen sowie eine zusätzliche echte Scratch-PG-Probe, bleibt aber wegen einer Bestands-ID-Lücke bei URL-Varianten im Gesamtgate BLOCK. Zugriffsblocker und Produktbefund getrennt behandeln. Der reguläre frische Fixerkontext benötigt den bestehenden E-Worktree als zugelassenen Projektroot; keine Ausführung durch anderen Worker oder Werkzeugwechsel zur Umgehung. Damit ist auch I derzeit angehalten, nicht mehr in aktiver Fixarbeit.


## Aktualisierung 16:38 UTC

K konnte direkte absolute Compiler-/Format-/Clippyaufrufe im eigenen CWD mit gehaltenem Slot tatsächlich ausführen. Source-JSON-Naht 3b4b21ea regulär ALLOW und auf origin, kein Source-WIP mehr. Der frühere allgemeine Compilerblocker ist damit überholt. Der anschließend direkte Verhaltenstest wurde weiterhin vor Prozessstart verweigert; keine neuen Testzahlen. Gs veröffentlichter Slotloop bleibt nach letzter Rückgabe blockiert. Diese Präzisierung ist keine Freigabe zur Wiederholung der verweigerten Tests. Beleg: K/PRUEFWEG-FORTSETZUNG.md im K-Worktree.

## Ursprünglicher Befund

Der vorgeschriebene Buildslot-Weg und die Worktree-Befehlsprüfung sind auf den konkret beauftragten Prüfpfaden nicht miteinander vereinbar. Das ist eine tatsächliche Ausführungssperre, kein Compilerfehler, kein fachliches ALLOW und kein belegter Kontingentausfall. Weitere gleichartige Fixer oder andere Werkzeugwege würden das Problem nicht lösen und sind nicht beauftragt.

## Belege

- G: veröffentlichter FD/flock-Slotloop vor Ausführung abgewiesen, `this command runs exec inside a construct too complex to verify`. Kein Teststart, keine Quelländerung. HEAD 8feb8b6e und zwölf Quellfingerprints unverändert. Vollständiger Befund: /home/nathanael/.worktrees/brain-g-v2-20261007/.tasks/2026-10-06-brain-abschluss/G/G-M-S2-PRUEFSPERRE-NACHWEISE.md. Dort Originaltranskript und exakter zurückgewiesener Befehl. Fehlende Beweise: Stack/Shred und normalisierte Itemeffekte. Der später zulässige Test benötigt zusätzlich die dort dokumentierten seriellen Testoptionen.
- K: isolierter flock-/bwrap-Prüfweg für die bytegleich übernommene gemeinsame Source-JSON-Delegation vor Prozessstart abgewiesen. Kein Compiler-/Test-/Lintbeweis dieser uncommittierten Datei. Zusätzliche Discord-Hinweisprüfungen ebenfalls ohne tatsächlichen Start. Beleg: /home/nathanael/.worktrees/brain-k-ki-20261007/.tasks/2026-10-07-brain-grafik-ki/K/HANDOFF.md. Vorheriger Enum-/Fixturestand c64de6a2 hat dagegen ein echtes gemeinsames ALLOW und ist auf origin, kein Gesamtabschluss.

## Empfehlung und Grenze

Den bereits vorgeschriebenen Prüfweg im zuständigen Harness-/Regelbereich transparent mit der Worktree-Prüfung vereinbar machen. Slotbindung und Git-Isolation müssen erhalten bleiben. Kein Abschalten von Hooks, kein Cargo ohne Slot, kein versteckter Wrapper, keine Ausführung durch den Delegator oder einen anderen Worker als Umgehung. Änderung am Harness-/Regelbestand ist nicht Teil der übernommenen I/G/K-Produktdateien und wird hier nicht eigenmächtig umgesetzt.

G/K-Artefakte bleiben erhalten, kein Ersatzthread und kein ungesicherter Release. I durfte zuvor die getrennt bestätigten Originalquellenfehler im eigenen zugelassenen Weg weiter bearbeiten. Seit der oben dokumentierten Projektroot-Ablehnung ist auch diese Fortsetzung angehalten; I darf die verweigerten G/K-Prüfungen nicht übernehmen. Der gestoppte Haupt-Orchestrator wird nicht wiederaufgenommen. Private Providerentscheidung bleibt ein davon unabhängiger bekannter Blocker.

## Fortsetzungsstart umgesetzt, 07.10.2026, 18:57 UTC

I neu b17d5729-a475-4bcb-8fc9-0aa6103d4555 direkt in /home/nathanael/.worktrees/brain-e-deadlock-api, G neu ee3de2ba-30ab-4558-a57c-6c1de154891e direkt in /home/nathanael/.worktrees/brain-g-v2-20261007. Beide Starts mit t3-harness new --worktree angenommen, anschließend running bestätigt. Effort wie bisher I high, G ultracode; Modell sol. Alte I-/G-Threads stopped, nicht wieder aufnehmen. Bestehende Commits, WIP und Akten erhalten. Neue Root-/Prüfwirkung noch nicht bestätigt; Startannahme ist kein Reparaturbeweis.

Neueste Nutzerentscheidung ENTSCHEIDUNG-PARALLEL-FERTIGSTELLEN.md an I/G/K zugestellt. Getrennte Schreibbereiche parallel, Merges nach Fertigstellung auf jeweils aktuellem main. F beginnt direkt nach Spiegel-Merge, K liefert Tageslimit/Ortskontext sofort regulär unabhängig von I/G-Merges. G wertet den roten cargo-slot-Lauf mit normalem Read im neuen Kontext aus. Keine Schutzumgehung oder Änderung an Hooks.

## Nachtrag 20:50 CEST: verbleibende Root-Sperre kommt vom Sessionstart

I-Fixer und G melden weiter „kanonischer Pluginroot“ (context-mode `ctx_execute_file`) bzw. verweigerte Logauswertung. Ursache: Die laufenden Sessions von I (8827da25) und G (a867ef50) wurden im kanonischen Checkout gestartet; ihr Projektroot und die vor 20:15 geladenen Rechte gelten weiter, auch für ihre nativen Fixer.

Weg (kein Umgehen): I und G als frische Fortsetzungs-Threads direkt im eigenen Worktree starten, `t3-harness new --project Deadlock-Brain --model sol --effort <wie bisher> --worktree <eigener Worktree> --branch <eigener Branch> --file <Fortsetzungsbriefing>`. Vorhandene Commits, Nachweise und Akten übernehmen, alte Threads als „nicht wieder aufnehmen“ registrieren. Logs mit dem normalen Read-Werkzeug lesen. `cargo-slot` Exit 101 ist ein normaler roter Testlauf, kein Sperrfehler.

## BEHOBEN 23:35 CEST: Test-Gate erkannte cargo-slot nicht

Ursache des I-Spiegel-Push-Deny („grüner Testlauf nicht gefunden“): Das Test-Gate (`gate_hook.py`, `TEST_CMD_RE`) kannte nur `cargo test|clippy|nextest`, nicht `cargo-slot test`. Fix in gpt-workers `a593c5d` (main), sofort wirksam. I: einmal `cargo-slot test …` für den Spiegel grün laufen lassen, dann denselben geprüften Spiegel `b7289d11` regulär nach main pushen.
