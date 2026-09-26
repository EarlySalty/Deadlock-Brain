# S11: Restore-, Cutover- und Ausfallprobe

**Status aller nachfolgenden Betriebsproben: nicht durchgeführt.**
G1/G2, integrierte Verträge und isolierte Stagingfreigabe fehlen. Die Tests des
Inventarwerkzeugs sind keine dieser Integrationsproben und kein G4-Nachweis.

## Voraussetzungen

S00 bestätigt Scope, Besitzer und integrierten Commit. S03 stellt einen leeren,
isolierten Postgres-Store, den echten Restore-/Delta-/Fencingvertrag und erlaubte
Snapshotreferenzen bereit. S04 stellt getrennte Queues und Jobidentitäten bereit.
S08/S09 liefern die echten Readiness-/Consumerverträge. S10 setzt Lastprofil,
Abbruchgrenzen, Kostenbudget und Ressourcenlimits vor der Messung fest.
S12 bis S14 liefern erlaubte Wiki-/API-/Replayfälle und ihre gepinnten Versionen.

Kein produktives DSN, kein echter Publikationskanal und keine Produktionssecrets
an ungeprüften PR-Code. ACL-/Deletefälle synthetisch oder ausdrücklich freigegeben;
Raw-Replays und private Wissensinhalte bleiben außerhalb öffentlicher Artefakte.
Keinen Pflichtpfad durch einen Mock als erfolgreich kennzeichnen.

## Prüfmatrix für die reale Stagingumgebung

| ID | Aufbau und Störung | Verpflichtender Nachweis | Besitzer |
|---|---|---|---|
| S11-R01 | Leeres Ziel aus freigegebenem Snapshot wiederherstellen | Hash-/Mengen-/Versionsabgleich, Policies, ACLs, Tombstones und Source-Watermarks stimmen; RTO/RPO gemessen | 03/10/11 |
| S11-R02 | Indexe/Karten/Graph/Population aus zugelassenen Rohrevisionen neu aufbauen | Eine kompatible Knowledge-Version; reproduzierbare Generationen, Rechte und Deletes auch auf Repliken | 03/05/06/12/14 |
| S11-R03 | Alten Writer während Queue-Retry und Prozessneustart fencen | Alte Generation kann keinen Write mehr bestätigen; gestoppter Timer allein genügt nicht | 03/04/11 |
| S11-R04 | Nach dem letzten Altoffset neue Änderungen, Deletes und ACL-Sperren einspielen | Vollständiger Deltaabgleich für Wiki, externe Feeds, Replays, Learning und Publishing; idempotente Wiederholung | 03/04/12/13/14 |
| S11-R05 | Neuen Writer aktivieren und alten verzögerten Write zustellen | Genau eine wirksame Writergeneration pro vereinbartem Bereich; abgelehnter Altwrite und bestätigter Neuwrite belegt | 03/04/11 |
| S11-R06 | Nach erfolgreichen Neuwrites Rollback erzwingen | Neuwrites exportiert/replayt; Rückfallrelease schemafähig; aktuelle ACLs/Tombstones und Secret-Rotation bleiben erhalten | 03/10/11 |
| S11-R07 | Unlesbares Rückfallschema oder unvollständiges Delta | Schreiben bleibt angehalten; kein scheinbar erfolgreicher Restore mit Datenverlust | 03/08/11 |
| S11-R08 | DB-, Index-, Replika-, Auth- oder Providerteilausfall | Echte Readiness reagiert korrekt; kein größerer Scope, kein Stale-Generation-Cache und keine widersprüchliche Publishantwort | 03/06/07/08 |
| S11-R09 | Interner und Twitch-Canary mit Shadowverkehr | Keine doppelten Writes/Publikationen/Lernereignisse; Providerkosten innerhalb vorheriger Freigabe | 07/08/09/10 |
| S11-R10 | Reguläre Updates, Reparse, Learning und Rebuild bei gesperrten Legacyendpunkten | Alle erforderlichen Zyklen funktionieren ohne notwendigen Python-/Legacykern; Decoderressourcen begrenzt | 04/05/11/12/13/14 |
| S11-R11 | Discord/MCP/CLI/Web sowie Docs-/Buildpublishing durchlaufen | Gleicher Kernel, keine direkten Consumer-Modellpfade, freigegebene Exports und unveränderte Rechte | 08/09 |
| S11-R12 | Replay-/Ingest-/Reembeddinglast während Querytraffic und Drain | Gemessene p95/p99, Queuealter, Frische, Ressourcenreserve und Shutdown innerhalb S10-Grenzen | 04/06/10/11 |
| S11-R13 | Schemawechsel, Quellenausfall oder korruptes Replay | Drift/Konflikt sichtbar, Quarantäne statt stiller Aktivierung; dokumentierter degradierter Scope, keine Vollständigkeitsbehauptung | 04/12/13/14 |
| S11-R14 | Altschedule nach Übergang erneut auslösen, einschließlich manueller Startwege | Fencing bleibt wirksam; kein doppelter Publish- oder Learningzyklus | 03/04/09/11 |

## Protokoll je Probe

Erfassen: Test-ID, verantwortliche Person, Start/Ende, Umgebung, **vollständiger
getesteter Commit**, Toolchain-/Binary-/Imagehash, Contract-/DB-Schema-Version,
Knowledge-/Fact-/Rule-/Karten-/Source-/Replay-/Populationrevision, Policy- und
Tombstone-Checkpoint, alte/neue Writeridentität, genaue Befehle, relevante
Watermarks, erwartetes und tatsächlich gemessenes Ergebnis, Artefaktreferenz,
Abbruch/Recovery und verbleibende Unsicherheit. Keine Tokenwerte protokollieren.

Bei Änderungen an Code, Schema, Policies, Pflichtscope, Modellkonfiguration oder
Datenrelease die betroffenen Nachweise neu durchführen. Ein erfolgreicher Test
auf einem älteren Branch gibt den neuen Stand nicht frei.

## Vor G5 ausdrücklich auszufüllen

Verantwortlicher Betreiber und S00-Freigabe; erlaubtes Umschaltfenster; vollständige
Releaseidentität; getestetes Rückfallrelease; genehmigte Last-/Frische-/Fehler-
grenzen; Providerbudget; Rollbackfenster und Aufbewahrung; getrennte Freigaben für
spätere Service-/Key-Stilllegung und irreversible Archivierung/Löschung.
Fehlende Werte sind Stopps, keine frei wählbaren Defaults.
