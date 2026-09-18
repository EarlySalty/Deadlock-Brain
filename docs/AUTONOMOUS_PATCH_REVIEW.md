# Eigenständige Patchanalyse und gemeinsame Historie

## Richtung

Das Brain leitet eine Erklärung aus Patchquellen und gespeicherten Spielmechaniken ab. Es liest für diese Analyse keine Creator-Transkripte und keine importierten Agenten-Insights. Das Transkript zu `ZWm7ixeWjbQ` ist ein zurückgehaltener Vergleich, keine vorgeschriebene Wahrheit. Das Ziel ist eine eigene, überprüfbare Erklärung, nicht die Imitation des Videos.

## Implementierte Schnittstellen

`deadlock-brain-patch-review history --query <Text>` fragt ausschließlich die revisionsgesicherte Patchhistorie ab. Andere Assistenten können dieselbe freigegebene Postgres-View `brain.patch_history_v1` lesen. Es wird kein Textmodell benötigt. `--entity` grenzt den exakten Entity-Namen ein, `--known-at` nimmt RFC3339 inklusive Zeitzone, `--limit` ist auf 500 begrenzt. Keine Treffer bedeuten nur „im erfassten Korpus nicht gefunden“. Aliasauflösung und die Anbindung der vorhandenen Bot-MCP-Tools sind noch separate Integrationsschritte.

`deadlock-brain-patch-review review --patch patch_<changelog_posts.id>` stellt einen Kontext zusammen, ohne ein Modell aufzurufen. Der Kontext enthält den gespeicherten Originaltext, alle gespeicherten Patch-Events und die zuletzt vor der Publikation beobachteten Assets-Snapshots. Es werden keine späteren Snapshots als früheres Wissen ausgegeben. Fehlende historische Mechaniken bleiben unbekannt. `--snapshot-limit` steuert ein transparentes Limit; Überläufe führen zu einem Fehler statt stiller Kürzung. Die vollständige Quellen- und Parserabdeckung ist damit noch nicht unabhängig zertifiziert.

Mit `--generate` wird der bereits vorhandene `deadlock_brain_core::ai::AiClient` verwendet. Kein neuer Provider, kein Modellwechsel. Die Antwort muss Ereignis-Hashes, Snapshot-IDs, Bedingungen, Gegenargumente und Prüfschritte nennen. Der Validator prüft Struktur und Referenzexistenz, nicht die Wahrheit der Schlussfolgerung. Das Ergebnis bleibt `draft`, öffentliche Veröffentlichung ist nicht angebunden. `--write` speichert den Kontext und den Entwurf in `brain.patch_review_runs`.

Das Storyboard enthält Kapitel, Erklärung, Bedingungen und Quellen. Es ist kein gerendertes Video. Visuelle Belege werden nicht erfunden; `visual_status` verlangt eine eigene Aufnahme oder eine geprüfte Darstellung. Das fremde Video wurde hier nicht bildlich transkribiert.

## Zeit und Revisionen

Die Migration `scripts/migrations/2026-09-18-patch-evidence.sql` protokolliert Änderungen und Löschungen an `patchnotes.changelog_posts` und `brain.patch_events`. Alte Fassungen bleiben erhalten. Ein Reimport löscht die Nachweishistorie nicht. Inhaltsgleiche Updates erzeugen keine neue inhaltliche Revision. Eine technische Löschung ist kein Beleg für einen Revert im Spiel.

`source_published_at` ist das Quelldatum, kein bewiesener Rollout-Zeitpunkt. `observed_at` ist der Zeitpunkt, zu dem die Revision protokolliert wurde. Der Baseline-Import behauptet nicht, frühere Erstbeobachtungszeiten wiederherstellen zu können. Historische Fragen müssen `observation_kind=baseline` und `earlier_observation_unknown` berücksichtigen. Die bisher überschriebenen `created_at`-Werte lassen sich nicht nachträglich zuverlässig rekonstruieren. Neue Updates erhalten den vorhandenen `created_at`-Wert; die Knowledge-Event-Materialisierung erhält getrennte Zeitsemantik.

Eine Revision desselben Quellpatches markiert zugehörige gespeicherte Entwürfe `needs_revalidation`. Auswirkungen späterer anderer Patches auf Themen, Regeln und Builds sind noch nicht als allgemeiner Abhängigkeitsgraph implementiert. Die Berichte sind patchbezogen und dürfen nicht als dauerhaft aktuelle Empfehlungen verwendet werden.

## Untertitel und importierte Insights

Der vorhandene Caption-Aufruf speichert jetzt Roh-JSON, vollständige Zeitsegmente, Wort-Offsets und getrennte Text-/Rohdaten-Hashes atomar. Derselbe Text mit anderer Zeitzuordnung erzeugt eine neue Evidenzfassung. Fehlende Zeitwerte bleiben `null`. `brain.youtube_caption_segments_v1` liefert die zum aktuellen Transkript passenden Segmente. Der Downloader bleibt die vorhandene externe Abhängigkeit `yt-dlp`; Parser und Persistenz sind Rust. Es startet kein dauerhafter STT-Dienst.

Bei veränderter Evidenz wird `needs_claim_revalidation` gesetzt. Der alte externe `transcript-claims prepare/ingest`-Workflow wurde nicht zu einer autonomen Verifikationsschleife umgebaut. Der neue Patchanalysepfad verwendet ihn nicht. Vor einer erneuten Freigabe alter Video-Claims muss diese Markierung im jeweiligen Consumer berücksichtigt werden.

Der Insight-Import stuft weder hohe Confidence noch ein angefordertes Trust-Label als historische Wahrheit ein. Importierte Ableitungen bleiben `curated_agent_inference` und standardmäßig `needs_review`. Datumsangaben beschränken den Patchbezug. Event-IDs werden vor dem Schreiben aufgelöst; URLs werden nicht mehr anhand ihrer Arrayposition mit Ereignissen verheiratet. Nur die tatsächlich importierten Insights werden materialisiert. Bereits früher hochgestufte Altbestände werden nicht pauschal rückwirkend verändert.

## Betrieb und Prüfung

Migration mit dem zuständigen Schema-Owner anwenden, bevor der neue Caption-Importer läuft. Die neuen Tabellen und Views erhalten keinen PUBLIC-Zugriff. Den bereits autorisierten Service-Rollen müssen passende SELECT-Rechte für die Views sowie INSERT-/Sequenzrechte für Evidenz und Review-Runs erteilt werden. Die Trigger laufen mit festem `search_path`, fest geprüften Quelltabellen und ohne dynamisches SQL. Lock- und Statement-Timeouts begrenzen die Migration. Keine automatische Installation in eine unbekannte Produktionsdatenbank.

```sh
cargo test --manifest-path rust/Cargo.toml --locked --package deadlock-brain-yt --bin deadlock-brain-yt
cargo test --manifest-path rust/Cargo.toml --locked --package deadlock-brain --bin deadlock-brain-patch-review
cargo test --manifest-path rust/Cargo.toml --locked --package deadlock-brain --bin deadlock-brain pg_insights::tests
cargo clippy --manifest-path rust/Cargo.toml --locked --package deadlock-brain --package deadlock-brain-yt
```

Der neue Workflow prüft Rust und eine isolierte Postgres-16-Testdatenbank. Die Caption-Datenbankintegration ist als zusätzlicher Scratch-DB-Test vorhanden und bleibt ohne vollständiges Brain-Testschema ignoriert. Die SQL-Fixture ist ausschließlich für eine leere Testdatenbank vorgesehen. Sie darf nicht in Produktion ausgeführt werden.

`tests/patch-understanding/heresy-comparison.json` enthält zehn genaue Textstellen mit Zeichenpositionen. Es weist keine Videozeiten zu und erklärt die Creator-Urteile nicht zu Sollantworten. Es ist noch kein gemessener Nachweis autonomer Spielkompetenz: Dafür müssen Primärdatenläufe, unabhängige fachliche Bewertungen und Patchwechsel-Tests folgen.
