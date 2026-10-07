# Paket G: Brain v2, strukturierte Entitäten, Rechenschicht, Werkzeuge

Rolle: Teil-Orchestrator (GPT 6.1 Sol, UltraCode). Du darfst native Claude-Code-Subagenten mit getrennten Schreibbereichen einsetzen, aber keine weiteren T3-Threads.
Auftraggeber und Haupt-Orchestrator: Claude-Session `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Regeln: `AUFTRAG.md`. Steuerung: `VON_HAUPT.md` (neueste oben, Abschnitte ab 07.10. 04:55 sind Leitlinien). Überblick über den Ist-Stand: `ROLLUP.md`.

## Nutzerentscheidung (07.10.2026, ca. 05:20)

Spielwissen wird nicht mehr als Dokumente mit Textsuche beantwortet. Neu:

1. **Daten:** lokaler 1:1-Spiegel der Deadlock-API je `client_version` plus Patches mit Datum (baut Paket E, Thread `082da08a`; du nutzt dessen Tabellen und baust keinen eigenen Import).
2. **Rechenschicht in Rust, deterministisch:** alles, was das Community-Sheet rechnet (https://docs.google.com/spreadsheets/d/1fj9XMQmVUY0FY4cbozvB18PMBnpbdsaMFTZRLa74VRY), also unter anderem Base DPS, DPM, Falloff, Skalierung je Boon, Werte auf Max-Level, HP und Regen, Spirit-Skalierung, Schadensrechner mit Items, Spirit, Shred, Resist und Amp, Damage Comparison über Boons, TTK, Hidden Mechanics, Boons/AP, Shop-Boni. Dazu für jeden Wert der Rang bzw. das Perzentil gegenüber allen aktiven Helden („gut oder schlecht im Vergleich“). Faktisch ableitbare Rollenwertungen aus diesen Zahlen sind erwünscht; reine Meinungswerte des Sheet-Autors nicht als Fakt übernehmen. Je Patchversion neu berechnet.
3. **Steckbrief = Ansicht:** je Held, Item, Fähigkeit: Rohwerte, berechnete Werte, Ränge, Buff/Nerf-Geschichte aus den Patches. Immer aktuell, kein Veröffentlichungs-Paket.
4. **Antworten mit Werkzeugen:** `brain-serve` `/v1/answer` bleibt der einzige Antwortweg für Discord und Twitch. Das Modell (`gpt-6-luna` über den bestehenden Provider, vom Nutzer bestätigt) bekommt Werkzeuge statt Textschnipsel: Entität finden, Steckbrief holen, Helden vergleichen, Schaden rechnen, Patchgeschichte, Build bauen (über den bestehenden Reasoner, Paket F entfernt gerade die Matchgrenze), Serverwissen. Lexikalische Suche nur noch für freien Text (Server-Guide, Betreiberwissen, Doku).

## Auftrag

1. **Sheet verstehen (zuerst):** Das ganze Sheet abrufen, alle Tabs (Heroes, damage calculator, Hero meta ranking, Damage Comparison, Hidden Mechanics, Boons/AP, shopBonuses, ttk, haze, scratchpad, raw_hero_data, raw_items_and_abilities, hero query und alle weiteren), mit Formeln, nicht nur Werten (etwa xlsx-Export des öffentlichen Sheets), und visuell nachvollziehen, was jede Tabelle zeigt. Ergebnis `G/SHEET-MODELL.md`: je Tab Zweck, Eingaben, Formeln, welche Rohdaten aus der API sie brauchen, was davon faktisch ist. Das Sheet ist öffentlich; keine Nutzerdaten im Spiel.
2. **Bestand vor Neubau** (Graphify, Skill `code-suche`, Pflicht): `dbrain-reasoner` hat bereits Mechanik und Kampfsimulation (`mechanics.rs`, `combat.rs` `simulate`, `weapon_spirit_scaling`), `dbrain-retrieval` hat Entitätskontext, `brain-serve` hat Netzrunden und Budgets. Die Rechenschicht wird aus diesen Bausteinen gezogen bzw. gemeinsam genutzt, nicht daneben neu geschrieben. Ergebnis `G/PLAN.md` mit Schnittstellen, Tabellen (aus E), Werkzeugliste mit Ein-/Ausgabe, Abbauliste und Reihenfolge; Überschneidungen mit A, E, F ausdrücklich nennen.
3. **Bauen, parallel wo Schreibbereiche getrennt sind:** Rechenschicht samt Rängen, abgeglichen gegen die Sheet-Werte für mehrere Helden des aktuellen Patches (Abweichungen erklären, nicht wegdefinieren); Steckbrief-Ansicht; Werkzeuge im Antwortdienst über den bestehenden Provider, mit den bestehenden Budgets und Rechten (`bot.public`).
4. **Abbau:** Was das neue Modell ersetzt (Spielwissen-Dokumentpakete, Steckbrief-Veröffentlichungsweg für Spielwissen, Sheet-Sync, doppelte Parser, Matchablagen) erst nach belegtem Gleichstand entfernen, als eigene, geprüfte Commits. Ziel ist ein sauberer, kleinerer Stand.
5. **Abnahme:** echte Fragen über `/v1/answer` wie „Ist Haze gerade gut?“, „Wer hat mehr Waffen-DPS, Warden oder Wraith?“, „Was hat sich bei Mystic Burst im letzten Patch geändert?“, „Wie viel Schaden macht Warden mit 20 Boons?“ und eine Build-Frage. Antworten mit Zahlen und Begründung, gegen Sheet und API geprüft.

## Grenzen

- Branches und Worktrees nur unter `~/.worktrees/brain-g-*`, Feature-Branches `feat/brain-v2-*`. Je Teil Tests, fmt, Clippy, `gate_hook.py --review`; bei BLOCK frischer Subagent als Fixer, nie der Implementierer.
- Release-Hold (`A/RELEASEFENSTER.md`): Main-Push erst nach Aufhebung; Build, Install und Tick nur über den dort genannten Eigentümer bzw. danach über den regulären `brain-release`-Weg.
- A schließt die laufende Freischaltung der bisherigen Steckbriefe als Übergang ab; nicht stören. E besitzt den API-Import, F die Publish-Regel des Reasoners. Schnittstellenbedarf vorher in `AN_HAUPT-G.md` melden.
- Rust only, Postgres only, keine Code-Kommentare, keine ENV-Konfiguration, Secrets nie im Klartext, keine Mitgliederdaten an externe Dienste, kein neues Modell und keine festen Timeouts außerhalb der Konfiguration. Keine hand-getaggten Labels, Sondergewichte je Held oder Referenz-Itemnamen im Code.
- Deutsch in Nutzertexten mit echten Umlauten, ohne Em-Dashes.

## Bericht

`G/REGISTER.md` für deine Subagenten, `AN_HAUPT-G.md` (neueste oben) mit Fortschritt, Blockern und am Ende: Commits, Testzahlen, Gate-Antworten, abgebaute Teile, Abnahmeantworten. Fertig heißt gemergt, deployt, live geprüft, aufgeräumt; dann `python3 ~/Documents/tools/t3-thread.py settle --selbst`.
