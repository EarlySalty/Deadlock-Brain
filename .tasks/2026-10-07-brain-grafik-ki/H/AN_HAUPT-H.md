# H an Hauptorchestrator

status: erledigt, 07.10.2026

## Übergabe

H gebaut und regulär reviewt, auf origin gesichert. Nicht gemergt und nicht live.

Code-SHA: `26859fda4b5e77a29b3af4cea0411d304a04eb2f`
Branch: `feat/brain-h-grafik-20261007`
Worktree: `/home/nathanael/.worktrees/brain-h-grafik-20261007`
Der Code-SHA wurde auf origin per ls-remote bestätigt. Ein anschließender Dokumentationscommit sichert die vollständige Übergabeakte, finale PNGs, Statusereignisse und Prüflogs. Er verändert keine Rust-Datei. Der aktuelle Branch-HEAD enthält deshalb zusätzlich diese Nachweise; der hier genannte Code-SHA bleibt die Bindung für Compiler, Tests, native Sichtprüfung und den Code-Gate. Branch und Worktree bleiben für K erhalten.

Geliefert: fester Rust-Renderer `render_hero_compare(&HeroCompareInput) -> Result<RenderedHeroCompare>` mit HTML und SVG aus derselben gebundenen Struktur. Genau zwei öffentliche Helden, eine Kennzahl, vollständige gleichartige Boonreihe, Ergebnis-/Versions-/Bedingungs-/Quellenbindung. Fehlende, ungeklärte, private oder gemischte Daten werden abgewiesen. Keine Spielrechnung, Datei-/Netzoperation oder freie Modell-Markups im Renderer. K-eigene Manifeste, Registrierungen, API, Speicherung, Site und Bots unverändert.

## Nachweise

- H/PRUEFUNGEN.md: +1.97.1, Compiler, Clippy -D warnings und Format grün. 12 neue Strukturtests und 22 vorhandene HTML-/Profiltests bestanden, 0 ignored. Vollständige 36-Punkt-Reihe erhalten. Native Cargo-Zwischenablage und vorhandener sccache, keine Target-Kopien.
- H/REVIEW.md: regulärer `gate_hook.py --review` am exakten SHA, gpt-6.1-sol ALLOW, zwei nicht blockierende NIT. Kein Modelloverride oder Übersteuern.
- H/SICHT-FINAL.md und H/sicht/nachfix/: native unabhängige Chrome-Sichtprüfung, Desktop 1440x1000 und Mobil 390x844, fünf finale PNGs einzeln gelesen. Vier Erstpassbefunde behoben. Isolierte Darstellung fertig J, weitere Fixes in dieser Sichtprüfung N. Ausschließlich synthetische Layoutprüfung, kein echter G-/Livebeweis. T3-Previewhost fehlte konkret; lokaler Chrome ohne neue Installation benutzt.
- H/BESTAND-ANSCHLUSS.md und REGISTER.md: tatsächlicher Workflow wf_97de2070-ec3, ein nativer high-Agent abgeschlossen; sämtliche eigenen Bau-/Sichtagenten high, höchstens drei gleichzeitig. Keine fremden Sessions kontaktiert.

TESTNACHWEIS[TW-1]: 34 passed, 0 ignored | Baseline: 0 rot
MERGEPROTOKOLL[MS-1]: 5 Git-Schritte einzeln | Anläufe: 1 | Gate: gpt-6.1-sol ALLOW, 2 NIT, kein Mainmerge

## Anschluss und Grenzen

K ergänzt `pub mod hero_compare_render;` in brain-maintenance/src/lib.rs und den in ANSCHLUSS.md beschriebenen Adapter aus geprüftem G-Ergebnis. Gs neue Ergebnistypen/Rechnung waren uncommittetes WIP; nichts davon kopiert. Die echte historische Version-6759-Probe ist nicht zur Veröffentlichung freigegeben und ihr historischer G-Lauf war rot. Deshalb keine Grafik mit diesen Zahlen als freigegebene Ausgabe oder Livebeweis erzeugt.

K besitzt Ergebnis-ID/Pfad, Speicherung, Veröffentlichung, aktuellen Rechte-/Quellen-/Versions-/Widerrufsschutz auch beim Abruf und Cache, feste Rust-Site-Allowlist sowie Discord-Anhang/Twitch-Link. Rechenregelrevision und sämtliche angewandten Szenariowerte müssen in den gebundenen Bedingungen erhalten bleiben. Kein neuer Profilpublisher oder Dauerdienst.

Zwei nicht blockierende Gate-Randfälle bleiben offen: akzeptierte 40-Zeichen-Namen können im SVG beschnitten werden; U+FFFE/U+FFFF werden noch nicht abgewiesen und können eigenständiges XML ungültig machen. Vorschlag: vor echter integrierter Veröffentlichung klein absichern, siehe REVIEW.md. Die erfolgreiche synthetische Sichtprüfung deckt diese Randfälle nicht ab.

Keine neue Produktentscheidung nötig. Eigene Vorschauen und Browser beendet; finale Vorschau nach Sichtprüfung am Hintergrundlimit gestoppt, Port danach frei. Keine produktiven Nachrichten oder API-Effekte, kein Mainmerge/Deploy/Cleanup des Featureworktrees.

Nächster Schritt: K öffnet H/ANSCHLUSS.md für die Integration.
