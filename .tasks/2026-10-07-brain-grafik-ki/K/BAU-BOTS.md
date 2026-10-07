# K: Bauauftrag für den öffentlichen Discord-Guide

Authoritative Arbeitskopie aus dem am 07.10.2026 nachgelesenen korrigierten /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-grafik-ki/AUFTRAG.md, Zeilen 14 bis 48. K/PLAN.md konkretisiert die erste Welle.

## Verbindlicher Umfang

Pate=Concierge=Deadlock Brain ist EINE persönliche KI-Hilfe. Serverguide ist eine Fähigkeit davon. Keine menschliche Patenvermittlung, keine Rollen/Übernahmen oder entsprechenden Nein-/Anfragepfade, keine zweite Persona. Fremde Mechanik unangetastet lassen. Normale Zustimmung, Privatsphäre und Ablehnung der KI-Hilfe erhalten.

Erste Discordstrecke ist die bestehende öffentliche Serverguide-Endantwort, nicht der private FAQ-Chat mit Verlauf. Bewusste öffentliche Anfrage bis zustellbarer Brainantwort durchgängig erhalten. Kein privater DM-/Profil-/Historytext an Remoteprovider, auch nicht an Loopback-Abo-Proxy. Ohne bewiesenen lokalen Provider private Eingänge vor Modellanfrage sperren und einen sinnvollen lokalen Hilfsweg nennen. Keine neue Gedächtnisablage, breite Kontaktserien oder proaktive DMs.

## Bestätigter Bestand

Bots-K-Worktree /home/nathanael/.worktrees/bots-k-guide-20261007, Branch feat/bots-k-guide-20261007, HEAD 56571e40fa215a5cbca081b09827d78fdff4c00d, bislang sauber.

- rust/bin/dl-bot/src/modglue.rs: bewusste Erwähnungen und DMs verwenden bereits einen Brain-Endantwortpfad, Personenbindung und erneute Rechteprüfung vor Versand.
- rust/crates/dl-brain/src/brain_api.rs: AsyncBrainClient, bot.public; derzeit auch private DMs darüber, kein belegter lokaler Providervertrag.
- rust/bin/dl-bot/src/main.rs: privater FAQpfad benutzt weiterhin shared_answers.
- rust/crates/dl-community/src/faq.rs: private FAQkanäle mit Verlauf, nicht als öffentliche Eingabe migrieren.

Vor konkreten Codefragen code-suche/Graphify; Fundstellen nachlesen. Keine Vollinventur, keine menschlichen Patenrecherchen.

## Eigentum und Schutz

Nur Botsmodglue/Brainantwortadapter und minimal nötige öffentliche Guideverdrahtung bearbeiten. Kein Brainclient-Pin/Manifestwechsel, keine G contracts/tools/provider/kernel/reasoner, E/F/I oder H-Dateien. K ist einziger Integrator/Deployer. Keine Commits, Pushes, Deploys vor Rückgabe.

NEVER read, print or write plaintext secrets. MUST NOT send private user/community data to remote models. Rust/Postgres, keine neuen Code-Kommentare, normale Configdateien/Infisical, keine ENV-Konfiguration oder Modellwechsel. Keine fremden Konten oder echte Moderationsaktionen als Test.

## Technischer Arbeitsweg

K hat den korrigierten Auftrag über den nativen Read-Zugriff auf dem ausdrücklich beauftragten Literalpfad erfolgreich nachgelesen. Keine /home/nathanael/Documents/Deadlock-Brain-Aliaspfade raten. Native Read ist für Dateien korrekt, die anschließend editiert werden. context-mode ctx_execute_file ist auf seinen ursprünglichen Projektroot beschränkt und eignet sich nicht für Worktree-Dateien außerhalb dieses Roots. Keine Settings-/Permissionsänderung und keine Umgehung eines echten Host-/Nutzer-Denys. Bei einem echten Deny stoppen.

Parent-CWD bleibt /home/nathanael/.worktrees/brain-k-ki-20261007. Nicht cd in Kanon und keine fremden EnterWorktree-Wechsel. Absolute Dateipfade, Cargo --manifest-path und Git -C mit literalen absoluten Pfaden.

## Beweis und Bericht

Compiler, Format, striktes Clippy, vorhandene passende Tests mit Test-Wächter und aktuellen HOSTPROBE-Slots --jobs 3/sccache. Keine zusätzlichen Target-Kopien oder Sperren umgehen. Testzahlen und echte Baseline nennen. Öffentliche/DM/private FAQ-Eingänge getrennt beurteilen; Ausfallparität und lokale Rechte dürfen nicht schwächer werden.

Rückgabe als K/BOTS-BERICHT.md im Bots-K-Worktree mit Dateien, Exits, Testzahlen, fehlendem lokalen Vertrag, Restumfang. Nicht vollständig migriert oder live behaupten. Rückfragen an K, nicht Nutzer.
