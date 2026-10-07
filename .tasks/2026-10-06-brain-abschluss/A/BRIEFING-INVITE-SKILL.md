# A-E3/E4: Invite-Status als Skill des gemeinsamen Brains

## Ziel und verbindlicher Vertrag

`VON_HAUPT.md`, `EIN-BRAIN.md` und `INVITE-VERTRAG.md` gelten, insbesondere die nachträgliche ausdrückliche Freigabe: nur eigener Status als Enum plus Zeitpunkt darf über den bestehenden Provider laufen. Die ältere lokale-Provider-Forderung der Inventur A-E1/E2 ist für diese exakt begrenzte Projektion erledigt. Kein Modell-/Timeoutwechsel, keine zweite Antwort-/Statusroute. B entfernt die fehlerhafte verspätete Lounge-Antwort unabhängig von A; gewollte funktionierende andere Antworten bleiben bis zum live belegten Ersatz erhalten.

Vorhandene Bausteine sind belegt: `dl-brain::answer_for_discord` und Brain `/v1/answer`, vertrauenswürdige Requester-Identität, `brain-serve/src/discord_live.rs` als requestgebundener Sourceadapter, bestehende authentifizierte MCP-Lesegrenze im dl-bot, bestehender zentraler PG-Pool. Brain hat eigene DB auf 5446; Invite-Daten liegen in zentralem `deadlock` auf 5432. Kein zentraler breiter Schreibaccount für Brain. Kein neuer Connector/Secretstore, bestehende Discord-Lesegrenze und Zugangslader verwenden.

### Gemeinsame lesende Schnittstelle

In der vorhandenen authentifizierten MCP-Grenze heißt das neue reine Lesetool `self_invite_status`. Argumente enthalten keinen frei wählbaren Nutzer/Steamcode; Subjekt ausschließlich aus den bereits vertraut geprüften Requester-Headern wie beim bestehenden Discord-Livepfad. Vor Umsetzung beide bestehenden Header-/Authverträge konkret nachlesen und exakt wiederverwenden, nicht anhand dieses Briefings neue ungeprüfte Header erfinden.

Ergebnis ausschließlich `{ "status": <enum>, "at": <RFC3339 oder null> }`. Gemeinsames Enum: `sent`, `pending`, `friendship_missing`, `already_has_game`, `error`, `unknown`, `unavailable`. `at` ist der belegte Ereignis-/Aufzeichnungszeitpunkt, kein erfundener Einladungszeitpunkt. Unknown-Statuswert/zusätzliche Felder strikt ablehnen. Keine Identität, Namen, IDs, Rohzeilen, Freitextfehler oder Daten Dritter im Ergebnis, Modellpayload, öffentlichen Quellenlabel oder Logs. Interne Ownership-/Requestscopebelege verbleiben innerhalb der vorhandenen Vertrauensgrenze. Unbekannte/fehlende Identität darf nicht in anonymen Member-Fallback oder freien Zielnutzerlookup laufen.

Quelle: bestehende `steam.beta_invite_audit`, `steam.invite_requests`, `steam.steam_tasks`, ggf. eng projizierte GC-Codes aus `steam.bot_event_log`, verifizierte kanonische Links. Bereits konkret gelesen in Steam `steam-persistence/src/betainvite.rs:43-84`, `invite.rs:25-43,107-139`, `links.rs:187-208` und `steam-flows/src/invite.rs:724-885`. Auditexistenz ist kein neuer Versand: GC-Code 5 ist AlreadyHasGame; Recovery setzt invited_at auf now. FAILED ohne GC-Code ist keine sichere Ablehnung; fehlendes Audit kein Beleg für nie eingeladen. Aktuelles `steam_id` verwenden, nicht nullable Legacy-steam_id64 allein. Bestehende primäre/verifizierte Kontoauswahl wiederverwenden, Mehrdeutigkeit fail closed. Friend-code/Fragetext darf nie ein Fremdsubjekt auswählen. Friendship_missing ausschließlich bei tatsächlichem Quellbeleg. Keine eigene Statuspersistenz, keine produktiven Datenmutationen oder Steamaufrufe.

## Eigentum und Arbeitsstand

A-E3, Brain: `/home/nathanael/.worktrees/brain-a-invite-brain-20261006`, Branch `fix/brain-a-invite-brain-20261006`, sauber erstellt von Main `10ebbb208aaaac9ddf6e07bc89a6af4a620f86ef`. Eigentum an eng nötigen Dateien für typed Invitevertrag/Scope in brain-contracts, Adapter und Assembly in brain-serve, erforderlichen Request-/Projektionsstellen in brain-api/brain-kernel/brain-providers und Tests dort. Nicht Storage/Migrationen, maintenance, CLI, dbrain-enrich oder Site. Bestehenden requestgebundenen Evidence-/Publikationsweg nutzen, keine Dauerablage als SourceRecordV2.

A-E4, Bots: `/home/nathanael/.worktrees/brain-a-invite-bots-20261006`, Branch `fix/brain-a-invite-bots-20261006`, sauber erstellt von Main `e1f11614e437d5e4e5610f5a9c5d997913f292ad`. Eigentum an MCP-Toolregistrierung und eng begrenztem neuen Lesemodul in `rust/bin/dl-bot/src/mcp/`, entsprechend nötigen mcp.rs-Teilen, `dl-brain/src/lib.rs` für fehlende Kommandoidentität, main.rs/modglue.rs für den vorhandenen Antwortanschluss und Tests darin. Neue Module nur an diesen vorhandenen Pfaden. `dl-brain/src/brain_api.rs` gehört F2c und ist tabu. B-/Steam-Mechanik und `dl-community/src/invite_lounge.rs` nicht verändern. Kein anderer Concierge-/FAQ-/Pitchumbau in diesem Paket. Wenn ein einziger notwendiger Completion-Hook tatsächlich außerhalb dieses Eigentums liegt, exakt Ort und Signatur an A melden; Statusreader und bestehenden gemeinsamen Statusfrageweg fertig bauen, nicht alles fallenlassen und nicht einen neuen Poll-/Botantwortpfad erfinden.

## Beweisziel

Compile/fmt/clippy und passende bestehende Suites mit eigenen Scratch-Postgres/Loopback-HTTP-Prüfungen. Rechte, Enumsemantik, Pending/Unknown/Error, Code 5, Recoveryzeit, kanonische/mehrdeutige Kontoauswahl, falsche/fehlende Identität und keine Drittpersondaten belegen. A-E3 prüft tatsächlich alle ausgehenden Providerpayloads: Statusfrage mit Namen/Steamcode/Chatkontext bleibt intern, vor jeder möglichen externen Klassifikation/Embedding/Generation auf die erlaubte minimale Aufgabe und Enum/Zeit reduzieren. Kein harter persönlicher Datenschutzbypass für sonstige Discord-/Twitchdaten. Kein weiterer Provider.

Vorhandener normaler Brain-Consumer/Sender beantwortet zeitnah, keine eigene Bot-Statusformulierung und keine doppelte Antwort oder gewöhnlicher Follow-up-Cooldown, der die echte Statusantwort verschluckt. Keine öffentlichen Community-Testnachrichten. Source- und Brainseite werden von A gemeinsam abgenommen; ungekoppelte Seite nicht allein produktiv ausliefern.

## Git, Gate und Rückgabe

Nur eigene Dateien, Rust ohne neue Code-Kommentare. Graphify vor Codefrage. Featurecommit/-push nach Prüfung erlaubt, kein Main-Merge/Deploy/Restart. Ein Git-Schritt pro Bash-Aufruf, literale absolute Pfade, kein add -A, Forcepush oder fremdes Reset/Stash. Selbstgate über `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo <eigener absoluter Worktree> --base <frischer voller Main-SHA> --head <voller Feature-SHA>` ohne Modellroulette. BLOCK genau zurückgeben für frischen Fixer, kein Review-Thread.

Blatt-Worker, keine Subagenten/T3-Threads oder Sessionnachrichten. Root-Akten schreibt allein Paket A. Native Rückgabe mit SHA/Worktree, konkreten Fundstellen/Wiederverwendung, Prüfzahlen/Altbaseline, Gatewortlaut/Modell/Exit/Log und noch offener Kopplung. Auftraggeber A `2c7de4c9-bac4-43ad-b91a-f8ac889f09b4`, Hauptorchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Wache nach 20 Minuten, spätestens 30. Gemeinsam festgelegten Transportvertrag nicht heimlich erweitern, nötige Abweichung mit Ursache melden.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: je eigener oben genannter Worktree
