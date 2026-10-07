# Runde 19: frischer Fixer durch Projektroot-Schutzgrenze blockiert

Stand: 07.10.2026. Bezug: gemeinsame Runde 18 auf Kandidat `501d3725e691c713f4b04468fd9d6b77977ae91c`, tatsächlicher neuer Bestands-ID-BLOCK mit unverändert Claude Opus 5.5. Die Entscheidung `ENTSCHEIDUNG-I-PATCH-ORIGINAL.md` erlaubt frische Fixer, aber ausdrücklich keine Umgehung einer Schutzablehnung.

## Tatsächliche Rückgabe des Fixers 13

```text
BLOCKER: Der Zugriff auf `/home/nathanael/.worktrees/brain-e-deadlock-api/rust/crates/deadlock-brain/src/pg_patchnotes.rs` wurde durch die Werkzeug-Schutzgrenze verweigert: außerhalb des zugelassenen Projektroots. Kein anderer Zugriffsweg versucht.

Ausgang `d3d3c5b68bea07a3e11c09572b200bbefd7b55cf` und Featurebranch bestätigt, Graphify abgefragt. Keine Änderungen oder Commits; Hauptsession-Artefakte unverändert. Fix, Scratch-PG-Probe und Selbst-Gate bleiben offen.
```

Das ist kein Produkturteil und kein weiterer inhaltlicher Gate-BLOCK. Fixer beendet; keine Hintergrundkinder oder neue Delegation gestartet. Hauptsession prüfte unmittelbar Status und log -1: HEAD weiterhin d3d3c5b6, ausschließlich eigene Aufgabenakten verändert oder untracked, keine Produktänderung. Kombinierter Kandidat 501 bleibt auf eigenem Remote-Branch gesichert, nicht nach main.

## Urteil und fachliche Fortsetzung

E-Fertigstellung ist derzeit durch zwei getrennte Dinge blockiert: tatsächlicher neuer Lookupfehler aus Runde 18 und regulärer Zugriff des vorgeschriebenen frischen Fixers auf seinen vorhandenen Arbeitsbaum. Die Kernlösung bleibt eine konsistente bestehende Patchidentität für nachweislich identische URL-Varianten, mit echten Lookupregressionen. Unterschiedliche Steam-Ereignis-/Announcement-GIDs nicht vermischen; die beiden NITs bleiben offen.

Kein Ersatzfix durch Hauptsession, anderer Worker, Toolwechsel, Wrapper oder Hook-/Berechtigungsänderung. G/K-Prüfsperren ebenso nicht übernommen. Für Fortsetzung den regulären frischen Fixerkontext mit dem bestehenden E-Worktree als zulässigem Projektroot bereitstellen, statt den Schutz zu umgehen. Qualifizierte Rückgabe über die zugewiesene AN_HAUPT-I.md; zentrale Akte bleibt beim Delegator, keine Sessionkontakte oder neuen Threads.

Kein Gesamt-ALLOW, vollständiger E-Main-Push, Releasebuild/install, eigener Neustart, vollständiger produktiver Import, Live-Receiptbeweis oder analytics_runtime-Übergabe. F/G-Anschluss, Warden-Publish/hero_build_id, Cleanup und Self-Settle bleiben offen. Historische Beleg-/Schemapinstufen auf main unverändert erhalten.
