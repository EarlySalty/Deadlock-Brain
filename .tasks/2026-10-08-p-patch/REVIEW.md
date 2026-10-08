# P: Befunde und regulärer Gate

1. Bestätigt: first_post_html nimmt den ersten sichtbaren Beitrag ohne Originalbindung. API-Import akzeptiert bislang Folgeseiten und kann vorhandene Ereignisse ersetzen. Bindung wird vor dem vorhandenen Reader geprüft.
2. Bestätigt: Kosmetikveto prüft die ganze Klausel einschließlich gebundenem Namen Metal Skin. Prüfung muss auf den Änderungsteil begrenzt werden; kosmetische Änderungen am selben Item bleiben ausgeschlossen.
3. Im erhaltenen P-Stand bereits korrigiert: resolve_api_source ruft die Originalquelle immer ab, Feedauszüge dienen nicht als Volltextfallback. Vorhandene Regression unlinked_feed_teaser_never_replaces_original_events wird beibehalten.
4. Bestätigt: section_heading erkennt kurze narrative Änderungen als Überschrift, expand_inline_bullets kann Sternchenzeilen als Text mit Sternchen normalisieren. Gameplayprojektion muss den vorhandenen Reader verwenden und kurze echte Änderungen vor der Überschriftenerkennung bewerten.

Noch kein eigenes Gateurteil. Das historische ALLOW wird nicht wiederverwendet. Korrigierter P-Stand: 45 Patchtests bestanden, 1 DB-Scratchtest ignoriert; striktes Clippy und Formatcheck Exit 0. Lieferschnitt gegen Main benötigt den vorhandenen Readerexport first_post_html aus dbrain-sources/src/forum.rs. Die Datei wurde nicht verändert, da außerhalb P-Eigentum. BERICHT.md nennt den konkreten Freigabepunkt.
