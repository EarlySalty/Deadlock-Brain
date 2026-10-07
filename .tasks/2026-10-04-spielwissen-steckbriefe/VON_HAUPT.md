# Von der Hauptsession an D5

## 05.10.2026, 09:35 Uhr: Diagnose angenommen, kein Produktfix

C-F16 ist fertig. Die Ursache ist die fehlende Operatorfreigabe, nicht ein neuer Codefehler. D5 ändert keine Konfiguration, keine Scopes und keine Guards. Kein Gate, kein Main-Push. C-F16 bleibt zu.

Die zwei Quellfreigaben und der normale Lauf gehören W1. HTML und die drei Antworten warten auf diesen Lauf.

## 05.10.2026, 08:43 Uhr: Ein frischer Fixer, nur der Live-Abbruch

`e56e075d` ist live. Die Wartung wiederholt `ENTITY_PROFILE_REFRESH_FAILED`. Profile, Bindungen, Projektionen und Quittungen bleiben null. W1 hat keinen neuen Fehlertext. Der lokale Import von C-F15 bleibt der letzte grüne Beleg.

1. Ein frischer Fixer, nicht W1. Er liest den Fehlertext, den der Runner für den Live-Tick jetzt behält.
2. Ist das derselbe bereits behobene Allokationsabbruch, stoppt er und nennt den Unterschied zwischen der lokalen Probe und dem Live-Tick. Keine zweite Codeänderung auf Verdacht.
3. Ist es eine neue Ursache, behebt er nur die. Grenzen bleiben. `canonical_raw_dir` bleibt aus. Kein zweiter Feed, kein neues Schema.
4. Danach normales Gate gegen aktuelles `origin/main` `e56e075d`. Vor ALLOW kein Main-Push durch D5.

## 05.10.2026, 05:16 Uhr: Ein frischer Fixer, nur der verschluckte Importfehler

W1 hat den normalen Tick gemessen. 17 JSON-Dokumente mit 78.749 Fakten aus Ref `46c3fd0c` liegen. GameTracking-Dokumente, Profile, Quittungen und Aktivierung sind null. Der Tick endet trotzdem Exit 0, Status `ENTITY_PROFILE_REFRESH_FAILED`.

Auf `origin/main` steht das in `rust/crates/brain-maintenance/src/integration/runner.rs:659`. `refresh_entity_profiles` liefert einen Fehler, der Lauf speichert nur den Sammelcode und wirft den Text weg.

1. Ein frischer Fixer, nicht W1. Er macht den echten Fehler sichtbar und behebt nur diese eine Ursache, damit der normale Import den aktuellen GameTracking-Ref `e0b9830a` ablegt.
2. Grenzen bleiben. `canonical_raw_dir` bleibt aus. Der NIT bleibt ungebaut. Kein zweiter Importweg, kein neuer Feed, keine Änderung der Patchtabellen.
3. Danach normales Gate gegen aktuelles `origin/main` `e036fbde`. Vor ALLOW kein weiterer Main-Push durch D5. W1 allein übernimmt den Stand.

## 05.10.2026, 04:36 Uhr: Kein Schema und kein Ersatzfeed

W1 besitzt die drei fehlenden Patchobjekte auf Port 5446. D5 baut keinen Ersatzfeed, kein paralleles Schema und kein DDL. Die vier Fachmigrationen bleiben unverändert.

Der Dienst `e036fbde` ist live. HTML und die drei Antworten fehlen noch. A4 und Wiki bleiben Stufe 2. Kein neuer Fixer. Der NIT bleibt ungebaut.

## 04.10.2026, 23:32 Uhr: Frischer Fixer, nur der eine BLOCK

Die Gate-Wiederholung ist BLOCK. Urteil gelesen: `/home/nathanael/.cache/brain-d5-c-current-main-gate-3bfe721-wiederholung.log`. Einziger Blocker: `read_entity_evidence` prüft `permits_entity_profile_model_context` nicht, dadurch bekäme jeder zugelassene Consumer Steckbrief-Belege. Die sechs NITs bleiben ungebaut.

`3bfe721` enthält das aktuelle Main nicht. Sechs Konflikte. Der reine Tree würde die live Personenrechte, den Discord-SDK-Vertrag und die Providerprüfung zurücksetzen. Der geprüfte Lauf war weiterhin der große Dreipunktdiff.

1. Ein frischer Fixer, nicht C und nicht C-F1. Er setzt den Stufe-1-Eigenanteil auf aktuelles `origin/main` `022ed841`. Erhalten bleiben `answer_for_discord`, `X-Discord-User-Id`, `handle_answer_with_discord`, `DiscordRequestContext`/`RequestScoped` und die Provider-Anfrageprüfung.
2. Derselbe Fixer behebt nur den BLOCK: `local_pg_reader.rs:690` samt `local_pg_reader.rs:498` und `:1166`, `brain-serve/src/service.rs:175`, `dbrain-retrieval/src/entity_profile_port.rs:46`, `dbrain-retrieval/src/release_port.rs:281`. Die vorhandene Freigabe muss den Modellkontext tatsächlich abschalten können.
3. Eine Hand in den sechs Konfliktpfaden: `brain-contracts/src/entity_profile.rs`, `brain-maintenance/src/entity_profile_render.rs`, `brain-serve/src/config.rs`, `brain-storage/src/entity_profile.rs`, `dbrain-sources/src/bin/brain-knowledge-import.rs`, `dbrain-sources/tests/entity_profiles.rs`. W1 baut dort nicht.
4. Danach normales Gate gegen aktuelles `origin/main` und Lieferung über `w1/EINGANG.md`. Vor ALLOW kein Main, kein Deploy, keine produktive Migration, keine Grants.

## 04.10.2026, 22:43 Uhr: Stand an W1 geben, Gate gegen aktuelles Main

Beide Exit-2-Läufe sind belegt. Der Diff gegen `8a88767` ist für das Gate zu groß. Der vollständige Diff von `3bfe72166d68521bb172c79e966ce5db4cd7b34e` gegen `origin/main` `022ed841` ist 576.344 Byte.

1. Lege genau diesen Stand in `welle1/w1/EINGANG.md`. Kein eigener Main-Push, kein Deploy, keine produktive Migration.
2. Keine dritte Gate-Runde gegen `8a88767` und kein weiterer Reviewer.
3. W1 prüft das Paket gegen das aktuelle Main. Vor ALLOW bleibt der Live-Stand unverändert.

## 04.10.2026, 20:07 Uhr: Schnitt, erst Stufe 1 live

Nach sechs Gate-Runden ohne Live-Stand wird geschnitten:
- **Stufe 1 (jetzt):** Git-Spielwerte des aktuellen Patches plus Patch-Historie aus `brain.patch_changes`, Steckbrief-Dokument im Brain und HTML auf `/brain`. Ziel: Grundbestand (Originalfix `62805c0` auf `e11cbba`) und C so schnell wie möglich mit ALLOW an W1, Deploy, drei echte Antworten.
- **Stufe 2 (danach):** Wiki-Fakten, Wiki-Quittungen, A4 und erweiterte Intervall-Ableitungen. A4 bleibt bis Stufe 1 live lesend oder pausiert.
Keine weiteren Verträge oder Grenzfälle in Stufe 1 aufnehmen, die nicht für die drei Antworten nötig sind.

## 04.10.2026, 18:27 Uhr: Integrations-BLOCK und Anbieterfreigabe

1. **Integrations-BLOCK gehört dir.** W1s Gate auf dem Integrationskandidaten `e11cbba` blockt in `brain-storage/src/entity_profile.rs:174`: Vergleichsgruppen beachten Subject und Dokumentpfad nicht, gleiche lokale `/value`-Felder aus verschiedenen Quellen (z. B. Gesundheit und Schaden) ergeben falsche Konflikte oder Vorzugswerte. Betroffen `file.kv_value`, `file.json_value`, `file.kv3_value`, `wiki.data.value`. Urteil: `/home/nathanael/.cache/brain-d5-a-integration-gate.log`. Frischer Fixer aus deinem Bereich, Eigentum am Pfad stimmst du mit A3 ab (eine Hand in `brain-storage`). D1/W1 baut dort nicht. Geprüften Fix über `welle1/w1/EINGANG.md` an W1.
2. **Anbieterfreigabe: ja.** Abgeleitete Git-Spielzahlen, benannte Patchänderungen und Wiki-Fakten dürfen an das bestehende zugelassene Antwortmodell (Fireworks über den vorhandenen Weg) gehen. Das ist Spielwissen, keine Nutzerdaten, und genau der Zweck. `provider_egress_allowed` entsprechend setzen. Weiter ausgeschlossen: Rohdateien, Rohmetadaten, Dateipfade und wörtliche Wiki-Texte auf öffentlichen Seiten. Kein neuer Anbieter, kein Modellwechsel.

## 04.10.2026, 15:05 Uhr: Produktfrage öffentliche Spielwerte entschieden

Abgeleitete Spielwerte aus den Git-Spieldaten (Zahlen, Patchdaten, Änderungen je Entität) dürfen auf der öffentlichen HTML-Seite stehen. Das sind Spielfakten, wie sie jede Community-Seite zeigt. Nicht öffentlich: Rohdateien, Dateipfade und wörtliche Wiki-Texte. Wiki bleibt intern.

Weiter so: A-Gate abschließen, A2 danach, Lieferung über W1-EINGANG. Fokus auf den Fertig-Beleg (drei echte Brain-Antworten plus automatischer Patchdurchlauf), keine Extras.
