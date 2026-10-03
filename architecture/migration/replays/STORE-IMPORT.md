# Replay-Import und interne Beispielabfrage

Der Import dekodiert eine lokale Demo und speichert ihre Beobachtungen über den bestehenden Postgres-Dokumentstore mit einem eigenen Release je Match und Lesebereich.

```sh
dbrain-replay-import import /privat/match.dem /privat/request.json /pfad/dbrain-replay-worker --infisical /pfad/config/infisical.json
dbrain-replay-import query RELEASE_ID LESEBEREICH --infisical /pfad/config/infisical.json
```

Die Anfrage benötigt einen passenden `expected_sha256`, belegte lokale Verarbeitungs- und Aufbewahrungsrechte sowie gegebenenfalls `match_reference` mit Beleg; Demo, Anfrage und temporäre Kopien müssen außerhalb von Git liegen.

Für einen Wegwerf-Testcluster ersetzt `--peer-test SOCKET PORT DATENBANK` den Infisical-Schalter, beispielsweise `--peer-test /tmp/brain-replay-r-core-20261003/pg 55439 brain_replay_test`; der aufgelöste Socketpfad muss unter `/tmp` liegen und der Datenbankname auf `_test` enden.

Identische Importe behalten Dokumentrevisionen und Release-ID; andere Rohfassungen desselben Matches behalten ihre eigenen Belege, ein Reparse ersetzt die aktive Beobachtungsgeneration atomar, während öffentliche Ausgabe, Anbieterfreigabe und die Umschaltung des allgemeinen Brain-Releases ausbleiben.
