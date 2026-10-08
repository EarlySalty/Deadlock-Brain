# Q2: zusätzliche Originalbindung ohne Modelllauf

## Öffentlicher Coachingweg geprüft

Am 08.10.2026 vor 03:05:48 UTC wurden die öffentliche Coachingseite und ihr tatsächliches Anfrageziel mit Moli gelesen. Keine Anmeldung, Formularabgabe oder Coachkontakt. Der Browser sperrte private Netze; persönliche Browser und Dienste blieben unangetastet.

- `https://deutsche-deadlock-community.de/coaching`: kostenloses deutschsprachiges Community-Coaching, Lernziel, Helden und passende Zeiten als Vorbereitung. Sichtbarer Anfragelink: `/coaching/anfrage`. Ein Coach kann einen Termin abstimmen; keine Sofortzusage.
- `https://deutsche-deadlock-community.de/coaching/anfrage`: öffentliche Anwendungshülle mit `Einloggen` beobachtet. Kein ausgefülltes Formular oder erfolgreicher Anmeldeweg belegt. Kein Layout-, Chrome- oder vollständiger JavaScript-Funktionsnachweis.
- Botsquelle `8e1b8f03`: `docs/coaching.md:4-11` bestätigt kostenloses Human-Coaching und Websiteanfrage. `rust/crates/dl-community/src/coaching_requests.rs:62` bindet dieselbe Website; `:2188-2197` leitet den vorhandenen Anfragebefehl dorthin.

Das ist eine unabhängige öffentliche Originalerwartung vor der ersten Brainantwort. Sie belegt keine eingespeiste Brainwissensversion und keine zugestellte Antwort. Das Patenangebot des Brain bleibt gemäß Auftrag in Ich-Form. Dieses eigene Hilfsangebot ist nicht mit einem bereits zugeteilten menschlichen Coach gleichzusetzen.

| Beleg | SHA256 |
|---|---|
| Öffentliche Coachingseite, geschützter Moliabzug | `27d9e8075c943fea5143b28a9d758e7ac634df9b2a462e069ecfc07fd80d0b3c` |
| Öffentliches Anfrageziel, geschützter Moliabzug | `bb34bd009b6290088f69339b39986886a3fba0d17a201972341e98413b3f4795` |
| Bots `docs/coaching.md` | `72c36f9551a4feddcbe3f67c7afc9353d255eceb292816ea253e88d7ec6e8cf5` |
| Bots `coaching_requests.rs` | `29ab5c31552fb0aab3308ea986e1d196ffff4c553698638d513c9ec56503f055` |

## Einladung: belegte Erwartung und ungelöste Ortsbindung

`docs/onboarding-und-invites.md:13-22` beschreibt eine kostenlose Anfrage am zuständigen Ort mit Steam-Freundescode und Communityhilfe. `invite_lounge.rs:16` enthält den passenden Hinweis, den Code hier beizufügen. Der Quelltext enthält zusätzlich den bestehenden automatisierten Versandweg; Q führt ihn nicht aus.

Die Kanalkennung der Dokumentation stimmt nicht mit der Konstante in `invite_lounge.rs:15` überein. Kennungen werden hier nicht veröffentlicht. Das ist eine dokumentierte Quellenabweichung, kein geprüfter Produktionsfehler. Vor einem Sollurteil sind der tatsächlich sichtbare Ort, Server-as-Code und die Originalnachricht lokal zu binden. Der Brain darf bei einer Frage am richtigen Ort nicht in denselben Bereich zurückverweisen. Kein eigener Versand, keine Personendaten oder angenommener Einladungsstatus.

Dokumenthash: `5d3b1bf20e6bd1d0fbda0fb237abe5d3e5a4d174e1f84faa4c0e8f93818094b4`. Quellhash: `523a49907a4ba050754544c99a9657a6f050315e53040b1a10280a16182958d5`. Beide auf Bots `8e1b8f03`.

## Bestehende Discordquelle ergänzt, nicht ersetzt

Die vorhandene Rust-Sammlung wurde erneut verwendet, weil die eingefrorene Altquelle keinen Treffer für die englischen Itemnamen und keinen Treffer mit drei numerischen Discord-Erwähnungen enthält. Kein neuer Connector, kein neuer Kanal und kein Eingriff in alte Originale. Neuer Abzug `q2-required-source-v1-20261008`: Obergrenze 80 Seiten im vorhandenen lesenden Adapter, 166 menschlich markierte Nachrichten, 12 Parserkandidaten, kein Sammlungsfehler, 0 Goldfälle und 0 Modellaufrufe.

| Lokale reine Stichwortprüfung | Altquelle, 87 Nachrichten | Ergänzung, 166 Nachrichten |
|---|---:|---:|
| Armor Piercing / Armor Piercer / Plated Armor | 0 | 0 |
| Haze | 1 | 2 |
| Neuer Spieler / schlecht / Anfänger / Coaching / Pate | 1 | 2 |
| Pocket | 1 | 4 |
| Einladung / Invite / Deadlock spielen | 1 | 1 |
| Genau drei numerische Discord-Erwähnungen | 0 | 0 |

In der Ergänzung enthält kein einzelner Text zugleich die geprüften Haze-, Pocket- und Hilfestichwörter. Diese Zählung liest lokal und gibt weder Texte noch Kennungen aus. Sie erkennt keine sinngleichen Formulierungen, beweist keine Echtheit und ordnet keine Nachricht automatisch dem Nutzertest zu. Die gemeinsame Dreifachnachricht wird nicht aus getrennten Treffern zusammengesetzt. Die Originalbindung der fünf Pflichtfälle bleibt offen.

Snapshot: `b2658ecc48cdd33bae624dbd53a00cb9205694020d909ef0f01a0aea8744c238`. Ergänzender Teilplan: `043ce3ffd912977b5511f877db8cc536ab79eed5b0f2706d41d77db37c2dab58`. Der ursprüngliche 166-Kandidatenplan und seine geschützte Prüfliste bleiben unverändert; die 12 neuen Parserkandidaten werden nicht hinzuaddiert oder als neue echte Fälle behauptet.

Die unabhängige Kopie unter `/home/nathanael/.local/share/brain-q2-private-20261008/required-user-sources/` ist geprüft: vier Dateien, ein Verzeichnis, zwei Digestbindungen, gleiche relative Pfade und Bytes, Eigentümer und 0600/0700 korrekt. Inventarhash `a6de1a486f269c696c653fcfcd649b1745a63f8a3ab0a18879e911f05d051468`.

## Tatsächliche verbleibende Grenze

Öffentliche Fakten und isolierte Consumerkontrollen können vor dem Livegang unabhängig vorbereitet werden. Ein echtes 30er-Goldset benötigt zusätzlich geschützte fachliche Originalprüfung; Stichwortzähler und Testfixtures ersetzen sie nicht. Der G-Vertrag `ANTWORTPORT-VERTRAG.md:11,31-37` nennt interne Accounted-/ToolExecution-Typen, liefert aber keinen tatsächlichen geschützten Messlauf aus dem ausgelieferten Consumer. Keine neue Exportpipeline als Ersatz. G/K-Livelieferung und erneute I-Wissensbindung bleiben Voraussetzung vor dem ersten Luna-Lauf.
