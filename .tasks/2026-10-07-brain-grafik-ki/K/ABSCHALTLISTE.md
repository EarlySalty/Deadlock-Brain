# K: Altwege und Abschaltgrenze

Bestand am eigenen Bots-K-Stand4af3776e und Twitch-K-Standf2490f8b nach Graphify gelesen. Diese Liste dokumentiert offene Ersatzprüfungen, keine Abschaltfreigabe. Kein Altweg gelöscht oder deaktiviert.

| Einstieg | Tatsächlicher Bestand | Zentraler Ersatz | Abschaltbeweis |
| --- | --- | --- | --- |
| Discord /brain, !brain, Erwähnung | modglue::BrainHandler und dl-brain, bestehender /v1/answer-Client | öffentlicher Consumer mit privatem Eingangs-/Zustellguard und echtem Autor auf4af3776e | lokale Prüfung/Gate ALLOW, echte Kanalprobe fehlt |
| Discord DM/private ungeklärte Kanäle am Brainhandler | privater Eingang vor Brainconsumer gesperrt, kurzer Hinweis | tatsächlich lokaler freigegebener zentraler Providervertrag fehlt | keine private Antwortfähigkeit oder echte DM-Abnahme behauptet |
| FAQ und Ticket-Autohilfe | main.rs:1135 über shared_answers; faq.rs:846 lookup_with_context und:1354 handle_ticket_message | noch nicht gegen den echten zentralen Vertrag migriert | keine live belegte Parität, kein Abschalten |
| Concierge/persönliche Hilfe | main.rs:1147 über shared_answers und bestehenden Conciergeprovider | dieselbe Brain-Persona vorgesehen, privater Provider-/Datenvertrag offen | keine zweite Persona oder menschliche Vermittlung, kein Abschalten |
| Passive Hilfe | main.rs:1785, passive_help.rs:76-79 über AnswerEngine::answer_with_context_and_style, CommunityAndGame | noch nicht zentral mit belegtem zulässigem Kontext und Veröffentlichungszweck angeschlossen | keine sichere private Freigabe oder live belegte Parität |

Twitchchat verwendet schon den bestehenden Typed-Consumer. f2490f8b ergänzt ausschließlich ehrlichen Ausfall und Coachingprojektion nach allgemeinem Linkfilter, kein Anschluss-Scheinfix. `tb-knowledge/src/brain.rs:111` erlaubt arbitrary Kontext; private Nutzung dieses Ports ist nicht freigegeben. Echte Coaching-/Ausfall-/Spielkanalproben fehlen.

Die shared_answers-Konstruktion steht in Bots-main.rs:1046. FAQ, Concierge und passive Hilfe sind drei tatsächliche Attachments, nicht drei gebaute neue Engines. Das Quelleninventar belegt keine Datenschutzfreigabe des aktiven Laufzeitproviders. Qs beauftragte tatsächliche Lokalproviderinventur wird nicht dupliziert. K liest keine privaten Originalfragen oder Laufzeit-Secrets und sendet keine privaten Proben.

Abschalten erst nach zulässiger zentraler Daten-/Providerbindung, echter beobachteter Antwortparität, Zustell-/Rechteprüfung und bestätigtem Ersatz. Modellwahl/Timeouts nicht eigenmächtig ändern. Der eng erlaubte eigene Invite-Status ist ein anderer begrenzter Datenfall: ausschließlich eigene Enum-/Zeitprojektion nach interner Identitäts-/Rechteprüfung, keine Rohfrage, IDs oder Fremddaten zum externen Modell. Persönlicher Titel bleibt im bestehenden Feature und bekommt keinen zweiten Generator.
