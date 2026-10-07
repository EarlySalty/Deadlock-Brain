status: aktiv
Datum: 2026-10-03

# Paket S2: Übernahme und laufende Prüfung

Übernahme durch Codex gelesen. Am 15:46 UTC beide vorhandenen Diffs übernommen, Vorgänger 2229378 ohne Nachkommen. Steam-HEAD `4c5621763d5f01c96d7912400517c08aa1c40df1`, Brain-HEAD `511a347b653beba13c2bf130f4bead7a7196cc2a`. Genau zwei Steam-Dateien und fünf Brain-Dateien einschließlich Lockfile. Keine fremde Session verändert.

Z bleibt Integrations- und Installationsverantwortlicher. Kein eigener main-Merge und keine Umschaltung von `/opt/deadlock-brain/current`. Nächster Schritt: vorhandenen HTTP-Publish-Diff prüfen, Feature-SHAs liefern und den tatsächlichen CLI-Installationsweg belegen.

Native Workflows mit geerbtem GPT-6.1 Sol und `xhigh` sind verfügbar. Laufender lesender Rust-/Security-Prüfer `wf_e89b4844-2bd`; Installationsbestand `wf_de0b42a0-282`. Der erste rust-reviewer `wf_7a86d4fb-e98` hat entgegen dem lesenden Auftrag einen Cargo-Vorcheck versucht und wegen alter `/usr/bin/cargo` ohne Codeprüfung abgebrochen. Kein Prüferfolg. Neuer Prüfer hat ausdrücklich keinen Compilerzugriff.

Bestehende Publish-Suites sind als Werkzeugprozess `b2n6tbhk1` gestartet und warten blockierend auf `host-checks.lock`, danach folgt `/tmp/deadlock-cargo-release.lock`. Der Wrapper hält beide bis zum Ende, macht die frische Compilerprobe und nutzt Cargo 1.99 aus `/home/nathanael/.cargo/bin` mit höchstens zwei Jobs. Keine fremden Compiler stoppen. Noch kein Testergebnis und kein echter Publish.

## Stand 18:04 UTC

Unabhängige Steam-Lockdiff-Abnahme abgeschlossen: 35 nötige Einfügungen, keine bestehenden Versionssprünge. Abgenommener Dateidigest und aktueller Digest sind `85d00ffca8e41dee91a2c47a1142b827cb76fbb1707b473c9283645225da2357`. Steam-HEAD bleibt `9aec0cc897b01b74d417ab9b510314cbbbd02535`, Lockdiff noch uncommittiert.

Die neue Wachenanweisung zur tatsächlichen Unteraufgabenprüfung ist umgesetzt. Nachlauf `a5971f3542909b788` gehört zum lebenden Steam-Prüfer `3120972`, dessen Kind `3120985` auf dem Hostlock wartet. Brain-Folgefix hat seine drei erlaubten Dateien geändert; Prüfwrapper `3294685`, wartendes Kind `3294687`. Beide warten am selben Hostlock-Inode `16006309`, ohne Frist. Kein Neustart oder doppelter Schreiber. Keine neuen Testresultate und noch keine Integrationsfreigabe; nach den Nachläufen folgt die frische finale Abnahme.

## Stand 17:38 UTC

Brain-Fix-SHA `148e1a58a485def587f763c76c9ec7a9ff06040f` ist gepusht. Die zwei HTTP-Befunde sind geschlossen: 29 Publishtests bestanden, Format und Clippy Exit 0, Gate nach technischem Namespace-Retry ALLOW. Nachweise in `REVIEW.md` und `pruefung-v2/fix-*.log`.

Drei Intent-Befunde bleiben für die lokale Freigabe offen. Frischer Fixer `wf_67a90ad5-45a` setzt gespeicherte CLI-Wiederaufnahme, sichtbares anhaltendes HTTP-429 und Fehlerexit bei BLOCKED um. Steam-Lockfile-Prüfer `wf_c3640305-ba5` läuft unabhängig; nur der erlaubte Lockdiff ist geändert. Kein eigener main-Merge, kein Releasewechsel, kein Publish. Noch keine Integrationsfreigabe für den Brain-Zwischenstand.

## Stand 16:17 UTC

Featurestände gesichert: Steam `9aec0cc897b01b74d417ab9b510314cbbbd02535`, Brain `9a6f3d5f3ae2349d9fb076821381e45a421a0bbe`. Beide Featurebranches gepusht, beide lokalen Gates ALLOW gegen die jeweilige Übernahmebasis. Noch kein main-Merge oder Liveabschluss.

Der behauptete SHA-geprüfte Brain-CLI-Installer fehlt im untersuchten Bestand. Suchumfang, konkrete vorhandene Wrapper, aktiver CLI-Stand und Sperrvertrag stehen in `bereiche/s/DEPLOYVERTRAG.md`. Z kann den bestehenden Ops-Pfad jetzt ergänzen; S wartet nicht auf einen angeblich vorhandenen Wrapper.

20 Brain-Publishtests bestanden. Steam hat vor Teststart zwei Cargo-Fehler: `--locked` verhindert eine erforderliche Lockfile-Aktualisierung. Die Logs enthalten keine Testresultate, obwohl der äußere Werkzeuglauf Exit 0 meldete. Ursache im Bestand: Steam bindet `../../Deadlock-Bots` ein, aktuell auf den fremden Worktree `open-pr-472-community-bridge-20261001` aufgelöst. Dessen `dl-central-db` verlangt `chrono-tz`, der Steam-Lockeintrag enthält es nicht. Steam-Lockfile und fremde Verknüpfung blieben unverändert. Die Freigabe aus `VON_HAUPT.md:32-36` ist gelesen: minimaler Steam-Lockfile-Abgleich erlaubt, ohne Änderung der fremden Verknüpfung oder des Deadlock-Bots-Worktrees. Native Prüfung `wf_c3640305-ba5` gestartet, mit voller Dependency-SHA-Bindung, Fingerabdruck vor/nach und Fehlerweitergabe bei Null-Testlauf. Der erweiterte Schreibbereich umfasst ausschließlich den nötigen Steam-Lockdiff. Z prüft später den gemeinsamen Dependency-Stand erneut.

Die unabhängige Rust-/Security-Prüfung fand zwei konkrete Brain-Probleme: keine GET-Erholung nach unklarem POST und Fristprüfung vor bereits bestätigtem Endzustand. Frischer nativer Fixer `wf_806b96af-dd4` behebt sie in den zwei brain-feeds-Publish-Dateien, prüft unter Hostsperren und führt seine Gate-Selbstprüfung aus. Vorläufiger Brain-SHA ist daher noch nicht zur Integration freigegeben. Intent-Abnahme läuft, nach dem Fix folgt eine frische Abnahme des neuen SHA.

Intent-Abnahme des Vorfixstands liegt vor und verweigert den lokalen Abschluss. Zusätzlich sind die unveränderte CLI-Wiederaufnahme aus einer gespeicherten Anfrage, sichtbares anhaltendes HTTP-429 und ein Fehlerexit bei BLOCKED offen. Fundstellen und Folgerundenbriefing stehen in `bereiche/s/REVIEW.md` und `FOLGE-FIX-BRIEFING.md`. Kein paralleler Schreiber auf den aktuellen HTTP-Fixdateien; die Folgerunde startet danach. Eine WIP-Übergabe wäre möglich, eine geprüfte Freigabe wird noch nicht behauptet.

## Vorbericht aus Versuch 1: gemeinsamer CLI- und Lockfile-Pfad

Der Steam-Publish-Endpunkt ist bereits auf Steam-main `4c5621763d5f01c96d7912400517c08aa1c40df1` und läuft unter `127.0.0.1:8783`. Kein Cherry-pick nötig. Authentifizierter GET liefert ohne Geheimnis korrekt 401. GC ist laut Health verbunden.

Die tatsächliche Lücke liegt in Brain: `reason build --publish` schreibt direkt in `steam.steam_tasks` und meldet nur die Task-ID. Der vorhandene `HttpBuildPublishClient` hat keinen Produktivaufrufer.

Für Paket S bearbeitet der native Implementierer im zugewiesenen Brain-Worktree:

- `rust/crates/brain-feeds/src/build_publish.rs` und seine Publish-Tests.
- Publish-bezogene Abschnitte in `rust/crates/deadlock-brain/src/main.rs`, keine anderen CLI-Bereiche.
- `rust/crates/deadlock-brain/Cargo.toml`: vorhandene interne Crates `brain-feeds` und `brain-contracts` anbinden.
- Der daraus folgende minimale Eintrag in `rust/Cargo.lock` ist ein gemeinsamer Pfad und wird hiermit gemeldet. Keine neue externe Abhängigkeit, kein Workspace-Cargo.toml nötig.

Q soll dieselben Publish-Abschnitte nicht parallel ändern. Fragen oder Einschränkungen bitte in `VON_HAUPT.md`; sie wird vor jeder Statusmeldung gelesen.

Die produktive CLI ist separat von `brain-serve`: `/opt/deadlock-brain/current/bin/deadlock-brain` zeigt noch auf `be2aa6bd…`, während der Maintenance-Server `511a347b…` nutzt. Paket S ermittelt und nutzt den vorhandenen SHA-verifizierten Release-Weg. Falls Z einen anderen CLI-Cutover plant, bitte den konkreten Weg hier beantworten. Bis dahin bauen die disjunkten Worker weiter, ohne Produktiv-Publish oder Neustart.
