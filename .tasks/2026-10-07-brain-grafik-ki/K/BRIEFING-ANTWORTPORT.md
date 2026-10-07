# K: Bestehenden Antwortprovider vollständig anschließen

## Ziel und verbindlicher Vertrag

Neuester ausdrücklicher Delegatorauftrag: den bestehenden `brain-serve/src/service.rs`-Enum vollständig für accounted und Turnmethoden delegieren. Default `answer` allein reicht nicht. `Kernel::with_tools` erst mit tatsächlichem G-V-Port, kein Ersatzadapter, kein fremder WIP. Dieser erlaubte Kernanschluss hat Vorrang vor nichtblockierenden Grafik-NITs.

Vertrag tatsächlich geliefert: `/home/nathanael/.worktrees/brain-g-v2-20261007/.tasks/2026-10-06-brain-abschluss/G/ANTWORTPORT-VERTRAG.md`. Lies ihn. Produktstand `dbce14aedadd94881a3cb21151d9840994094cd9`, gemeinsamer regulärer Provider-/Kernelgate gegen `a6568629` ALLOW. Origin-Nachweis jetzt ausdrücklich erbracht: `origin/feat/brain-v2-g-20261007` auf `2b67796fb80ae3440a0c9e76671dfc8169032844`, dbce14ae-Ancestor Exit 0. Haupt-K hat frischen Fetch und diese Ancestry selbst geprüft; Delegator bestätigt separat ls-remote. Der ältere offene Originvermerk im Vertragsdokument ist überholt. Dies ist keine Gesamt-Rechenkern-/G-V-/Spiegel-/F-Abnahme.

Bestehende Methoden: `answer`, `answer_turn`, `answer_accounted`, `answer_turn_accounted`. Beide konkreten geprüften G-Provider vollständig delegieren, einschließlich Kontext, Belegen, tatsächlichen Tools, vollständiger Historie, Deadline und Fehlerabrechnung. Keine leeren Ersatzslices, kein Fehler- oder Usageverlust, keine zweite Antwortengine und kein neuer Ledger. Erstturn-Egressprüfung der konkreten Provider auch ohne Werkzeugbelege erhalten. Bots behalten den Veröffentlichungszweck.

## Eigentum und Arbeitsstand

Arbeitskopie ausschließlich `/home/nathanael/.worktrees/brain-k-ki-20261007`, Branch `feat/brain-k-ki-20261007`, aktueller HEAD `b310e223c1fdbaed17661f10e8edebae5b1534df`. Akten sind eigener unstaged WIP. Artefaktcheckpoint ist committed, aber wegen JSONB-Fingerprintfehler regulär BLOCK und nicht gepusht; nicht ändern oder als ALLOW behandeln. Grafikfixrunde ist vorbereitet, noch nicht gestartet. Parallel läuft nur a92067f4cff648233 mit read-only Produktdateirechten an einer bestehenden isolierten Maintenance-Testfixture.

Du bist einziger nativer Produktwriter am zentralen Antwortanschluss. K ist laut Auftrag und G-Vertrag einziger service.rs-/Consumerwriter. Schreibbereich: bestehende `rust/crates/brain-serve/src/service.rs` und eng zugehörige eigene Anschlussprüfung. Du darfst ausschließlich bereits committed, regulär geprüfte G-Vertrags-/Provider-/Kerneldateien aus dem exakt gepinnten dbce14ae-Objekt übernehmen, falls für den tatsächlichen vorhandenen Enumanschluss zwingend. Keine Datei aus dem lebenden G-Worktree kopieren. Vor Übernahme konkrete Datei-/Commitkarte gegen K erstellen und Änderungen aus aktuellem Main/K erhalten. Graphify vor Bestandssuche; bekannte konkrete Fundstelle `service.rs:124,129,431`. Kein Import unfreigegebener Reasoner-, E-, F-, G-V-, Analytics- oder Loaderteile. Kein einfacher Komplettoverwrite von Main/K. Nötige Manifest-/Lockanpassungen nur für diese geprüfte Quelle, keine neue produktive Crate oder Connector.

Fehlt eine Voraussetzung außerhalb des gesicherten Vertrags, genaue Datei-/Signaturabhängigkeit zurückgeben, nicht Ersatztypen oder einen scheinbar fertigen Toolport bauen. Keine parallele Änderung an Gs Arbeitskopie, keine Änderungen an K-Artefakt-/H-Dateien oder Bots-/Twitch-Worktrees. Keine Modell-, Providerkonfigurations- oder Timeoutänderung. Kein Commit, Push, Merge, Releasebuild, Deploy oder Runtimeeingriff. K allein integriert und sichert.

## Datenschutz und Beweisziel

NEVER read, print or write plaintext secrets.
MUST NOT send private user/community data to remote models.
Codex-Abobrücke 127.0.0.1:18769 verarbeitet extern. Keine private FAQ-/DM-/Communityprobe, keine Rohfragen oder IDs in echte Modellaufrufe, kein behaupteter lokaler Provider. Bestehende private Eingangssperren erhalten. Eigener Invite-Status bleibt exakt genehmigte Enum-/Zeitprojektion nach interner Identitäts-/Rechteprüfung; diese Runde erweitert sie nicht. Kein neuer Provider oder Modellwechsel. Nutzerentscheidung zu privater Verarbeitung bleibt offen.

Compiler, passende Formatierung, striktes scoped Clippy und vorhandene Anschlussprüfungen ausführen. Echte lokale HTTP-Prüfung der beiden konkreten Provider-/Enumzweige mit synthetischer Strukturprobe, ohne echten Modellaufruf. Abrechnung bei Erfolg, Toolturn und Fehler sowie unveränderte Deadline-/Autorisierungsweitergabe nachweisen. Vorhandene Tests erhalten, keine Unterdrückung oder pauschales Gesamtgrün bei roten Bestandsprüfungen.

Vorhandene Caches und absolute moderne Cargo-CLI verwenden, tatsächlich freien Buildslot `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/build-slot-{1,2,3}.lock` halten, höchstens `--jobs 3`. Keine globalen Targetkopien oder fremden Compilerstopps. Exakte Befehle, Exit-Codes, passed/failed/ignored/filtered und Logpfade zurückgeben. Keine Test-Exitmaskierung. Kein eigener Reviewer, regulärer Gate folgt durch K auf committed Checkpoint.

## Routing

K-Session `988eeaea-28ee-424c-b362-e250610cde91`, Produzent teil-k, Versuch 1. Delegator `481426fe-b477-42b3-91c6-901811fcba1d`, Hauptorchestrator `d3a1741e-82bc-4a48-865b-2845c663dca7`. Keine weiteren Subagenten, T3-Threads, ListAgents oder SendMessage. Keine zentralen Register/TODO-Edits. Rust/Postgres, keine neuen Code-Kommentare. Native Rückgabe nur bei Abschluss oder echtem Blocker, keine Rundenzwischenmeldungen.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-k-ki-20261007
