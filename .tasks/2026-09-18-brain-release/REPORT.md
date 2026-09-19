# REPORT: Brain-Release

Branch `codex/brain-release-20260918`, Worktree `/home/nathanael/.worktrees/brain-release-20260918`, Basis `9efeb1e`. Einziger Worker, keine Unter-Agenten. Nur Code/Tests/Feature-Push durch mich; Merge, Produktionsmigration, Deploy, Modelllauf und Live-Beweis übernimmt der Orchestrator.

## TLDR

Technischer, deployfähiger Releasestand steht und ist gegen die reale Referenz-DB geprüft: deterministische Kontextauswahl mit Gameplay-Projektion (realer patch_285-Kontext 667.808 Bytes, unter Budget), revisionssicherer Patch-Sync mit Inhalts-/Eventbasis-Drift, identitätsgeprüfter Quellenrefresh, drei Evidenzmigrationen (first_observed_at, Caption-Text-SHA, kanonische Identität, R4 Identitätswechsel), voller R6-Konsistenzvertrag in Review und Drift, MCP-Anschluss der revisionssicheren History über den bestehenden Weg, versionierter Sync-Betriebsweg, repo_root-Relokation. Offen bleiben die vier DB-gestützten YT-Vertragstests, die Consumer-Revalidierungsprüfung und die getrennte 11941-Parität; kein Nachweis autonomen Spielverständnisses, U7/U8 nicht angebunden.

## Geliefert und geprüft

1. **Kontextauswahl (U1-Blocker).** `deadlock-brain-patch-review build_context`: voller Patchtext und alle Events, plus deterministische Gameplay-Projektion je Entity (Allowlist spielrelevanter Felder; `properties` auf value/scale_function/postvalue_label; Icon/CSS/Tooltip/Lore entfernt). Belegte Verbindungen Held->Kernkit (items signature1..4/weapon_primary=class_name) und Ability->Besitzerheld halten indirekt betroffene Helden mit echten Properties im Kontext. Übrige Entities bleiben als kompakter, nach Typ gruppierter Namenskatalog sichtbar. Jeder Beleg behält snapshot-id, content_hash, json_path; unbestätigte Vorversion bleibt unbekannt; kein String-Kürzen. Überlauf blockt gemessen vor jedem Modellaufruf. Real gemessen (Referenz-DB, patch_285): 667.808 Bytes, 122 Events, 147 Detail-Entities (47 direkt + 100 Kernkit), Katalog 1371.
2. **Revisionssicherer Patch-Sync.** `deadlock-brain pg sync-patchnotes [--apply] [--limit]`. Drift = Inhalts-/Eventbasis-Abgleich (R6-Vertrag): neue Quelle, geänderter raw_content, fehlende oder gemischte Eventbasis lösen einen Reimport aus; nur einheitliche, passende Basis gilt als unverändert. `--apply` importiert über `import_one_patchnote` (from_row, prune+reinsert), sodass Snapshot==Quelle stabil bleibt (zweiter Sync ist ein No-op). Kanonische Identität einheitlich zur Parser-Konvention und zum realen Bestand (Quellen-URL; nur numerische IDs -> patch_<n>); Prune entfernt auch Altparser-Events ohne metadata.importer über die Snapshot-Verknüpfung.
3. **Identitätsgeprüfter Quellenrefresh.** `deadlock-brain pg refresh-official --patch-id <id> [--apply] [--response-file <p>]` über `ajaxgetpartnerevent`. Echter URL-Parser prüft Scheme/Host/Port/Userinfo/Pfad (games/<appid>/announcements/detail/<numerische gid>); Query/Fragment können den Pfad nicht ersetzen. Vor jedem Write: aktiver Capture-Trigger auf patchnotes.changelog_posts Pflicht (sonst Abbruch), Compare-and-Swap unter Row-Lock gegen Nebenläufigkeit, Provenance (live_http vs local_response_file), Roh-Beleg (Body/GID/updatetime/Hash) über brain.source_documents. Nur Quellzeile, kein Sammler, keine Veröffentlichung.
4. **Evidenzmigrationen.** `2026-09-18-patch-evidence-followup.sql` (first_observed_at-Semantik, Caption-Text-SHA, kanonischer Reviewschlüssel) und `-followup2.sql` (R4: Identitätswechsel invalidiert alten UND neuen Review). Erste Migration unverändert; alle drei idempotent; Regressionen: alte Migration allein lässt die neuen Assertions nachweislich scheitern.
5. **Voller R6-Vertrag.** `build_context` und `detect_patchnote_drift` prüfen beide, dass ALLE aktiven Events auf genau einer Basisfassung beruhen, die zum Quelltext passt; fehlende/gemischte Basis ist fail-closed (Block bzw. drift=true), nie „unverändert".
6. **MCP-Anschluss.** Neues Tool `patch_history_v1` in `mcp/server.py` routet über die revisionssichere Rust-CLI (`brain.patch_history_v1`); Treffer tragen state/observation_kind/first_observed_at, damit gelöschte oder revalidierungsbedürftige Fassungen nicht als aktuelle Fakten erscheinen. Bestehende Toolnamen bleiben, kein zweiter Dienst, kein neuer Port (weiter FastMCP-stdio). tools/list und tools/call real gegen Scratch geprüft.
7. **Betriebsweg.** `scripts/ops/patchnotes-sync-v2.sh`: ersetzt den MAX(id)-Vorcheck durch den Inhalts-/Eventbasis-Drift; neue Quellen weiter über den vorhandenen Sammler, unveränderte lösen keinen Neulauf aus.
8. **Relokation.** `repo_root()` priorisiert `DEADLOCK_BRAIN_ROOT` (aus Feature-Worktree gebautes, danach bereinigtes Release zeigt aufs installierte Repo) mit erhaltenem Build-Zeit-Fallback. `target-rel`/`target-u0` aus Git ausgeschlossen.

## End-to-End-Pipeline-Smoke (real, auf Klon der Referenz-DB)

`normalize entities --rebuild` -> `refresh-official --apply` (Abrams-Zeile aus offizieller Quelle entfernt, identity_verified, Roh-Beleg gespeichert) -> `sync-patchnotes --apply` (Events 123->122, obsolete Zeile aus aktiven Events weg, als Revision erhalten) -> `review` (Kontext 667.808 Bytes, kein Block) -> zweiter `sync-patchnotes` (drift=false) -> zweiter `--apply` (kein neuer Import). Stabil.

## Tests (wirklich ausgeführt)

Stable `cargo 1.97.1`, eigenes `target-rel` (nicht das geteilte `target-u0`), `--jobs 2`. DB gegen isolierte `initdb`-Cluster bzw. Klon der Referenz-DB, nie den geteilten Bestand mutierend.

```
cargo test -p deadlock-brain --bin deadlock-brain            -> 51 passed
cargo test -p deadlock-brain --bin deadlock-brain-patch-review -> 11 passed
cargo test -p deadlock-brain-core --lib config              -> 2 passed (Relokation)
cargo clippy -p deadlock-brain -p deadlock-brain-yt         -> 0 Warnungen
mcp/test_server.py (venv)                                   -> 6 passed
SQL (isolierter Cluster): alte Migration scheitert an neuen Assertions (history, caption, identity, R4);
  alt+followup+followup2 bestehen alle Suiten; alle drei Migrationen idempotent.
CLI-Smoke: URL-Identität, item_or_ability, deterministic_gameplay_projection_v2, gemessener Ueberlauf-Block;
  Drift false/true, R6-Block, Null-Basis-Drift; refresh Evidenz-Negativabbruch; URL-Rotproben (Injection/Userinfo/Port).
MCP: tools/list zeigt patch_history_v1; tools/call liefert Treffer aus brain.patch_history_v1 (state, first_observed_at).
```

TESTNACHWEIS[TW-1]: 70 passed, 5 ignored | Baseline: 0 rot

(70 = 51 deadlock-brain + 11 patch-review + 2 config + 6 MCP; 5 ignored = YT-Integrationen im deadlock-brain-yt-Bin, siehe offene Punkte. Kein bestehender Test abgeschwächt; die falsche patch_2-Erwartung wurde auf die korrekte URL-Identität berichtigt.)

## Betrieb und Installation

1. Migrationen in Reihenfolge anwenden (Schema-Owner): `2026-09-18-patch-evidence.sql`, `-followup.sql`, `-followup2.sql`. Keine PUBLIC-Rechte; Service-Rollen brauchen SELECT auf die Views sowie INSERT/Sequenz auf Evidenz und Review-Runs.
2. Release aus dem eigenen Worktree bauen, installieren, `DEADLOCK_BRAIN_ROOT` auf das installierte Repo setzen (repo_root nutzt es).
3. Timer auf `scripts/ops/patchnotes-sync-v2.sh` umstellen (ersetzt MAX(id)-Vorcheck). Erwartet `DEADLOCK_CENTRAL_DSN` und `DEADLOCK_BRAIN_ROOT`.
4. Quellenrefresh existierender offizieller Quellen: `deadlock-brain pg refresh-official --patch-id <id> --apply` (nach Evidenzmigration), danach `sync-patchnotes --apply`.
5. MCP: unveränderte Registrierung (`mcp/server.py` via venv-Python, stdio). `patch_history_v1` nutzt das Release-Binary; `DEADLOCK_BRAIN_PATCH_REVIEW_BIN` optional überschreibbar. Der übrige Python-Transport bleibt bestehen (dokumentiert, kein Umbau der Alt-Tools).

## Offene Abnahmegrenzen (nicht wegprüfen)

- **Vier DB-gestützte YT-Vertragstests** noch nicht gegen ein passend isoliertes Fixture-Schema ausgeführt; die historische 11941-Claim-Parität bleibt bewusst getrennt und ist ohne den exakten Alt-Snapshot NICHT ausgeführt (nicht durch Abschwächen grün machen; separater deterministischer Query-/Filtervertragstest steht noch aus).
- **Consumer-Revalidierungsprüfung**: bestehende Verbraucher auf `needs_revalidation`/`needs_claim_revalidation` noch nicht vollständig auditiert.
- **Kein Nachweis autonomen Spielverständnisses.** Kontext/Prompt/History/Tests funktionieren; Entwürfe bleiben ungeprüft. Kein Modelllauf ausgeführt. U7 (Bildbelege) und U8 (Video) nicht angebunden; weder JSON noch Storyboard sind ein Video.
- **Nur direkt genannte Helden plus Kernkit sind belegte Detailtiefe**; für allgemeine Systemänderungen bleibt der Namenskatalog eine Grenze. Der reale Lauf ist ein Diagnoselauf, keine Vollabnahme.
- Merge, Produktionsmigration, Deploy und Live-Beweis liegen beim Orchestrator.
