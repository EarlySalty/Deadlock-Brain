# Entscheidung: Weiterbau nach behobener Prüfsperre

status: aktiv, 07.10.2026, ca. 20:20 CEST. Im Auftrag des Nutzers („alles fertig bekommen, Astra und die Agents sollen weiterbauen“), gegeben von der Claude-Session `3fcd8f71-443e-48ae-825c-527eb52fbe56`, da der bisherige Haupt-Orchestrator `d3a1741e` gestoppt ist. Gilt für den Delegator `481426fe` und die Pakete I, G, K.

## 1. Sperren sind behoben

Siehe `BLOCKER-PRUEFWEG.md`, Abschnitt BEHOBEN (claude-config `fd78852`):
- Cargo-Prüfungen nur noch über `cargo-slot <cargo-argumente>` (Slot und bei Release die Release-Sperre automatisch). Keine `exec`/`flock`-Schleife mehr.
- `~/.worktrees`, `~/repos`, `/tmp` sind als Arbeitsverzeichnisse freigegeben.

Falls eine laufende Session die neuen Rechte noch nicht sieht: frische native Fixer bzw. Neustart der Session, kein Umgehungsweg.

## 2. Schnitt bei I: Spiegel zuerst, Patch-Erkennung später

Ziel ist der Helden-/Item-/Fähigkeitenspiegel je `client_version` mit dem Leser `asset_mirror`, Builds lesen daraus. Die neue Patch-Erkennung über `/v2/patches` (Feed-Discovery, Originalauflösung neuer URLs, URL-Varianten) ist für G, F und K nicht nötig und hat 19 Gate-Runden gekostet.

- **Nachtrag 20:30 auf Nutzerwunsch: Patch-Erkennung möglichst mitnehmen.** Zuerst genau eine frische Fixer-Runde für den einzigen offenen Fund (URL-Varianten wie `?l=english` bekommen eine neue Patch-ID statt der bestehenden), jetzt mit freigegebenem Worktree. Bei ALLOW geht alles zusammen nach main.
- Kommt dabei ein neuer Fund, wird die Patch-Erkennung herausgelöst (eigener Branch `feat/brain-patch-discovery`) und der Spiegelteil geht allein durch Gate, Merge, Deploy und ersten Import (Beweis `mirror_complete=true`, `load_mirrored_assets` liefert Helden und Items). Der bestehende Patch-Import bleibt dann bis zum Nachziehen wie auf main.

## 3. Danach F, dann G, dann K

1. F auf den neuen main, Gate, Merge, Deploy, Warden-Build mit `--publish`, `hero_build_id` melden.
2. G: die zwei Restkerne im Rechenkern (Stack-Schaden ohne Shred, doppelte Item-IDs) mit `cargo-slot` testen und fertig fixen, S3/S4, dann Merge und Deploy durch G selbst.
3. K: Anschluss an G und Antwortverdrahtung für öffentliche Spiel- und Serverfragen, Merge, Deploy, Live-Beweis in Discord und Twitch mit Testkonten.
4. Abnahme mit echten öffentlichen Fragen (Q-Material), dann A-Fragen erneut.

## 4. Datenschutz bis zur Nutzerentscheidung

**Überholt durch `ENTSCHEIDUNG-DATENSCHUTZ-NUTZER-2045.md` (Testfreigabe über Luna, rollenbasierte Sicht). Der folgende Absatz gilt nicht mehr.**

Unverändert: private und Community-Inhalte nicht an den externen Proxy. Öffentliche Spiel- und öffentliche Serverfragen laufen über den bestehenden Provider und werden jetzt live gebracht. Private Antworten bleiben gesperrt, bis der Nutzer entscheidet. Das blockiert den Rest nicht.

## 5. Führung

Der Delegator führt weiter, startet für tote oder gestoppte Pakete frische Worker aus der Pyramide und meldet nur Abschluss, echte Nutzerentscheidungen oder Blocker nach fünf erfolglosen Runden. Statusdateien wie bisher.

## 6. Nachtrag 21:00: Modellvergleich über dieselbe feste Schicht

Nutzerziel: Nicht das Modell entscheidet, was es sehen und rechnen darf, sondern die feste Schicht (Rollensicht der Brücke, Werkzeuge, Rust-Rechnung, Belegprüfung). Dadurch lassen sich Modelle fair vergleichen.

- Q baut die Abnahme so, dass derselbe Fragensatz (mindestens 30 echte Fragen, feste erwartete Antwortart) wiederholbar gegen eine wählbare Providerkonfiguration läuft. Je Lauf festhalten: Modell, aufgerufene Werkzeuge und Argumente, ob die Zahlen mit der Rust-Rechnung übereinstimmen, Belegtreue, Antwortzeit, Tokenverbrauch.
- Erster Lauf mit `gpt-6-luna` (Codex-Abo, ohne Zusatzkosten). Läufe mit kostenpflichtigen Anbietern (z. B. OpenAI-API) erst nach Nutzerfreigabe der Kosten; Umschalten nur über die Konfiguration, kein eigener Connector.
- Reihenfolge: erst nach Live-Gang von I, G und K, damit der Vergleich auf dem echten Werkzeugweg läuft.
