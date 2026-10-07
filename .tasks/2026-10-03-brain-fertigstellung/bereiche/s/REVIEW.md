status: aktiv
Datum: 2026-10-03

# Prüfung Paket S2

## Geprüfte Featurestände

Steam: `9aec0cc897b01b74d417ab9b510314cbbbd02535`, Branch `feat/steam-publish-fertig-20261003`, Basis `4c5621763d5f01c96d7912400517c08aa1c40df1`.

Brain: `9a6f3d5f3ae2349d9fb076821381e45a421a0bbe`, Branch `feat/brain-fertig-s-20261003`, Basis `511a347b653beba13c2bf130f4bead7a7196cc2a`.

Beide Featurebranches sind nach origin gepusht. Die sieben übernommenen Dateien waren vor den Commits bytegleich zum Startabdruck. Keine main-Integration, kein eigener Releasewechsel.

## Lokaler Merge-Gate

Je ein Aufruf von `/home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo <Worktree> --base <Basis-SHA> --timeout 600`, ohne Modellübersteuerung. Beide Werkzeugprozesse haben Exit 0.

- Steam: `[gpt-6.1-sol] ALLOW: No blocking defect is established by the supplied changes.` Volltext: `pruefung-v2/gate-steam.log`.
- Brain: `[gpt-6.1-sol] ALLOW: No grounded merge-blocking defect found.` Volltext: `pruefung-v2/gate-brain.log`.

Die Selbstprüfung betrifft den eigenen Diff gegen die jeweilige Übernahmebasis. Sie ersetzt Zs Gate für den kombinierten, aktualisierten Integrationsstand nicht.

Zwei nicht blockierende Hinweise bleiben dokumentiert: Steam testet den Admission-Helper direkt, statt im neuen Test den kompletten wartenden GC-Sendeweg aufzurufen. Brain ordnet eine Zeitüberschreitung beim Lesen des Antwortkörpers als `Unavailable` ein, obwohl eine beim Senden als `Timeout` erscheint. Keiner der beiden Hinweise ist ein BLOCK.

## Laufende unabhängige Prüfung

Native Rust-/Security-Codeprüfung `wf_e89b4844-2bd` und Intent-Abnahme mit frischem Kontext `wf_0114aa23-616` sind abgeschlossen. Beide haben den Vorfixstand geprüft. GPT-6.1 Sol geerbt, Effort `xhigh`, UltraCode tatsächlich verfügbar. Die Intent-Abnahme verweigert den lokalen Abschluss bis zu den folgenden Korrekturen.

Zusätzlich zu den zwei HTTP-Befunden verlangt die Intent-Abnahme:

1. Gespeicherte unveränderliche Publish-Anfrage und explizite CLI-Wiederaufnahme ohne erneute AI-Berechnung. Sonst kann ein zweiter CLI-Lauf andere Inhalte und eine neue ID erzeugen.
2. Anhaltendes HTTP-429 beim Statuspolling sichtbar als Drosselung behalten, statt am Ende allein timeout zu melden.
3. BLOCKED-Ausgabe bei fehlender regulärer Freigabe mit Fehlerexit verbinden. Der Zweig bestand bereits vorher; eine Testbaseline dazu wurde nicht gemessen.

Fundstellen am Vorfix-SHA: `main.rs:2364`, `:2197`, `:2301-2311` und `brain-feeds/src/build_publish.rs:282-289`. Die Folgerunde ist in `FOLGE-FIX-BRIEFING.md` vorbereitet und startet nach dem ersten Fixer, damit kein Dateieigentum überlappt. Fehlender Live-Beweis allein wäre kein Hindernis für die fachliche Übergabe an Z; die bestätigten lokalen Abweichungen bleiben bis zum Fix offen.

Der erste rust-reviewer `wf_7a86d4fb-e98` versuchte entgegen seinem lesenden Auftrag einen Cargo-Vorcheck mit der alten System-Cargo-Version. Er brach am Lockfileformat ab und reviewte den Diff nicht. Das ist kein Compiler- oder Testnachweis. Die Wiederholung ist ein lesender General-Purpose-Prüfer ohne Cargo-Auftrag.

## Compiler und bestehende Suites

Werkzeugprozess `b2n6tbhk1`: beide Hostsperren in vorgeschriebener Reihenfolge. Nach Erwerb zunächst echte fremde Compiler gefunden und mit gehaltenen Sperren alle 30 Sekunden neu geprüft. Nach freier Probe Start mit Cargo 1.99, `PATH=/home/nathanael/.cargo/bin:$PATH`, `CARGO_BUILD_JOBS=2`, `SQLX_OFFLINE=true`, `--locked`, `-j 2`, `--include-ignored`.

Die erste Prüfrunde ist beendet. Brain: 2 Libtests, 16 HTTP-Integrationstests, 2 CLI-Publishtests bestanden; zusammen 20 passed, 0 failed, 0 ignored, 93 filtered. Steam: beide Cargo-Aufrufe scheiterten vor Teststart an `--locked`; keine Testresultate. Der äußere Werkzeugprozess meldete dennoch Exit 0. Dieser Exit wird ausdrücklich nicht als Steam-Prüferfolg gewertet. Die eigenen Kinder waren beendet und beide Sperren wurden im EXIT-Trap geschlossen.

Steam bindet `../../Deadlock-Bots` an einen fremden Worktree `open-pr-472-community-bridge-20261001`. Dessen aktueller `dl-central-db`-Manifest verlangt `chrono-tz`; der unveränderte Steam-Lockeintrag enthält es nicht. Kein Lockfile-Abgleich außerhalb des genehmigten Eigentums und keine Änderung der fremden Verknüpfung. Entscheidung in AN_HAUPT.md angefordert.

Die unabhängige Prüfung `wf_e89b4844-2bd` ist abgeschlossen. Zwei bestätigte P2-Befunde im Brain-Client: nach drei unklaren POST-Antworten kein hashgebundener Status-GET; Fristprüfung verwirft einen bereits validierten Endzustand. Statische Prüfung von Hashbindung, transaktionaler Doppelanlagensicherung, positivem Build-ID-Vertrag, Blocking-Isolation, Auth/URL-Grenzen und GC-Admission ohne weitere bestätigte Funde. Frischer Fixer `wf_806b96af-dd4` bearbeitet die zwei betroffenen brain-feeds-Dateien. Ein neuer SHA setzt die bisherige Brain-Gateabnahme zurück.

Erste Fixrunde abgeschlossen: Brain-SHA `148e1a58a485def587f763c76c9ec7a9ff06040f`, Featurebranch gepusht. Statusprüfung vor POST, hashgebundene Erholung nach verlorener POST-Antwort, bestätigter Endzustand vor Fristprüfung und einheitliche Timeout-Klassifikation umgesetzt. 4 Lib-, 23 HTTP-, 2 CLI-Tests bestanden, insgesamt 29 passed, 0 failed, 0 ignored. Format und Clippy Exit 0. Logs `pruefung-v2/fix-*.log`.

Gate im ersten Anlauf ohne Urteil wegen Namespace-Speicherfehler, technischer Retry: `[gpt-6.1-sol] ALLOW: No blocking defect established by the supplied diff.` Das ist keine Freigabe der noch offenen drei Intent-Befunde. Frischer Folgerunden-Fixer `wf_67a90ad5-45a` ist gestartet und besitzt die drei Dateien aus FOLGE-FIX-BRIEFING.md.

TESTNACHWEIS[TW-1]: 29 passed, 0 ignored | Baseline: nicht erhoben rot

Eine Baseline wurde nicht gemessen. Die Steam-Blockade wird nicht als vorbestehender Testfehler behauptet; es liefen dort keine Tests.

## Unabhängige Abnahme des Steam-Lockdiffs

`wf_2a889a29-6ad` hat den autorisierten Lockdiff gegen Steam-HEAD `9aec0cc897b01b74d417ab9b510314cbbbd02535` lesend abgenommen. 35 Einfügungen, keine Entfernungen und keine Änderung bestehender Paketversionen. Die nötige Kette ist `dl-central-db` zu `chrono-tz 0.10.4`, `phf 0.12.1`, `phf_shared 0.12.1` und `siphasher 1.0.3`.

Manifestbeleg: `/home/nathanael/.worktrees/open-pr-472-community-bridge-20261001/rust/crates/dl-central-db/Cargo.toml:14` verlangt `chrono-tz.workspace`; das Workspace-Manifest an Zeile 82 erlaubt Version `0.10`. SHA256 des abgenommenen und am 18:03-UTC-Snapshot weiterhin identischen Steam-Lockfiles: `85d00ffca8e41dee91a2c47a1142b827cb76fbb1707b473c9283645225da2357`.

Die Abnahme bindet den Dateiinhaltsstand, noch keinen neuen Steam-Commit oder Testlauf. Aktuelle Wrapper: Steam `3120972` mit `flock`-Kind `3120985`, Brain `3294685` mit `flock`-Kind `3294687`. Beide leben und warten auf `host-checks.lock`, Inode `16006309`; aktuelle Prüflogs enthalten lediglich den Sperrerwerbsbeginn. Keine bestätigten Nachlauftestzahlen, keine fremden Prozesse gestoppt. Der unterbrochene Steam-Vorläufer zählt nicht als Prüfung.

## Noch fehlender Wirkungsnachweis

CLI-Installation des gemeinsamen SHA durch Z und danach echter HTTP-Publish mit `DONE` und `hero_build_id`. Bis dahin kein vollständiger Abschluss, keine Releasezeigeränderung durch S und keine blinde Wiederholung eines unklaren Publishes.

MERGEPROTOKOLL[MS-1]: 6 Git-Schritte einzeln | Anläufe: 0 | Gate: zweimal ALLOW, kein main-Merge
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: bereiche/s/REVIEW.md
