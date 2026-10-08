# Q2: Vorbereitung und offene Abnahme

Stand: 08.10.2026. **Teilstand gebaut und lokal geprüft. Keine fachliche Gesamtfreigabe, kein Providerlauf.**

## Tatsächlich vorbereitet

Der bestehende Rust-Collector wurde um vier lokale Befehle ergänzt: private Kopien vergleichen, Quellbindungen und Dublettenkandidaten prüfen, geschützte Originalprüfliste vorbereiten, Goldstruktur prüfen. Keine neue Pipeline, keine Provideranbindung und kein Botcode. Die früheren Q-Berichte und Sicherungen bleiben unverändert.

Die bestehenden zwei privaten Kopien stimmen überein: je 17 Dateien, vier Verzeichnisse und acht JSON-/Digestbindungen, Eigentümer und 0600/0700 geprüft. Der unveränderte alte Collector bestätigte zunächst vier gebundene Quellen und 166 Kandidaten, Teilplanhash `c2332949206a0773b604391d7f540c9520598a66409053f94defb4076447b0ed`.

Die neue lokale Prüfung findet 37 mögliche Dubletten unter diesen 166 Einträgen. Es bleiben 129 unterschiedlich normalisierte Kandidaten, **nicht 129 echte oder geprüfte Goldfragen**. Die geschützte Prüfliste enthält noch 0 akzeptierte Goldfälle. `check-review` verweigert erwartungsgemäß mit `review_gold_coverage_incomplete`, Exit 1. Die Listen wurden nicht an ein Modell gesendet.

`FALLVERTRAEGE.json` hält die fünf verbindlichen Nutzerfälle und die gemeinsame dreifache Nachrichtenform fest. Originalnachrichten und Ortsbindung sind noch lokal zu vervollständigen. Drei Teilfragen bleiben eine Nachricht mit einer Reservierung; Modellaufrufe werden gesondert beobachtet. Die Datei wird nicht als fertiges 30er-Set ausgegeben.

Aktuelle öffentliche Itemoriginale wurden mit dem vorhandenen Collector erneut eingefroren, 746 Entitäten. Haze-Kernwerte und Pockets Affliction stimmen mit den vorbereiteten Fakten überein. Für die Itemfrage zeigt die aktuelle Quelle `Armor Piercer`, Klasse `upgrade_aprounds`, Procchance 55; Plated Armor nennt 30 Prozent Kugelabwehr und 50 Prozent On-Hit-Verhinderung. Namensalias und vollständige Wechselwirkungsrechnung gegen den ausgelieferten Rust-Kern sind noch zu prüfen. Kein pauschales Soll zur Wechselwirkung erfunden. Einladungsablauf und Coachingquelle noch nicht als aktuell unabhängig geprüft ausgegeben.

Die fünf bisherigen Originalpatchstichproben wurden als bestehende Erwartungen übernommen. Kein neuer Patch-Livebeweis und keine aktuelle Gesamtwissensbindung behauptet. `METHODIK.md`, `COLLECTOR.md` und `LAUFVERTRAG.json` trennen private Rohbelege, feste Vergleichsschicht, echte Zustellung und gemessene/reservierte/unbekannte Abrechnung. Eine ausführbare Kanalwiederholung ist noch nicht geliefert.

## Fortsetzung nach ausdrücklicher K-Übergabe

Der Delegator hat den Teilcheckpoint ausdrücklich als Zwischenstand bestätigt, nicht als Auftragsende. Der gesicherte Botsconsumer `8e1b8f03` und dessen vorhandener Wirefixturepfad wurden übernommen, ohne fremden K-Worktree zu bauen oder Produktcode zu ändern. Ein eigener detached Kontrollcheckout bindet diesen tatsächlich bestätigten Remote-main-SHA.

Der vorhandene Einzeltest lief nach einem vor Testbeginn abgebrochenen ENOSPC-Versuch erfolgreich: 1 passed, 0 failed, 0 ignored, 332 filtered, Exit 0. Der Testprofilbau dauerte 10m46s, der Fixturelauf 0,03s. Weder Zeit noch synthetische Wirefälle sind P1- oder Goldbelege. Quellen-, Lock- und Protokollhashes sowie Wiederholungsbefehl stehen in `CONSUMER-BINDUNG.md`.

Die echte Callback-/Consumer-/SDK-Kette und die requestgebundene Lesesperre sind damit lokal gebunden. K exportiert darüber keinen tatsächlichen Accounted-/ToolExecution-Messnachweis. Der vom Delegator bei G angefragte sichere Messnachweis und die G/K-Livebedingung bleiben offen. Weiterarbeit an festem 30er-Set und wiederholbarem Lauf bleibt derselbe Auftrag; kein Cleanup, Self-Settle oder Endstatus aus diesem Kontrolllauf.

## Erhaltene lokale Artefakte

Die neue geschützte Prüfliste und aktuelle öffentliche Assetquelle sind außerhalb des Worktrees unter `/home/nathanael/.local/share/brain-q2-private-20261008/` erhalten. Prüflistenkopie: eine Datei; Assetkopie: zwei Dateien mit einer Digestbindung. Beide Seiten wurden auf Bytegleichheit, Eigentümer und Rechte geprüft. Alte private Kopien nicht überschrieben. Technische Hashes stehen in `STATUS.json`; dort stehen keine Originalfragen oder Personenkennungen.

Die neue Inventarmethode bildet SHA256 über eine sortierte JSON-Zuordnung relativer Pfade zu Dateihashes. Ihr Hash ist nicht der frühere Inventarhash mit anderer Darstellung. Keine scheinbare Drift durch Vergleich unterschiedlicher Hashverfahren.

## Neue Laufzeitbeobachtung, keine Kanalabnahme

| Gegenstand | Beobachtung | Grenze |
|---|---|---|
| Brain, 00:47:17 UTC | PID 3178539, exe aus `b7289d115c0b64016fcfe6cbfe3c797cf9fa76e2`, ohne deleted, NRestarts 0, Binaryhash in STATUS | I-Spiegelstand, keine belegte gemeinsame G/K-Livelieferung |
| Health/Ready, 00:48:09 UTC | `/healthz` und `/readyz` auf konfiguriertem Port 8788, HTTP 200, JSON, ok/ready | Keine zugestellte Antwort |
| Wissensbindung | maintenance/docs `5819bf58…`, Bindungshash in STATUS | Nicht mit Code-SHA gleichgesetzt |
| Provider | Bestehender `codex_subscription`, `gpt-6-luna`, Loopbackproxy 18769 | Keine private Testausführung, keine Konfigurationsänderung |
| Twitch, 00:56:43 UTC | Lesender Deployprüfer Exit 0, vier Prozesse ohne deleted auf `b0bd68248c3accc1771e938e6166c3a122ac154e` | Kein Nachweis neuer K-Gesamtfunktion |
| Discord | Userunit aktiv, Wrapper-PID 2766542, NRestarts 0 | Vollständiger Consumer-exe-/SHA-Nachweis fehlt |

Die ersten Proben verwendeten unbestätigte Standardpfade/Ports und lieferten 404. Nach Lesen der tatsächlich laufenden Konfiguration und der implementierten Routen wurden die richtigen Ziele geprüft. Die 404 werden nicht als Produktionsausfall ausgegeben. Keine fremden Dienste angehalten oder neu gestartet.

## Präzise offene Teile

1. **Goldset:** Lokale fachliche Originalprüfung, Bereinigung und feste Sollfakten für mindestens 30 echte Fälle stehen aus. Der Codiermodellkontext darf private Originale nicht lesen; automatische Etiketten wären kein Ersatz. Prüfliste liegt geschützt bereit. Die aktuelle Testfreigabe wird nicht wieder als Datenschutzsperre ausgegeben.
2. **Livevoraussetzung:** Belegte G/K-Livelieferung, erneute vollständige I-Importbindung und Consumerbeweise fehlen für den Vergleich. Beobachtetes Brainbinary stammt noch aus dem Spiegelstand. Kein Warten auf den zurückgezogenen Brückenfix und kein Polling fremder Sessions.
3. **Wiederholung und Messweg:** Ein konkreter freigegebener Kanalrunner sowie vollständige lokale Toolargument-/Ergebnis-/Verbrauchsbelege aus der ausgelieferten festen Schicht sind noch nicht belegt. Aktuelles `brain-contracts/src/lib.rs:409` stellt in `AnswerResponse` Text, Belege und flache Usage bereit, aber keinen vollständigen Werkzeugtrace und keine getrennte beobachtete/reservierte Abrechnung. Das ist ein Befund am geprüften Mainbestand, keine Bewertung ungelesenen G-WIPs. Produktanschlüsse gehören G/K; Q ändert diese Verträge nicht.

## P0 bis P11

| Kriterium | Stand |
|---|---|
| P0 | Teilvorbereitung. 166 Kandidaten, 37 Dublettenkandidaten, 0 Goldfälle. Kein fertiges 30er-Set. |
| P1 | Nicht abgenommen. 0 echte Erwähnungs-/DM-/Twitch-Testantworten, keine Ende-zu-Ende-Zeit. |
| P2 | Fünf historische Originalerwartungen vorbereitet. Aktuelle Wissensbindung, Antworten und kosmetische Gegenprobe offen. |
| P3 | Erwartete Coaching-/Patenantwortart festgehalten. Originalquelle und Livewortlaut offen. |
| P4 | Antwortart ohne Interna vorbereitet. Drei echte Selbstbildantworten offen. |
| P5 | Ehrliche Antwortgrenze als Antwortart vorbereitet. Echte Fälle und Antworten offen. |
| P6 | Eigener Status nicht abgenommen. Keine Einladung gesendet. |
| P7 | Kein Publish durch Q und keine neue Build-ID als Q-Abnahme behauptet. |
| P8 | Keine neue Ausfallzustellung. Fremde Dienste unangetastet. |
| P9 | Aktuelle technische Teilbeobachtung, keine vollständige I/G/K-Prozess-/Import-/Consumerabnahme. |
| P10 | Keine Produktänderung oder Abschaltung; bestehende Ersatzliste bleibt bei K zu vervollständigen. |
| P11 | Eigener sicherer Teilstand in regulärer Sicherung; Gesamtauftrag und Cleanup offen. |

## Werkzeugprüfungen und Sicherung

Alle Cargo-Aufrufe liefen über `cargo-slot`, Toolchain 1.97.1. `test --locked --offline --jobs 3 -- --include-ignored`: 17 passed, 0 failed, 0 ignored, 0 filtered. Clippy mit `--all-targets -- -D warnings`, Build und `fmt --check` jeweils Exit 0. Dateisystemtests prüfen echte Dateien; neue synthetische Testfixtures werden nicht als Gold- oder Kanalfragen gezählt. Baseline nicht gemessen, keine Altfehlerbehauptung.

TESTNACHWEIS[TW-1]: 18 passed, 0 ignored | Baseline: nicht gemessen; 17 Collector-Tests plus 1 Consumerfixture, keine Altfehlerbehauptung

TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 1 belegt | Senke: .tasks/2026-10-08-q2-abnahme/BERICHT.md

Der Textnachweis betrifft den Lesertext dieses Berichts. Das Absolutwort bezieht sich auf die vollständig protokollierten eigenen Cargo-Aufrufe über cargo-slot; technische Pfade und IDs sind keine Umlaut-Ersatzschreibung.

Sourcegate für Collectorfix `b215876e`: gpt-6.1-sol ALLOW, regulär erneut bestätigt; siehe `REVIEW.md`. Der bestätigte Verzeichnisvergleichsfehler aus Runde 1 wurde im frischen nativen Fixer behoben. Der sichere Collector-/Consumer-Belegteil ist mit `2e0de01f0da4bbb7f6630832b52b0694ea566c6f` auf main gesichert. Das Gesamtgate antwortete `[gpt-6.1-sol] ALLOW: No blocking defect established in the supplied diff.`, Exit 0. `HEAD:main` lief mit Exit 0; `ls-remote` bestätigte denselben vollständigen SHA unabhängig. Die Felder `merged` und `pushed` in STATUS gelten ausschließlich für diesen Teilstand, nicht für die Gesamtfreigabe. Keine Produktdeploys oder Dienstneustarts durch Q. Kein Cleanup und kein Self-Settle bei offenem Gesamtauftrag.

Die folgende Zählung betrifft die abgeschlossene Integration von `2e0de01f`: Status, explizites Add, Stagingliste, Commit, Fetch, Mainlog, Vorfahrenprüfung (Exit 0), Push und Remoteprüfung. Frühere Git-Schritte und spätere Nachträge sind nicht darin enthalten.

MERGEPROTOKOLL[MS-1]: 9 Git-Schritte einzeln | Anläufe: 1 | Gate: gpt-6.1-sol ALLOW, HEAD:main Exit 0, Remote-SHA bestätigt
