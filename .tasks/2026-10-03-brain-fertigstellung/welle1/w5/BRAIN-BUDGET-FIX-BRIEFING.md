[Orchestrator] BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-discord-live-budget-fix

# W5-F1: Netzwerkbudget des Discord-Live-Anschlusses

Frischer Blatt-Worker für genau einen Gate-BLOCK. Keine Unteragenten oder weiteren Threads. Auftraggeber D1b Codex `01a10513-a65d-7531-a870-b4ff8f346f72`, Hauptsession T3 `92efdb66-6e7e-495c-873e-32f2912c2fa2`. Bereich /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung/welle1. Fachbericht ausschließlich w5/BRAIN-BUDGET-FIX-AN_D1.md, neueste Meldung oben, höchstens fünf Zeilen. Steuerung w5/BRAIN-BUDGET-FIX-VON_D1.md. Regeln ../DELEGATOR-REGELN.md, aktuelle VON_HAUPT.md, w5/BRIEFING.md und w5/AN_D1.md lesen. humanizer und no-em-dashes auf sämtliche Texte anwenden, natürliche deutsche Texte mit echten Umlauten, keine Gedankenstriche oder Code-Kommentare.

## Stand und Eigentum

W5 `4582e35a-5466-4e4f-a3b3-c173560c4fc5` hat den Brainanschluss gebaut und seine Brainarbeit nach BLOCK beendet. Er arbeitet nur noch im getrennten Bots-Repo am laufenden Bots-Release/Liveabschluss. Sein sauberer, gepushter Brainstand ist `cfc31a271119ed9b8741aca053f62369016d2cdf` auf Branch feat/brain-discord-live-fakten-20261004 im Worktree /home/nathanael/.worktrees/brain-discord-live-fakten-fertig. 32 Crate-Tests sowie Clippy/fmt Exit 0. Bots separat main `85b9c1fcee2ff82b5db0131d6577ba460f655a95`, sechs MCP-Tests und Gate ALLOW; dort nicht bauen.

Eigenen Worktree /home/nathanael/.worktrees/brain-discord-live-budget-fix und Branch fix/brain-discord-live-budget-20261004 vom bestätigten cfc31a2 anlegen. Freiheit des Pfads prüfen, alten Worktree/Branch erhalten. Du bist nicht allein im Dateisystem; fremde Änderungen erhalten. Nur rust/crates/brain-serve/src/discord_live.rs und unmittelbar nötige bestehende Regressionen dieses Anschlusses ändern. Weitere Dateien nur bei zwingender Notwendigkeit mit konkretem Pfad an D1b melden. Keine Neuimplementierung, NITs, Refactorings oder fremde Korrekturen.

## Einziger Bauauftrag

Reguläres Gate e1431b71c114279c17ef9306a36ed96e8fe0637f..cfc31a271119ed9b8741aca053f62369016d2cdf: Exit 1, BLOCK. Befund laut w5/AN_D1.md 06:16: discord_live.rs:241/288/304, validate_evidence und validate_publication rufen erneut per HTTP ab, selbst bei max_network_rounds=0. Diese Abrufe sind nicht in der Retrieval-Usage erfasst. Genau das beheben. Netzwerkbudget muss sämtliche tatsächlich ausgelösten Abrufe begrenzen und erfassen; null Runden dürfen keinen Netzwerkabruf auslösen. Bestehenden Quellen-/Retrievalvertrag nutzen, keine Umgehung durch künstliche Usagezahlen. Publikations- und Providerfreigabe, öffentliche Zulassung sowie private Feldsperre erhalten. Keine Rechteumdeklaration.

Die Gate-NITs observed_at-Frische, striktes isError:false und Ausschnitt der Tokenauflösung sind kein Zusatzauftrag. Bots liefert isError:false und 60-Sekunden-Cache; vorhandener Secretresolver nutzt denselben Snapshot. Nur soweit der BLOCK selbst zwingend davon abhängt, eng ändern. Vollständigen Gatebefund bei Bedarf im bestehenden W5-Gatelog suchen; kein eigener Reviewer und keine Gateänderung.

## Prüfung und Übergabe

Passende vorhandene brain-serve-Prüfungen sowie eine gezielte Regression für den Budget-BLOCK. HOSTPROBE.md in /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/ beachten: ein freier Slot, drei Jobs, fmt ohne Slot, nur betroffene Crate. Bestehende Toolchain 1.97.1 nutzen. Früh committen, nach grünem Schritt eigenen Branch pushen.

Danach genau das bestehende Gate: `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo <eigener Worktree> --base e1431b71c114279c17ef9306a36ed96e8fe0637f --head <Fix-HEAD>`. BLOCK: nicht selbst weiterfixen, Stand sichern und an D1b für frischen Fixer melden. Exit 2: einmal wiederholen, danach melden. Nur ALLOW ist Übergabe als geprüft.

Nach ALLOW kompletten benötigten Brainanschluss samt Fix-SHA, Branch und Prüfungen in w1/EINGANG.md ergänzen; fremde Einträge erhalten. Einziger Brain-main-/Deploy-Owner ist W1-F2 `5c831e60-c729-43ec-9e43-59c5c2bfea41`. Kein eigener Brain-main-Push, Brain-Deploy, Konfigurationsänderung oder Nutzerchat. Main bleibt für W1s laufenden Herkunftsbuild auf 6dccad919bdfa370ce921fe680280e1a61f52b06.

Bestehender Laufzeithinweis von W5: budgets.max_network_rounds derzeit 1, Live-Retrieval und Antwortprovider benötigen zusammen mindestens 2. D1b koordiniert nötige Einstellung mit W1; keine automatische Budgeterhöhung im Fixcode. Nach geprüfter Übergabe Worktree/Branch bis Integration und tatsächlichem Livebeleg erhalten. Bericht: Befehl, Exit, Testzahl, SHA und offener Blocker. Fertig ist dein Sourceauftrag erst nach ALLOW und geordneter Übergabe; Liveabschluss bleibt bei W1.
