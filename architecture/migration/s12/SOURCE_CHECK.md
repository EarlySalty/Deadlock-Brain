# S12-Quellen- und Basisprüfung, 24. September 2026

## Integrierte Repo-Basis

Live abgefragt: `origin/main = 3032651`. STATUS: G0/G1 offen; Contract-/Schema-Version offen. S00-Freigaben erlauben keine produktive S12-Implementierung vor G1. Arbeitsbereich ausschließlich `architecture/migration/s12/**` und eigene Übergabe. Fremde uncommittete Runtime-Arbeit im Hauptcheckout blieb unverändert; eigener Worktree/Branch.

Reale bestehende Integrationspfade: `rust/crates/dbrain-sources/src/wiki.rs`, `rust/crates/dbrain-retrieval/src/game_wiki.rs`, `rust/crates/deadlock-brain/src/wiki_refresh.rs`. Offene Vorarbeit PR #13, Head `3aae8e0d9f8878a559e2683cdbf0c1306a4b91f3`. Sie wurde anhand der Live-PR-Metadaten und Dateiliste eingeordnet, nicht als bereits integrierte oder live abgenommene Implementierung gewertet.

## Einzelner erlaubter Metadaten-Probeabruf

```text
GET https://deadlock.wiki/api.php?action=query&meta=siteinfo&siprop=general%7Cnamespaces%7Crightsinfo&format=json
Arbeitsserver, 2026-09-24
curl --max-time 20 --max-filesize 1000000
HTTP-Status: 403
Antwortumfang: 5789 Bytes
```

Der Antwortbody wurde nur temporär lokal abgelegt, nicht als Wiki-JSON verarbeitet und nicht ins Repository übernommen. Kein angeblich erfolgreiches `siteinfo`, keine daraus abgeleitete Lizenzfreigabe, kein bekannter Live-Namespace-Denominator. Keine Auth-/WAF-Umgehung, keine Massendiscovery und kein Vollimport. Ein zusätzlicher Web-Leseversuch auf diesen Metadatenendpoint lieferte ebenfalls keinen verwertbaren Inhalt; belastbarer HTTP-Status stammt aus dem Server-Probeabruf.

## Syntaxprüfung gegen Primärdokumentation

Am 24.09.2026 gelesen:

- https://www.mediawiki.org/wiki/API:Siteinfo — Metadaten und dynamische Namespaces.
- https://www.mediawiki.org/wiki/API:Allpages — namespaceweise Enumeration und vollständige Continuation; auch leere Teilbatches können fortgesetzt werden.
- https://www.mediawiki.org/wiki/API:Revisions — Revisions-IDs/Quellzeiten, Content-Slots und unterdrückter Inhalt.

Diese Prüfung rechtfertigt synthetische Response-Fixtures, aber keine Aussage über die installierten Deadlock-Wiki-Erweiterungen, aktuelle Data-/Template-/Lua-Schemata, aktuelle Lizenz-/Zugriffsfreigaben oder echten vollständigen Datenbestand. Es werden keine Textpassagen aus diesen Quellen als Testdaten kopiert.
