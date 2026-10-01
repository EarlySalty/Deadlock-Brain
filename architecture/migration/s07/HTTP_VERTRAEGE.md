# S07 · HTTP-Verträge, geprüft am 24.09.2026

Status: Dokumentationsprüfung, kein Live-Vertragstest. Angaben des Anbieters
werden von vorgeschlagenen internen Validierungsregeln getrennt.

## Jev / TypeSafe

`POST https://api.typesafe.ai/v1/systemone`, Bearer-Authentifizierung, JSON.
Request: `model`, `state`, `questions` nach frei gewählten IDs. State und
Instructions können Text, Objekt oder Array sein. Response: `model`, `answers`
unter den zugehörigen IDs, `usage` mit Eingabe-/Ausgabetokens. Fehler werden
unter anderem als 401, 422, 429 und 529 dokumentiert. [J1]

| Typ | Unterschied, der beim Rust-Port erhalten bleiben muss | Quelle |
|---|---|---|
| Noul | `noul` ist eine Ja-Wahrscheinlichkeit zwischen 0 und 1, kein gemeinsamer Confidence-Slot. | [J2] |
| Choice | Kriterien sind benannte Alternativen; Rückgabe enthält `choice`, vollständige `probabilities` und `confidence`. Auswahl entspricht einer maximalen Wahrscheinlichkeit. | [J3] |
| Score | Geordnete Rubrik mit 2–10 Stufen. Score ist der gewichtete Mittelwert der nullbasierten Stufen, kann gebrochen und größer als 1 sein. `legend`, `probabilities` und `confidence` bleiben erhalten. | [J4] |

Choice-/Score-Confidence beschreibt die Verteilung, nicht garantierte
fachliche Korrektheit. Eine nicht veröffentlichte Berechnungsformel wird nicht
nachgebaut. Schwellen dürfen nicht zwischen den drei Typen übertragen werden.
Anbieteraussagen über Kalibrierung ersetzen keine eigenen Entwicklungs- und
Testlabels. [J5]

Die Modelldokumentation nennt `jev-1.13.0` und bewegliche Aliasse.
`GET https://api.typesafe.ai/v1/models` verwendet `models` mit `name`,
`description`, `release_date`. Das ist nicht das Fireworks-Katalogformat.
Dokumentierter Aliasstand ist kein tatsächlich aufgelöstes Modell. [J6]

## Fireworks und Embeddings

Der vorhandene Brain-Chatpfad nutzt
`POST https://api.fireworks.ai/inference/v1/chat/completions`.
Die Referenz dokumentiert unter anderem `model`, `messages`, `max_tokens`
sowie Responses mit `id`, `created`, `model`, `choices`, `object` und optionalen
Nutzungsdaten. Ein fehlendes oder nulles `usage` ist nicht „null Kosten“.
Das bestehende `max_tokens`-Wirefeld wird nicht grundlos ersetzt. [F1]

Für Embeddings dokumentiert Fireworks einen gesonderten Inference-Endpunkt
`/embeddings`. Modell und Eingaben sind eigene Requestparameter. Embedding-
Modell, Dimension, Indexzuordnung, Normalisierung und Revision müssen zu dem
von S06 gelesenen Index passen; aus einem Chatmodell folgt keine Embedding-
Fähigkeit. Diese Kompatibilitätsentscheidung bleibt offen. [F2]

**Resolver-Lücke:** Die gelesene offizielle „List Models“-Referenz beschreibt
`GET https://api.fireworks.ai/v1/accounts/{account_id}/models` mit
`models`/`name` und Paginierung. Sie belegt nicht die vom bestehenden Code
verwendete Inference-Variante `/inference/v1/models` mit `data`/`id`.
Diese Variante ist damit weder als ungültig bewiesen noch primärquellen- oder
livebestätigt. Vor einem Umbau exakten autorisierten Vertrag klären; keine
Management-Credentials vorsorglich beschaffen. [F3]

## Interne Prüfregeln: Vorschlag an 02, noch kein freigegebener Contract

Erwartete Question-IDs und Antworttypen müssen exakt zum gesendeten Request
passen. Labels und Score-Indizes dürfen nicht hinzugefügt oder ausgelassen
werden. Alle Zahlen müssen endlich und im jeweiligen Wertebereich liegen.
Verteilungen summieren sich mit explizit versionierter Rundungstoleranz zu 1;
Score und gewählte Maximaloption müssen dazu passen. Gleichstände sind zulässig.
Für die vorliegenden String-Rubriken muss auch die Legende exakt passen.
Strukturierte Rubriken sind absichtlich noch nicht in den Positivfixtures.

Zusätzliche entscheidungsrelevante Felder werden im vorgeschlagenen Profil
abgelehnt. Unbekannte harmlose Metadaten benötigen eine ausdrücklich
versionierte Kompatibilitätsregel. Dies ist unsere Fail-closed-Entscheidung,
nicht eine Behauptung, dass der Anbieter niemals Felder ergänzt. Doppelte
JSON-Schlüssel müssen vor Verlust durch Map-Deserialisierung erkannt werden.
Fehlende Kosten-/Nutzungsprovenienz bleibt unbekannt; negative Tokenzahlen
sind ungültig. Antworttext ist niemals eine Berechtigung oder Provider-URL.

Die Fixtures enthalten reservierte Test-Modellnamen. Kein Fixture darf als
freigegebene Livekonfiguration benutzt werden. Modellfamilie, Aliasauflösung,
antwortende Revision, Rubrikrevision, Rechte-/Konfigurationsstand und Messdaten
bleiben getrennte Informationen. Kein neues Modell ist mit diesem Dokument genehmigt.

## Primärquellen

- [J1] https://docs.typesafe.ai/api
- [J2] https://docs.typesafe.ai/primitives/noul
- [J3] https://docs.typesafe.ai/primitives/choice
- [J4] https://docs.typesafe.ai/primitives/score
- [J5] https://docs.typesafe.ai/confidence
- [J6] https://docs.typesafe.ai/models
- [F1] https://docs.fireworks.ai/api-reference/post-chatcompletions
- [F2] https://docs.fireworks.ai/guides/querying-embeddings-models
- [F3] https://docs.fireworks.ai/api-reference/list-models

Quellen sind mutable öffentliche Dokumentation, kein unveränderlicher Schema-
Snapshot. Die Fixtures sind selbst erstellte Beispiele, keine kopierten
Antworten und keine Belege für Produktivmodellverfügbarkeit oder Latenz.
