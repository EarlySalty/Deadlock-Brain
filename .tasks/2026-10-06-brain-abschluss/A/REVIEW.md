# Paket A: offene Gatebefunde

## A-E3f, aktueller SHA-gebundener gpt-6.1-sol BLOCK

Feature 19f6d49f196a8ea4b9d9d90c188f3a3867914779, Basis fde910f6a0199c00f44083e73fc8f4c5e4f80b86. Gate und genau diesen HEAD direkt im Reviewzustand bestätigt, separater Exitnachweis fehlt. Gate /tmp/brain-a-invite-brain-e3f-nit-selfgate.log, Zustand /home/nathanael/Documents/.claude/gpt-workers/review-state/d4f63773103a06db.json.

> [gpt-6.1-sol] BLOCK: General invite questions are hijacked by personal-status routing.

brain-contracts/src/invite.rs:148 erkennt „Wann sind Einladungen wieder verfügbar?“ und „When does an invite expire?“ als eigene Statusfrage. Projektion/Retrieval ersetzen damit die allgemeine Frage. Gate nennt Zwillinge invite.rs:162/:192, brain-api/src/lib.rs:230, provider_input.rs:15, brain-providers/src/hardening.rs:45, brain-serve/src/discord_live.rs:529/:570/:661/:691. Frischer E3g-Fixer nach vorrangiger G1-Abnahme und gemeinsamer Dateiabgrenzung mit G, nicht E3f wieder aufnehmen. Keine zweite Engine oder Vorratsarbeit, kein Gate-Neuwurf. Prüfgrenzen und Sourceübergabe in E3F-RUECKGABE.md, kein Push/Main/Deploy.

## A-E4f, Runde 2, gpt-6.1-sol BLOCK

Feature 2be2df16d7a9390823a05691bef1ae69f1b8c8c1, Basis e18f522226f8e2dec5a1c03fe97c2aba3200c8d1, Exit 1. A las /tmp/brain-a-invite-bots-e4f-proof-20261007/gate.log und die tatsächlichen Sourcepfade.

> [gpt-6.1-sol] BLOCK: Arbitrary questions can bypass backend rate limits.

Matcher dl-brain/src/lib.rs:149 gewährt beliebigen Fragen mit „invite status“ eine Cooldown-Ausnahme. Zwillinge reserve_question:59 und handle_brain_query:233, vollständiger Answerer weiterhin :198/:247. Nachrichtenweg hat weitere Kanal-/Tageslimits; Kommandoweg keine Ersatzbegrenzung oder entsprechende laufende Reservierung. Ausnahme muss auf eine begrenzte eigene Statusoperation wirken, ohne zweite Antwortengine. Die zwei vorherigen Funde sind laut Fixer behoben, dieser neue BLOCK bleibt offen.

Kein Push, Main oder Deploy. Nach verbindlicher G1-Priorität frischen E4g-Fixer starten, nicht E4f wieder aufnehmen und kein Modellroulette. Artefakte/sauberen Worktree erhalten. Zahlen und korrigierte tatsächliche Brainbaseline in E4F-RUECKGABE.md.

## A-E4, Runde 1, gpt-6.1-sol BLOCK

Feature 11eb66ead80c29e157dec8c6d8211bbc1f02fd4d, Basis e18f5222. Direkt gelesen: /tmp/a-e4-gate-20261007.log.

> [gpt-6.1-sol] BLOCK: Historical observations still mask newer request states; completion markers still identify users rather than reservations.

1. dl-bot/src/mcp/self_invite.rs:175: historische GC-/Taskbelege verdecken neuere Pending-/Errorrequests. Neueste Requestzustände zeitlich mitauswerten.
2. dl-brain/src/lib.rs:178, Zwillinge :163/:168: Abschlussmarker identifizieren Nutzer statt konkrete Reservierung. Abschluss einer älteren Anfrage darf keine spätere Reservierung freigeben.

Frischer Fixer A-E4f, wf_65c71ff2-1dc, Task w25lmbzqu, Agent a00a0ac818d8f4752, bestehender sauber übergebener eigener Botsworktree. Briefing BRIEFING-E4F.md. Nur diese Funde, normaler Selbstgate, kein neuer Reviewer oder BLOCK-Neuwurf. Main und Betrieb bleiben gesperrt. Originalprüfung Bot 326/7, Brain 13/0; SHA-verifizierte Basis 332/10, alle sieben aktuellen roten dort rot. Eigene PG gestoppt, kein Deploy.

## A-R2: autorseitiger ENV-Vertragsbefund behoben auf Feature

Direkt vom parallelen Orchestrator gemeldet und Source geprüft: runtime_tooling.rs:154-221 verwendet DEADLOCK_CENTRAL_DSN sowie Datenpfad und Providerendpoint als ENV-Konfiguration des getesteten Enrich-CLI. Der bestehende reale Exit-/Persistenz-/Retrynachweis bleibt erhalten, die Aufnahme muss dem vorhandenen expliziten Configvertrag folgen. Kein zusätzlicher Reviewer oder behaupteter Gate-BLOCK aus dieser Rückgabe.

A-R2 abgeschlossen: Feature 8a3a921938b9318e9a76ea8d6e45172693292a83, Format/Compiler/striktes Clippy und 151 reale Tests grün, normaler gpt-6.1-sol ALLOW. A las Gate/SHA/sauberen Status/Testresultatzeilen, regulärer Featurebackup Exit 0. Details R2-RUECKGABE.md. Keine Integration in bfda408c oder Aktivierung, kein weiterer Standardbuild. Main und Betrieb bleiben gesperrt; live_strecke hält die finale Profilstrecke, G1 hat Vorrang.

## A-E3, Runde 1, gpt-6.1-sol BLOCK

Feature d8a0e727687dda38675833af38dbb698f2ef949c, Basis fde910f6, Gate Exit 1. Log /tmp/brain-a-invite-brain-e3-gate.log.

> [gpt-6.1-sol] BLOCK: The invite detector hijacks ordinary questions and answers a different question.

Fund brain-contracts/src/invite.rs:136. Einladung plus Statusstamm erkennt die Verfahrensfrage „Wie kann ich eine Einladung verschicken?“ fälschlich. Fallback erkennt „Welche FPS bekomme ich in Deadlock?“ und die Variante mit „bekomm ich“ ohne Invitebezug. project_query ersetzt diese gewöhnlichen Fragen durch eine persönliche Statusfrage, retrieve_with_usage umgeht dann normales Retrieval. Beide Zweige enger machen und gewöhnliche Fragen erhalten.

Zusätzlicher eigener bestätigter Fehler: „Bin ich eingeladen?“ wird mangels Marker eingeladen nicht erkannt. Der zugehörige neue Vertragstest ist rot. Datenschutzprojektion muss für diese eindeutige persönliche Statusfrage greifen.

Prüfstand 101 passed, 4 failed, 1 ignored. Weitere rote Analytics-/Scratchprüfungen durch tatsächlichen privaten Harness-/Schemavertrag klären, nicht pauschal als Altfehler bezeichnen. Frischer Fixer A-E3f, Workflow wf_a108250c-d15, Task woo8yd6rv, Briefing BRIEFING-E3F.md. Kein Main-/Featurepush des blockierten Stands und kein Deploy. Folgerunde mit derselben unveränderten konfigurierten Gatekette, kein BLOCK-Neuwurf oder eigenes Reviewmodell.

## A-F2, vorherige Runde erledigt

URL-Rekombinationsblock durch F2c in b29a55a4 behoben, regulärer Gatewiederanlauf ALLOW. Aktuelles B-Main in e18f5222 erhalten, gemeinsamer Gate ALLOW und regulärer Main-Push Exit 0. Runtimeabschluss noch offen. Originale Berichte F2C-RUECKGABE.md und F2-INTEGRATION.md; keine neue Codekorrektur aus nichtblockierenden Hinweisen erfunden.
