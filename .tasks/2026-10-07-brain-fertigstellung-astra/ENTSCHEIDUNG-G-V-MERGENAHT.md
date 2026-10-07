[Orchestrator]
# G-V: gesicherte alte Providerinput-Stände geordnet integrieren

Antwort auf G/G-V-MERGE-NAHT.md, Wache 33. Keine neue Funktion und keine Übernahme von Ks Orts-/Privatfixarbeit.

G darf im eigenen erhaltenen Merge von HEAD 253d383eab7a2f018ee1c834370140bf4a55e844 und MERGE_HEAD b7289d115c0b64016fcfe6cbfe3c797cf9fa76e2 ausschließlich die vorhandene provider_input.rs-Mergenaht aus diesen beiden gesicherten Seiten auflösen. Beide bisherigen Verträge und Sicherheitsgrenzen erhalten, tatsächliche kombinierte Payload-/Budget-/Providerprüfungen und regulärer Gate auf dem zusammengeführten Stand. Keine Kopie fremden K-WIPs, keine neue Ortsbindung, Privatprojektion oder Query-Literalimplementierung. Kein Reset/Abort/Stash des erhaltenen Mergeindex.

Dies ist eine begrenzte Integrationsausnahme von der vollständigen Dateisperre, keine zweite Implementierung des K-Deltas. G liefert die gesicherte Mergeauflösung als konsumierbaren Commit mit tatsächlichen Prüfbelegen. K übernimmt diese geprüfte Basis später geordnet, bevor K seine eigene Ortsänderung an dieser Naht endgültig integriert. Neuer K-Funktionsumfang bleibt exklusiv K. Falls ein tatsächlicher über die beiden alten Seiten hinausgehender Konflikt entsteht, konkrete semantische Stelle melden, nicht beide Änderungen parallel neu bauen.

Ks dringender Privatfix nach ENTSCHEIDUNG-K-PRIVATFIX-0015.md läuft weiter zuerst. Die zuvor verlangte zusätzliche private Mitleseprojektion/Threadsicht ist für diesen Fix ausdrücklich kein Blocker mehr. Eine gemeinsame Integration muss die neueste Nutzerentscheidung bewahren, privat kein Discord-Nachrichtenlesen anderer Personen für diese Anfrage; öffentliche Anfragen bleiben unverändert.

Andere G-V-/Tool-/Deadline-/Analyticsbereiche unverändert. G führt denselben bestehenden nativen Ausführer weiter, kein neuer Thread oder Doppelwriter. Auftraggeber 481426fe-b477-42b3-91c6-901811fcba1d; K wird über diese begrenzte Ausnahme informiert.
