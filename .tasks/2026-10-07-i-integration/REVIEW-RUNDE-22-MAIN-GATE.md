# Runde 22: gemeinsamer Gate und tatsächlicher Main-Push-Deny

## Getrennte tatsächliche Urteile

Gemeinsamer Kandidat af4736089cc5ce5d41ed442d445c49a30d5c6375 gegen ca4d877f13042c9a7a7023e54f6bf2c688b69ac4. Expliziter lokaler Gate über gate_hook.py --review --model claude-opus-5-5: Exit 0, Original /tmp/brain-i-af473608-common-gate-opus55.log mit normalem Read geprüft.

```text
ALLOW: No merge-blocking defect established in this diff.
1. `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:45` | NIT: One unavailable older original aborts the patch sync and the scheduled build-data step | Replay a real feed snapshot with one unavailable older original to establish the operational impact.
2. `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:222` | NIT: An image URL makes otherwise complete original text fail the completeness check | Check one official full-text patch containing an image link.

WIRKUNGSPRUEFUNG[WP-1]: 2 Befunde | Zwillingssuche: keine Fundstelle | Fremddienst-Pfade: 4/4 geprüft
```

Kandidat anschließend auf Arbeitsbranch gesichert. Der tatsächliche reguläre git push origin HEAD:main wurde durch den unveränderten PreToolUse-Hook vor Ausführung verweigert. Dieser Hook meldete selbst [gpt-6.1-sol]; kein Modell- oder Environmentwechsel durch diese Session. Die explizite Opusfreigabe wird nicht als tatsächliche Main-Freigabe ausgegeben.

```text
[gpt-6.1-sol] BLOCK: Original selection and cosmetic filtering can silently corrupt patch imports.

1. rust/crates/dbrain-sources/src/forum.rs:484 | BLOCKING: First visible post is treated as the thread’s original | Accepted `/threads/.../page-2` and `?page=2` URLs can supply a reply containing gameplay changes. `first_post_html` selects that reply without checking original-post identity. Query normalization at api_sync.rs:178 can retain the root patch’s ID; import_prepared_patch then prunes its existing events before inserting the reply’s events.

2. rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:339 | BLOCKING: Bound gameplay entity names trigger the cosmetic veto | A bound item `Metal Skin` with “cooldown reduced from 22 to 20” is rejected because `skin` occurs in the subject. The shared classifier affects all three paths: candidate detection, completeness checking, and prepared-event projection. Valid changes disappear; reimporting a mixed patch can delete previously stored events for that item.

3. rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:45 | NIT: One unavailable historical original aborts the scheduled build-data run | Both Steam and forum fetch errors propagate out of the loop; the script’s `set -e` prevents the subsequent build-data command. Replay one feed snapshot containing an unavailable older original to establish operational impact.

4. rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:222 | NIT: Any hyperlink or remote image disqualifies otherwise complete original text | The image regression explicitly confirms this rejection. Check one official complete patch containing an incidental image or link to establish the production impact.
```

Nach Deny zuerst git status und git log -1 einzeln: Kandidat weiterhin af473608, sauber und auf Arbeitsbranch gesichert. Kein Main-Push oder Deploy.

## Verifikation und verbindlicher Schnitt

Zuerst Graphify, dann tatsächliche E-Quelle geprüft. Forum first_post_html, Zeilen 479 bis 487, nimmt parse_thread_html(...).posts.into_iter().next() ohne Bindung an den ursprünglichen Startbeitrag. trusted_original_url, Zeile 131, akzeptiert den /threads/-Pfad einschließlich Folgeseiten. Das ist ein neuer inhaltlicher Originalquellenfund.

has_gameplay_change, Zeilen 334 bis 360, berechnet cosmetic über Wörter der vollständigen Klausel einschließlich subject. Wort skin ist ausdrücklich in der Vetoliste. Zeile 392 verlangt !cosmetic auch bei einem bereits gebundenen item und bei cooldown. Damit wird Metal Skin: cooldown reduced from 22 to 20 verworfen. Kein dynamischer Reproduktionstest neu behauptet; der konkrete statische Kern ist bestätigt.

Genau eine fachliche URL-Fixrunde ist bereits beendet. Keine zweite Discovery-Fixrunde und keine weiteren Discovery-Worker. Stattdessen den ausdrücklich beauftragten Fallback umgesetzt: neuer lokaler und Remote-Branch feat/brain-patch-discovery hält exakt af473608. Der gemeinsame Integrationsbranch bleibt auf demselben gesicherten Stand erhalten.

Spiegelbranch feat/brain-assets-mirror-20261007 im eigenen Integrationsarbeitsbaum von ca4d877f angelegt. Der zuvor übergebene gleichnamige Vorbereitungsref war bei der tatsächlichen Prüfung nicht vorhanden; kein bestehender Ref überschrieben. Bestehende Spiegeldateien gezielt aus af473608 übernommen. pg_patchnotes.rs, dessen Unterpfad und forum.rs bleiben identisch zu ca4d877f, durch git diff --exit-code bestätigt. Neue Discovery-CLI-Varianten und der neue Timer-Aufruf ausgeschlossen. Der nicht gelieferte Discovery-CLI-Test bleibt beim erhaltenen Discoverybranch; der Spiegel-Timertest erwartet jetzt die tatsächlichen zwei Aufrufe assets und build-data und prüft ausdrücklich, dass sync-patchnotes nicht startet. Keine alte Main-Suite abgeschwächt.

Spiegel erhält eigenen Compiler-/Test-/Gatebeweis. Die Freigabe für af473608 wird dafür nicht wiederverwendet. Bestehender Patchimport bleibt wie main. Toolfehler hätten diesen Schnitt nicht ausgelöst. Main-Push, Deployment, Liveprüfung und F-Arbeit stehen noch aus.

MERGEPROTOKOLL[MS-1]: 8 Git-Schritte einzeln | Anläufe: 1 | Gate: explizit [claude-opus-5-5] ALLOW, tatsächlicher Main-Push-Hook [gpt-6.1-sol] BLOCK

Zählbereich: Kandidatensicherung, verweigerter Main-Push, unmittelbar folgende Status-/HEAD-/Worktree-/Discoveryref-Prüfung, Discoverybranchanlage, Discovery-Remote-Sicherung. Spätere Spiegelbranchanlage und gezielte Übernahme sind eigener Schnittbereich.
