# G-S2: begrenzte Fortsetzung nach fünf Fixrunden

Stand: 07.10.2026, 15:38 UTC. Derselbe offene G-Auftrag, keine neue Architektur oder Produktfreigabe.

## Entscheidung und Ziel

Qualifizierte Fachrückgabe aus AN_HAUPT-G.md und G/G-M-S2-BLOCK-NACHWEISE.md angenommen. Die zwei dort nach Graphify am tatsächlichen Code bestätigten Restkerne gehören zum bestehenden S2-Vertrag. Empfehlung zur eng begrenzten Fortsetzung angenommen:

1. Stackbonus durch dieselbe bestehende Schadensstufe einschließlich Rüstungsverringerung führen. Default-/Fast-/Bindingpfade gemeinsam betrachten; im expliziten Szenario keinen zweiten Shred anwenden.
2. Doppelte Item-IDs vor Aufbau der Effektzustände über die vorhandene gemeinsame Normalisierung behandeln. Stat-, Shop- und Effektpopulation müssen denselben Bestand sehen. Keinen zweiten Inventar-, Effekt- oder Rechenpfad bauen.

## Eigentum und erhaltener Stand

G bleibt Bereichsführung und startet genau einen frischen nativen Fixer für beide eng gekoppelten combat.rs-Kerne. Kein neuer T3-Thread und keine parallelen Schreiber. Worktree /home/nathanael/.worktrees/brain-g-v2-20261007, Branch feat/brain-v2-g-20261007, gemeldeter HEAD 8feb8b6ec0bf3dac7a8e180bfacc59ed001d3206. Aktuellen Stand vor Beginn prüfen. Alle fünf vorhandenen Fixcommits und fremden/unstaged S3-/S4-Bestand erhalten. I-/K-Eigentum bleibt unverändert.

## Nachweis und Grenze

Vor Umsetzung im vorhandenen Bestand mit code-suche/Graphify nachsehen. Passende bestehende numerische Fälle tatsächlich ausführen, insbesondere Stack mit/ohne Shred und doppelte Item-IDs für beide öffentlichen Eingänge. Originale öffentliche API-Daten verwenden, soweit bestehende Fixtures sie liefern. Synthetische Regressionen nur als solche kennzeichnen. Kein privater Modellaufruf.

Compiler, Format und Clippy auf dem tatsächlichen Prüfstand. Tests bisher ausdrücklich nicht ausgeführt, daher keine alte Compilerfreigabe als Rechenbeweis verwenden. Nur zugelassene vorhandene Cargo-Slots im eigenen Worktree; keine verschachtelten Ersatzwege oder Wiederholung einer verweigerten Ausführung. Bei echter Prüfsperre den genauen Deny und das konkrete fehlende Beweisziel zurückgeben, keinen Hook ändern.

Anschließend gemeinsamer vollständiger S2-Gate gegen S1 bd83d7ab, unverändert gpt-6.1-sol als bereits urteilsgebendes Modell. Kein Neu-Würfeln, keine Deltafreigabe statt Gruppenprüfung, kein Push oder Merge als Umgehung. Bei erneutem BLOCK frischer Kontext gemäß bestehendem Ablauf; spätestens nach fünf erfolglosen Runden qualifizierte Rückgabe. Nur bei tatsächlichem ALLOW und passenden Nachweisen S2 sichern und erhaltene S3/S4 fortführen.

## Routing

Fachrückgabe in eigener G-Akte, direkte Bereichsführung bleibt im übernommenen Thread a867ef50-88e6-41ac-a852-724f5184c6e6. Auftraggeber 481426fe-b477-42b3-91c6-901811fcba1d führt zentrale TODO/REGISTER. Keine neue Meldung je Runde, keine Wiederaufnahme gestoppter Threads. Gesamtabnahme, Runtime und P0 bis P11 bleiben offen.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
