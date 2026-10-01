status: erledigt
Datum: 2026-09-29

# R-BD: unabhängige Consumer-Abnahme vor G5

## Urteil

**Fertig: J. Abweichungen: begrenzter Prüfhinweis unten. Fix nötig: N für den beauftragten B-/D-Umfang. Ergebnis: GO für die technische Consumer-Abnahme, keine G5-, Merge- oder Produktionsfreigabe.**

Keine bestätigten blockierenden Funktions- oder Sicherheitsmängel in den geprüften Änderungen. 106 eigene Tests bestanden, 0 fehlgeschlagen, 0 ignoriert, 0 gefiltert. Die 232 Tests aus B-REPORT und die 68 Tests aus D-REPORT sind davon getrennte Autorennachweise und werden nicht zu den eigenen Tests addiert.

Unabhängiger Reviewer ohne Autorenschaft an den Consumer-Änderungen. Keine Produktdateien geändert, keine Unteragenten gestartet, keine fremden Sessions kontaktiert. Berichtbranch: `review/pre-g5-consumer-abnahme-20260929`. Auftrag: `R-BD-BRIEFING.md` im Worktree `brain-technical-closeout-20260929`.

## Geprüfte Stände und Abgrenzung

| Gegenstand | Exakter Stand | Ergebnis der Identitätsprüfung |
| --- | --- | --- |
| Bots B | `46edc1023a819ba0ba3c37b7de96c333f485f797` | Sauber vor und nach den Prüfungen. Gegenüber Code-HEAD `553e13491489a3cca9c74118fcc82e54cff3502b` unterscheidet sich der Stand durch B-REPORT.md. |
| Docs D | `ff20af7a8e3fcacc349cb2d97eda34dfa0897957` | Sauber vor und nach den Prüfungen, entspricht PR #4. |
| 2nd-Brain D | `ab83b691befd761a16d971af5c249a604c5d4e0d` | Sauber vor und nach den Prüfungen, entspricht PR #2. |
| Twitch D | `cf3d77085ed350554914b14c3d9981d37b95903a` | Per `git show` gelesen. Merge `13321934f421b9a1cd81d0be8668c2e5a8fd925b` ist Vorfahr, `git merge-base --is-ancestor` Exit 0. Kein Testworktree neu angelegt. |
| Berichtbasis | `1d2a440e279893c1c0693b3bf6d5d6f3bfa30000` | Eigener sauberer Worktree; D-REPORT.md vorhanden. Gemeinsamer Client zusätzlich am Pin `3b86d3cbe5ea39a67b8b1fbd8a3d48ab935982ef` gelesen. |

B wurde gegen die vorgegebene Basis `42175e5fd68a1c83bf4201b4c40cca1099f00652` geprüft, nicht gegen den ersten Elternteil des erhaltenen Main-Merges. Eigener Diff einschließlich Bericht: 16 Dateien, 1059 Einfügungen, 82 Löschungen. Die Eltern von `00a4e1d7ab1372883a391a7df21cbdd3a07dd588` sind exakt `74cc114290e6490afc5dc922ef660c34025c5f36` und `42175e5fd68a1c83bf4201b4c40cca1099f00652`. Eingehende Main-Änderungen sind kein C9-Neubau.

## B: geprüfte Wirkung

1. **Startpfad und Modi: bestanden.** `rust/bin/dl-bot/src/main.rs:1056-1157` liest die geladene Betriebsdatei, wählt Legacy, Typed oder Shadow und registriert denselben Handler am Slash-Command. `main.rs:1525-1526` hängt ihn an den Message-Dispatcher; `modglue.rs:764-779` verarbeitet `!brain`. Damit bleibt der Adapter nicht als ungenutzte Bibliothek liegen. `runtime_config.rs:171-190,216-220` bildet fehlende Modusangabe auf Legacy ab. Das Enum lehnt explizit leere und unbekannte Zeichenfolgen ab. Die entsprechenden Gegenbeispiele stehen in `main.rs:2400-2440`. Der Schutz für Typed zusammen mit Open-Test-Review-Builds liegt vor der Registrierung in `main.rs:1077-1078`.
2. **Typed und Shadow: bestanden.** `main.rs:1126-1138` verdrahtet Typed direkt mit `BrainApiAnswerer`; `rust/crates/dl-brain/src/brain_api.rs:103-124` ruft den kanonischen Client, ohne Legacy-, Modell- oder Retrieval-Rückfall. Das vorab konstruierte Legacy-Objekt wird im Typed-Zweig nicht als Antwortpfad verwendet. `modglue.rs:373-400` startet die Shadow-Probe mit eigenem Timeout in `tokio::spawn` und wartet anschließend auf die sichtbare Legacy-Antwort, nicht auf die Probe. Probe-Ergebnis, fehlender Beleg, Fehler und Timeout haben eigene Log-Zweige. Die Admission-Semaphore und die äußere Frist des Adapters begrenzen gleichzeitig laufende HTTP-Aufrufe beziehungsweise die Wartezeit.
3. **Identität und Status: bestanden.** `brain_api.rs:39-75` kombiniert einen zufälligen 128-Bit-Instanzanteil mit einer überlaufgeprüften Sequenz. Request und Einmal-Conversation bleiben über unabhängige Adapterinstanzen getrennt. Eigener Test `independent_instances_never_share_request_or_conversation_ids` bestanden. `brain_api.rs:82-100` erhält `build_rejected` als sichtbaren Erklärungstext, behandelt `unavailable` und technische Fehler als Backendfehler und bildet fehlende Belege auf `NoAnswer` ab. Der eigene Testlauf bestätigte diese Pfade. `dl-brain/src/lib.rs:74-82` loggt Backendfehler und verbraucht dafür keinen Cooldown, eine neue Anfrage bleibt möglich.
4. **Discord-Ausgabe und Schutzgrenzen: bestanden.** Beide Eingänge prüfen den Kanal und lehnen DMs ab (`modglue.rs:527-550`). `allowed_mentions` ist in Text und Embed ausgeschaltet (`587-620`) und bleibt im BridgeReply erhalten (`569-584`). Beschreibung und Titel werden nach UTF-16-Einheiten begrenzt (`650-689`); lange Ablehnungstexte laufen durch denselben Formatter. Sende- und Editierfehler werden mit Kanal beziehungsweise Nachrichten-ID geloggt (`474-523`). Betriebsfelder sind im Katalog `protected`, ohne Browserwerte; der eigene Katalogtest bestand. Endpunkt, Scope-Menge und Timeout werden vor Verwendung validiert. `runtime_config.rs:694-726` führt `BRAIN_API_TOKEN` über den bestehenden Secret-Bootstrap, nicht über die Betriebsdatei. Produktive Zugangsdaten wurden nicht gelesen.
5. **Workflow und Policy: bestanden.** Der Diff von `.github/workflows/pr-release-gate.yml` reduziert Schreibrechte auf Leserechte und entfernt die beiden mutierenden Aufrufe `pulls.updateBranch` und `pulls.merge`. Die Prüfung der bisherigen Gates wird dadurch nicht abgeschwächt. Die beiden bestehenden Policy-Tests bestanden im eigenen Lauf. Der neue Offline-Workflow startet die Fixture-Prüfung, nicht den Bot. Das ist ergänzende Evidenz, kein Ersatz für das lokale Merge-Gate.

## D: Regression und Belegprüfung

### Docs

`tools/brain-adapter/src/lib.rs:28-33,72-88` prüft exakt `{docs.public}` und verwendet `AsyncBrainClient`. CLI-Aufrufer sind `src/main.rs:5-9,25-44`. `prepare` serialisiert die geprüfte Query ohne Netzaufruf. In diesen Pfaden ist kein Corpus-Export oder lokaler Modell-/Retrieval-Rückfall vorhanden.

`src/infisical.rs:79-116,123-209` prüft Konfiguration und lokalen Endpunkt, liest den Bootstrap aus einer privaten, nicht verlinkten Credential-Datei und lädt den ausgewählten Brain-Zugang über den geschützten Unix-Socket. Docs verwendet eine Credential-Datei, nicht den Credential-FD von 2nd-Brain. Die CLI akzeptiert keinen direkten Zugangswert als Argument. Die eigene Suite bestätigte 18 Tests einschließlich lokalem HTTP-Transport, UDS-Fixture, Scope- und CLI-Grenzen.

### 2nd-Brain

`tools/brain-adapter/src/lib.rs:30-63,103-127` verlangt eine explizit gebundene interne Scope und die exakte Übereinstimmung der Query. Öffentliche, leere, Wildcard- und erweiterte Scope-Mengen scheitern vor dem Brain-Aufruf. `src/main.rs:12-50` führt Query und Answer über `InternalBrainAdapter`; ein sekundärer Antwort- oder Corpuspfad ist dort nicht vorhanden.

`src/infisical.rs:80-110,122-206` bindet den Secret-Namen an `BRAIN_SERVE_SECOND_BRAIN_TOKEN`, nutzt den Credential-FD und begrenzt die Antwortgröße. Eigene Format-, Test- und Clippy-Prüfung mit Toolchain 1.97.1 erfolgreich, 16 Tests bestanden.

Der externe Jobblocker wurde unabhängig nachgelesen: `gh api repos/EarlySalty/Deadlock-2nd-Brain/actions/jobs/109214649281` liefert `steps: []`, `runner_id: 0`, leeren `runner_name`. Die Check-Annotation meldet fehlgeschlagene Kontozahlungen beziehungsweise ein Ausgabenlimit und ausdrücklich einen nicht gestarteten Job. Beide API-Aufrufe Exit 0. Das ist kein fehlgeschlagener lokaler Test und kein Codebefund.

### Twitch

Am zugewiesenen SHA bestätigt `rust/crates/tb-config/src/dashboard_options.rs:10-60` die drei Modi; `tb-config/src/file.rs:201-212` propagiert Parsing- und Validierungsfehler. `tb-dashboard-api/src/handlers/self_explainer.rs:789-805,855-886` enthält im Typed-Fehlerpfad eine Unavailable-Antwort statt Legacy-Fallback. Shadow startet die Probe abgekoppelt und wartet im sichtbaren Pfad auf Legacy. `tb-knowledge/src/brain.rs:92-132` verwendet den kanonischen Client und behandelt Fehlerstatus entsprechend.

Die 21 Self-Explainer-Tests und 31 Knowledge-Tests aus D-REPORT wurden in diesem Review nicht erneut gebaut oder ausgeführt. Ihre Zahlen bleiben übernommene, klar abgegrenzte Belege. Quellpfade, zugehörige Tests, exakter Prüf-SHA und Merge-Abstammung wurden unabhängig gelesen. Der bereits entfernte D-Testworktree wurde nicht als Produktänderung ausgegeben. Es erfolgte kein zusätzlicher Consumer-Neubau.

## Mängelliste und verbleibende Unsicherheit

**Bestätigte, auftragsrelevante Mängel: 0. Erforderliche Fixes: 0.** Deshalb gibt es keine erfundenen Fehlerstellen oder vorsorglichen Implementierungsaufträge.

**Nicht blockierender Prüfhinweis, kein nachgewiesener Laufzeitfehler:** Twitch erzeugt IDs weiterhin aus PID und Prozesszähler (`self_explainer.rs:754-759`), anders als der geprüfte Bots-Adapter mit Instanzzufall. Bei gleicher PID nach Neustart oder in getrennten PID-Namensräumen können IDs erneut entstehen. Eine konkrete falsche Antwort oder ein Zugriff auf fremde Daten wurde damit nicht nachgewiesen; dafür wurde weder eine Produktionskonfiguration noch ein produktiver Conversation-Store untersucht. Der Brain-Cache bindet zusätzliche semantische und Berechtigungsdaten, eine ID-Wiederholung allein beweist daher keinen Datenabfluss. Bei künftigem Mehrinstanzbetrieb sollte die Twitch-ID-Erzeugung gesondert mit einem Instanzanteil und einem Neustart-Gegenbeispiel geprüft werden. Dieser Hinweis ist keine neu belegte Regression von Paket D und kein Anlass, in diesem Review Consumer-Code umzubauen.

Die fehlende G5-Freigabe, die nicht gestartete 2nd-Brain-CI und die nicht wiederholten produktionsabhängigen Tests sind Auftrags- beziehungsweise Beleggrenzen, keine als Codefehler umetikettierten Blocker.

## Eigene Befehle und Ergebnisse

Die Befehle liefen mit geleerter geerbter Umgebung, explizitem PATH und eigenem `CARGO_TARGET_DIR=/tmp/review-bd-20260929-target`. Es wurden keine produktiven DSNs oder Anwendungsschlüssel übergeben. Cargo durfte keine Netzwerkabhängigkeiten nachladen. Die geprüften Worktrees blieben sauber.

### Umgebungen

Für 2nd-Brain:

```text
env -i HOME=/home/nathanael PATH=/home/nathanael/.cargo/bin:/usr/bin:/bin CARGO_TARGET_DIR=/tmp/review-bd-20260929-target CARGO_BUILD_JOBS=2 CARGO_NET_OFFLINE=true SQLX_OFFLINE=true
```

Für Bots und Docs:

```text
env -i HOME=/tmp/review-bd-20260929-home CARGO_HOME=/home/nathanael/.cargo RUSTUP_HOME=/home/nathanael/.rustup PATH=/home/nathanael/.cargo/bin:/usr/bin:/bin CARGO_TARGET_DIR=/tmp/review-bd-20260929-target CARGO_BUILD_JOBS=2 CARGO_NET_OFFLINE=true SQLX_OFFLINE=true
```

Beim Python-Policy-Test zusätzlich `PYTHONDONTWRITEBYTECODE=1`; kein neuer Python-Code erstellt. Technisch wurden diese vollständigen Umgebungen an `spawnSync` übergeben, ohne Pipe hinter den Prüfkommandos. Der Exitcode wurde direkt vom Kindprozess übernommen.

### 2nd-Brain, Arbeitsverzeichnis `/home/nathanael/.worktrees/second-c9-consumer-wiring`

| Befehl nach dem Umgebungsprefix | Exit | Ergebnis |
| --- | ---: | --- |
| `cargo --version` | 0 | Cargo 1.97.1, `c980f4866 2026-06-30` |
| `cargo fmt --manifest-path tools/brain-adapter/Cargo.toml -- --check` | 0 | Formatprüfung bestanden |
| `cargo test --manifest-path tools/brain-adapter/Cargo.toml --all-targets --locked --offline -- --include-ignored` | 0 | 12 Bibliotheks- und 4 CLI-Tests bestanden; 0 failed, ignored, filtered |
| `cargo clippy --manifest-path tools/brain-adapter/Cargo.toml --all-targets --locked --offline -- -D warnings` | 0 | Ohne Warnungsfehler |

### Weitere eigene Tests

| Arbeitsverzeichnis unter `/home/nathanael/.worktrees/` | Befehl nach dem Umgebungsprefix | Exit | Ergebnis |
| --- | --- | ---: | --- |
| `bots-c9-consumer-wiring` | `cargo test --manifest-path rust/Cargo.toml -p dl-brain --all-targets --locked --offline -- --include-ignored --test-threads=2` | 0 | 12 bestanden |
| `bots-c9-consumer-wiring` | `cargo test --manifest-path rust/Cargo.toml -p dl-core --all-targets --locked --offline -- --include-ignored --test-threads=2` | 0 | 40 + 4 + 8 + 3 + 3 = 58 bestanden |
| `bots-c9-consumer-wiring` | `python3 -m unittest tests/test_pr_release_gate_policy.py` | 0 | 2 bestanden |
| `docs-c9-consumer-wiring` | `cargo test --manifest-path tools/brain-adapter/Cargo.toml --all-targets --locked --offline -- --include-ignored` | 0 | 12 Bibliotheks- und 6 CLI-Tests bestanden |

Jeder Cargo-Testlauf meldete 0 fehlgeschlagene, ignorierte und gefilterte Tests. Gesamt: 104 Rust-Tests und 2 Policy-Tests. Keine separate Vorher-Baseline nötig, da der Review keine Produktänderung vornahm und keinen eigenen Testfehler als vorbestehend einstuft.

### Weitere Belegprüfung

Graphify wurde vor der Codesuche gegen `/home/nathanael/.graphify/global-graph.json` aufgerufen; keine Neuindizierung. Danach belegten SHA-gebundene `git grep -n -E`-Suchen die Zwillinge in Bots-Composition und -Handler, beiden Docs-/2nd-Brain-CLI-Zweigen sowie Twitch-Typed-/Legacy-/Shadow-Routen. Die erste Docs-/2nd-Brain-Suche hatte wegen eines falsch maskierten Regex Exit 128; die korrigierte Suche ohne Klammermuster endete für beide mit Exit 0. Ein Kontextwerkzeug verweigerte eine Dateianalyse außerhalb seines Projektroots; die Datei wurde mit dem regulären Read-Werkzeug gelesen. Beides war ein Werkzeugproblem, kein Testfehler.

`gh run view 36556103003 --repo EarlySalty/Deadlock-Bots --log` endete mit Exit 0. Der ergänzende CI-Beleg meldet für `fmt`, `bot-fmt`, `test`, `bot-brain`, `bot-shadow` und `clippy` jeweils `exit=0`. Er wird nicht als zusätzlicher eigener Testlauf gezählt. Das PR-Statusobjekt für Bots hatte daneben noch laufende Workspace-/Config-Jobs. Es wird daher kein vollständig grüner CI- oder Workspace-Zustand behauptet.

Der B-Autorennachweis unterscheidet 232 ausgeführte Tests von drei explizit gefilterten Golden-/Live-Fällen. Sein Workspace-`--no-run` ist Kompilierung, kein ausgeführter Testlauf. Die gemeldeten Workspace-fmt-/clippy-Abweichungen bleiben als Autoreneinordnung außerhalb der C9-Pfade stehen; hier wurde keine neue Zahl-gegen-Zahl-Baseline dafür erhoben. Die Abnahme beruht nicht auf einer Behauptung, der gesamte Workspace sei grün.

## Fremddienstpfade und Abschlussgrenzen

Sieben abgegrenzte Pfade geprüft: Bots zu Brain, Bots-Ausgabe zu Discord, Docs zu Brain und Infisical, 2nd-Brain zu Brain und Infisical, Twitch zu Brain. Der gepinnte Async-Client verwendet lokalen Endpunkt, Request-Frist, deaktivierte Redirects und Proxies, Größen- und Antwortvertragsprüfung. Der Infisical-Transport prüft Socket und Verzeichnisbesitz und deaktiviert Redirects und Proxies. Transport-/Konfigurationsfehler werden an den Aufrufer zurückgegeben; CLIs liefern eine generische Fehlermeldung und Exit 64, Botpfade protokollieren Backendfehler. Erneute Anfragen sind möglich. Die Diagnosen der Adapter sind absichtlich grob und enthalten keine ausgelesenen Zugangswerte.

HTTP- und UDS-Fixtures sind eigene lokale Testgegenstellen. Der Discord-Sendepfad wurde gelesen, nicht gegen Discord ausgeführt. Es gab keine produktive Datenbankverbindung, Dienstumschaltung, Nachricht, Task-Erzeugung oder Veröffentlichung.

Per `gh pr view` bestätigt: Bots #459 offen, Draft, `autoMergeRequest: null`; Docs #4 und 2nd-Brain #2 offen, jeweils ohne Auto-Merge. Twitch #984 ist bereits gemergt. Diese Zustände wurden nicht verändert. Die vier geprüften PRs wurden am Review-Thread verknüpft. Ein Bericht-GO erteilt keine Erlaubnis, diese PRs zu mergen oder G5 zu umgehen.

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 7/7 geprüft
TESTNACHWEIS[TW-1]: 106 passed, 0 ignored | Baseline: 0 eigene Tests rot; fremde Workspace-Baseline nicht erneut gemessen
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: .tasks/2026-09-29-technical-closeout/REVIEW-BD.md
MERGEPROTOKOLL[MS-1]: 0 Git-Schritte einzeln | Anläufe: 0 | Gate: kein Main-Merge beauftragt oder ausgelöst; diese Zählung betrifft Main, nicht die Ablage auf dem Berichtbranch
