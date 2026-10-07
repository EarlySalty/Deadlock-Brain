# Delegator Brain (D1)

Zuerst lesen: `../DELEGATOR-REGELN.md` (gilt vollständig), dann `../PLAN-NEU.md`.

## Bereich

Deadlock Brain Welle 1 live: Brain-Integration, Patchnotes in Rust, Steam-Publish mit echter `hero_build_id`, Consumer (Docs, Second-Brain, Twitch-Antwort in earlysalty). Welle 2 (Replay, Q-Retention, Wiki, Forum, Serverguide) baust du nicht.

Dein Bereichsordner ist dieser Ordner (`welle1/`). `AN_HAUPT.md`, `VON_HAUPT.md` und `REGISTER.md` liegen hier.

## Pakete und fertige Worker-Briefings

| Paket | Briefing | Repo-Projekt in T3 | Abhängigkeit |
|---|---|---|---|
| W1 Brain-Integration | `w1/BRIEFING.md` | Deadlock-Brain | sofort |
| W2 Patchnotes | `w2/BRIEFING.md` | Deadlock--Patchnotes-Bot | sofort |
| W3 Steam-Publish | `w3/BRIEFING.md` | Deadlock-Steam-Bot | Steam-Teil sofort, echter Publish nach W1-Deploy |
| W4 Consumer | `w4/BRIEFING.md` | Deadlock-Brain (Consumer-Repos über Worktrees) | eigene Repos sofort, Live-Test nach W1-Deploy |

Die Briefings sind Entwürfe. Passe sie an, wenn die STAND.md-Dateien etwas anderes zeigen. In den Worker-Briefings den Auftraggeber auf dich (deine Thread-ID) ändern und `AN_HAUPT.md` der Worker durch deinen Kanal ersetzen (z. B. `w1/AN_D1.md`). W1 ist der einzige, der ins Brain-Repo nach main merged und Brain deployt; Brain-Änderungen der anderen gehen über `w1/EINGANG.md` an W1.

Starte höchstens drei gleichzeitig. Empfehlung: W1, W2, W3 sofort, W4, sobald einer davon fertig ist oder W1 deployt hat.

## Vor dem ersten Start

Die alten Threads P, Q, R, S2, K und Z wurden angehalten und schreiben ihren Stand nach `../bereiche/<x>/STAND.md`. Fass ihre Worktrees erst an, wenn der alte Thread nicht mehr `running` ist (`t3-thread.py read --thread <id>`): P `09ce79b1-d5ed-4ae8-b77a-1fa54fdfcb1a`, S2 `c6ddac1c-c0d2-4bbc-b6f8-7bc462570a52`, Q `80314ca4-8ef0-4006-8c4e-62b3bc5566a9`, R `e9e3df50-7001-4f55-8a64-350b0c2cbc12`, K `27b6a744-a92e-4765-88c6-2c70a2b0fcd8`, Z `b73d9271-6c39-4c58-b831-3399fc6dceb6`. Schreib diesen Threads nichts.

## Fertig

Alle vier Pakete gemergt, deployt, live belegt (Brain-Release-SHA für CLI und serve, Patchnotes-Unit aktiv, `hero_build_id`, echte Antworten aus Docs, Second-Brain und earlysalty). Danach einen fehlerfreien Tageslauf beobachten und erst dann der Hauptsession vorschlagen, welche Legacy-Dienste abgeschaltet werden.
