# Runde 18: gemeinsamer korrigierter E-Kandidat bleibt BLOCK

Stand: 07.10.2026. Kandidat `501d3725e691c713f4b04468fd9d6b77977ae91c`, Tree `4cca98fe791105203b8951a4f24c6c4abaf43cab`, Basis `ca4d877f13042c9a7a7023e54f6bf2c688b69ac4`. Gemeinsamer Gate mit unverändert `claude-opus-5-5`, Exit 1. Nach Deny zuerst Status und log -1 geprüft: eigene Quelle sauber, HEAD unverändert. Kein Main-Push oder Deploy.

Prüfungen am tatsächlichen Kandidaten: Format und striktes Clippy einschließlich brain-serve Exit 0; vollständige Suite 558 passed, 0 failed, 25 ignored. Zusätzlich ausdrücklich echte Bestands-ID-Scratchprobe 1 passed, 0 failed, 0 ignored, 116 filtered, Exit 0. Fehlende Scratch-DSN im Erstlauf offen dokumentiert, anschließend eigene isolierte Unixsocket-Instanz eingerichtet und nach der Probe beendet. Einzelheiten und Befehle in `PRUEFUNG-KANDIDAT-501.md`.

## Originalurteil

Log `/tmp/brain-i-e-patch-candidate-gate-opus55.log`:

```text
BLOCK: The patch import can assign an existing patch a new ID.

1. `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:149` | **BLOCKING:** The existing-ID lookup compares only the fragment-free URL and the original feed link. `post_row` then strips the query for its generated ID. For an existing `changelog_posts` URL without a query, a feed link with `?l=english#notes` matches neither lookup value; if no matching source document exists, the import creates a negative ID instead of retaining the existing patch ID. The same lookup gap covers trailing-slash variants and the accepted Steam store/community URLs for the same event.
2. `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:45` | **NIT:** One historical Steam or forum original that fails to load aborts the entire sync, so the scheduled build-data step does not run. Check this with a replay of a real feed snapshot containing one unavailable older original.
3. `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:183` | **NIT:** A harmless image link makes otherwise complete original text fail the completeness check. Check one official full-text patch containing an image link to establish the production impact.

WIRKUNGSPRUEFUNG[WP-1]: 3 Befunde | Zwillingssuche: keine Fundstelle | Fremddienst-Pfade: 6/6 geprüft
```

## Berechtigter Kern und begrenzte Folgerunde

Graphify zuerst gefragt; neue private Hilfen noch nicht im vorhandenen Graph gefunden. Tatsächlichen Kandidaten nachgelesen: `load_post_row` vergleicht fragmentfreie Anfrage-URL und unveränderten Feedlink, während `post_row` zusätzlich die Query entfernt. Eine bestehende queryfreie URL passt daher nicht zur Anfrage `?l=english#notes`. Die vorhandene Scratchprobe deckt zwar einen Querylink ab, speichert für diesen Fall jedoch die queryhaltige Anfrage-URL und prüft den vom Gate benannten queryfreien Bestand nicht. Das bestehende ALLOW des begrenzten Fixes entkräftet diesen neuen Kern nicht.

Frischer nativer Fixer 13 ausschließlich für Bestandsidentität im vorhandenen Patch-/URL-/Lookupweg. Query-, Slash- und tatsächlich gleiche Ereignisidentitäten empirisch über den echten vorhandenen Lookup prüfen. Unterschiedliche Steam-Ereignis- und Announcement-GIDs bleiben getrennt; keine bloß angenommene Gleichheit. Kein pauschaler Fehlerfang, keine Lockerung der Herkunfts-/Netzgrenzen, keine G/K-Arbeit und kein NIT-Scope-Zuwachs. Entscheidung `ENTSCHEIDUNG-I-PATCH-ORIGINAL.md` erlaubt diese frische Folgerunde; bisher ein begrenzter Fixer-BLOCK und dieser gemeinsame BLOCK in der neuen Fortsetzung. Spätestens nach fünf erfolglosen Runden qualifizierte Rückgabe.
