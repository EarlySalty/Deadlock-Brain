# Q2: Methodik und Wiederholung

## Fester Bestand vor dem ersten Lauf

`Q/private/q-partial-v1-20261007` bleibt unverändert. `compare-private` prüft die bestehende Arbeitskopie und unabhängige Sicherung mit Dateimodus 0600, Verzeichnismodus 0700, Eigentümer, Pfaden, Bytes und vorhandenen JSON-/Digestbindungen. Die neue Inventarmethode ist ausdrücklich benannt; ihr Hash wird nicht mit dem historischen anders gebildeten Inventarhash gleichgesetzt.

`audit` bindet den vorhandenen Teilplan erneut an vier Quellhashes. Gleiche Fragen nach Entfernung numerischer Erwähnungen, Kleinschreibung und Leerraumvereinheitlichung werden als mögliche Dubletten markiert. Das erkennt keine sinngleichen Fragen und beweist keine menschliche Herkunft. Mehrere Teilfragen bleiben im ursprünglichen Nachrichtencontainer.

`prepare-review` erzeugt eine geschützte lokale Prüfliste. Sie enthält die vorhandenen Originalfragen, Quellzeilen und ungeprüften Parserhinweise, aber setzt keinen Fall auf akzeptiert. stdout enthält Mengen und Hashes, keine Fragen oder Personenkennungen. Ein vorhandenes Ziel wird nicht überschrieben.

Vor Goldfreigabe lokal je Fall prüfen: echte Frage, Ursprung, Überschneidungen, minimale bereinigte Providerfrage, feste Antwortart, unabhängige Originalfakten und Belegdateien. Originaltext und Personenreferenzen bleiben lokal. Parserlabels nicht übernehmen, ohne sie zu prüfen. Sechs Antwortarten: Spiel, Patch, Server/eigener Status, Coaching, Selbstbild, ehrliches Nichtwissen/Unsinn. Unbeschriftet bedeutet nicht Unsinn.

`check-review` prüft Struktur und Quellenbindung, keine fachliche Richtigkeit. Er verlangt mindestens 30 unterschiedliche akzeptierte Nachrichtenfälle, die drei Quellengruppen, sechs Antwortarten sowie lokale Prüf- und Originalfaktenbelege. Prüfhashes müssen auf tatsächlich lokal vorhandene Nachweise zurückgeführt werden; die bloße Eintragung eines Hashes ist kein fachlicher Beweis. Der Validator gibt keine Erlaubnis für Liveaufrufe. Die zusätzlichen Nutzerfälle und ihre Originaltransportform sind außerdem anhand `FALLVERTRAEGE.json` zu binden, bevor das Set vollständig heißt.

## Wiederverwendete Werkzeuge und Grenzen

| Bestand | Zweck und Grenze |
|---|---|
| Vorhandener `Q/collector` | Quellensicherung, Integrität, lokale Vorbereitung und Strukturprüfung. Kein Provider oder Zustellungsrunner. |
| `architecture/migration/evals` | Offlineauswertung vorhandener Messdaten. Der kanonische S10-Katalog enthält andere Kontrollfälle und ersetzt keine echten Q-Goldfragen. |
| `scripts/run_local_pilot.sh`, `scripts/test_brain_serve.sh` | Isolierte Laufzeit- und Consumerprüfungen. Kein Nachweis echter Discord-/Twitch-Zustellung. |
| `brain-client` und bestehende Discord-/Twitch-Consumer | Geplanter echter Antwortweg. Kein zweiter Connector und keine Umgehung der Consumerrechte. |

Die Bestandsprüfung wurde mit Graphify begonnen. Der zugewiesene Worktree besitzt keinen eigenen Graph. Globale Abfragen wurden verwendet, Fundstellen anschließend im aktuellen Checkout geprüft. Kein Graph-Neubau.

## Ausführungsbedingung

Bevor Fragen an einen Provider gehen: belegte G/K-Livelieferung, I-Spiegel erneut an tatsächlichen Import binden, Consumer-Prozess und Release-SHA feststellen. Health/Ready 200 genügt nicht. Konfiguration, Wissensstand und Quellstand getrennt erfassen. Erste Konfiguration `codex_subscription`, Modell `gpt-6-luna`. Kostenpflichtige Vergleiche benötigen ausdrückliche Kostenfreigabe.

Interne Rollen- und Identitätsbindung beweisen. Fehlende Bindung erlaubt keinen erweiterten Zugriff. Keine zusätzliche Kategoriesperre. Private Antworten werden aus eigener Frage und freigegebenem Spiel-/Serverwissen erzeugt; private Discord-Nachrichten anderer Personen werden gemäß Privatfixentscheidung nicht gelesen. Antwortzustellung und fehlender Lesezugriff sind getrennte Prüfpunkte. Auch Argumente und Werkzeugantworten bleiben lokal geschützt.

## Ablauf auf demselben festen Set

1. Goldset nach Originalprüfung versionieren. Privaten Setinhalt unveränderlich sichern; im Repo stehen Version, Hash, Mengen, Antwortarten und sichere Fallverträge. Set-, Quellen-, Rechte-, Werkzeug- und Rust-Rechenbindung gemeinsam einfrieren.
2. Echte Fragen über belegte Testkonten an vorhandene Consumer geben. Uhr startet beim angenommenen Nachrichtenereignis und endet bei sichtbarer Antwortzustellung. Discord-Erwähnung, DM und Twitch getrennt erfassen. Reine HTTP-Zeit ist kein P1-Nachweis.
3. Je Fall lokale Rohbelege und sichere Messwerte gemäß `LAUFVERTRAG.json` speichern. Zahlen mit dem tatsächlich ausgelieferten Rust-Rechenkern vergleichen; ein vom Modell berechneter Wert ist kein Rechenmaßstab. Quellen auf konkrete Antwortbehauptungen zurückführen.
4. Dieselbe Setversion nach späteren Deploys wiederholen. Datenstandsänderungen dokumentieren, nicht erwartete Fakten passend machen. Bei genehmigtem Modellvergleich bleibt die feste Schicht unverändert; gewechselt wird die vorhandene zentrale Providerkonfiguration.
5. A-Fragen erneut prüfen und P0 bis P11 getrennt ausweisen. Fehlende Messwerte bleiben unbekannt. Messurteile führen nicht zu eigenmächtiger produktiver Modellwahl.

Eine konkrete ausführbare Kanalwiederholung ist noch nicht belegt. Der vorbereitete Collector ersetzt diesen fehlenden Liveweg nicht. Toolargumente und Antwortbelege müssen aus dem tatsächlich ausgelieferten G/K-Pfad stammen; keine erfundenen Traces ergänzen.

## Verbrauch und Zeit

`UsageAccounting.observed`, `reserved` und `unaccounted` getrennt erfassen. Ein numerischer Nullwert aus einer Standardstruktur beweist keine beobachtete Abrechnung. `AnswerResponse.usage` allein reicht nicht für eine vollständige Werkzeug-/Abrechnungsrekonstruktion. Fehlende tatsächliche Tokenwerte als `null` mit Grund speichern. Kein Preis aus Reservierung ableiten, kein Abopreis als gemessene API-Kosten ausgeben.

P1 gilt je belegtem Fall bei korrekter aktueller Antwort und Ende-zu-Ende-Zeit strikt unter 20 Sekunden. Keine historischen Healthproben, synthetischen HTTP-Fixtures oder Compilerläufe als Ersatz zählen.

## Tagesgrenze und Ortskontext

Erste Spielfrage, unmittelbare Folgefrage und dreifache Teilfrage als ein vollständiges Nachrichtenereignis prüfen. Fachliche Teilfragen, Nachrichten und Modellaufrufe getrennt zählen. Mehrere Erwähnungen führen nicht zu mehreren Reservierungen derselben Nachricht.

50/51, zwei Nutzer und Berliner Tageswechsel isoliert mit injizierter Zeit prüfen. Sommerzeitbeginn am 29.03.2026 und Winterzeitbeginn am 25.10.2026 ergänzen. Kein Produktivspam, keine manuelle Änderung von Produktivzählern. Genau eine sichtbare Grenzrückmeldung mit Hinweis auf morgen; keine Sekunden-, Stunden-/Kanal- oder globale Tagesgrenze.

Einladungsfrage am tatsächlich zuständigen Ort: konkreter nächster Schritt, kein Rückverweis dorthin. Gültigen Ablauf unabhängig an aktueller Quelle prüfen. Kanalzweck, Thread-/DM-/Twitch-Kontext und Eingangsart minimal rollenbasiert binden. Fehlende Ortsdaten und künstliche Kontrollfälle gesondert ausweisen, nicht als echte Goldfragen zählen. Keine Einladung senden und keinen fremden Build veröffentlichen.
