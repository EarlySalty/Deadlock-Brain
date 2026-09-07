# Research: Reddit-Ingest für Deadlock-Brain

status: aktiv
datum: 2026-09-07
klasse: hoch

## Auftrag

Öffentliche `r/Deadlock`-Threads landen als Rohdokumente und quarantined Claims
im Brain, analog `pull forum` / `parse forum-claims`.

## Beobachtungen (belegt, Datei:Zeile)

- Forum-Ingest ist die Vorlage: Sitemap → Thread-HTML → `source_documents` +
  Snapshots `forum_thread`/`forum_post` (`forum.rs:17,72,164,520`).
- Claims sind bewusst nicht Ground Truth (`ARCHITECTURE.md:81`,
  `forum_claims.rs:90`, `currentness=historical_quarantine`).
- HTTP ist blocking `HttpClient` mit Cache und User-Agent
  (`http.rs:71`, `config.rs:9`). Forum ruft `http.get` synchron aus async
  Funktionen (`forum.rs:170`).
- `forum_claims` Insert nutzt `sqlx::query` statt `query!`
  (`forum_claims.rs:100`), deshalb kein sqlx-Prepare nötig, wenn Reddit dasselbe tut.
- Tabelle `brain.forum_claims` hat keine Source-Spalte
  (`0012_brain_knowledge_timeline.sql:219`). Rebuild macht heute
  `DELETE FROM brain.forum_claims` komplett (`forum_claims.rs:48`). Das muss
  source-scoped werden, sobald Reddit dieselbe Tabelle nutzt.
- Reddit stand als Zukunftsquelle in `BUILD_LEARNING.md:71` und wurde am
  2026-07-12 als „getarnte AI stellt Fragen“ gestrichen. User 2026-09-07:
  nur lesen, bauen. Kein Reddit-Code im Repo, kein Commit.
- Default-User-Agent enthält Kontaktmail, das ist für Reddit-JSON Pflicht.

## Hypothesen (unbelegt — nie als Fakt weiterreichen)

- Datacenter-IPs bekommen auf `reddit.com/*.json` oft 403. Dann RSS
  (`/r/Deadlock/new/.rss`, `/comments/{id}.rss`) als Fallback. Ob das von
  diesem Host geht, zeigt erst ein Live-Lauf; Parser müssen beide Formate
  aus Fixtures können.
- Tiefe Comment-Bäume (`kind=more`) ohne Extra-Request lassen Lücken. V1
  nimmt nur, was im ersten Thread-Dokument steckt.

## Wahrscheinlich zu ändernde Dateien

- `dbrain-sources/src/reddit.rs` (neu)
- `dbrain-sources/src/lib.rs`
- `dbrain-normalize/src/reddit_claims.rs` (neu)
- `dbrain-normalize/src/forum_claims.rs` (DELETE)
- `dbrain-normalize/src/lib.rs`
- `deadlock-brain/src/main.rs`
- `README.md`

## Risiken / Seiteneffekte

- Reddit-Rate-Limit / 403: fail-soft, Summary zählt Fehler, kein Bypass.
- Unscoped DELETE in `parse forum-claims --rebuild` würde Reddit-Claims
  mitlöschen, wenn man das nicht vorher source-scoped macht.
- Gemeinsame Tabelle `forum_claims` ist Namens-Altlast; Contract verbietet
  neue Tabelle/Migration in diesem Slice.

## Offene Fragen

- keine produktseitig. Ob JSON oder RSS auf dem Live-Host ankommt, ist
  Laufzeit, nicht Spec.
