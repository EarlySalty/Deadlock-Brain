# Paket A: Abnahmeziele

Diese Ziele kommen aus dem Nutzerauftrag und dem gültigen Schnitt vom 04.10.2026. Der tatsächliche Stand wird in STAND.md mit Laufzeitbelegen geführt.

## Muss vor Abschluss belegt sein

1. Spielwissen Stufe 1: Git-Spielwerte des aktuellen Patches und Patch-Historie sind in den vorhandenen PostgreSQL-Profilen gespeichert. Steckbrief-Dokumente und HTML erscheinen auf der vorhandenen `/brain`-Seite. Drei normale Antworten stimmen mit dem gespeicherten Bestand überein: aktuelle Heldenzahl, tatsächliches aktuelles Item und rückwirkende Patchfrage vom 16.09. mit belegtem Jahr. Ein regulärer Tick verarbeitet den vorhandenen neuen Stand ohne Handkorrekturen in der DB. Reiner Dienstdeploy oder Health 200 reicht nicht.
2. Spielwissen Stufe 2: vorhandene Wiki-Fakten, Quittungen, A4 und erweiterte Intervalle werden nach Stufe 1 weitergeführt. Bestehende Herkunfts-, Sichtbarkeits-, Publikations- und Modellgrenzen bleiben wirksam. Öffentlich erlaubte abgeleitete Spielwerte dürfen ausgegeben werden; rohe Spieldateien, interne Pfade und wörtliche Wiki-Texte bleiben intern.
3. Antwortwege: Discord-Brain, Serverguide-MVP und Twitch sind gegen die tatsächliche aktuelle Laufzeit geprüft. Linkantworten werden weitergesendet. Rechte stammen aus dem Kontext der fragenden Person. Ohne solche Identität bleibt nur die vorhandene engere lesende Sicht. Proaktive Discord-Antworten bleiben im freigegebenen Kanal. Game Invites bleiben vollständig bei Paket B.
4. Betrieb: CLI, Serve und Maintenance stammen aus geprüftem aktuellem Remote-main. Tageslauf-Fehler werden an der Ursache behoben. Legacy-Timer werden erst stillgelegt, wenn der reguläre Rust-Weg den gleichen Datenbestand nachweislich fortschreibt. Fehlende Nutzung gilt bis zum Verdrahtungsbeleg als Anschlussproblem.
5. Reasoner: erst nach belegtem Stufe-1-Datenmodell. Vorhandenen Befund zu Warden 779996 umsetzen: Spirit-Konversionsgraph, bedingte Effekte am Schadensprofil, Lebensverlust am tatsächlichen Pool und Zusammenspiel. Keine Zwecklabels, Referenz-Itemnamen oder Warden-Sondergewichte. Gegen echte Referenz und Population messen, vorhandene Suites erhalten. Abschluss immer mit `--publish` und tatsächlicher `hero_build_id`.

## Aus dem Auftrag ausgeschlossen

Replays/Demos, Steam-Depot, Forum, Begrüßungsserien, Grafiken und voller Serverguide über das MVP hinaus. Keine Produktentscheidung durch stillen Modellwechsel, neue Kosten, Freigabe privater Daten oder Abschaltung von Nutzerfunktionen ersetzen. Solche Entscheidungen kommen als konkrete Frage in AN_HAUPT-A.md; unabhängige Arbeit geht weiter.

## Integration

Brain-Schreib- und Lesepfade gemeinsam integrieren. Ein eindeutiger Eigentümer je Datei und ein eigener Worktree je gleichzeitig schreibendem Worker. Gewöhnliche Worker delegieren nicht. Gate ist einziger Reviewer; bei BLOCK ein frischer Fixer. Je Änderung Gate-Selbstprüfung, regulärer Merge/Push, bestehender Release-Helfer, Neustart, vierteiliger Live- und Funktionsbeweis, sichere Branch-/Worktreebereinigung. Tests sind kein pauschaler Neubauteil; bestehende betroffene Suites bleiben funktionsfähig.
