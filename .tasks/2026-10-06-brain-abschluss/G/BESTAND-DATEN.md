# G: Bestandsbefund Daten und Abbau

status: erledigt, 07.10.2026. Recherche abgeschlossen, keine Löschung oder Betriebsänderung.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/brain-storage/src/asset_mirror.rs (Paket E) | Anknüpfung: Es gemeinsamen Spiegelleser, vorhandene Reasoner-Parser, Profile und Patchgeschichte

## Verbindlicher Leseschnitt

Hauptsteuerung 05:40: aktuelle Spielwerte über `brain_storage::asset_mirror`, nicht `entity_snapshots`. Nach Hold-Ende Integrationsreihenfolge E, F, G.

Der Rechercheagent fand in Es Worktree `/home/nathanael/.worktrees/brain-e-deadlock-api/rust/crates/brain-storage/src/asset_mirror.rs`:

- `latest_mirrored_client_version`.
- `load_mirrored_assets(pool, client_version, kind, language)`.
- Arten `items`, `heroes`, `heroes_all`; Sprachen `english`, `german`; Original-JSON, lokaler Zugriff, Fehler ohne HTTP-Fallback.

Belegter WIP-Vertrag nutzt `brain.source_runs` und `brain.source_documents`. Auswahl: `summary.client_version`, `mirror_complete`, `endpoints["<kind>/<language>"].source_document_id`. Dokumentbindung: `metadata.adapter.{client_version,kind,language}`. Originaldaten: `metadata.contract.data.payload.value`.

Der abschließende E-Commit und dessen Prüfnachweise sind noch zu übernehmen. Es Worktree-`AN_HAUPT-E.md` ist neuer als die gemeinsame Akte, die noch eine überholte Matchimport-Empfehlung enthält. Maßgeblich bleibt die neueste Hauptsteuerung.

`entity_snapshots` dedupliziert identische Payloads nach Quelle, Typ, ID und Hash (`dbrain-sources/src/store.rs:244`), liefert keine vollständige Versionshistorie. `hero_catalog` wird überschrieben und ist keine aktive versionsgebundene Vergleichspopulation (`dbrain-builds/src/sync.rs:239`). G nutzt `heroes` derselben vollständigen Spiegelversion für Ränge, `heroes_all` für belegte historische/inaktive Entitäten.

## Bestehende Modellierung und Patchgeschichte

`dbrain-reasoner/src/data.rs` hat Skalierungs-, Ability-, Waffen- und Heldenparser (`:315`, `:340`, `:513`, `:573`). SQL-Loader ab `:649`, `:667`, `:736`, `:932` wählen bislang Abrufzeit statt Clientversion; Itemloader mischt `deadlock_data/item_card` hinzu. Bestehende Parser wiederverwenden, aktuelle Auswahl durch Es gemeinsamen Leser ersetzen. Keine neuen Rohdatenkopien oder Importer.

`brain-storage/src/entity_profile.rs:467` besitzt strukturierte Profile mit Freigabe-, Fakt-, Herkunfts- und Projektionsprüfung. Seine Korpusbindung muss durch die freigegebene Rohdaten-Rechenansicht für Spielwissen ergänzt beziehungsweise ersetzt werden.

Patchbestand behalten: Originalparser `deadlock-brain/src/pg_patchnotes.rs`, View `brain.patch_changes` aus `dbrain-sources/src/store.rs:364`, strukturierter Leser `brain-storage/src/entity_profile.rs:587`. Clientversion und Balancepatch-Datum sind noch nicht abschließend zugeordnet. Numerische Zunahme ist nicht automatisch Buff, numerische Abnahme nicht automatisch Nerf. Keine neue Patchparser-Pipeline.

## Geordneter Abbau

| Pfad | Eigentümer und Voraussetzung |
| --- | --- |
| `brain-maintenance/src/entity_profile_render.rs`, `integration/entity_profiles.rs` | G-Spielwissensansicht und echte Antwortparität zuerst; As Übergangsfreischaltung bis dahin erhalten. Freitext-/Serverwissen bleibt. |
| `dbrain-sources/src/google_sheet.rs`, `scripts/run_sheet_sync_with_infisical.sh` | Aktuelle Rechen-/Shop-/Boon-Werte aus E und gemeinsamer Mechanik belegen. Wrapper startet zusätzlich Lern-/Enrichjobs, diese nicht versehentlich entfernen. |
| `dbrain-sources/src/deadlock_api.rs`, `statlocker.rs`, `brain-feeds/src/bin/brain-match-ingest.rs` | E besitzt Rohmatch-, Spielerhistory-, Demo- und Zusatzfeed-Abbau. G speichert keine Matches. |
| `dbrain-learn/src/match_demo_learning.rs`, `dbrain-reasoner/src/families/data.rs` | Vor Tabellenabbau müssen F und E verpflichtende Population-/Rohmatchabhängigkeiten entkoppeln. |
| Doppelte aktuelle Spielwertauswahl und Parserwege | Nach Gleichstand die gemeinsame Quelle und bestehenden Parser behalten; historische Quellen für freie Aussagen nicht pauschal löschen. |

A-Profil-, A-Sheet- und Spielstilartefakte sind laut Recherche bereits Vorfahren der G-Basis, Ancestor-Exit jeweils 0. Kein erneuter Import oder Cleanup fremder Branches. Abbau als eigene geprüfte Commits nach belegtem Gleichstand.

## Herkunft und Grenzen

Rechercheagent `a45d24611b1d8a737`, Workflow `wf_4e39a761-389`, Abschluss bestätigt. Vollbericht `wyqxc1iva.output`, `result[2]`. Worker konnte die gemeinsame Berichtdatei wegen Worktree-Isolation nicht schreiben; Bereichsführung übernahm den Befund hier. Keine eigene Produkt-, DB-, Git- oder Runtimeänderung des Rechercheagenten. Der damalige Hinweis „Sheetmodell fehlt“ ist durch den inzwischen abgeschlossenen Sheetworkflow überholt.
