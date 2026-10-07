status: aktiv
Datum: 2026-10-03

# Geprüfter Brain-Patchnotes-Kandidat für Z

Komponente lokal geprüft und im eigenen Brain-Branch gesichert. Paket P insgesamt noch aktiv; Runtime, DevFeed und Echtdaten-/Provider-Nachweise fehlen noch. Kein Einzelmerge, Push, Produktivaufruf oder installierter Unitwechsel.

## Herkunft

Worktree `/home/nathanael/.worktrees/brain-patchnotes-rust-sync`, Branch `feat/brain-patchnotes-rust-sync-20261003`. Vollständiger Commit `1a5b2b33ec9f8c79a073fe67c083c14232289a2b`, Basis `511a347b653beba13c2bf130f4bead7a7196cc2a`. Einzige Produktdatei `rust/crates/brain-feeds/src/bin/brain-patchnotes-ingest.rs`, vor Commit erneut verifizierter SHA256 `d443fe3c76eabac50174cbd4084ab215c2c01ce586ddec149f33b8dd5d87202f`. Keine Manifest- oder fremde Produktdatei geändert.

## Aufruf und Konfiguration

`brain-patchnotes-ingest stage-candidate --config <absoluter-konfigpfad>`

JSON, unbekannte Felder abgelehnt. Erforderlich sind `infisical_config`, `url`, `base_release_id`, `base_release_sha256`, `owner` und `policy`. URL vorgesehen `http://127.0.0.1:8791/v1/brain/patchnotes/v1`; nur literales Loopback, keine Proxies/Redirects, Timeout und streamweises 64-MiB-Limit. `policy` enthält exakt:

```json
{
  "visibility": "public",
  "allowed_scopes": ["<aus-dem-geprüften-Standardvertrag>"],
  "provider_egress_allowed": false,
  "publication_allowed": false,
  "raw_retention_allowed": true
}
```

Der Scope-Platzhalter ist kein Produktivwert. Z bindet den tatsächlichen Standard-/Patchnotesscope und die ausdrücklich freigegebenen Berechtigungen. Keine stillen Scope-/Egressfreigaben. Der Basishash bindet das serialisierte vorhandene CorpusRelease. Z liest die aktuelle Standardbasis unter seinen gemeinsamen Sperren erneut und gibt sie ausdrücklich an den fachlichen Writer.

Infrastruktur ausschließlich bestehender Infisical-/Credential-FD-5-Weg, keine Writer-DSN-Option, Credentialdatei oder ENV-Fallback. Storeendpoint und Rolle sind fest gebunden und geprüft. Benötigte vorhandene Geheimnisse bleiben im nativen Zugang; Werte werden nicht ausgegeben.

## Kandidat und gemeinsame Aktivierung

Vollständigen nichtleeren Feed vor Source-Lease validieren. Checkpoint erst unter Lease lesen; Basishash gegen Store prüfen, fremde Pins erhalten, Batch und Kandidat atomar über bestehenden PgStore committen/publizieren. Zuvor committed, noch nicht aktivierte Source-Pins bleiben bei unverändertem Feed sichtbar. Deterministische Kandidatenidentität, Wiederholung verwendet vorhandenen Kandidaten und dessen Zeit. No-change erzeugt keinen neuen Release.

Ausgabe enthält Basis-/Kandidatenkennungen und SHA256, Wissensversionen, Source-Generation/Revisionspin-Hash, `allowed_changed_sources=["patchnotes-feed"]`, `activation_target="standard"`, `activation_performed=false`, `reader_state="not_checked"`, `base_binding="provided_by_activation_adapter"`. Kandidatenstatus `published`, `reused` oder `not_needed`; bei No-change sind Kandidatenfelder null.

Z aktiviert diesen Kandidaten über den gemeinsamen Adapter und bestehenden Journal-/ConfigWriter-/Restart-/Readinessweg. Nur Ps Standardziel ändern. Qs gekoppelte interne Bindung und unabhängige Docs-/Twitch-/C9-Grants erhalten. `preview --infisical-config ...` und `validate --feed-file ...` bleiben schreibfrei, `apply` bleibt geschlossen.

## Tatsächliche Prüfungen und Grenzen

Geschützter Wrapper Exit 0, beide Hostsperren in richtiger Reihenfolge, frische NonZombie-Probe, maximal zwei Jobs. Check und Clippy `-D warnings` Exit 0, rustfmt und whitespace ohne Befund. 16 Binarytests sowie 3 vorhandene Patchnotes-Librarytests tatsächlich bestanden, 0 failed/ignored; 16 andere Libraryfälle bei gezieltem `patchnotes::`-Filter nicht gelaufen.

TESTNACHWEIS[TW-1]: 19 passed, 0 ignored | Baseline: nicht gemessen, keine Altfehlerbehauptung

Befehle in `/tmp/brain-patchnotes-kandidat-check-20261003.sh`: explizites eigenes Workspace-Manifest und target-dir, `--locked`, Package brain-feeds, Binary brain-patchnotes-ingest, `-j 2`; beide Testläufe `--include-ignored --test-threads=2`. Logs `/tmp/brain-patchnotes-kandidat-20261003-{check,clippy,tests,existing-tests}.log`, Wrapperlog `/tmp/brain-patchnotes-kandidat-wrapper-20261003.log`.

Kein PostgreSQL-Integrationstest, echter Produktionsimport, aktiver Readernachweis oder Aktivierungsaufruf. Diese Grenzen müssen im gemeinsam integrierten Stand geschlossen werden. Der Commit ist ein lokal geprüfter Baustein für Z, kein abgeschlossener Liveauftrag.
