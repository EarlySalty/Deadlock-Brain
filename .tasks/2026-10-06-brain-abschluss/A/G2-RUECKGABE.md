# A-G2: enge vorhandene Twitch-Spielwissensfreigabe

Read-only Rückgabe, Messfenster 07.10.2026 01:11 bis 01:13 UTC, Source 8d61a949c9856a69543747b0e59dbfb5bbcbe440. Keine Konfig-, DB-, Secret-, Dienst- oder Repositoryänderung. Diese Rückgabe ist keine Freigabe für eine eigene Operation im aktuellen Releasefenster.

## Urteil

Genau ein bestehendes boolesches Feld fehlt. Am vorhandenen Credential twitch-bot/Kanal twitch sind Scopes exakt bot.public und Egress exakt public, kein eigener Releasepin. entity_profile_model_context fehlt und ergibt false. Maintenancebindung ist vorhanden. Die einzige nötige Consumeränderung ist entity_profile_model_context=true an genau diesem Credential, alle anderen Felder erhalten. Keine Sourcepolicy erweitern, keine Rohchatfreigabe oder neuer Reader.

Sourcebelege: brain-serve/src/config.rs:532, brain-storage/src/local_pg_reader.rs:498 und :739. Runtimeconfig /home/nathanael/.config/deadlock-brain/brain-serve.json:11, Maintenancebindung :70. Nur Secretreferenz festgestellt, kein Secretwert ausgegeben.

## Die Freigabe ersetzt fehlende Veröffentlichung nicht

Messung: null git-game-facts-derived-Heads, null Köpfe mit brain.entity_projection.contract, null veröffentlichte Releases mit abgeleiteter Profilquelle. Aktives Release maintenance-fe0fbb3c1a38f9c92777b26494a36b36786192858b3fa276959cdcbb446210d0 hat 62 Dokumentquellen, keine der registrierten Spielquellen oder abgeleitete Profile. readyz bestätigt diesen Stand; abweichende Statusdatei ist kein Aktivierungsbeweis.

Interne Originale: 64.342 Fakten, 18.581 Patchänderungen, letzte gemessene Änderung 29.09.2026. Kein Beleg für aktuelle öffentliche Verfügbarkeit. Bestehende Originalquellen bleiben intern und ohne Publikations-/Providerfreigabe. Öffentliche Ableitung über vorhandenen verifizierten Pfad, Sourcepolicy in brain-storage/src/entity_derivation.rs:255. Keine Produktionszustände von Hand reparieren.

## Vorhandene Mechanik, ausschließlich für den einzigen Live-Agent

Vorhandenes CLI: /opt/deadlock-brain/maintenance-current/brain-maintain --config /etc/deadlock-brain/maintenance-runtime.json write-serve-config --input <frisch geschützte Proposaldatei> --expected-sha256 <frisch gemessener Servehash>. ConfigWriter validiert, UID 1000, gemeinsame Sperre und Byte-/SHA-CAS, atomarer 0600-Ersatz samt fsync. Bei Konflikt frisch lesen und neu erzeugen. Kein alter Snapshot-Hash als Ausführungswert.

ConfigWriter in brain-maintenance/src/integration/config_writer.rs:118 startet keinen Dienst neu. Consumergrants werden beim normalen Serve-Neustart geladen. register-config registriert Wartungsquellen, keine Twitch-Credentials. Keine neue Registrierungsroute nötig.

Normale Standard/entity-profiles-Aktivierung verschiebt die gekoppelten Docs-/Discord-/second-brain-/Operatorpins mit. Das ist im bestehenden activation.rs:89 explizit; nicht als vollständige Pin-Erhaltung beschreiben. Unabhängige Pins und C9 bleiben entsprechend dem vorhandenen Vertrag erhalten. Das boolesche Credentialupdate allein erhält sämtliche Pins. Ein separat fest gepinntes Twitch-Profilrelease wäre erst bei tatsächlich vorhandener geprüfter Veröffentlichung möglich; im Messstand existiert keines.

A führt weder Writer noch Restart/Tick aus. Alles bleibt beim exklusiven Live-Agent der freigegebenen fde910f6-Recovery. Belege /tmp/brain-a-twitch-grant-proof-20261007/.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/brain-maintenance/src/integration/config_writer.rs:118 | Anknüpfung: bestehender ConfigWriter/CAS und vorhandene Profilpublikation
WIRKUNGSPRUEFUNG[WP-1]: 2 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft
